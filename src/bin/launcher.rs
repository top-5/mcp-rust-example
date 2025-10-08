use std::process::{Command, Stdio};
use std::fs::OpenOptions;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Action to perform
    #[arg(value_enum)]
    action: Action,
    
    /// Port to bind the server to
    #[arg(short, long, default_value = "8080")]
    port: u16,
    
    /// Host to bind the server to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Action {
    Start,
    Stop,
    Status,
    Restart,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Start => start_server(args.host, args.port)?,
        Action::Stop => stop_server()?,
        Action::Status => check_server_status()?,
        Action::Restart => {
            stop_server()?;
            std::thread::sleep(std::time::Duration::from_secs(2));
            start_server(args.host, args.port)?;
        }
    }

    Ok(())
}

fn start_server(host: String, port: u16) -> anyhow::Result<()> {
    if is_server_running()? {
        println!("🟢 MCP server is already running!");
        return Ok(());
    }

    println!("🚀 Starting MCP server in background...");
    
    // Create logs directory
    std::fs::create_dir_all("logs")?;
    
    // Setup log files
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/mcp-server.log")?;
    
    let err_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/mcp-server-error.log")?;

    // Start the server process
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--bin", "mcp-server", "--", "--host", &host, "--port", &port.to_string()])
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file))
        .stdin(Stdio::null());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        const DETACHED_PROCESS: u32 = 0x00000008;
        cmd.creation_flags(CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS);
    }

    let child = cmd.spawn()?;
    let pid = child.id();

    // Save PID to file for tracking
    std::fs::write("mcp-server.pid", pid.to_string())?;

    // Give the server a moment to start
    std::thread::sleep(std::time::Duration::from_secs(3));

    if is_server_running()? {
        println!("✅ MCP server started successfully!");
        println!("   URL: http://{}:{}/mcp", host, port);
        println!("   PID: {}", pid);
        println!("   Logs: logs/mcp-server.log");
        println!("   Errors: logs/mcp-server-error.log");
        println!("");
        println!("💡 Use 'cargo run --bin launcher status' to check server status");
        println!("💡 Use 'cargo run --bin launcher stop' to stop the server");
    } else {
        println!("❌ Failed to start server. Check logs/mcp-server-error.log for details");
    }

    Ok(())
}

fn stop_server() -> anyhow::Result<()> {
    let pid_file = PathBuf::from("mcp-server.pid");
    
    if !pid_file.exists() {
        println!("❌ No PID file found. Server might not be running or was started manually.");
        return Ok(());
    }

    let pid_str = std::fs::read_to_string(&pid_file)?;
    let pid: u32 = pid_str.trim().parse()?;

    println!("🛑 Stopping MCP server (PID: {})...", pid);

    #[cfg(windows)]
    {
        let output = Command::new("taskkill")
            .args(&["/F", "/PID", &pid.to_string()])
            .output()?;
        
        if output.status.success() {
            println!("✅ Server stopped successfully");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("⚠️  Warning: {}", stderr);
        }
    }

    #[cfg(not(windows))]
    {
        let output = Command::new("kill")
            .args(&["-TERM", &pid.to_string()])
            .output()?;
        
        if output.status.success() {
            println!("✅ Server stopped successfully");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("⚠️  Warning: {}", stderr);
        }
    }

    // Remove PID file
    std::fs::remove_file(&pid_file)?;

    Ok(())
}

fn check_server_status() -> anyhow::Result<()> {
    if is_server_running()? {
        let pid_file = PathBuf::from("mcp-server.pid");
        if pid_file.exists() {
            let pid_str = std::fs::read_to_string(&pid_file)?;
            let pid = pid_str.trim();
            println!("🟢 MCP server is running (PID: {})", pid);
            println!("   URL: http://127.0.0.1:8080/mcp");
            
            // Check if we can reach the server
            println!("   Checking connectivity...");
            match std::process::Command::new("powershell")
                .args(&["-c", "try { (Invoke-WebRequest -Uri 'http://127.0.0.1:8080/mcp' -Method GET -TimeoutSec 5).StatusCode } catch { 'Failed' }"])
                .output() {
                Ok(output) => {
                    let response_str = String::from_utf8_lossy(&output.stdout);
                    let response = response_str.trim();
                    if response.contains("405") || response.contains("200") {
                        println!("   ✅ Server is responding to HTTP requests");
                    } else {
                        println!("   ⚠️  Server process exists but may not be responding: {}", response);
                    }
                }
                Err(_) => {
                    println!("   ⚠️  Could not test server connectivity");
                }
            }
        } else {
            println!("🟡 A server process is running but not managed by launcher");
        }
    } else {
        println!("🔴 MCP server is not running");
    }

    Ok(())
}

fn is_server_running() -> anyhow::Result<bool> {
    let pid_file = PathBuf::from("mcp-server.pid");
    
    if !pid_file.exists() {
        return Ok(false);
    }

    let pid_str = std::fs::read_to_string(&pid_file)?;
    let pid = pid_str.trim();

    #[cfg(windows)]
    {
        let output = Command::new("tasklist")
            .args(&["/FI", &format!("PID eq {}", pid), "/FO", "CSV"])
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.contains(pid))
    }

    #[cfg(not(windows))]
    {
        let output = Command::new("ps")
            .args(&["-p", pid])
            .output()?;
        
        Ok(output.status.success())
    }
}