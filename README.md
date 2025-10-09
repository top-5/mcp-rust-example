# MCP Rust HTTP Server# MCP Rust HTTP Server Example# MCP Rust HTTP Server Example



[![CI](https://github.com/top-5/mcp-rust-example/actions/workflows/ci.yml/badge.svg)](https://github.com/top-5/mcp-rust-example/actions/workflows/ci.yml)

[![Rust 2024](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)[![CI](https://github.com/top-5/mcp-rust-example/actions/workflows/ci.yml/badge.svg)](https://github.com/top-5/mcp-rust-example/actions/workflows/ci.yml)[![CI](https://github.com/YOUR_USERNAME/mcp-rust-example/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_USERNAME/mcp-rust-example/actions/workflows/ci.yml)



Example implementation of an MCP (Model Context Protocol) server in Rust using the [official MCP SDK](https://github.com/modelcontextprotocol/rust-sdk).[![Rust 2024](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)[![Rust 2024](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)



## What is this?[![MCP](https://img.shields.io/badge/MCP-2024--11--05-blue.svg)](https://github.com/modelcontextprotocol/rust-sdk)[![MCP](https://img.shields.io/badge/MCP-2024--11--05-blue.svg)](https://github.com/modelcontextprotocol/rust-sdk)



This is a working example of an MCP server that:[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

- Exposes tools that AI assistants (like GitHub Copilot) can call

- Uses HTTP transport with JWT authentication

- Demonstrates MCP Sampling (server requesting AI completions)

- Includes a background launcher and token managementProduction-ready MCP (Model Context Protocol) HTTP server using the official Rust SDK with JWT authentication and sampling support.This project demonstrates how to create a production-ready MCP (Model Context Protocol) HTTP server using the official Rust SDK, with full authentication, sampling support, and background process management.



## Features



- **4 Example Tools**: counter operations + sampling demo## Features## Features

- **JWT Authentication**: Bearer token security for MCP endpoints

- **MCP Sampling**: Server can request completions from the connected AI

- **Background Daemon**: Process launcher with PID management

- **Token Manager**: CLI tool for user/token management- ✅ **Official Rust MCP SDK** - Built with `rmcp` from https://github.com/modelcontextprotocol/rust-sdk### Core MCP Implementation

- **CI/CD**: GitHub Actions with build, test, and security audit

- ✅ **StreamableHTTP Transport** - MCP 2024-11-05 protocol over HTTP✅ **Official Rust MCP SDK**: Built with `rmcp` from https://github.com/modelcontextprotocol/rust-sdk  

## Quick Start

- ✅ **JWT Authentication** - Bearer token security with token manager CLI✅ **StreamableHTTP Transport**: Full MCP protocol over HTTP with streaming support (MCP 2024-11-05)  

```bash

# Start the server- ✅ **MCP Sampling** - Server can request LLM completions from client✅ **Tool System**: Automatic tool registration using `#[tool]` macros  

cargo run --bin mcp-server

- ✅ **Background Launcher** - Daemon with PID management and logging✅ **MCP Sampling**: Server can request LLM completions from the client (callback to AI models)

# Or as a background daemon

cargo run --bin launcher start- ✅ **VS Code Integration** - Works with VS Code's native MCP support

cargo run --bin launcher status

cargo run --bin launcher stop- ✅ **CI/CD Pipeline** - Automated builds, tests, and security scanning### Security & Authentication

```

🔐 **JWT Bearer Token Auth**: Protect MCP endpoints with token-based authentication  

## VS Code Integration

## Quick Start🔐 **Token Management CLI**: Built-in utility for user and token management  

The project includes `.vscode/mcp.json` with a test token. To use:

🔐 **Configurable Security**: Per-user tokens with expiry dates (2099-12-31)  

1. Start the server: `cargo run --bin mcp-server`

2. Reload VS Code MCP servers (Command Palette → "MCP: Reload")### 1. Build and Run🔐 **Selective Protection**: Web routes public, MCP routes authenticated

3. Use tools in Copilot: `@workspace /tools`



## Available Tools

```bash### Server Management

- `mcp_rustexam_increment` - Increment a counter

- `mcp_rustexam_get_counter` - Get counter value# Build the server🚀 **Background Launcher**: Full daemon with PID management and logging  

- `mcp_rustexam_reset_counter` - Reset counter to 0

- `mcp_rustexam_test_sampling` - Demo of MCP sampling (server → AI callback)cargo build --release🚀 **Process Control**: Start, stop, restart, status commands  



## Token Management🚀 **Combined Architecture**: MCP + Web server on same port (avoiding OAuth conflicts)  



```bash# Run directly🚀 **Graceful Shutdown**: Proper cleanup and signal handling

# Add a new user and generate JWT token

cargo run --bin token-manager -- add usernamecargo run --bin mcp-server



# List users### VS Code Integration

cargo run --bin token-manager -- list

# Or use the launcher (background daemon)💡 **Direct Integration**: Works with VS Code's native MCP support  

# Remove user

cargo run --bin token-manager -- remove usernamecargo run --bin launcher start💡 **Bearer Token Support**: Custom headers for authenticated requests  



# Rotate JWT secret (invalidates all tokens)cargo run --bin launcher status💡 **Hot Reload**: Reconnect to pick up server changes  

cargo run --bin token-manager -- rotate-secret

```cargo run --bin launcher stop💡 **Tool Discovery**: Automatic registration as `mcp_rustexam_*` tools



Tokens are stored in `etc/config.ini` (not committed to git).```



## Project Structure### Example Tools



```### 2. Configure VS Code🔧 **Counter Operations**: increment, get_counter, reset_counter  

src/

├── bin/🔧 **Sampling Demo**: test_sampling - demonstrates AI callback functionality  

│   ├── mcp_server.rs      # Main MCP server with tools

│   ├── launcher.rs        # Background daemon launcherThe `.vscode/mcp.json` file is already configured with a test token. Just:🔧 **Stateful Design**: Shared state using Arc<Mutex<T>>

│   ├── token_manager.rs   # Token management CLI

│   └── mcp_client.rs      # Test client

├── auth.rs                # JWT authentication middleware

└── lib.rs                 # Library root1. Start the server: `cargo run --bin mcp-server`### Developer Experience



.vscode/                   # VS Code MCP configuration (test token)2. Reload VS Code MCP servers📦 **Rust 2024 Edition**: Latest language features  

.github/workflows/ci.yml   # CI/CD pipeline

scripts/                   # Build and test scripts3. Use tools via GitHub Copilot chat📦 **Comprehensive Logging**: Tracing with multiple log levels  

```

📦 **Error Handling**: Proper ErrorData with codes and messages  

## Development

**Example MCP config:**📦 **Documentation**: Inline examples and troubleshooting guides

```bash

# Format + lint```json

cargo fmt && cargo clippy

{### CI/CD & Automation

# Run tests

cargo test  "servers": {🔄 **GitHub Actions**: Automated build, test, and security scanning  



# Run CI scripts locally    "rustexam": {🔄 **Multi-Platform**: Ubuntu and Windows (WSL) testing  

./scripts/build.sh

./scripts/test.sh      "url": "http://127.0.0.1:8080/mcp",🔄 **Build Scripts**: Reusable scripts for local and CI environments  

```

      "type": "http",🔄 **Cargo Caching**: Optimized CI builds with dependency caching  

## How It Works

      "headers": {🔄 **Security Audits**: Automated vulnerability scanning with cargo-audit

### 1. MCP Tools

        "Authorization": "Bearer <your-token>"

Tools are defined using the `#[tool]` macro:

      }## Quick Start

```rust

#[tool(description = "Increment the counter by 1")]    }

pub async fn increment(&self) -> Result<CallToolResult, ErrorData> {

    let mut counter = self.counter.lock().await;  }### 1. Start the MCP Server

    *counter += 1;

    Ok(CallToolResult::success(vec![Content::text(}

        format!("Counter: {}", *counter)

    )]))``````bash

}

```# Start server in background



### 2. MCP Sampling### 3. Manage Users/Tokenscargo run --bin launcher start



The server can request AI completions from the client:



```rust```bash# Check server status

#[tool(description = "Test MCP sampling")]

pub async fn test_sampling(# Add a new usercargo run --bin launcher status

    &self,

    context: rmcp::service::RequestContext<rmcp::service::RoleServer>,cargo run --bin token-manager -- add myuser

) -> Result<CallToolResult, ErrorData> {

    let response = context.peer.create_message(CreateMessageRequestParam {# Stop server

        messages: vec![SamplingMessage {

            role: Role::User,# List userscargo run --bin launcher stop

            content: Content::text("Say Hi"),

        }],cargo run --bin token-manager -- list```

        // ...

    }).await?;

    

    Ok(CallToolResult::success(vec![Content::text(# Remove a user### 2. Configure VS Code

        response.content.as_text().unwrap().text.clone()

    )]))cargo run --bin token-manager -- remove myuser

}

``````The server is pre-configured for VS Code integration. Two configuration methods:



### 3. Authentication



- MCP endpoint `/mcp` - Requires JWT bearer token## Available Tools**Method 1: User Settings** (`.vscode/settings.json`)

- Public endpoints `/health`, `/web`, `/dashboard` - No auth

```json

## Contributing

Access via GitHub Copilot in VS Code:{

See [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md).

  "mcp": {

## License

- `mcp_rustexam_increment` - Increment counter by 1    "servers": {

Apache 2.0

- `mcp_rustexam_get_counter` - Get current counter value      "mcp-rust-example": {

- `mcp_rustexam_reset_counter` - Reset counter to 0        "command": "cargo",

- `mcp_rustexam_test_sampling` - Test MCP sampling (AI callback demo)        "args": ["run", "--bin", "mcp-server", "--", "--port", "8080"],

        "cwd": "c:\\build\\top-5\\mcp-rust-example",

## Project Structure        "env": {},

        "transport": "http",

```        "url": "http://127.0.0.1:8080/mcp"

mcp-rust-example/      }

├── src/    }

│   ├── bin/  }

│   │   ├── mcp_server.rs       # Main MCP server}

│   │   ├── launcher.rs         # Background daemon```

│   │   ├── token_manager.rs    # Token management CLI

│   │   └── mcp_client.rs       # Test client**Method 2: Workspace Settings** (`.vscode/mcp.json`)

│   ├── auth.rs                 # JWT authentication```json

│   └── lib.rs{

├── .vscode/                    # VS Code MCP configuration  "servers": {

├── .github/workflows/ci.yml    # CI/CD pipeline    "mcp-rust-example": {

├── scripts/      "url": "http://127.0.0.1:8080/mcp",

│   ├── build.sh                # Build automation      "type": "http"

│   ├── test.sh                 # Test automation    }

│   └── setup-ci.sh             # Environment setup  },

├── etc/config.ini              # Server config (gitignored)  "inputs": []

└── logs/                       # Server logs (gitignored)}

``````



## Development### 3. Using with VS Code



```bash1. **Start the server**:

# Format code   ```bash

cargo fmt   cargo run --bin launcher start

   ```

# Lint

cargo clippy2. **Open VS Code** in this workspace



# Run tests3. **Access MCP tools** through:

cargo test   - GitHub Copilot Chat in Agent mode

   - Command Palette: "MCP: ..." commands

# Build and test (same as CI)   - Any VS Code extension that supports MCP

./scripts/build.sh && ./scripts/test.sh

```### 4. Available MCP Tools



## CI/CDThe server provides these example tools:



GitHub Actions workflow runs on every push:- **`increment`**: Increment the shared counter by 1

- **`get_counter`**: Get the current counter value  

- ✅ Build (debug + release)- **`reset_counter`**: Reset the counter to zero

- ✅ Tests (unit + integration)- **`test_sampling`**: Demo MCP sampling - asks the LLM to say "Hi" and identify itself

- ✅ Security audit (cargo-audit)

- ✅ Dependency check (cargo-outdated)#### MCP Sampling Example



Uses reusable scripts in `scripts/` directory for consistency between local and CI environments.The `test_sampling` tool demonstrates **MCP Sampling**, where the server requests AI completions from the client:



## Authentication```

You → Call test_sampling tool

The server uses JWT tokens for authentication:Server → Requests LLM completion from VS Code

VS Code/Copilot → Runs GPT-4/Claude/etc (your subscription)

- **MCP endpoint** (`/mcp`) - Requires authenticationServer ← Receives AI response

- **Web endpoints** (`/web`, `/health`, `/dashboard`) - PublicYou ← Gets formatted result with model info

```

Tokens are stored in `etc/config.ini` (not committed to git).

**Key benefit**: The server uses YOUR AI access (Copilot, API keys) without needing its own LLM credentials!

## MCP Sampling

## Manual Server Control

The `test_sampling` tool demonstrates MCP sampling - the server can request completions from the connected AI:

### Run server directly (foreground):

```rust```bash

let response = context.peer.create_message(CreateMessageRequestParam {cargo run --bin mcp-server

    messages: vec![SamplingMessage {```

        role: Role::User,

        content: Content::text("Say Hi"),### Run server on different port/host:

    }],```bash

    // ...cargo run --bin mcp-server -- --port 3000 --host 0.0.0.0

}).await?;```

```

### Background launcher options:

This allows the MCP server to use AI capabilities without needing its own LLM API keys.```bash

# Start with custom port

## Contributingcargo run --bin launcher start --port 3000



See [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md) for development workflow and guidelines.# Check if running

cargo run --bin launcher status

## License

# Restart server

Apache 2.0cargo run --bin launcher restart


# Stop server
cargo run --bin launcher stop
```

## Authentication & Security

The server supports JWT-based bearer token authentication to prevent unauthorized access to MCP tools. This is **highly recommended** for production deployments.

### Quick Setup

1. **Add a user** and generate their token:
   ```bash
   cargo run --bin token-manager add alice
   ```
   
   This will:
   - Generate a JWT token for user "alice" (expires 2099-12-31)
   - Save the token to `etc/config.ini`
   - Display the VS Code configuration

2. **Configure VS Code** with the bearer token:
   ```json
   {
     "servers": {
       "rustexam": {
         "url": "http://127.0.0.1:8080/mcp",
         "type": "http",
         "headers": {
           "Authorization": "Bearer <your-token-here>"
         }
       }
     },
     "inputs": []
   }
   ```
   
   Add this to `.vscode/mcp.json` in your workspace.

3. **Start the server** - it will automatically load authentication from `etc/config.ini`

### Token Management Commands

```bash
# Add a new user
cargo run --bin token-manager add <username>

# List all authorized users
cargo run --bin token-manager list

# Show VS Code config for specific user
cargo run --bin token-manager show-config <username>

# Remove a user
cargo run --bin token-manager remove <username>

# Rotate JWT secret (invalidates all tokens)
cargo run --bin token-manager rotate-secret
```

### How It Works

1. **JWT Tokens**: Each user gets a signed JWT token containing:
   - Username (sub claim)
   - Issue timestamp (iat)
   - Expiry date: 2099-12-31 (exp)
   - Issuer: mcp-rust-example (iss)

2. **Server Validation**: On each MCP request, the server:
   - Checks if the bearer token exists in the authorized list
   - Validates JWT signature using the secret from config
   - Verifies token expiry date
   - Logs authenticated requests with username

3. **Configuration File** (`etc/config.ini`):
   ```ini
   [server]
   host = 127.0.0.1
   port = 8080
   jwt_secret = <64-character-random-string>

   [auth]
   alice = eyJ0eXAiOiJKV1QiLCJhbGc...
   bob = eyJ0eXAiOiJKV1QiLCJhbGc...
   ```

### Security Notes

- **Never commit** `etc/config.ini` to version control (it's in `.gitignore`)
- Tokens are long-lived (expires 2099) - rotate if compromised
- Web endpoints (`/web`, `/health`, `/dashboard`) remain **unauthenticated**
- Only the MCP endpoint (`/mcp`) requires authentication
- Use `rotate-secret` command if you suspect secret compromise

### Running Without Authentication

If `etc/config.ini` doesn't exist or has no users, the server runs **without authentication**:
```
⚠️  Authentication: DISABLED (no users configured)
```

This is useful for local development but **not recommended for production**.

## Implementation Highlights

### What We've Built

This repository demonstrates a **production-ready MCP server** with several advanced features:

#### 1. Official SDK Integration
- Uses the official `rmcp` crate from modelcontextprotocol/rust-sdk
- Implements `StreamableHttpService` for HTTP transport
- Proper session management with `LocalSessionManager`
- Full MCP 2024-11-05 protocol compliance

#### 2. Authentication System
- **JWT-based security**: HS256 signed tokens with expiry dates
- **Token manager CLI**: Complete user lifecycle management
- **Middleware architecture**: Axum middleware for bearer token validation
- **Dual-mode operation**: Runs with or without authentication
- **Selective protection**: Only MCP routes require auth, web routes public

#### 3. Advanced MCP Features
- **Sampling support**: Server can request LLM completions from clients
- **Tool macros**: Automatic registration with `#[tool]` and `#[tool_router]`
- **Request context**: Access to peer connections for callbacks
- **Stateful tools**: Shared state using `Arc<Mutex<T>>`

#### 4. Combined Server Architecture
- **Dual-purpose server**: MCP protocol + web endpoints on same port
- **Route isolation**: `/mcp` for protocol, `/web`, `/health`, `/dashboard` for HTTP
- **OAuth conflict resolution**: Avoiding root `/` route prevents VS Code OAuth errors
- **Authenticated MCP, public web**: Different security levels per route

#### 5. Process Management
- **Background daemon**: Full launcher with PID file management
- **Log rotation**: Separate stdout/stderr logging
- **Lifecycle control**: Start, stop, restart, status commands
- **Clean shutdown**: Proper signal handling and cleanup

#### 6. Developer Experience
- **Comprehensive logging**: Tracing with request/response tracking
- **Error handling**: Proper MCP error codes and structured errors
- **Documentation**: Inline examples and troubleshooting
- **Testing utilities**: Example client implementation included

### Key Technical Decisions

1. **Why Streamable HTTP?**
   - Works through firewalls and proxies
   - Standard HTTP infrastructure (load balancers, auth, caching)
   - SSE for server-to-client notifications
   - No WebSocket complexity

2. **Why JWT tokens?**
   - Stateless authentication (no session storage needed)
   - Easy to generate and validate
   - Includes expiry and claims
   - Can be used across multiple servers

3. **Why combined MCP + Web?**
   - Single port for easier deployment
   - Shared authentication middleware
   - Health checks for monitoring
   - Dashboard for debugging

4. **Why avoid root route `/`?**
   - VS Code MCP client attempts OAuth discovery on `/`
   - Returns 404 for OAuth metadata requests
   - Prevents "OAuth client registration" errors
   - Documented in troubleshooting section

## Development

### Project Structure
```
mcp-rust-example/
├── src/
│   ├── bin/
│   │   ├── mcp_server.rs      # Main MCP server with auth + sampling
│   │   ├── launcher.rs        # Background process manager (start/stop/status)
│   │   ├── mcp_client.rs      # Example MCP client for testing
│   │   └── token_manager.rs   # Token management CLI (add/list/remove users)
│   ├── auth.rs                # JWT authentication middleware
│   └── lib.rs                 # Library root (exports auth module)
├── .vscode/
│   ├── settings.json          # VS Code user settings (optional)
│   └── mcp.json               # MCP server configuration with bearer token
├── etc/
│   ├── config.ini             # Server config & auth tokens (gitignored)
│   └── README.md              # Configuration documentation
├── logs/                      # Server logs (auto-created)
│   ├── mcp-server.log         # stdout
│   └── mcp-server-error.log   # stderr
├── Cargo.toml                 # Dependencies and binary configurations
├── README.md                  # This file
├── LICENSE                    # Apache 2.0
└── .gitignore                 # Excludes logs and config.ini
```

### Binaries

The project includes 4 executable binaries:

1. **mcp-server**: Main MCP server with tools and authentication
2. **launcher**: Background process manager with PID tracking
3. **token-manager**: CLI for user and token management
4. **mcp-client**: Example client for testing the server

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

### Adding Tools with Sampling

For tools that need to call back to the LLM:

```rust
#[tool(description = "AI-powered tool")]
pub async fn ai_tool(
    &self,
    context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
) -> Result<CallToolResult, ErrorData> {
    // Request LLM completion from the client
    let response = context.peer.create_message(CreateMessageRequestParam {
        messages: vec![SamplingMessage {
            role: Role::User,
            content: Content::text("Your prompt here"),
        }],
        model_preferences: Some(ModelPreferences {
            hints: Some(vec![ModelHint {
                name: Some("gpt-4".to_string()),
            }]),
            cost_priority: Some(0.5),
            speed_priority: Some(0.8),
            intelligence_priority: Some(0.7),
        }),
        system_prompt: Some("System prompt".to_string()),
        include_context: Some(ContextInclusion::None),
        temperature: Some(0.7),
        max_tokens: 150,
        stop_sequences: None,
        metadata: None,
    }).await?;
    
    // Process the AI response
    Ok(CallToolResult::success(vec![Content::text(
        response.message.content.as_text()
            .map(|t| &t.text)
            .unwrap_or(&"No response".to_string())
            .clone()
    )]))
}
```

### Logs and Debugging

Server logs are written to:
- `logs/mcp-server.log` - Normal output
- `logs/mcp-server-error.log` - Error output

## CI/CD & Testing

### GitHub Actions Pipeline

The project includes a comprehensive CI/CD pipeline in `.github/workflows/ci.yml`:

**🔄 Build and Test Job** (ubuntu-latest)
- ✅ Code formatting check (`cargo fmt`)
- ✅ Linting with Clippy (`cargo clippy`)
- ✅ Debug and release builds
- ✅ Unit tests (`cargo test`)
- ✅ Integration tests (token-manager, server startup)
- ✅ Artifact uploads (binaries with 7-day retention)
- ✅ Cargo caching for faster builds

**🪟 WSL Test Job** (windows-latest)
- ✅ Tests on Windows with WSL Ubuntu-22.04
- ✅ Validates cross-platform compatibility
- ✅ Runs same build and test scripts

**🔒 Security Audit Job**
- ✅ Scans dependencies for known vulnerabilities
- ✅ Uses `cargo-audit`

**📦 Dependency Check Job**
- ✅ Identifies outdated dependencies
- ✅ Uses `cargo-outdated`

### Local Testing with Scripts

The `scripts/` directory contains reusable build and test scripts:

```bash
# Setup environment (first time only)
./scripts/setup-ci.sh

# Build the project
./scripts/build.sh

# Run all tests
./scripts/test.sh
```

**What the scripts do:**

`build.sh`:
- Checks code formatting
- Runs Clippy linter
- Builds in debug and release modes
- Lists binary sizes

`test.sh`:
- Runs unit tests
- Tests token-manager CLI (add/list/remove users)
- Starts server and validates health endpoint
- Comprehensive integration testing

`setup-ci.sh`:
- Installs Rust toolchain
- Installs rustfmt and clippy
- Installs system dependencies

See [`scripts/README.md`](scripts/README.md) for detailed documentation.

### Pre-commit Checks

Before committing, run:

```bash
# Format code
cargo fmt

# Fix linting issues
cargo clippy --fix

# Build and test
./scripts/build.sh && ./scripts/test.sh
```

### Continuous Integration

The workflow runs automatically on:
- Push to `main`, `master`, or `develop` branches
- Pull requests to these branches

View build status and artifacts in the GitHub Actions tab.

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

**Symptom:** VS Code shows a dialog saying:
```
Dynamic Client Registration not supported
The authorization server 'http://127.0.0.1:8080/' does not support automatic client registration.
```

**Root Cause:** You mapped the root route `/` which returns HTML content instead of MCP protocol responses. When VS Code's MCP client makes discovery requests to the base URL and receives HTML, it incorrectly assumes the server is an OAuth authorization server.

**Solution:**
1. **Remove the root route `/` handler** - This is the most common cause
2. Move web content to dedicated paths like `/web` or `/dashboard`  
3. Keep the root path available for MCP protocol discovery
4. Restart the server and VS Code

**Example of what causes the issue:**
```rust
// ❌ This triggers OAuth client registration:
let app = Router::new()
    .route("/", get(home_page))  // HTML at root confuses VS Code
    .nest_service("/mcp", mcp_service);

// ✅ This works correctly:
let app = Router::new()
    .route("/web", get(home_page))     // Web content on dedicated path
    .nest_service("/mcp", mcp_service); // MCP uninterrupted
```

### Server not starting
1. Check `logs/mcp-server-error.log`
2. Verify port is available
3. Check file permissions for log directory
## Summary

This repository provides a **complete, production-ready MCP server implementation** in Rust with:

### ✅ What's Included

- **4 Binaries**: Server, launcher, token manager, test client
- **JWT Authentication**: Complete token-based security system
- **MCP Sampling**: AI callbacks without server needing LLM access
- **Process Management**: Background daemon with full lifecycle control
- **VS Code Integration**: Direct integration with native MCP support
- **Comprehensive Docs**: Examples, troubleshooting, and architecture notes

### 🚀 Quick Commands

\\\ash
# Setup authentication
cargo run --bin token-manager add alice

# Start server in background
cargo run --bin launcher start

# Test sampling (calls back to your AI)
# In VS Code: Use tool mcp_rustexam_test_sampling

# Manage server
cargo run --bin launcher status
cargo run --bin launcher stop
\\\

### 🎓 Key Learnings

1. **Streamable HTTP** works better than stdio for production servers
2. **Avoid root route** / to prevent VS Code OAuth confusion
3. **MCP Sampling** lets servers use client's AI without credentials
4. **JWT tokens** provide stateless authentication
5. **Combined MCP+Web** on one port simplifies deployment

### 📚 Resources

- Official Rust SDK: https://github.com/modelcontextprotocol/rust-sdk
- MCP Specification: https://modelcontextprotocol.io
- VS Code MCP Docs: https://code.visualstudio.com/docs/copilot/copilot-extensibility-overview

### 🤝 Contributing

This is an example project demonstrating MCP patterns. Feel free to fork and adapt for your use case!

### 📄 License

Apache 2.0 - See [LICENSE](LICENSE) file for details.

---

**Built with** ❤️ **using the official** [Model Context Protocol Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)
