# MCP Rust HTTP Server Example

This project demonstrates how to create an MCP (Model Context Protocol) HTTP server using the official Rust SDK.

## Features

✅ **Official Rust MCP SDK**: Built with `rmcp` from https://github.com/modelcontextprotocol/rust-sdk
✅ **StreamableHTTP Transport**: Full MCP protocol over HTTP with streaming support
✅ **Background Server**: Launcher utility for running server in background
✅ **VS Code Integration**: Direct integration with VS Code's MCP support
✅ **Example Tools**: Counter operations, echo, and basic shell commands

## Quick Start

### 1. Start the MCP Server

```bash
# Start server in background
cargo run --bin launcher start

# Check server status
cargo run --bin launcher status

# Stop server
cargo run --bin launcher stop
```

### 2. Configure VS Code

The server is pre-configured for VS Code integration. Two configuration methods:

**Method 1: User Settings** (`.vscode/settings.json`)
```json
{
  "mcp": {
    "servers": {
      "mcp-rust-example": {
        "command": "cargo",
        "args": ["run", "--bin", "mcp-server", "--", "--port", "8080"],
        "cwd": "c:\\build\\top-5\\mcp-rust-example",
        "env": {},
        "transport": "http",
        "url": "http://127.0.0.1:8080/mcp"
      }
    }
  }
}
```

**Method 2: Workspace Settings** (`.vscode/mcp.json`)
```json
{
  "servers": {
    "mcp-rust-example": {
      "url": "http://127.0.0.1:8080/mcp",
      "type": "http"
    }
  },
  "inputs": []
}
```

### 3. Using with VS Code

1. **Start the server**:
   ```bash
   cargo run --bin launcher start
   ```

2. **Open VS Code** in this workspace

3. **Access MCP tools** through:
   - GitHub Copilot Chat in Agent mode
   - Command Palette: "MCP: ..." commands
   - Any VS Code extension that supports MCP

### 4. Available MCP Tools

The server provides these tools:

- **`increment`**: Increment the counter by 1
- **`get_counter`**: Get the current counter value  
- **`reset_counter`**: Reset the counter to zero

## Manual Server Control

### Run server directly (foreground):
```bash
cargo run --bin mcp-server
```

### Run server on different port/host:
```bash
cargo run --bin mcp-server -- --port 3000 --host 0.0.0.0
```

### Background launcher options:
```bash
# Start with custom port
cargo run --bin launcher start --port 3000

# Check if running
cargo run --bin launcher status

# Restart server
cargo run --bin launcher restart

# Stop server
cargo run --bin launcher stop
```

## Development

### Project Structure
```
src/
├── bin/
│   ├── mcp_server.rs    # Main MCP server
│   └── launcher.rs      # Background launcher utility
├── .vscode/
│   ├── settings.json    # VS Code MCP configuration
│   └── mcp.json         # Alternative MCP config
└── logs/                # Server logs (created automatically)
    ├── mcp-server.log
    └── mcp-server-error.log
```

### Adding New Tools

Add tools to the `McpExampleServer` impl block:

```rust
#[tool(description = "Your tool description")]
pub async fn your_tool(&self) -> Result<CallToolResult, ErrorData> {
    // Your tool implementation
    Ok(CallToolResult::success(vec![Content::text("Result".to_string())]))
}
```

The `#[tool_router]` macro will automatically register the tool.

### Logs and Debugging

Server logs are written to:
- `logs/mcp-server.log` - Normal output
- `logs/mcp-server-error.log` - Error output

## Testing the MCP Protocol

You can test the MCP server using any MCP-compatible client:

### With VS Code:
1. Configure MCP server in VS Code settings
2. Use GitHub Copilot Agent mode
3. Access tools through Copilot chat

### Manual Testing:
The server implements the full MCP Streamable HTTP protocol at `http://127.0.0.1:8080/mcp`

## Architecture

This implementation uses:

- **rmcp**: Official Rust MCP SDK
- **StreamableHttpService**: HTTP transport with streaming
- **axum**: HTTP server framework
- **tokio**: Async runtime
- **Tool Router**: Automatic tool registration via macros

The server fully implements the MCP specification for HTTP transport, making it compatible with any MCP client including VS Code, Copilot, and custom implementations.

## Combining MCP with Web Routes

This server demonstrates how to successfully combine MCP protocol endpoints with regular web routes in a single server. The implementation includes:

**🔧 MCP Protocol:**
- Endpoint: `http://127.0.0.1:8080/mcp`
- Tools: `increment`, `get_counter`, `reset_counter`

**🌐 Web Endpoints:**
- Home: `http://127.0.0.1:8080/web` - Web interface
- Health: `http://127.0.0.1:8080/health` - JSON health check
- Status: `http://127.0.0.1:8080/api/status` - Server status API
- Dashboard: `http://127.0.0.1:8080/dashboard` - Admin dashboard

### ⚠️ Important Route Mapping Caveat

**DO NOT map the root route "/" when combining MCP with web content!**

When you add a route handler for the root path `/`, VS Code's MCP client will attempt OAuth client ID authentication instead of using the configured MCP endpoint. This happens because:

1. VS Code MCP client makes discovery requests to the base URL
2. When `/` returns HTML content instead of MCP protocol responses, the client gets confused
3. The client falls back to OAuth dynamic registration mode
4. This causes authentication errors and breaks MCP tool functionality

**Solution:** Use dedicated paths for web content (e.g., `/web`, `/dashboard`) and keep the root path available for MCP protocol discovery.

```rust
// ❌ This breaks VS Code MCP integration:
let app = Router::new()
    .route("/", get(home_page))  // Causes OAuth client registration!
    .nest_service("/mcp", mcp_service);

// ✅ This works correctly:
let app = Router::new()
    .route("/web", get(home_page))     // Web content on dedicated path
    .route("/health", get(health_check))
    .route("/dashboard", get(dashboard))
    .nest_service("/mcp", mcp_service); // MCP remains uninterrupted
```

This allows you to build rich web interfaces and APIs alongside MCP functionality without breaking VS Code integration.

## Troubleshooting

### Port already in use
```bash
# Check what's using port 8080
netstat -ano | findstr :8080

# Kill existing server
cargo run --bin launcher stop
```

### VS Code not finding tools
1. Verify server is running: `cargo run --bin launcher status`  
2. Check VS Code MCP configuration
3. Restart VS Code
4. Check server logs for errors

### OAuth Client Registration Error
If you see OAuth client registration errors in VS Code:
1. **Check if you mapped the root route "/"** - This is the most common cause
2. Remove any route handlers for the root path
3. Move web content to dedicated paths like `/web` or `/dashboard`
4. Restart the server and VS Code

### Server not starting
1. Check `logs/mcp-server-error.log`
2. Verify port is available
3. Check file permissions for log directory