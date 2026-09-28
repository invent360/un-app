//! Secrets management module
//!
//! Provides secure access to secrets from various sources:
//! - Environment variables (with redaction in logs)
//! - Files (for Docker secrets, Kubernetes secrets)
//! - Future: AWS Secrets Manager, HashiCorp Vault, etc.
//!
//! Key features:
//! - Type-safe secret access
//! - Automatic redaction in Debug output
//! - Validation of required secrets at startup
//! - Secure memory handling

use std::fmt;
use std::path::Path;
use thiserror::Error;

/// Secret management errors
#[derive(Error, Debug)]
pub enum SecretsError {
    #[error("Required secret not found: {0}")]
    NotFound(String),
    #[error("Failed to read secret file: {0}")]
    FileReadError(String),
    #[error("Invalid secret format: {0}")]
    InvalidFormat(String),
    #[error("Secret validation failed: {0}")]
    ValidationFailed(String),
}

/// A secret value that redacts itself in Debug output
#[derive(Clone)]
pub struct Secret {
    value: String,
    name: String,
}

impl Secret {
    /// Create a new secret
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Get the secret value
    pub fn expose(&self) -> &str {
        &self.value
    }

    /// Get the secret name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if the secret is empty
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Get the length of the secret
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Secret({}: [REDACTED])", self.name)
    }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED:{}]", self.name)
    }
}

/// Secrets manager for loading and accessing secrets
pub struct SecretsManager {
    /// Base path for file-based secrets (e.g., /run/secrets)
    secrets_path: Option<String>,
}

impl Default for SecretsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretsManager {
    /// Create a new secrets manager
    pub fn new() -> Self {
        Self {
            secrets_path: std::env::var("SECRETS_PATH").ok(),
        }
    }

    /// Create with a specific secrets path
    pub fn with_path(path: impl Into<String>) -> Self {
        Self {
            secrets_path: Some(path.into()),
        }
    }

    /// Get a required secret, failing if not found
    pub fn get_required(&self, name: &str) -> Result<Secret, SecretsError> {
        self.get(name)
            .ok_or_else(|| SecretsError::NotFound(name.to_string()))
    }

    /// Get an optional secret
    pub fn get(&self, name: &str) -> Option<Secret> {
        // Priority: File-based secrets > Environment variables

        // Try file-based secret first (Docker/Kubernetes secrets)
        if let Some(ref base_path) = self.secrets_path {
            let secret_path = Path::new(base_path).join(name);
            if secret_path.exists() {
                if let Ok(value) = std::fs::read_to_string(&secret_path) {
                    let trimmed = value.trim().to_string();
                    if !trimmed.is_empty() {
                        return Some(Secret::new(name, trimmed));
                    }
                }
            }
        }

        // Fall back to environment variable
        // Try both the exact name and uppercase version
        std::env::var(name)
            .or_else(|_| std::env::var(name.to_uppercase()))
            .ok()
            .filter(|v| !v.is_empty())
            .map(|value| Secret::new(name, value))
    }

    /// Get a secret with a default value
    pub fn get_or_default(&self, name: &str, default: &str) -> Secret {
        self.get(name).unwrap_or_else(|| Secret::new(name, default))
    }

    /// Validate that all required secrets are present
    pub fn validate_required(&self, names: &[&str]) -> Result<(), SecretsError> {
        let mut missing = Vec::new();

        for name in names {
            if self.get(name).is_none() {
                missing.push(*name);
            }
        }

        if missing.is_empty() {
            Ok(())
        } else {
            Err(SecretsError::NotFound(missing.join(", ")))
        }
    }
}

/// Application secrets configuration
/// Loaded once at startup and passed through app data
pub struct AppSecrets {
    /// Database connection URL
    pub database_url: Secret,
    /// CSRF secret key
    pub csrf_secret: Secret,
    /// Session secret key
    pub session_secret: Secret,
    /// Admin password hash
    pub admin_password_hash: Option<Secret>,
    /// API key for external services
    pub api_key: Option<Secret>,
}

