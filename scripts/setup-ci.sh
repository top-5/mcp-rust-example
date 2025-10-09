#!/bin/bash
set -euo pipefail

# CI Setup script
# Prepares the environment for building and testing

echo "========================================="
echo "Setting up CI Environment"
echo "========================================="

# Install Rust if not present
if ! command -v rustc &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Update Rust toolchain
echo "🔄 Updating Rust toolchain..."
rustup update stable
rustup default stable

# Install required components
echo "🔧 Installing Rust components..."
rustup component add rustfmt clippy

# Install system dependencies
echo "📦 Installing system dependencies..."
if command -v apt-get &> /dev/null; then
    # Ubuntu/Debian
    sudo apt-get update
    sudo apt-get install -y curl build-essential pkg-config libssl-dev
elif command -v yum &> /dev/null; then
    # RHEL/CentOS
    sudo yum install -y curl gcc gcc-c++ make openssl-devel
fi

# Verify installation
echo ""
echo "✅ Environment setup complete!"
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo "Rustfmt: $(rustfmt --version)"
echo "Clippy: $(cargo clippy --version)"
