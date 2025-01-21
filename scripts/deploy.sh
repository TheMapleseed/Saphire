#!/bin/bash
set -e

# Check system requirements
if ! command -v cargo &> /dev/null; then
    echo "cargo not found. Please install Rust."
    exit 1
fi

# Build in release mode
cargo build --release

# Create necessary directories
mkdir -p ~/.config/mac-linux-kvm
mkdir -p ~/Documents/KVM_Logs

# Copy binary
sudo cp target/release/mac-linux-kvm /usr/local/bin/

# Set permissions
sudo chmod +x /usr/local/bin/mac-linux-kvm

echo "Deployment complete!" 
