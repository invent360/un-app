//! Forecast service for scenario-based financial projections
//!
//! Provides the forecast engine with:
//! - Scenario creation and management
//! - Forecast calculation algorithm
//! - Import/export with reproducibility
//! - Golden fixture validation
//!
//! # R5-12: Unified Task-Driven Forecast
//!
//! This service uses the canonical calculation engine from `uno_api::services::forecast`.
//! The engine provides:
//! - 22+ validation rules (FC-01 through FC-22)
//! - Cohort-based credit renewal tracking
//! - Peak funding requirement analysis
//! - Settlement lag modeling
//! - Revenue identity guarantee: `pool = ulo + uno + referral`
//!
//! The enterprise workflow (scenarios, approval gates, golden fixtures) wraps the
//! canonical engine for database persistence and workflow management.

use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use std::sync::Arc;
use uuid::Uuid;

// R5-12: Import canonical forecast engine
#[cfg(feature = "ssr")]
use uno_api::models::{
    ForecastConfig as CanonicalConfig,
    ForecastTask as CanonicalTask,
    ForecastRow as CanonicalRow,
    RateBasis,
};
#[cfg(feature = "ssr")]
use uno_api::services::forecast as canonical_forecast;

use crate::server::repositories::{
    DynForecastRepository, ForecastRepository, ForecastScenario, ForecastTask, ForecastResult,
    ForecastGoldenFixture, ForecastStatus, CreateScenarioInput, UpdateScenarioInput,
    AddTaskInput, UpdateTaskInput, CreateGoldenFixtureInput, ScenarioExport,
    ValidationResult, ValidationDifference,
};
use crate::types::AppError;

// ============================================
// FORECAST ENGINE
// ============================================

/// Agreement share percentages (basis points)
#[derive(Debug, Clone)]
pub struct AgreementShares {
    pub participant_bps: i32,  // 5000 = 50%
    pub referral_bps: i32,     // 1000 = 10%
    pub uno_bps: i32,          // 4000 = 40%
}

impl Default for AgreementShares {
    fn default() -> Self {
        Self {
            participant_bps: 5000,
            referral_bps: 1000,
            uno_bps: 4000,
        }
    }
}

/// Forecast engine configuration
#[derive(Debug, Clone)]
pub struct ForecastEngineConfig {
    pub agreement_shares: AgreementShares,
    pub hosting_cost_per_user_micros: i64,
    pub messaging_cost_per_message_micros: i64,
    pub default_churn_rate: f64,
}

impl Default for ForecastEngineConfig {
    fn default() -> Self {
        Self {
            agreement_shares: AgreementShares::default(),
            hosting_cost_per_user_micros: 50000,
            messaging_cost_per_message_micros: 500,
            default_churn_rate: 0.05,
        }
    }
}

/// Weekly projection result from forecast engine
#[derive(Debug, Clone)]
pub struct WeeklyProjection {
    pub week_number: i32,
    pub week_start: NaiveDate,
    pub week_end: NaiveDate,
    pub country_code: String,
    pub task_code: String,

    // Inventory
    pub opening_licenses: i32,
    pub new_claims: i32,
    pub activations: i32,
    pub churned: i32,
    pub closing_licenses: i32,
    pub active_licenses: i32,
    pub productive_licenses: i32,

    // Revenue (applying retention/churn)
    pub gross_revenue_micros: i64,
    pub participant_share_micros: i64,
    pub referral_share_micros: i64,
    pub uno_share_micros: i64,

    // Costs
    pub acquisition_cost_micros: i64,
    pub support_cost_micros: i64,
    pub hosting_cost_micros: i64,
    pub messaging_cost_micros: i64,
    pub other_cost_micros: i64,

    // Derived
    pub net_profit_micros: i64,
    pub cumulative_profit_micros: i64,
    pub break_even_reached: bool,
    pub funding_required_micros: i64,
    pub cumulative_funding_micros: i64,
}

// R5-12: Deprecated ForecastEngine removed
// All forecasts now use the canonical engine from uno_api::services::forecast::simulate()

