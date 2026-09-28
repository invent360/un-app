//! Client configuration.

use std::time::Duration;

/// Configuration for the UNO API client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL of the uno-app API (e.g., "https://api.example.com").
    pub base_url: String,
    /// Client identifier for authentication.
    pub client_id: String,
    /// Secret key for HMAC signing.
    pub secret_key: Vec<u8>,
    /// Request timeout duration.
    pub timeout: Duration,
    /// Maximum request age in seconds (for replay protection).
    pub max_request_age_secs: i64,
    /// API version prefix (e.g., "/api/v1").
    pub api_prefix: String,
}

impl ClientConfig {
    /// Create a new client configuration.
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL of the API server
    /// * `client_id` - Client identifier
    /// * `secret_key` - Secret key for request signing
    pub fn new(base_url: impl Into<String>, client_id: impl Into<String>, secret_key: &[u8]) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            client_id: client_id.into(),
            secret_key: secret_key.to_vec(),
            timeout: Duration::from_secs(30),
            max_request_age_secs: 300, // 5 minutes
            api_prefix: "/api/v1/admin".to_string(),
        }
    }

    /// Set the request timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the maximum request age for replay protection.
    pub fn with_max_request_age(mut self, secs: i64) -> Self {
        self.max_request_age_secs = secs;
        self
    }

    /// Set a custom API prefix.
    pub fn with_api_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.api_prefix = prefix.into();
        self
    }

    /// Build the full URL for an endpoint.
    pub fn url(&self, endpoint: &str) -> String {
        format!(
            "{}{}{}",
            self.base_url,
            self.api_prefix,
            if endpoint.starts_with('/') {
                endpoint.to_string()
            } else {
                format!("/{}", endpoint)
            }
        )
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
            client_id: String::new(),
            secret_key: Vec::new(),
            timeout: Duration::from_secs(30),
            max_request_age_secs: 300,
            api_prefix: "/api/v1/admin".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_building() {
        let config = ClientConfig::new("https://api.example.com", "client", b"secret");

        assert_eq!(
            config.url("/licenses"),
            "https://api.example.com/api/v1/admin/licenses"
        );
        assert_eq!(
            config.url("licenses"),
            "https://api.example.com/api/v1/admin/licenses"
        );
    }

    #[test]
    fn test_trailing_slash_handling() {
        let config = ClientConfig::new("https://api.example.com/", "client", b"secret");
        assert_eq!(
            config.url("/licenses"),
            "https://api.example.com/api/v1/admin/licenses"
        );
    }
}