impl AppSecrets {
    /// Load secrets from the environment/files
    pub fn load() -> Result<Self, SecretsError> {
        let manager = SecretsManager::new();

        // Required secrets
        let database_url = manager.get_required("DATABASE_URL")?;

        // Secrets with defaults for development
        let csrf_secret = manager.get_or_default(
            "CSRF_SECRET",
            // In production, this should fail if not set
            if cfg!(debug_assertions) {
                "dev-csrf-secret-change-in-production"
            } else {
                return Err(SecretsError::NotFound("CSRF_SECRET".to_string()));
            }
        );

        let session_secret = manager.get_or_default(
            "SESSION_SECRET",
            if cfg!(debug_assertions) {
                "dev-session-secret-change-in-production"
            } else {
                return Err(SecretsError::NotFound("SESSION_SECRET".to_string()));
            }
        );

        // Optional secrets
        let admin_password_hash = manager.get("ADMIN_PASSWORD_HASH");
        let api_key = manager.get("API_KEY");

        // Validate secret strength in production
        if !cfg!(debug_assertions) {
            if csrf_secret.len() < 32 {
                return Err(SecretsError::ValidationFailed(
                    "CSRF_SECRET must be at least 32 characters".to_string()
                ));
            }
            if session_secret.len() < 32 {
                return Err(SecretsError::ValidationFailed(
                    "SESSION_SECRET must be at least 32 characters".to_string()
                ));
            }
        }

        Ok(Self {
            database_url,
            csrf_secret,
            session_secret,
            admin_password_hash,
            api_key,
        })
    }

    /// Get the CSRF secret for middleware
    pub fn csrf_secret(&self) -> &str {
        self.csrf_secret.expose()
    }

    /// Get the session secret
    pub fn session_secret(&self) -> &str {
        self.session_secret.expose()
    }

    /// Get the database URL
    pub fn database_url(&self) -> &str {
        self.database_url.expose()
    }
}

impl fmt::Debug for AppSecrets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppSecrets")
            .field("database_url", &self.database_url)
            .field("csrf_secret", &self.csrf_secret)
            .field("session_secret", &self.session_secret)
            .field("admin_password_hash", &self.admin_password_hash.as_ref().map(|_| "[SET]"))
            .field("api_key", &self.api_key.as_ref().map(|_| "[SET]"))
            .finish()
    }
}

/// Generate a secure random secret suitable for CSRF/session keys
pub fn generate_secret(length: usize) -> String {
    use rand::Rng;

    let mut rng = rand::rng();
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_redaction() {
        let secret = Secret::new("api_key", "super-secret-value");

        // Should not expose the value in Debug
        let debug_output = format!("{:?}", secret);
        assert!(!debug_output.contains("super-secret-value"));
        assert!(debug_output.contains("REDACTED"));

        // Should not expose in Display
        let display_output = format!("{}", secret);
        assert!(!display_output.contains("super-secret-value"));
        assert!(display_output.contains("REDACTED"));

        // expose() should return the actual value
        assert_eq!(secret.expose(), "super-secret-value");
    }

    #[test]
    fn test_secrets_manager_env_var() {
        std::env::set_var("TEST_SECRET_123", "test-value");

        let manager = SecretsManager::new();
        let secret = manager.get("TEST_SECRET_123");

        assert!(secret.is_some());
        assert_eq!(secret.unwrap().expose(), "test-value");

        std::env::remove_var("TEST_SECRET_123");
    }

    #[test]
    fn test_generate_secret() {
        let secret1 = generate_secret(32);
        let secret2 = generate_secret(32);

        assert_eq!(secret1.len(), 32);
        assert_eq!(secret2.len(), 32);
        assert_ne!(secret1, secret2); // Should be random
    }

    #[test]
    fn test_validate_required() {
        std::env::set_var("REQUIRED_SECRET_1", "value1");
        std::env::set_var("REQUIRED_SECRET_2", "value2");

        let manager = SecretsManager::new();

        // Should pass when all required secrets exist
        assert!(manager.validate_required(&["REQUIRED_SECRET_1", "REQUIRED_SECRET_2"]).is_ok());

        // Should fail when a secret is missing
        assert!(manager.validate_required(&["REQUIRED_SECRET_1", "NONEXISTENT_SECRET"]).is_err());

        std::env::remove_var("REQUIRED_SECRET_1");
        std::env::remove_var("REQUIRED_SECRET_2");
    }
}
