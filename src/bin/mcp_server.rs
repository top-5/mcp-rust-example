use axum::{Json, middleware, response::Html};
use clap::Parser;
use mcp_rust_example::auth::{AuthConfig, auth_middleware};
use rmcp::{
    ServerHandler,
    handler::server::router::tool::ToolRouter,
    model::*,
    tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        StreamableHttpService, session::local::LocalSessionManager,
    },
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to bind the server to
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// Host to bind the server to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}

/// A simple MCP server with example tools
#[derive(Clone)]
pub struct McpExampleServer {
    counter: Arc<Mutex<i32>>,
    tool_router: ToolRouter<Self>,
}

#[tool_router(router = tool_router)]
impl McpExampleServer {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            tool_router: Self::tool_router(),
        }
    }

    /// Increment the counter by 1
    #[tool(description = "Increment the counter by 1")]
    pub async fn increment(&self) -> Result<CallToolResult, ErrorData> {
        let mut counter = self.counter.lock().await;
        *counter += 1;
        let value = *counter;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Counter incremented to: {}",
            value
        ))]))
    }

    /// Get the current counter value
    #[tool(description = "Get the current counter value")]
    pub async fn get_counter(&self) -> Result<CallToolResult, ErrorData> {
        let counter = self.counter.lock().await;
        let value = *counter;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Current counter value: {}",
            value
        ))]))
    }

    /// Reset the counter to zero
    #[tool(description = "Reset the counter to zero")]
    pub async fn reset_counter(&self) -> Result<CallToolResult, ErrorData> {
        let mut counter = self.counter.lock().await;
        *counter = 0;

        Ok(CallToolResult::success(vec![Content::text(
            "Counter reset to 0".to_string(),
        )]))
    }

    /// Test sampling by asking the LLM to say "Hi"
    #[tool(description = "Test MCP sampling by asking the connected LLM to say 'Hi'")]
    pub async fn test_sampling(
        &self,
        context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        tracing::info!("🔔 Sampling request initiated - asking LLM to say Hi");

        // Request the client to run an LLM completion
        let response = context
            .peer
            .create_message(CreateMessageRequestParam {
                messages: vec![SamplingMessage {
                    role: Role::User,
                    content: Content::text("Please say 'Hi' and tell me what model you are."),
                }],
                model_preferences: Some(ModelPreferences {
                    hints: Some(vec![
                        ModelHint {
                            name: Some("gpt-4".to_string()),
                        },
                        ModelHint {
                            name: Some("claude".to_string()),
                        },
                    ]),
                    cost_priority: Some(0.3),
                    speed_priority: Some(0.8),
                    intelligence_priority: Some(0.7),
                }),
                system_prompt: Some(
                    "You are a helpful assistant testing MCP sampling.".to_string(),
                ),
                include_context: Some(ContextInclusion::None),
                temperature: Some(0.7),
                max_tokens: 150,
                stop_sequences: None,
                metadata: None,
            })
            .await
            .map_err(|e| {
                ErrorData::new(
                    ErrorCode::INTERNAL_ERROR,
                    format!("Sampling request failed: {}", e),
                    None,
                )
            })?;

        tracing::info!(
            "✅ Sampling response received from model: {}",
            response.model
        );

        Ok(CallToolResult::success(vec![Content::text(format!(
            "📡 MCP Sampling Test Results:\n\nModel used: {}\nStop reason: {}\n\nResponse:\n{}",
            response.model,
            response
                .stop_reason
                .unwrap_or_else(|| "unknown".to_string()),
            response
                .message
                .content
                .as_text()
                .map(|t| &t.text)
                .unwrap_or(&"No text response".to_string())
        ))]))
    }
}

#[tool_handler]
impl ServerHandler for McpExampleServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "rustexam-server".to_string(),
                title: Some("Rust Example MCP Server".to_string()),
                version: "1.0.0".to_string(),
                website_url: None,
                icons: None,
            },
            instructions: Some(
                "This is an example MCP server built with the official Rust SDK. \
                 It provides counter operations, echo functionality, and basic shell command execution."
                    .to_string(),
            ),
        }
    }
}

// Web route handlers
async fn home_page() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>MCP Rust Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }
        .header { background: #f4f4f4; padding: 20px; border-radius: 5px; }
        .endpoint { background: #e8f5e8; padding: 10px; margin: 5px 0; border-radius: 3px; }
        .mcp { background: #e8e8f5; padding: 10px; margin: 5px 0; border-radius: 3px; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🚀 MCP Rust Server</h1>
        <p>A combined MCP + Web server built with the official Rust SDK</p>
    </div>
    
    <h2>🔧 MCP Protocol</h2>
    <div class="mcp">
        <strong>MCP Endpoint:</strong> <code>/mcp</code><br>
        <strong>Protocol:</strong> Streamable HTTP<br>
        <strong>Tools:</strong> increment, get_counter, reset_counter
    </div>
    
    <h2>🌐 Web API Endpoints</h2>
    <div class="endpoint"><a href="/health">GET /health</a> - Health check</div>
    <div class="endpoint"><a href="/api/status">GET /api/status</a> - Server status JSON</div>
    <div class="endpoint"><a href="/dashboard">GET /dashboard</a> - Admin dashboard</div>
    
    <h2>💡 How to Connect</h2>
    <p><strong>VS Code MCP:</strong> Already configured! Tools available as mcp_rustexam_*</p>
    <p><strong>Web Browser:</strong> You're here! Try the endpoints above</p>
</body>
</html>
    "#,
    )
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "MCP Rust Server",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "mcp_endpoint": "/mcp",
        "tools": ["increment", "get_counter", "reset_counter"]
    }))
}

