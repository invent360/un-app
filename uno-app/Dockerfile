# Build from the repository root: docker build -f uno-app/Dockerfile .
FROM rust:1.92.0-slim-bookworm AS builder
ENV CARGO_BUILD_JOBS=1 CARGO_PROFILE_DEV_DEBUG=0
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev curl perl make ca-certificates && rm -rf /var/lib/apt/lists/*
COPY scripts/install_leptos_build_tools.sh /tmp/install_leptos_build_tools.sh
RUN rustup target add wasm32-unknown-unknown && sh /tmp/install_leptos_build_tools.sh
WORKDIR /workspace
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY .cargo ./.cargo
COPY uno-api ./uno-api
COPY file-storage ./file-storage
COPY uno-app ./uno-app
COPY uno-admin ./uno-admin
RUN cargo leptos build --release --project uno-app --frontend-only --lib-cargo-args=--locked
RUN CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 cargo leptos build --release --project uno-app --server-only --bin-cargo-args=--locked

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/* && useradd -r -u 1000 -s /bin/false uno
WORKDIR /app
COPY --from=builder --chown=uno:uno /workspace/target/release/uno-app /app/uno-app
COPY --from=builder --chown=uno:uno /workspace/target/site/uno-app /app/site
ENV LEPTOS_SITE_ROOT=/app/site LEPTOS_SITE_ADDR=0.0.0.0:3000 LEPTOS_ENV=PROD RUST_ENV=production RUST_LOG=info FILE_STORAGE_BACKEND=local FILE_STORAGE_LOCAL_PATH=/var/lib/uno-app/media FILE_STORAGE_LOCAL_URL=/files
EXPOSE 3000
# Provision and mount the media volume separately. Do not create /data/uploads here.
HEALTHCHECK --interval=30s --timeout=5s --start-period=30s --retries=3 CMD curl -fsS http://localhost:3000/api/v1/ready || exit 1
USER uno
CMD ["./uno-app"]
