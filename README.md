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
