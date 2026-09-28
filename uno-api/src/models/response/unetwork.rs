//! Unetwork API response models.
//!
//! Response DTOs for the Unetwork License API endpoints.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Response item from `licenses_get_all_ids`.
///
/// Contains basic license information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LicenseIdItem {
    /// License ID (hex string, e.g., "0x0111e1758d35...").
    pub id: String,

    /// License alias/name.
    pub alias: Option<String>,

    /// Whether the license is leased.
    #[serde(rename = "isLeased")]
    pub is_leased: bool,
}

/// Full license details from Unetwork API.
///
/// This represents the complete license object returned by the API,
/// including lease information, device binding, activation status, and settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnetworkLicense {
    /// License ID (hex string).
    pub id: String,

    /// Node ID.
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,

    /// Owner wallet address.
    #[serde(rename = "ownerWalletAddress")]
    pub owner_wallet_address: Option<String>,

    /// License alias/name.
    pub alias: Option<String>,

    /// Device ID (if bound).
    #[serde(rename = "deviceId")]
    pub device_id: Option<String>,

    /// Device name (if bound).
    #[serde(rename = "deviceName")]
    pub device_name: Option<String>,

    // === Activation fields ===

    /// Activation start timestamp.
    #[serde(rename = "activationStartAt")]
    pub activation_start_at: Option<DateTime<Utc>>,

    /// Activation end timestamp.
    #[serde(rename = "activationEndAt")]
    pub activation_end_at: Option<DateTime<Utc>>,

    /// Who activated the license.
    #[serde(rename = "activationBy")]
    pub activation_by: Option<String>,

    /// Postponed activation time in milliseconds.
    #[serde(rename = "activationPostponedMs")]
    pub activation_postponed_ms: Option<i64>,

    // === Lease fields ===

    /// User ID who leased the license.
    #[serde(rename = "leaseUserId")]
    pub lease_user_id: Option<Uuid>,

    /// Lease share percentage (revenue share).
    #[serde(rename = "leaseSharePercentage")]
    pub lease_share_percentage: Option<f64>,

    /// Minimum uptime percentage for lease.
    #[serde(rename = "leaseMinUptimePercentage")]
    pub lease_min_uptime_percentage: Option<f64>,

    /// Lease start date.
    #[serde(rename = "leaseFrom")]
    pub lease_from: Option<DateTime<Utc>>,

    /// Lease end date.
    #[serde(rename = "leaseTo")]
    pub lease_to: Option<DateTime<Utc>>,

    // === Status fields ===

    /// Last successful validation timestamp.
    #[serde(rename = "validationLastSuccessAt")]
    pub validation_last_success_at: Option<DateTime<Utc>>,

    /// Uptime percentage (0.0 to 1.0).
    pub uptime: Option<f64>,

    /// Total count (for paginated responses).
    #[serde(rename = "totalCount")]
    pub total_count: Option<i64>,

    /// Whether the license is currently online.
    #[serde(rename = "isOnline")]
    pub is_online: Option<bool>,

    /// License settings.
    pub settings: Option<LicenseSettings>,
}

impl UnetworkLicense {
    /// Check if the license is currently leased.
    pub fn is_leased(&self) -> bool {
        self.lease_user_id.is_some()
    }

    /// Check if the license is bound to a device.
    pub fn is_bound(&self) -> bool {
        self.device_id.is_some()
    }

    /// Check if the license activation is currently valid.
    ///
    /// Returns `true` if the current time is within the activation window.
    pub fn is_activation_valid(&self) -> bool {
        let now = Utc::now();
        match (self.activation_start_at, self.activation_end_at) {
            (Some(start), Some(end)) => now >= start && now <= end,
            (Some(start), None) => now >= start,
            _ => false,
        }
    }

    /// Check if the lease is currently active.
    ///
    /// Returns `true` if the current time is within the lease period.
    pub fn is_lease_active(&self) -> bool {
        let now = Utc::now();
        match (self.lease_from, self.lease_to) {
            (Some(from), Some(to)) => now >= from && now <= to,
            (Some(from), None) => now >= from,
            _ => false,
        }
    }

    /// Get the uptime as a percentage (0-100).
    pub fn uptime_percentage(&self) -> f64 {
        self.uptime.unwrap_or(0.0) * 100.0
    }

    /// Check if the license meets its minimum uptime requirement.
    pub fn meets_uptime_requirement(&self) -> bool {
        match (self.uptime, self.lease_min_uptime_percentage) {
            (Some(uptime), Some(min)) => (uptime * 100.0) >= min,
            _ => true, // No requirement or no uptime data
        }
    }
}

