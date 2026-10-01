//! R3-13: Onboarding journey HTTP handlers
//!
//! Provides endpoints for user onboarding progress tracking and funnel analytics.
//!
//! R5-02: All user-facing onboarding endpoints require authentication.
//! User identity is derived from the authenticated principal, never from request body/query.
//! Client cannot set authoritative fields like device_verified - those require server verification.

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::app::ServiceFactory;
use crate::server::extractors::auth::{get_authenticated_user, Permission};
use crate::server::services::JourneyService;
use crate::types::{OnboardingProgress, OnboardingState, SetupStep};

// ============================================
// REQUEST/RESPONSE TYPES
// ============================================

// Admin-only request types (user_id from path, authenticated by AdminAuth middleware)
#[derive(Debug, Deserialize)]
pub struct AdminGetProgressPath {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct AdminTransitionRequest {
    pub new_state: OnboardingState,
    pub reason: Option<String>,
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

// User-facing request types (user_id derived from authenticated principal)
// R5-02: No user_id in requests - always from auth token
// R5-02: device_verified cannot be client-controlled - requires server verification

#[derive(Debug, Deserialize)]
pub struct CompleteEligibilityRequest {
    pub country_code: String,
    pub referral_code: Option<String>,
    // Note: device_verified is intentionally omitted - must be verified server-side
}

#[derive(Debug, Deserialize)]
pub struct AcceptEconomicsRequest {
    pub consent_version: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CompleteSetupStepRequest {
    pub step: SetupStep,
}

#[derive(Debug, Deserialize)]
pub struct ReserveLicenseRequest {
    pub license_id: Uuid,
    // Note: expires_at is server-controlled for security
}

#[derive(Debug, Deserialize)]
pub struct ActivateLicenseRequest {
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

/// Get or create onboarding progress for authenticated user
/// R5-02: Requires authentication, derives user_id from principal
pub async fn get_or_create_progress(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
) -> HttpResponse {
    // R5-02: Require authentication
    let user = match get_authenticated_user(&req) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }
    };

    // R5-02: Parse user_id from authenticated principal
    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user identifier",
                "code": "BAD_REQUEST"
            }));
        }
    };

    match factory.journey_service.get_or_create_progress(user_id).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Get existing progress (admin endpoint - user_id from path)
pub async fn get_progress(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
    path: web::Path<AdminGetProgressPath>,
) -> HttpResponse {
    // R5-02: Require admin permission for viewing any user's progress
    let user = match get_authenticated_user(&req) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }
    };

    // Admin endpoints require Operator permission (enforced by AdminAuth middleware,
    // but we double-check here for defense in depth)
    if user.require(Permission::Operator).is_err() {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Operator permission required",
            "code": "FORBIDDEN"
        }));
    }

    match factory.journey_service.get_progress(path.user_id).await {
        Ok(Some(progress)) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "No onboarding progress found for user"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Resume onboarding from paused state (user-facing, requires auth)
/// R5-02: User_id derived from authenticated principal
pub async fn resume_onboarding(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
) -> HttpResponse {
    // R5-02: Require authentication
    let user = match get_authenticated_user(&req) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }
    };

    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user identifier",
                "code": "BAD_REQUEST"
            }));
        }
    };

    match factory.journey_service.resume_onboarding(user_id).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Transition to new state (admin endpoint - user_id from path)
/// R5-02: Actor derived from authenticated principal
pub async fn transition_state(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
    path: web::Path<AdminGetProgressPath>,
    body: web::Json<AdminTransitionRequest>,
) -> HttpResponse {
    // R5-02: Require admin permission
    let user = match get_authenticated_user(&req) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }
    };

    if user.require(Permission::Operator).is_err() {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Operator permission required",
            "code": "FORBIDDEN"
        }));
    }

    // R5-02: Actor is always the authenticated user, not from request
    match factory.journey_service.transition_state(
        path.user_id,
        body.new_state.clone(),
        Some(&user.id),
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
/// R5-02: Requires authentication, user_id from principal
/// R5-02: device_verified is NOT client-controlled - server must verify independently
pub async fn complete_eligibility(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
    body: web::Json<CompleteEligibilityRequest>,
) -> HttpResponse {
    // R5-02: Require authentication
    let user = match get_authenticated_user(&req) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }
    };

    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user identifier",
                "code": "BAD_REQUEST"
            }));
        }
    };

    // R5-02: device_verified=false until server verifies device independently
    // TODO: Implement actual device verification via upstream provider
    let device_verified = false;

    match factory.journey_service.complete_eligibility(
        user_id,
        &body.country_code,
        device_verified,
        body.referral_code.as_deref(),
    ).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Accept economics/terms
/// R5-02: Requires authentication, user_id from principal
pub async fn accept_economics(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
    body: web::Json<AcceptEconomicsRequest>,
) -> HttpResponse {
    // R5-02: Require authentication
    let user = match get_authenticated_user(&req) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }
    };

    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user identifier",
                "code": "BAD_REQUEST"
            }));
        }
    };

    match factory.journey_service.accept_economics(
        user_id,
        body.consent_version,
    ).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Complete setup step
/// R5-02: Requires authentication, user_id from principal
pub async fn complete_setup_step(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
    body: web::Json<CompleteSetupStepRequest>,
) -> HttpResponse {
    // R5-02: Require authentication
    let user = match get_authenticated_user(&req) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }
    };

    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user identifier",
                "code": "BAD_REQUEST"
            }));
        }
    };

    match factory.journey_service.complete_setup_step(user_id, body.step).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Reserve license
/// R5-02: Requires authentication, user_id from principal
/// R5-02: expires_at is server-controlled for security
pub async fn reserve_license(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
    body: web::Json<ReserveLicenseRequest>,
) -> HttpResponse {
    // R5-02: Require authentication
    let user = match get_authenticated_user(&req) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }
    };

    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user identifier",
                "code": "BAD_REQUEST"
            }));
        }
    };

    // R5-02: Server-controlled reservation expiry (15 minutes from now)
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(15);

    match factory.journey_service.reserve_license(
        user_id,
        body.license_id,
        expires_at,
    ).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// Activate license (complete onboarding)
/// R5-02: Requires authentication, user_id from principal
pub async fn activate_license(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
    body: web::Json<ActivateLicenseRequest>,
) -> HttpResponse {
    // R5-02: Require authentication
    let user = match get_authenticated_user(&req) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }
    };

    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user identifier",
                "code": "BAD_REQUEST"
            }));
        }
    };

    match factory.journey_service.activate_license(user_id, body.license_id).await {
        Ok(progress) => HttpResponse::Ok().json(ProgressResponse { progress }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}
