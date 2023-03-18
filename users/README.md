# Quick reference
---

- Maintained by: <br>
  [The Rust Project developers](https://github.com/rust-pro/web-server)

- Where to get help: <br>
  The
  [Cargo](https://doc.rust-lang.org/stable/cargo/),
  [Postgresql](https://www.postgresql.org/download/),
  [Apache Kafka](https://kafka.apache.org/quickstart),
  [Diesel CLI](https://diesel.rs/guides/getting-started.html),
  [CMake](https://cmake.org/download/#latest),
  [LLVM](https://releases.llvm.org/download.html)
  [Apollo federation](https://www.apollographql.com/docs/federation/quickstart/local-composition/)

# Supported tags and respective `Dockerfile` links
---
[latest , main, sha-hash](https://github.com/rust-pro/web-server/blob/main/users/docker/production/Dockerfile)

# Quick reference (cont.)
---

- Where to file issues:
  https://github.com/rust-pro/web-server/issues

# What is Rust?
---
![Logo](https://raw.githubusercontent.com/docker-library/docs/a11c341c57de07fbccfed7b21ea92d4bc40130a2/rust/logo.png)
Rust is a systems programming language sponsored by Mozilla Research. It is designed to be a "safe, concurrent,
practical language", supporting functional and imperative-procedural paradigms. Rust is syntactically similar to C++,
but is designed for better memory safety while maintaining performance. <br>
[wikipedia.org/wiki/Rust_(programming_language)](https://en.wikipedia.org/wiki/Rust_(programming_language))

# How to use this image
--- 

## Some simple static content

```shell
docker network create microservice_user_network

docker run --name some-postgres --network microservice_user_network --network-alias microservice_user_db --env POSTGRES_USER=postgres --env POSTGRES_PASSWORD=mysecretpassword --env POSTGRES_DB=users --detach postgres

docker run --name some-user --publish 80:80 --network microservice_user_network --network-alias microservice_user --env APP_URL=0.0.0.0 --env DB_HOST=microservice_user_db --env DB_USERNAME=postgres --env DB_PASSWORD=mysecretpassword --env DB_DATABASE=users --detach kukun/rust-users

docker network inspect microservice_user_network

```

## Example docker-compose.yml for `kukun/rust-users`:

```yaml
services:
  users:
    image: kukun/rust-users:latest
    container_name: users
    restart: always
    depends_on:
      - user-db
    links:
      - user-db
    environment:
      APP_URL: ${APP_URL} #0.0.0.0
      APP_PORT: ${APP_PORT}
      APP_NAME: ${APP_NAME}
      APP_CLIENT: "http://localhost"
      DB_CONNECTION: ${DB_CONNECTION}
      DB_HOST: user-db
      DB_PORT: ${DB_PORT}
      DB_USERNAME: ${DB_USERNAME}
      DB_PASSWORD: ${DB_PASSWORD}
      DB_DATABASE: ${DB_DATABASE}
      ACCESS_TOKEN_SECRET: ${ACCESS_TOKEN_SECRET}
      REFRESH_TOKEN_SECRET: ${REFRESH_TOKEN_SECRET}
      PASSWORD_SECRET_KEY: ${PASSWORD_SECRET_KEY}
      JWT_SECRET_KEY: ${JWT_SECRET_KEY}
      REFRESH_TOKEN_COOKIE_NAME: ${REFRESH_TOKEN_COOKIE_NAME}
    networks:
      users:
      haproxy_network:
        aliases:
          - microservice_user

  user-db:
    image: postgres:15.2-alpine
    restart: always
    environment:
      POSTGRES_USER: ${DB_USERNAME}
      POSTGRES_PASSWORD: ${DB_PASSWORD}
      POSTGRES_DB: ${DB_DATABASE}
    volumes:
      - type: volume
        source: db-data
        target: /var/lib/postgresql/data
        volume:
          nocopy: true
    healthcheck:
      test: [ "CMD-SHELL", "pg_isready -U postgres" ]
      interval: 1m
      timeout: 5s
      start_period: 10s
      retries: 3
    networks:
      - users

volumes:
  db-data:

networks:
  users:
    driver: bridge
  haproxy_network:
    external: true
    name: haproxy_network
```

## Environment Variables

`APP_URL` <br>
`APP_PORT` <br>
`APP_NAME` <br>
`APP_CLIENT` <br>

`DB_CONNECTION` <br>
`DB_HOST` <br>
`DB_PORT` <br>
`DB_USERNAME` <br>
`DB_PASSWORD` <br>
`DB_DATABASE` <br>

`ACCESS_TOKEN_SECRET` <br>
`REFRESH_TOKEN_SECRET` <br>
`PASSWORD_SECRET_KEY` <br>
`JWT_SECRET_KEY` <br>
`REFRESH_TOKEN_COOKIE_NAME` <br>