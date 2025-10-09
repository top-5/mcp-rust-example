# MCP Rust Example Server

A Model Context Protocol (MCP) server implementation built with the [official Rust SDK](https://github.com/modelcontextprotocol/rust-sdk). This project demonstrates how to build a production-ready MCP server with HTTP transport, authentication, and both MCP protocol and web endpoints.

## What It Does

This server provides:

### 🔧 MCP Protocol Tools
- **increment** - Increments a shared counter by 1
- **get_counter** - Returns the current counter value  
- **reset_counter** - Resets the counter to zero
- **test_sampling** - Tests MCP sampling by asking the connected LLM to say "Hi"

### 🌐 Web Endpoints
- `GET /web` - Home page with server information
- `GET /health` - Health check endpoint (JSON)
- `GET /api/status` - Server status and configuration (JSON)
- `GET /dashboard` - Admin dashboard
- `POST /mcp` - MCP protocol endpoint (Streamable HTTP)

### 🔐 Authentication
- JWT-based bearer token authentication
- Token management CLI tool
- Per-user access control
- Optional authentication mode

## Quick Start

### Prerequisites
- Rust 1.70+ with 2024 edition support
- VS Code (optional, for MCP integration)

### 1. Build the Project
```powershell
cargo build --release
```

### 2. Set Up Authentication (Optional)

Create a configuration file and add a user:

```powershell
# Create initial config
New-Item -ItemType Directory -Force -Path etc
@"
[server]
jwt_secret = your-secret-key-here

[auth]
"@ | Out-File -FilePath etc/config.ini -Encoding UTF8

# Add a user and generate token
cargo run --bin token-manager add myusername
```

The token manager will generate a JWT token and show you how to configure VS Code.

### 3. Run the Server
```powershell
cargo run --bin mcp-server
```

By default, the server runs on `http://127.0.0.1:8080`

**Custom host/port:**
```powershell
cargo run --bin mcp-server -- --host 0.0.0.0 --port 3000
```

## Usage

### VS Code Integration

The server is pre-configured for VS Code MCP. Add to `.vscode/mcp.json`:

```json
{
  "servers": {
    "rustexam": {
      "url": "http://127.0.0.1:8080/mcp",
      "type": "http",
      "headers": {
        "Authorization": "Bearer YOUR_TOKEN_HERE"
      }
    }
  }
}
```

Tools will be available as:
- `mcp_rustexam_increment`
- `mcp_rustexam_get_counter`
- `mcp_rustexam_reset_counter`
- `mcp_rustexam_test_sampling`

### Web Browser

Visit `http://127.0.0.1:8080/web` in your browser for the web interface.

### Direct HTTP Calls

```powershell
# Health check
Invoke-RestMethod -Uri http://127.0.0.1:8080/health

# MCP protocol (with authentication)
$headers = @{
    "Authorization" = "Bearer YOUR_TOKEN_HERE"
    "Content-Type" = "application/json"
}
Invoke-RestMethod -Uri http://127.0.0.1:8080/mcp -Method POST -Headers $headers -Body '{...}'
```

## Token Management

The `token-manager` binary provides user management:

```powershell
# Add a new user
cargo run --bin token-manager add username

# List all users
cargo run --bin token-manager list

# Remove a user
cargo run --bin token-manager remove username

# Rotate JWT secret (invalidates all tokens)
cargo run --bin token-manager rotate-secret

# Show VS Code config for a user
cargo run --bin token-manager show-config username
```

## Project Structure

```
├── src/
│   ├── lib.rs              # Library root
│   ├── auth.rs             # JWT authentication & middleware
│   └── bin/
│       ├── mcp_server.rs   # Main MCP + web server
│       ├── token_manager.rs# Token management CLI
│       ├── launcher.rs     # Server launcher utility
│       └── mcp_client.rs   # Example MCP client
├── etc/
│   └── config.ini          # Server configuration (JWT, users)
├── logs/
│   └── *.log               # Server logs
├── scripts/
│   ├── build.sh            # Build script
│   ├── test.sh             # Test script
│   └── setup-ci.sh         # CI setup
└── .vscode/
    └── mcp.json            # VS Code MCP configuration
```

## Architecture

### MCP Protocol
- Built with the official [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk)
- Uses Streamable HTTP transport
- Protocol version: 2024-11-05
- Tool-based architecture with `#[tool]` macros

### Web Server
- Built with [Axum](https://github.com/tokio-rs/axum)
- Async runtime: Tokio
- Combined MCP + web endpoints in single server

### Authentication
- JWT tokens with HS256 algorithm
- Bearer token authentication via Axum middleware
- Tokens stored in INI config file
- Long-lived tokens (expires 2099-12-31)

### State Management
- Shared counter state using `Arc<Mutex<T>>`
- Thread-safe across async tasks
- Per-server instance state

## Development

### Running Tests
```powershell
cargo test
```

### Enable Debug Logging
```powershell
$env:RUST_LOG="debug"
cargo run --bin mcp-server
```

### Building for Production
```powershell
cargo build --release
```

Binaries will be in `target/release/`:
- `mcp-server.exe` - Main server
- `token-manager.exe` - User management
- `launcher.exe` - Launcher utility

## Configuration

Configuration is stored in `etc/config.ini`:

```ini
[server]
jwt_secret = your-secret-here

[auth]
user1 = eyJ0eXAiOiJKV1QiLCJhbGci...
user2 = eyJ0eXAiOiJKV1QiLCJhbGci...
```

### Running Without Authentication

Simply start the server without creating `etc/config.ini`. The server will run in open mode (no auth required).

## License

Apache-2.0

## Author

Top-5 <top-5@noname.org>

## Resources

- [Model Context Protocol](https://modelcontextprotocol.io/)
- [MCP Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [MCP Specification](https://spec.modelcontextprotocol.io/)
