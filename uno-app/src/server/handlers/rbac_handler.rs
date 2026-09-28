//! RBAC API handlers with HMAC authentication

use actix_web::{HttpResponse, web};
use uno_api::auth::{verify_request, SignedRequest};
use crate::server::app::ServiceFactory;
use crate::server::middleware::rbac_middleware::check_permission;
use crate::types::rbac::{AssignRoleRequest, RemoveRoleRequest, permissions};

/// Maximum request age in seconds (5 minutes)
const MAX_REQUEST_AGE_SECS: i64 = 300;

/// POST /api/v1/admin/roles
/// List all available roles
pub async fn list_roles(
    factory: Option<web::Data<ServiceFactory>>,
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

    // Verify HMAC signature
    if let Err(e) = verify_request(&body, &factory.client_registry, MAX_REQUEST_AGE_SECS) {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    match factory.rbac_service.list_roles().await {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(e) => {
            tracing::error!("Failed to list roles: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/users/{id}/permissions
/// Get permissions for a specific user
pub async fn get_user_permissions(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
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

    // Verify HMAC signature
    if let Err(e) = verify_request(&body, &factory.client_registry, MAX_REQUEST_AGE_SECS) {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let user_id = path.into_inner();

    match factory.rbac_service.get_user_permissions(&user_id).await {
        Ok(perms) => HttpResponse::Ok().json(perms),
        Err(e) => {
            tracing::error!("Failed to get user permissions: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/users/me/permissions
/// Get permissions for the current user (based on client_id)
pub async fn get_my_permissions(
    factory: Option<web::Data<ServiceFactory>>,
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

    // Verify HMAC signature
    if let Err(e) = verify_request(&body, &factory.client_registry, MAX_REQUEST_AGE_SECS) {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    match factory.rbac_service.get_user_permissions(&body.client_id).await {
        Ok(perms) => HttpResponse::Ok().json(perms),
        Err(e) => {
            tracing::error!("Failed to get user permissions: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/users/roles/assign
/// Assign a role to a user
pub async fn assign_role(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<AssignRoleRequest>>,
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

    // Verify HMAC signature
    if let Err(e) = verify_request(&body, &factory.client_registry, MAX_REQUEST_AGE_SECS) {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    // Check permission to manage users
    if let Err(resp) = check_permission(&factory, &body.client_id, permissions::USER_MANAGE).await {
        return resp;
    }

    match factory.rbac_service.assign_role(
        &body.payload.user_id,
        &body.payload.role_name,
        &body.client_id,
    ).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": format!("Role '{}' assigned to user '{}'", body.payload.role_name, body.payload.user_id)
        })),
        Err(e) => {
            tracing::error!("Failed to assign role: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/users/roles/remove
/// Remove a role from a user
pub async fn remove_role(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<RemoveRoleRequest>>,
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

    // Verify HMAC signature
    if let Err(e) = verify_request(&body, &factory.client_registry, MAX_REQUEST_AGE_SECS) {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    // Check permission to manage users
    if let Err(resp) = check_permission(&factory, &body.client_id, permissions::USER_MANAGE).await {
        return resp;
    }

    match factory.rbac_service.remove_role(
        &body.payload.user_id,
        &body.payload.role_name,
    ).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": format!("Role '{}' removed from user '{}'", body.payload.role_name, body.payload.user_id)
        })),
        Err(e) => {
            tracing::error!("Failed to remove role: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// Configure RBAC API routes
pub fn configure_rbac_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin")
            .route("/roles", web::post().to(list_roles))
            .route("/users/me/permissions", web::post().to(get_my_permissions))
            .route("/users/{id}/permissions", web::post().to(get_user_permissions))
            .route("/users/roles/assign", web::post().to(assign_role))
            .route("/users/roles/remove", web::post().to(remove_role))
    );
}
