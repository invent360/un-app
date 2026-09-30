//! Pilot handler for controlled rollout management
//!
//! Phase 9: Release Validation - REST endpoints for pilot cohort
//! and participant management.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::app::ServiceFactory;
use crate::server::repositories::{AddParticipantInput, TransitionStateInput, RecordPilotActivityInput, PilotParticipantState};
use crate::server::services::DynPilotService;
use crate::types::AppError;

// =============================================================================
// Request/Response Types
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct AddParticipantRequest {
    pub external_user_id: String,
    pub market_code: String,
    pub cohort_id: String,
    pub invitation_code: Option<String>,
    pub invitation_channel: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransitionStateRequest {
    pub new_state: String,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecordActivityRequest {
    pub activity_type: String,
    pub activity_data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct MilestoneCheckRequest {
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct EnrollmentCheckResponse {
    pub can_enroll: bool,
    pub market_code: String,
}

// =============================================================================
// Admin Handlers (Protected by AdminAuth)
// =============================================================================

/// GET /api/v1/admin/pilot/cohorts - List all cohorts
pub async fn list_cohorts(
    factory: web::Data<ServiceFactory>,
) -> Result<HttpResponse, AppError> {
    let pilot_service = get_pilot_service(&factory)?;
    let cohorts = pilot_service.get_active_cohorts(None).await?;
    Ok(HttpResponse::Ok().json(cohorts))
}

/// GET /api/v1/admin/pilot/cohorts/{id} - Get cohort details
pub async fn get_cohort(
    factory: web::Data<ServiceFactory>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let cohort_id = path.into_inner();
    let pilot_service = get_pilot_service(&factory)?;
    let report = pilot_service.get_cohort_report(&cohort_id).await?;
    Ok(HttpResponse::Ok().json(report))
}

/// GET /api/v1/admin/pilot/cohorts/{id}/participants - List participants
pub async fn list_participants(
    factory: web::Data<ServiceFactory>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let cohort_id = path.into_inner();
    let participants = factory.pilot_repository.as_ref()
        .ok_or_else(|| AppError::InternalServerError("Pilot repository not available".to_string()))?
        .get_cohort_participants(&cohort_id)
        .await?;
    Ok(HttpResponse::Ok().json(participants))
}

/// POST /api/v1/admin/pilot/participants - Add a new participant
pub async fn add_participant(
    factory: web::Data<ServiceFactory>,
    body: web::Json<AddParticipantRequest>,
) -> Result<HttpResponse, AppError> {
    let pilot_service = get_pilot_service(&factory)?;

    let input = AddParticipantInput {
        external_user_id: body.external_user_id.clone(),
        market_code: body.market_code.clone(),
        cohort_id: body.cohort_id.clone(),
        invitation_code: body.invitation_code.clone(),
        invitation_channel: body.invitation_channel.clone(),
    };

    let participant = pilot_service.add_participant(input, "admin").await?;
    Ok(HttpResponse::Created().json(participant))
}

/// GET /api/v1/admin/pilot/participants/{id} - Get participant details
pub async fn get_participant(
    factory: web::Data<ServiceFactory>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let participant_id = path.into_inner();
    let participant = factory.pilot_repository.as_ref()
        .ok_or_else(|| AppError::InternalServerError("Pilot repository not available".to_string()))?
        .get_participant(participant_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Participant not found".to_string()))?;
    Ok(HttpResponse::Ok().json(participant))
}

/// POST /api/v1/admin/pilot/participants/{id}/transition - Transition state
pub async fn transition_state(
    factory: web::Data<ServiceFactory>,
    path: web::Path<Uuid>,
    body: web::Json<TransitionStateRequest>,
) -> Result<HttpResponse, AppError> {
    let participant_id = path.into_inner();
    let pilot_service = get_pilot_service(&factory)?;

    let new_state = parse_state(&body.new_state)?;

    let input = TransitionStateInput {
        participant_id,
        new_state,
        triggered_by: "admin".to_string(),
        actor: Some("admin".to_string()),
        reason: body.reason.clone(),
        metadata: None,
    };

    let participant = pilot_service.transition_state(input).await?;
    Ok(HttpResponse::Ok().json(participant))
}

/// POST /api/v1/admin/pilot/participants/{id}/activity - Record activity
pub async fn record_activity(
    factory: web::Data<ServiceFactory>,
    path: web::Path<Uuid>,
    body: web::Json<RecordActivityRequest>,
) -> Result<HttpResponse, AppError> {
    let participant_id = path.into_inner();
    let pilot_service = get_pilot_service(&factory)?;

    let input = RecordPilotActivityInput {
        participant_id,
        activity_type: body.activity_type.clone(),
        activity_data: body.activity_data.clone(),
    };

    pilot_service.record_activity(input).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// GET /api/v1/admin/pilot/participants/{id}/history - Get state history
pub async fn get_history(
    factory: web::Data<ServiceFactory>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let participant_id = path.into_inner();
    let history = factory.pilot_repository.as_ref()
        .ok_or_else(|| AppError::InternalServerError("Pilot repository not available".to_string()))?
        .get_state_history(participant_id)
        .await?;
    Ok(HttpResponse::Ok().json(history))
}

/// GET /api/v1/admin/pilot/pending/d7 - Get participants pending D7 check
pub async fn get_pending_d7(
    factory: web::Data<ServiceFactory>,
) -> Result<HttpResponse, AppError> {
    let pilot_service = get_pilot_service(&factory)?;
    let participants = pilot_service.get_pending_d7().await?;
    Ok(HttpResponse::Ok().json(participants))
}

/// GET /api/v1/admin/pilot/pending/d30 - Get participants pending D30 check
pub async fn get_pending_d30(
    factory: web::Data<ServiceFactory>,
) -> Result<HttpResponse, AppError> {
    let pilot_service = get_pilot_service(&factory)?;
    let participants = pilot_service.get_pending_d30().await?;
    Ok(HttpResponse::Ok().json(participants))
}

/// POST /api/v1/admin/pilot/participants/{id}/d7 - Process D7 check
pub async fn process_d7(
    factory: web::Data<ServiceFactory>,
    path: web::Path<Uuid>,
    body: web::Json<MilestoneCheckRequest>,
) -> Result<HttpResponse, AppError> {
    let participant_id = path.into_inner();
    let pilot_service = get_pilot_service(&factory)?;
    let participant = pilot_service.process_d7_check(participant_id, body.is_active, "admin").await?;
    Ok(HttpResponse::Ok().json(participant))
}

/// POST /api/v1/admin/pilot/participants/{id}/d30 - Process D30 check
pub async fn process_d30(
    factory: web::Data<ServiceFactory>,
    path: web::Path<Uuid>,
    body: web::Json<MilestoneCheckRequest>,
) -> Result<HttpResponse, AppError> {
    let participant_id = path.into_inner();
    let pilot_service = get_pilot_service(&factory)?;
    let participant = pilot_service.process_d30_check(participant_id, body.is_active, "admin").await?;
    Ok(HttpResponse::Ok().json(participant))
}

// =============================================================================
// Public Handlers (Check enrollment availability)
// =============================================================================

/// GET /api/v1/pilot/{market_code}/check - Check enrollment availability
pub async fn check_enrollment(
    factory: web::Data<ServiceFactory>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let market_code = path.into_inner();
    let pilot_service = get_pilot_service(&factory)?;
    let can_enroll = pilot_service.can_enroll(&market_code).await?;

    Ok(HttpResponse::Ok().json(EnrollmentCheckResponse {
        can_enroll,
        market_code,
    }))
}

// =============================================================================
// Helpers
// =============================================================================

fn get_pilot_service(factory: &ServiceFactory) -> Result<DynPilotService, AppError> {
    factory.pilot_service.clone()
        .ok_or_else(|| AppError::InternalServerError("Pilot service not available".to_string()))
}

fn parse_state(state: &str) -> Result<PilotParticipantState, AppError> {
    match state.to_lowercase().as_str() {
        "invited" => Ok(PilotParticipantState::Invited),
        "registered" => Ok(PilotParticipantState::Registered),
        "license_claimed" => Ok(PilotParticipantState::LicenseClaimed),
        "d7_active" => Ok(PilotParticipantState::D7Active),
        "d7_inactive" => Ok(PilotParticipantState::D7Inactive),
        "d30_active" => Ok(PilotParticipantState::D30Active),
        "d30_inactive" => Ok(PilotParticipantState::D30Inactive),
        "graduated" => Ok(PilotParticipantState::Graduated),
        "dropped" => Ok(PilotParticipantState::Dropped),
        "withdrawn" => Ok(PilotParticipantState::Withdrawn),
        _ => Err(AppError::ValidationError(format!("Invalid state: {}", state))),
    }
}
