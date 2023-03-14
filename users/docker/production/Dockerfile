FROM rust:1.68-slim-bookworm AS builder
# Set Docker Environment Variables
ARG USER="kukun"
# Name project in Cargo.toml
ARG MICRO_SERVICE_NAME="users"
ARG WORKDIR="/app/${MICRO_SERVICE_NAME}"

ENV CARGO_TERM_COLOR="always"

ENV APP_URL="0.0.0.0"
ENV APP_PORT="80"
ENV APP_NAME="AppName"
ENV APP_CLIENT="http://localhost"

ENV DB_CONNECTION="postgres"
ENV DB_HOST="127.0.0.1"
ENV DB_PORT="5432"
ENV DB_USERNAME="postgres"
ENV DB_PASSWORD="password"
ENV DB_DATABASE=""

ENV ACCESS_TOKEN_SECRET=""
ENV REFRESH_TOKEN_SECRET=""
ENV PASSWORD_SECRET_KEY=""
ENV JWT_SECRET_KEY=""
ENV REFRESH_TOKEN_COOKIE_NAME="cookie"

RUN apt-get update && apt-get install -y libpq-dev clang

RUN useradd --system --groups root --user-group --create-home --home-dir /home/${USER} ${USER}

WORKDIR ${WORKDIR}

RUN USER=root cargo init

RUN mkdir -p benches && echo 'fn main() { println!("Hello, bench!"); }' > benches/main.rs

COPY Cargo.lock ${MICRO_SERVICE_NAME}/Cargo.toml ${MICRO_SERVICE_NAME}/diesel.toml ./

COPY common ../common

RUN cargo install --path . --locked

RUN rm -rf src/*.rs && rm -rf benches/*.rs

COPY ${MICRO_SERVICE_NAME}/ ./

RUN rm ./target/release/deps/${MICRO_SERVICE_NAME}*

RUN cargo install --path . --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=builder /usr/local/cargo/bin/${MICRO_SERVICE_NAME} /bin/
EXPOSE ${APP_PORT}
CMD ["users"]
