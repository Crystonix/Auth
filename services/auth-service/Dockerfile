# Stage 1 — Build
FROM rust:1.71 as builder

# Install required dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev

# Set workdir
WORKDIR /usr/src/auth-service

# Copy manifests first for caching dependencies
COPY Cargo.toml Cargo.lock ./
COPY db/Cargo.toml db/
COPY models/Cargo.toml models/
COPY routes/Cargo.toml routes/
COPY config/Cargo.toml config/

# Create empty src files to satisfy cargo
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Fetch dependencies (cached layer)
RUN cargo fetch

# Copy the source code
COPY . .

# Build the release binary
RUN cargo build --release

# Stage 2 — Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Set workdir
WORKDIR /app

# Copy the compiled binary from builder
COPY --from=builder /usr/src/auth-service/target/release/auth-service .

# Copy .env file if needed
# COPY .env .env

# Expose port
EXPOSE 4000

# Run the service
CMD ["./auth-service"]
