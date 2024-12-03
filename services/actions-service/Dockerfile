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
COPY services/actions-service/Cargo.toml services/actions-service/Cargo.lock services/actions-service/
RUN cd services/actions-service && cargo fetch

COPY services/actions-service/src/ services/actions-service/src/
RUN cd services/actions-service && cargo build --release

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN addgroup --service actions-service
RUN adduser --service actions-service -G actions-service
USER actions-service

# Copy compiled binary from builder
COPY --from=builder --chown=actions-service:actions-service /usr/src/scac/services/actions-service/target/release/actions-service /usr/local/bin/actions-service

# Start the service
CMD ["/usr/local/bin/actions-service"]
