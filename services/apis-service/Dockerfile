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
COPY services/apis-service/Cargo.toml services/apis-service/Cargo.lock services/apis-service/
RUN cd services/apis-service && cargo fetch

COPY services/apis-service/src/ services/apis-service/src/
RUN cd services/apis-service && cargo build

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN groupadd apis-service
RUN useradd -m -g apis-service apis-service
USER apis-service

# Copy compiled binary from builder
COPY --from=builder --chown=apis-service:apis-service /usr/src/scac/services/apis-service/target/debug/apis-service /usr/local/bin/apis-service

# Start the service
CMD ["/usr/local/bin/apis-service"]
