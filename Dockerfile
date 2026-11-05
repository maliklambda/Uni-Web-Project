FROM rust:1.89 AS builder
WORKDIR /usr/src/query_visualizer
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*
EXPOSE 3000
COPY --from=builder /usr/local/cargo/bin/query_visualizer /usr/local/bin/query_visualizer
CMD ["query_visualizer"]
