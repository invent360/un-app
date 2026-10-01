//! Forecast scenario handlers (Phase 8)
//!
//! Provides scenario-based forecast engine for financial planning,
//! import/export, and golden fixture validation.

use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::repositories::{
    DynForecastRepository, CreateScenarioInput, UpdateScenarioInput, AddTaskInput,
    UpdateTaskInput, CreateGoldenFixtureInput, ForecastStatus,
};
use crate::server::services::{
    DynForecastService, AgreementShares, WeeklyProjection,
};
use crate::types::AppError;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct ListScenariosQuery {
    pub status: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateScenarioRequest {
    pub name: String,
    pub description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub snapshot_date: NaiveDate,
    pub created_by: String,
    pub forecast_weeks: Option<i32>,
    pub algorithm_version: Option<String>,
    pub assumptions: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateScenarioRequest {
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

#[derive(Debug, Deserialize)]
pub struct ApproveScenarioRequest {
    pub approved_by: String,
}

#[derive(Debug, Deserialize)]
pub struct AddTaskRequest {
    pub task_code: String,
    pub task_name: Option<String>,
    pub country_code: String,
    pub device_type: Option<String>,
    pub daily_rate_micros: i64,
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

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
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

#[derive(Debug, Deserialize)]
pub struct ImportScenarioRequest {
    pub scenario: serde_json::Value,
    pub import_as_draft: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    pub fixture_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGoldenFixtureRequest {
    pub fixture_name: String,
    pub description: Option<String>,
    pub approved_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ScenarioResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub version: i32,
    pub status: String,
    pub created_by: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub snapshot_date: NaiveDate,
    pub algorithm_version: String,
    pub assumptions: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
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
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ResultResponse {
    pub id: Uuid,
    pub scenario_id: Uuid,
    pub week_number: i32,
    pub week_start: NaiveDate,
    pub week_end: NaiveDate,
    pub country_code: Option<String>,
    pub task_code: Option<String>,
    pub opening_licenses: i32,
    pub new_claims: i32,
    pub activations: i32,
    pub churned: i32,
    pub closing_licenses: i32,
    pub active_licenses: i32,
    pub productive_licenses: i32,
    pub gross_revenue_micros: i64,
    pub participant_share_micros: i64,
    pub referral_share_micros: i64,
    pub uno_share_micros: i64,
    pub acquisition_cost_micros: i64,
    pub support_cost_micros: i64,
    pub hosting_cost_micros: i64,
    pub messaging_cost_micros: i64,
    pub other_cost_micros: i64,
    pub total_cost_micros: i64,
    pub net_profit_micros: i64,
    pub cumulative_profit_micros: i64,
    pub break_even_reached: bool,
    pub funding_required_micros: i64,
    pub cumulative_funding_micros: i64,
    pub is_actual: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ValidationResultResponse {
    pub valid: bool,
    pub fixture_name: String,
    pub differences: Vec<ValidationDifferenceResponse>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationDifferenceResponse {
    pub field: String,
    pub expected: String,
    pub actual: String,
    pub difference_pct: f64,
}

#[derive(Debug, Serialize)]
pub struct GoldenFixtureResponse {
    pub id: Uuid,
    pub fixture_name: String,
    pub description: Option<String>,
    pub algorithm_version: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ScenarioWithTasksResponse {
    pub scenario: ScenarioResponse,
    pub tasks: Vec<TaskResponse>,
}

// ============================================================================
// Handlers
// ============================================================================

/// GET /api/v1/admin/forecasts
/// List all forecast scenarios
pub async fn list_scenarios(
    service: web::Data<DynForecastService>,
    query: web::Query<ListScenariosQuery>,
) -> Result<impl Responder, AppError> {
    let status = query.status.as_ref().and_then(|s| match s.to_lowercase().as_str() {
        "draft" => Some(ForecastStatus::Draft),
        "active" => Some(ForecastStatus::Active),
        "approved" => Some(ForecastStatus::Approved),
        "archived" => Some(ForecastStatus::Archived),
        _ => None,
    });
    let scenarios = service.list_scenarios(status).await?;

    let response: Vec<ScenarioResponse> = scenarios
        .into_iter()
        .map(|s| ScenarioResponse {
            id: s.id,
            name: s.name,
            description: s.description,
            version: s.version,
            status: format!("{:?}", s.status).to_lowercase(),
            created_by: s.created_by,
            approved_by: s.approved_by,
            approved_at: s.approved_at,
            start_date: s.start_date,
            end_date: s.end_date,
            snapshot_date: s.snapshot_date,
            algorithm_version: s.algorithm_version,
            assumptions: s.assumptions,
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/admin/forecasts
/// Create a new forecast scenario
pub async fn create_scenario(
    service: web::Data<DynForecastService>,
    body: web::Json<CreateScenarioRequest>,
) -> Result<impl Responder, AppError> {
    let scenario = service
        .create_scenario(CreateScenarioInput {
            name: body.name.clone(),
            description: body.description.clone(),
            start_date: body.start_date,
            end_date: body.end_date,
            snapshot_date: body.snapshot_date,
            created_by: body.created_by.clone(),
            forecast_weeks: body.forecast_weeks.unwrap_or(12),
            algorithm_version: body.algorithm_version.clone().unwrap_or_else(|| "1.0".to_string()),
            assumptions: body.assumptions.clone().unwrap_or_else(|| serde_json::json!({})),
            tags: body.tags.clone().unwrap_or_default(),
        })
        .await?;

    Ok(HttpResponse::Created().json(ScenarioResponse {
        id: scenario.id,
        name: scenario.name,
        description: scenario.description,
        version: scenario.version,
        status: format!("{:?}", scenario.status).to_lowercase(),
        created_by: scenario.created_by,
        approved_by: scenario.approved_by,
        approved_at: scenario.approved_at,
        start_date: scenario.start_date,
        end_date: scenario.end_date,
        snapshot_date: scenario.snapshot_date,
        algorithm_version: scenario.algorithm_version,
        assumptions: scenario.assumptions,
        created_at: scenario.created_at,
        updated_at: scenario.updated_at,
    }))
}

/// GET /api/v1/admin/forecasts/{id}
/// Get a specific forecast scenario with its tasks
pub async fn get_scenario(
    service: web::Data<DynForecastService>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let scenario = service.get_scenario(id).await?;

    match scenario {
        Some(s) => {
            let tasks = service.get_scenario_tasks(id).await?;
            let response = ScenarioWithTasksResponse {
                scenario: ScenarioResponse {
                    id: s.id,
                    name: s.name,
                    description: s.description,
                    version: s.version,
                    status: format!("{:?}", s.status).to_lowercase(),
                    created_by: s.created_by,
                    approved_by: s.approved_by,
                    approved_at: s.approved_at,
                    start_date: s.start_date,
                    end_date: s.end_date,
                    snapshot_date: s.snapshot_date,
                    algorithm_version: s.algorithm_version,
                    assumptions: s.assumptions,
                    created_at: s.created_at,
                    updated_at: s.updated_at,
                },
                tasks: tasks
                    .into_iter()
                    .map(|t| TaskResponse {
                        id: t.id,
                        scenario_id: t.scenario_id,
                        task_code: t.task_code,
                        task_name: t.task_name,
                        country_code: t.country_code,
                        device_type: t.device_type,
                        daily_rate_micros: t.daily_rate_micros,
                        max_capacity: t.max_capacity,
                        launch_date: t.launch_date,
                        end_date: t.end_date,
                        registration_to_claim_rate: t.registration_to_claim_rate,
                        claim_to_activation_rate: t.claim_to_activation_rate,
                        d7_retention_rate: t.d7_retention_rate,
                        d30_retention_rate: t.d30_retention_rate,
                        monthly_churn_rate: t.monthly_churn_rate,
                        avg_daily_earnings_micros: t.avg_daily_earnings_micros,
                        productivity_rate: t.productivity_rate,
                        acquisition_cost_per_user_micros: t.acquisition_cost_per_user_micros,
                        support_cost_per_user_monthly_micros: t.support_cost_per_user_monthly_micros,
                        weekly_claim_target: t.weekly_claim_target,
                        total_target: t.total_target,
                        created_at: t.created_at,
                    })
                    .collect(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

/// PUT /api/v1/admin/forecasts/{id}
/// Update a forecast scenario
pub async fn update_scenario(
    service: web::Data<DynForecastService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateScenarioRequest>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let scenario = service
        .update_scenario(
            id,
            UpdateScenarioInput {
                name: body.name.clone(),
                description: body.description.clone(),
                start_date: body.start_date,
                end_date: body.end_date,
                snapshot_date: body.snapshot_date,
                forecast_weeks: body.forecast_weeks,
                assumptions: body.assumptions.clone(),
                tags: body.tags.clone(),
                notes: body.notes.clone(),
            },
        )
        .await?;

    Ok(HttpResponse::Ok().json(ScenarioResponse {
        id: scenario.id,
        name: scenario.name,
        description: scenario.description,
        version: scenario.version,
        status: format!("{:?}", scenario.status).to_lowercase(),
        created_by: scenario.created_by,
        approved_by: scenario.approved_by,
        approved_at: scenario.approved_at,
        start_date: scenario.start_date,
        end_date: scenario.end_date,
        snapshot_date: scenario.snapshot_date,
        algorithm_version: scenario.algorithm_version,
        assumptions: scenario.assumptions,
        created_at: scenario.created_at,
        updated_at: scenario.updated_at,
    }))
}

/// POST /api/v1/admin/forecasts/{id}/approve
/// Approve a forecast scenario
pub async fn approve_scenario(
    service: web::Data<DynForecastService>,
    path: web::Path<Uuid>,
    body: web::Json<ApproveScenarioRequest>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let scenario = service.approve_scenario(id, &body.approved_by).await?;

    Ok(HttpResponse::Ok().json(ScenarioResponse {
        id: scenario.id,
        name: scenario.name,
        description: scenario.description,
        version: scenario.version,
        status: format!("{:?}", scenario.status).to_lowercase(),
        created_by: scenario.created_by,
        approved_by: scenario.approved_by,
        approved_at: scenario.approved_at,
        start_date: scenario.start_date,
        end_date: scenario.end_date,
        snapshot_date: scenario.snapshot_date,
        algorithm_version: scenario.algorithm_version,
        assumptions: scenario.assumptions,
        created_at: scenario.created_at,
        updated_at: scenario.updated_at,
    }))
}

/// DELETE /api/v1/admin/forecasts/{id}
/// Archive a forecast scenario
pub async fn archive_scenario(
    service: web::Data<DynForecastService>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    service.archive_scenario(id).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// POST /api/v1/admin/forecasts/{id}/tasks
/// Add a task to a scenario
pub async fn add_task(
    service: web::Data<DynForecastService>,
    path: web::Path<Uuid>,
    body: web::Json<AddTaskRequest>,
) -> Result<impl Responder, AppError> {
    let scenario_id = path.into_inner();
    let task = service
        .add_task(AddTaskInput {
            scenario_id,
            task_code: body.task_code.clone(),
            task_name: body.task_name.clone().unwrap_or_else(|| body.task_code.clone()),
            country_code: body.country_code.clone(),
            device_type: body.device_type.clone(),
            daily_rate_micros: body.daily_rate_micros,
            max_capacity: body.max_capacity,
            launch_date: body.launch_date,
            end_date: body.end_date,
            registration_to_claim_rate: body.registration_to_claim_rate.unwrap_or(1.0),
            claim_to_activation_rate: body.claim_to_activation_rate.unwrap_or(0.10),
            d7_retention_rate: body.d7_retention_rate.unwrap_or(0.70),
            d30_retention_rate: body.d30_retention_rate.unwrap_or(0.50),
            monthly_churn_rate: body.monthly_churn_rate.unwrap_or(0.05),
            avg_daily_earnings_micros: body.avg_daily_earnings_micros.unwrap_or(body.daily_rate_micros),
            productivity_rate: body.productivity_rate.unwrap_or(1.0),
            acquisition_cost_per_user_micros: body.acquisition_cost_per_user_micros.unwrap_or(0),
            support_cost_per_user_monthly_micros: body.support_cost_per_user_monthly_micros.unwrap_or(0),
            weekly_claim_target: body.weekly_claim_target,
            total_target: body.total_target,
        })
        .await?;

    Ok(HttpResponse::Created().json(TaskResponse {
        id: task.id,
        scenario_id: task.scenario_id,
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
        created_at: task.created_at,
    }))
}

/// PUT /api/v1/admin/forecasts/{scenario_id}/tasks/{task_id}
/// Update a task
pub async fn update_task(
    service: web::Data<DynForecastService>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateTaskRequest>,
) -> Result<impl Responder, AppError> {
    let (_scenario_id, task_id) = path.into_inner();
    let task = service
        .update_task(
            task_id,
            UpdateTaskInput {
                task_name: body.task_name.clone(),
                device_type: body.device_type.clone(),
                daily_rate_micros: body.daily_rate_micros,
                max_capacity: body.max_capacity,
                launch_date: body.launch_date,
                end_date: body.end_date,
                registration_to_claim_rate: body.registration_to_claim_rate,
                claim_to_activation_rate: body.claim_to_activation_rate,
                d7_retention_rate: body.d7_retention_rate,
                d30_retention_rate: body.d30_retention_rate,
                monthly_churn_rate: body.monthly_churn_rate,
                avg_daily_earnings_micros: body.avg_daily_earnings_micros,
                productivity_rate: body.productivity_rate,
                acquisition_cost_per_user_micros: body.acquisition_cost_per_user_micros,
                support_cost_per_user_monthly_micros: body.support_cost_per_user_monthly_micros,
                weekly_claim_target: body.weekly_claim_target,
                total_target: body.total_target,
            },
        )
        .await?;

    Ok(HttpResponse::Ok().json(TaskResponse {
        id: task.id,
        scenario_id: task.scenario_id,
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
        created_at: task.created_at,
    }))
}

/// DELETE /api/v1/admin/forecasts/{scenario_id}/tasks/{task_id}
/// Remove a task
pub async fn remove_task(
    service: web::Data<DynForecastService>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<impl Responder, AppError> {
    let (_scenario_id, task_id) = path.into_inner();
    service.delete_task(task_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// POST /api/v1/admin/forecasts/{id}/run
/// Run the forecast engine
pub async fn run_forecast(
    service: web::Data<DynForecastService>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let scenario_id = path.into_inner();
    let results = service.run_forecast(scenario_id).await?;

    let response: Vec<ResultResponse> = results
        .into_iter()
        .map(|r| ResultResponse {
            id: r.id,
            scenario_id: r.scenario_id,
            week_number: r.week_number,
            week_start: r.week_start,
            week_end: r.week_end,
            country_code: r.country_code,
            task_code: r.task_code,
            opening_licenses: r.opening_licenses,
            new_claims: r.new_claims,
            activations: r.activations,
            churned: r.churned,
            closing_licenses: r.closing_licenses,
            active_licenses: r.active_licenses,
            productive_licenses: r.productive_licenses,
            gross_revenue_micros: r.gross_revenue_micros,
            participant_share_micros: r.participant_share_micros,
            referral_share_micros: r.referral_share_micros,
            uno_share_micros: r.uno_share_micros,
            acquisition_cost_micros: r.acquisition_cost_micros,
            support_cost_micros: r.support_cost_micros,
            hosting_cost_micros: r.hosting_cost_micros,
            messaging_cost_micros: r.messaging_cost_micros,
            other_cost_micros: r.other_cost_micros,
            total_cost_micros: r.total_cost_micros,
            net_profit_micros: r.net_profit_micros,
            cumulative_profit_micros: r.cumulative_profit_micros,
            break_even_reached: r.break_even_reached,
            funding_required_micros: r.funding_required_micros,
            cumulative_funding_micros: r.cumulative_funding_micros,
            is_actual: r.is_actual,
            created_at: r.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/v1/admin/forecasts/{id}/results
/// Get forecast results
pub async fn get_results(
    service: web::Data<DynForecastService>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let scenario_id = path.into_inner();
    let results = service.get_results(scenario_id).await?;

    let response: Vec<ResultResponse> = results
        .into_iter()
        .map(|r| ResultResponse {
            id: r.id,
            scenario_id: r.scenario_id,
            week_number: r.week_number,
            week_start: r.week_start,
            week_end: r.week_end,
            country_code: r.country_code,
            task_code: r.task_code,
            opening_licenses: r.opening_licenses,
            new_claims: r.new_claims,
            activations: r.activations,
            churned: r.churned,
            closing_licenses: r.closing_licenses,
            active_licenses: r.active_licenses,
            productive_licenses: r.productive_licenses,
            gross_revenue_micros: r.gross_revenue_micros,
            participant_share_micros: r.participant_share_micros,
            referral_share_micros: r.referral_share_micros,
            uno_share_micros: r.uno_share_micros,
            acquisition_cost_micros: r.acquisition_cost_micros,
            support_cost_micros: r.support_cost_micros,
            hosting_cost_micros: r.hosting_cost_micros,
            messaging_cost_micros: r.messaging_cost_micros,
            other_cost_micros: r.other_cost_micros,
            total_cost_micros: r.total_cost_micros,
            net_profit_micros: r.net_profit_micros,
            cumulative_profit_micros: r.cumulative_profit_micros,
            break_even_reached: r.break_even_reached,
            funding_required_micros: r.funding_required_micros,
            cumulative_funding_micros: r.cumulative_funding_micros,
            is_actual: r.is_actual,
            created_at: r.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/v1/admin/forecasts/{id}/export
/// Export a scenario with all data
pub async fn export_scenario(
    service: web::Data<DynForecastService>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let scenario_id = path.into_inner();
    let export = service.export_scenario(scenario_id).await?;
    Ok(HttpResponse::Ok().json(export))
}

/// POST /api/v1/admin/forecasts/import
/// Import a scenario from JSON
pub async fn import_scenario(
    service: web::Data<DynForecastService>,
    body: web::Json<ImportScenarioRequest>,
) -> Result<impl Responder, AppError> {
    use crate::server::repositories::ScenarioExport;

    let export: ScenarioExport = serde_json::from_value(body.scenario.clone())
        .map_err(|e| AppError::BadRequest(format!("Invalid scenario format: {}", e)))?;

    // Use "import" as created_by, could be expanded to take from request
    let created_by = "import";
    let scenario = service
        .import_scenario(export, created_by)
        .await?;

    Ok(HttpResponse::Created().json(ScenarioResponse {
        id: scenario.id,
        name: scenario.name,
        description: scenario.description,
        version: scenario.version,
        status: format!("{:?}", scenario.status).to_lowercase(),
        created_by: scenario.created_by,
        approved_by: scenario.approved_by,
        approved_at: scenario.approved_at,
        start_date: scenario.start_date,
        end_date: scenario.end_date,
        snapshot_date: scenario.snapshot_date,
        algorithm_version: scenario.algorithm_version,
        assumptions: scenario.assumptions,
        created_at: scenario.created_at,
        updated_at: scenario.updated_at,
    }))
}

/// POST /api/v1/admin/forecasts/{id}/validate
/// Validate scenario results against golden fixture
pub async fn validate_scenario(
    service: web::Data<DynForecastService>,
    path: web::Path<Uuid>,
    body: web::Json<ValidateRequest>,
) -> Result<impl Responder, AppError> {
    let scenario_id = path.into_inner();
    let result = service
        .validate_against_golden(&body.fixture_name, scenario_id)
        .await?;

    let differences_count = result.differences.len();
    let differences: Vec<ValidationDifferenceResponse> = result
        .differences
        .into_iter()
        .map(|d| ValidationDifferenceResponse {
            field: d.field,
            expected: d.expected,
            actual: d.actual,
            difference_pct: d.difference_pct,
        })
        .collect();

    let response = ValidationResultResponse {
        valid: result.valid,
        fixture_name: result.fixture_name,
        differences,
        message: if result.valid {
            "Scenario results match golden fixture".to_string()
        } else {
            format!(
                "Scenario results differ from golden fixture: {} differences found",
                differences_count
            )
        },
    };

    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/v1/admin/forecasts/golden
/// List golden fixtures
pub async fn list_golden_fixtures(
    service: web::Data<DynForecastService>,
) -> Result<impl Responder, AppError> {
    let fixtures = service.list_golden_fixtures().await?;

    let response: Vec<GoldenFixtureResponse> = fixtures
        .into_iter()
        .map(|f| GoldenFixtureResponse {
            id: f.id,
            fixture_name: f.fixture_name,
            description: f.description,
            algorithm_version: f.algorithm_version,
            approved_by: f.approved_by,
            approved_at: f.approved_at,
            created_at: f.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/admin/forecasts/{id}/golden
/// Create a golden fixture from scenario results
pub async fn create_golden_fixture(
    service: web::Data<DynForecastService>,
    path: web::Path<Uuid>,
    body: web::Json<CreateGoldenFixtureRequest>,
) -> Result<impl Responder, AppError> {
    let scenario_id = path.into_inner();

    // Get scenario data to use as fixture inputs
    let scenario = service.get_scenario(scenario_id).await?
        .ok_or_else(|| AppError::NotFound(format!("Scenario {} not found", scenario_id)))?;
    let tasks = service.get_scenario_tasks(scenario_id).await?;
    let results = service.get_results(scenario_id).await?;

    if results.is_empty() {
        return Err(AppError::ValidationError("Cannot create golden fixture without results. Run forecast first.".to_string()));
    }

    // Calculate totals
    let totals = serde_json::json!({
        "total_weeks": results.iter().map(|r| r.week_number).max().unwrap_or(0),
        "final_cumulative_profit_micros": results.last().map(|r| r.cumulative_profit_micros).unwrap_or(0),
        "break_even_week": results.iter().find(|r| r.break_even_reached).map(|r| r.week_number),
    });

    let fixture = service
        .create_golden_fixture(CreateGoldenFixtureInput {
            fixture_name: body.fixture_name.clone(),
            description: body.description.clone(),
            input_scenario: serde_json::to_value(&scenario).unwrap_or_default(),
            input_tasks: serde_json::to_value(&tasks).unwrap_or_default(),
            expected_results: serde_json::to_value(&results).unwrap_or_default(),
            expected_totals: totals,
            algorithm_version: scenario.algorithm_version.clone(),
            tolerance_percentage: 0.01, // 1% tolerance
        })
        .await?;

    Ok(HttpResponse::Created().json(GoldenFixtureResponse {
        id: fixture.id,
        fixture_name: fixture.fixture_name,
        description: fixture.description,
        algorithm_version: fixture.algorithm_version,
        approved_by: fixture.approved_by,
        approved_at: fixture.approved_at,
        created_at: fixture.created_at,
    }))
}

// NOTE: get_golden_fixture endpoint not yet implemented - need to add to service trait
