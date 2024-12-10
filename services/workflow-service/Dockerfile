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
COPY services/workflow-service/Cargo.toml services/workflow-service/Cargo.lock services/workflow-service/
RUN cd services/workflow-service && cargo fetch

COPY services/workflow-service/src/ services/workflow-service/src/
RUN cd services/workflow-service && cargo build

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN groupadd workflow-service
RUN useradd -m -g workflow-service workflow-service
USER workflow-service

# Copy compiled binary from builder
COPY --from=builder --chown=workflow-service:workflow-service /usr/src/scac/services/workflow-service/target/debug/workflow-service /usr/local/bin/workflow-service

# Start the service
CMD ["/usr/local/bin/workflow-service"]
