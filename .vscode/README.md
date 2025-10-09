# VS Code Configuration

This directory contains example VS Code configuration for the MCP Rust server.

## Files

### `mcp.json`
MCP server configuration for VS Code. Contains a **test token** for the `testuser` account.

**⚠️ This is a SAMPLE token - safe to commit to git.**

To use:
1. Start the MCP server: `cargo run --bin mcp-server`
2. Reload VS Code MCP servers
3. The server will be available as `rustexam`

### `settings.json`
VS Code workspace settings including:
- MCP server configuration
- Chat sampling permissions

## Generating Your Own Token

If you want to create a different user:

```bash
# Generate a new user and token
cargo run --bin token-manager -- add myuser

# Copy the token from the output and update mcp.json
```

## Security Note

The token in these files is a **test token** for local development. It's safe to commit because:
- Only works on localhost (127.0.0.1:8080)
- Generated for testing purposes
- Not used in production

For production deployments, use proper secrets management.
