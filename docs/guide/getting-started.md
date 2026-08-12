# Installation

## Binary Download

Download the latest binary from [GitHub Releases](https://github.com/codecoradev/cosy/releases/latest).

### Linux

```bash
# x86_64
tar xzf cosy-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
sudo mv cosy /usr/local/bin/

# ARM64
tar xzf cosy-v0.1.0-aarch64-unknown-linux-gnu.tar.gz
sudo mv cosy /usr/local/bin/
```

### macOS (Apple Silicon)

```bash
tar xzf cosy-v0.1.0-aarch64-apple-darwin.tar.gz
sudo mv cosy /usr/local/bin/
```

### Windows

Download `cosy-v0.1.0-x86_64-pc-windows-msvc.zip`, extract, and add `cosy.exe` to your PATH.

### Verify

```bash
cosy --version
# cosy 0.1.0
```

## Docker

```bash
docker pull ghcr.io/codecoradev/cosy:latest
```

## Build from Source

Requires Rust 1.70+.

```bash
git clone https://github.com/codecoradev/cosy.git
cd cosy
cargo build --release
# Binary: target/release/cosy
```

## Verify Checksums

Each release includes `SHA256SUMS.txt`. Verify your download:

```bash
sha256sum -c SHA256SUMS.txt --ignore-missing
```
