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
COPY services/authentifications-service/Cargo.toml services/authentifications-service/Cargo.lock services/authentifications-service/
RUN cd services/authentifications-service && cargo fetch

COPY services/authentifications-service/src/ services/authentifications-service/src/
RUN cd services/authentifications-service && cargo build

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN groupadd authentifications-service
RUN useradd -m -g authentifications-service authentifications-service
USER authentifications-service

# Copy compiled binary from builder
COPY --from=builder --chown=authentifications-service:authentifications-service /usr/src/scac/services/authentifications-service/target/debug/authentifications-service /usr/local/bin/authentifications-service

# Start the service
CMD ["/usr/local/bin/authentifications-service"]
