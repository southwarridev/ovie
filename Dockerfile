# Ovie Programming Language v2.3.0
# Multi-stage Docker image
# Publisher: Ovie Language Team | MIT License
#
# Usage:
#   docker pull ghcr.io/southwarridev/ovie:latest
#   docker run --rm -v $(pwd):/workspace ghcr.io/southwarridev/ovie:latest oviec run /workspace/main.ov

# ── Stage 1: Build ────────────────────────────────────────────────────────
FROM rust:1.75-slim AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY oviec/ oviec/
COPY aproko/ aproko/

# Build release binaries
RUN cargo build --release --bin oviec

# ── Stage 2: Runtime ──────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.title="Ovie Programming Language"
LABEL org.opencontainers.image.description="Ovie v2.3.0 — Complete Module System. Self-hosted systems programming language."
LABEL org.opencontainers.image.version="2.3.0"
LABEL org.opencontainers.image.authors="Ovie Language Team <ovielang@gmail.com>"
LABEL org.opencontainers.image.source="https://github.com/southwarridev/ovie"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.url="https://southwarridev.github.io/ovie/"

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create ovie user (don't run as root)
RUN useradd -m -s /bin/bash ovie

# Copy compiler binary
COPY --from=builder /build/target/release/oviec /usr/local/bin/oviec
RUN chmod +x /usr/local/bin/oviec

# Copy standard library and resources
COPY std/ /usr/local/lib/ovie/std/
COPY examples/ /usr/local/lib/ovie/examples/
COPY docs/ /usr/local/lib/ovie/docs/

# Create symlink: ovie -> oviec (same binary)
RUN ln -s /usr/local/bin/oviec /usr/local/bin/ovie

# Set environment
ENV OVIE_HOME=/usr/local/lib/ovie
ENV OVIE_STD=/usr/local/lib/ovie/std
ENV PATH="/usr/local/bin:$PATH"

# Default working directory
WORKDIR /workspace
VOLUME ["/workspace"]

USER ovie

# Verify installation
RUN oviec --version

ENTRYPOINT ["oviec"]
CMD ["--help"]
