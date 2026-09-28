//! License analytics repository trait definition

use async_trait::async_trait;
use crate::models::entity::{LicenseAnalyticsEntity, NewLicenseAnalytics};

/// Repository trait for License Analytics operations
#[async_trait]
pub trait LicenseAnalyticsRepositoryTrait: Send + Sync {
    /// Upsert analytics record (insert or update)
    async fn upsert_analytics(&self, analytics: NewLicenseAnalytics) -> Result<LicenseAnalyticsEntity, String>;

    /// Bulk upsert analytics records
    async fn upsert_analytics_batch(&self, analytics: Vec<NewLicenseAnalytics>) -> Result<usize, String>;

    /// Get analytics for a license within date range
    async fn get_analytics_by_license(
        &self,
        license_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<LicenseAnalyticsEntity>, String>;

    /// Get most recent analytics entry for a license
    async fn get_latest_analytics(&self, license_id: &str) -> Result<Option<LicenseAnalyticsEntity>, String>;

    /// Get all analytics for a specific date
    async fn get_analytics_by_date(&self, date: &str) -> Result<Vec<LicenseAnalyticsEntity>, String>;

    /// Delete analytics for a license
    async fn delete_analytics_by_license(&self, license_id: &str) -> Result<usize, String>;
}
