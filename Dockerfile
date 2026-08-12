# ─── Stage 1: Builder ──────────────────────────────────────────────
FROM rust:1.96-bookworm AS builder

WORKDIR /build

# Install system deps for resvg/usvg
RUN apt-get update && apt-get install -y \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy src to build dependencies only
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && \
    echo "pub mod cli; pub mod render; pub mod schema; pub mod server; pub mod template; pub mod text;" > src/lib.rs

# Build dependencies (cached layer)
RUN cargo build --release 2>/dev/null || true

# Copy actual source
COPY src/ ./src/
COPY templates/ ./templates/

# Touch source files to force rebuild of cosy (not deps)
RUN touch src/main.rs src/lib.rs

# Build release binary
RUN cargo build --release

# ─── Stage 2: Runtime ──────────────────────────────────────────────
FROM debian:bookworm-slim

WORKDIR /app

# Install minimal runtime deps + curl for healthcheck
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/cosy /usr/local/bin/cosy

# Copy templates (needed at runtime)
COPY --from=builder /build/templates/ ./templates/

# Non-root user for security
RUN useradd -r -s /bin/false cosy && chown -R cosy:cosy /app
USER cosy

# Default config
ENV COSY_API_KEY=""
ENV RUST_LOG="info"
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -sf http://localhost:3000/api/health || exit 1

ENTRYPOINT ["cosy"]
CMD ["serve", "--port", "3000"]
