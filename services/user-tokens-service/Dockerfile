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
COPY services/user-tokens-service/Cargo.toml services/user-tokens-service/Cargo.lock services/user-tokens-service/
RUN cd services/user-tokens-service && cargo fetch

COPY services/user-tokens-service/src/ services/user-tokens-service/src/
RUN cd services/user-tokens-service && cargo build --release

# Stage 2: Final Image
FROM alpine:${ALPINE_VERSION} AS runtime

# Install runtime dependencies if required
RUN apk add --no-cache libgcc

# Add non-root user and group
RUN addgroup -S user-tokens-service
RUN adduser -S user-tokens-service -G user-tokens-service
USER user-tokens-service

# Copy compiled binary from builder
COPY --from=builder /usr/src/scac/services/user-tokens-service/target/release/user-tokens-service /usr/local/bin/user-tokens-service

# Start the service
CMD ["/usr/local/bin/user-tokens-service"]
