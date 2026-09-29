//! Forecast engine data models
//!
//! This module provides typed structures for the revenue growth forecast calculator.
//! It models a 2,500-license portfolio with task-driven revenue projections.
//!
//! # Overview
//!
//! The forecast engine calculates weekly financial projections including:
//! - Device deployment and churn
//! - Task-based reward pool accumulation
//! - 50/40/10 revenue split (ULO/UNO/Referral)
//! - Operating costs (credits, support, acquisition, coordination)
//! - Cash flow with settlement lag
//!
//! # Revenue Identity
//!
//! For every week: `ulo + uno + referral == pool`

use serde::{Deserialize, Serialize};

/// Maximum forecast horizon in weeks (5 years)
pub const MAX_HORIZON_WEEKS: u32 = 260;

/// Default values for forecast configuration
pub mod defaults {
    pub const WEEKS: u32 = 52;
    pub const CAPACITY: u32 = 2500;
    pub const OPENING: u32 = 0;
    pub const NEW_NET: u32 = 250;
    pub const CHURN: f64 = 0.02;
    pub const SUCCESS: f64 = 0.85;
    pub const CREDIT: f64 = 1.99;
    pub const SUPPORT: f64 = 0.25;
    pub const ACQUISITION: f64 = 300.0;
    pub const COORDINATION: f64 = 150.0;
    pub const MAINTENANCE_ACQUISITION: f64 = 60.0;
    pub const MAINTENANCE_COORDINATION: f64 = 60.0;
    pub const OVERHEAD: f64 = 0.0;
    pub const SETUP: f64 = 450.0;
    pub const CAPITAL: f64 = 0.0;
    pub const AMORT_WEEKS: u32 = 104;
    pub const TAX: f64 = 0.0;
    pub const FEE: f64 = 0.0;
    pub const LAG: u32 = 2;
    pub const OPENING_CASH: f64 = 25000.0;
    pub const UNO_SHARE: f64 = 0.4;
    pub const ULO_SHARE: f64 = 0.5;
    pub const REFERRAL_SHARE: f64 = 0.1;
    pub const UP_USD: f64 = 1.0;
    pub const ANDROID: f64 = 1.0;
    pub const IOS: f64 = 0.0;
    pub const WINDOWS: f64 = 0.0;
}

/// Rate basis for task rewards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RateBasis {
    /// Pool rate (before 50/40/10 split)
    Pool,
    /// Historical UNO 50% share (doubled to reconstruct pool)
    HistoricalUno,
    /// Current ULO share
    Ulo,
}

impl Default for RateBasis {
    fn default() -> Self {
        Self::Pool
    }
}

impl RateBasis {
    /// Get the divisor for converting rate to pool basis
    pub fn divisor(&self, ulo_share: f64) -> f64 {
        match self {
            RateBasis::Pool => 1.0,
            RateBasis::HistoricalUno => 0.5,
            RateBasis::Ulo => ulo_share,
        }
    }
}

/// A single task contributing to the reward pool
///
/// Tasks represent different types of device activities that generate rewards.
/// Each task has its own rate, eligibility, and device support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastTask {
    /// Task name for identification
    pub name: String,

    /// Whether this task is enabled in the forecast
    #[serde(default)]
    pub enabled: bool,

    /// Android device support
    #[serde(default)]
    pub android: bool,

    /// iOS device support
    #[serde(default)]
    pub ios: bool,

    /// Windows device support
    #[serde(default)]
    pub windows: bool,

    /// Daily reward rate in UP (Unetwork Points)
    #[serde(default)]
    pub rate: f64,

    /// Rate basis for conversion
    #[serde(default)]
    pub basis: RateBasis,

    /// Fraction of devices eligible for this task (0.0-1.0)
    #[serde(default = "default_one")]
    pub eligible: f64,

    /// Activity factor (0.0-1.0)
    #[serde(default = "default_one")]
    pub activity: f64,

    /// Start week (1-based)
    #[serde(default = "default_one_u32")]
    pub start: u32,

    /// End week (1-based)
    #[serde(default = "default_max_weeks")]
    pub end: u32,

    /// Daily pool cap in UP (0 = no cap)
    #[serde(default)]
    pub cap: f64,

    /// Extra cost per device-day in USD
    #[serde(default)]
    pub extra: f64,

    /// Whether this is an illustrative placeholder task
    #[serde(default)]
    pub illustrative: bool,
}

