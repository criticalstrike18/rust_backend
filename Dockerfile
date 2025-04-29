# Build stage
FROM rustlang/rust:nightly-slim as builder

WORKDIR /usr/src/app

# Install dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy project files, including .sqlx directory
COPY Cargo.* ./
COPY .sqlx/ ./.sqlx/
COPY src/ ./src/
COPY migrations/ ./migrations/

# Create empty src directory structure if it doesn't exist already
RUN mkdir -p src

# Enable offline mode for sqlx
ENV SQLX_OFFLINE=true
ENV SQLX_RUNTIME_ASSERTIONS=1

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary and migrations
COPY --from=builder /usr/src/app/target/release/rust_backend /app/rust_backend
COPY --from=builder /usr/src/app/migrations /app/migrations
COPY .env /app/.env

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["/app/rust_backend"]