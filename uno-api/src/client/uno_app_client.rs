//! UNO API HTTP client.

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::auth::{sign_request, SignedRequest};
use crate::error::ApiError;
use crate::models::marketplace;
use crate::models::*;

use super::ClientConfig;

/// HTTP client for the UNO license management API.
///
/// All requests are automatically signed with HMAC-SHA256 using
/// the configured client credentials.
pub struct UnoApiClient {
    config: ClientConfig,
    http: Client,
}

impl UnoApiClient {
    /// Create a new API client with the given configuration.
    pub fn new(config: ClientConfig) -> Self {
        let http = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to build HTTP client");

        Self { config, http }
    }

    /// Create a signed request for a payload.
    fn sign<T: Serialize + Clone>(&self, payload: T) -> SignedRequest<T> {
        sign_request(&self.config.client_id, &self.config.secret_key, &payload)
    }

    /// Send a POST request with a signed payload.
    async fn post<T: Serialize + Clone, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        payload: T,
    ) -> Result<R, ApiError> {
        let url = self.config.url(endpoint);
        let signed = self.sign(payload);

        let response = self
            .http
            .post(&url)
            .bearer_auth(self.machine_key()?)
            .json(&signed)
            .send()
            .await
            .map_err(|e| ApiError::Http(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Send a GET request.
    async fn get<R: DeserializeOwned>(&self, endpoint: &str) -> Result<R, ApiError> {
        let url = self.config.url(endpoint);

        // For GET requests, sign an empty payload
        let signed = self.sign(serde_json::Value::Null);

        let response = self
            .http
            .get(&url)
            .bearer_auth(self.machine_key()?)
            .header("X-Client-Id", &self.config.client_id)
            .header("X-Timestamp", signed.timestamp.to_string())
            .header("X-Nonce", &signed.nonce)
            .header("X-Signature", &signed.signature)
            .send()
            .await
            .map_err(|e| ApiError::Http(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Send a DELETE request.
    async fn delete<T: Serialize + Clone, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        payload: T,
    ) -> Result<R, ApiError> {
        let url = self.config.url(endpoint);
        let signed = self.sign(payload);

        let response = self
            .http
            .delete(&url)
            .bearer_auth(self.machine_key()?)
            .json(&signed)
            .send()
            .await
            .map_err(|e| ApiError::Http(e.to_string()))?;

        self.handle_response(response).await
    }

    fn machine_key(&self) -> Result<&str, ApiError> {
        self.config
            .api_key
            .as_deref()
            .filter(|key| key.len() >= 32 && *key != "dev-admin-key")
            .ok_or_else(|| {
                ApiError::Validation("Machine bearer credential is not configured".into())
            })
    }

    /// Handle the HTTP response.
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
                401 => Err(ApiError::Auth(crate::error::AuthError::InvalidSignature)),
                403 => Err(ApiError::Auth(crate::error::AuthError::MissingAuth)),
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

    // ========== License Operations ==========

    /// Publish a batch of licenses.
    pub async fn publish_licenses(
        &self,
        request: PublishLicensesRequest,
    ) -> Result<PublishResult, ApiError> {
        self.post("/licenses", request).await
    }

    /// Import licenses from CSV data.
    pub async fn import_csv(&self, csv_data: &str) -> Result<ImportResult, ApiError> {
        let request = CsvImportRequest::new(csv_data);
        self.post("/licenses/import", request).await
    }

    /// Import licenses with custom options.
    pub async fn import_csv_with_options(
        &self,
        request: CsvImportRequest,
    ) -> Result<ImportResult, ApiError> {
        self.post("/licenses/import", request).await
    }

    /// Get a license by ID.
    pub async fn get_license(&self, id: &str) -> Result<Option<LicenseDto>, ApiError> {
        match self.get::<LicenseDto>(&format!("/licenses/{}", id)).await {
            Ok(license) => Ok(Some(license)),
            Err(ApiError::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Search licenses with filters.
    pub async fn search_licenses(
        &self,
        filters: LicenseFilters,
        pagination: PaginationParams,
    ) -> Result<Paginated<LicenseDto>, ApiError> {
        #[derive(Serialize, Clone)]
        struct SearchRequest {
            filters: LicenseFilters,
            pagination: PaginationParams,
        }

        self.post(
            "/licenses/search",
            SearchRequest {
                filters,
                pagination,
            },
        )
        .await
    }

    /// Revoke licenses.
    pub async fn revoke_licenses(&self, request: RevokeRequest) -> Result<RevokeResult, ApiError> {
        self.delete("/licenses", request).await
    }

    /// Revoke a single license by ID.
    pub async fn revoke_license(&self, id: &str) -> Result<RevokeResult, ApiError> {
        self.revoke_licenses(RevokeRequest {
            license_ids: vec![id.to_string()],
            reason: None,
        })
        .await
    }

    // ========== Variant Operations ==========

    /// List all variants.
    pub async fn list_variants(&self) -> Result<Vec<VariantDto>, ApiError> {
        self.get("/variants").await
    }

    /// Get variant summary statistics.
    pub async fn get_variant_summary(&self) -> Result<VariantSummary, ApiError> {
        self.get("/variants/summary").await
    }

    /// Create or update a variant.
    pub async fn upsert_variant(&self, input: VariantInput) -> Result<VariantDto, ApiError> {
        self.post("/variants", input).await
    }

    // ========== Health & Info ==========

    /// Check API health.
    pub async fn health(&self) -> Result<bool, ApiError> {
        #[derive(serde::Deserialize)]
        struct HealthResponse {
            status: String,
        }

        match self.get::<HealthResponse>("/health").await {
            Ok(resp) => Ok(resp.status == "ok"),
            Err(_) => Ok(false),
        }
    }

    // ========== Marketplace Operations ==========

    /// Get claimed licenses for sync with uno-admin.
    pub async fn get_claimed_licenses(
        &self,
        since: Option<&str>,
        limit: Option<i32>,
    ) -> Result<marketplace::ClaimedLicensesResponse, ApiError> {
        let request = marketplace::GetClaimedLicensesRequest {
            since: since.map(String::from),
            limit,
        };
        self.post("/licenses/claimed", request).await
    }

    /// Get all referrals for sync with uno-admin.
    pub async fn get_referrals(&self) -> Result<marketplace::ReferralsResponse, ApiError> {
        self.post("/referrals", serde_json::Value::Null).await
    }

    /// Sync referrals from uno-admin to uno-app.
    pub async fn sync_referrals(
        &self,
        referrals: Vec<marketplace::ReferralInput>,
    ) -> Result<marketplace::SyncResult, ApiError> {
        let request = marketplace::SyncReferralsRequest { referrals };
        self.post("/referrals/sync", request).await
    }

    /// Get visitor stats by country for sync with uno-admin.
    pub async fn get_visitor_stats(
        &self,
        period: Option<&str>,
        limit: Option<i32>,
    ) -> Result<marketplace::VisitorStatsResponse, ApiError> {
        let request = marketplace::GetVisitorStatsRequest {
            period: period.map(String::from),
            limit,
        };
        self.post("/stats/visitors", request).await
    }
}

/// Builder for LicenseFilters (re-exported for convenience).
pub use crate::traits::LicenseFilters;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = ClientConfig::new("https://api.example.com", "client1", b"secret");
        let _client = UnoApiClient::new(config);
    }
}
