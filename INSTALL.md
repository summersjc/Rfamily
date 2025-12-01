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

#### macOS Security Notice

macOS may block the binary because it's not signed by a registered Apple developer. To run it, use one of these methods:

##### Method 1: Remove quarantine flag (Recommended)

```bash
xattr -d com.apple.quarantine rfamily
chmod +x rfamily
./rfamily --version
```

##### Method 2: Right-click to open

1. Right-click the `rfamily` binary in Finder
2. Select "Open"
3. Click "Open" in the security dialog that appears

##### Method 3: System Settings

1. Try to run `./rfamily` in Terminal
2. Go to **System Settings** → **Privacy & Security**
3. Scroll down to find the security message about `rfamily`
4. Click **"Open Anyway"**
5. Run `./rfamily` again and confirm

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
