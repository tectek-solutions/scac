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
COPY services/trigger-service/Cargo.toml services/trigger-service/Cargo.lock services/trigger-service/
RUN cd services/trigger-service && cargo fetch

COPY services/trigger-service/src/ services/trigger-service/src/
RUN cd services/trigger-service && cargo build --release

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN groupadd trigger-service
RUN useradd -m -G trigger-service trigger-service
USER trigger-service

# Copy compiled binary from builder
COPY --from=builder --chown=trigger-service:trigger-service /usr/src/scac/services/trigger-service/target/release/trigger-service /usr/local/bin/trigger-service

# Start the service
CMD ["/usr/local/bin/trigger-service"]
