//! Forecast service for scenario-based financial projections
//!
//! Provides the forecast engine with:
//! - Scenario creation and management
//! - Forecast calculation algorithm
//! - Import/export with reproducibility
//! - Golden fixture validation
//!
//! # R4-07: Engine Unification
//!
//! This service implements scenario management and database persistence. The calculation
//! engine here is a simplified version suitable for multi-country, multi-task scenarios.
//!
//! For detailed portfolio modeling with cohort tracking, credit renewal, and settlement
//! lag, see `uno_api::services::forecast` which provides:
//! - 22+ validation rules (FC-01 through FC-22)
//! - Cohort-based credit renewal tracking
//! - Peak funding requirement analysis
//! - Settlement lag modeling
//! - Revenue identity guarantee: `pool = ulo + uno + referral`
//!
//! The canonical calculation engine is in uno-api; this service provides the
//! enterprise workflow (scenarios, approval gates, golden fixtures) on top of it.

use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use std::sync::Arc;
use uuid::Uuid;

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

/// Forecast engine for calculating weekly projections
///
/// This is a simplified calculation engine for multi-country, multi-task scenarios.
/// For sophisticated portfolio modeling (cohort tracking, credit renewal, settlement lag),
/// see `uno_api::services::forecast::simulate()` which implements the full algorithm
/// ported from the JavaScript HTML calculator.
///
/// When higher fidelity is needed, convert scenarios to `uno_api::models::ForecastConfig`
/// and use `uno_api::services::simulate()` for the calculation.
pub struct ForecastEngine {
    config: ForecastEngineConfig,
}

impl ForecastEngine {
    pub fn new(config: ForecastEngineConfig) -> Self {
        Self { config }
    }

    /// Calculate weekly projections for a scenario
    pub fn calculate_projections(
        &self,
        scenario: &ForecastScenario,
        tasks: &[ForecastTask],
    ) -> Vec<WeeklyProjection> {
        let mut projections = Vec::new();
        let mut cumulative_profit: i64 = 0;
        let mut cumulative_funding: i64 = 0;
        let mut break_even_reached = false;

        // Track previous week's closing licenses by task
        let mut prev_closing: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

        for week_num in 1..=scenario.forecast_weeks {
            let week_start = scenario.start_date + chrono::Duration::days((week_num - 1) as i64 * 7);
            let week_end = week_start + chrono::Duration::days(6);

            for task in tasks {
                let task_key = format!("{}:{}", task.country_code, task.task_code);

                // Get previous closing (or 0 for first week)
                let opening = *prev_closing.get(&task_key).unwrap_or(&0);

                // Calculate new claims for this week
                let new_claims = task.weekly_claim_target.unwrap_or(10);
                let activations = ((new_claims as f64) * task.claim_to_activation_rate) as i32;

                // Calculate churn (weekly churn = monthly / 4)
                let weekly_churn_rate = task.monthly_churn_rate / 4.0;
                let churned = ((opening as f64) * weekly_churn_rate) as i32;

                // Calculate closing licenses
                let closing = opening + activations - churned;
                let active = closing;
                let productive = ((closing as f64) * task.productivity_rate) as i32;

                // Store for next week
                prev_closing.insert(task_key.clone(), closing);

                // Calculate revenue
                let gross_revenue = (productive as i64) * task.daily_rate_micros * 7;
                let participant_share = (gross_revenue * self.config.agreement_shares.participant_bps as i64) / 10000;
                let referral_share = (gross_revenue * self.config.agreement_shares.referral_bps as i64) / 10000;
                let uno_share = (gross_revenue * self.config.agreement_shares.uno_bps as i64) / 10000;

                // Calculate costs
                let acquisition_cost = (new_claims as i64) * task.acquisition_cost_per_user_micros;
                let support_cost = (active as i64) * task.support_cost_per_user_monthly_micros / 4;
                let hosting_cost = (active as i64) * self.config.hosting_cost_per_user_micros;
                let messaging_cost = (active as i64) * self.config.messaging_cost_per_message_micros * 50; // ~50 msgs/user/week
                let other_cost: i64 = 0;

                let total_cost = acquisition_cost + support_cost + hosting_cost + messaging_cost + other_cost;

                // Calculate profit
                let net_profit = uno_share - total_cost;
                cumulative_profit += net_profit;

                // Track funding required (negative profit)
                let funding_required = if net_profit < 0 { -net_profit } else { 0 };
                cumulative_funding += funding_required;

                // Check break-even
                if cumulative_profit >= 0 && !break_even_reached {
                    break_even_reached = true;
                }

                projections.push(WeeklyProjection {
                    week_number: week_num,
                    week_start,
                    week_end,
                    country_code: task.country_code.clone(),
                    task_code: task.task_code.clone(),
                    opening_licenses: opening,
                    new_claims,
                    activations,
                    churned,
                    closing_licenses: closing,
                    active_licenses: active,
                    productive_licenses: productive,
                    gross_revenue_micros: gross_revenue,
                    participant_share_micros: participant_share,
                    referral_share_micros: referral_share,
                    uno_share_micros: uno_share,
                    acquisition_cost_micros: acquisition_cost,
                    support_cost_micros: support_cost,
                    hosting_cost_micros: hosting_cost,
                    messaging_cost_micros: messaging_cost,
                    other_cost_micros: other_cost,
                    net_profit_micros: net_profit,
                    cumulative_profit_micros: cumulative_profit,
                    break_even_reached,
                    funding_required_micros: funding_required,
                    cumulative_funding_micros: cumulative_funding,
                });
            }
        }

        projections
    }

