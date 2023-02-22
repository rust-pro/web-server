FROM rust:1.67 as build

ENV USER kukun
ENV APP_HOME /app/users
ENV CARGO_TERM_COLOR always

RUN apt-get update && apt-get install -y libpq-dev clang

RUN useradd --system --groups root --user-group --create-home --home-dir /home/$USER $USER

WORKDIR $APP_HOME

RUN USER=root cargo init
COPY Cargo.lock users/Cargo.toml users/diesel.toml users/docker/development/.env ./
COPY common ../common

# Remove [[bench]] in Cargo.toml (Line 34~36)
RUN sed -i '34,36d' Cargo.toml

# Tạo layer cho dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release

RUN rm src/*.rs && rm -f Cargo.toml

COPY users/Cargo.toml ./
COPY users/benches ./benches
COPY users/src ./src
COPY users/test ./test

# Build app: /usr/local/cargo/bin/users
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo install --path . --locked

# Run file users in: /usr/local/cargo/bin
CMD ["users"]