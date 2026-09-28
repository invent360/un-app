//! Application settings loaded from environment variables

use std::env;

/// Database configuration
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/uno_app".to_string()),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
        }
    }
}

/// Admin API configuration
pub struct AdminConfig {
    pub api_key: Option<String>,
}

impl AdminConfig {
    pub fn from_env() -> Self {
        Self {
            api_key: env::var("ADMIN_API_KEY").ok(),
        }
    }

    pub fn validate_key(&self, key: &str) -> bool {
        match &self.api_key {
            Some(expected) => expected == key,
            None => false,
        }
    }
}

/// Application settings
pub struct AppSettings {
    pub database: DatabaseConfig,
    pub admin: AdminConfig,
}

impl AppSettings {
    pub fn from_env() -> Self {
        Self {
            database: DatabaseConfig::from_env(),
            admin: AdminConfig::from_env(),
        }
    }
}
