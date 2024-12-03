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
COPY services/users-service/Cargo.toml services/users-service/Cargo.lock services/users-service/
RUN cd services/users-service && cargo fetch

COPY services/users-service/src/ services/users-service/src/
RUN cd services/users-service && cargo build --release

# Stage 2: Final Image
FROM debian:bookworm AS runtime

# Install runtime dependencies if required
RUN apt-get update
RUN apt-get install -y libpq5

# Add non-root user and group
RUN addgroup --service users-service
RUN adduser --service --ingroup users-service users-service
USER users-service

# Copy compiled binary from builder
COPY --from=builder --chown=users-service:users-service /usr/src/scac/services/users-service/target/release/users-service /usr/local/bin/users-service

# Start the service
CMD ["/usr/local/bin/users-service"]
