FROM rust:1.70-alpine

RUN apk add --no-cache musl-dev libgcc curl

WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release

EXPOSE 3000
ENTRYPOINT ["./target/release/axum-api"]