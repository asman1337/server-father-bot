FROM rust:1.84-slim-bullseye AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/server-father-bot
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl1.1 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/server-father-bot/target/release/server-father-bot /usr/local/bin/server-father-bot
COPY --from=builder /usr/src/server-father-bot/.env.example /usr/local/bin/.env

WORKDIR /usr/local/bin

CMD ["server-father-bot"] 