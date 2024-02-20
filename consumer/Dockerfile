FROM rust:latest as builder

# Install build dependencies for librdkafka
RUN apt-get update && apt-get install -y cmake build-essential pkg-config libssl-dev libsasl2-dev zlib1g-dev libzstd-dev

WORKDIR /usr/src/consumer
COPY . .
RUN cargo build --release

FROM debian:latest

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 libsasl2-2 zlib1g libzstd1 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/consumer/target/release/consumer /usr/local/bin/consumer

ENV KAFKA_PASSWORD=pyt6rRH0Mw

CMD ["consumer"]