# Define build arguments
ARG RUST_VERSION=1.82.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-bookworm AS builder

VOLUME [ "/usr/src/scac" ]

# Install required build dependencies
RUN apt-get update
RUN apt-get install -y musl-dev gcc g++ cmake make libpq-dev

# Set working directory
WORKDIR /usr/src/scac

# Copy dependencies and build in two steps to leverage caching
COPY shared/ shared/
COPY services/workflows-service/Cargo.toml services/workflows-service/Cargo.lock services/workflows-service/
RUN cd services/workflows-service && cargo fetch

COPY services/workflows-service/src/ services/workflows-service/src/
RUN cd services/workflows-service && cargo build

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN groupadd workflows-service
RUN useradd -m -g workflows-service workflows-service
USER workflows-service

# Copy compiled binary from builder
COPY --from=builder --chown=workflows-service:workflows-service /usr/src/scac/services/workflows-service/target/debug/workflows-service /usr/local/bin/workflows-service

# Start the service
CMD ["/usr/local/bin/workflows-service"]
