//! License lifecycle HTTP handlers
//!
//! Provides REST endpoints for license cancellation, release, reactivation, and exposure tracking.

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::server::app::ServiceFactory;
use crate::server::services::{
    CancelLicenseInput, ReleaseLicenseInput, ReactivateLicenseInput,
};

// ============================================
// REQUEST/RESPONSE TYPES
// ============================================

#[derive(Debug, Deserialize)]
pub struct CancelRequest {
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReactivateRequest {
    pub new_valid_to: DateTime<Utc>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PendingExpiryQuery {
    pub days: Option<i32>,
    pub limit: Option<i32>,
}

// ============================================
// HANDLERS
// ============================================

/// POST /api/v1/admin/licenses/{id}/cancel
/// Cancel a license
pub async fn cancel_license(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    body: web::Json<CancelRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let license_id = path.into_inner();
    let canceller_id = get_admin_id(&req);

    let input = CancelLicenseInput {
        license_id,
        canceller_id,
        reason: body.reason.clone(),
    };

    match factory.lifecycle_service.cancel(input).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "License cancelled"
        })),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/admin/licenses/{id}/release
/// Release a license (user-initiated)
pub async fn release_license(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    body: web::Json<ReleaseRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let license_id = path.into_inner();
    let releaser_id = get_admin_id(&req);

    let input = ReleaseLicenseInput {
        license_id,
        releaser_id,
        reason: body.reason.clone(),
    };

    match factory.lifecycle_service.release(input).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "License released"
        })),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/admin/licenses/{id}/reactivate
/// Reactivate an expired or cancelled license
pub async fn reactivate_license(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    body: web::Json<ReactivateRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let license_id = path.into_inner();
    let reactivator_id = get_admin_id(&req);

    let input = ReactivateLicenseInput {
        license_id,
        reactivator_id,
        new_valid_to: body.new_valid_to,
        reason: body.reason.clone(),
    };

    match factory.lifecycle_service.reactivate(input).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "License reactivated"
        })),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/licenses/{id}/exposure
/// Get exposure metrics for a license
pub async fn get_exposure(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let license_id = path.into_inner();

    match factory.lifecycle_service.get_exposure(&license_id).await {
        Ok(metrics) => HttpResponse::Ok().json(metrics),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/licenses/{id}/lifecycle
/// Get lifecycle history for a license
pub async fn get_lifecycle_history(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let license_id = path.into_inner();

    match factory.lifecycle_service.get_lifecycle_history(&license_id).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/licenses/pending-expiry
/// Get licenses pending expiry
pub async fn get_pending_expiry(
    factory: Option<web::Data<ServiceFactory>>,
    query: web::Query<PendingExpiryQuery>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let days = query.days.unwrap_or(30);
    let limit = query.limit.unwrap_or(100).min(500);

    match factory.lifecycle_service.get_pending_expiry(days, limit).await {
        Ok(licenses) => HttpResponse::Ok().json(licenses),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/admin/licenses/process-expired
/// Trigger processing of expired licenses
pub async fn process_expired(
    factory: Option<web::Data<ServiceFactory>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    match factory.lifecycle_service.process_expired().await {
        Ok(count) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "processed_count": count
        })),
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
        _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
    };

    HttpResponse::build(status).json(serde_json::json!({
        "error": e.to_string(),
        "code": e.error_response().code
    }))
}

fn get_admin_id(req: &HttpRequest) -> String {
    // Extract admin ID from request extensions or headers
    // In production, this would come from the authenticated admin context
    req.headers()
        .get("X-Admin-Id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "system".to_string())
}
