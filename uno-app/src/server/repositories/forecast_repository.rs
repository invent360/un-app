//! Forecast repository for scenario-based financial projections
//!
//! Implements CRUD operations for:
//! - Forecast scenarios with versioning and approval
//! - Task configurations within scenarios
//! - Weekly projection results
//! - Golden test fixtures for validation

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Forecast scenario status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "forecast_status", rename_all = "lowercase")]
pub enum ForecastStatus {
    Draft,
    Active,
    Approved,
    Archived,
}

impl std::fmt::Display for ForecastStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForecastStatus::Draft => write!(f, "draft"),
            ForecastStatus::Active => write!(f, "active"),
            ForecastStatus::Approved => write!(f, "approved"),
            ForecastStatus::Archived => write!(f, "archived"),
        }
    }
}

/// Forecast scenario definition
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForecastScenario {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub version: i32,
    pub status: ForecastStatus,

    // Authorship and approval
    pub created_by: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,

    // Time parameters
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub snapshot_date: NaiveDate,
    pub forecast_weeks: i32,

    // Algorithm version for reproducibility
    pub algorithm_version: String,

    // Global assumptions (JSONB)
    pub assumptions: serde_json::Value,

    // Metadata
    pub tags: Vec<String>,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Task configuration within a scenario
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForecastTask {
    pub id: Uuid,
    pub scenario_id: Uuid,
    pub task_code: String,
    pub task_name: String,
    pub country_code: String,

    // Task parameters
    pub device_type: Option<String>,
    pub daily_rate_micros: i64,
    pub max_capacity: Option<i32>,
    pub launch_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,

    // Conversion assumptions
    pub registration_to_claim_rate: f64,
    pub claim_to_activation_rate: f64,

    // Retention assumptions
    pub d7_retention_rate: f64,
    pub d30_retention_rate: f64,
    pub monthly_churn_rate: f64,

    // Productivity assumptions
    pub avg_daily_earnings_micros: i64,
    pub productivity_rate: f64,

    // Cost assumptions
    pub acquisition_cost_per_user_micros: i64,
    pub support_cost_per_user_monthly_micros: i64,

    // Capacity planning
    pub weekly_claim_target: Option<i32>,
    pub total_target: Option<i32>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Weekly projection result
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForecastResult {
    pub id: Uuid,
    pub scenario_id: Uuid,
    pub week_number: i32,
    pub week_start: NaiveDate,
    pub week_end: NaiveDate,
    pub country_code: Option<String>,
    pub task_code: Option<String>,

    // Inventory projections
    pub opening_licenses: i32,
    pub new_claims: i32,
    pub activations: i32,
    pub churned: i32,
    pub closing_licenses: i32,
    pub active_licenses: i32,
    pub productive_licenses: i32,

    // Revenue projections (micros)
    pub gross_revenue_micros: i64,
    pub participant_share_micros: i64,
    pub referral_share_micros: i64,
    pub uno_share_micros: i64,

    // Cost projections (micros)
    pub acquisition_cost_micros: i64,
    pub support_cost_micros: i64,
    pub hosting_cost_micros: i64,
    pub messaging_cost_micros: i64,
    pub other_cost_micros: i64,
    pub total_cost_micros: i64,

    // Profit projections
    pub net_profit_micros: i64,
    pub cumulative_profit_micros: i64,
    pub break_even_reached: bool,

    // Funding projections
    pub funding_required_micros: i64,
    pub cumulative_funding_micros: i64,

    // Metadata
    pub is_actual: bool,

    pub created_at: DateTime<Utc>,
}

/// Golden test fixture
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForecastGoldenFixture {
    pub id: Uuid,
    pub fixture_name: String,
    pub description: Option<String>,

    // Input configuration
    pub input_scenario: serde_json::Value,
    pub input_tasks: serde_json::Value,

    // Expected outputs
    pub expected_results: serde_json::Value,
    pub expected_totals: serde_json::Value,

    // Validation
    pub algorithm_version: String,
    pub tolerance_percentage: f64,

    // Approval
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub finance_approved: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Scenario snapshot for versioning
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForecastScenarioSnapshot {
    pub id: Uuid,
    pub scenario_id: Uuid,
    pub snapshot_version: i32,

    // Full scenario state
    pub scenario_data: serde_json::Value,
    pub tasks_data: serde_json::Value,
    pub results_data: serde_json::Value,

    // Metadata
    pub snapshot_reason: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}

// ============================================
// INPUT STRUCTS
// ============================================

/// Input for creating a scenario
#[derive(Debug, Clone)]
pub struct CreateScenarioInput {
    pub name: String,
    pub description: Option<String>,
    pub created_by: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub snapshot_date: NaiveDate,
    pub forecast_weeks: i32,
    pub algorithm_version: String,
    pub assumptions: serde_json::Value,
    pub tags: Vec<String>,
}

/// Input for updating a scenario
#[derive(Debug, Clone)]
pub struct UpdateScenarioInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub snapshot_date: Option<NaiveDate>,
    pub forecast_weeks: Option<i32>,
    pub assumptions: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
}

/// Input for adding a task
#[derive(Debug, Clone)]
pub struct AddTaskInput {
    pub scenario_id: Uuid,
    pub task_code: String,
    pub task_name: String,
    pub country_code: String,
    pub device_type: Option<String>,
    pub daily_rate_micros: i64,
    pub max_capacity: Option<i32>,
    pub launch_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub registration_to_claim_rate: f64,
    pub claim_to_activation_rate: f64,
    pub d7_retention_rate: f64,
    pub d30_retention_rate: f64,
    pub monthly_churn_rate: f64,
    pub avg_daily_earnings_micros: i64,
    pub productivity_rate: f64,
    pub acquisition_cost_per_user_micros: i64,
    pub support_cost_per_user_monthly_micros: i64,
    pub weekly_claim_target: Option<i32>,
    pub total_target: Option<i32>,
}

/// Input for updating a task
#[derive(Debug, Clone)]
pub struct UpdateTaskInput {
    pub task_name: Option<String>,
    pub device_type: Option<String>,
    pub daily_rate_micros: Option<i64>,
    pub max_capacity: Option<i32>,
    pub launch_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub registration_to_claim_rate: Option<f64>,
    pub claim_to_activation_rate: Option<f64>,
    pub d7_retention_rate: Option<f64>,
    pub d30_retention_rate: Option<f64>,
    pub monthly_churn_rate: Option<f64>,
    pub avg_daily_earnings_micros: Option<i64>,
    pub productivity_rate: Option<f64>,
    pub acquisition_cost_per_user_micros: Option<i64>,
    pub support_cost_per_user_monthly_micros: Option<i64>,
    pub weekly_claim_target: Option<i32>,
    pub total_target: Option<i32>,
}

/// Input for creating a golden fixture
#[derive(Debug, Clone)]
pub struct CreateGoldenFixtureInput {
    pub fixture_name: String,
    pub description: Option<String>,
    pub input_scenario: serde_json::Value,
    pub input_tasks: serde_json::Value,
    pub expected_results: serde_json::Value,
    pub expected_totals: serde_json::Value,
    pub algorithm_version: String,
    pub tolerance_percentage: f64,
}

/// Scenario export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioExport {
    pub scenario: ForecastScenario,
    pub tasks: Vec<ForecastTask>,
    pub results: Vec<ForecastResult>,
    pub exported_at: DateTime<Utc>,
}

/// Validation result against golden fixture
#[derive(Debug, Clone, Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub fixture_name: String,
    pub algorithm_version: String,
    pub tolerance: f64,
    pub differences: Vec<ValidationDifference>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationDifference {
    pub field: String,
    pub expected: String,
    pub actual: String,
    pub difference_pct: f64,
}

// ============================================
// TRAIT DEFINITION
// ============================================

/// Dynamic type alias for ForecastRepository trait object
pub type DynForecastRepository = Arc<dyn ForecastRepository + Send + Sync>;

/// Forecast repository trait
#[async_trait]
pub trait ForecastRepository: Send + Sync {
    // --- Scenario Management ---

    /// Create a new scenario
    async fn create_scenario(&self, input: CreateScenarioInput) -> Result<ForecastScenario, AppError>;

    /// Get scenario by ID
    async fn get_scenario(&self, id: Uuid) -> Result<Option<ForecastScenario>, AppError>;

    /// List scenarios with optional status filter
    async fn list_scenarios(&self, status: Option<ForecastStatus>) -> Result<Vec<ForecastScenario>, AppError>;

    /// Update scenario
    async fn update_scenario(&self, id: Uuid, input: UpdateScenarioInput) -> Result<ForecastScenario, AppError>;

    /// Update scenario status
    async fn update_scenario_status(&self, id: Uuid, status: ForecastStatus) -> Result<ForecastScenario, AppError>;

    /// Approve scenario
    async fn approve_scenario(&self, id: Uuid, approved_by: &str) -> Result<ForecastScenario, AppError>;

    /// Archive scenario
    async fn archive_scenario(&self, id: Uuid) -> Result<ForecastScenario, AppError>;

    /// Delete scenario (only draft)
    async fn delete_scenario(&self, id: Uuid) -> Result<(), AppError>;

    // --- Task Configuration ---

    /// Add task to scenario
    async fn add_task(&self, input: AddTaskInput) -> Result<ForecastTask, AppError>;

    /// Get task by ID
    async fn get_task(&self, id: Uuid) -> Result<Option<ForecastTask>, AppError>;

    /// Get tasks for scenario
    async fn get_scenario_tasks(&self, scenario_id: Uuid) -> Result<Vec<ForecastTask>, AppError>;

    /// Update task
    async fn update_task(&self, id: Uuid, input: UpdateTaskInput) -> Result<ForecastTask, AppError>;

    /// Delete task
    async fn delete_task(&self, id: Uuid) -> Result<(), AppError>;

    // --- Results ---

    /// Save forecast results (bulk)
    async fn save_results(&self, scenario_id: Uuid, results: Vec<ForecastResult>) -> Result<i32, AppError>;

    /// Get results for scenario
    async fn get_results(&self, scenario_id: Uuid) -> Result<Vec<ForecastResult>, AppError>;

    /// Get results by country
    async fn get_results_by_country(&self, scenario_id: Uuid, country_code: &str) -> Result<Vec<ForecastResult>, AppError>;

    /// Clear results for scenario
    async fn clear_results(&self, scenario_id: Uuid) -> Result<i32, AppError>;

    /// Run forecast using database function
    async fn run_forecast_db(&self, scenario_id: Uuid) -> Result<i32, AppError>;

    // --- Golden Fixtures ---

    /// Create golden fixture
    async fn create_golden_fixture(&self, input: CreateGoldenFixtureInput) -> Result<ForecastGoldenFixture, AppError>;

    /// Get golden fixture by name
    async fn get_golden_fixture(&self, name: &str) -> Result<Option<ForecastGoldenFixture>, AppError>;

    /// List golden fixtures
    async fn list_golden_fixtures(&self) -> Result<Vec<ForecastGoldenFixture>, AppError>;

    /// Approve golden fixture
    async fn approve_golden_fixture(&self, id: Uuid, approved_by: &str, finance_approved: bool) -> Result<ForecastGoldenFixture, AppError>;

    // --- Snapshots ---

    /// Create scenario snapshot
    async fn create_snapshot(&self, scenario_id: Uuid, reason: &str, created_by: &str) -> Result<ForecastScenarioSnapshot, AppError>;

    /// Get latest snapshot
    async fn get_latest_snapshot(&self, scenario_id: Uuid) -> Result<Option<ForecastScenarioSnapshot>, AppError>;

    /// Get snapshot by version
    async fn get_snapshot(&self, scenario_id: Uuid, version: i32) -> Result<Option<ForecastScenarioSnapshot>, AppError>;

    /// List snapshots for scenario
    async fn list_snapshots(&self, scenario_id: Uuid) -> Result<Vec<ForecastScenarioSnapshot>, AppError>;

    // --- Export/Import ---

    /// Export scenario using database function
    async fn export_scenario(&self, id: Uuid) -> Result<serde_json::Value, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct ForecastRepositoryImpl {
    pool: ConnectionPool,
}

impl ForecastRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ForecastRepository for ForecastRepositoryImpl {
    async fn create_scenario(&self, input: CreateScenarioInput) -> Result<ForecastScenario, AppError> {
        let scenario = sqlx::query_as::<_, ForecastScenario>(
            r#"
            INSERT INTO forecast_scenarios (
                name, description, created_by, start_date, end_date,
                snapshot_date, forecast_weeks, algorithm_version, assumptions, tags
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(&input.name)
        .bind(&input.description)
        .bind(&input.created_by)
        .bind(input.start_date)
        .bind(input.end_date)
        .bind(input.snapshot_date)
        .bind(input.forecast_weeks)
        .bind(&input.algorithm_version)
        .bind(&input.assumptions)
        .bind(&input.tags)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(scenario)
    }

    async fn get_scenario(&self, id: Uuid) -> Result<Option<ForecastScenario>, AppError> {
        let scenario = sqlx::query_as::<_, ForecastScenario>(
            "SELECT * FROM forecast_scenarios WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(scenario)
    }

    async fn list_scenarios(&self, status: Option<ForecastStatus>) -> Result<Vec<ForecastScenario>, AppError> {
        let scenarios = if let Some(s) = status {
            sqlx::query_as::<_, ForecastScenario>(
                r#"
                SELECT * FROM forecast_scenarios
                WHERE status = $1
                ORDER BY updated_at DESC
                "#,
            )
            .bind(s)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, ForecastScenario>(
                r#"
                SELECT * FROM forecast_scenarios
                WHERE status != 'archived'
                ORDER BY updated_at DESC
                "#,
            )
            .fetch_all(&self.pool)
            .await
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(scenarios)
    }

    async fn update_scenario(&self, id: Uuid, input: UpdateScenarioInput) -> Result<ForecastScenario, AppError> {
        // Build dynamic update query
        let scenario = sqlx::query_as::<_, ForecastScenario>(
            r#"
            UPDATE forecast_scenarios SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                start_date = COALESCE($4, start_date),
                end_date = COALESCE($5, end_date),
                snapshot_date = COALESCE($6, snapshot_date),
                forecast_weeks = COALESCE($7, forecast_weeks),
                assumptions = COALESCE($8, assumptions),
                tags = COALESCE($9, tags),
                notes = COALESCE($10, notes),
                version = version + 1,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&input.name)
        .bind(&input.description)
        .bind(input.start_date)
        .bind(input.end_date)
        .bind(input.snapshot_date)
        .bind(input.forecast_weeks)
        .bind(&input.assumptions)
        .bind(&input.tags)
        .bind(&input.notes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(scenario)
    }

    async fn update_scenario_status(&self, id: Uuid, status: ForecastStatus) -> Result<ForecastScenario, AppError> {
        let scenario = sqlx::query_as::<_, ForecastScenario>(
            r#"
            UPDATE forecast_scenarios
            SET status = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(scenario)
    }

    async fn approve_scenario(&self, id: Uuid, approved_by: &str) -> Result<ForecastScenario, AppError> {
        let scenario = sqlx::query_as::<_, ForecastScenario>(
            r#"
            UPDATE forecast_scenarios
            SET status = 'approved', approved_by = $2, approved_at = NOW(), updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(approved_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(scenario)
    }

    async fn archive_scenario(&self, id: Uuid) -> Result<ForecastScenario, AppError> {
        self.update_scenario_status(id, ForecastStatus::Archived).await
    }

    async fn delete_scenario(&self, id: Uuid) -> Result<(), AppError> {
        // Only allow deletion of draft scenarios
        let result = sqlx::query(
            "DELETE FROM forecast_scenarios WHERE id = $1 AND status = 'draft'",
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::ValidationError("Can only delete draft scenarios".to_string()));
        }

        Ok(())
    }

    async fn add_task(&self, input: AddTaskInput) -> Result<ForecastTask, AppError> {
        let task = sqlx::query_as::<_, ForecastTask>(
            r#"
            INSERT INTO forecast_tasks (
                scenario_id, task_code, task_name, country_code, device_type,
                daily_rate_micros, max_capacity, launch_date, end_date,
                registration_to_claim_rate, claim_to_activation_rate,
                d7_retention_rate, d30_retention_rate, monthly_churn_rate,
                avg_daily_earnings_micros, productivity_rate,
                acquisition_cost_per_user_micros, support_cost_per_user_monthly_micros,
                weekly_claim_target, total_target
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            RETURNING *
            "#,
        )
        .bind(input.scenario_id)
        .bind(&input.task_code)
        .bind(&input.task_name)
        .bind(&input.country_code)
        .bind(&input.device_type)
        .bind(input.daily_rate_micros)
        .bind(input.max_capacity)
        .bind(input.launch_date)
        .bind(input.end_date)
        .bind(input.registration_to_claim_rate)
        .bind(input.claim_to_activation_rate)
        .bind(input.d7_retention_rate)
        .bind(input.d30_retention_rate)
        .bind(input.monthly_churn_rate)
        .bind(input.avg_daily_earnings_micros)
        .bind(input.productivity_rate)
        .bind(input.acquisition_cost_per_user_micros)
        .bind(input.support_cost_per_user_monthly_micros)
        .bind(input.weekly_claim_target)
        .bind(input.total_target)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(task)
    }

    async fn get_task(&self, id: Uuid) -> Result<Option<ForecastTask>, AppError> {
        let task = sqlx::query_as::<_, ForecastTask>(
            "SELECT * FROM forecast_tasks WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(task)
    }

    async fn get_scenario_tasks(&self, scenario_id: Uuid) -> Result<Vec<ForecastTask>, AppError> {
        let tasks = sqlx::query_as::<_, ForecastTask>(
            r#"
            SELECT * FROM forecast_tasks
            WHERE scenario_id = $1
            ORDER BY country_code, task_code
            "#,
        )
        .bind(scenario_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(tasks)
    }

    async fn update_task(&self, id: Uuid, input: UpdateTaskInput) -> Result<ForecastTask, AppError> {
        let task = sqlx::query_as::<_, ForecastTask>(
            r#"
            UPDATE forecast_tasks SET
                task_name = COALESCE($2, task_name),
                device_type = COALESCE($3, device_type),
                daily_rate_micros = COALESCE($4, daily_rate_micros),
                max_capacity = COALESCE($5, max_capacity),
                launch_date = COALESCE($6, launch_date),
                end_date = COALESCE($7, end_date),
                registration_to_claim_rate = COALESCE($8, registration_to_claim_rate),
                claim_to_activation_rate = COALESCE($9, claim_to_activation_rate),
                d7_retention_rate = COALESCE($10, d7_retention_rate),
                d30_retention_rate = COALESCE($11, d30_retention_rate),
                monthly_churn_rate = COALESCE($12, monthly_churn_rate),
                avg_daily_earnings_micros = COALESCE($13, avg_daily_earnings_micros),
                productivity_rate = COALESCE($14, productivity_rate),
                acquisition_cost_per_user_micros = COALESCE($15, acquisition_cost_per_user_micros),
                support_cost_per_user_monthly_micros = COALESCE($16, support_cost_per_user_monthly_micros),
                weekly_claim_target = COALESCE($17, weekly_claim_target),
                total_target = COALESCE($18, total_target),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&input.task_name)
        .bind(&input.device_type)
        .bind(input.daily_rate_micros)
        .bind(input.max_capacity)
        .bind(input.launch_date)
        .bind(input.end_date)
        .bind(input.registration_to_claim_rate)
        .bind(input.claim_to_activation_rate)
        .bind(input.d7_retention_rate)
        .bind(input.d30_retention_rate)
        .bind(input.monthly_churn_rate)
        .bind(input.avg_daily_earnings_micros)
        .bind(input.productivity_rate)
        .bind(input.acquisition_cost_per_user_micros)
        .bind(input.support_cost_per_user_monthly_micros)
        .bind(input.weekly_claim_target)
        .bind(input.total_target)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(task)
    }

    async fn delete_task(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM forecast_tasks WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn save_results(&self, scenario_id: Uuid, results: Vec<ForecastResult>) -> Result<i32, AppError> {
        let mut count = 0;
        for result in results {
            sqlx::query(
                r#"
                INSERT INTO forecast_results (
                    scenario_id, week_number, week_start, week_end, country_code, task_code,
                    opening_licenses, new_claims, activations, churned, closing_licenses,
                    active_licenses, productive_licenses,
                    gross_revenue_micros, participant_share_micros, referral_share_micros, uno_share_micros,
                    acquisition_cost_micros, support_cost_micros, hosting_cost_micros,
                    messaging_cost_micros, other_cost_micros,
                    cumulative_profit_micros, break_even_reached,
                    funding_required_micros, cumulative_funding_micros, is_actual
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27)
                ON CONFLICT (scenario_id, week_number, country_code, task_code) DO UPDATE SET
                    opening_licenses = EXCLUDED.opening_licenses,
                    new_claims = EXCLUDED.new_claims,
                    activations = EXCLUDED.activations,
                    churned = EXCLUDED.churned,
                    closing_licenses = EXCLUDED.closing_licenses,
                    active_licenses = EXCLUDED.active_licenses,
                    productive_licenses = EXCLUDED.productive_licenses,
                    gross_revenue_micros = EXCLUDED.gross_revenue_micros,
                    participant_share_micros = EXCLUDED.participant_share_micros,
                    referral_share_micros = EXCLUDED.referral_share_micros,
                    uno_share_micros = EXCLUDED.uno_share_micros,
                    acquisition_cost_micros = EXCLUDED.acquisition_cost_micros,
                    support_cost_micros = EXCLUDED.support_cost_micros,
                    hosting_cost_micros = EXCLUDED.hosting_cost_micros,
                    messaging_cost_micros = EXCLUDED.messaging_cost_micros,
                    other_cost_micros = EXCLUDED.other_cost_micros,
                    cumulative_profit_micros = EXCLUDED.cumulative_profit_micros,
                    break_even_reached = EXCLUDED.break_even_reached,
                    funding_required_micros = EXCLUDED.funding_required_micros,
                    cumulative_funding_micros = EXCLUDED.cumulative_funding_micros
                "#,
            )
            .bind(scenario_id)
            .bind(result.week_number)
            .bind(result.week_start)
            .bind(result.week_end)
            .bind(&result.country_code)
            .bind(&result.task_code)
            .bind(result.opening_licenses)
            .bind(result.new_claims)
            .bind(result.activations)
            .bind(result.churned)
            .bind(result.closing_licenses)
            .bind(result.active_licenses)
            .bind(result.productive_licenses)
            .bind(result.gross_revenue_micros)
            .bind(result.participant_share_micros)
            .bind(result.referral_share_micros)
            .bind(result.uno_share_micros)
            .bind(result.acquisition_cost_micros)
            .bind(result.support_cost_micros)
            .bind(result.hosting_cost_micros)
            .bind(result.messaging_cost_micros)
            .bind(result.other_cost_micros)
            .bind(result.cumulative_profit_micros)
            .bind(result.break_even_reached)
            .bind(result.funding_required_micros)
            .bind(result.cumulative_funding_micros)
            .bind(result.is_actual)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            count += 1;
        }

        Ok(count)
    }

    async fn get_results(&self, scenario_id: Uuid) -> Result<Vec<ForecastResult>, AppError> {
        let results = sqlx::query_as::<_, ForecastResult>(
            r#"
            SELECT * FROM forecast_results
            WHERE scenario_id = $1
            ORDER BY week_number, country_code, task_code
            "#,
        )
        .bind(scenario_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn get_results_by_country(&self, scenario_id: Uuid, country_code: &str) -> Result<Vec<ForecastResult>, AppError> {
        let results = sqlx::query_as::<_, ForecastResult>(
            r#"
            SELECT * FROM forecast_results
            WHERE scenario_id = $1 AND country_code = $2
            ORDER BY week_number, task_code
            "#,
        )
        .bind(scenario_id)
        .bind(country_code)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn clear_results(&self, scenario_id: Uuid) -> Result<i32, AppError> {
        let result = sqlx::query("DELETE FROM forecast_results WHERE scenario_id = $1")
            .bind(scenario_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() as i32)
    }

    async fn run_forecast_db(&self, scenario_id: Uuid) -> Result<i32, AppError> {
        let (count,): (i32,) = sqlx::query_as("SELECT run_forecast($1)")
            .bind(scenario_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(count)
    }

    async fn create_golden_fixture(&self, input: CreateGoldenFixtureInput) -> Result<ForecastGoldenFixture, AppError> {
        let fixture = sqlx::query_as::<_, ForecastGoldenFixture>(
            r#"
            INSERT INTO forecast_golden_fixtures (
                fixture_name, description, input_scenario, input_tasks,
                expected_results, expected_totals, algorithm_version, tolerance_percentage
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(&input.fixture_name)
        .bind(&input.description)
        .bind(&input.input_scenario)
        .bind(&input.input_tasks)
        .bind(&input.expected_results)
        .bind(&input.expected_totals)
        .bind(&input.algorithm_version)
        .bind(input.tolerance_percentage)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(fixture)
    }

    async fn get_golden_fixture(&self, name: &str) -> Result<Option<ForecastGoldenFixture>, AppError> {
        let fixture = sqlx::query_as::<_, ForecastGoldenFixture>(
            "SELECT * FROM forecast_golden_fixtures WHERE fixture_name = $1",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(fixture)
    }

    async fn list_golden_fixtures(&self) -> Result<Vec<ForecastGoldenFixture>, AppError> {
        let fixtures = sqlx::query_as::<_, ForecastGoldenFixture>(
            "SELECT * FROM forecast_golden_fixtures ORDER BY fixture_name",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(fixtures)
    }

    async fn approve_golden_fixture(&self, id: Uuid, approved_by: &str, finance_approved: bool) -> Result<ForecastGoldenFixture, AppError> {
        let fixture = sqlx::query_as::<_, ForecastGoldenFixture>(
            r#"
            UPDATE forecast_golden_fixtures
            SET approved_by = $2, approved_at = NOW(), finance_approved = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(approved_by)
        .bind(finance_approved)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(fixture)
    }

    async fn create_snapshot(&self, scenario_id: Uuid, reason: &str, created_by: &str) -> Result<ForecastScenarioSnapshot, AppError> {
        // Get next version
        let (next_version,): (i32,) = sqlx::query_as(
            "SELECT COALESCE(MAX(snapshot_version), 0) + 1 FROM forecast_scenario_snapshots WHERE scenario_id = $1",
        )
        .bind(scenario_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Export current state
        let export = self.export_scenario(scenario_id).await?;

        let snapshot = sqlx::query_as::<_, ForecastScenarioSnapshot>(
            r#"
            INSERT INTO forecast_scenario_snapshots (
                scenario_id, snapshot_version, scenario_data, tasks_data, results_data,
                snapshot_reason, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(scenario_id)
        .bind(next_version)
        .bind(&export["scenario"])
        .bind(&export["tasks"])
        .bind(&export["results"])
        .bind(reason)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(snapshot)
    }

    async fn get_latest_snapshot(&self, scenario_id: Uuid) -> Result<Option<ForecastScenarioSnapshot>, AppError> {
        let snapshot = sqlx::query_as::<_, ForecastScenarioSnapshot>(
            r#"
            SELECT * FROM forecast_scenario_snapshots
            WHERE scenario_id = $1
            ORDER BY snapshot_version DESC
            LIMIT 1
            "#,
        )
        .bind(scenario_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(snapshot)
    }

    async fn get_snapshot(&self, scenario_id: Uuid, version: i32) -> Result<Option<ForecastScenarioSnapshot>, AppError> {
        let snapshot = sqlx::query_as::<_, ForecastScenarioSnapshot>(
            r#"
            SELECT * FROM forecast_scenario_snapshots
            WHERE scenario_id = $1 AND snapshot_version = $2
            "#,
        )
        .bind(scenario_id)
        .bind(version)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(snapshot)
    }

    async fn list_snapshots(&self, scenario_id: Uuid) -> Result<Vec<ForecastScenarioSnapshot>, AppError> {
        let snapshots = sqlx::query_as::<_, ForecastScenarioSnapshot>(
            r#"
            SELECT * FROM forecast_scenario_snapshots
            WHERE scenario_id = $1
            ORDER BY snapshot_version DESC
            "#,
        )
        .bind(scenario_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(snapshots)
    }

    async fn export_scenario(&self, id: Uuid) -> Result<serde_json::Value, AppError> {
        let (export,): (serde_json::Value,) = sqlx::query_as("SELECT export_scenario($1)")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(export)
    }
}
