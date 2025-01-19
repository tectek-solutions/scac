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
COPY services/about-service/Cargo.toml services/about-service/Cargo.lock services/about-service/
RUN cd services/about-service && cargo fetch

COPY services/about-service/src/ services/about-service/src/
RUN cd services/about-service && cargo build

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN groupadd about-service
RUN useradd -m -g about-service about-service
USER about-service

# Copy compiled binary from builder
COPY --from=builder --chown=about-service:about-service /usr/src/scac/services/about-service/target/debug/about-service /usr/local/bin/about-service

# Start the service
CMD ["/usr/local/bin/about-service"]
