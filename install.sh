#!/bin/bash

# Rfamily Installation Script
# This script builds and installs the rfamily binary

set -e

echo "=== Rfamily Installation ==="
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is not installed."
    echo "Please install from https://rustup.rs/"
    exit 1
fi

# Build release binary
echo "Building release binary..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Error: Build failed"
    exit 1
fi

# Ensure the binary is executable
chmod +x target/release/rfamily

echo ""
echo "✓ Build successful!"
echo ""

# Show binary info
echo "Binary information:"
ls -lh target/release/rfamily
echo ""

# Ask user where to install
echo "Installation options:"
echo "  1) Install to /usr/local/bin (requires sudo, system-wide)"
echo "  2) Install to ~/.local/bin (user only, no sudo)"
echo "  3) Skip installation (binary is in target/release/rfamily)"
echo ""
read -p "Choose option [1-3]: " choice

case $choice in
    1)
        echo ""
        echo "Installing to /usr/local/bin (requires sudo)..."
        sudo cp target/release/rfamily /usr/local/bin/
        sudo chmod +x /usr/local/bin/rfamily
        echo "✓ Installed to /usr/local/bin/rfamily"
        echo ""
        echo "You can now run: rfamily --help"
        ;;
    2)
        mkdir -p ~/.local/bin
        cp target/release/rfamily ~/.local/bin/
        chmod +x ~/.local/bin/rfamily
        echo "✓ Installed to ~/.local/bin/rfamily"
        echo ""
        if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
            echo "Note: Add ~/.local/bin to your PATH by adding this to your ~/.zshrc or ~/.bashrc:"
            echo '  export PATH="$HOME/.local/bin:$PATH"'
        else
            echo "You can now run: rfamily --help"
        fi
        ;;
    3)
        echo ""
        echo "Binary location: $(pwd)/target/release/rfamily"
        echo "You can run it directly: ./target/release/rfamily --help"
        ;;
    *)
        echo "Invalid option. Binary is available at: $(pwd)/target/release/rfamily"
        ;;
esac

echo ""
echo "=== Installation Complete ==="
echo ""
echo "Quick start:"
echo "  rfamily --list-presets              # List all 51 language presets"
echo "  rfamily -p japanese -c 1000 -o out.ged   # Generate Japanese family data"
echo "  rfamily --help                      # Show all options"
