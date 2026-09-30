//! Agent workflow HTTP handlers
//!
//! Provides REST endpoints for agent approval, rejection, suspension, and lifecycle management.
//!
//! # Authentication
//! All mutating endpoints require JWT authentication. Actor identity for audit trails
//! is derived from the verified JWT token (R4-02 security fix).

use actix_web::{web, HttpRequest, HttpResponse};
use crate::server::extractors::auth::get_actor_id;
use serde::Deserialize;

use crate::server::app::ServiceFactory;
use crate::server::services::{
    AgentStatus, ApproveAgentInput, RejectAgentInput, SuspendAgentInput, LiftSuspensionInput,
};

// ============================================
// REQUEST/RESPONSE TYPES
// ============================================

#[derive(Debug, Deserialize)]
pub struct ListAgentsQuery {
    pub status: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PendingExpirylQuery {
    pub days: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ApproveRequest {
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RejectRequest {
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct SuspendRequest {
    pub reason: String,
    pub duration_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct LiftSuspensionRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TerminateRequest {
    pub reason: String,
}

// ============================================
// HANDLERS
// ============================================

/// GET /api/v1/admin/agents
/// List agents with optional status filter
pub async fn list_agents(
    factory: Option<web::Data<ServiceFactory>>,
    query: web::Query<ListAgentsQuery>,
    _req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let status = query.status.as_ref().and_then(|s| AgentStatus::from_str(s));
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    match factory.agent_service.list_agents(status, limit, offset).await {
        Ok(agents) => HttpResponse::Ok().json(agents),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/agents/pending
/// Get agents pending approval
pub async fn get_pending_approvals(
    factory: Option<web::Data<ServiceFactory>>,
    query: web::Query<ListAgentsQuery>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let limit = query.limit.unwrap_or(50).min(100);

    match factory.agent_service.get_pending_approvals(limit).await {
        Ok(agents) => HttpResponse::Ok().json(agents),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/agents/summary
/// Get agent status summary
pub async fn get_summary(factory: Option<web::Data<ServiceFactory>>) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    match factory.agent_service.get_summary().await {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/agents/{id}
/// Get agent by ID
pub async fn get_agent(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let agent_id = path.into_inner();

    match factory.agent_service.get_agent(&agent_id).await {
        Ok(Some(agent)) => HttpResponse::Ok().json(agent),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Agent not found",
            "code": "NOT_FOUND"
        })),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/admin/agents/{id}/approve
/// Approve an agent
pub async fn approve_agent(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    body: web::Json<ApproveRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    // R4-02: Actor ID derived from verified JWT, not insecure header
    let approver_id = match get_actor_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };

    let agent_id = path.into_inner();

    let input = ApproveAgentInput {
        agent_id,
        approver_id,
        notes: body.notes.clone(),
    };

    match factory.agent_service.approve(input).await {
        Ok(agent) => HttpResponse::Ok().json(agent),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/admin/agents/{id}/reject
/// Reject an agent application
pub async fn reject_agent(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    body: web::Json<RejectRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    // R4-02: Actor ID derived from verified JWT, not insecure header
    let rejector_id = match get_actor_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };

    let agent_id = path.into_inner();

    let input = RejectAgentInput {
        agent_id,
        rejector_id,
        reason: body.reason.clone(),
    };

    match factory.agent_service.reject(input).await {
        Ok(agent) => HttpResponse::Ok().json(agent),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/admin/agents/{id}/suspend
/// Suspend an agent
pub async fn suspend_agent(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    body: web::Json<SuspendRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    // R4-02: Actor ID derived from verified JWT, not insecure header
    let suspender_id = match get_actor_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };

    let agent_id = path.into_inner();

    let input = SuspendAgentInput {
        agent_id,
        suspender_id,
        reason: body.reason.clone(),
        duration_days: body.duration_days,
    };

    match factory.agent_service.suspend(input).await {
        Ok(agent) => HttpResponse::Ok().json(agent),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/admin/agents/{id}/lift-suspension
/// Lift an agent's suspension
pub async fn lift_suspension(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    body: web::Json<LiftSuspensionRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    // R4-02: Actor ID derived from verified JWT, not insecure header
    let lifter_id = match get_actor_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };

    let agent_id = path.into_inner();

    let input = LiftSuspensionInput {
        agent_id,
        lifter_id,
        reason: body.reason.clone(),
    };

    match factory.agent_service.lift_suspension(input).await {
        Ok(agent) => HttpResponse::Ok().json(agent),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/admin/agents/{id}/terminate
/// Terminate an agent
pub async fn terminate_agent(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    body: web::Json<TerminateRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    // R4-02: Actor ID derived from verified JWT, not insecure header
    let terminator_id = match get_actor_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };

    let agent_id = path.into_inner();

    match factory.agent_service.terminate(&agent_id, &terminator_id, &body.reason).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Agent terminated"
        })),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/agents/{id}/history
/// Get agent status history
pub async fn get_status_history(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let agent_id = path.into_inner();

    match factory.agent_service.get_status_history(&agent_id).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(e) => error_response(e),
    }
}

// ============================================
// HELPER FUNCTIONS
// ============================================

fn service_unavailable() -> HttpResponse {
    HttpResponse::ServiceUnavailable().json(serde_json::json!({
        "error": "Database not available",
        "code": "SERVICE_UNAVAILABLE"
    }))
}

fn error_response(e: crate::types::AppError) -> HttpResponse {
    let status = match &e {
        crate::types::AppError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
        crate::types::AppError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
        crate::types::AppError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
        crate::types::AppError::Forbidden(_) => actix_web::http::StatusCode::FORBIDDEN,
        crate::types::AppError::ServiceUnavailable(_) => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
        _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
    };

    HttpResponse::build(status).json(serde_json::json!({
        "error": e.to_string(),
        "code": e.error_response().code
    }))
}

// R4-02: Removed insecure get_admin_id function that read from X-Admin-Id header.
// Actor identity is now derived from verified JWT via get_actor_id().