async fn api_status() -> Json<serde_json::Value> {
    Json(json!({
        "server": "mcp-rust-server",
        "version": "1.0.0",
        "mcp": {
            "protocol": "streamable-http",
            "endpoint": "/mcp",
            "tools_count": 3
        },
        "web": {
            "endpoints": ["/web", "/health", "/api/status", "/dashboard"]
        }
    }))
}

async fn dashboard() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>MCP Server Dashboard</title>
    <style>
        body { font-family: monospace; margin: 40px; background: #1e1e1e; color: #d4d4d4; }
        .panel { background: #2d2d30; padding: 20px; margin: 10px 0; border-radius: 5px; }
        .status { color: #4ec9b0; }
        .endpoint { color: #dcdcaa; }
    </style>
</head>
<body>
    <h1>🎛️ MCP Server Dashboard</h1>
    
    <div class="panel">
        <h3>🟢 Server Status</h3>
        <div class="status">✅ MCP Server: Running</div>
        <div class="status">✅ Web Server: Running</div>
        <div class="status">✅ Tools: 3 loaded</div>
    </div>
    
    <div class="panel">
        <h3>🔧 MCP Tools</h3>
        <div class="endpoint">increment - Increment counter</div>
        <div class="endpoint">get_counter - Get counter value</div>
        <div class="endpoint">reset_counter - Reset counter</div>
    </div>
    
    <div class="panel">
        <h3>🌐 Web Endpoints</h3>
        <div class="endpoint">GET /web - Home page</div>
        <div class="endpoint">GET /health - Health check</div>
        <div class="endpoint">GET /api/status - Status API</div>
        <div class="endpoint">GET /dashboard - This dashboard</div>
        <div class="endpoint">POST /mcp - MCP protocol endpoint</div>
    </div>
</body>
</html>
    "#,
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting MCP Rust Example Server...");
    tracing::info!(
        "Using official Rust MCP SDK from https://github.com/modelcontextprotocol/rust-sdk"
    );

    // Load authentication configuration
    let auth_config = match AuthConfig::load() {
        Ok(config) => {
            tracing::info!(
                "🔐 Authentication enabled with {} authorized users",
                config.authorized_tokens.len()
            );
            Arc::new(config)
        }
        Err(e) => {
            tracing::warn!(
                "⚠️  Failed to load auth config: {}. Running WITHOUT authentication!",
                e
            );
            tracing::warn!(
                "⚠️  To enable authentication, create etc/config.ini using token-manager"
            );
            Arc::new(AuthConfig {
                jwt_secret: String::new(),
                authorized_tokens: std::collections::HashMap::new(),
            })
        }
    };

    let bind_address = format!("{}:{}", args.host, args.port);

    // Create the MCP service using the official SDK
    let service = StreamableHttpService::new(
        || Ok(McpExampleServer::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    // Create axum router with both MCP service and web routes
    // Note: Avoiding root route "/" to prevent MCP client confusion
    // MCP routes require authentication, web routes are public
    let has_auth = !auth_config.authorized_tokens.is_empty();

    let mcp_router = if has_auth {
        axum::Router::new()
            .nest_service("/mcp", service)
            .layer(middleware::from_fn_with_state(
                auth_config.clone(),
                auth_middleware,
            ))
    } else {
        axum::Router::new().nest_service("/mcp", service)
    };

    let app = axum::Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/api/status", axum::routing::get(api_status))
        .route("/dashboard", axum::routing::get(dashboard))
        .route("/web", axum::routing::get(home_page)) // Move home to /web to avoid conflict
        .merge(mcp_router);

    // Start the server
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;

    tracing::info!(
        "🚀 Combined MCP + Web Server running on http://{}",
        bind_address
    );
    tracing::info!("📋 MCP Protocol:");
    tracing::info!("   Endpoint: http://{}/mcp", bind_address);
    tracing::info!("   Tools: increment, get_counter, reset_counter");
    if has_auth {
        tracing::info!("   🔐 Authentication: ENABLED (Bearer token required)");
    } else {
        tracing::info!("   ⚠️  Authentication: DISABLED (no users configured)");
    }
    tracing::info!("🌐 Web Endpoints:");
    tracing::info!("   Home: http://{}/web", bind_address);
    tracing::info!("   Health: http://{}/health", bind_address);
    tracing::info!("   Dashboard: http://{}/dashboard", bind_address);
    tracing::info!("");
    if !has_auth {
        tracing::info!("💡 To enable authentication:");
        tracing::info!("   cargo run --bin token-manager add <username>");
    }
    tracing::info!("💡 VS Code MCP: Already configured as mcp_rustexam_* tools");
    tracing::info!(
        "💡 Web Browser: Visit http://{}/web for web interface",
        bind_address
    );
    tracing::info!("");
    tracing::info!("Press Ctrl+C to shutdown");

    // Serve with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen for Ctrl+C signal");
            tracing::info!("Shutting down server...");
        })
        .await?;

    Ok(())
}