fn default_one() -> f64 { 1.0 }
fn default_one_u32() -> u32 { 1 }
fn default_max_weeks() -> u32 { MAX_HORIZON_WEEKS }

impl Default for ForecastTask {
    fn default() -> Self {
        Self {
            name: "New task".to_string(),
            enabled: false,
            android: true,
            ios: false,
            windows: false,
            rate: 0.0,
            basis: RateBasis::Pool,
            eligible: 1.0,
            activity: 1.0,
            start: 1,
            end: MAX_HORIZON_WEEKS,
            cap: 0.0,
            extra: 0.0,
            illustrative: false,
        }
    }
}

impl ForecastTask {
    /// Create the default illustrative task
    pub fn illustrative_default() -> Self {
        Self {
            name: "Illustrative combined reward pool — REPLACE".to_string(),
            enabled: true,
            android: true,
            ios: false,
            windows: false,
            rate: 0.25, // $7.50/month / 30 days
            basis: RateBasis::Pool,
            eligible: 1.0,
            activity: 1.0,
            start: 1,
            end: MAX_HORIZON_WEEKS,
            cap: 0.0,
            extra: 0.0,
            illustrative: true,
        }
    }

    /// Create the default task set (1 illustrative + 9 named tasks)
    pub fn default_tasks() -> Vec<Self> {
        let mut tasks = vec![Self::illustrative_default()];

        let named_tasks = [
            ("Proof of Work", true, false, false),
            ("Scout", true, false, false),
            ("Other", true, false, false),
            ("CLI / Caller-ID", true, false, false),
            ("Runner Calls", true, false, false),
            ("Extended Telemetry", true, false, false),
            ("Ugrid", true, false, false),
            ("Entropy", true, false, false),
            ("GPU Contribution", false, false, true), // Windows only
        ];

        for (name, android, ios, windows) in named_tasks {
            tasks.push(Self {
                name: name.to_string(),
                enabled: false,
                android,
                ios,
                windows,
                rate: 0.0,
                basis: RateBasis::Pool,
                eligible: 1.0,
                activity: 1.0,
                start: 1,
                end: MAX_HORIZON_WEEKS,
                cap: 0.0,
                extra: 0.0,
                illustrative: false,
            });
        }

        tasks
    }

    /// Calculate device mix fraction for this task
    pub fn device_mix(&self, config: &ForecastConfig) -> f64 {
        (if self.android { config.android } else { 0.0 })
            + (if self.ios { config.ios } else { 0.0 })
            + (if self.windows { config.windows } else { 0.0 })
    }

    /// Calculate effective rate in USD per device-day
    pub fn effective_rate(&self, config: &ForecastConfig) -> f64 {
        let divisor = self.basis.divisor(config.ulo);
        if divisor == 0.0 {
            // FC-03: Don't silently zero - this should be caught in validation
            return 0.0;
        }
        self.rate * config.up_usd / divisor
    }
}

