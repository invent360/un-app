//! RBAC permission checking utilities

use actix_web::HttpResponse;
use crate::server::app::ServiceFactory;

/// Check if user has required permission, returning an error response if not
pub async fn check_permission(
    factory: &ServiceFactory,
    user_id: &str,
    permission: &str,
) -> Result<(), HttpResponse> {
    match factory.rbac_service.check_permission(user_id, permission).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(HttpResponse::Forbidden().json(serde_json::json!({
            "error": format!("Missing required permission: {}", permission),
            "code": "FORBIDDEN",
            "required_permission": permission
        }))),
        Err(e) => Err(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string(),
            "code": "INTERNAL_ERROR"
        }))),
    }
}

/// Macro for easy permission checks in handlers
/// Usage: require_permission!(factory, user_id, "content:publish");
#[macro_export]
macro_rules! require_permission {
    ($factory:expr, $user_id:expr, $permission:expr) => {
        if let Err(resp) = $crate::server::middleware::rbac_middleware::check_permission(
            $factory, $user_id, $permission
        ).await {
            return resp;
        }
    };
}
