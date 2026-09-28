//! Unetwork API request models.
//!
//! Request DTOs for the Unetwork License API endpoints.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request for `licenses_get_all_ids` RPC endpoint.
///
/// All fields are optional - an empty request returns all license IDs.
///
/// # Example
///
/// ```
/// use uno_api::models::request::GetAllLicenseIdsRequest;
///
/// // Get all licenses
/// let all = GetAllLicenseIdsRequest::new();
///
/// // Get leased and bound licenses
/// let filtered = GetAllLicenseIdsRequest::new()
///     .with_leased(true)
///     .with_bound(true);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct GetAllLicenseIdsRequest {
    /// Filter by leased status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leased: Option<bool>,

    /// Filter by bound status (parameter name from API: `p_is_bound`).
    #[serde(rename = "p_is_bound", skip_serializing_if = "Option::is_none")]
    pub is_bound: Option<bool>,

    /// Filter by grouped status (parameter name from API: `p_is_grouped`).
    #[serde(rename = "p_is_grouped", skip_serializing_if = "Option::is_none")]
    pub is_grouped: Option<bool>,

    /// Filter by group ID (parameter name from API: `p_group_id`).
    #[serde(rename = "p_group_id", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
}

impl GetAllLicenseIdsRequest {
    /// Create an empty request (returns all licenses).
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by leased status.
    pub fn with_leased(mut self, is_leased: bool) -> Self {
        self.is_leased = Some(is_leased);
        self
    }

    /// Filter by bound status.
    pub fn with_bound(mut self, is_bound: bool) -> Self {
        self.is_bound = Some(is_bound);
        self
    }

    /// Filter by grouped status.
    pub fn with_grouped(mut self, is_grouped: bool) -> Self {
        self.is_grouped = Some(is_grouped);
        self
    }

    /// Filter by group ID.
    pub fn with_group_id(mut self, group_id: Uuid) -> Self {
        self.group_id = Some(group_id);
        self
    }
}

/// Request for `license_groups_get_all` Edge Function.
///
/// Empty request body - no parameters needed.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct GetAllLicenseGroupsRequest {}

impl GetAllLicenseGroupsRequest {
    /// Create a new empty request.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Request for `licenses_get_licenses` Edge Function.
///
/// Paginated request with multiple filter options.
///
/// # Example
///
/// ```
/// use uno_api::models::request::GetLicensesRequest;
///
/// let request = GetLicensesRequest::new()
///     .with_role("uno")
///     .with_leased(true)
///     .with_enabled(true)
///     .with_pagination(1, 20);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetLicensesRequest {
    /// Role filter (e.g., "uno").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Filter by leased status.
    #[serde(rename = "isLeased", skip_serializing_if = "Option::is_none")]
    pub is_leased: Option<bool>,

    /// Filter by grouped status.
    #[serde(rename = "isGrouped", skip_serializing_if = "Option::is_none")]
    pub is_grouped: Option<bool>,

    /// Filter by bound status.
    #[serde(rename = "isBound", skip_serializing_if = "Option::is_none")]
    pub is_bound: Option<bool>,

    /// Filter by enabled status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Page number (1-based).
    #[serde(default = "default_page")]
    pub page: u32,

    /// Page size.
    #[serde(rename = "pageSize", default = "default_page_size")]
    pub page_size: u32,

    /// Skip N records (alternative pagination).
    #[serde(default)]
    pub skip: u32,

    /// Take N records (alternative pagination).
    #[serde(default = "default_take")]
    pub take: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

fn default_take() -> u32 {
    20
}

impl Default for GetLicensesRequest {
    fn default() -> Self {
        Self {
            role: None,
            is_leased: None,
            is_grouped: None,
            is_bound: None,
            enabled: None,
            page: 1,
            page_size: 20,
            skip: 0,
            take: 20,
        }
    }
}

impl GetLicensesRequest {
    /// Create a new request with default pagination.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set role filter.
    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    /// Filter by leased status.
    pub fn with_leased(mut self, is_leased: bool) -> Self {
        self.is_leased = Some(is_leased);
        self
    }

    /// Filter by grouped status.
    pub fn with_grouped(mut self, is_grouped: bool) -> Self {
        self.is_grouped = Some(is_grouped);
        self
    }

    /// Filter by bound status.
    pub fn with_bound(mut self, is_bound: bool) -> Self {
        self.is_bound = Some(is_bound);
        self
    }

    /// Filter by enabled status.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Set pagination (page-based).
    ///
    /// This also calculates the `skip` and `take` values automatically.
    pub fn with_pagination(mut self, page: u32, page_size: u32) -> Self {
        self.page = page;
        self.page_size = page_size;
        self.skip = (page.saturating_sub(1)) * page_size;
        self.take = page_size;
        self
    }

    /// Set skip/take pagination directly.
    pub fn with_skip_take(mut self, skip: u32, take: u32) -> Self {
        self.skip = skip;
        self.take = take;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_request_serialization() {
        let req = GetAllLicenseIdsRequest::new();
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_filtered_request_serialization() {
        let req = GetAllLicenseIdsRequest::new()
            .with_leased(true)
            .with_bound(true);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"is_leased\":true"));
        assert!(json.contains("\"p_is_bound\":true"));
    }