/// Forecast configuration containing all input parameters
///
/// All monetary values are in USD unless otherwise noted.
/// All fractions use 0.0-1.0 (not percentages).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastConfig {
    // Timing
    /// Forecast horizon in weeks (1-260)
    #[serde(default = "serde_defaults::weeks")]
    pub weeks: u32,

    // Capacity
    /// Total license capacity
    #[serde(default = "serde_defaults::capacity")]
    pub capacity: u32,

    /// Opening productive licenses
    #[serde(default)]
    pub opening: u32,

    /// Target net additions per week
    #[serde(default = "serde_defaults::new_net")]
    pub new_net: u32,

    // Rates
    /// Weekly churn fraction (0.0-1.0)
    #[serde(default = "serde_defaults::churn")]
    pub churn: f64,

    /// Activation success fraction (0.0-1.0)
    #[serde(default = "serde_defaults::success")]
    pub success: f64,

    // Costs
    /// Credit cost per 30-day block in USD
    #[serde(default = "serde_defaults::credit")]
    pub credit: f64,

    /// Support cost per device-month in USD
    #[serde(default = "serde_defaults::support")]
    pub support: f64,

    /// Growth acquisition cost per week in USD
    #[serde(default = "serde_defaults::acquisition")]
    pub acquisition: f64,

    /// Growth coordinator pay per week in USD
    #[serde(default = "serde_defaults::coordination")]
    pub coordination: f64,

    /// Maintenance acquisition cost per week in USD
    #[serde(default = "serde_defaults::maintenance_acquisition")]
    pub maintenance_acquisition: f64,

    /// Maintenance coordinator pay per week in USD
    #[serde(default = "serde_defaults::maintenance_coordination")]
    pub maintenance_coordination: f64,

    /// Other cash overhead per week in USD
    #[serde(default)]
    pub overhead: f64,

    /// Pre-production setup cost in USD
    #[serde(default = "serde_defaults::setup")]
    pub setup: f64,

    // Capital allocation
    /// Historic license cost to allocate in USD
    #[serde(default)]
    pub capital: f64,

    /// Capital allocation period in weeks
    #[serde(default = "serde_defaults::amort_weeks")]
    pub amort_weeks: u32,

    // Tax and fees
    /// Positive-week tax fraction (0.0-1.0)
    #[serde(default)]
    pub tax: f64,

    /// Fee fraction of UNO rewards (0.0-1.0)
    #[serde(default)]
    pub fee: f64,

    // Cash flow
    /// UNO settlement delay in weeks
    #[serde(default = "serde_defaults::lag")]
    pub lag: u32,

    /// Opening funding in USD
    #[serde(default = "serde_defaults::opening_cash")]
    pub opening_cash: f64,

    // Revenue split
    /// UNO (operator) share fraction (0.0-1.0)
    #[serde(default = "serde_defaults::uno_share")]
    pub uno: f64,

    /// ULO (user) share fraction (0.0-1.0)
    #[serde(default = "serde_defaults::ulo_share")]
    pub ulo: f64,

    /// Referral share fraction (0.0-1.0)
    #[serde(default = "serde_defaults::referral_share")]
    pub referral: f64,

    // Currency
    /// USD per UP (Unetwork Points) realized
    #[serde(default = "serde_defaults::up_usd")]
    pub up_usd: f64,

    // Device mix (must sum to 1.0)
    /// Android device fraction (0.0-1.0)
    #[serde(default = "serde_defaults::android")]
    pub android: f64,

    /// iOS device fraction (0.0-1.0)
    #[serde(default)]
    pub ios: f64,

    /// Windows device fraction (0.0-1.0)
    #[serde(default)]
    pub windows: f64,
}

