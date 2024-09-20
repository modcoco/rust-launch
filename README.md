## How to run or build

```bash
# Run
cargo run

# Add new lib
cargo new [crate_name] --lib

# Show current cargo info
rustup update
rustup show

# Add toolchain, if the project contains a rust-toolchain.toml file, just use 'rustup show'
rustup toolchain install nightly-2024-09-01
rustup default nightly-2024-09-01
rustup component list --toolchain nightly-2024-09-01

# Format code
cargo fmt
cargo clippy

# Build x86_64
rustup target add x86_64-unknown-linux-gnu
cargo build --target x86_64-unknown-linux-gnu --release

# Build aarch64
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu --release

```

## Build image

### GNU

```bash
docker buildx build --platform linux/amd64,linux/arm64 \
    -t rust-launch:v1 \
    -f container/Dockerfile.gnu \
    --output type=oci,dest=rust-app_v1.tar \
    .

```

### MUSL

```bash
docker buildx build --platform linux/amd64,linux/arm64 \
    -t rust-launch:v1 \
    -f container/Dockerfile.musl \
    --output type=oci,dest=rust-app_v1.tar \
    .

```

## Cross compilation aarch64 help

This explanation covers the main steps and key components of cross-compiling for AArch64.

```bash
# Install base aarch64 tool
sudo apt-get install build-essential gdb-multiarch -y
sudo apt-get install binutils-aarch64-linux-gnu gcc-aarch64-linux-gnu libc6-dev-arm64-cross -y
aarch64-linux-gnu-gcc --version

# Ser aarach basic env
export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
export AR_aarch64_unknown_linux_gnu=aarch64-linux-gnu-ar
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

# Set aarach env, build aarach64 openssl, the openssl project need it.
export OPENSSL_LIB_DIR=/usr/aarch64-linux-gnu/lib
export OPENSSL_INCLUDE_DIR=/usr/aarch64-linux-gnu/include

# Cross build aarach openssl
wget https://www.openssl.org/source/openssl-1.1.1l.tar.gz
./Configure linux-aarch64 --cross-compile-prefix=aarch64-linux-gnu-
make && make install DESTDIR=/usr/aarch64-linux-gnu

# Use linux aarach base and aarach openssl, then build
export OPENSSL_LIB_DIR=/usr/aarch64-linux-gnu/usr/local/lib
export OPENSSL_INCLUDE_DIR=/usr/aarch64-linux-gnu/usr/local/include

```

## Security

```bash
cargo install --locked cargo-deny
cargo deny init
cargo deny check

```

## Prepare sqlx
```bash
cargo sqlx prepare -- --lib
cargo sqlx prepare -- --all-targets
```
