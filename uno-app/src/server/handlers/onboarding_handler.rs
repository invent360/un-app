//! R3-13: Onboarding journey HTTP handlers
//!
//! Provides endpoints for user onboarding progress tracking and funnel analytics.

use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::app::ServiceFactory;
use crate::server::services::JourneyService;
use crate::types::{OnboardingProgress, OnboardingState, SetupStep};

// ============================================
// REQUEST/RESPONSE TYPES
// ============================================

#[derive(Debug, Deserialize)]
pub struct GetProgressQuery {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ResumeRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct TransitionRequest {
    pub user_id: Uuid,
    pub new_state: OnboardingState,
    pub actor_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecordFunnelRequest {
    pub session_id: Option<Uuid>,
    pub visitor_id: Option<String>,
    pub stage_id: String,
    pub license_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct FunnelMetricsQuery {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CompleteEligibilityRequest {
    pub user_id: Uuid,
    pub country_code: String,
    pub device_verified: bool,
    pub referral_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AcceptEconomicsRequest {
    pub user_id: Uuid,
    pub consent_version: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CompleteSetupStepRequest {
    pub user_id: Uuid,
    pub step: SetupStep,
}

#[derive(Debug, Deserialize)]
pub struct ReserveLicenseRequest {
    pub user_id: Uuid,
    pub license_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ActivateLicenseRequest {
    pub user_id: Uuid,
    pub license_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ProgressResponse {
    pub progress: OnboardingProgress,
}

#[derive(Debug, Serialize)]
pub struct FunnelEntryResponse {
    pub id: Uuid,
    pub session_id: Option<Uuid>,
    pub visitor_id: Option<String>,
    pub license_id: Option<Uuid>,
    pub stage_id: String,
    pub measured_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StageMetricsResponse {
    pub stage_id: String,
    pub total_entries: i64,
    pub unique_users: i64,
    pub conversions: i64,
    pub conversion_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct FunnelMetricsResponse {
    pub stages: Vec<StageMetricsResponse>,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

// ============================================
// HANDLERS
// ============================================

/// Get or create onboarding progress for a user
pub async fn get_or_create_progress(
    factory: web::Data<ServiceFactory>,
    query: web::Query<GetProgressQuery>,
) -> HttpResponse {
    match factory.journey_service.get_or_create_progress(query.user_id).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Get existing progress (if any)
pub async fn get_progress(
    factory: web::Data<ServiceFactory>,
    query: web::Query<GetProgressQuery>,
) -> HttpResponse {
    match factory.journey_service.get_progress(query.user_id).await {
        Ok(Some(progress)) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "No onboarding progress found for user"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Resume onboarding from paused state
pub async fn resume_onboarding(
    factory: web::Data<ServiceFactory>,
    body: web::Json<ResumeRequest>,
) -> HttpResponse {
    match factory.journey_service.resume_onboarding(body.user_id).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Transition to new state
pub async fn transition_state(
    factory: web::Data<ServiceFactory>,
    body: web::Json<TransitionRequest>,
) -> HttpResponse {
    match factory.journey_service.transition_state(
        body.user_id,
        body.new_state.clone(),
        body.actor_id.as_deref(),
    ).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Record funnel stage for analytics
pub async fn record_funnel_stage(
    factory: web::Data<ServiceFactory>,
    body: web::Json<RecordFunnelRequest>,
) -> HttpResponse {
    match factory.journey_service.record_funnel_stage(
        body.session_id,
        body.visitor_id.as_deref(),
        &body.stage_id,
        body.license_id,
        body.metadata.clone(),
    ).await {
        Ok(entry) => HttpResponse::Ok().json(FunnelEntryResponse {
            id: entry.id,
            session_id: entry.session_id,
            visitor_id: entry.visitor_id,
            license_id: entry.license_id,
            stage_id: entry.stage_id,
            measured_at: entry.measured_at,
        }),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Get funnel metrics for date range
pub async fn get_funnel_metrics(
    factory: web::Data<ServiceFactory>,
    query: web::Query<FunnelMetricsQuery>,
) -> HttpResponse {
    match factory.journey_service.get_funnel_metrics(query.from, query.to).await {
        Ok(stages) => HttpResponse::Ok().json(FunnelMetricsResponse {
            stages: stages.into_iter().map(|s| StageMetricsResponse {
                stage_id: s.stage_id,
                total_entries: s.total_entries,
                unique_users: s.unique_users,
                conversions: s.conversions,
                conversion_rate: s.conversion_rate,
            }).collect(),
            from: query.from,
            to: query.to,
        }),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Complete eligibility check
pub async fn complete_eligibility(
    factory: web::Data<ServiceFactory>,
    body: web::Json<CompleteEligibilityRequest>,
) -> HttpResponse {
    match factory.journey_service.complete_eligibility(
        body.user_id,
        &body.country_code,
        body.device_verified,
        body.referral_code.as_deref(),
    ).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Accept economics/terms
pub async fn accept_economics(
    factory: web::Data<ServiceFactory>,
    body: web::Json<AcceptEconomicsRequest>,
) -> HttpResponse {
    match factory.journey_service.accept_economics(
        body.user_id,
        body.consent_version,
    ).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Complete setup step
pub async fn complete_setup_step(
    factory: web::Data<ServiceFactory>,
    body: web::Json<CompleteSetupStepRequest>,
) -> HttpResponse {
    match factory.journey_service.complete_setup_step(body.user_id, body.step).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Reserve license
pub async fn reserve_license(
    factory: web::Data<ServiceFactory>,
    body: web::Json<ReserveLicenseRequest>,
) -> HttpResponse {
    match factory.journey_service.reserve_license(
        body.user_id,
        body.license_id,
        body.expires_at,
    ).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Activate license (complete onboarding)
pub async fn activate_license(
    factory: web::Data<ServiceFactory>,
    body: web::Json<ActivateLicenseRequest>,
) -> HttpResponse {
    match factory.journey_service.activate_license(body.user_id, body.license_id).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}
