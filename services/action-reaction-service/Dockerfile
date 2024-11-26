# Define build arguments
ARG RUST_VERSION=1.82.0
ARG ALPINE_VERSION=3.20

# Stage 1: Builder
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS builder

# Install required build dependencies
RUN apk add --no-cache musl-dev gcc

# Set working directory
WORKDIR /usr/src/action-reaction-service

# Copy dependencies and build in two steps to leverage caching
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY src ./src
RUN cargo build --release

# Stage 2: Final Image
FROM alpine:${ALPINE_VERSION}

# Install runtime dependencies if required
RUN apk add --no-cache libgcc

# Add non-root user and group
RUN addgroup -S action-reaction-service
RUN adduser -S action-reaction-service -G action-reaction-service
USER action-reaction-service

# Copy compiled binary from builder
COPY --from=builder /usr/src/action-reaction-service/target/release/action-reaction-service /usr/local/bin/action-reaction-service

# Start the service
CMD ["/usr/local/bin/action-reaction-service"]

# Expose the service port
EXPOSE 8000
