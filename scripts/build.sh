#!/bin/bash
set -euo pipefail

# Build script for MCP Rust Server
# Can be run locally or in CI

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "========================================="
echo "Building MCP Rust Server"
echo "========================================="
echo "Project root: $PROJECT_ROOT"
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo ""

cd "$PROJECT_ROOT"

# Check formatting
echo "📋 Checking code formatting..."
if cargo fmt -- --check; then
    echo "✅ Code formatting OK"
else
    echo "❌ Code formatting failed"
    echo "Run: cargo fmt"
    exit 1
fi

# Run clippy (linter)
echo ""
echo "🔍 Running Clippy (linter)..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    echo "✅ Clippy checks passed"
else
    echo "❌ Clippy found issues"
    exit 1
fi

# Build all targets
echo ""
echo "🔨 Building all targets..."
if cargo build --all-targets --verbose; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

# Build in release mode
echo ""
echo "🚀 Building release binaries..."
if cargo build --release --all-targets; then
    echo "✅ Release build successful"
else
    echo "❌ Release build failed"
    exit 1
fi

# List built binaries
echo ""
echo "📦 Built binaries:"
ls -lh target/release/mcp-server 2>/dev/null || echo "  mcp-server (not found)"
ls -lh target/release/launcher 2>/dev/null || echo "  launcher (not found)"
ls -lh target/release/token-manager 2>/dev/null || echo "  token-manager (not found)"
ls -lh target/release/mcp-client 2>/dev/null || echo "  mcp-client (not found)"

echo ""
echo "========================================="
echo "✅ Build completed successfully!"
echo "========================================="
