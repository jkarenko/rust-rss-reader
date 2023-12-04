FROM rust:bookworm as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
WORKDIR /opt/rss-reader-rust
COPY --from=builder /usr/src/app/target/release/rss-reader-rust .
COPY feeds.txt keywords.txt ./
EXPOSE 3030
ENV RUST_LOG=info
CMD ["/opt/rss-reader-rust/rss-reader-rust"]