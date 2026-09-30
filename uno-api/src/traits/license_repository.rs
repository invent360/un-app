//! License repository trait.

use async_trait::async_trait;

use crate::error::DbError;
use crate::models::{License, LicenseSummary, Paginated, PaginationParams, SplitType};

/// Repository trait for license operations.
///
/// This trait defines the interface that must be implemented by the host
/// application (e.g., uno-app) to provide database operations for licenses.
///
/// License IDs are strings (VARCHAR(66) in database) to preserve full upstream
/// identifiers including blockchain hex addresses.
#[async_trait]
pub trait LicenseRepository: Send + Sync {
    /// Insert a new license.
    async fn insert(&self, license: &License) -> Result<(), DbError>;

    /// Insert multiple licenses in a batch.
    /// Returns the number of successfully inserted licenses.
    async fn insert_batch(&self, licenses: &[License]) -> Result<usize, DbError>;

    /// Get a license by its ID (full upstream identifier).
    async fn get_by_id(&self, id: &str) -> Result<Option<License>, DbError>;

    /// Get a license by its lease code.
    async fn get_by_lease_code(&self, lease_code: &str) -> Result<Option<License>, DbError>;

    /// Get the first unclaimed license for a given split type.
    async fn get_first_unclaimed(&self, split_type: SplitType) -> Result<Option<License>, DbError>;

    /// Get the first unclaimed license (any split type).
    async fn get_first_unclaimed_any(&self) -> Result<Option<License>, DbError>;

    /// Get licenses by split type with pagination.
    async fn get_by_split_type(
        &self,
        split_type: SplitType,
        pagination: &PaginationParams,
    ) -> Result<Paginated<License>, DbError>;

    /// Get all unclaimed licenses with pagination.
    async fn get_unclaimed(
        &self,
        pagination: &PaginationParams,
    ) -> Result<Paginated<License>, DbError>;

    /// Count total licenses.
    async fn count_total(&self) -> Result<i64, DbError>;

    /// Count unclaimed licenses.
    async fn count_unclaimed(&self) -> Result<i64, DbError>;

    /// Count unclaimed licenses by split type.
    async fn count_unclaimed_by_split(&self, split_type: SplitType) -> Result<i64, DbError>;

    /// Mark a license as claimed.
    async fn claim(
        &self,
        id: &str,
        device_id: Option<String>,
    ) -> Result<License, DbError>;

    /// Check if a lease code exists.
    async fn lease_code_exists(&self, lease_code: &str) -> Result<bool, DbError>;

    /// Delete a license by ID.
    async fn delete(&self, id: &str) -> Result<(), DbError>;

    /// Delete multiple licenses by IDs.
    async fn delete_batch(&self, ids: &[String]) -> Result<usize, DbError>;

    /// Get license summary statistics.
    async fn get_summary(&self) -> Result<LicenseSummary, DbError>;

    /// Search licenses with filters.
    async fn search(
        &self,
        filters: &LicenseFilters,
        pagination: &PaginationParams,
    ) -> Result<Paginated<License>, DbError>;
}

/// Filters for license search.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LicenseFilters {
    /// Filter by split type.
    pub split_type: Option<SplitType>,
    /// Filter by claimed status.
    pub claimed: Option<bool>,
    /// Filter by bound to device status.
    pub bound_to_device: Option<bool>,
    /// Search by lease code (partial match).
    pub lease_code_search: Option<String>,
    /// Filter expired licenses.
    pub include_expired: Option<bool>,
}

impl LicenseFilters {
    /// Create empty filters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by split type.
    pub fn with_split_type(mut self, split_type: SplitType) -> Self {
        self.split_type = Some(split_type);
        self
    }

    /// Filter by claimed status.
    pub fn with_claimed(mut self, claimed: bool) -> Self {
        self.claimed = Some(claimed);
        self
    }

    /// Filter unclaimed only.
    pub fn unclaimed_only(mut self) -> Self {
        self.claimed = Some(false);
        self
    }
}
