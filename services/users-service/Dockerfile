# Define build arguments
ARG RUST_VERSION=1.82.0
ARG ALPINE_VERSION=3.20

# Stage 1: Builder
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS builder

# Install required build dependencies
RUN apk add --no-cache musl-dev gcc libpq libpq-dev

# Set working directory
WORKDIR /usr/src/scac

# Copy dependencies and build in two steps to leverage caching
COPY shared/ shared/
COPY services/users-service/Cargo.toml services/users-service/Cargo.lock services/users-service/
RUN cd services/users-service && cargo fetch

COPY services/users-service/src/ services/users-service/src/
RUN cd services/users-service && cargo build --release

# Stage 2: Final Image
FROM alpine:${ALPINE_VERSION} AS runtime

# Install runtime dependencies if required
RUN apk add --no-cache libgcc

# Add non-root user and group
RUN addgroup -S users-service
RUN adduser -S users-service -G users-service
USER users-service

# Copy compiled binary from builder
COPY --from=builder /usr/src/scac/services/users-service/target/release/users-service /usr/local/bin/users-service

# Start the service
CMD ["/usr/local/bin/users-service"]
