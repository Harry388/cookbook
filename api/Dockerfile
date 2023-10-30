FROM rust:alpine3.18

EXPOSE 8000

WORKDIR /code

RUN apk add --no-cache musl-dev

RUN cargo install cargo-watch

COPY ./ ./