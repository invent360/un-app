//! Unetwork license service implementation.
//!
//! Provides a service wrapper around `UnetworkLicenseTrait` implementations
//! with additional convenience methods.

use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::ApiError;
use crate::models::request::{GetAllLicenseIdsRequest, GetLeaseDetailsRequest, GetLicensesRequest};
use crate::models::response::{LeaseDetails, LicenseGroup, LicenseIdItem, UnetworkLicense};

use super::UnetworkLicenseTrait;

/// Unetwork license service.
///
/// This service wraps a `UnetworkLicenseTrait` implementation (typically
/// the HTTP client) and provides additional convenience methods.
///
/// # Example
///
/// ```ignore
/// use uno_api::client::UnetworkClient;
/// use uno_api::config::UnetworkConfig;
/// use uno_api::services::unetwork::UnetworkLicenseService;
/// use std::sync::Arc;
///
/// let config = UnetworkConfig::new("your-jwt-token");
/// let client = Arc::new(UnetworkClient::new(config));
/// let service = UnetworkLicenseService::new(client);
///
/// // Get all leased licenses
/// let leased = service.get_leased_license_ids().await?;
///
/// // Find a specific license
/// let license = service.find_license_by_id("0x123...").await?;
/// ```
pub struct UnetworkLicenseService<P: UnetworkLicenseTrait> {
    provider: Arc<P>,
}

