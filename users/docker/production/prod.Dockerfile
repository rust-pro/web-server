FROM rust:1.65 as builder

ENV CARGO_TERM_COLOR always
RUN apt-get update && apt-get install -y libpq-dev clang

# Set the application directory
WORKDIR /usr/src/users

# Install musl-tools to make many crates compile successfully
RUN apk add --no-cache musl-dev

# Install cargo-watch
RUN cargo install cargo-watch

# create empty project for caching dependencies
RUN USER=root cargo init
COPY common-utils ../common-utils
COPY Cargo.lock users/Cargo.toml ./

# cache dependencies
RUN cargo install --path . --locked

# Copy the files to the Docker image
COPY users/ ./
RUN touch src/main.rs
RUN cargo install --path . --locked

# Multi-stage builds
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=builder /usr/local/cargo/bin/users /bin/users
CMD ["users"]
