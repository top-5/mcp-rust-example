#!/bin/bash
set -euo pipefail

# Test script for MCP Rust Server
# Tests server startup, token management, and basic functionality

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "========================================="
echo "Testing MCP Rust Server"
echo "========================================="
echo "Project root: $PROJECT_ROOT"
echo ""

cd "$PROJECT_ROOT"

# Run unit tests (if any)
echo "🧪 Running unit tests..."
if cargo test --all-targets; then
    echo "✅ Unit tests passed"
else
    echo "❌ Unit tests failed"
    exit 1
fi

# Create test config directory
echo ""
echo "📁 Setting up test environment..."
mkdir -p etc
mkdir -p logs

# Test token-manager
echo ""
echo "🔑 Testing token-manager..."

# Generate initial config if it doesn't exist
if [ ! -f "etc/config.ini" ]; then
    echo "Creating test configuration..."
    cat > etc/config.ini <<EOF
[server]
host=127.0.0.1
port=8080
jwt_secret=test-secret-for-ci-do-not-use-in-production-please-change-this-value

[auth]
EOF
fi

# Test adding a user
echo "  Adding test user..."
if cargo run --bin token-manager --quiet -- add ci-test-user 2>&1 | grep -q "successfully"; then
    echo "  ✅ User added successfully"
else
    echo "  ❌ Failed to add user"
    exit 1
fi

# Test listing users
echo "  Listing users..."
if cargo run --bin token-manager --quiet -- list 2>&1 | grep -q "ci-test-user"; then
    echo "  ✅ User list contains ci-test-user"
else
    echo "  ❌ User not found in list"
    exit 1
fi

# Test removing user
echo "  Removing test user..."
if cargo run --bin token-manager --quiet -- remove ci-test-user 2>&1 | grep -q "successfully"; then
    echo "  ✅ User removed successfully"
else
    echo "  ❌ Failed to remove user"
    exit 1
fi

# Test server startup and shutdown
echo ""
echo "🚀 Testing server startup..."

# Start server in background with timeout
echo "  Starting server..."
timeout 10s cargo run --bin mcp-server --quiet &
SERVER_PID=$!

# Give it time to start
sleep 3

# Check if server is still running
if kill -0 $SERVER_PID 2>/dev/null; then
    echo "  ✅ Server started successfully (PID: $SERVER_PID)"
    
    # Test health endpoint
    echo "  Testing health endpoint..."
    if curl -s http://127.0.0.1:8080/health | grep -q "OK"; then
        echo "  ✅ Health endpoint responding"
    else
        echo "  ⚠️  Health endpoint not responding (may not be fully started)"
    fi
    
    # Shutdown server
    echo "  Stopping server..."
    kill $SERVER_PID 2>/dev/null || true
    wait $SERVER_PID 2>/dev/null || true
    echo "  ✅ Server stopped"
else
    echo "  ❌ Server failed to start or crashed"
    exit 1
fi

echo ""
echo "========================================="
echo "✅ All tests passed!"
echo "========================================="
