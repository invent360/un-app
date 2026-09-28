//! License repository trait definition

use async_trait::async_trait;
use std::collections::HashMap;
use crate::models::entity::{LicenseEntity, NewLicense, NewLicenseFromApi};

/// Marketplace data for a license
#[derive(Debug, Clone, Default)]
pub struct MarketplaceData {
    pub is_on_marketplace: bool,
    pub is_on_uno_marketplace: bool,
}

/// Split percentage data for a license
#[derive(Debug, Clone, Default)]
pub struct SplitData {
    pub uno_share: f64,
    pub ulo_share: f64,
    pub agent_share: f64,
}

/// Repository trait for License operations
#[async_trait]
pub trait LicenseRepositoryTrait: Send + Sync {
    /// Save a new license to the database
    async fn save_license(&self, new_license: NewLicense) -> Result<LicenseEntity, String>;

    /// Get a license by its primary key (UUID)
    async fn get_license_by_id(&self, id: &str) -> Result<Option<LicenseEntity>, String>;

    /// Get a license by its license_id (blockchain ID)
    async fn get_license_by_license_id(&self, license_id: &str) -> Result<Option<LicenseEntity>, String>;

    /// Get all licenses for a specific agent
    async fn get_licenses_by_agent_id(&self, agent_id: &str) -> Result<Vec<LicenseEntity>, String>;

    /// Get all licenses for a specific node
    async fn get_licenses_by_node_id(&self, node_id: &str) -> Result<Vec<LicenseEntity>, String>;

    /// List all licenses
    async fn list_licenses(&self) -> Result<Vec<LicenseEntity>, String>;

    /// Bulk insert licenses (returns count of inserted)
    async fn bulk_save_licenses(&self, licenses: Vec<NewLicense>) -> Result<usize, String>;

    /// Update an existing license
    async fn update_license(&self, license: LicenseEntity) -> Result<LicenseEntity, String>;

    /// Delete a license by ID
    async fn delete_license(&self, id: &str) -> Result<bool, String>;

    /// Count all licenses
    async fn count_licenses(&self) -> Result<i64, String>;

    /// Count online licenses
    async fn count_online_licenses(&self) -> Result<i64, String>;

    /// Get the most recent synced_at timestamp across all licenses
    async fn get_last_synced(&self) -> Result<Option<String>, String>;

    /// Upsert a license from API data (insert or update by license_id)
    async fn upsert_license_from_api(&self, license: NewLicenseFromApi) -> Result<LicenseEntity, String>;

    /// Bulk upsert licenses from API data
    async fn upsert_licenses_from_api(&self, licenses: Vec<NewLicenseFromApi>) -> Result<usize, String>;

    /// Get licenses that are on the marketplace
    async fn get_marketplace_licenses(&self) -> Result<Vec<LicenseEntity>, String>;

    /// Update marketplace status for a license
    async fn update_marketplace_status(
        &self,
        license_id: &str,
        is_on_marketplace: bool,
        status: Option<&str>,
        referral_code: Option<&str>,
    ) -> Result<(), String>;

    /// Publish licenses to marketplace (set is_on_marketplace = true, status = "unclaimed")
    async fn publish_to_marketplace(&self, license_ids: Vec<String>) -> Result<usize, String>;

    /// Unpublish licenses from marketplace (set is_on_marketplace = false)
    async fn unpublish_from_marketplace(&self, license_ids: Vec<String>) -> Result<usize, String>;

    /// Update marketplace status including UNO marketplace field
    async fn update_marketplace_status_extended(
        &self,
        license_id: &str,
        is_on_marketplace: bool,
        is_on_uno_marketplace: bool,
        status: Option<&str>,
        referral_code: Option<&str>,
    ) -> Result<(), String>;

    /// Set is_published flag for a license
    async fn set_is_published(&self, license_id: &str, is_published: bool) -> Result<(), String>;

    /// Get marketplace data for multiple license IDs (batch lookup)
    /// Returns a HashMap of license_id -> MarketplaceData
    async fn get_marketplace_data_batch(&self, license_ids: &[String]) -> Result<HashMap<String, MarketplaceData>, String>;

    /// Get split data for multiple license IDs (batch lookup)
    /// Returns a HashMap of license_id -> SplitData
    async fn get_split_data_batch(&self, license_ids: &[String]) -> Result<HashMap<String, SplitData>, String>;
}
