//! Voluntary exit handlers for Phase 7
//!
//! Endpoints for:
//! - Participant exit workflow
//! - Payout processing
//! - Exit feedback
//! - Market waitlist

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::extractors::auth::get_actor_id;
use crate::server::repositories::{
    DynExitRepository, ExitType, ExitStatus,
    InitiateExitInput, SubmitFeedbackInput, AddToWaitlistInput,
};
use crate::types::AppError;

// ============================================
// REQUEST/RESPONSE TYPES
// ============================================

// R4-02: user_id and requested_by now come from authenticated token
#[derive(Debug, Deserialize)]
pub struct InitiateExitRequest {
    pub license_id: String,
    pub exit_type: Option<String>,
    pub exit_reason: Option<String>,
    pub exit_details: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RequestPayoutRequest {
    pub method: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessPayoutRequest {
    pub reference: String,
}

#[derive(Debug, Deserialize)]
pub struct FailPayoutRequest {
    pub reason: String,
}

// R4-02: rejected_by now comes from authenticated token
#[derive(Debug, Deserialize)]
pub struct RejectExitRequest {
    pub reason: String,
}

// R4-02: processed_by now comes from authenticated token (no body fields needed)
#[derive(Debug, Deserialize)]
pub struct ApproveExitRequest {}

// R4-02: cancelled_by now comes from authenticated token (no body fields needed)
#[derive(Debug, Deserialize)]
pub struct CancelExitRequest {}

// R4-02: actor_id now comes from authenticated token
#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitFeedbackRequest {
    pub primary_reason: Option<String>,
    pub satisfaction_score: Option<i32>,
    pub would_recommend: Option<bool>,
    pub earnings_met_expectations: Option<bool>,
    pub support_quality_score: Option<i32>,
    pub comments: Option<String>,
    pub improvement_suggestions: Option<String>,
}

// R4-02: user_id now comes from authenticated token (optional - unauthenticated users can join waitlist)
#[derive(Debug, Deserialize)]
pub struct AddToWaitlistRequest {
    pub email: String,
    pub country_code: String,
    pub consent_given: bool,
    pub consent_version: Option<String>,
    pub source: Option<String>,
    pub campaign_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWaitlistStatusRequest {
    pub status: String,
    pub notification_reference: Option<String>,
}

// R4-02: user_id now comes from authenticated token
#[derive(Debug, Deserialize)]
pub struct CalculateBalanceRequest {
    pub license_id: String,
}

#[derive(Debug, Deserialize)]
pub struct WaitlistNotifyQuery {
    pub country_code: String,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct StatusQuery {
    pub status: Option<String>,
    pub limit: Option<i32>,
}

// ============================================
// PUBLIC ENDPOINTS (User-facing)
// ============================================

/// POST /api/v1/exit
/// Initiate a voluntary exit
pub async fn initiate_exit(
    req: HttpRequest,
    repo: web::Data<DynExitRepository>,
    body: web::Json<InitiateExitRequest>,
) -> Result<HttpResponse, AppError> {
    // R4-02: Get user_id from authenticated token, not request body
    let user_id = get_actor_id(&req)?;

    let exit_type = body.exit_type
        .as_ref()
        .and_then(|t| ExitType::from_str(t))
        .unwrap_or(ExitType::Voluntary);

    let input = InitiateExitInput {
        user_id: user_id.clone(),
        license_id: body.license_id.clone(),
        exit_type,
        exit_reason: body.exit_reason.clone(),
        exit_details: body.exit_details.clone(),
        // R4-02: requested_by is always the authenticated user
        requested_by: user_id,
        requested_by_type: "user".to_string(),
    };

    let exit = repo.initiate_exit(input).await?;
    Ok(HttpResponse::Created().json(exit))
}

/// GET /api/v1/exit/{id}
/// Get exit by ID
pub async fn get_exit(
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let exit = repo.get_exit(id).await?
        .ok_or_else(|| AppError::NotFound(format!("Exit {} not found", id)))?;
    Ok(HttpResponse::Ok().json(exit))
}

/// GET /api/v1/exit/license/{license_id}
/// Get exit by license ID
pub async fn get_exit_by_license(
    repo: web::Data<DynExitRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let license_id = path.into_inner();
    let exit = repo.get_exit_by_license(&license_id).await?;

    match exit {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::Ok().json(serde_json::json!({
            "has_exit": false
        }))),
    }
}

/// GET /api/v1/exit/user/{user_id}
/// Get all exits for a user
pub async fn get_user_exits(
    repo: web::Data<DynExitRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let exits = repo.get_user_exits(&user_id).await?;
    Ok(HttpResponse::Ok().json(exits))
}

/// POST /api/v1/exit/balance
/// Calculate exit balance (preview before exit)
pub async fn calculate_balance(
    req: HttpRequest,
    repo: web::Data<DynExitRepository>,
    body: web::Json<CalculateBalanceRequest>,
) -> Result<HttpResponse, AppError> {
    // R4-02: Get user_id from authenticated token, not request body
    let user_id = get_actor_id(&req)?;
    let balance = repo.calculate_exit_balance(&user_id, &body.license_id).await?;
    Ok(HttpResponse::Ok().json(balance))
}

/// POST /api/v1/exit/{id}/cancel
/// Cancel exit (user can cancel before processing)
pub async fn cancel_exit(
    req: HttpRequest,
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
    _body: web::Json<CancelExitRequest>,
) -> Result<HttpResponse, AppError> {
    // R4-02: Get cancelled_by from authenticated token, not request body
    let actor_id = get_actor_id(&req)?;
    let id = path.into_inner();
    let exit = repo.cancel_exit(id, &actor_id, "user").await?;
    Ok(HttpResponse::Ok().json(exit))
}

/// POST /api/v1/exit/{id}/payout
/// Request payout (user selects payout method)
pub async fn request_payout(
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
    body: web::Json<RequestPayoutRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let exit = repo.request_payout(id, &body.method, body.details.clone()).await?;
    Ok(HttpResponse::Ok().json(exit))
}

/// POST /api/v1/exit/{id}/feedback
/// Submit exit feedback
pub async fn submit_feedback(
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
    body: web::Json<SubmitFeedbackRequest>,
) -> Result<HttpResponse, AppError> {
    let exit_id = path.into_inner();

    // Validate score if provided
    if let Some(score) = body.satisfaction_score {
        if score < 1 || score > 5 {
            return Err(AppError::BadRequest("Satisfaction score must be between 1 and 5".to_string()));
        }
    }
    if let Some(score) = body.support_quality_score {
        if score < 1 || score > 5 {
            return Err(AppError::BadRequest("Support quality score must be between 1 and 5".to_string()));
        }
    }

    let input = SubmitFeedbackInput {
        exit_id,
        primary_reason: body.primary_reason.clone(),
        satisfaction_score: body.satisfaction_score,
        would_recommend: body.would_recommend,
        earnings_met_expectations: body.earnings_met_expectations,
        support_quality_score: body.support_quality_score,
        comments: body.comments.clone(),
        improvement_suggestions: body.improvement_suggestions.clone(),
    };

    let feedback = repo.submit_feedback(input).await?;
    Ok(HttpResponse::Created().json(feedback))
}

/// GET /api/v1/exit/{id}/feedback
/// Get exit feedback
pub async fn get_feedback(
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let exit_id = path.into_inner();
    let feedback = repo.get_exit_feedback(exit_id).await?;

    match feedback {
        Some(f) => Ok(HttpResponse::Ok().json(f)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "No feedback submitted",
            "code": "NOT_FOUND"
        }))),
    }
}

