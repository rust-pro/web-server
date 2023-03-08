FROM rust:1.67

# Set Docker Environment Variables
ARG USER="kukun"
ARG APP_NAME="Production"
ARG WORKDIR="/app/users"

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

RUN useradd --system --groups root --user-group --create-home --home-dir /home/$USER $USER

WORKDIR $WORKDIR

COPY Cargo.lock ./
COPY common ../common
COPY users/ ./

RUN cargo build --release

CMD ["./target/release/users"]