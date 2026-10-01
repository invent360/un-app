//! Allocation service for revenue distribution.
//!
//! Provides business logic for allocating revenue pools using the 50/40/10 model.
//! Uses the RevenueSplit struct for precise basis-point calculations.

use crate::models::{Allocation, AgreementVersion, RevenueSplit, RevenueSplitError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Default break-even threshold in micros ($5.60 = 5,600,000 micros).
/// R5-07: This is now a fallback; use AlertPolicy for configurable thresholds.
#[deprecated(since = "0.2.0", note = "Use AlertPolicy from database instead")]
pub const BREAK_EVEN_THRESHOLD_MICROS: i64 = 5_600_000;

/// Default break-even threshold (fallback when policy not loaded).
const DEFAULT_BREAK_EVEN_MICROS: i64 = 5_600_000;

/// Alert policy configuration loaded from database.
/// R5-07: Replaces hardcoded thresholds with configurable policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertPolicy {
    /// Policy identifier code (e.g., "license_break_even").
    pub policy_code: String,
    /// Human-readable policy name.
    pub policy_name: String,
    /// Threshold value in micros.
    pub threshold_micros: i64,
    /// Period type for threshold evaluation.
    pub period_type: Option<String>,
    /// What the threshold measures.
    pub basis: Option<String>,
    /// Whether this policy is active.
    pub is_active: bool,
}

/// Request to record an allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationRequest {
    /// License ID to allocate for.
    pub license_id: String,
    /// Pool amount in micros.
    pub pool_micros: i64,
    /// Currency code (default: USD).
    #[serde(default = "default_currency")]
    pub currency: String,
    /// Agreement version to use (default: 1).
    #[serde(default = "default_version")]
    pub agreement_version: i32,
    /// Period start for this allocation.
    pub period_start: DateTime<Utc>,
    /// Period end for this allocation.
    pub period_end: DateTime<Utc>,
    /// Source of allocation (e.g., "daily_sync").
    pub source: String,
    /// External reference (e.g., upstream transaction ID).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_ref: Option<String>,
    /// R3-08: Provider identifier for deduplication (e.g., "unetwork", "marketplace")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    /// R3-08: Unique event ID from provider for duplicate prevention
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reward_event_id: Option<String>,
    /// R5-07: Referral agent ID (if license was referred).
    /// When None, allocation uses allocate_without_referral() and share goes to reserve.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referral_agent_id: Option<Uuid>,
}

fn default_currency() -> String {
    "USD".to_string()
}

fn default_version() -> i32 {
    1
}

/// Recorded allocation entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationEntry {
    pub id: Uuid,
    pub license_id: String,
    pub agreement_version: i32,
    pub pool_micros: i64,
    pub pool_currency: String,
    pub ulo_micros: i64,
    pub uno_micros: i64,
    pub referral_micros: i64,
    /// R3-07: Reserve allocation for no-referral cases
    #[serde(default)]
    pub reserve_micros: i64,
    pub ulo_bps: u32,
    pub uno_bps: u32,
    pub referral_bps: u32,
    pub remainder_micros: i64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub source: String,
    pub external_ref: Option<String>,
    /// R3-08: Provider identifier for deduplication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    /// R3-08: Unique event ID from provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reward_event_id: Option<String>,
    /// R3-07: Agent receiving referral share (if any)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referral_agent_id: Option<Uuid>,
    pub allocated_at: DateTime<Utc>,
}

impl AllocationEntry {
    /// Create from request and allocation result.
    /// R5-07: Uses referral_agent_id from request for proper attribution.
    pub fn from_request(request: &AllocationRequest, allocation: &Allocation) -> Self {
        Self {
            id: Uuid::new_v4(),
            license_id: request.license_id.clone(),
            agreement_version: request.agreement_version,
            pool_micros: allocation.pool_micros,
            pool_currency: request.currency.clone(),
            ulo_micros: allocation.ulo_micros,
            uno_micros: allocation.uno_micros,
            referral_micros: allocation.referral_micros,
            reserve_micros: allocation.reserve_micros,
            ulo_bps: allocation.ulo_bps,
            uno_bps: allocation.uno_bps,
            referral_bps: allocation.referral_bps,
            remainder_micros: allocation.remainder_micros,
            period_start: request.period_start,
            period_end: request.period_end,
            source: request.source.clone(),
            external_ref: request.external_ref.clone(),
            provider_id: request.provider_id.clone(),
            reward_event_id: request.reward_event_id.clone(),
            referral_agent_id: request.referral_agent_id, // R5-07: Use from request
            allocated_at: Utc::now(),
        }
    }
}

