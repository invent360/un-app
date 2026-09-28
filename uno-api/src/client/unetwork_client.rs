//! Unetwork API HTTP client.
//!
//! JWT-authenticated client for the Unetwork License API.
//! Uses Bearer token authentication instead of HMAC signing.
//!
//! # Example
//!
//! ```ignore
//! use uno_api::client::UnetworkClient;
//! use uno_api::config::UnetworkConfig;
//! use uno_api::models::request::GetAllLicenseIdsRequest;
//! use uno_api::services::unetwork::UnetworkLicenseTrait;
//!
//! let config = UnetworkConfig::new("your-jwt-token");
//! let client = UnetworkClient::new(config);
//!
//! let ids = client.get_all_license_ids(GetAllLicenseIdsRequest::new()).await?;
//! ```

use async_trait::async_trait;
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::config::UnetworkConfig;
use crate::error::{ApiError, AuthError};
use crate::models::request::{
    GetAllLicenseGroupsRequest, GetAllLicenseIdsRequest, GetLeaseDetailsRequest,
    GetLicensesRequest, GetRewardsBalanceRequest, GetRewardsRequest,
};
use crate::models::response::{
    LeaseDetails, LicenseGroup, LicenseIdItem, RewardAllocation, RewardBalance, UnetworkLicense,
};
use crate::services::unetwork::{UnetworkLicenseTrait, UnetworkRewardsTrait};

/// HTTP client for the Unetwork License API.
///
/// All requests use JWT Bearer token authentication.
///
/// # Example
///
/// ```ignore
/// use uno_api::client::UnetworkClient;
/// use uno_api::config::UnetworkConfig;
///
/// let config = UnetworkConfig::new("your-jwt-token")
///     .with_base_url("https://api.unityedge.io");
///
/// let client = UnetworkClient::new(config);
/// ```
pub struct UnetworkClient {
    config: UnetworkConfig,
    http: Client,
}

impl UnetworkClient {
    /// Create a new Unetwork API client.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration including JWT token and base URL
    ///
    /// # Panics
    ///
    /// Panics if the HTTP client fails to build (should not happen with default settings).
    pub fn new(config: UnetworkConfig) -> Self {
        let http = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to build HTTP client");

        Self { config, http }
    }

    /// Create a client with a custom reqwest Client.
    ///
    /// Useful for testing or when you need custom HTTP settings.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration including JWT token and base URL
    /// * `http` - Custom reqwest Client
    pub fn with_http_client(config: UnetworkConfig, http: Client) -> Self {
        Self { config, http }
    }

    /// Get the current JWT token.
    pub fn jwt_token(&self) -> &str {
        &self.config.jwt_token
    }

    /// Update the JWT token (e.g., after refresh).
    ///
    /// # Arguments
    ///
    /// * `token` - The new JWT token
    pub fn set_jwt_token(&mut self, token: impl Into<String>) {
        self.config.jwt_token = token.into();
    }

    /// Get the base URL.
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// Send a POST request to an RPC endpoint.
    ///
    /// RPC endpoints use the pattern: `POST /rest/v1/rpc/{function_name}`
    async fn post_rpc<T: Serialize, R: DeserializeOwned>(
        &self,
        function_name: &str,
        payload: T,
    ) -> Result<R, ApiError> {
        let url = self.config.rpc_url(function_name);
        self.post_json(&url, payload).await
    }

    /// Send a POST request to an Edge Function endpoint.
    ///
    /// Edge Function endpoints use the pattern: `POST /functions/v1/{function_name}`
    async fn post_function<T: Serialize, R: DeserializeOwned>(
        &self,
        function_name: &str,
        payload: T,
    ) -> Result<R, ApiError> {
        let url = self.config.functions_url(function_name);
        self.post_json(&url, payload).await
    }

