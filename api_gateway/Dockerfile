FROM rust:latest as builder

# Install build dependencies for librdkafka
RUN apt-get update && apt-get install -y cmake build-essential pkg-config libssl-dev libsasl2-dev zlib1g-dev libzstd-dev

WORKDIR /usr/src/api_gateway
COPY . .
RUN cargo build --release

FROM debian:latest

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 libsasl2-2 zlib1g libzstd1 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/api_gateway/target/release/api_gateway /usr/local/bin/api_gateway

ENV KAFKA_PASSWORD=pyt6rRH0Mw

EXPOSE 8080
CMD ["api_gateway"]