    /// Convert projection to ForecastResult
    fn projection_to_result(scenario_id: Uuid, proj: &WeeklyProjection) -> ForecastResult {
        ForecastResult {
            id: Uuid::new_v4(),
            scenario_id,
            week_number: proj.week_number,
            week_start: proj.week_start,
            week_end: proj.week_end,
            country_code: Some(proj.country_code.clone()),
            task_code: Some(proj.task_code.clone()),
            opening_licenses: proj.opening_licenses,
            new_claims: proj.new_claims,
            activations: proj.activations,
            churned: proj.churned,
            closing_licenses: proj.closing_licenses,
            active_licenses: proj.active_licenses,
            productive_licenses: proj.productive_licenses,
            gross_revenue_micros: proj.gross_revenue_micros,
            participant_share_micros: proj.participant_share_micros,
            referral_share_micros: proj.referral_share_micros,
            uno_share_micros: proj.uno_share_micros,
            acquisition_cost_micros: proj.acquisition_cost_micros,
            support_cost_micros: proj.support_cost_micros,
            hosting_cost_micros: proj.hosting_cost_micros,
            messaging_cost_micros: proj.messaging_cost_micros,
            other_cost_micros: proj.other_cost_micros,
            total_cost_micros: proj.acquisition_cost_micros + proj.support_cost_micros +
                               proj.hosting_cost_micros + proj.messaging_cost_micros + proj.other_cost_micros,
            net_profit_micros: proj.net_profit_micros,
            cumulative_profit_micros: proj.cumulative_profit_micros,
            break_even_reached: proj.break_even_reached,
            funding_required_micros: proj.funding_required_micros,
            cumulative_funding_micros: proj.cumulative_funding_micros,
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

pub struct ForecastServiceImpl {
    repo: DynForecastRepository,
    engine: ForecastEngine,
}

impl ForecastServiceImpl {
    pub fn new(repo: DynForecastRepository) -> Self {
        Self {
            repo,
            engine: ForecastEngine::new(ForecastEngineConfig::default()),
        }
    }

    pub fn with_config(repo: DynForecastRepository, config: ForecastEngineConfig) -> Self {
        Self {
            repo,
            engine: ForecastEngine::new(config),
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

        // Run the forecast engine
        let projections = self.engine.calculate_projections(&scenario, &tasks);

        // Convert projections to results
        let results: Vec<ForecastResult> = projections
            .iter()
            .map(|p| ForecastEngine::projection_to_result(scenario_id, p))
            .collect();

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
