# Stage 1 — Build
FROM rust:1.90 AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev

WORKDIR /usr/src/app

# Copy Cargo files first to leverage caching
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release || true

# Copy full source
COPY . .

# Build release binary
RUN cargo build --release

# Stage 2 — Runtime
FROM debian:bookworm-slim

# Install runtime deps
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Set logging
ENV RUST_LOG=info

# Create non-root user
RUN useradd -m appuser
USER appuser

# Copy binary from builder
COPY --from=builder /usr/src/app/target/release/commissions-service .

# Strip binary to reduce image size
RUN strip ./commissions-service

# Expose port
EXPOSE 5000

# Start service
CMD ["./commissions-service"]
