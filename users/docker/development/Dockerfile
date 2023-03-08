FROM rust:1.67 as build

ENV USER kukun
ENV APP_HOME /app/users
ENV CARGO_TERM_COLOR always

RUN apt-get update && apt-get install -y libpq-dev clang

RUN useradd --system --groups root --user-group --create-home --home-dir /home/$USER $USER

WORKDIR $APP_HOME

RUN USER=root cargo init
RUN mkdir -p benches && echo 'fn main() { println!("Hello, bench!"); }' > benches/main.rs
COPY Cargo.lock users/Cargo.toml users/diesel.toml users/docker/development/.env ./
COPY common ../common

# Tạo layer cho dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release

RUN rm -rf src/*.rs && rm -rf benches/*.rs

COPY users/benches ./benches
COPY users/src ./src
COPY users/test ./test

# Build ứng dụng
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo install --path . --locked

CMD ["users"]