    #[test]
    fn test_group_id_filter() {
        let uuid = Uuid::parse_str("af01bb17-3eff-48fe-8a6d-193b81de483d").unwrap();
        let req = GetAllLicenseIdsRequest::new().with_group_id(uuid);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"p_group_id\":\"af01bb17-3eff-48fe-8a6d-193b81de483d\""));
    }

    #[test]
    fn test_empty_groups_request() {
        let req = GetAllLicenseGroupsRequest::new();
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_licenses_request_defaults() {
        let req = GetLicensesRequest::new();
        assert_eq!(req.page, 1);
        assert_eq!(req.page_size, 20);
        assert_eq!(req.skip, 0);
        assert_eq!(req.take, 20);
    }

    #[test]
    fn test_pagination_calculation() {
        let req = GetLicensesRequest::new().with_pagination(3, 20);
        assert_eq!(req.page, 3);
        assert_eq!(req.page_size, 20);
        assert_eq!(req.skip, 40);
        assert_eq!(req.take, 20);
    }

    #[test]
    fn test_licenses_request_serialization() {
        let req = GetLicensesRequest::new()
            .with_role("uno")
            .with_leased(true)
            .with_grouped(true)
            .with_bound(true)
            .with_enabled(true)
            .with_pagination(1, 20);

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"role\":\"uno\""));
        assert!(json.contains("\"isLeased\":true"));
        assert!(json.contains("\"isGrouped\":true"));
        assert!(json.contains("\"isBound\":true"));
        assert!(json.contains("\"enabled\":true"));
        assert!(json.contains("\"page\":1"));
        assert!(json.contains("\"pageSize\":20"));
    }

    #[test]
    fn test_skip_take_pagination() {
        let req = GetLicensesRequest::new().with_skip_take(100, 50);
        assert_eq!(req.skip, 100);
        assert_eq!(req.take, 50);
    }

    #[test]
    fn test_builder_chain() {
        let req = GetLicensesRequest::new()
            .with_role("admin")
            .with_leased(false)
            .with_enabled(true);

        assert_eq!(req.role, Some("admin".to_string()));
        assert_eq!(req.is_leased, Some(false));
        assert_eq!(req.enabled, Some(true));
    }
}

// ============================================================================
// Rewards API Requests
// ============================================================================

/// Request for `rewards_get_allocations` RPC endpoint.
///
/// Get reward allocations for a specific license with pagination.
///
/// # Example
///
/// ```
/// use uno_api::models::request::GetRewardsRequest;
///
/// let request = GetRewardsRequest::new("0x1234...")
///     .with_pagination(0, 20);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRewardsRequest {
    /// License ID to get rewards for.
    #[serde(rename = "licenseid")]
    pub license_id: String,

    /// Number of records to skip.
    #[serde(default)]
    pub skip: u32,

    /// Number of records to take.
    #[serde(default = "default_take")]
    pub take: u32,
}

impl GetRewardsRequest {
    /// Create a new request for a license.
    pub fn new(license_id: impl Into<String>) -> Self {
        Self {
            license_id: license_id.into(),
            skip: 0,
            take: 20,
        }
    }

    /// Set pagination.
    pub fn with_pagination(mut self, skip: u32, take: u32) -> Self {
        self.skip = skip;
        self.take = take;
        self
    }
}

/// Request for `rewards_get_balance` RPC endpoint.
///
/// Get the current reward balance for a license.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRewardsBalanceRequest {
    /// License ID to get balance for.
    #[serde(rename = "licenseid")]
    pub license_id: String,
}

impl GetRewardsBalanceRequest {
    /// Create a new request for a license.
    pub fn new(license_id: impl Into<String>) -> Self {
        Self {
            license_id: license_id.into(),
        }
    }
}

// ============================================================================
// Lease Details API Requests
// ============================================================================

/// Request for `get_lease_id_by_license` RPC endpoint.
///
/// Get lease details for a specific license.
///
/// # Example
///
/// ```
/// use uno_api::models::request::GetLeaseDetailsRequest;
///
/// let request = GetLeaseDetailsRequest::new("0x1234...");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetLeaseDetailsRequest {
    /// License ID to get lease details for.
    #[serde(rename = "licenseid")]
    pub license_id: String,
}

impl GetLeaseDetailsRequest {
    /// Create a new request for a license.
    pub fn new(license_id: impl Into<String>) -> Self {
        Self {
            license_id: license_id.into(),
        }
    }
}

#[cfg(test)]
mod rewards_tests {
    use super::*;

    #[test]
    fn test_get_rewards_request() {
        let req = GetRewardsRequest::new("0x1234");
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"licenseid\":\"0x1234\""));
        assert!(json.contains("\"skip\":0"));
        assert!(json.contains("\"take\":20"));
    }

    #[test]
    fn test_get_rewards_request_pagination() {
        let req = GetRewardsRequest::new("0x1234").with_pagination(10, 50);
        assert_eq!(req.skip, 10);
        assert_eq!(req.take, 50);
    }

    #[test]
    fn test_get_rewards_balance_request() {
        let req = GetRewardsBalanceRequest::new("0x5678");
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"licenseid\":\"0x5678\""));
    }

    #[test]
    fn test_get_lease_details_request() {
        let req = GetLeaseDetailsRequest::new("0xabcd");
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"licenseid\":\"0xabcd\""));
    }
}