/// Credit expenditure entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditExpenditure {
    pub id: Uuid,
    pub license_id: String,
    pub amount_micros: i64,
    pub currency: String,
    pub expenditure_type: String,
    pub description: Option<String>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub external_ref: Option<String>,
    pub expended_at: DateTime<Utc>,
}

/// Pool balance summary for a license.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolBalanceSummary {
    pub license_id: String,
    pub total_pool_micros: i64,
    pub total_ulo_micros: i64,
    pub total_uno_allocated_micros: i64,
    pub total_referral_micros: i64,
    pub total_credit_expenditure_micros: i64,
    pub net_uno_contribution_micros: i64,
    pub below_break_even_threshold: bool,
    /// R5-07: The threshold used for break-even calculation.
    #[serde(default)]
    pub break_even_threshold_micros: i64,
}

impl PoolBalanceSummary {
    /// Calculate the net UNO contribution using default threshold.
    #[deprecated(since = "0.2.0", note = "Use calculate_net_contribution_with_threshold instead")]
    pub fn calculate_net_contribution(&mut self) {
        self.calculate_net_contribution_with_threshold(DEFAULT_BREAK_EVEN_MICROS);
    }

    /// R5-07: Calculate the net UNO contribution using configurable threshold.
    pub fn calculate_net_contribution_with_threshold(&mut self, threshold_micros: i64) {
        self.net_uno_contribution_micros =
            self.total_uno_allocated_micros - self.total_credit_expenditure_micros;
        self.break_even_threshold_micros = threshold_micros;
        self.below_break_even_threshold =
            self.net_uno_contribution_micros < threshold_micros;
    }

    /// Get the net contribution in dollars.
    pub fn net_contribution_dollars(&self) -> f64 {
        self.net_uno_contribution_micros as f64 / 1_000_000.0
    }

    /// Check if at break-even or above.
    pub fn is_sustainable(&self) -> bool {
        !self.below_break_even_threshold
    }

    /// R5-07: Get threshold gap (how much more needed to reach break-even).
    pub fn threshold_gap_micros(&self) -> i64 {
        if self.below_break_even_threshold {
            self.break_even_threshold_micros - self.net_uno_contribution_micros
        } else {
            0
        }
    }
}

/// Allocation service for revenue distribution.
///
/// This service handles the business logic for allocating revenue pools
/// using the RevenueSplit model. It does not directly interact with the
/// database - that's handled by the host application's repository implementations.
#[derive(Debug, Clone)]
pub struct AllocationService {
    /// Default agreement version to use.
    default_version: i32,
    /// Cached agreement versions.
    versions: Vec<AgreementVersion>,
    /// R5-07: Cached alert policies for threshold lookups.
    policies: Vec<AlertPolicy>,
}

impl Default for AllocationService {
    fn default() -> Self {
        Self::new()
    }
}

impl AllocationService {
    /// Create a new allocation service with default settings.
    pub fn new() -> Self {
        Self {
            default_version: 1,
            versions: vec![],
            policies: vec![],
        }
    }

    /// Create with specific agreement versions.
    pub fn with_versions(versions: Vec<AgreementVersion>) -> Self {
        let default_version = versions
            .iter()
            .find(|v| v.is_active)
            .map(|v| v.version)
            .unwrap_or(1);

        Self {
            default_version,
            versions,
            policies: vec![],
        }
    }

    /// R5-07: Create with agreement versions and alert policies.
    pub fn with_versions_and_policies(
        versions: Vec<AgreementVersion>,
        policies: Vec<AlertPolicy>,
    ) -> Self {
        let default_version = versions
            .iter()
            .find(|v| v.is_active)
            .map(|v| v.version)
            .unwrap_or(1);

        Self {
            default_version,
            versions,
            policies,
        }
    }

    /// R5-07: Set alert policies.
    pub fn set_policies(&mut self, policies: Vec<AlertPolicy>) {
        self.policies = policies;
    }

    /// R5-07: Get an alert policy by code.
    pub fn get_policy(&self, policy_code: &str) -> Option<&AlertPolicy> {
        self.policies
            .iter()
            .find(|p| p.policy_code == policy_code && p.is_active)
    }

    /// R5-07: Get the break-even threshold from policy or use default.
    pub fn get_break_even_threshold(&self) -> i64 {
        self.get_policy("license_break_even")
            .map(|p| p.threshold_micros)
            .unwrap_or(DEFAULT_BREAK_EVEN_MICROS)
    }

