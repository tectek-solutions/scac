# Define build arguments
ARG RUST_VERSION=1.82.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-bookworm AS runtime

# Install required build dependencies
RUN apt-get update && \
    apt-get install -y musl-dev gcc g++ cmake make libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/scac

# Copy only the migration-related files to leverage caching
COPY shared/database shared/database
RUN cargo install diesel_cli --no-default-features --features postgres

# Change working directory to the service
WORKDIR /usr/src/scac/shared/database

CMD ["diesel", "migration", "run"]
