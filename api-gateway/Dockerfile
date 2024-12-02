# Define build arguments
ARG RUST_VERSION=1.82.0
ARG ALPINE_VERSION=3.20

# Stage 1: Builder
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS builder

# Install required build dependencies
RUN apk add --no-cache musl-dev gcc g++ cmake make

# Set working directory
WORKDIR /usr/src/scac

# Copy dependencies and build in two steps to leverage caching
COPY shared/ shared/
COPY api-gateway/Cargo.toml api-gateway/Cargo.lock api-gateway/
RUN cd api-gateway && cargo fetch

COPY api-gateway/src/ api-gateway/src/
RUN cd api-gateway && cargo build --release

# Stage 2: Final Image
FROM alpine:${ALPINE_VERSION} AS runtime

# Install runtime dependencies if required
RUN apk add --no-cache libgcc

# Add non-root user and group
RUN addgroup -S actions-service
RUN adduser -S actions-service -G actions-service
USER actions-service

# Copy compiled binary from builder
COPY --from=builder /usr/src/scac/api-gateway/target/release/actions-service /usr/local/bin/actions-service

# Start the service
CMD ["/usr/local/bin/actions-service"]
