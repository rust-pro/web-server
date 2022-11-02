FROM rust:1.64 as builder

ENV CARGO_TERM_COLOR always
RUN apt-get update && apt-get install -y libpq-dev

WORKDIR /usr/src/docker-build
# create empty project for caching dependencies
RUN USER=root cargo init
COPY Cargo.lock actix/Cargo.toml ./
# cache dependencies
RUN cargo install --path . --locked

COPY actix/ ./
RUN touch src/main.rs
RUN cargo install --path . --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=builder /usr/local/cargo/bin/actix /bin/
CMD ["actix"]
