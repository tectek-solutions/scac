# Define build arguments
ARG RUST_VERSION=1.82.0
ARG ALPINE_VERSION=3.20

# Stage 1: Builder
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS builder

# Install required build dependencies
RUN apk add --no-cache musl-dev gcc libpq libpq-dev postgresql-dev curl openssl-dev

# Set working directory
WORKDIR /usr/src/scac

# Copy dependencies and build in two steps to leverage caching
COPY shared/ shared/
COPY services/workflow-service/Cargo.toml services/workflow-service/Cargo.lock services/workflow-service/
RUN cd services/workflow-service && cargo fetch

COPY services/workflow-service/src/ services/workflow-service/src/
RUN cd services/workflow-service && cargo build --release

# Stage 2: Final Image
FROM alpine:${ALPINE_VERSION} AS runtime

# Install runtime dependencies if required
RUN apk add --no-cache libgcc

# Add non-root user and group
RUN addgroup -S workflow-service
RUN adduser -S workflow-service -G workflow-service
USER workflow-service

# Copy compiled binary from builder
COPY --from=builder /usr/src/scac/services/workflow-service/target/release/workflow-service /usr/local/bin/workflow-service

# Start the service
CMD ["/usr/local/bin/workflow-service"]
