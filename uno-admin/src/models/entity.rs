//! Database entity models
//!
//! These models map directly to database tables.

use serde::{Deserialize, Serialize};

/// Agent entity as stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEntity {
    pub id: String,
    pub name: String,
    pub email: String,
    pub country: String,
    pub commission_percent: f64,
    pub referral_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Data for creating a new agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAgent {
    pub name: String,
    pub email: String,
    pub country: String,
    pub commission_percent: f64,
    pub referral_code: Option<String>,
}

impl NewAgent {
    pub fn new(name: String, email: String, country: String, commission_percent: f64) -> Self {
        Self {
            name,
            email,
            country,
            commission_percent,
            referral_code: None,
        }
    }

    pub fn with_referral_code(mut self, code: String) -> Self {
        self.referral_code = Some(code);
        self
    }
}

/// License entity as stored in the database
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseEntity {
    pub id: String,
    pub license_id: String,
    pub node_id: String,
    pub lease_code: Option<String>,
    pub alias: Option<String>,
    pub agent_id: String,
    pub ulo_name: String,
    pub uno_share: f64,
    pub agent_share: f64,
    pub ulo_share: f64,
    pub uptime: f64,
    pub is_online: bool,
    pub created_at: String,
    pub updated_at: String,
    // Extended fields from LicenseApi
    pub owner_wallet_address: Option<String>,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub activation_start_at: Option<String>,
    pub activation_end_at: Option<String>,
    pub activation_by: Option<String>,
    pub activation_postponed_ms: Option<i64>,
    pub lease_user_id: Option<String>,
    pub lease_share_percentage: f64,
    pub lease_min_uptime_percentage: f64,
    pub lease_from: Option<String>,
    pub lease_to: Option<String>,
    pub validation_last_success_at: Option<String>,
    pub settings: Option<String>,
    pub synced_at: Option<String>,
    // Marketplace fields
    pub is_on_marketplace: bool,
    pub is_on_uno_marketplace: bool,
    pub is_published: bool,
    pub marketplace_status: Option<String>,
    pub marketplace_referral_code: Option<String>,
}

/// Data for creating a new license
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLicense {
    pub license_id: String,
    pub node_id: String,
    pub lease_code: Option<String>,
    pub alias: Option<String>,
    pub agent_id: String,
    pub ulo_name: String,
    pub uno_share: f64,
    pub agent_share: f64,
    pub ulo_share: f64,
}

impl NewLicense {
    pub fn new(
        license_id: String,
        node_id: String,
        agent_id: String,
        ulo_name: String,
    ) -> Self {
        Self {
            license_id,
            node_id,
            lease_code: None,
            alias: None,
            agent_id,
            ulo_name,
            uno_share: 47.0,
            agent_share: 3.0,
            ulo_share: 50.0,
        }
    }

    pub fn with_splits(mut self, uno: f64, agent: f64, ulo: f64) -> Self {
        self.uno_share = uno;
        self.agent_share = agent;
        self.ulo_share = ulo;
        self
    }

    pub fn with_lease_code(mut self, code: String) -> Self {
        self.lease_code = Some(code);
        self
    }

    pub fn with_alias(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }
}

impl LicenseEntity {
    /// Get shortened license ID for display
    pub fn license_id_short(&self) -> String {
        if self.license_id.len() > 16 {
            format!(
                "{}...{}",
                &self.license_id[..10],
                &self.license_id[self.license_id.len() - 6..]
            )
        } else {
            self.license_id.clone()
        }
    }

    /// Get shortened node ID for display
    pub fn node_id_short(&self) -> String {
        if self.node_id.len() > 16 {
            format!(
                "{}...{}",
                &self.node_id[..10],
                &self.node_id[self.node_id.len() - 6..]
            )
        } else {
            self.node_id.clone()
        }
    }

    /// Get uptime as percentage
    pub fn uptime_percentage(&self) -> f64 {
        self.uptime * 100.0
    }
}

/// Node entity as stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEntity {
    pub id: String,
    pub node_id: String,
    pub name: Option<String>,
    pub total_licenses: i32,
    pub online_licenses: i32,
    pub total_earnings_micros: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// Data for creating a new node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewNode {
    pub node_id: String,
    pub name: Option<String>,
}

impl NewNode {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            name: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

impl NodeEntity {
    /// Get shortened node ID for display
    pub fn node_id_short(&self) -> String {
        if self.node_id.len() > 16 {
            format!(
                "{}...{}",
                &self.node_id[..10],
                &self.node_id[self.node_id.len() - 6..]
            )
        } else {
            self.node_id.clone()
        }
    }

