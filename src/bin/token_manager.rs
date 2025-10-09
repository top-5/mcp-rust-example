use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use clap::{Parser, Subcommand};
use configparser::ini::Ini;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::path::Path;

const CONFIG_PATH: &str = "etc/config.ini";

#[derive(Parser)]
#[command(name = "token-manager")]
#[command(about = "Manage MCP server authentication tokens")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new user and generate their JWT token
    Add {
        /// Username to add
        username: String,
    },
    /// List all authorized users
    List,
    /// Remove a user
    Remove {
        /// Username to remove
        username: String,
    },
    /// Rotate the JWT secret (invalidates all existing tokens)
    RotateSecret,
    /// Show VS Code configuration for a user
    ShowConfig {
        /// Username to show config for
        username: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    sub: String, // Subject (username)
    iat: i64,    // Issued at
    exp: i64,    // Expiry (2099-12-31)
    iss: String, // Issuer
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { username } => add_user(&username)?,
        Commands::List => list_users()?,
        Commands::Remove { username } => remove_user(&username)?,
        Commands::RotateSecret => rotate_secret()?,
        Commands::ShowConfig { username } => show_vscode_config(&username)?,
    }

    Ok(())
}

fn load_config() -> Result<Ini> {
    if !Path::new(CONFIG_PATH).exists() {
        anyhow::bail!(
            "Config file not found at: {}\nPlease create it first.",
            CONFIG_PATH
        );
    }
    let mut conf = Ini::new();
    conf.load(CONFIG_PATH)
        .map_err(|e| anyhow::anyhow!("Failed to load config file: {}", e))?;
    Ok(conf)
}

fn save_config(conf: &Ini) -> Result<()> {
    conf.write(CONFIG_PATH)
        .map_err(|e| anyhow::anyhow!("Failed to save config file: {}", e))
}

fn get_jwt_secret(conf: &Ini) -> Result<String> {
    conf.get("server", "jwt_secret")
        .context("JWT secret not found in config")
}

fn generate_token(username: &str, secret: &str) -> Result<String> {
    // Token expires on December 31, 2099
    let exp = Utc
        .with_ymd_and_hms(2099, 12, 31, 23, 59, 59)
        .single()
        .expect("Invalid date")
        .timestamp();

    let claims = TokenClaims {
        sub: username.to_string(),
        iat: Utc::now().timestamp(),
        exp,
        iss: "mcp-rust-example".to_string(),
    };

    let header = Header::new(Algorithm::HS256);
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

fn add_user(username: &str) -> Result<()> {
    let mut conf = load_config()?;
    let secret = get_jwt_secret(&conf)?;

    // Check if user already exists
    if conf.get("auth", username).is_some() {
        anyhow::bail!(
            "User '{}' already exists. Use 'remove' first if you want to regenerate.",
            username
        );
    }

    // Generate token
    let token = generate_token(username, &secret)?;

    // Save to config
    conf.set("auth", username, Some(token.clone()));
    save_config(&conf)?;

    println!("✅ User '{}' added successfully!", username);
    println!();
    println!("📋 Token generated:");
    println!("{}", token);
    println!();
    println!("🔧 Add this to your .vscode/settings.json:");
    println!();
    print_vscode_config(&token);

    Ok(())
}

fn list_users() -> Result<()> {
    let conf = load_config()?;

    let auth_map = conf
        .get_map_ref()
        .get("auth")
        .context("No auth section found in config")?;

    println!("📋 Authorized Users:");
    println!();

    let mut users: Vec<_> = auth_map
        .iter()
        .filter(|(k, _)| !k.starts_with('#'))
        .collect();

    users.sort_by_key(|(k, _)| *k);

    if users.is_empty() {
        println!("  No users configured yet.");
        println!();
        println!("💡 Add a user with: cargo run --bin token-manager add <username>");
    } else {
        for (username, token_opt) in &users {
            if let Some(token) = token_opt {
                println!("  • {}", username);
                println!("    Token: {}...", &token[..20.min(token.len())]);
            }
        }
        println!();
        println!("Total: {} user(s)", users.len());
    }

    Ok(())
}

fn remove_user(username: &str) -> Result<()> {
    let mut conf = load_config()?;

    if conf.get("auth", username).is_none() {
        anyhow::bail!("User '{}' not found", username);
    }

    conf.set("auth", username, None);
    save_config(&conf)?;

    println!("✅ User '{}' removed successfully!", username);

    Ok(())
}

fn rotate_secret() -> Result<()> {
    println!("⚠️  WARNING: Rotating JWT secret will invalidate ALL existing tokens!");
    println!("   All users will need new tokens.");
    println!();
    print!("Continue? (yes/no): ");

    use std::io::{self, Write};
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() != "yes" {
        println!("Cancelled.");
        return Ok(());
    }

    let mut conf = load_config()?;

    // Generate new secret
    use rand::Rng;
    use rand::distr::Alphanumeric;
    let new_secret: String = rand::rng()
        .sample_iter(Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    conf.set("server", "jwt_secret", Some(new_secret.clone()));

    // Clear all user tokens
    let users_to_clear: Vec<String> = conf
        .get_map_ref()
        .get("auth")
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();

    for username in users_to_clear {
        conf.set("auth", &username, None);
    }

    save_config(&conf)?;

    println!("✅ JWT secret rotated successfully!");
    println!("   New secret: {}", new_secret);
    println!();
    println!("⚠️  All user tokens have been cleared.");
    println!("   Re-add users with: cargo run --bin token-manager add <username>");

    Ok(())
}

fn show_vscode_config(username: &str) -> Result<()> {
    let conf = load_config()?;

    let token = conf
        .get("auth", username)
        .context(format!("User '{}' not found", username))?;

    println!("🔧 VS Code Configuration for user '{}':", username);
    println!();
    print_vscode_config(&token);

    Ok(())
}

fn print_vscode_config(token: &str) {
    println!(
        r#"Add this to .vscode/mcp.json:

{{
  "servers": {{
    "rustexam": {{
      "url": "http://127.0.0.1:8080/mcp",
      "type": "http",
      "headers": {{
        "Authorization": "Bearer {}"
      }}
    }}
  }},
  "inputs": []
}}"#,
        token
    );
}
