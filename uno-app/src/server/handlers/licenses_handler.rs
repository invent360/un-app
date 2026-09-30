//! License-related API handlers

use crate::server::app::ServiceFactory;
use crate::server::extractors::auth::{get_authenticated_user, Permission};
use crate::types::{ClaimRequest, LicenseVariant};
use actix_web::{web, HttpRequest, HttpResponse};

/// GET /api/v1/licenses/variants
/// Returns available license split options
pub async fn get_variants(factory: Option<web::Data<ServiceFactory>>) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Database not available",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Get available split options
    match factory.license_service.get_available_splits().await {
        Ok(splits) => {
            // Convert to LicenseVariant for UI compatibility
            let variants: Vec<LicenseVariant> = splits
                .into_iter()
                .map(|sa| {
                    LicenseVariant::from_split_with_counts(
                        sa.split_type,
                        sa.available,
                        sa.claimed,
                        sa.total,
                    )
                })
                .collect();
            HttpResponse::Ok().json(variants)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string(),
            "code": "INTERNAL_ERROR"
        })),
    }
}

/// POST /api/v1/licenses/claim
///
/// Requires authentication with LicenseClaim permission.
/// The authenticated user's ID is used to bind the claimed license to the owner.
pub async fn claim_license(
    req: HttpRequest,
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<ClaimRequest>,
) -> HttpResponse {
    // Require authentication
    let user = match get_authenticated_user(&req) {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }
    };

    // Require LicenseClaim permission
    if let Err(_) = user.require(Permission::LicenseClaim) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Insufficient permissions to claim licenses",
            "code": "FORBIDDEN"
        }));
    }

    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Database not available",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Check session blacklist for revoked tokens
    // R3-03: Fail closed on database errors - security over availability
    if let Some(token) = user.token() {
        match factory.session_service.is_session_valid(token).await {
            Ok(false) => {
                return HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Session has been revoked",
                    "code": "SESSION_REVOKED"
                }));
            }
            Err(e) => {
                tracing::error!(error = %e, "Session validation unavailable - failing closed");
                return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                    "error": "Auth service unavailable",
                    "code": "SERVICE_UNAVAILABLE"
                }));
            }
            Ok(true) => {}
        }
    }

    // Parse user_id for ownership binding
    let user_id = match uuid::Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user identifier",
                "code": "BAD_REQUEST"
            }));
        }
    };

    // Create claim request with authenticated user_id
    let mut claim_request = body.into_inner();
    claim_request.user_id = Some(user_id);

    match factory.license_service.claim_license(claim_request).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            let status = if matches!(e, crate::types::AppError::LicenseUnavailable(_)) {
                actix_web::http::StatusCode::SERVICE_UNAVAILABLE
            } else {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            };
            let code = if status == actix_web::http::StatusCode::SERVICE_UNAVAILABLE {
                "SERVICE_UNAVAILABLE"
            } else {
                "INTERNAL_ERROR"
            };
            HttpResponse::build(status).json(serde_json::json!({
                "error": e.to_string(),
                "code": code
            }))
        }
    }
}