/// License settings embedded in the license object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LicenseSettings {
    /// Settings source (e.g., "license", "wallet").
    pub source: Option<String>,

    /// Whether marketplace is enabled.
    #[serde(rename = "marketplaceOn")]
    pub marketplace_on: Option<bool>,

    /// Default lease duration in months.
    #[serde(rename = "leaseDefaultDurationMonths")]
    pub lease_default_duration_months: Option<i32>,

    /// Default share percentage for leases.
    #[serde(rename = "leaseDefaultSharePercentage")]
    pub lease_default_share_percentage: Option<f64>,

    /// Default minimum uptime percentage for leases.
    #[serde(rename = "leaseDefaultMinUptimePercentage")]
    pub lease_default_min_uptime_percentage: Option<f64>,
}

impl LicenseSettings {
    /// Check if marketplace is enabled.
    pub fn is_marketplace_enabled(&self) -> bool {
        self.marketplace_on.unwrap_or(false)
    }
}

/// License group from `license_groups_get_all`.
///
/// A group contains multiple licenses organized by region or purpose.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LicenseGroup {
    /// Group ID.
    pub id: Uuid,

    /// Group name.
    pub name: String,

    /// Creation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp.
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

    /// Number of licenses in this group.
    #[serde(rename = "licenseCount")]
    pub license_count: i32,

    /// License IDs in this group (simple list).
    #[serde(rename = "licenseIds", default)]
    pub license_ids: Vec<String>,

    /// Full license details for this group.
    #[serde(default)]
    pub licenses: Vec<UnetworkLicense>,
}

impl LicenseGroup {
    /// Get the number of online licenses in this group.
    pub fn online_count(&self) -> usize {
        self.licenses
            .iter()
            .filter(|l| l.is_online.unwrap_or(false))
            .count()
    }

    /// Get the number of leased licenses in this group.
    pub fn leased_count(&self) -> usize {
        self.licenses.iter().filter(|l| l.is_leased()).count()
    }

    /// Get the average uptime across all licenses in this group.
    pub fn average_uptime(&self) -> f64 {
        let uptimes: Vec<f64> = self
            .licenses
            .iter()
            .filter_map(|l| l.uptime)
            .collect();

        if uptimes.is_empty() {
            0.0
        } else {
            uptimes.iter().sum::<f64>() / uptimes.len() as f64
        }
    }

    /// Find a license by ID within this group.
    pub fn find_license(&self, license_id: &str) -> Option<&UnetworkLicense> {
        self.licenses.iter().find(|l| l.id == license_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_id_item_deserialization() {
        let json = r#"{"id":"0x01234","alias":"Test","isLeased":true}"#;
        let item: LicenseIdItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "0x01234");
        assert_eq!(item.alias, Some("Test".to_string()));
        assert!(item.is_leased);
    }

    #[test]
    fn test_license_id_item_null_alias() {
        let json = r#"{"id":"0x01234","alias":null,"isLeased":false}"#;
        let item: LicenseIdItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.alias, None);
        assert!(!item.is_leased);
    }

    #[test]
    fn test_license_is_leased() {
        let mut license = create_test_license();
        assert!(!license.is_leased());

        license.lease_user_id = Some(Uuid::new_v4());
        assert!(license.is_leased());
    }

    #[test]
    fn test_license_is_bound() {
        let mut license = create_test_license();
        assert!(!license.is_bound());

        license.device_id = Some("device-123".to_string());
        assert!(license.is_bound());
    }

