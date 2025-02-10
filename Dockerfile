FROM rust:alpine3.21 AS builder

RUN apk add --no-cache musl-dev libgcc curl

WORKDIR /app

ARG POSTGRES_USER
ENV POSTGRES_USER=${POSTGRES_USER}

ARG POSTGRES_PASSWORD
ENV POSTGRES_PASSWORD=${POSTGRES_PASSWORD}

ARG POSTGRES_DB
ENV POSTGRES_DB=${POSTGRES_DB}

ARG POSTGRES_HOST
ENV POSTGRES_HOST=${POSTGRES_HOST}

ARG POSTGRES_PORT
ENV POSTGRES_PORT=${POSTGRES_PORT}

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /usr/src/app

COPY --from=builder /app/target/release/axum-api .

EXPOSE 3000
CMD ["./axum-api"]
