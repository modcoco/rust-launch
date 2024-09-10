## How to run or build
```bash
# Run
cargo run

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
sudo apt-get install gcc-aarch64-linux-gnu
aarch64-linux-gnu-gcc --version
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu --release

```

## Cross compilation help
This explanation covers the main steps and key components of cross-compiling for AArch64.
```bash
# Install base tool
sudo apt-get install build-essential gcc-aarch64-linux-gnu libc6-dev-arm64-cross -y
# Set aarach env
export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
export AR_aarch64_unknown_linux_gnu=aarch64-linux-gnu-ar
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
export OPENSSL_LIB_DIR=/usr/aarch64-linux-gnu/lib
export OPENSSL_INCLUDE_DIR=/usr/aarch64-linux-gnu/include
# Cross build aarach openssl
wget https://www.openssl.org/source/openssl-1.1.1l.tar.gz
./Configure linux-aarch64 --cross-compile-prefix=aarch64-linux-gnu-
make && sudo make install DESTDIR=/usr/aarch64-linux-gnu

# Use linux aarach base and aarach openssl
export OPENSSL_LIB_DIR=/usr/aarch64-linux-gnu/usr/local/lib
export OPENSSL_INCLUDE_DIR=/usr/aarch64-linux-gnu/usr/local/include

```

## Security
```bash
cargo install --locked cargo-deny
cargo deny init
cargo deny check

```
