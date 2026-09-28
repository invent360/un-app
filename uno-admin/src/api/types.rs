use serde::{Deserialize, Serialize};

// ============================================
// REQUEST TYPES
// ============================================

/// Request payload for fetching reward allocations
#[derive(Debug, Clone, Serialize)]
pub struct RewardsAllocationRequest {
    pub skip: u32,
    pub take: u32,
}

/// Request payload for fetching lease by license
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseByLicenseRequest {
    #[serde(rename = "licenseid")]
    pub license_id: String,
}

/// Request payload for fetching license analytics
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseAnalyticsRequest {
    pub license_id: String,
    pub start_date: String,
    pub end_date: String,
}

/// Request payload for fetching allocations summary by license
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationsSummaryByLicenseRequest {
    pub license_id: String,
}

impl AllocationsSummaryByLicenseRequest {
    pub fn new(license_id: impl Into<String>) -> Self {
        Self {
            license_id: license_id.into(),
        }
    }
}

/// Request payload for fetching a license by ID
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseByIdRequest {
    pub role: String,
    pub license_id: String,
    pub skip: u32,
    pub take: u32,
}

impl LicenseByIdRequest {
    /// Create a request to fetch a single license by ID
    pub fn new(license_id: impl Into<String>) -> Self {
        Self {
            role: "uno".to_string(),
            license_id: license_id.into(),
            skip: 0,
            take: 1,
        }
    }

    /// Create a request with custom role
    pub fn with_role(license_id: impl Into<String>, role: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            license_id: license_id.into(),
            skip: 0,
            take: 1,
        }
    }

    /// Create a request with pagination
    pub fn with_pagination(license_id: impl Into<String>, skip: u32, take: u32) -> Self {
        Self {
            role: "uno".to_string(),
            license_id: license_id.into(),
            skip,
            take,
        }
    }
}

impl RewardsAllocationRequest {
    pub fn new(skip: u32, take: u32) -> Self {
        Self { skip, take }
    }

    pub fn first_page(page_size: u32) -> Self {
        Self::new(0, page_size)
    }

    pub fn next_page(&self) -> Self {
        Self::new(self.skip + self.take, self.take)
    }
}

/// A single reward allocation record
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RewardAllocation {
    pub id: String,
    pub user_id: String,
    #[serde(rename = "type")]
    pub allocation_type: String,
    pub description: Option<String>,
    pub node_id: String,
    pub license_id: String,
    pub license_lease_id: String,
    pub task_key: Option<String>,
    pub task_metadata: Option<serde_json::Value>,
    pub completed_at: String,
    pub created_at: String,
    pub amount_micros: i64,
}

impl RewardAllocation {
    /// Convert micros to actual token amount (divide by 1,000,000)
    pub fn amount(&self) -> f64 {
        self.amount_micros as f64 / 1_000_000.0
    }

    /// Format amount with specified decimal places
    pub fn amount_formatted(&self, decimals: usize) -> String {
        format!("{:.1$}", self.amount(), decimals)
    }

    /// Get shortened node ID (first 8 + last 6 chars)
    pub fn node_id_short(&self) -> String {
        if self.node_id.len() > 16 {
            format!("{}...{}", &self.node_id[..10], &self.node_id[self.node_id.len()-6..])
        } else {
            self.node_id.clone()
        }
    }

    /// Get shortened license ID
    pub fn license_id_short(&self) -> String {
        if self.license_id.len() > 16 {
            format!("{}...{}", &self.license_id[..10], &self.license_id[self.license_id.len()-6..])
        } else {
            self.license_id.clone()
        }
    }

    /// Format the completed date (just the date part)
    pub fn completed_date(&self) -> String {
        self.completed_at.split('T').next().unwrap_or(&self.completed_at).to_string()
    }
}

/// Paginated response wrapper
#[derive(Debug, Clone)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub has_more: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: u32, page_size: u32) -> Self {
        let has_more = data.len() as u32 == page_size;
        Self {
            data,
            page,
            page_size,
            has_more,
        }
    }
}

/// Summary statistics for allocations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllocationsSummary {
    pub total_count: usize,
    pub total_amount_micros: i64,
    pub unique_nodes: usize,
    pub unique_licenses: usize,
}

impl AllocationsSummary {
    pub fn from_allocations(allocations: &[RewardAllocation]) -> Self {
        use std::collections::HashSet;

        let nodes: HashSet<_> = allocations.iter().map(|a| &a.node_id).collect();
        let licenses: HashSet<_> = allocations.iter().map(|a| &a.license_id).collect();
        let total_amount: i64 = allocations.iter().map(|a| a.amount_micros).sum();

        Self {
            total_count: allocations.len(),
            total_amount_micros: total_amount,
            unique_nodes: nodes.len(),
            unique_licenses: licenses.len(),
        }
    }

