//! GCS authentication via service account JWT

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

use crate::error::{Result, StorageError};

/// Service account key file structure
#[derive(Debug, Deserialize, Clone)]
pub struct ServiceAccountKey {
    pub client_email: String,
    pub private_key: String,
    pub token_uri: String,
}

impl ServiceAccountKey {
    /// Load from file path
    pub fn from_file(path: &str) -> Result<Self> {
        let contents = std::fs::read_to_string(path).map_err(|e| {
            StorageError::ConfigError(format!(
                "Failed to read service account key file '{}': {}",
                path, e
            ))
        })?;
        Self::from_json(&contents)
    }

    /// Parse from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| {
            StorageError::ConfigError(format!("Failed to parse service account key JSON: {}", e))
        })
    }
}

/// OAuth2 token response from Google
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[allow(dead_code)]
    pub expires_in: u64,
    #[allow(dead_code)]
    pub token_type: String,
}

/// JWT claims for service account authentication
#[derive(Serialize)]
struct JwtClaims {
    iss: String,
    scope: String,
    aud: String,
    iat: u64,
    exp: u64,
}

/// Cached access token
pub struct CachedToken {
    pub access_token: String,
    pub expires_at: std::time::Instant,
}

/// GCS authentication manager
pub struct GcsAuth {
    service_account: ServiceAccountKey,
    client: reqwest::Client,
    token_cache: RwLock<Option<CachedToken>>,
}

impl GcsAuth {
    /// GCS scope for read/write access
    const SCOPE: &'static str = "https://www.googleapis.com/auth/devstorage.read_write";

    /// Create a new auth manager
    pub fn new(service_account: ServiceAccountKey, client: reqwest::Client) -> Self {
        Self {
            service_account,
            client,
            token_cache: RwLock::new(None),
        }
    }

    /// Get the service account key
    pub fn service_account(&self) -> &ServiceAccountKey {
        &self.service_account
    }

    /// Get a valid access token (from cache or by refreshing)
    pub async fn get_access_token(&self) -> Result<String> {
        // Check cache first
        {
            let cache = self.token_cache.read().await;
            if let Some(ref cached) = *cache {
                // Return cached token if it's still valid (with 60s buffer)
                if cached.expires_at > std::time::Instant::now() + std::time::Duration::from_secs(60)
                {
                    return Ok(cached.access_token.clone());
                }
            }
        }

        // Refresh token
        let token = self.refresh_access_token().await?;

        // Update cache (tokens are valid for 1 hour)
        {
            let mut cache = self.token_cache.write().await;
            *cache = Some(CachedToken {
                access_token: token.clone(),
                expires_at: std::time::Instant::now() + std::time::Duration::from_secs(3500),
            });
        }

        Ok(token)
    }

    /// Generate a JWT for service account authentication
    fn generate_jwt(&self) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| StorageError::InternalError(format!("Time error: {}", e)))?
            .as_secs();

        let claims = JwtClaims {
            iss: self.service_account.client_email.clone(),
            scope: Self::SCOPE.to_string(),
            aud: self.service_account.token_uri.clone(),
            iat: now,
            exp: now + 3600, // 1 hour
        };

        let header = Header::new(Algorithm::RS256);

        let encoding_key =
            EncodingKey::from_rsa_pem(self.service_account.private_key.as_bytes()).map_err(
                |e| {
                    StorageError::ConfigError(format!(
                        "Invalid private key in service account: {}",
                        e
                    ))
                },
            )?;

        encode(&header, &claims, &encoding_key)
            .map_err(|e| StorageError::InternalError(format!("Failed to generate JWT: {}", e)))
    }

    /// Exchange JWT for access token
    async fn refresh_access_token(&self) -> Result<String> {
        let jwt = self.generate_jwt()?;

        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ];

        let response = self
            .client
            .post(&self.service_account.token_uri)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(StorageError::AuthenticationError(format!(
                "Failed to get access token ({}): {}",
                status, error_text
            )));
        }

        let token_response: TokenResponse = response.json().await?;
        Ok(token_response.access_token)
    }
}