mod serde_defaults {
    pub fn weeks() -> u32 { super::defaults::WEEKS }
    pub fn capacity() -> u32 { super::defaults::CAPACITY }
    pub fn new_net() -> u32 { super::defaults::NEW_NET }
    pub fn churn() -> f64 { super::defaults::CHURN }
    pub fn success() -> f64 { super::defaults::SUCCESS }
    pub fn credit() -> f64 { super::defaults::CREDIT }
    pub fn support() -> f64 { super::defaults::SUPPORT }
    pub fn acquisition() -> f64 { super::defaults::ACQUISITION }
    pub fn coordination() -> f64 { super::defaults::COORDINATION }
    pub fn maintenance_acquisition() -> f64 { super::defaults::MAINTENANCE_ACQUISITION }
    pub fn maintenance_coordination() -> f64 { super::defaults::MAINTENANCE_COORDINATION }
    pub fn setup() -> f64 { super::defaults::SETUP }
    pub fn amort_weeks() -> u32 { super::defaults::AMORT_WEEKS }
    pub fn lag() -> u32 { super::defaults::LAG }
    pub fn opening_cash() -> f64 { super::defaults::OPENING_CASH }
    pub fn uno_share() -> f64 { super::defaults::UNO_SHARE }
    pub fn ulo_share() -> f64 { super::defaults::ULO_SHARE }
    pub fn referral_share() -> f64 { super::defaults::REFERRAL_SHARE }
    pub fn up_usd() -> f64 { super::defaults::UP_USD }
    pub fn android() -> f64 { super::defaults::ANDROID }
}

impl Default for ForecastConfig {
    fn default() -> Self {
        Self {
            weeks: super::defaults::WEEKS,
            capacity: super::defaults::CAPACITY,
            opening: super::defaults::OPENING,
            new_net: super::defaults::NEW_NET,
            churn: super::defaults::CHURN,
            success: super::defaults::SUCCESS,
            credit: super::defaults::CREDIT,
            support: super::defaults::SUPPORT,
            acquisition: super::defaults::ACQUISITION,
            coordination: super::defaults::COORDINATION,
            maintenance_acquisition: super::defaults::MAINTENANCE_ACQUISITION,
            maintenance_coordination: super::defaults::MAINTENANCE_COORDINATION,
            overhead: super::defaults::OVERHEAD,
            setup: super::defaults::SETUP,
            capital: super::defaults::CAPITAL,
            amort_weeks: super::defaults::AMORT_WEEKS,
            tax: super::defaults::TAX,
            fee: super::defaults::FEE,
            lag: super::defaults::LAG,
            opening_cash: super::defaults::OPENING_CASH,
            uno: super::defaults::UNO_SHARE,
            ulo: super::defaults::ULO_SHARE,
            referral: super::defaults::REFERRAL_SHARE,
            up_usd: super::defaults::UP_USD,
            android: super::defaults::ANDROID,
            ios: super::defaults::IOS,
            windows: super::defaults::WINDOWS,
        }
    }
}

impl ForecastConfig {
    /// Create a preset for the downside scenario ($5/month pool)
    pub fn downside() -> Self {
        Self::default()
    }

    /// Create a preset for the reference scenario ($7.50/month pool)
    pub fn reference() -> Self {
        Self::default()
    }

    /// Create a preset for the upside scenario ($10/month pool)
    pub fn upside() -> Self {
        Self::default()
    }
}

/// A single week's forecast results
///
/// Contains all 31 computed fields for the week.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastRow {
    // Week identifier
    /// Week number (1-based)
    pub week: u32,

    // Device metrics
    /// Opening productive licenses
    pub opening: f64,
    /// Licenses lost to churn
    pub churn: f64,
    /// New licenses added
    pub additions: f64,
    /// Total activation attempts
    pub attempts: f64,
    /// Closing productive licenses
    pub closing: f64,
    /// Total device-days of exposure
    pub device_days: f64,

    // Revenue (Revenue identity: pool = ulo + uno + referral)
    /// Total reward pool in USD
    pub pool: f64,
    /// ULO (user) share in USD
    pub ulo: f64,
    /// Referral share in USD
    pub referral: f64,
    /// UNO (operator) share in USD
    pub uno: f64,

    // Costs
    /// Credit block costs in USD
    pub credits: f64,
    /// Support costs in USD
    pub support: f64,
    /// Task-specific extra costs in USD
    pub task_costs: f64,
    /// Acquisition costs in USD
    pub acquisition: f64,
    /// Coordinator costs in USD
    pub coordination: f64,
    /// Overhead costs in USD
    pub overhead: f64,
    /// Fee costs in USD
    pub fees: f64,
    /// Capital allocation charge in USD
    pub capital_charge: f64,
    /// Tax provision in USD
    pub tax: f64,

    // Profit metrics
    /// Total operating costs in USD
    pub operating_costs: f64,
    /// Pre-tax profit in USD
    pub pretax: f64,
    /// Net profit in USD
    pub profit: f64,
    /// Cumulative profit to date in USD
    pub cumulative_profit: f64,

    // Cash flow metrics
    /// UNO cash received (with settlement lag) in USD
    pub receipts: f64,
    /// Cash costs in USD
    pub cash_costs: f64,
    /// Net cash flow in USD
    pub cash_flow: f64,
    /// Closing cash balance in USD
    pub cash: f64,
    /// Cumulative cash flow in USD
    pub cumulative_cash: f64,
    /// Required funding (peak deficit) in USD
    pub required_funding: f64,
}

