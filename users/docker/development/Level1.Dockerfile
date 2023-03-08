# Level 1
# Xây dựng hình ảnh có thể chạy được với các gói phụ thuộc cần thiết
# Thời gian Build trung bình: ~600s mỗi lần
# Kích thước hình ảnh tạo ra: 3.62GB

FROM rust:1.67

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

COPY Cargo.lock ./
COPY common ../common
COPY ${MICRO_SERVICE_NAME}/ ./

RUN cargo build --release

CMD ["./target/release/users"]