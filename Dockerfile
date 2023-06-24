FROM rust:1.70 as builder
WORKDIR /usr/src/prometheus_wasd_exporter
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/prometheus-wasd-exporter /usr/local/bin/
CMD ["prometheus-wasd-exporter"]
