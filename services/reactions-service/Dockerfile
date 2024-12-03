# Define build arguments
ARG RUST_VERSION=1.82.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

# Install required build dependencies
RUN apt-get update && apt-get install -y musl-dev gcc g++ cmake make libpq-dev libssl-dev

# Set working directory
WORKDIR /usr/src/scac

# Copy dependencies and build in two steps to leverage caching
COPY shared/ shared/
COPY services/reactions-service/Cargo.toml services/reactions-service/Cargo.lock services/reactions-service/
RUN cd services/reactions-service && cargo fetch

COPY services/reactions-service/src/ services/reactions-service/src/
RUN cd services/reactions-service && cargo build --release

# Stage 2: Final Image
FROM debian:slim-bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update && apt-get install -y libpq libssl

# Add non-root user and group
RUN addgroup -S reactions-service
RUN adduser -S reactions-service -G reactions-service
USER reactions-service

# Copy compiled binary from builder
COPY --from=builder --chown=reactions-service:reactions-service /usr/src/scac/services/reactions-service/target/release/reactions-service /usr/local/bin/reactions-service

# Start the service
CMD ["/usr/local/bin/reactions-service"]
