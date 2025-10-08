use rmcp::{
    model::*,
    transport::streamable_http_client::StreamableHttpClientTransport,
    ServiceExt,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// MCP server URL
    #[arg(short, long, default_value = "http://127.0.0.1:8080/mcp")]
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🔌 Connecting to MCP server at: {}", args.url);

    // Create transport
    let transport = StreamableHttpClientTransport::from_uri(&args.url);

    // Create client info
    let client_info = ClientInfo {
        protocol_version: ProtocolVersion::V_2024_11_05,
        capabilities: ClientCapabilities::default(),
        client_info: Implementation {
            name: "mcp-rust-example-client".to_string(),
            title: Some("MCP Rust Example Client".to_string()),
            version: "1.0.0".to_string(),
            website_url: None,
            icons: None,
        },
    };

    // Connect to server using the correct pattern
    let client = client_info.serve(transport).await.map_err(|e| {
        anyhow::anyhow!("Failed to connect to MCP server: {}", e)
    })?;

    tracing::info!("✅ Connected to MCP server!");

    // Get server info
    if let Some(server_info) = client.peer_info() {
        tracing::info!("📋 Server: {} v{}", 
            server_info.server_info.name,
            server_info.server_info.version
        );
        if let Some(instructions) = &server_info.instructions {
            tracing::info!("📝 Instructions: {}", instructions);
        }
    }

    // List available tools
    tracing::info!("🔧 Fetching available tools...");
    let tools_result = client.list_tools(Default::default()).await?;
    
    tracing::info!("📋 Available tools ({} total):", tools_result.tools.len());
    for tool in &tools_result.tools {
        tracing::info!("   - {}: {}", 
            tool.name, 
            tool.description.as_deref().unwrap_or("No description")
        );
    }

    // Test some tools
    tracing::info!("🧪 Testing tools...");

    // Test echo tool - skip for now since our server doesn't have this tool yet

    // Test counter operations
    tracing::info!("Testing counter operations...");
    
    // Get initial value
    let get_result = client.call_tool(CallToolRequestParam {
        name: "get_counter".into(),
        arguments: None,
    }).await?;
    
    if let Some(content) = get_result.content.first() {
        if let Some(text) = content.as_text() {
            tracing::info!("📊 {}", text.text);
        }
    }

    // Increment counter
    for i in 1..=3 {
        tracing::info!("Incrementing counter ({})", i);
        let inc_result = client.call_tool(CallToolRequestParam {
            name: "increment".into(),
            arguments: None,
        }).await?;
        
        if let Some(content) = inc_result.content.first() {
            if let Some(text) = content.as_text() {
                tracing::info!("✅ {}", text.text);
            }
        }
    }

    // Shell command test - skip for now since we removed it from server

    // Disconnect
    tracing::info!("🔌 Disconnecting from server...");
    client.cancel().await?;
    tracing::info!("✅ Client test completed successfully!");

    Ok(())
}