    /// Get total earnings in token units
    pub fn total_earnings(&self) -> f64 {
        self.total_earnings_micros as f64 / 1_000_000.0
    }

    /// Get online percentage
    pub fn online_percentage(&self) -> f64 {
        if self.total_licenses == 0 {
            0.0
        } else {
            (self.online_licenses as f64 / self.total_licenses as f64) * 100.0
        }
    }
}

/// Reward entity as stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardEntity {
    pub id: String,
    pub user_id: String,
    pub reward_type: String,
    pub description: Option<String>,
    pub node_id: String,
    pub license_id: String,
    pub license_lease_id: String,
    pub task_key: Option<String>,
    pub task_metadata: Option<String>,
    pub completed_at: String,
    pub created_at: String,
    pub amount_micros: i64,
}

/// Data for creating a new reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewReward {
    /// Optional ID - if provided, use this instead of generating new UUID
    pub id: Option<String>,
    pub user_id: String,
    pub reward_type: String,
    pub description: Option<String>,
    pub node_id: String,
    pub license_id: String,
    pub license_lease_id: String,
    pub task_key: Option<String>,
    pub task_metadata: Option<String>,
    pub completed_at: String,
    pub amount_micros: i64,
}

impl NewReward {
    pub fn new(
        user_id: String,
        reward_type: String,
        node_id: String,
        license_id: String,
        license_lease_id: String,
        completed_at: String,
        amount_micros: i64,
    ) -> Self {
        Self {
            id: None,
            user_id,
            reward_type,
            description: None,
            node_id,
            license_id,
            license_lease_id,
            task_key: None,
            task_metadata: None,
            completed_at,
            amount_micros,
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_task(mut self, key: String, metadata: Option<String>) -> Self {
        self.task_key = Some(key);
        self.task_metadata = metadata;
        self
    }
}

impl RewardEntity {
    /// Get amount in token units (from micros)
    pub fn amount(&self) -> f64 {
        self.amount_micros as f64 / 1_000_000.0
    }

    /// Get shortened license ID for display
    pub fn license_id_short(&self) -> String {
        if self.license_id.len() > 16 {
            format!(
                "{}...{}",
                &self.license_id[..10],
                &self.license_id[self.license_id.len() - 6..]
            )
        } else {
            self.license_id.clone()
        }
    }
}

/// License analytics entity for daily uptime records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseAnalyticsEntity {
    pub license_id: String,
    pub date: String,
    pub uptime: f64,
    pub required_uptime: f64,
    pub created_at: String,
    pub updated_at: String,
}

impl LicenseAnalyticsEntity {
    /// Check if uptime meets required threshold
    pub fn meets_requirement(&self) -> bool {
        self.uptime >= self.required_uptime
    }

    /// Get uptime as percentage
    pub fn uptime_percentage(&self) -> f64 {
        self.uptime * 100.0
    }

    /// Get required uptime as percentage
    pub fn required_uptime_percentage(&self) -> f64 {
        self.required_uptime * 100.0
    }
}

/// Data for creating/updating license analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLicenseAnalytics {
    pub license_id: String,
    pub date: String,
    pub uptime: f64,
    pub required_uptime: f64,
}

impl NewLicenseAnalytics {
    pub fn new(license_id: String, date: String, uptime: f64, required_uptime: f64) -> Self {
        Self {
            license_id,
            date,
            uptime,
            required_uptime,
        }
    }
}

/// Data for creating/updating a license from API data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLicenseFromApi {
    pub license_id: String,
    pub node_id: String,
    pub owner_wallet_address: Option<String>,
    pub alias: Option<String>,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub activation_start_at: Option<String>,
    pub activation_end_at: Option<String>,
    pub activation_by: Option<String>,
    pub activation_postponed_ms: Option<i64>,
    pub lease_user_id: Option<String>,
    pub lease_share_percentage: f64,
    pub lease_min_uptime_percentage: f64,
    pub lease_from: Option<String>,
    pub lease_to: Option<String>,
    pub validation_last_success_at: Option<String>,
    pub uptime: f64,
    pub settings: Option<String>,
    pub is_online: bool,
}

/// Pagination parameters for list queries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-indexed)
    pub page: u32,
    /// Items per page
    pub limit: u32,
}

impl PaginationParams {
    pub fn new(page: u32, limit: u32) -> Self {
        Self { page, limit }
    }

    /// Calculate offset for database query
    pub fn offset(&self) -> u32 {
        if self.page == 0 {
            0
        } else {
            (self.page - 1) * self.limit
        }
    }
}

