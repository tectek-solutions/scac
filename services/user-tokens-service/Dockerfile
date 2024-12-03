# Define build arguments
ARG RUST_VERSION=1.82.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-bookworm AS builder

# Install required build dependencies
RUN apt-get update
RUN apt-get install -y musl-dev gcc g++ cmake make libpq-dev

# Set working directory
WORKDIR /usr/src/scac

# Copy dependencies and build in two steps to leverage caching
COPY shared/ shared/
COPY services/user-tokens-service/Cargo.toml services/user-tokens-service/Cargo.lock services/user-tokens-service/
RUN cd services/user-tokens-service && cargo fetch

COPY services/user-tokens-service/src/ services/user-tokens-service/src/
RUN cd services/user-tokens-service && cargo build --release

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN addgroup -S user-tokens-service
RUN adduser -S user-tokens-service -G user-tokens-service
USER user-tokens-service

# Copy compiled binary from builder
COPY --from=builder --chown=user-tokens-service:user-tokens-service /usr/src/scac/services/user-tokens-service/target/release/user-tokens-service /usr/local/bin/user-tokens-service

# Start the service
CMD ["/usr/local/bin/user-tokens-service"]
