//! Unetwork service module.
//!
//! Provides trait definitions and implementations for interacting with
//! the Unetwork License and Rewards APIs.
//!
//! # Architecture
//!
//! This module follows a trait-based design:
//!
//! - `UnetworkLicenseTrait` - Defines the interface for license operations
//! - `UnetworkRewardsTrait` - Defines the interface for rewards operations
//! - `UnetworkLicenseService` - Wraps a license provider with convenience methods
//! - `UnetworkRewardsService` - Wraps a rewards provider with convenience methods
//! - `UnetworkClient` (in `client` module) - HTTP implementation of both traits
//!
//! # Example
//!
//! ```ignore
//! use uno_api::client::UnetworkClient;
//! use uno_api::config::UnetworkConfig;
//! use uno_api::services::unetwork::{UnetworkLicenseTrait, UnetworkLicenseService};
//! use uno_api::models::request::GetAllLicenseIdsRequest;
//! use std::sync::Arc;
//!
//! let config = UnetworkConfig::new("your-jwt-token");
//! let client = Arc::new(UnetworkClient::new(config));
//! let service = UnetworkLicenseService::new(client);
//!
//! // Use the license service
//! let ids = service.get_leased_license_ids().await?;
//!
//! // Use the rewards service
//! let rewards_service = UnetworkRewardsService::new(client.clone());
//! let balance = rewards_service.get_balance("0x123...").await?;
//! ```

mod licenses_service;
mod rewards_service;

pub use licenses_service::{LicenseStatistics, UnetworkLicenseService};
pub use rewards_service::UnetworkRewardsService;

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::ApiError;
use crate::models::request::{
    GetAllLicenseIdsRequest, GetLeaseDetailsRequest, GetLicensesRequest, GetRewardsBalanceRequest,
    GetRewardsRequest,
};
use crate::models::response::{
    LeaseDetails, LicenseGroup, LicenseIdItem, RewardAllocation, RewardBalance, UnetworkLicense,
};

/// Trait defining Unetwork license operations.
///
/// This trait can be implemented by HTTP clients or mock implementations
/// for testing purposes.
///
/// All methods are async and return `Result<T, ApiError>`.
#[async_trait]
pub trait UnetworkLicenseTrait: Send + Sync {
    /// Get all license IDs with optional filtering.
    ///
    /// # Arguments
    ///
    /// * `request` - Filter parameters (all optional)
    ///
    /// # Returns
    ///
    /// List of license ID items with basic info (id, alias, isLeased).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let ids = client.get_all_license_ids(
    ///     GetAllLicenseIdsRequest::new().with_leased(true)
    /// ).await?;
    /// ```
    async fn get_all_license_ids(
        &self,
        request: GetAllLicenseIdsRequest,
    ) -> Result<Vec<LicenseIdItem>, ApiError>;

    /// Get license IDs by group.
    ///
    /// Convenience method that calls `get_all_license_ids` with group filter.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The UUID of the license group
    async fn get_license_ids_by_group(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<LicenseIdItem>, ApiError> {
        self.get_all_license_ids(GetAllLicenseIdsRequest::new().with_group_id(group_id))
            .await
    }

    /// Get all license groups with their licenses.
    ///
    /// # Returns
    ///
    /// List of license groups, each containing full license details.
    async fn get_all_license_groups(&self) -> Result<Vec<LicenseGroup>, ApiError>;

    /// Get licenses with pagination and filtering.
    ///
    /// # Arguments
    ///
    /// * `request` - Pagination and filter parameters
    ///
    /// # Returns
    ///
    /// List of full license details with total count in each item.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let licenses = client.get_licenses(
    ///     GetLicensesRequest::new()
    ///         .with_role("uno")
    ///         .with_pagination(1, 20)
    /// ).await?;
    /// ```
    async fn get_licenses(
        &self,
        request: GetLicensesRequest,
    ) -> Result<Vec<UnetworkLicense>, ApiError>;

    /// Get total count of licenses matching the filter.
    ///
    /// This extracts `totalCount` from the first license in the response.
    ///
    /// # Arguments
    ///
    /// * `request` - Filter parameters (pagination is ignored)
    async fn get_licenses_count(
        &self,
        request: GetLicensesRequest,
    ) -> Result<i64, ApiError> {
        let mut req = request;
        req.take = 1; // Only need one to get count
        req.skip = 0;

        let licenses = self.get_licenses(req).await?;
        Ok(licenses.first().and_then(|l| l.total_count).unwrap_or(0))
    }

    /// Get lease details for a license.
    ///
    /// # Arguments
    ///
    /// * `request` - The license ID to get lease details for
    ///
    /// # Returns
    ///
    /// Lease details if the license has an active lease, or None.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let details = client.get_lease_details(
    ///     GetLeaseDetailsRequest::new("0x123...")
    /// ).await?;
    /// ```
    async fn get_lease_details(
        &self,
        request: GetLeaseDetailsRequest,
    ) -> Result<Option<LeaseDetails>, ApiError>;
}

// ============================================================================
// Rewards Trait
// ============================================================================

/// Trait defining Unetwork rewards operations.
///
/// This trait can be implemented by HTTP clients or mock implementations
/// for testing purposes.
///
/// All methods are async and return `Result<T, ApiError>`.
#[async_trait]
pub trait UnetworkRewardsTrait: Send + Sync {
    /// Get reward allocations for a license with pagination.
    ///
    /// # Arguments
    ///
    /// * `request` - The license ID and pagination parameters
    ///
    /// # Returns
    ///
    /// List of reward allocations.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let rewards = client.get_rewards(
    ///     GetRewardsRequest::new("0x123...").with_pagination(0, 20)
    /// ).await?;
    /// ```
    async fn get_rewards(
        &self,
        request: GetRewardsRequest,
    ) -> Result<Vec<RewardAllocation>, ApiError>;

    /// Get reward balance for a license.
    ///
    /// # Arguments
    ///
    /// * `request` - The license ID
    ///
    /// # Returns
    ///
    /// Current reward balance including totals and pending amounts.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let balance = client.get_balance(
    ///     GetRewardsBalanceRequest::new("0x123...")
    /// ).await?;
    /// ```
    async fn get_balance(
        &self,
        request: GetRewardsBalanceRequest,
    ) -> Result<RewardBalance, ApiError>;
}
