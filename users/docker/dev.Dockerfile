FROM rust:1.65 as builder

ENV CARGO_TERM_COLOR always
RUN apt-get update && apt-get install -y libpq-dev clang

# Set the application directory
WORKDIR /usr/src/users

# Install cargo-watch, diesel_cli
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres

# Copy the files to the Docker image
COPY common-utils ../common-utils
COPY users/ ./
