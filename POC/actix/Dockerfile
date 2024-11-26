FROM rust:1.82-bookworm

RUN apt-get update && apt-get install -y \
    libsqlite3-dev \
    libssl-dev \
    pkg-config

WORKDIR /app/backend

COPY . .

EXPOSE 8000/tcp

CMD ["cargo", "run", "--release"]