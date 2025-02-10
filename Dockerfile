FROM rust:1.70-alpine AS builder

RUN apk add --no-cache musl-dev libgcc curl

WORKDIR /app/src/app

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/axum-api .

EXPOSE 3000
CMD ["./axum-api"]
