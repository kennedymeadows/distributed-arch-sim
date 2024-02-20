FROM rust:latest as builder

WORKDIR /usr/src/client
COPY . .
RUN cargo install --path .

FROM debian:latest
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/client /usr/local/bin/client
CMD ["client"]