// ============================================
// WAITLIST ENDPOINTS (Public)
// ============================================

/// POST /api/v1/waitlist
/// Add to market waitlist
pub async fn add_to_waitlist(
    req: HttpRequest,
    repo: web::Data<DynExitRepository>,
    body: web::Json<AddToWaitlistRequest>,
) -> Result<HttpResponse, AppError> {
    // R4-02: Get user_id from authenticated token if available (waitlist allows unauthenticated)
    let user_id = get_actor_id(&req).ok();

    let input = AddToWaitlistInput {
        user_id,
        email: body.email.clone(),
        country_code: body.country_code.clone(),
        consent_given: body.consent_given,
        consent_version: body.consent_version.clone(),
        source: body.source.clone(),
        campaign_id: body.campaign_id,
    };

    let entry = repo.add_to_waitlist(input).await?;
    Ok(HttpResponse::Created().json(entry))
}

/// GET /api/v1/waitlist/{email}/{country_code}
/// Get waitlist entry
pub async fn get_waitlist_entry(
    repo: web::Data<DynExitRepository>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let (email, country_code) = path.into_inner();
    let entry = repo.get_waitlist_entry(&email, &country_code).await?;

    match entry {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Not on waitlist",
            "code": "NOT_FOUND"
        }))),
    }
}

