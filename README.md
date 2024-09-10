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

## Security
```bash
cargo install --locked cargo-deny
cargo deny init
cargo deny check

```

```bash
sudo apt-get install build-essential
sudo apt-get update
sudo apt-get install gcc-aarch64-linux-gnu libc6-dev-arm64-cross
export OPENSSL_DIR=/usr/lib/aarch64-linux-gnu
export OPENSSL_LIB_DIR=/usr/lib/aarch64-linux-gnu
export OPENSSL_INCLUDE_DIR=/usr/aarch64-linux-gnu/include



 

export TARGET=aarch64-unknown-linux-gnu
export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
export OPENSSL_LIB_DIR=/usr/aarch64-linux-gnu/lib
export OPENSSL_INCLUDE_DIR=/usr/aarch64-linux-gnu/include
export PKG_CONFIG_PATH=/usr/aarch64-linux-gnu/lib/pkgconfig

CROSS_COMPILE=aarch64-linux-gnu cargo build --release --target aarch64-unknown-linux-gnu

 wget https://www.openssl.org/source/openssl-1.1.1l.tar.gz


# 常见的 PKG_CONFIG_PATH
/usr/lib/pkgconfig
/usr/local/lib/pkgconfig
/usr/share/pkgconfig
# 查看默认搜索路径
pkg-config --variable pc_path pkg-config

```
