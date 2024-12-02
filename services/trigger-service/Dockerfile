# Define build arguments
ARG RUST_VERSION=1.82.0
ARG ALPINE_VERSION=3.20

# Stage 1: Builder
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS builder

# Install required build dependencies
RUN apk add --no-cache musl-dev gcc libpq libpq-dev

# Set working directory
WORKDIR /usr/src/scac

# Copy dependencies and build in two steps to leverage caching
COPY shared/ shared/
COPY services/trigger-service/Cargo.toml services/trigger-service/Cargo.lock services/trigger-service/
RUN cd services/trigger-service && cargo fetch

COPY services/trigger-service/src/ services/trigger-service/src/
RUN cd services/trigger-service && cargo build --release

# Stage 2: Final Image
FROM alpine:${ALPINE_VERSION} AS runtime

# Install runtime dependencies if required
RUN apk add --no-cache libgcc

# Add non-root user and group
RUN addgroup -S trigger-service
RUN adduser -S trigger-service -G trigger-service
USER trigger-service

# Copy compiled binary from builder
COPY --from=builder --chown=trigger-service:trigger-service /usr/src/scac/services/trigger-service/target/release/trigger-service /usr/local/bin/trigger-service

# Start the service
CMD ["/usr/local/bin/trigger-service"]
