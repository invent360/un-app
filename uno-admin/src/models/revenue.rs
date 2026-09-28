use serde::{Deserialize, Serialize};
use crate::api::RewardAllocation;
use super::license::LicenseConfig;

/// Computed revenue for a single allocation with agent share breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedAllocation {
    pub original: RewardAllocation,
    pub license_config: Option<LicenseConfig>,
    pub uno_share_micros: i64,     // UNO's portion after agent deduction
    pub agent_share_micros: i64,   // Agent's commission
    pub agent_name: Option<String>,
}

impl ComputedAllocation {
    /// Create from API allocation + license config
    /// Note: API amount_micros is already UNO's share (ULO paid by Unity)
    pub fn compute(allocation: RewardAllocation, config: Option<&LicenseConfig>) -> Self {
        match config {
            Some(cfg) => {
                // Original amount is UNO's full share (after ULO deduction by Unity)
                // Agent gets their percentage from UNO's share
                let total_uno = allocation.amount_micros;
                let agent_pct = cfg.splits.agent_portion_of_uno();
                let agent_share = (total_uno as f64 * agent_pct) as i64;
                let uno_share = total_uno - agent_share;

                Self {
                    original: allocation,
                    license_config: Some(cfg.clone()),
                    uno_share_micros: uno_share,
                    agent_share_micros: agent_share,
                    agent_name: if cfg.agent_name.is_empty() { None } else { Some(cfg.agent_name.clone()) },
                }
            }
            None => {
                // No config: all goes to UNO (no agent share)
                Self {
                    original: allocation.clone(),
                    license_config: None,
                    uno_share_micros: allocation.amount_micros,
                    agent_share_micros: 0,
                    agent_name: None,
                }
            }
        }
    }

    /// Get original amount in token units
    pub fn original_amount(&self) -> f64 {
        self.original.amount_micros as f64 / 1_000_000.0
    }

    /// Get UNO share in token units
    pub fn uno_share(&self) -> f64 {
        self.uno_share_micros as f64 / 1_000_000.0
    }

    /// Get agent share in token units
    pub fn agent_share(&self) -> f64 {
        self.agent_share_micros as f64 / 1_000_000.0
    }

    /// Format UNO share
    pub fn uno_share_formatted(&self, decimals: usize) -> String {
        format!("{:.1$}", self.uno_share(), decimals)
    }

    /// Format agent share
    pub fn agent_share_formatted(&self, decimals: usize) -> String {
        format!("{:.1$}", self.agent_share(), decimals)
    }

    /// Get license ID
    pub fn license_id(&self) -> &str {
        &self.original.license_id
    }

    /// Get node ID
    pub fn node_id(&self) -> &str {
        &self.original.node_id
    }
}

/// Summary of computed revenues
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevenueSummary {
    pub total_allocations: usize,
    pub total_original_micros: i64,
    pub total_uno_share_micros: i64,
    pub total_agent_share_micros: i64,
    pub unique_licenses: usize,
    pub unique_agents: usize,
    pub unique_nodes: usize,
    // From API summary endpoint
    pub unclaimed_balance_micros: i64,
    pub today_amount_micros: i64,
    pub this_week_amount_micros: i64,
    pub last_7_days_amount_micros: i64,
}

impl RevenueSummary {
    pub fn total_original(&self) -> f64 {
        self.total_original_micros as f64 / 1_000_000.0
    }

    pub fn total_uno_share(&self) -> f64 {
        self.total_uno_share_micros as f64 / 1_000_000.0
    }

    pub fn total_agent_share(&self) -> f64 {
        self.total_agent_share_micros as f64 / 1_000_000.0
    }

    pub fn unclaimed_balance(&self) -> f64 {
        self.unclaimed_balance_micros as f64 / 1_000_000.0
    }

    pub fn today_amount(&self) -> f64 {
        self.today_amount_micros as f64 / 1_000_000.0
    }

    pub fn this_week_amount(&self) -> f64 {
        self.this_week_amount_micros as f64 / 1_000_000.0
    }

    pub fn last_7_days_amount(&self) -> f64 {
        self.last_7_days_amount_micros as f64 / 1_000_000.0
    }

    /// Format with decimals
    pub fn total_original_formatted(&self, decimals: usize) -> String {
        format!("{:.1$}", self.total_original(), decimals)
    }

    pub fn unclaimed_balance_formatted(&self, decimals: usize) -> String {
        format!("{:.1$}", self.unclaimed_balance(), decimals)
    }
}

/// Time range for filtering allocations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TimeRange {
    Today,
    ThisWeek,
    Last7Days,
    ThisMonth,
    Last30Days,
    #[default]
    All,
}

impl TimeRange {
    pub fn label(&self) -> &'static str {
        match self {
            TimeRange::Today => "Today",
            TimeRange::ThisWeek => "This Week",
            TimeRange::Last7Days => "Last 7 Days",
            TimeRange::ThisMonth => "This Month",
            TimeRange::Last30Days => "Last 30 Days",
            TimeRange::All => "All Time",
        }
    }

    pub fn all_options() -> &'static [TimeRange] {
        &[
            TimeRange::Today,
            TimeRange::ThisWeek,
            TimeRange::Last7Days,
            TimeRange::ThisMonth,
            TimeRange::Last30Days,
            TimeRange::All,
        ]
    }
}

/// Uptime data point for analytics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UptimeDataPoint {
    pub date: String,
    pub uptime: f64,  // 0.0 to 1.0
}

impl UptimeDataPoint {
    /// Get uptime as percentage
    pub fn uptime_percentage(&self) -> f64 {
        self.uptime * 100.0
    }
}

/// Daily earnings data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyEarnings {
    pub date: String,
    pub total_micros: i64,
    pub uno_share_micros: i64,
    pub agent_share_micros: i64,
    pub allocation_count: usize,
}

impl DailyEarnings {
    pub fn total(&self) -> f64 {
        self.total_micros as f64 / 1_000_000.0
    }

    pub fn uno_share(&self) -> f64 {
        self.uno_share_micros as f64 / 1_000_000.0
    }

    pub fn agent_share(&self) -> f64 {
        self.agent_share_micros as f64 / 1_000_000.0
    }
}