    #[test]
    fn test_uptime_percentage() {
        let mut license = create_test_license();
        license.uptime = Some(0.95);
        assert!((license.uptime_percentage() - 95.0).abs() < 0.01);

        license.uptime = None;
        assert!((license.uptime_percentage() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_meets_uptime_requirement() {
        let mut license = create_test_license();
        license.uptime = Some(0.85);
        license.lease_min_uptime_percentage = Some(80.0);
        assert!(license.meets_uptime_requirement());

        license.lease_min_uptime_percentage = Some(90.0);
        assert!(!license.meets_uptime_requirement());
    }

    #[test]
    fn test_license_settings_deserialization() {
        let json = r#"{
            "source": "license",
            "marketplaceOn": true,
            "leaseDefaultDurationMonths": 12,
            "leaseDefaultSharePercentage": 50,
            "leaseDefaultMinUptimePercentage": 80
        }"#;

        let settings: LicenseSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.source, Some("license".to_string()));
        assert!(settings.is_marketplace_enabled());
        assert_eq!(settings.lease_default_duration_months, Some(12));
        assert_eq!(settings.lease_default_share_percentage, Some(50.0));
    }

    #[test]
    fn test_license_group_online_count() {
        let mut group = create_test_group();
        group.licenses = vec![
            create_license_with_online(true),
            create_license_with_online(false),
            create_license_with_online(true),
        ];
        assert_eq!(group.online_count(), 2);
    }

    #[test]
    fn test_license_group_average_uptime() {
        let mut group = create_test_group();
        group.licenses = vec![
            create_license_with_uptime(0.9),
            create_license_with_uptime(0.8),
            create_license_with_uptime(0.7),
        ];
        let avg = group.average_uptime();
        assert!((avg - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_license_group_find_license() {
        let mut group = create_test_group();
        let mut lic1 = create_test_license();
        lic1.id = "0x111".to_string();
        let mut lic2 = create_test_license();
        lic2.id = "0x222".to_string();

        group.licenses = vec![lic1, lic2];

        assert!(group.find_license("0x111").is_some());
        assert!(group.find_license("0x333").is_none());
    }

    // Test helpers
    fn create_test_license() -> UnetworkLicense {
        UnetworkLicense {
            id: "0x123".to_string(),
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

    fn create_test_group() -> LicenseGroup {
        LicenseGroup {
            id: Uuid::new_v4(),
            name: "Test Group".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            license_count: 0,
            license_ids: vec![],
            licenses: vec![],
        }
    }

    fn create_license_with_online(is_online: bool) -> UnetworkLicense {
        let mut lic = create_test_license();
        lic.is_online = Some(is_online);
        lic
    }

    fn create_license_with_uptime(uptime: f64) -> UnetworkLicense {
        let mut lic = create_test_license();
        lic.uptime = Some(uptime);
        lic
    }
}

// ============================================================================
// Rewards API Responses
// ============================================================================

/// Reward allocation from `rewards_get_allocations`.
///
/// Represents a single reward allocation event for a license.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RewardAllocation {
    /// Unique allocation ID.
    pub id: Uuid,

    /// License ID this reward is for.
    #[serde(rename = "licenseId")]
    pub license_id: String,

    /// Reward amount.
    pub amount: f64,

    /// Reward currency/token.
    pub currency: Option<String>,

    /// Allocation period (e.g., "2024-01").
    pub period: Option<String>,

    /// Allocation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    /// Allocation status (e.g., "pending", "paid").
    pub status: Option<String>,

    /// Transaction hash if paid.
    #[serde(rename = "txHash")]
    pub tx_hash: Option<String>,

    /// Total count for pagination.
    #[serde(rename = "totalCount")]
    pub total_count: Option<i64>,
}

impl RewardAllocation {
    /// Check if the reward has been paid.
    pub fn is_paid(&self) -> bool {
        self.status.as_deref() == Some("paid") || self.tx_hash.is_some()
    }

    /// Check if the reward is pending.
    pub fn is_pending(&self) -> bool {
        self.status.as_deref() == Some("pending")
    }
}

/// Reward balance from `rewards_get_balance`.
///
/// Represents the current reward balance for a license.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RewardBalance {
    /// License ID.
    #[serde(rename = "licenseId")]
    pub license_id: String,

    /// Total earned rewards.
    #[serde(rename = "totalEarned")]
    pub total_earned: f64,

    /// Total paid out rewards.
    #[serde(rename = "totalPaid")]
    pub total_paid: f64,

    /// Pending/unpaid balance.
    #[serde(rename = "pendingBalance")]
    pub pending_balance: f64,

    /// Currency/token type.
    pub currency: Option<String>,

    /// Last update timestamp.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl RewardBalance {
    /// Get the available balance (total earned - total paid).
    pub fn available_balance(&self) -> f64 {
        self.total_earned - self.total_paid
    }
}

// ============================================================================
// Lease Details API Responses
// ============================================================================

/// Lease details from `get_lease_id_by_license`.
///
/// Full lease information for a license.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaseDetails {
    /// Lease ID.
    pub id: Uuid,

    /// License ID.
    #[serde(rename = "licenseId")]
    pub license_id: String,

    /// Lessee user ID.
    #[serde(rename = "userId")]
    pub user_id: Option<Uuid>,

    /// Lease share percentage (operator's share).
    #[serde(rename = "sharePercentage")]
    pub share_percentage: Option<f64>,

    /// Minimum uptime percentage required.
    #[serde(rename = "minUptimePercentage")]
    pub min_uptime_percentage: Option<f64>,

    /// Lease start date.
    #[serde(rename = "leaseFrom")]
    pub lease_from: Option<DateTime<Utc>>,

    /// Lease end date.
    #[serde(rename = "leaseTo")]
    pub lease_to: Option<DateTime<Utc>>,

    /// Lease creation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    /// Lease status (e.g., "active", "expired", "terminated").
    pub status: Option<String>,

    /// Current uptime for this lease period.
    #[serde(rename = "currentUptime")]
    pub current_uptime: Option<f64>,

    /// Total rewards earned during this lease.
    #[serde(rename = "totalRewards")]
    pub total_rewards: Option<f64>,
}

impl LeaseDetails {
    /// Check if the lease is currently active.
    pub fn is_active(&self) -> bool {
        if self.status.as_deref() == Some("active") {
            return true;
        }

        let now = Utc::now();
        match (self.lease_from, self.lease_to) {
            (Some(from), Some(to)) => now >= from && now <= to,
            (Some(from), None) => now >= from,
            _ => false,
        }
    }

    /// Check if the lease has expired.
    pub fn is_expired(&self) -> bool {
        if self.status.as_deref() == Some("expired") {
            return true;
        }

        if let Some(to) = self.lease_to {
            return Utc::now() > to;
        }
        false
    }

    /// Get current uptime as a percentage (0-100).
    pub fn uptime_percentage(&self) -> f64 {
        self.current_uptime.unwrap_or(0.0) * 100.0
    }

    /// Check if the lease meets the uptime requirement.
    pub fn meets_uptime_requirement(&self) -> bool {
        match (self.current_uptime, self.min_uptime_percentage) {
            (Some(uptime), Some(min)) => (uptime * 100.0) >= min,
            _ => true,
        }
    }

    /// Get the user's share percentage (100 - operator share).
    pub fn user_share_percentage(&self) -> f64 {
        100.0 - self.share_percentage.unwrap_or(0.0)
    }
}

#[cfg(test)]
mod rewards_response_tests {
    use super::*;

    #[test]
    fn test_reward_allocation_is_paid() {
        let mut reward = create_test_reward();
        assert!(!reward.is_paid());

        reward.status = Some("paid".to_string());
        assert!(reward.is_paid());

        reward.status = None;
        reward.tx_hash = Some("0x123".to_string());
        assert!(reward.is_paid());
    }

    #[test]
    fn test_reward_allocation_is_pending() {
        let mut reward = create_test_reward();
        assert!(!reward.is_pending());

        reward.status = Some("pending".to_string());
        assert!(reward.is_pending());
    }

    #[test]
    fn test_reward_balance_available() {
        let balance = RewardBalance {
            license_id: "0x123".to_string(),
            total_earned: 100.0,
            total_paid: 40.0,
            pending_balance: 60.0,
            currency: Some("USDT".to_string()),
            updated_at: None,
        };

        assert!((balance.available_balance() - 60.0).abs() < 0.01);
    }

    #[test]
    fn test_lease_details_is_active() {
        let mut lease = create_test_lease();
        assert!(!lease.is_active());

        lease.status = Some("active".to_string());
        assert!(lease.is_active());
    }

    #[test]
    fn test_lease_details_uptime() {
        let mut lease = create_test_lease();
        lease.current_uptime = Some(0.95);
        assert!((lease.uptime_percentage() - 95.0).abs() < 0.01);
    }

    #[test]
    fn test_lease_meets_uptime_requirement() {
        let mut lease = create_test_lease();
        lease.current_uptime = Some(0.85);
        lease.min_uptime_percentage = Some(80.0);
        assert!(lease.meets_uptime_requirement());

        lease.min_uptime_percentage = Some(90.0);
        assert!(!lease.meets_uptime_requirement());
    }

    #[test]
    fn test_lease_user_share() {
        let mut lease = create_test_lease();
        lease.share_percentage = Some(30.0);
        assert!((lease.user_share_percentage() - 70.0).abs() < 0.01);
    }

    fn create_test_reward() -> RewardAllocation {
        RewardAllocation {
            id: Uuid::new_v4(),
            license_id: "0x123".to_string(),
            amount: 10.0,
            currency: None,
            period: None,
            created_at: None,
            status: None,
            tx_hash: None,
            total_count: None,
        }
    }

    fn create_test_lease() -> LeaseDetails {
        LeaseDetails {
            id: Uuid::new_v4(),
            license_id: "0x123".to_string(),
            user_id: None,
            share_percentage: None,
            min_uptime_percentage: None,
            lease_from: None,
            lease_to: None,
            created_at: None,
            status: None,
            current_uptime: None,
            total_rewards: None,
        }
    }
}
