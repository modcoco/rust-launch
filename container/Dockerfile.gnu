FROM --platform=$BUILDPLATFORM rust:1.81.0 AS builder
ARG TARGETOS
ARG TARGETARCH
WORKDIR /workspace
COPY . .
RUN if [ "${TARGETARCH}" = "amd64" ]; then \
        export TARGET_TRIPLE=x86_64-unknown-linux-gnu; \
        rustup target add x86_64-unknown-linux-gnu; \
    elif [ "${TARGETARCH}" = "arm64" ]; then \
        apt-get update; \
        apt-get install build-essential gdb-multiarch -y; \
        apt-get install binutils-aarch64-linux-gnu gcc-aarch64-linux-gnu libc6-dev-arm64-cross -y; \
        export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc; \
        export AR_aarch64_unknown_linux_gnu=aarch64-linux-gnu-ar; \
        export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc; \
        export OPENSSL_LIB_DIR=/usr/aarch64-linux-gnu/lib; \
        export OPENSSL_INCLUDE_DIR=/usr/aarch64-linux-gnu/include; \
        wget https://www.openssl.org/source/openssl-1.1.1l.tar.gz; \
        tar -xzf openssl-1.1.1l.tar.gz; \
        (cd openssl-1.1.1l &&./Configure linux-aarch64 --cross-compile-prefix=aarch64-linux-gnu- && make && make install DESTDIR=/usr/aarch64-linux-gnu); \
        export OPENSSL_LIB_DIR=/usr/aarch64-linux-gnu/usr/local/lib; \
        export OPENSSL_INCLUDE_DIR=/usr/aarch64-linux-gnu/usr/local/include; \
        export TARGET_TRIPLE=aarch64-unknown-linux-gnu; \
        rustup target add aarch64-unknown-linux-gnu; \
    fi && \
    cargo update &&\
    cargo build --release --target ${TARGET_TRIPLE} && \
    cp /workspace/target/${TARGET_TRIPLE}/release/rust_boot /workspace/target/rust_boot

FROM debian:bookworm-20240904-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN useradd -m app
WORKDIR /home/app
COPY --from=builder /workspace/target/rust_boot /bin
COPY --from=builder /workspace/.kube .kube
COPY --from=builder /workspace/.env .env
RUN chown -R app:app /home/app
USER app
ENTRYPOINT ["/bin/rust_boot"]
