FROM rust:1.67 as builder
WORKDIR app
COPY . .
RUN cargo install --path .
RUN cargo build --release --bin ec5

FROM debian:buster-slim as runtime
WORKDIR app
COPY --from=builder /app/target/release/ec5 /usr/local/bin
COPY --from=builder /app/ec2pricing.yaml /usr/local/bin
COPY --from=builder /app/docker-entrypoint.sh /
RUN chmod +x /docker-entrypoint.sh
ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["ec5"]
