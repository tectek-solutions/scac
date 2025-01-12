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
COPY services/reactions-service/Cargo.toml services/reactions-service/Cargo.lock services/reactions-service/
RUN cd services/reactions-service && cargo fetch

COPY services/reactions-service/src/ services/reactions-service/src/
RUN cd services/reactions-service && cargo build

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN groupadd reactions-service
RUN useradd -m -g reactions-service reactions-service
USER reactions-service

# Copy compiled binary from builder
COPY --from=builder --chown=reactions-service:reactions-service /usr/src/scac/services/reactions-service/target/debug/reactions-service /usr/local/bin/reactions-service

# Start the service
CMD ["/usr/local/bin/reactions-service"]