impl ForecastRow {
    /// Assert that the revenue identity holds: pool = ulo + uno + referral
    ///
    /// Returns true if the identity holds within floating-point tolerance.
    pub fn revenue_identity_holds(&self) -> bool {
        let sum = self.ulo + self.uno + self.referral;
        (self.pool - sum).abs() < 1e-8
    }

    /// Get all field names for CSV export
    pub fn csv_headers() -> &'static [&'static str] {
        &[
            "week", "opening", "churn", "additions", "attempts", "closing",
            "device_days", "pool", "ulo", "referral", "uno", "credits",
            "support", "task_costs", "acquisition", "coordination", "overhead",
            "fees", "capital_charge", "tax", "operating_costs", "pretax",
            "profit", "cumulative_profit", "receipts", "cash_costs", "cash_flow",
            "cash", "cumulative_cash", "required_funding",
        ]
    }

    /// Convert row to CSV values
    pub fn to_csv_values(&self) -> Vec<String> {
        vec![
            self.week.to_string(),
            format!("{:.2}", self.opening),
            format!("{:.2}", self.churn),
            format!("{:.2}", self.additions),
            format!("{:.2}", self.attempts),
            format!("{:.2}", self.closing),
            format!("{:.2}", self.device_days),
            format!("{:.2}", self.pool),
            format!("{:.2}", self.ulo),
            format!("{:.2}", self.referral),
            format!("{:.2}", self.uno),
            format!("{:.2}", self.credits),
            format!("{:.2}", self.support),
            format!("{:.2}", self.task_costs),
            format!("{:.2}", self.acquisition),
            format!("{:.2}", self.coordination),
            format!("{:.2}", self.overhead),
            format!("{:.2}", self.fees),
            format!("{:.2}", self.capital_charge),
            format!("{:.2}", self.tax),
            format!("{:.2}", self.operating_costs),
            format!("{:.2}", self.pretax),
            format!("{:.2}", self.profit),
            format!("{:.2}", self.cumulative_profit),
            format!("{:.2}", self.receipts),
            format!("{:.2}", self.cash_costs),
            format!("{:.2}", self.cash_flow),
            format!("{:.2}", self.cash),
            format!("{:.2}", self.cumulative_cash),
            format!("{:.2}", self.required_funding),
        ]
    }
}

/// Complete forecast results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastResult {
    /// Weekly forecast rows
    pub rows: Vec<ForecastRow>,
    /// Peak funding required in USD
    pub required_funding: f64,
    /// First week with positive profit (None if never)
    pub first_positive_week: Option<u32>,
    /// Week when cumulative profit reaches zero (None if never)
    pub profit_payback: Option<u32>,
    /// Week when cumulative cash reaches zero (None if never)
    pub cash_payback: Option<u32>,
}

impl ForecastResult {
    /// Export all rows to CSV format
    pub fn to_csv(&self) -> String {
        let mut csv = ForecastRow::csv_headers().join(",");
        csv.push('\n');

        for row in &self.rows {
            csv.push_str(&row.to_csv_values().join(","));
            csv.push('\n');
        }

        csv
    }

