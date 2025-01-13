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
COPY services/authentications-service/Cargo.toml services/authentications-service/Cargo.lock services/authentications-service/
RUN cd services/authentications-service && cargo fetch

COPY services/authentications-service/src/ services/authentications-service/src/
RUN cd services/authentications-service && cargo build

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN groupadd authentications-service
RUN useradd -m -g authentications-service authentications-service
USER authentications-service

# Copy compiled binary from builder
COPY --from=builder --chown=authentications-service:authentications-service /usr/src/scac/services/authentications-service/target/debug/authentications-service /usr/local/bin/authentications-service

# Start the service
CMD ["/usr/local/bin/authentications-service"]
