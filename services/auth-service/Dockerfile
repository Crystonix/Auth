# Stage 1 — Build
FROM rust:1.90 AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev

WORKDIR /usr/src/app

# Copy source code + SQLx cache
COPY  . .

# Build release binary (offline, no DB needed)
RUN cargo build --release

# Stage 2 — Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary
COPY --from=builder /usr/src/app/target/release/auth-service .

EXPOSE 4000
CMD ["./auth-service"]
