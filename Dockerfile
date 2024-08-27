FROM rust:slim-bookworm AS builder

# Dependencies
RUN apt-get update \
    && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    curl \
    && apt-get clean

WORKDIR /app

COPY . .

RUN cargo build --package api --bin main

FROM rust:slim-buster

# Dependencies
RUN apt-get update \
    && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    openssl \
    curl \
    && apt-get clean

# Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Take compiled binary and necessary files
COPY --from=builder /app/target/debug/main /usr/local/bin/main
COPY --from=builder /app/infrastructure/migrations /home/migrations
COPY --from=builder /app/wait-for-it.sh /usr/local/bin/wait-for-it.sh

RUN chmod +x /usr/local/bin/wait-for-it.sh

WORKDIR /home/migrations

# Check DB & Start API
CMD diesel migration run \
    && main