//! UNO API client wrapper for license management.
//!
//! This module wraps uno-api's HTTP client for communication
//! with uno-app's license management API.

#[cfg(feature = "ssr")]
pub use ssr_impl::*;

#[cfg(feature = "ssr")]
mod ssr_impl {
    use uno_api::client::{ClientConfig, UnoApiClient};
    use uno_api::error::ApiError;
    use uno_api::models::*;

    /// Create a configured UnoApiClient from environment variables.
    ///
    /// Environment variables (inline values override .env):
    /// - UNO_API_URL: Base URL of the uno-app API (default: http://localhost:3000)
    /// - ADMIN_CLIENT_ID or UNO_CLIENT_ID: Client identifier for HMAC auth
    /// - ADMIN_SECRET_KEY or UNO_SECRET_KEY: Secret key for HMAC signing
    ///
    /// Example: ADMIN_CLIENT_ID=admin ADMIN_SECRET_KEY=secret123 cargo leptos serve
    pub fn create_uno_client() -> Result<UnoApiClient, String> {
        let api_url =
            std::env::var("UNO_API_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

        // ADMIN_CLIENT_ID takes precedence over UNO_CLIENT_ID (inline override)
        let client_id = std::env::var("ADMIN_CLIENT_ID")
            .or_else(|_| std::env::var("UNO_CLIENT_ID"))
            .map_err(|_| "ADMIN_CLIENT_ID or UNO_CLIENT_ID not set")?;

        // ADMIN_SECRET_KEY takes precedence over UNO_SECRET_KEY (inline override)
        let secret_key = std::env::var("ADMIN_SECRET_KEY")
            .or_else(|_| std::env::var("UNO_SECRET_KEY"))
            .map_err(|_| "ADMIN_SECRET_KEY or UNO_SECRET_KEY not set")?;

        let api_key = std::env::var("ADMIN_API_KEY")
            .ok()
            .filter(|key| key.len() >= 32 && key != "dev-admin-key")
            .ok_or("ADMIN_API_KEY must be configured for the portal machine boundary")?;
        let config =
            ClientConfig::new(&api_url, &client_id, secret_key.as_bytes()).with_api_key(api_key);
        Ok(UnoApiClient::new(config))
    }

    /// Wrapper providing convenient access to uno-api operations.
    #[derive(Clone)]
    pub struct UnoLicenseClient {
        inner: std::sync::Arc<UnoApiClient>,
    }

    impl UnoLicenseClient {
        /// Create a new client from environment configuration.
        pub fn from_env() -> Result<Self, String> {
            let client = create_uno_client()?;
            Ok(Self {
                inner: std::sync::Arc::new(client),
            })
        }

        /// Create a new client with explicit configuration.
        pub fn new(api_url: &str, client_id: &str, secret_key: &[u8]) -> Self {
            let config = ClientConfig::new(api_url, client_id, secret_key);
            Self {
                inner: std::sync::Arc::new(UnoApiClient::new(config)),
            }
        }

        /// Publish licenses to uno-app.
        pub async fn publish_licenses(
            &self,
            request: PublishLicensesRequest,
        ) -> Result<PublishResult, ApiError> {
            self.inner.publish_licenses(request).await
        }

        /// Import licenses from CSV.
        pub async fn import_csv(&self, csv_data: &str) -> Result<ImportResult, ApiError> {
            self.inner.import_csv(csv_data).await
        }

        /// Import licenses with custom options.
        pub async fn import_csv_with_options(
            &self,
            request: CsvImportRequest,
        ) -> Result<ImportResult, ApiError> {
            self.inner.import_csv_with_options(request).await
        }

        /// List all variants.
        pub async fn list_variants(&self) -> Result<Vec<VariantDto>, ApiError> {
            self.inner.list_variants().await
        }

        /// Get variant summary.
        pub async fn get_variant_summary(&self) -> Result<VariantSummary, ApiError> {
            self.inner.get_variant_summary().await
        }

        /// Search licenses.
        pub async fn search_licenses(
            &self,
            filters: uno_api::traits::LicenseFilters,
            pagination: PaginationParams,
        ) -> Result<Paginated<LicenseDto>, ApiError> {
            self.inner.search_licenses(filters, pagination).await
        }

        /// Revoke licenses.
        pub async fn revoke_licenses(
            &self,
            request: RevokeRequest,
        ) -> Result<RevokeResult, ApiError> {
            self.inner.revoke_licenses(request).await
        }

        /// Check API health.
        pub async fn health(&self) -> bool {
            self.inner.health().await.unwrap_or(false)
        }

        // ========== Marketplace Operations ==========

        /// Get claimed licenses for sync.
        pub async fn get_claimed_licenses(
            &self,
            since: Option<&str>,
            limit: Option<i32>,
        ) -> Result<uno_api::models::marketplace::ClaimedLicensesResponse, ApiError> {
            self.inner.get_claimed_licenses(since, limit).await
        }

        /// Get all referrals for sync.
        pub async fn get_referrals(
            &self,
        ) -> Result<uno_api::models::marketplace::ReferralsResponse, ApiError> {
            self.inner.get_referrals().await
        }

        /// Sync referrals to uno-app.
        pub async fn sync_referrals(
            &self,
            referrals: Vec<uno_api::models::marketplace::ReferralInput>,
        ) -> Result<uno_api::models::marketplace::SyncResult, ApiError> {
            self.inner.sync_referrals(referrals).await
        }

        /// Get visitor stats from uno-app.
        pub async fn get_visitor_stats(
            &self,
            period: Option<&str>,
            limit: Option<i32>,
        ) -> Result<uno_api::models::marketplace::VisitorStatsResponse, ApiError> {
            self.inner.get_visitor_stats(period, limit).await
        }
    }

    pub use uno_api::models::marketplace::{
        ClaimedLicenseDto, ClaimedLicensesResponse, CountryVisitorStats, GetVisitorStatsRequest,
        ReferralDto, ReferralInput, ReferralsResponse, SyncReferralsRequest, SyncResult,
        VisitorStatsResponse,
    };
    /// Re-export common types for convenience
    pub use uno_api::models::{
        CsvImportRequest, ImportError, ImportResult, LicenseDto, LicenseInput, LicenseStatus,
        Paginated, PaginationParams, PublishError, PublishLicensesRequest, PublishResult,
        RevokeRequest, RevokeResult, VariantDto, VariantInput, VariantStatus, VariantSummary,
    };
    pub use uno_api::traits::LicenseFilters;
}
