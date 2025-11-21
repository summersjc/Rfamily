# Installation

## Pre-built Binaries

Download the appropriate binary for your platform from the [latest release](https://github.com/summersjc/Rfamily/releases/latest):

### Linux

**x86_64 (Intel/AMD):**
```bash
wget https://github.com/summersjc/Rfamily/releases/latest/download/rfamily-linux-x86_64.tar.gz
tar xzf rfamily-linux-x86_64.tar.gz
sudo mv rfamily /usr/local/bin/
```

### macOS
**Intel (x86_64):**
```bash
curl -LO https://github.com/summersjc/Rfamily/releases/latest/download/rfamily-macos-x86_64.tar.gz
tar xzf rfamily-macos-x86_64.tar.gz
sudo mv rfamily /usr/local/bin/
```

**Apple Silicon (M1/M2/M3):**
```bash
curl -LO https://github.com/summersjc/Rfamily/releases/latest/download/rfamily-macos-aarch64.tar.gz
tar xzf rfamily-macos-aarch64.tar.gz
sudo mv rfamily /usr/local/bin/
```

### Windows
Download [rfamily-windows-x86_64.zip](https://github.com/summersjc/Rfamily/releases/latest/download/rfamily-windows-x86_64.zip), extract it, and add the directory to your PATH.

## Build from Source

If you have Rust installed:

```bash
cargo install --git https://github.com/summersjc/Rfamily
```

Or clone and build:

```bash
git clone https://github.com/summersjc/Rfamily.git
cd Rfamily
cargo build --release
# Binary will be in target/release/rfamily
```

## Verify Installation

```bash
rfamily --version
rfamily --list-presets
```
