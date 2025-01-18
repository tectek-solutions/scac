# Define build arguments
ARG RUST_VERSION=1.82.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-slim-bookworm AS builder


# Install required build dependencies
RUN apt-get update
RUN apt-get install -y gcc g++ cmake make

# Set working directory
WORKDIR /usr/src/scac

# Copy dependencies and build in two steps to leverage caching
COPY shared/ shared/
COPY api-gateway/Cargo.toml api-gateway/Cargo.lock api-gateway/
RUN cd api-gateway && cargo fetch

COPY api-gateway/src/ api-gateway/src/
RUN cd api-gateway && cargo build

# Stage 2: Final Image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies if required
RUN apt-get update

# Add non-root user and group
RUN groupadd api-gateway
RUN useradd -m -g api-gateway api-gateway
USER api-gateway

# Copy compiled binary from builder
COPY --from=builder --chown=api-gateway:api-gateway /usr/src/scac/api-gateway/target/debug/api-gateway /usr/local/bin/api-gateway

# Start the service
CMD ["/usr/local/bin/api-gateway"]
