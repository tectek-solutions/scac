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
COPY services/api-services-service/Cargo.toml services/api-services-service/Cargo.lock services/api-services-service/
RUN cd services/api-services-service && cargo fetch

COPY services/api-services-service/src/ services/api-services-service/src/
RUN cd services/api-services-service && cargo build

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN groupadd api-services-service
RUN useradd -m -g api-services-service api-services-service
USER api-services-service

# Copy compiled binary from builder
COPY --from=builder --chown=api-services-service:api-services-service /usr/src/scac/services/api-services-service/target/debug/api-services-service /usr/local/bin/api-services-service

# Start the service
CMD ["/usr/local/bin/api-services-service"]