    /// Get the default revenue split (50/40/10).
    pub fn default_split(&self) -> RevenueSplit {
        RevenueSplit::default()
    }

    /// Get the split for a specific agreement version.
    pub fn get_split(&self, version: i32) -> Option<RevenueSplit> {
        self.versions
            .iter()
            .find(|v| v.version == version)
            .map(|v| v.split())
    }

    /// Allocate a pool amount using the default split.
    pub fn allocate(&self, pool_micros: i64) -> Result<Allocation, RevenueSplitError> {
        self.default_split().allocate(pool_micros)
    }

    /// Allocate a pool amount using a specific agreement version.
    pub fn allocate_with_version(
        &self,
        pool_micros: i64,
        version: i32,
    ) -> Result<Allocation, RevenueSplitError> {
        let split = self
            .get_split(version)
            .unwrap_or_else(|| self.default_split());
        split.allocate(pool_micros)
    }

    /// Create an allocation entry from a request.
    ///
    /// R5-07: Checks for referral_agent_id in the request:
    /// - If present: Uses standard allocate() with referral_micros going to agent
    /// - If absent: Uses allocate_without_referral() with reserve_micros going to reserve
    pub fn create_entry(
        &self,
        request: &AllocationRequest,
    ) -> Result<AllocationEntry, RevenueSplitError> {
        let split = self
            .get_split(request.agreement_version)
            .unwrap_or_else(|| self.default_split());

        // R5-07: Use appropriate allocation based on referral presence
        let allocation = if request.referral_agent_id.is_some() {
            // Has referral agent: standard allocation with referral_micros
            split.allocate(request.pool_micros)?
        } else {
            // No referral agent: referral share goes to reserve
            split.allocate_without_referral(request.pool_micros)?
        };

        Ok(AllocationEntry::from_request(request, &allocation))
    }

    /// Calculate UNO contribution after credit expenditure.
    pub fn calculate_net_contribution(
        &self,
        total_uno_allocated: i64,
        total_credit_expenditure: i64,
    ) -> i64 {
        total_uno_allocated - total_credit_expenditure
    }

    /// Check if a license is below break-even threshold (using configured policy).
    /// R5-07: Uses policy-configured threshold instead of hardcoded value.
    pub fn is_below_break_even(&self, net_contribution_micros: i64) -> bool {
        net_contribution_micros < self.get_break_even_threshold()
    }

    /// R5-07: Check against a specific threshold (for testing or custom policies).
    pub fn is_below_threshold(&self, net_contribution_micros: i64, threshold_micros: i64) -> bool {
        net_contribution_micros < threshold_micros
    }

    /// Get break-even threshold in dollars (using configured policy).
    /// R5-07: Uses policy-configured threshold instead of hardcoded value.
    pub fn break_even_threshold_dollars(&self) -> f64 {
        self.get_break_even_threshold() as f64 / 1_000_000.0
    }

