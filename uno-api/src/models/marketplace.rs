//! Marketplace DTOs for license distribution and referral sync.

use serde::{Deserialize, Serialize};

/// Request to get claimed licenses from uno-app.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetClaimedLicensesRequest {
    /// ISO timestamp for incremental sync (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// Maximum number of results (default 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl Default for GetClaimedLicensesRequest {
    fn default() -> Self {
        Self {
            since: None,
            limit: Some(100),
        }
    }
}

/// Response DTO for a claimed license.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimedLicenseDto {
    /// The license ID that was claimed.
    pub license_id: String,
    /// When the license was claimed (ISO timestamp).
    pub claimed_at: String,
    /// Referral code used during claim (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    /// Claim token for verification.
    pub claim_token: String,
}

/// Response wrapper for claimed licenses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimedLicensesResponse {
    /// List of claimed licenses.
    pub licenses: Vec<ClaimedLicenseDto>,
    /// Total count of claimed licenses matching criteria.
    pub total: i64,
    /// R3-06: Cursor for next page (timestamp:id format for compound key pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// R3-06: Whether there are more results after this batch.
    #[serde(default)]
    pub has_more: bool,
}

/// Request to sync referrals to uno-app.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncReferralsRequest {
    /// List of referrals to sync.
    pub referrals: Vec<ReferralInput>,
}

/// Input DTO for creating/updating a referral.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralInput {
    /// Username of the referrer.
    pub username: String,
    /// Email address.
    pub email: String,
    /// Country code (ISO 3166-1 alpha-2).
    pub country_code: String,
    /// Unique referral code.
    pub referral_code: String,
    /// Status: "pending" | "active" | "suspended".
    pub status: String,
}

/// Response DTO for a referral.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralDto {
    /// Referral ID.
    pub id: i32,
    /// Username of the referrer.
    pub username: String,
    /// Email address.
    pub email: String,
    /// Country code (ISO 3166-1 alpha-2).
    pub country_code: String,
    /// Unique referral code.
    pub referral_code: String,
    /// Status: "pending" | "active" | "suspended".
    pub status: String,
    /// When the referral was created (ISO timestamp).
    pub created_at: String,
}

/// Response wrapper for referrals list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralsResponse {
    /// List of referrals.
    pub referrals: Vec<ReferralDto>,
    /// Total count.
    pub total: i64,
}

/// Result of a sync operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    /// Number of items created.
    pub created: i32,
    /// Number of items updated.
    pub updated: i32,
    /// Number of items that failed.
    pub failed: i32,
    /// Error messages for failed items.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
}

impl SyncResult {
    /// Create a successful result with no changes.
    pub fn empty() -> Self {
        Self {
            created: 0,
            updated: 0,
            failed: 0,
            errors: Vec::new(),
        }
    }

    /// Check if the sync was fully successful.
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }

    /// Total number of items processed.
    pub fn total_processed(&self) -> i32 {
        self.created + self.updated + self.failed
    }
}

/// Request to publish licenses to marketplace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishLicensesRequest {
    /// List of license IDs to publish.
    pub license_ids: Vec<String>,
}

/// Result of publishing licenses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResult {
    /// Number of licenses successfully published.
    pub published: i32,
    /// Number of licenses that failed to publish.
    pub failed: i32,
    /// Error messages for failed licenses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
}

impl PublishResult {
    /// Check if all licenses were published successfully.
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }
}

// ========== Visitor Stats ==========

/// Request to get visitor stats from uno-app.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVisitorStatsRequest {
    /// Period filter: "daily" | "weekly" | "monthly" | "all"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// Maximum number of countries to return (default 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl Default for GetVisitorStatsRequest {
    fn default() -> Self {
        Self {
            period: Some("monthly".to_string()),
            limit: Some(20),
        }
    }
}

/// Response DTO for visitor stats by country.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitorStatsResponse {
    /// List of country stats.
    pub stats: Vec<CountryVisitorStats>,
    /// Total visitor count.
    pub total_visitors: i64,
    /// Unique visitor count.
    pub unique_visitors: i64,
    /// Period label: "Today", "This Week", "This Month", "All Time".
    pub period: String,
}

/// Visitor stats for a single country.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryVisitorStats {
    /// ISO 3166-1 alpha-2 country code.
    pub country_code: String,
    /// Total visitor count from this country.
    pub visitor_count: i64,
    /// Unique visitor count from this country.
    pub unique_visitors: i64,
}
