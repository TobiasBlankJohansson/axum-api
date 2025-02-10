FROM rust:alpine3.21 AS builder

RUN apk add --no-cache musl-dev libgcc curl

WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

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