impl<P: UnetworkLicenseTrait> UnetworkLicenseService<P> {
    /// Create a new service with the given provider.
    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }

    /// Get the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }

    /// Get all leased license IDs.
    pub async fn get_leased_license_ids(&self) -> Result<Vec<LicenseIdItem>, ApiError> {
        self.provider
            .get_all_license_ids(GetAllLicenseIdsRequest::new().with_leased(true))
            .await
    }

    /// Get all bound license IDs.
    pub async fn get_bound_license_ids(&self) -> Result<Vec<LicenseIdItem>, ApiError> {
        self.provider
            .get_all_license_ids(GetAllLicenseIdsRequest::new().with_bound(true))
            .await
    }

    /// Get all grouped license IDs.
    pub async fn get_grouped_license_ids(&self) -> Result<Vec<LicenseIdItem>, ApiError> {
        self.provider
            .get_all_license_ids(GetAllLicenseIdsRequest::new().with_grouped(true))
            .await
    }

    /// Get leased, bound, and grouped license IDs.
    pub async fn get_active_license_ids(&self) -> Result<Vec<LicenseIdItem>, ApiError> {
        self.provider
            .get_all_license_ids(
                GetAllLicenseIdsRequest::new()
                    .with_leased(true)
                    .with_bound(true)
                    .with_grouped(true),
            )
            .await
    }

    /// Get enabled licenses with pagination.
    pub async fn get_enabled_licenses(
        &self,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<UnetworkLicense>, ApiError> {
        self.provider
            .get_licenses(
                GetLicensesRequest::new()
                    .with_enabled(true)
                    .with_pagination(page, page_size),
            )
            .await
    }

    /// Get licenses by role (e.g., "uno").
    pub async fn get_licenses_by_role(
        &self,
        role: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<UnetworkLicense>, ApiError> {
        self.provider
            .get_licenses(
                GetLicensesRequest::new()
                    .with_role(role)
                    .with_pagination(page, page_size),
            )
            .await
    }

    /// Get all licenses matching multiple filters.
    pub async fn get_filtered_licenses(
        &self,
        is_leased: Option<bool>,
        is_grouped: Option<bool>,
        is_bound: Option<bool>,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<UnetworkLicense>, ApiError> {
        let mut request = GetLicensesRequest::new().with_pagination(page, page_size);

        if let Some(leased) = is_leased {
            request = request.with_leased(leased);
        }
        if let Some(grouped) = is_grouped {
            request = request.with_grouped(grouped);
        }
        if let Some(bound) = is_bound {
            request = request.with_bound(bound);
        }

        self.provider.get_licenses(request).await
    }

    /// Find a license by ID across all groups.
    ///
    /// This fetches all groups and searches for the license.
    /// For large datasets, consider using a more targeted approach.
    pub async fn find_license_by_id(
        &self,
        license_id: &str,
    ) -> Result<Option<UnetworkLicense>, ApiError> {
        let groups = self.provider.get_all_license_groups().await?;

        for group in groups {
            if let Some(license) = group.licenses.into_iter().find(|l| l.id == license_id) {
                return Ok(Some(license));
            }
        }

        Ok(None)
    }

    /// Get lease details for a license.
    ///
    /// Returns the lease details if the license has an active lease.
    pub async fn get_lease_details(
        &self,
        license_id: &str,
    ) -> Result<Option<LeaseDetails>, ApiError> {
        self.provider
            .get_lease_details(GetLeaseDetailsRequest::new(license_id))
            .await
    }

    /// Get licenses by group ID with full details.
    pub async fn get_licenses_by_group(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<UnetworkLicense>, ApiError> {
        let groups = self.provider.get_all_license_groups().await?;

        for group in groups {
            if group.id == group_id {
                return Ok(group.licenses);
            }
        }

        Ok(Vec::new())
    }

    /// Find a license group by name.
    pub async fn find_group_by_name(&self, name: &str) -> Result<Option<LicenseGroup>, ApiError> {
        let groups = self.provider.get_all_license_groups().await?;
        Ok(groups.into_iter().find(|g| g.name == name))
    }

    /// Get all online licenses across all groups.
    pub async fn get_online_licenses(&self) -> Result<Vec<UnetworkLicense>, ApiError> {
        let groups = self.provider.get_all_license_groups().await?;

        let online: Vec<UnetworkLicense> = groups
            .into_iter()
            .flat_map(|g| g.licenses)
            .filter(|l| l.is_online.unwrap_or(false))
            .collect();

        Ok(online)
    }

    /// Get licenses that are below their uptime requirement.
    pub async fn get_underperforming_licenses(&self) -> Result<Vec<UnetworkLicense>, ApiError> {
        let groups = self.provider.get_all_license_groups().await?;

        let underperforming: Vec<UnetworkLicense> = groups
            .into_iter()
            .flat_map(|g| g.licenses)
            .filter(|l| !l.meets_uptime_requirement())
            .collect();

        Ok(underperforming)
    }

    /// Get summary statistics across all groups.
    pub async fn get_statistics(&self) -> Result<LicenseStatistics, ApiError> {
        let groups = self.provider.get_all_license_groups().await?;

        let mut stats = LicenseStatistics::default();

        for group in &groups {
            stats.total_groups += 1;
            stats.total_licenses += group.license_count as usize;

            for license in &group.licenses {
                if license.is_online.unwrap_or(false) {
                    stats.online_count += 1;
                }
                if license.is_leased() {
                    stats.leased_count += 1;
                }
                if license.is_bound() {
                    stats.bound_count += 1;
                }
                if let Some(uptime) = license.uptime {
                    stats.total_uptime += uptime;
                    stats.uptime_count += 1;
                }
            }
        }

        if stats.uptime_count > 0 {
            stats.average_uptime = stats.total_uptime / stats.uptime_count as f64;
        }

        Ok(stats)
    }
}

/// Summary statistics for licenses.
#[derive(Debug, Clone, Default)]
pub struct LicenseStatistics {
    /// Total number of license groups.
    pub total_groups: usize,
    /// Total number of licenses.
    pub total_licenses: usize,
    /// Number of online licenses.
    pub online_count: usize,
    /// Number of leased licenses.
    pub leased_count: usize,
    /// Number of bound licenses.
    pub bound_count: usize,
    /// Average uptime across all licenses (0.0 to 1.0).
    pub average_uptime: f64,
    /// Internal: sum of uptimes.
    total_uptime: f64,
    /// Internal: count of licenses with uptime data.
    uptime_count: usize,
}

impl LicenseStatistics {
    /// Get average uptime as a percentage (0-100).
    pub fn average_uptime_percentage(&self) -> f64 {
        self.average_uptime * 100.0
    }

    /// Get the online percentage.
    pub fn online_percentage(&self) -> f64 {
        if self.total_licenses == 0 {
            0.0
        } else {
            (self.online_count as f64 / self.total_licenses as f64) * 100.0
        }
    }
}

// Forward trait implementation to provider
#[async_trait]
impl<P: UnetworkLicenseTrait> UnetworkLicenseTrait for UnetworkLicenseService<P> {
    async fn get_all_license_ids(
        &self,
        request: GetAllLicenseIdsRequest,
    ) -> Result<Vec<LicenseIdItem>, ApiError> {
        self.provider.get_all_license_ids(request).await
    }

    async fn get_all_license_groups(&self) -> Result<Vec<LicenseGroup>, ApiError> {
        self.provider.get_all_license_groups().await
    }

    async fn get_licenses(
        &self,
        request: GetLicensesRequest,
    ) -> Result<Vec<UnetworkLicense>, ApiError> {
        self.provider.get_licenses(request).await
    }

    async fn get_lease_details(
        &self,
        request: GetLeaseDetailsRequest,
    ) -> Result<Option<LeaseDetails>, ApiError> {
        self.provider.get_lease_details(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    // Mock provider for testing
    struct MockProvider {
        license_ids: Vec<LicenseIdItem>,
        groups: Vec<LicenseGroup>,
        licenses: Vec<UnetworkLicense>,
    }

    impl MockProvider {
        fn new() -> Self {
            Self {
                license_ids: vec![],
                groups: vec![],
                licenses: vec![],
            }
        }

        fn with_license_ids(mut self, ids: Vec<LicenseIdItem>) -> Self {
            self.license_ids = ids;
            self
        }

        fn with_groups(mut self, groups: Vec<LicenseGroup>) -> Self {
            self.groups = groups;
            self
        }

        fn with_licenses(mut self, licenses: Vec<UnetworkLicense>) -> Self {
            self.licenses = licenses;
            self
        }
    }

    #[async_trait]
    impl UnetworkLicenseTrait for MockProvider {
        async fn get_all_license_ids(
            &self,
            _request: GetAllLicenseIdsRequest,
        ) -> Result<Vec<LicenseIdItem>, ApiError> {
            Ok(self.license_ids.clone())
        }

        async fn get_all_license_groups(&self) -> Result<Vec<LicenseGroup>, ApiError> {
            Ok(self.groups.clone())
        }

        async fn get_licenses(
            &self,
            _request: GetLicensesRequest,
        ) -> Result<Vec<UnetworkLicense>, ApiError> {
            Ok(self.licenses.clone())
        }

        async fn get_lease_details(
            &self,
            _request: GetLeaseDetailsRequest,
        ) -> Result<Option<LeaseDetails>, ApiError> {
            Ok(None)
        }
    }

    fn create_test_license(id: &str) -> UnetworkLicense {
        UnetworkLicense {
            id: id.to_string(),
            node_id: None,
            owner_wallet_address: None,
            alias: None,
            device_id: None,
            device_name: None,
            activation_start_at: None,
            activation_end_at: None,
            activation_by: None,
            activation_postponed_ms: None,
            lease_user_id: None,
            lease_share_percentage: None,
            lease_min_uptime_percentage: None,
            lease_from: None,
            lease_to: None,
            validation_last_success_at: None,
            uptime: None,
            total_count: None,
            is_online: None,
            settings: None,
        }
    }

    fn create_test_group(id: Uuid, name: &str, licenses: Vec<UnetworkLicense>) -> LicenseGroup {
        LicenseGroup {
            id,
            name: name.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            license_count: licenses.len() as i32,
            license_ids: licenses.iter().map(|l| l.id.clone()).collect(),
            licenses,
        }
    }

    #[tokio::test]
    async fn test_service_creation() {
        let provider = Arc::new(MockProvider::new());
        let service = UnetworkLicenseService::new(provider);

        let ids = service
            .get_all_license_ids(GetAllLicenseIdsRequest::new())
            .await
            .unwrap();
        assert!(ids.is_empty());
    }

    #[tokio::test]
    async fn test_get_leased_license_ids() {
        let provider = Arc::new(MockProvider::new().with_license_ids(vec![
            LicenseIdItem {
                id: "0x111".to_string(),
                alias: Some("Test".to_string()),
                is_leased: true,
            },
        ]));

        let service = UnetworkLicenseService::new(provider);
        let ids = service.get_leased_license_ids().await.unwrap();

        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0].id, "0x111");
    }

    #[tokio::test]
    async fn test_find_license_by_id() {
        let mut license = create_test_license("0x123");
        license.alias = Some("Found License".to_string());

        let group = create_test_group(Uuid::new_v4(), "Test Group", vec![license]);

        let provider = Arc::new(MockProvider::new().with_groups(vec![group]));
        let service = UnetworkLicenseService::new(provider);

        let found = service.find_license_by_id("0x123").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().alias, Some("Found License".to_string()));

        let not_found = service.find_license_by_id("0x999").await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_find_group_by_name() {
        let group = create_test_group(Uuid::new_v4(), "Nigeria", vec![]);

        let provider = Arc::new(MockProvider::new().with_groups(vec![group]));
        let service = UnetworkLicenseService::new(provider);

        let found = service.find_group_by_name("Nigeria").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Nigeria");

        let not_found = service.find_group_by_name("Unknown").await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_get_online_licenses() {
        let mut online_license = create_test_license("0x111");
        online_license.is_online = Some(true);

        let mut offline_license = create_test_license("0x222");
        offline_license.is_online = Some(false);

        let group = create_test_group(
            Uuid::new_v4(),
            "Test",
            vec![online_license, offline_license],
        );

        let provider = Arc::new(MockProvider::new().with_groups(vec![group]));
        let service = UnetworkLicenseService::new(provider);

        let online = service.get_online_licenses().await.unwrap();
        assert_eq!(online.len(), 1);
        assert_eq!(online[0].id, "0x111");
    }

    #[tokio::test]
    async fn test_get_statistics() {
        let mut license1 = create_test_license("0x111");
        license1.is_online = Some(true);
        license1.lease_user_id = Some(Uuid::new_v4());
        license1.device_id = Some("device1".to_string());
        license1.uptime = Some(0.9);

        let mut license2 = create_test_license("0x222");
        license2.is_online = Some(false);
        license2.uptime = Some(0.7);

        let group = create_test_group(Uuid::new_v4(), "Test", vec![license1, license2]);

        let provider = Arc::new(MockProvider::new().with_groups(vec![group]));
        let service = UnetworkLicenseService::new(provider);

        let stats = service.get_statistics().await.unwrap();

        assert_eq!(stats.total_groups, 1);
        assert_eq!(stats.total_licenses, 2);
        assert_eq!(stats.online_count, 1);
        assert_eq!(stats.leased_count, 1);
        assert_eq!(stats.bound_count, 1);
        assert!((stats.average_uptime - 0.8).abs() < 0.01);
        assert!((stats.online_percentage() - 50.0).abs() < 0.01);
    }
}