    /// R5-07: Get default break-even threshold in dollars (static fallback).
    pub fn default_break_even_dollars() -> f64 {
        DEFAULT_BREAK_EVEN_MICROS as f64 / 1_000_000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_service_default() {
        let service = AllocationService::new();
        let allocation = service.allocate(1_000_000).unwrap();

        assert_eq!(allocation.ulo_micros, 500_000);
        assert_eq!(allocation.uno_micros, 400_000);
        assert_eq!(allocation.referral_micros, 100_000);
        assert!(allocation.verify());
    }

    #[test]
    fn test_allocation_entry_with_referral() {
        // R5-07: Test allocation with referral agent (standard path)
        let service = AllocationService::new();
        let referral_agent = Uuid::new_v4();
        let request = AllocationRequest {
            license_id: "test-license-123".to_string(),
            pool_micros: 1_000_000,
            currency: "USD".to_string(),
            agreement_version: 1,
            period_start: Utc::now(),
            period_end: Utc::now(),
            source: "test".to_string(),
            external_ref: None,
            provider_id: None,
            reward_event_id: None,
            referral_agent_id: Some(referral_agent),
        };

        let entry = service.create_entry(&request).unwrap();

        assert_eq!(entry.license_id, "test-license-123");
        assert_eq!(entry.pool_micros, 1_000_000);
        assert_eq!(entry.ulo_micros, 500_000);
        assert_eq!(entry.uno_micros, 400_000);
        assert_eq!(entry.referral_micros, 100_000); // Has referral
        assert_eq!(entry.reserve_micros, 0);         // No reserve
        assert_eq!(entry.referral_agent_id, Some(referral_agent));
    }

    #[test]
    fn test_allocation_entry_without_referral() {
        // R5-07: Test allocation without referral agent (reserve path)
        let service = AllocationService::new();
        let request = AllocationRequest {
            license_id: "test-license-456".to_string(),
            pool_micros: 1_000_000,
            currency: "USD".to_string(),
            agreement_version: 1,
            period_start: Utc::now(),
            period_end: Utc::now(),
            source: "test".to_string(),
            external_ref: None,
            provider_id: None,
            reward_event_id: None,
            referral_agent_id: None, // No referral
        };

        let entry = service.create_entry(&request).unwrap();

        assert_eq!(entry.license_id, "test-license-456");
        assert_eq!(entry.pool_micros, 1_000_000);
        assert_eq!(entry.ulo_micros, 500_000);
        assert_eq!(entry.uno_micros, 400_000);
        assert_eq!(entry.referral_micros, 0);        // No referral
        assert_eq!(entry.reserve_micros, 100_000);   // R5-07: Goes to reserve
        assert_eq!(entry.referral_agent_id, None);
    }

    #[test]
    fn test_net_contribution_calculation() {
        let service = AllocationService::new();

        let net = service.calculate_net_contribution(400_000, 100_000);
        assert_eq!(net, 300_000);

        // R5-07: Test with default threshold (fallback when no policy loaded)
        assert!(service.is_below_break_even(net));
        assert!(!service.is_below_break_even(DEFAULT_BREAK_EVEN_MICROS));
    }

    #[test]
    fn test_pool_balance_summary() {
        let mut summary = PoolBalanceSummary {
            license_id: "test".to_string(),
            total_pool_micros: 10_000_000,
            total_ulo_micros: 5_000_000,
            total_uno_allocated_micros: 4_000_000,
            total_referral_micros: 1_000_000,
            total_credit_expenditure_micros: 500_000,
            net_uno_contribution_micros: 0,
            below_break_even_threshold: false,
            break_even_threshold_micros: 0,
        };

        // R5-07: Use configurable threshold
        summary.calculate_net_contribution_with_threshold(DEFAULT_BREAK_EVEN_MICROS);

        assert_eq!(summary.net_uno_contribution_micros, 3_500_000);
        assert!(summary.below_break_even_threshold); // 3.5M < 5.6M threshold
        assert!(!summary.is_sustainable());
        assert_eq!(summary.threshold_gap_micros(), 2_100_000); // 5.6M - 3.5M
    }

    #[test]
    fn test_break_even_threshold_default() {
        // R5-07: Test default threshold (static fallback)
        assert_eq!(AllocationService::default_break_even_dollars(), 5.60);
    }

    #[test]
    fn test_break_even_with_policy() {
        // R5-07: Test with custom policy threshold
        let policy = AlertPolicy {
            policy_code: "license_break_even".to_string(),
            policy_name: "License Break-Even Alert".to_string(),
            threshold_micros: 7_000_000, // $7.00 instead of $5.60
            period_type: Some("monthly".to_string()),
            basis: Some("uno_net".to_string()),
            is_active: true,
        };

        let service = AllocationService::with_versions_and_policies(vec![], vec![policy]);

        assert_eq!(service.get_break_even_threshold(), 7_000_000);
        assert_eq!(service.break_even_threshold_dollars(), 7.0);

        // 6M is below 7M threshold
        assert!(service.is_below_break_even(6_000_000));
        // 7M meets threshold
        assert!(!service.is_below_break_even(7_000_000));
    }

    #[test]
    fn test_get_policy() {
        let policies = vec![
            AlertPolicy {
                policy_code: "license_break_even".to_string(),
                policy_name: "Break-Even".to_string(),
                threshold_micros: 5_600_000,
                period_type: None,
                basis: None,
                is_active: true,
            },
            AlertPolicy {
                policy_code: "inactive_policy".to_string(),
                policy_name: "Inactive".to_string(),
                threshold_micros: 1_000_000,
                period_type: None,
                basis: None,
                is_active: false,
            },
        ];

        let service = AllocationService::with_versions_and_policies(vec![], policies);

        // Active policy found
        assert!(service.get_policy("license_break_even").is_some());
        // Inactive policy not returned
        assert!(service.get_policy("inactive_policy").is_none());
        // Unknown policy not found
        assert!(service.get_policy("unknown").is_none());
    }
}
