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
COPY services/authentification-service/Cargo.toml services/authentification-service/Cargo.lock services/authentification-service/
RUN cd services/authentification-service && cargo fetch

COPY services/authentification-service/src/ services/authentification-service/src/
RUN cd services/authentification-service && cargo build --release

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN groupadd authentification-service
RUN useradd -m -g authentification-service authentification-service
USER authentification-service

# Copy compiled binary from builder
COPY --from=builder --chown=authentification-service:authentification-service /usr/src/scac/services/authentification-service/target/release/authentification-service /usr/local/bin/authentification-service

# Start the service
CMD ["/usr/local/bin/authentification-service"]
