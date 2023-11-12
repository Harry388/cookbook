FROM rust:1.73-bookworm

EXPOSE 8000

WORKDIR /code

RUN cargo install cargo-watch

RUN cargo install sqlx-cli --no-default-features --features native-tls,mysql

COPY ./ ./