//! Audit log API handlers with HMAC authentication

use actix_web::{HttpResponse, web};
use uno_api::auth::{verify_request_with_replay_protection_async, SignedRequest};
use crate::server::app::ServiceFactory;
use crate::types::AuditLogFilter;

/// Maximum request age in seconds (5 minutes)
const MAX_REQUEST_AGE_SECS: i64 = 300;

/// POST /api/v1/admin/audit-logs
/// List audit logs with filtering
pub async fn list_audit_logs(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<AuditLogFilter>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    match factory.audit_service.list_logs(body.payload.clone()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            tracing::error!("Failed to list audit logs: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/contents/{id}/audit
/// Get audit history for a specific content item
pub async fn get_content_audit_history(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    body: web::Json<SignedRequest<serde_json::Value>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let content_id = path.into_inner();

    match factory.audit_service.get_content_history(content_id).await {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(e) => {
            tracing::error!("Failed to get content audit history: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// Configure audit API routes
pub fn configure_audit_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin")
            .route("/audit-logs", web::post().to(list_audit_logs))
            .route("/contents/{id}/audit", web::post().to(get_content_audit_history))
    );
}