    pub fn total_amount(&self) -> f64 {
        self.total_amount_micros as f64 / 1_000_000.0
    }

    pub fn total_amount_formatted(&self, decimals: usize) -> String {
        format!("{:.1$}", self.total_amount(), decimals)
    }
}

// ============================================
// NEW API RESPONSE TYPES
// ============================================

/// Balance response from rewards_get_balance
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    #[serde(default)]
    pub balance_micros: i64,
}

impl BalanceResponse {
    pub fn balance(&self) -> f64 {
        self.balance_micros as f64 / 1_000_000.0
    }
}

/// Allocations summary from rewards_get_allocations_summary API
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AllocationsSummaryApi {
    #[serde(default)]
    pub total_amount_micros: i64,
    #[serde(default)]
    pub last7_days_amount_micros: i64,
    #[serde(default)]
    pub this_week_amount_micros: i64,
    #[serde(default)]
    pub today_amount_micros: i64,
}

impl AllocationsSummaryApi {
    pub fn total_amount(&self) -> f64 {
        self.total_amount_micros as f64 / 1_000_000.0
    }

    pub fn last_7_days_amount(&self) -> f64 {
        self.last7_days_amount_micros as f64 / 1_000_000.0
    }

    pub fn this_week_amount(&self) -> f64 {
        self.this_week_amount_micros as f64 / 1_000_000.0
    }

    pub fn today_amount(&self) -> f64 {
        self.today_amount_micros as f64 / 1_000_000.0
    }
}

/// License ID response from licenses_get_all_ids
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseIdResponse {
    pub id: String,
    pub alias: String,
    pub is_leased: bool,
}

/// Full license details from licenses_get_licenses
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseApi {
    pub id: String,
    pub node_id: String,
    pub owner_wallet_address: String,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default)]
    pub device_id: Option<String>,
    #[serde(default)]
    pub device_name: Option<String>,
    #[serde(default)]
    pub activation_start_at: Option<String>,
    #[serde(default)]
    pub activation_end_at: Option<String>,
    #[serde(default)]
    pub activation_by: Option<String>,
    #[serde(default)]
    pub activation_postponed_ms: Option<i64>,
    #[serde(default)]
    pub lease_user_id: Option<String>,
    #[serde(default)]
    pub lease_share_percentage: f64,
    #[serde(default)]
    pub lease_min_uptime_percentage: f64,
    #[serde(default)]
    pub lease_from: Option<String>,
    #[serde(default)]
    pub lease_to: Option<String>,
    #[serde(default)]
    pub validation_last_success_at: Option<String>,
    #[serde(default)]
    pub uptime: f64,
    #[serde(default)]
    pub settings: Option<serde_json::Value>,
    #[serde(default)]
    pub is_online: bool,
}

impl LicenseApi {
    /// Get shortened license ID
    pub fn id_short(&self) -> String {
        if self.id.len() > 16 {
            format!("{}...{}", &self.id[..10], &self.id[self.id.len()-6..])
        } else {
            self.id.clone()
        }
    }

    /// Get shortened node ID
    pub fn node_id_short(&self) -> String {
        if self.node_id.len() > 16 {
            format!("{}...{}", &self.node_id[..10], &self.node_id[self.node_id.len()-6..])
        } else {
            self.node_id.clone()
        }
    }

    /// Get uptime as percentage
    pub fn uptime_percentage(&self) -> f64 {
        self.uptime * 100.0
    }

    /// Check if license is currently leased
    pub fn is_leased(&self) -> bool {
        self.lease_user_id.is_some()
    }
}

/// Lease details response from get_lease_id_by_license
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseResponse {
    #[serde(default)]
    pub end_at: Option<String>,
    #[serde(default)]
    pub start_at: Option<String>,
    #[serde(default)]
    pub lease_code: Option<String>,
    #[serde(default)]
    pub share_percentage: f64,
    #[serde(default)]
    pub min_uptime_percentage: f64,
}

/// Uptime analytics data point from license_analytics_get_by_license
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct UptimeAnalyticsPoint {
    pub date: String,
    pub uptime: f64,
}

impl UptimeAnalyticsPoint {
    /// Get uptime as percentage
    pub fn uptime_percentage(&self) -> f64 {
        self.uptime * 100.0
    }
}

/// License settings response from licenses_get_license_settings
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LicenseSettingsResponse {
    #[serde(default)]
    pub lease_default_share_percentage: f64,
    #[serde(default)]
    pub lease_default_min_uptime_percentage: f64,
    #[serde(default)]
    pub is_in_marketplace: bool,
}

/// Wallet settings response from wallet_settings_get
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WalletSettingsResponse {
    #[serde(default)]
    pub receive_marketing_notifications: Option<bool>,
    #[serde(default)]
    pub receive_transactional_notifications: Option<bool>,
}
