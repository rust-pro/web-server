# Level 3
# Sử dụng rust:1.67-slim-bookworm thay vì rust:1.67

# Xây dựng hình ảnh có thể chạy được với các gói phụ thuộc cần thiết
# Thời gian Build trung bình: ~273s cho lần đầu tiên, ~20s cho các lần tiếp theo
# Kích thước hình ảnh tạo ra: 3.02GB

FROM rust:1.67-slim-bookworm

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

COPY Cargo.lock ${MICRO_SERVICE_NAME}/Cargo.toml ${MICRO_SERVICE_NAME}/diesel.toml ${MICRO_SERVICE_NAME}/docker/development/.env ./

COPY common ../common

# Tạo layer cho dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release

RUN rm -rf src/*.rs && rm -rf benches/*.rs

COPY ${MICRO_SERVICE_NAME}/ ./

# Build ứng dụng

RUN rm ./target/release/deps/${MICRO_SERVICE_NAME}*

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo install --path . --locked

CMD ["users"]