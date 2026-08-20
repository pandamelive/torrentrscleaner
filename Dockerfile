FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/torrent-rs-cleaner ./
EXPOSE 8090
CMD ["./torrent-rs-cleaner"]