// ============================================
// ADMIN ENDPOINTS
// ============================================

/// GET /api/v1/admin/exits
/// Get exits by status
pub async fn get_exits_by_status(
    repo: web::Data<DynExitRepository>,
    query: web::Query<StatusQuery>,
) -> Result<HttpResponse, AppError> {
    let status = query.status
        .as_ref()
        .and_then(|s| ExitStatus::from_str(s))
        .unwrap_or(ExitStatus::Requested);
    let limit = query.limit.unwrap_or(50);

    let exits = repo.get_exits_by_status(status, limit).await?;
    Ok(HttpResponse::Ok().json(exits))
}

/// GET /api/v1/admin/exits/pending-payout
/// Get exits pending payout
pub async fn get_exits_pending_payout(
    repo: web::Data<DynExitRepository>,
    query: web::Query<StatusQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(50);
    let exits = repo.get_exits_pending_payout(limit).await?;
    Ok(HttpResponse::Ok().json(exits))
}

/// POST /api/v1/admin/exits/{id}/approve
/// Approve exit
pub async fn approve_exit(
    req: HttpRequest,
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
    _body: web::Json<ApproveExitRequest>,
) -> Result<HttpResponse, AppError> {
    // R4-02: Get processed_by from authenticated token, not request body
    let actor_id = get_actor_id(&req)?;
    let id = path.into_inner();
    let exit = repo.approve_exit(id, &actor_id, "admin").await?;
    Ok(HttpResponse::Ok().json(exit))
}

/// POST /api/v1/admin/exits/{id}/reject
/// Reject exit
pub async fn reject_exit(
    req: HttpRequest,
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
    body: web::Json<RejectExitRequest>,
) -> Result<HttpResponse, AppError> {
    // R4-02: Get rejected_by from authenticated token, not request body
    let actor_id = get_actor_id(&req)?;
    let id = path.into_inner();
    let exit = repo.reject_exit(id, &body.reason, &actor_id, "admin").await?;
    Ok(HttpResponse::Ok().json(exit))
}

/// POST /api/v1/admin/exits/{id}/status
/// Update exit status
pub async fn update_status(
    req: HttpRequest,
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateStatusRequest>,
) -> Result<HttpResponse, AppError> {
    // R4-02: Get actor_id from authenticated token, not request body
    let actor_id = get_actor_id(&req)?;
    let id = path.into_inner();
    let status = ExitStatus::from_str(&body.status)
        .ok_or_else(|| AppError::BadRequest(format!("Invalid status: {}", body.status)))?;

    let exit = repo.update_status(
        id,
        status,
        &actor_id,
        "admin",
        body.notes.as_deref(),
    ).await?;

    Ok(HttpResponse::Ok().json(exit))
}

/// POST /api/v1/admin/exits/{id}/payout/initiate
/// Initiate payout processing
pub async fn initiate_payout(
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
    body: web::Json<ProcessPayoutRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let exit = repo.initiate_payout(id, &body.reference).await?;
    Ok(HttpResponse::Ok().json(exit))
}