    /// Assert revenue identity holds for all rows
    pub fn all_revenue_identities_hold(&self) -> bool {
        self.rows.iter().all(|r| r.revenue_identity_holds())
    }
}

/// A saved forecast scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastScenario {
    /// Schema version for forward compatibility
    pub version: u32,
    /// Scenario name
    pub name: String,
    /// Scenario description
    #[serde(default)]
    pub description: String,
    /// Configuration parameters
    pub config: ForecastConfig,
    /// Task definitions
    pub tasks: Vec<ForecastTask>,
    /// Created timestamp (Unix epoch seconds)
    #[serde(default)]
    pub created_at: i64,
    /// Last modified timestamp (Unix epoch seconds)
    #[serde(default)]
    pub modified_at: i64,
}

impl ForecastScenario {
    /// Current schema version
    pub const CURRENT_VERSION: u32 = 1;

    /// Create a new scenario with the given name
    pub fn new(name: impl Into<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        Self {
            version: Self::CURRENT_VERSION,
            name: name.into(),
            description: String::new(),
            config: ForecastConfig::default(),
            tasks: ForecastTask::default_tasks(),
            created_at: now,
            modified_at: now,
        }
    }

    /// Create the downside scenario preset ($5/month pool)
    pub fn downside() -> Self {
        let mut scenario = Self::new("Downside");
        scenario.description = "$5/month pool - conservative estimate".to_string();
        // Set illustrative task rate to $5/30 = ~$0.167/day
        if let Some(task) = scenario.tasks.first_mut() {
            task.rate = 5.0 / 30.0;
        }
        scenario
    }

    /// Create the reference scenario preset ($7.50/month pool)
    pub fn reference() -> Self {
        let mut scenario = Self::new("Reference");
        scenario.description = "$7.50/month pool - baseline estimate".to_string();
        // Set illustrative task rate to $7.50/30 = $0.25/day
        if let Some(task) = scenario.tasks.first_mut() {
            task.rate = 7.5 / 30.0;
        }
        scenario
    }

    /// Create the upside scenario preset ($10/month pool)
    pub fn upside() -> Self {
        let mut scenario = Self::new("Upside");
        scenario.description = "$10/month pool - optimistic estimate".to_string();
        // Set illustrative task rate to $10/30 = ~$0.333/day
        if let Some(task) = scenario.tasks.first_mut() {
            task.rate = 10.0 / 30.0;
        }
        scenario
    }

    /// Get the three default scenarios
    pub fn default_scenarios() -> Vec<Self> {
        vec![Self::downside(), Self::reference(), Self::upside()]
    }

    /// Update the modified timestamp
    pub fn touch(&mut self) {
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
    }
}

/// Validation error for forecast configuration
#[derive(Debug, Clone, PartialEq)]
pub enum ForecastValidationError {
    /// A field is not a valid number
    NotNumeric(String),
    /// A field must be positive
    MustBePositive(String),
    /// A field must be an integer
    MustBeInteger(String),
    /// Maximum horizon exceeded
    MaxHorizonExceeded(u32),
    /// A field must be in range [0, 1]
    OutOfRange(String),
    /// Success rate must be in (0, 1]
    InvalidSuccessRate,
    /// Opening inventory exceeds capacity
    OpeningExceedsCapacity,
    /// Device mix must sum to 1
    DeviceMixInvalid,
    /// Revenue shares must sum to 1
    SharesInvalid,
    /// Field cannot be negative
    Negative(String),
    /// Illustrative and named tasks cannot be mixed
    IllustrativeMixed,
    /// Task has invalid field
    TaskInvalid { task: String, field: String },
    /// Task factors cannot exceed 1
    TaskFactorExceeds { task: String, field: String },
    /// Task end before start
    TaskEndBeforeStart(String),
    /// Invalid rate basis
    InvalidRateBasis(String),
    /// Task has no supported devices
    TaskNoDevices(String),
    /// ULO basis requires positive ULO share
    UloBasisRequiresShare(String),
    /// UP/USD conversion rate is zero (would cause silent zeroing)
    UpUsdZero,
    /// Amortization weeks is zero (would cause division by zero)
    AmortWeeksZero,
}