    /// Internal POST with JSON body and JWT auth.
    async fn post_json<T: Serialize, R: DeserializeOwned>(
        &self,
        url: &str,
        payload: T,
    ) -> Result<R, ApiError> {
        if !self.config.has_token() {
            return Err(ApiError::Auth(AuthError::MissingAuth));
        }

        let response = self
            .http
            .post(url)
            .header("Authorization", format!("Bearer {}", self.config.jwt_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| ApiError::Http(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Handle HTTP response with error mapping.
    async fn handle_response<R: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<R, ApiError> {
        let status = response.status();

        if status.is_success() {
            response
                .json()
                .await
                .map_err(|e| ApiError::Serialization(e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();

            match status.as_u16() {
                401 => Err(ApiError::Auth(AuthError::InvalidSignature)),
                403 => Err(ApiError::Auth(AuthError::MissingAuth)),
                404 => Err(ApiError::NotFound(error_text)),
                409 => Err(ApiError::Conflict(error_text)),
                422 => Err(ApiError::Validation(error_text)),
                _ => Err(ApiError::Http(format!(
                    "HTTP {}: {}",
                    status.as_u16(),
                    error_text
                ))),
            }
        }
    }
}

#[async_trait]
impl UnetworkLicenseTrait for UnetworkClient {
    /// Get all license IDs.
    ///
    /// Calls: `POST /rest/v1/rpc/licenses_get_all_ids`
    async fn get_all_license_ids(
        &self,
        request: GetAllLicenseIdsRequest,
    ) -> Result<Vec<LicenseIdItem>, ApiError> {
        self.post_rpc("licenses_get_all_ids", request).await
    }

    /// Get all license groups.
    ///
    /// Calls: `POST /functions/v1/license_groups_get_all`
    async fn get_all_license_groups(&self) -> Result<Vec<LicenseGroup>, ApiError> {
        self.post_function("license_groups_get_all", GetAllLicenseGroupsRequest::new())
            .await
    }

    /// Get licenses with pagination.
    ///
    /// Calls: `POST /functions/v1/licenses_get_licenses`
    async fn get_licenses(
        &self,
        request: GetLicensesRequest,
    ) -> Result<Vec<UnetworkLicense>, ApiError> {
        self.post_function("licenses_get_licenses", request).await
    }

    /// Get lease details for a license.
    ///
    /// Calls: `POST /rest/v1/rpc/get_lease_id_by_license`
    async fn get_lease_details(
        &self,
        request: GetLeaseDetailsRequest,
    ) -> Result<Option<LeaseDetails>, ApiError> {
        let result: Vec<LeaseDetails> = self.post_rpc("get_lease_id_by_license", request).await?;
        Ok(result.into_iter().next())
    }
}

#[async_trait]
impl UnetworkRewardsTrait for UnetworkClient {
    /// Get reward allocations for a license.
    ///
    /// Calls: `POST /rest/v1/rpc/rewards_get_allocations`
    async fn get_rewards(
        &self,
        request: GetRewardsRequest,
    ) -> Result<Vec<RewardAllocation>, ApiError> {
        self.post_rpc("rewards_get_allocations", request).await
    }

    /// Get reward balance for a license.
    ///
    /// Calls: `POST /rest/v1/rpc/rewards_get_balance`
    async fn get_balance(
        &self,
        request: GetRewardsBalanceRequest,
    ) -> Result<RewardBalance, ApiError> {
        let result: Vec<RewardBalance> = self.post_rpc("rewards_get_balance", request).await?;
        result.into_iter().next().ok_or_else(|| {
            ApiError::NotFound("No balance found for license".to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_client_creation() {
        let config = UnetworkConfig::new("test-jwt-token");
        let client = UnetworkClient::new(config);
        assert_eq!(client.jwt_token(), "test-jwt-token");
    }

    #[test]
    fn test_jwt_token_update() {
        let config = UnetworkConfig::new("old-token");
        let mut client = UnetworkClient::new(config);

        client.set_jwt_token("new-token");
        assert_eq!(client.jwt_token(), "new-token");
    }

    #[test]
    fn test_base_url() {
        let config = UnetworkConfig::new("token")
            .with_base_url("https://custom.api.com");
        let client = UnetworkClient::new(config);
        assert_eq!(client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_default_base_url() {
        let config = UnetworkConfig::new("token");
        let client = UnetworkClient::new(config);
        assert_eq!(client.base_url(), "https://api.unityedge.io");
    }

    #[test]
    fn test_custom_http_client() {
        let config = UnetworkConfig::new("token");
        let http = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap();

        let client = UnetworkClient::with_http_client(config, http);
        assert_eq!(client.jwt_token(), "token");
    }
}
