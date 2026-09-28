# UNO App Production Dockerfile
# Multi-stage build for minimal image size

# =============================================================================
# Stage 1: Build Environment
# =============================================================================
FROM rust:1.90-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    perl \
    make \
    && rm -rf /var/lib/apt/lists/*

# Install wasm target and cargo-leptos
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos@0.3.7 --locked

# Create app directory
WORKDIR /app

# Copy dependency files first (for layer caching)
COPY Cargo.toml Cargo.lock ./

# Create dummy src for dependency caching
RUN mkdir src && \
    echo 'fn main() { println!("dummy"); }' > src/main.rs && \
    echo 'pub fn dummy() {}' > src/lib.rs

# Build dependencies only (this layer will be cached)
RUN cargo leptos build --release || true

# Remove dummy files
RUN rm -rf src

# Copy actual source code
COPY src ./src
COPY assets ./assets
COPY style ./style
COPY end2end ./end2end
COPY deps ./deps
COPY migrations ./migrations
COPY Cargo.toml Cargo.lock ./

# Build the application
RUN cargo leptos build --release

# =============================================================================
# Stage 2: Runtime Environment
# =============================================================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/false -u 1000 uno-app

# Create app directories
WORKDIR /app
RUN mkdir -p /app/site /data && chown -R uno-app:uno-app /app /data

# Copy built artifacts from builder
COPY --from=builder --chown=uno-app:uno-app /app/target/release/uno-app /app/
COPY --from=builder --chown=uno-app:uno-app /app/target/site /app/site

# Set environment variables
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_ENV=PROD
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/api/v1/health || exit 1

# Run as non-root user
USER uno-app

# Run the application
CMD ["./uno-app"]
