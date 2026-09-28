//! Unetwork API client configuration.
//!
//! Provides JWT-based configuration for the Unetwork License API.
//! Unlike `ClientConfig` which uses HMAC signing, this uses JWT bearer tokens.

use std::time::Duration;

/// Configuration for the Unetwork License API client.
///
/// This configuration uses JWT bearer token authentication, which is provided
/// by the parent application (e.g., uno-app or uno-admin).
///
/// # Example
///
/// ```
/// use uno_api::config::UnetworkConfig;
/// use std::time::Duration;
///
/// let config = UnetworkConfig::new("your-jwt-token")
///     .with_base_url("https://staging.unityedge.io")
///     .with_timeout(Duration::from_secs(60));
/// ```
#[derive(Debug, Clone)]
pub struct UnetworkConfig {
    /// Base URL of the Unetwork API.
    pub base_url: String,
    /// JWT bearer token for authentication.
    pub jwt_token: String,
    /// Request timeout duration.
    pub timeout: Duration,
}

impl UnetworkConfig {
    /// Default base URL for the Unetwork API.
    pub const DEFAULT_BASE_URL: &'static str = "https://api.unityedge.io";

    /// Default request timeout in seconds.
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

    /// Create a new Unetwork configuration with the given JWT token.
    ///
    /// # Arguments
    ///
    /// * `jwt_token` - JWT bearer token for authentication
    ///
    /// # Example
    ///
    /// ```
    /// use uno_api::config::UnetworkConfig;
    ///
    /// let config = UnetworkConfig::new("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...");
    /// assert_eq!(config.base_url, "https://api.unityedge.io");
    /// ```
    pub fn new(jwt_token: impl Into<String>) -> Self {
        Self {
            base_url: Self::DEFAULT_BASE_URL.to_string(),
            jwt_token: jwt_token.into(),
            timeout: Duration::from_secs(Self::DEFAULT_TIMEOUT_SECS),
        }
    }

    /// Set a custom base URL.
    ///
    /// Trailing slashes are automatically removed.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL for the API
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into().trim_end_matches('/').to_string();
        self
    }

    /// Set the request timeout.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The timeout duration for HTTP requests
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Build URL for RPC endpoint (Supabase PostgREST).
    ///
    /// Pattern: `POST /rest/v1/rpc/{function_name}`
    ///
    /// # Arguments
    ///
    /// * `function_name` - The RPC function name
    ///
    /// # Example
    ///
    /// ```
    /// use uno_api::config::UnetworkConfig;
    ///
    /// let config = UnetworkConfig::new("token");
    /// assert_eq!(
    ///     config.rpc_url("licenses_get_all_ids"),
    ///     "https://api.unityedge.io/rest/v1/rpc/licenses_get_all_ids"
    /// );
    /// ```
    pub fn rpc_url(&self, function_name: &str) -> String {
        format!("{}/rest/v1/rpc/{}", self.base_url, function_name)
    }

    /// Build URL for Edge Functions endpoint.
    ///
    /// Pattern: `POST /functions/v1/{function_name}`
    ///
    /// # Arguments
    ///
    /// * `function_name` - The Edge Function name
    ///
    /// # Example
    ///
    /// ```
    /// use uno_api::config::UnetworkConfig;
    ///
    /// let config = UnetworkConfig::new("token");
    /// assert_eq!(
    ///     config.functions_url("license_groups_get_all"),
    ///     "https://api.unityedge.io/functions/v1/license_groups_get_all"
    /// );
    /// ```
    pub fn functions_url(&self, function_name: &str) -> String {
        format!("{}/functions/v1/{}", self.base_url, function_name)
    }

    /// Check if the JWT token is set (non-empty).
    pub fn has_token(&self) -> bool {
        !self.jwt_token.is_empty()
    }
}

impl Default for UnetworkConfig {
    fn default() -> Self {
        Self {
            base_url: Self::DEFAULT_BASE_URL.to_string(),
            jwt_token: String::new(),
            timeout: Duration::from_secs(Self::DEFAULT_TIMEOUT_SECS),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_defaults() {
        let config = UnetworkConfig::new("test-token");
        assert_eq!(config.base_url, "https://api.unityedge.io");
        assert_eq!(config.jwt_token, "test-token");
        assert_eq!(config.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_rpc_url() {
        let config = UnetworkConfig::new("token");
        assert_eq!(
            config.rpc_url("licenses_get_all_ids"),
            "https://api.unityedge.io/rest/v1/rpc/licenses_get_all_ids"
        );
    }

    #[test]
    fn test_functions_url() {
        let config = UnetworkConfig::new("token");
        assert_eq!(
            config.functions_url("license_groups_get_all"),
            "https://api.unityedge.io/functions/v1/license_groups_get_all"
        );
    }

    #[test]
    fn test_custom_base_url() {
        let config = UnetworkConfig::new("token")
            .with_base_url("https://staging.unityedge.io/");
        assert_eq!(config.base_url, "https://staging.unityedge.io");
    }

    #[test]
    fn test_custom_timeout() {
        let config = UnetworkConfig::new("token")
            .with_timeout(Duration::from_secs(60));
        assert_eq!(config.timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_has_token() {
        let config_with_token = UnetworkConfig::new("token");
        assert!(config_with_token.has_token());

        let config_without_token = UnetworkConfig::default();
        assert!(!config_without_token.has_token());
    }

    #[test]
    fn test_builder_chain() {
        let config = UnetworkConfig::new("my-jwt")
            .with_base_url("https://test.api.com")
            .with_timeout(Duration::from_secs(120));

        assert_eq!(config.jwt_token, "my-jwt");
        assert_eq!(config.base_url, "https://test.api.com");
        assert_eq!(config.timeout, Duration::from_secs(120));
    }
}