/// Paginated result wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, total: i64, page: u32, limit: u32) -> Self {
        let total_pages = if limit == 0 {
            0
        } else {
            ((total as u32) + limit - 1) / limit
        };
        Self {
            items,
            total,
            page,
            limit,
            total_pages,
        }
    }
}

/// Parameters for listing rewards by license ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRewardsByLicenseParams {
    pub license_id: String,
    pub pagination: PaginationParams,
}

/// Parameters for listing rewards by date range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRewardsByDateParams {
    /// Start date (inclusive) in ISO format
    pub start_date: String,
    /// End date (inclusive) in ISO format
    pub end_date: String,
    pub pagination: PaginationParams,
}

/// Parameters for listing rewards by license ID and date range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRewardsByLicenseAndDateParams {
    pub license_id: String,
    /// Start date (inclusive) in ISO format
    pub start_date: String,
    /// End date (inclusive) in ISO format
    pub end_date: String,
    pub pagination: PaginationParams,
}

/// Sync job status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncJobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Retrying,
}

impl SyncJobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SyncJobStatus::Pending => "pending",
            SyncJobStatus::Running => "running",
            SyncJobStatus::Completed => "completed",
            SyncJobStatus::Failed => "failed",
            SyncJobStatus::Retrying => "retrying",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => SyncJobStatus::Pending,
            "running" => SyncJobStatus::Running,
            "completed" => SyncJobStatus::Completed,
            "failed" => SyncJobStatus::Failed,
            "retrying" => SyncJobStatus::Retrying,
            _ => SyncJobStatus::Pending,
        }
    }
}

impl std::fmt::Display for SyncJobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// License reward context for job details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseRewardContext {
    pub license_id: String,
    pub amount_micros: i64,
}

/// Sync job context with per-license breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncJobContext {
    pub license_rewards: Vec<LicenseRewardContext>,
    pub total_amount_micros: i64,
    pub unique_licenses: usize,
}

/// Sync job entity as stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncJobEntity {
    pub id: String,
    pub job_type: String,
    pub target_date: String,
    pub status: String,
    pub attempt_count: i32,
    pub max_attempts: i32,
    pub records_fetched: i32,
    pub records_inserted: i32,
    pub error_message: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub next_retry_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub job_context: Option<String>,
    pub duration_ms: Option<i64>,
    pub job_logs: Option<String>,
}

impl SyncJobEntity {
    /// Get status as enum
    pub fn status_enum(&self) -> SyncJobStatus {
        SyncJobStatus::from_str(&self.status)
    }

    /// Check if job is in a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status_enum(),
            SyncJobStatus::Completed | SyncJobStatus::Failed
        )
    }

    /// Check if job can be retried
    pub fn can_retry(&self) -> bool {
        self.attempt_count < self.max_attempts && !self.is_terminal()
    }

    /// Parse job context from JSON
    pub fn parse_context(&self) -> Option<SyncJobContext> {
        self.job_context.as_ref()
            .and_then(|s| serde_json::from_str(s).ok())
    }

    /// Get formatted duration string
    pub fn duration_formatted(&self) -> String {
        match self.duration_ms {
            Some(ms) if ms >= 60000 => format!("{}m {}s", ms / 60000, (ms % 60000) / 1000),
            Some(ms) if ms >= 1000 => format!("{}.{}s", ms / 1000, (ms % 1000) / 100),
            Some(ms) => format!("{}ms", ms),
            None => "-".to_string(),
        }
    }

    /// Get result outcome (Success | Failure)
    pub fn result_outcome(&self) -> &'static str {
        match self.status.as_str() {
            "completed" => "Success",
            "failed" => "Failure",
            _ => "-",
        }
    }

    /// Parse job_logs into a vector of log lines
    pub fn parse_logs(&self) -> Vec<String> {
        self.job_logs
            .as_ref()
            .map(|s| s.lines().map(String::from).collect())
            .unwrap_or_default()
    }
}

/// Data for creating a new sync job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSyncJob {
    pub job_type: String,
    pub target_date: String,
    pub max_attempts: i32,
}

impl NewSyncJob {
    pub fn rewards_sync(target_date: String) -> Self {
        Self {
            job_type: "rewards_sync".to_string(),
            target_date,
            max_attempts: 3,
        }
    }

    pub fn licenses_sync() -> Self {
        Self {
            job_type: "licenses_sync".to_string(),
            target_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            max_attempts: 3,
        }
    }

    pub fn with_max_attempts(mut self, max: i32) -> Self {
        self.max_attempts = max;
        self
    }
}