// ============================================
// R5-12: CANONICAL ENGINE CONVERSION
// ============================================

#[cfg(feature = "ssr")]
mod canonical_conversion {
    use super::*;
    use crate::server::repositories::{ForecastScenario as DbScenario, ForecastTask as DbTask};

    /// Helper to extract a value from JSON assumptions
    fn get_json_f64(assumptions: &serde_json::Value, key: &str, default: f64) -> f64 {
        assumptions.get(key)
            .and_then(|v| v.as_f64())
            .unwrap_or(default)
    }

    fn get_json_u64(assumptions: &serde_json::Value, key: &str, default: u64) -> u64 {
        assumptions.get(key)
            .and_then(|v| v.as_u64())
            .unwrap_or(default)
    }

    /// Convert database scenario + tasks to canonical forecast config
    ///
    /// R5-12: This function maps the multi-country, multi-task database model
    /// to the canonical single-config model. The canonical engine handles
    /// cohort tracking, credit renewal, and settlement lag.
    pub fn to_canonical_config(
        scenario: &DbScenario,
        tasks: &[DbTask],
        engine_config: &ForecastEngineConfig,
    ) -> (CanonicalConfig, Vec<CanonicalTask>) {
        // Extract assumptions from scenario JSON
        let assumptions = &scenario.assumptions;

        // Build canonical config from scenario and engine defaults
        let config = CanonicalConfig {
            weeks: scenario.forecast_weeks as u32,
            capacity: get_json_u64(assumptions, "capacity", 2500) as u32,
            opening: get_json_u64(assumptions, "opening", 0) as u32,
            new_net: get_json_u64(assumptions, "new_net", 250) as u32,
            churn: get_json_f64(assumptions, "churn", 0.02),
            success: get_json_f64(assumptions, "success", 0.85),
            credit: get_json_f64(assumptions, "credit", 1.99),
            support: get_json_f64(assumptions, "support", 0.25),
            acquisition: get_json_f64(assumptions, "acquisition", 300.0),
            coordination: get_json_f64(assumptions, "coordination", 150.0),
            maintenance_acquisition: get_json_f64(assumptions, "maintenance_acquisition", 60.0),
            maintenance_coordination: get_json_f64(assumptions, "maintenance_coordination", 60.0),
            overhead: get_json_f64(assumptions, "overhead", 0.0),
            setup: get_json_f64(assumptions, "setup", 450.0),
            capital: get_json_f64(assumptions, "capital", 0.0),
            amort_weeks: get_json_u64(assumptions, "amort_weeks", 104) as u32,
            tax: get_json_f64(assumptions, "tax", 0.0),
            fee: get_json_f64(assumptions, "fee", 0.0),
            lag: get_json_u64(assumptions, "lag", 2) as u32,
            opening_cash: get_json_f64(assumptions, "opening_cash", 25000.0),
            // 50/40/10 split from engine config
            uno: (engine_config.agreement_shares.uno_bps as f64) / 10000.0,
            ulo: (engine_config.agreement_shares.participant_bps as f64) / 10000.0,
            referral: (engine_config.agreement_shares.referral_bps as f64) / 10000.0,
            up_usd: get_json_f64(assumptions, "up_usd", 1.0),
            android: get_json_f64(assumptions, "android", 1.0),
            ios: get_json_f64(assumptions, "ios", 0.0),
            windows: get_json_f64(assumptions, "windows", 0.0),
        };

        // Convert database tasks to canonical tasks
        let canonical_tasks: Vec<CanonicalTask> = tasks
            .iter()
            .map(|t| {
                // F7: Calculate task start/end weeks based on launch_date if provided
                let start_week = t.launch_date
                    .and_then(|ld| {
                        let days_from_start = (ld - scenario.start_date).num_days();
                        if days_from_start > 0 {
                            Some((days_from_start / 7 + 1) as u32)
                        } else {
                            Some(1)
                        }
                    })
                    .unwrap_or(1);

                let end_week = t.end_date
                    .and_then(|ed| {
                        let days_from_start = (ed - scenario.start_date).num_days();
                        if days_from_start > 0 {
                            Some((days_from_start / 7 + 1) as u32)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(scenario.forecast_weeks as u32);

                CanonicalTask {
                    name: t.task_name.clone(),
                    enabled: true, // All tasks in DB are enabled by definition
                    android: t.device_type.as_deref() != Some("ios") && t.device_type.as_deref() != Some("windows"),
                    ios: t.device_type.as_deref() == Some("ios") || t.device_type.as_deref() == Some("all"),
                    windows: t.device_type.as_deref() == Some("windows") || t.device_type.as_deref() == Some("all"),
                    // Convert micros to USD
                    rate: (t.daily_rate_micros as f64) / 1_000_000.0,
                    basis: RateBasis::Pool,
                    // F7 FIX: eligible is the productivity rate (eligibility factor)
                    eligible: t.productivity_rate,
                    // F7 FIX: activity should be 1.0 (not productivity_rate which would cause squaring)
                    // The productivity_rate already accounts for the effective participation
                    activity: 1.0,
                    start: start_week,
                    end: end_week,
                    // B6 FIX: cap is the DAILY POOL CAP in UP units (not device count)
                    // The canonical engine uses: pool_contribution.min(task.cap * config.up_usd)
                    // max_capacity from DB is in UP units per day; 0.0 means unlimited
                    cap: t.max_capacity.map(|c| c as f64).unwrap_or(0.0),
                    // B6 FIX: extra is per-DEVICE-DAY cost in USD (not weekly!)
                    // The canonical engine model struct says: "Extra cost per device-day in USD"
                    // Conversion: monthly micros → USD → daily
                    //   - Divide by 1_000_000 to convert micros to USD
                    //   - Divide by 30.44 (average days/month) to get daily rate
                    //   - Equivalent: monthly / 4.33 weeks / 7 days
                    // Input: monthly cost per user in micros (e.g., $2/month = 2_000_000)
                    // Output: daily cost per user in USD (e.g., $2 / 30.44 ≈ $0.066/day)
                    extra: (t.support_cost_per_user_monthly_micros as f64) / 1_000_000.0 / 30.44,
                    illustrative: false,
                }
            })
            .collect();

        (config, canonical_tasks)
    }

    /// Convert canonical forecast row to database result
    ///
    /// R5-12: Maps the 31-field canonical row to the database result format
    ///
    /// B6 DOC: Cost field mapping (semantic clarification)
    /// The database field names are historical; the actual content is:
    ///
    /// | DB Field                | Canonical Source              | Actual Meaning                |
    /// |-------------------------|-------------------------------|-------------------------------|
    /// | `acquisition_cost_micros` | row.acquisition             | Customer acquisition costs    |
    /// | `support_cost_micros`     | row.support                 | Per-user support costs        |
    /// | `hosting_cost_micros`     | row.overhead + row.coordination | Platform overhead + task coordination |
    /// | `messaging_cost_micros`   | row.fees                    | Platform/transaction fees (not messaging) |
    /// | `other_cost_micros`       | row.task_costs + row.capital_charge + row.tax | Task-specific + financial costs |
    ///
    /// Note: `messaging_cost_micros` is a legacy name; it actually stores platform fees.
    /// Future refactoring should rename to `platform_fees_micros` for clarity.
    pub fn from_canonical_row(
        scenario_id: Uuid,
        row: &CanonicalRow,
        scenario: &DbScenario,
    ) -> ForecastResult {
        let week_start = scenario.start_date + chrono::Duration::days((row.week as i64 - 1) * 7);
        let week_end = week_start + chrono::Duration::days(6);

        // Convert USD to micros (multiply by 1,000,000)
        let to_micros = |usd: f64| -> i64 { (usd * 1_000_000.0) as i64 };

        ForecastResult {
            id: Uuid::new_v4(),
            scenario_id,
            week_number: row.week as i32,
            week_start,
            week_end,
            country_code: None, // Canonical engine is portfolio-level
            task_code: None,    // Aggregated across all tasks
            opening_licenses: row.opening as i32,
            new_claims: row.additions as i32,
            activations: row.additions as i32, // Same as additions in canonical model
            churned: row.churn as i32,
            closing_licenses: row.closing as i32,
            active_licenses: row.closing as i32,
            productive_licenses: row.closing as i32, // All closing are productive
            gross_revenue_micros: to_micros(row.pool),
            // ULO = participant share (50%)
            participant_share_micros: to_micros(row.ulo),
            referral_share_micros: to_micros(row.referral),
            uno_share_micros: to_micros(row.uno),
            // F7: Preserve itemized costs where possible
            acquisition_cost_micros: to_micros(row.acquisition),
            support_cost_micros: to_micros(row.support),
            // F7: hosting includes overhead + coordination (infrastructure costs)
            hosting_cost_micros: to_micros(row.overhead + row.coordination),
            // F7: messaging field used for fees (platform transaction fees)
            messaging_cost_micros: to_micros(row.fees),
            // F7: other includes task-specific costs + financial charges
            other_cost_micros: to_micros(row.task_costs + row.capital_charge + row.tax),
            total_cost_micros: to_micros(row.operating_costs),
            net_profit_micros: to_micros(row.profit),
            cumulative_profit_micros: to_micros(row.cumulative_profit),
            break_even_reached: row.cumulative_profit >= 0.0,
            funding_required_micros: to_micros(row.required_funding),
            // F7: Use cumulative cash metric for cumulative funding
            cumulative_funding_micros: to_micros(row.required_funding.max(-row.cumulative_cash)),
            is_actual: false,
            created_at: Utc::now(),
        }
    }
}

// ============================================
// SERVICE TRAIT
// ============================================

/// Dynamic type alias for ForecastService trait object
pub type DynForecastService = Arc<dyn ForecastService + Send + Sync>;

/// Forecast service trait
#[async_trait]
pub trait ForecastService: Send + Sync {
    // --- Scenario Management ---
    async fn create_scenario(&self, input: CreateScenarioInput) -> Result<ForecastScenario, AppError>;
    async fn get_scenario(&self, id: Uuid) -> Result<Option<ForecastScenario>, AppError>;
    async fn list_scenarios(&self, status: Option<ForecastStatus>) -> Result<Vec<ForecastScenario>, AppError>;
    async fn update_scenario(&self, id: Uuid, input: UpdateScenarioInput) -> Result<ForecastScenario, AppError>;
    async fn approve_scenario(&self, id: Uuid, approved_by: &str) -> Result<ForecastScenario, AppError>;
    async fn archive_scenario(&self, id: Uuid) -> Result<ForecastScenario, AppError>;
    async fn delete_scenario(&self, id: Uuid) -> Result<(), AppError>;

    // --- Task Configuration ---
    async fn add_task(&self, input: AddTaskInput) -> Result<ForecastTask, AppError>;
    async fn get_scenario_tasks(&self, scenario_id: Uuid) -> Result<Vec<ForecastTask>, AppError>;
    async fn update_task(&self, id: Uuid, input: UpdateTaskInput) -> Result<ForecastTask, AppError>;
    async fn delete_task(&self, id: Uuid) -> Result<(), AppError>;

    // --- Forecast Execution ---
    async fn run_forecast(&self, scenario_id: Uuid) -> Result<Vec<ForecastResult>, AppError>;
    async fn get_results(&self, scenario_id: Uuid) -> Result<Vec<ForecastResult>, AppError>;

    // --- Import/Export ---
    async fn export_scenario(&self, id: Uuid) -> Result<ScenarioExport, AppError>;
    async fn import_scenario(&self, export: ScenarioExport, created_by: &str) -> Result<ForecastScenario, AppError>;

    // --- Validation ---
    async fn validate_against_golden(&self, fixture_name: &str, scenario_id: Uuid) -> Result<ValidationResult, AppError>;
    async fn create_golden_fixture(&self, input: CreateGoldenFixtureInput) -> Result<ForecastGoldenFixture, AppError>;
    async fn list_golden_fixtures(&self) -> Result<Vec<ForecastGoldenFixture>, AppError>;
    async fn approve_golden_fixture(&self, id: Uuid, approved_by: &str, finance_approved: bool) -> Result<ForecastGoldenFixture, AppError>;

    // --- Snapshots ---
    async fn create_snapshot(&self, scenario_id: Uuid, reason: &str, created_by: &str) -> Result<(), AppError>;
}

// ============================================
// SERVICE IMPLEMENTATION
// ============================================

/// R5-12: Forecast service using canonical engine only
pub struct ForecastServiceImpl {
    repo: DynForecastRepository,
    /// Configuration for revenue shares and cost defaults
    config: ForecastEngineConfig,
}

impl ForecastServiceImpl {
    pub fn new(repo: DynForecastRepository) -> Self {
        Self {
            repo,
            config: ForecastEngineConfig::default(),
        }
    }

    pub fn with_config(repo: DynForecastRepository, config: ForecastEngineConfig) -> Self {
        Self {
            repo,
            config,
        }
    }
}

#[async_trait]
impl ForecastService for ForecastServiceImpl {
    async fn create_scenario(&self, input: CreateScenarioInput) -> Result<ForecastScenario, AppError> {
        self.repo.create_scenario(input).await
    }

    async fn get_scenario(&self, id: Uuid) -> Result<Option<ForecastScenario>, AppError> {
        self.repo.get_scenario(id).await
    }

    async fn list_scenarios(&self, status: Option<ForecastStatus>) -> Result<Vec<ForecastScenario>, AppError> {
        self.repo.list_scenarios(status).await
    }

    async fn update_scenario(&self, id: Uuid, input: UpdateScenarioInput) -> Result<ForecastScenario, AppError> {
        // Ensure scenario exists and is in draft status
        let scenario = self.repo.get_scenario(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Scenario {} not found", id)))?;

        if scenario.status != ForecastStatus::Draft {
            return Err(AppError::ValidationError("Can only update draft scenarios".to_string()));
        }

        self.repo.update_scenario(id, input).await
    }

    async fn approve_scenario(&self, id: Uuid, approved_by: &str) -> Result<ForecastScenario, AppError> {
        // Ensure scenario has been run
        let results = self.repo.get_results(id).await?;
        if results.is_empty() {
            return Err(AppError::ValidationError("Cannot approve scenario without results. Run forecast first.".to_string()));
        }

        // Create snapshot before approval
        self.repo.create_snapshot(id, "approval", approved_by).await?;

        self.repo.approve_scenario(id, approved_by).await
    }

    async fn archive_scenario(&self, id: Uuid) -> Result<ForecastScenario, AppError> {
        self.repo.archive_scenario(id).await
    }

    async fn delete_scenario(&self, id: Uuid) -> Result<(), AppError> {
        self.repo.delete_scenario(id).await
    }

    async fn add_task(&self, input: AddTaskInput) -> Result<ForecastTask, AppError> {
        // Ensure scenario is in draft status
        let scenario = self.repo.get_scenario(input.scenario_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Scenario {} not found", input.scenario_id)))?;

        if scenario.status != ForecastStatus::Draft {
            return Err(AppError::ValidationError("Can only add tasks to draft scenarios".to_string()));
        }

        self.repo.add_task(input).await
    }

    async fn get_scenario_tasks(&self, scenario_id: Uuid) -> Result<Vec<ForecastTask>, AppError> {
        self.repo.get_scenario_tasks(scenario_id).await
    }

    async fn update_task(&self, id: Uuid, input: UpdateTaskInput) -> Result<ForecastTask, AppError> {
        // Get task to check scenario status
        let task = self.repo.get_task(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Task {} not found", id)))?;

        let scenario = self.repo.get_scenario(task.scenario_id).await?
            .ok_or_else(|| AppError::InternalServerError("Task's scenario not found".to_string()))?;

        if scenario.status != ForecastStatus::Draft {
            return Err(AppError::ValidationError("Can only update tasks in draft scenarios".to_string()));
        }

        self.repo.update_task(id, input).await
    }

    async fn delete_task(&self, id: Uuid) -> Result<(), AppError> {
        self.repo.delete_task(id).await
    }

    async fn run_forecast(&self, scenario_id: Uuid) -> Result<Vec<ForecastResult>, AppError> {
        // Get scenario and tasks
        let scenario = self.repo.get_scenario(scenario_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Scenario {} not found", scenario_id)))?;

        let tasks = self.repo.get_scenario_tasks(scenario_id).await?;

        if tasks.is_empty() {
            return Err(AppError::ValidationError("Scenario has no tasks configured".to_string()));
        }

        // R5-12: Use canonical forecast engine from uno-api (only path)
        #[cfg(feature = "ssr")]
        let results = {
            use canonical_conversion::{to_canonical_config, from_canonical_row};

            // Convert to canonical format
            let (config, canonical_tasks) = to_canonical_config(&scenario, &tasks, &self.config);

            // Run canonical simulation with full validation
            let forecast_result = canonical_forecast::simulate(&config, &canonical_tasks)
                .map_err(|errors| {
                    let error_msgs: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
                    AppError::ValidationError(format!(
                        "Forecast validation failed: {}",
                        error_msgs.join("; ")
                    ))
                })?;

            // Verify revenue identity holds (FC-07)
            if !forecast_result.all_revenue_identities_hold() {
                return Err(AppError::InternalServerError(
                    "Forecast calculation violated revenue identity (pool != ulo + uno + referral)".to_string()
                ));
            }

            // Convert canonical rows to database results
            let results: Vec<ForecastResult> = forecast_result
                .rows
                .iter()
                .map(|row| from_canonical_row(scenario_id, row, &scenario))
                .collect();

            tracing::info!(
                scenario_id = %scenario_id,
                weeks = forecast_result.rows.len(),
                required_funding = forecast_result.required_funding,
                first_positive_week = ?forecast_result.first_positive_week,
                profit_payback = ?forecast_result.profit_payback,
                cash_payback = ?forecast_result.cash_payback,
                "R5-12: Canonical forecast completed"
            );

            results
        };

        // R5-12: Non-SSR builds should not call forecasts
        #[cfg(not(feature = "ssr"))]
        let results: Vec<ForecastResult> = {
            return Err(AppError::InternalServerError(
                "Forecast engine only available in SSR builds".to_string()
            ));
        };

        // Clear existing results and save new ones
        self.repo.clear_results(scenario_id).await?;
        self.repo.save_results(scenario_id, results).await?;

        // Activate scenario if draft
        if scenario.status == ForecastStatus::Draft {
            self.repo.update_scenario_status(scenario_id, ForecastStatus::Active).await?;
        }

        self.repo.get_results(scenario_id).await
    }

    async fn get_results(&self, scenario_id: Uuid) -> Result<Vec<ForecastResult>, AppError> {
        self.repo.get_results(scenario_id).await
    }

    async fn export_scenario(&self, id: Uuid) -> Result<ScenarioExport, AppError> {
        let scenario = self.repo.get_scenario(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Scenario {} not found", id)))?;

        let tasks = self.repo.get_scenario_tasks(id).await?;
        let results = self.repo.get_results(id).await?;

        Ok(ScenarioExport {
            scenario,
            tasks,
            results,
            exported_at: Utc::now(),
        })
    }

    async fn import_scenario(&self, export: ScenarioExport, created_by: &str) -> Result<ForecastScenario, AppError> {
        // Create new scenario based on export
        let input = CreateScenarioInput {
            name: format!("{} (imported)", export.scenario.name),
            description: export.scenario.description,
            created_by: created_by.to_string(),
            start_date: export.scenario.start_date,
            end_date: export.scenario.end_date,
            snapshot_date: export.scenario.snapshot_date,
            forecast_weeks: export.scenario.forecast_weeks,
            algorithm_version: export.scenario.algorithm_version,
            assumptions: export.scenario.assumptions,
            tags: export.scenario.tags,
        };

        let new_scenario = self.repo.create_scenario(input).await?;

        // Import tasks
        for task in export.tasks {
            let task_input = AddTaskInput {
                scenario_id: new_scenario.id,
                task_code: task.task_code,
                task_name: task.task_name,
                country_code: task.country_code,
                device_type: task.device_type,
                daily_rate_micros: task.daily_rate_micros,
                max_capacity: task.max_capacity,
                launch_date: task.launch_date,
                end_date: task.end_date,
                registration_to_claim_rate: task.registration_to_claim_rate,
                claim_to_activation_rate: task.claim_to_activation_rate,
                d7_retention_rate: task.d7_retention_rate,
                d30_retention_rate: task.d30_retention_rate,
                monthly_churn_rate: task.monthly_churn_rate,
                avg_daily_earnings_micros: task.avg_daily_earnings_micros,
                productivity_rate: task.productivity_rate,
                acquisition_cost_per_user_micros: task.acquisition_cost_per_user_micros,
                support_cost_per_user_monthly_micros: task.support_cost_per_user_monthly_micros,
                weekly_claim_target: task.weekly_claim_target,
                total_target: task.total_target,
            };
            self.repo.add_task(task_input).await?;
        }

        Ok(new_scenario)
    }

    async fn validate_against_golden(&self, fixture_name: &str, scenario_id: Uuid) -> Result<ValidationResult, AppError> {
        let fixture = self.repo.get_golden_fixture(fixture_name).await?
            .ok_or_else(|| AppError::NotFound(format!("Golden fixture '{}' not found", fixture_name)))?;

        let results = self.repo.get_results(scenario_id).await?;

        if results.is_empty() {
            return Err(AppError::ValidationError("No results to validate. Run forecast first.".to_string()));
        }

        // Compare results against expected
        let expected_results: Vec<serde_json::Value> = serde_json::from_value(fixture.expected_results.clone())
            .map_err(|e| AppError::InternalServerError(format!("Failed to parse expected results: {}", e)))?;

        let mut differences = Vec::new();
        let tolerance = fixture.tolerance_percentage;

        for (i, result) in results.iter().enumerate() {
            if i >= expected_results.len() {
                break;
            }

            let expected = &expected_results[i];

            // Compare key metrics
            let fields_to_check = [
                ("active_licenses", result.active_licenses as f64, expected["active_licenses"].as_f64().unwrap_or(0.0)),
                ("gross_revenue_micros", result.gross_revenue_micros as f64, expected["gross_revenue_micros"].as_f64().unwrap_or(0.0)),
                ("net_profit_micros", result.net_profit_micros as f64, expected["net_profit_micros"].as_f64().unwrap_or(0.0)),
            ];

            for (field, actual, expected_val) in fields_to_check {
                if expected_val != 0.0 {
                    let diff_pct = ((actual - expected_val) / expected_val).abs() * 100.0;
                    if diff_pct > tolerance {
                        differences.push(ValidationDifference {
                            field: format!("week_{}.{}", result.week_number, field),
                            expected: format!("{:.2}", expected_val),
                            actual: format!("{:.2}", actual),
                            difference_pct: diff_pct,
                        });
                    }
                }
            }
        }

        Ok(ValidationResult {
            valid: differences.is_empty(),
            fixture_name: fixture_name.to_string(),
            algorithm_version: fixture.algorithm_version,
            tolerance,
            differences,
        })
    }

    async fn create_golden_fixture(&self, input: CreateGoldenFixtureInput) -> Result<ForecastGoldenFixture, AppError> {
        self.repo.create_golden_fixture(input).await
    }

    async fn list_golden_fixtures(&self) -> Result<Vec<ForecastGoldenFixture>, AppError> {
        self.repo.list_golden_fixtures().await
    }

    async fn approve_golden_fixture(&self, id: Uuid, approved_by: &str, finance_approved: bool) -> Result<ForecastGoldenFixture, AppError> {
        self.repo.approve_golden_fixture(id, approved_by, finance_approved).await
    }

    async fn create_snapshot(&self, scenario_id: Uuid, reason: &str, created_by: &str) -> Result<(), AppError> {
        self.repo.create_snapshot(scenario_id, reason, created_by).await?;
        Ok(())
    }
}