/// POST /api/v1/admin/exits/{id}/payout/complete
/// Complete payout
pub async fn complete_payout(
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
    body: web::Json<ProcessPayoutRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let exit = repo.complete_payout(id, &body.reference).await?;
    Ok(HttpResponse::Ok().json(exit))
}

/// POST /api/v1/admin/exits/{id}/payout/fail
/// Fail payout
pub async fn fail_payout(
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
    body: web::Json<FailPayoutRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let exit = repo.fail_payout(id, &body.reason).await?;
    Ok(HttpResponse::Ok().json(exit))
}

/// GET /api/v1/admin/exits/{id}/audit
/// Get exit audit log
pub async fn get_audit_log(
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let exit_id = path.into_inner();
    let logs = repo.get_exit_audit_log(exit_id).await?;
    Ok(HttpResponse::Ok().json(logs))
}

// ============================================
// ADMIN WAITLIST ENDPOINTS
// ============================================

/// GET /api/v1/admin/waitlist/{country_code}
/// Get waitlist for country
pub async fn get_country_waitlist(
    repo: web::Data<DynExitRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let country_code = path.into_inner();
    let entries = repo.get_country_waitlist(&country_code).await?;
    Ok(HttpResponse::Ok().json(entries))
}

/// PUT /api/v1/admin/waitlist/{id}/status
/// Update waitlist entry status
pub async fn update_waitlist_status(
    repo: web::Data<DynExitRepository>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateWaitlistStatusRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let entry = repo.update_waitlist_status(
        id,
        &body.status,
        body.notification_reference.as_deref(),
    ).await?;
    Ok(HttpResponse::Ok().json(entry))
}

/// GET /api/v1/admin/waitlist/notify
/// Get waitlist entries to notify (for worker)
pub async fn get_waitlist_to_notify(
    repo: web::Data<DynExitRepository>,
    query: web::Query<WaitlistNotifyQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(50);
    let entries = repo.get_waitlist_to_notify(&query.country_code, limit).await?;
    Ok(HttpResponse::Ok().json(entries))
}

// ============================================
// ROUTE CONFIGURATION
// ============================================

/// Configure public exit routes
pub fn configure_public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/exit")
            .route("", web::post().to(initiate_exit))
            .route("/balance", web::post().to(calculate_balance))
            .route("/{id}", web::get().to(get_exit))
            .route("/license/{license_id}", web::get().to(get_exit_by_license))
            .route("/user/{user_id}", web::get().to(get_user_exits))
            .route("/{id}/cancel", web::post().to(cancel_exit))
            .route("/{id}/payout", web::post().to(request_payout))
            .route("/{id}/feedback", web::post().to(submit_feedback))
            .route("/{id}/feedback", web::get().to(get_feedback)),
    );

    cfg.service(
        web::scope("/waitlist")
            .route("", web::post().to(add_to_waitlist))
            .route("/{email}/{country_code}", web::get().to(get_waitlist_entry)),
    );
}

/// Configure admin exit routes
pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/exits")
            .route("", web::get().to(get_exits_by_status))
            .route("/pending-payout", web::get().to(get_exits_pending_payout))
            .route("/{id}/approve", web::post().to(approve_exit))
            .route("/{id}/reject", web::post().to(reject_exit))
            .route("/{id}/status", web::post().to(update_status))
            .route("/{id}/payout/initiate", web::post().to(initiate_payout))
            .route("/{id}/payout/complete", web::post().to(complete_payout))
            .route("/{id}/payout/fail", web::post().to(fail_payout))
            .route("/{id}/audit", web::get().to(get_audit_log)),
    );

    cfg.service(
        web::scope("/waitlist")
            .route("/notify", web::get().to(get_waitlist_to_notify))
            .route("/{country_code}", web::get().to(get_country_waitlist))
            .route("/{id}/status", web::put().to(update_waitlist_status)),
    );
}
