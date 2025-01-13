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
COPY services/actions-service/Cargo.toml services/actions-service/Cargo.lock services/actions-service/
RUN cd services/actions-service && cargo fetch

COPY services/actions-service/src/ services/actions-service/src/
RUN cd services/actions-service && cargo build

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN groupadd actions-service
RUN useradd -m -g actions-service actions-service
USER actions-service

# Copy compiled binary from builder
COPY --from=builder --chown=actions-service:actions-service /usr/src/scac/services/actions-service/target/debug/actions-service /usr/local/bin/actions-service

# Start the service
CMD ["/usr/local/bin/actions-service"]
