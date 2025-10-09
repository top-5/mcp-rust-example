use anyhow::{Context, Result};
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use configparser::ini::Ini;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::path::Path;

const CONFIG_PATH: &str = "etc/config.ini";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String, // Subject (username)
    pub iat: i64,    // Issued at
    pub exp: i64,    // Expiry
    pub iss: String, // Issuer
}

pub struct AuthConfig {
    pub jwt_secret: String,
    pub authorized_tokens: std::collections::HashMap<String, String>,
}

impl AuthConfig {
    pub fn load() -> Result<Self> {
        if !Path::new(CONFIG_PATH).exists() {
            anyhow::bail!(
                "Config file not found at: {}\n\
                 Please create it or disable authentication.",
                CONFIG_PATH
            );
        }

        let mut conf = Ini::new();
        conf.load(CONFIG_PATH)
            .map_err(|e| anyhow::anyhow!("Failed to load config file: {}", e))?;

        let jwt_secret = conf
            .get("server", "jwt_secret")
            .context("JWT secret not found in config")?;

        let mut authorized_tokens = std::collections::HashMap::new();

        // Get all keys from auth section
        if let Some(auth_map) = conf.get_map_ref().get("auth") {
            for (username, token) in auth_map.iter() {
                if !username.starts_with('#') {
                    if let Some(token_value) = token {
                        authorized_tokens.insert(username.to_string(), token_value.to_string());
                    }
                }
            }
        }

        Ok(Self {
            jwt_secret,
            authorized_tokens,
        })
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenClaims> {
        // First check if token exists in authorized list
        let is_authorized = self.authorized_tokens.values().any(|t| t == token);

        if !is_authorized {
            anyhow::bail!("Token not in authorized list");
        }

        // Then verify JWT signature and expiry
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&["mcp-rust-example"]);

        let token_data = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )?;

        Ok(token_data.claims)
    }
}

/// Axum middleware for bearer token authentication
pub async fn auth_middleware(
    auth_config: axum::extract::State<std::sync::Arc<AuthConfig>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Extract bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify token
    match auth_config.verify_token(token) {
        Ok(claims) => {
            tracing::info!(
                "Authenticated request from user: {} (issued: {})",
                claims.sub,
                chrono::DateTime::from_timestamp(claims.iat, 0)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_else(|| "unknown".to_string())
            );
            Ok(next.run(req).await)
        }
        Err(e) => {
            tracing::warn!("Authentication failed: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