impl std::fmt::Display for ForecastValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotNumeric(field) => write!(f, "{} must be numeric", field),
            Self::MustBePositive(field) => write!(f, "{} must be positive", field),
            Self::MustBeInteger(field) => write!(f, "{} must be an integer", field),
            Self::MaxHorizonExceeded(weeks) => write!(f, "Maximum horizon is {} weeks (got {})", MAX_HORIZON_WEEKS, weeks),
            Self::OutOfRange(field) => write!(f, "{} must be between 0 and 1", field),
            Self::InvalidSuccessRate => write!(f, "Success must be in (0, 1]"),
            Self::OpeningExceedsCapacity => write!(f, "Opening productive inventory exceeds capacity"),
            Self::DeviceMixInvalid => write!(f, "Device mix must total 1"),
            Self::SharesInvalid => write!(f, "Reward shares must total 1"),
            Self::Negative(field) => write!(f, "{} cannot be negative", field),
            Self::IllustrativeMixed => write!(f, "Disable the illustrative combined pool before enabling named tasks"),
            Self::TaskInvalid { task, field } => write!(f, "{}: invalid {}", task, field),
            Self::TaskFactorExceeds { task, field } => write!(f, "{}: {} cannot exceed 1", task, field),
            Self::TaskEndBeforeStart(task) => write!(f, "{}: end before start", task),
            Self::InvalidRateBasis(task) => write!(f, "{}: invalid reward basis", task),
            Self::TaskNoDevices(task) => write!(f, "{}: no supported device", task),
            Self::UloBasisRequiresShare(task) => write!(f, "{}: ULO basis needs a positive ULO share", task),
            Self::UpUsdZero => write!(f, "UP/USD conversion rate cannot be zero (would cause silent zeroing of all rewards)"),
            Self::AmortWeeksZero => write!(f, "Capital allocation period cannot be zero (would cause division error)"),
        }
    }
}

impl std::error::Error for ForecastValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ForecastConfig::default();
        assert_eq!(config.weeks, 52);
        assert_eq!(config.capacity, 2500);
        assert!((config.uno + config.ulo + config.referral - 1.0).abs() < 1e-8);
        assert!((config.android + config.ios + config.windows - 1.0).abs() < 1e-8);
    }

    #[test]
    fn test_default_tasks() {
        let tasks = ForecastTask::default_tasks();
        assert_eq!(tasks.len(), 10);
        assert!(tasks[0].illustrative);
        assert!(tasks[0].enabled);
        for task in &tasks[1..] {
            assert!(!task.illustrative);
            assert!(!task.enabled);
        }
    }

    #[test]
    fn test_rate_basis_divisor() {
        assert_eq!(RateBasis::Pool.divisor(0.5), 1.0);
        assert_eq!(RateBasis::HistoricalUno.divisor(0.5), 0.5);
        assert_eq!(RateBasis::Ulo.divisor(0.5), 0.5);
    }

    #[test]
    fn test_scenario_presets() {
        let scenarios = ForecastScenario::default_scenarios();
        assert_eq!(scenarios.len(), 3);
        assert_eq!(scenarios[0].name, "Downside");
        assert_eq!(scenarios[1].name, "Reference");
        assert_eq!(scenarios[2].name, "Upside");
    }

    #[test]
    fn test_csv_headers() {
        let headers = ForecastRow::csv_headers();
        assert_eq!(headers.len(), 30);
        assert_eq!(headers[0], "week");
    }
}
