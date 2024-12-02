# Define build arguments
ARG RUST_VERSION=1.82.0
ARG ALPINE_VERSION=3.20

# Stage 1: Builder
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS builder

# Install required build dependencies
RUN apk add --no-cache musl-dev gcc

# Set working directory
WORKDIR /usr/src/scac

# Copy dependencies and build in two steps to leverage caching
COPY shared/ shared/
COPY services/authentifcation-service/Cargo.toml services/authentifcation-service/Cargo.lock services/authentifcation-service/
RUN cd services/authentifcation-service && cargo fetch

COPY services/authentifcation-service/src/ services/authentifcation-service/src/
RUN cd services/authentifcation-service && cargo build --release

# Stage 2: Final Image
FROM alpine:${ALPINE_VERSION} AS runtime

# Install runtime dependencies if required
RUN apk add --no-cache libgcc

# Add non-root user and group
RUN addgroup -S authentifcation-service
RUN adduser -S authentifcation-service -G authentifcation-service
USER authentifcation-service

# Copy compiled binary from builder
COPY --from=builder /usr/src/scac/services/authentifcation-service/target/release/authentifcation-service /usr/local/bin/authentifcation-service

# Start the service
CMD ["/usr/local/bin/authentifcation-service"]
