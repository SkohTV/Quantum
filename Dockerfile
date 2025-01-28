# Build stage
FROM rust:1.82 AS builder

WORKDIR /quantum

COPY src/ src
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --release -v



# Release stage
FROM debian:12-slim AS release

WORKDIR /quantum
COPY .env .
COPY --from=builder /quantum/target/release/quantum /quantum/bin

ENTRYPOINT ["/quantum/bin"]
