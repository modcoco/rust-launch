FROM rust:1.80.1-slim-bullseye AS builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo update
RUN cargo build --release

FROM debian:bullseye-slim
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN useradd -m app
WORKDIR /home/app
COPY --from=builder /app/target/release/boot /bin
COPY --from=builder /app/.kube .kube
COPY --from=builder /app/.env .env
RUN chown -R app:app /home/app
USER app
CMD ["/bin/boot"]