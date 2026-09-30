//! Consent and privacy management handlers
//!
//! Provides HTTP endpoints for:
//! - Viewing required consents
//! - Recording consent decisions
//! - Withdrawing consent
//! - Data access requests (GDPR)

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::app::ServiceFactory;
use crate::server::extractors::auth::{get_authenticated_user, get_verified_user};

/// Response for consent operations
#[derive(Debug, Serialize)]
pub struct ConsentResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ConsentResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: None,
            error: None,
        }
    }

    pub fn success_with_message(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: None,
            error: Some(message.into()),
        }
    }
}

/// Request to record consent
#[derive(Debug, Deserialize)]
pub struct RecordConsentRequest {
    pub consent_type: String,
    pub consented: bool,
    #[serde(default = "default_consent_method")]
    pub consent_method: String,
}

fn default_consent_method() -> String {
    "explicit_click".to_string()
}

/// Request to withdraw consent
#[derive(Debug, Deserialize)]
pub struct WithdrawConsentRequest {
    pub consent_type: String,
    pub reason: Option<String>,
}

/// Request to create a data access request
#[derive(Debug, Deserialize)]
pub struct CreateDataRequestRequest {
    pub request_type: String,
    pub reason: Option<String>,
}

/// Get all required consent versions
///
/// GET /api/v1/consent/required
///
/// Returns list of all active consent versions users need to agree to.
pub async fn get_required_consents(factory: web::Data<ServiceFactory>) -> HttpResponse {
    match factory.consent_service.get_required_consents().await {
        Ok(consents) => HttpResponse::Ok().json(consents),
        Err(e) => {
            tracing::error!(error = %e, "Failed to get required consents");
            HttpResponse::InternalServerError().json(ConsentResponse::error("Failed to get consents"))
        }
    }
}

/// Get user's current consent status
///
/// GET /api/v1/consent/status
///
/// Returns the user's current consent status for all consent types.
pub async fn get_consent_status(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
) -> HttpResponse {
    let user = match get_authenticated_user(&req) {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Unauthorized().json(ConsentResponse::error("Not authenticated"));
        }
    };

    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ConsentResponse::error("Invalid user ID"));
        }
    };

    match factory.consent_service.check_required_consents(user_id).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => {
            tracing::error!(user_id = %user_id, error = %e, "Failed to get consent status");
            HttpResponse::InternalServerError()
                .json(ConsentResponse::error("Failed to get consent status"))
        }
    }
}

/// Record a consent decision
///
/// POST /api/v1/consent/record
///
/// Records the user's consent decision for a specific consent type.
/// Uses session blacklist checking to ensure revoked sessions are rejected.
pub async fn record_consent(
    req: HttpRequest,
    body: web::Json<RecordConsentRequest>,
    factory: web::Data<ServiceFactory>,
) -> HttpResponse {
    // Use get_verified_user for mutation operations - checks blacklist
    let user = match get_verified_user(&req, &factory.session_service).await {
        Ok(user) => user,
        Err(e) => {
            return HttpResponse::Unauthorized().json(ConsentResponse::error(e.to_string()));
        }
    };

    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ConsentResponse::error("Invalid user ID"));
        }
    };

    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|s| s.parse().ok());

    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match factory
        .consent_service
        .record_consent(
            user_id,
            &body.consent_type,
            body.consented,
            &body.consent_method,
            ip_address,
            user_agent,
            None, // session_id
        )
        .await
    {
        Ok(consent) => {
            tracing::info!(
                user_id = %user_id,
                consent_type = %body.consent_type,
                consented = %body.consented,
                "Consent recorded"
            );
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "consent": consent,
            }))
        }
        Err(e) => {
            tracing::warn!(
                user_id = %user_id,
                consent_type = %body.consent_type,
                error = %e,
                "Failed to record consent"
            );
            HttpResponse::BadRequest().json(ConsentResponse::error(e.to_string()))
        }
    }
}

/// Withdraw consent for a specific type
///
/// POST /api/v1/consent/withdraw
///
/// Withdraws consent for a non-mandatory consent type.
/// Uses session blacklist checking to ensure revoked sessions are rejected.
pub async fn withdraw_consent(
    req: HttpRequest,
    body: web::Json<WithdrawConsentRequest>,
    factory: web::Data<ServiceFactory>,
) -> HttpResponse {
    // Use get_verified_user for mutation operations - checks blacklist
    let user = match get_verified_user(&req, &factory.session_service).await {
        Ok(user) => user,
        Err(e) => {
            return HttpResponse::Unauthorized().json(ConsentResponse::error(e.to_string()));
        }
    };

    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ConsentResponse::error("Invalid user ID"));
        }
    };

    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|s| s.parse().ok());

    match factory
        .consent_service
        .withdraw_consent(
            user_id,
            &body.consent_type,
            body.reason.as_deref(),
            ip_address,
        )
        .await
    {
        Ok(withdrawn) => {
            if withdrawn {
                tracing::info!(
                    user_id = %user_id,
                    consent_type = %body.consent_type,
                    "Consent withdrawn"
                );
                HttpResponse::Ok().json(ConsentResponse::success_with_message("Consent withdrawn"))
            } else {
                HttpResponse::NotFound()
                    .json(ConsentResponse::error("No active consent found to withdraw"))
            }
        }
        Err(e) => {
            tracing::warn!(
                user_id = %user_id,
                consent_type = %body.consent_type,
                error = %e,
                "Failed to withdraw consent"
            );
            HttpResponse::BadRequest().json(ConsentResponse::error(e.to_string()))
        }
    }
}

/// Create a data access request (GDPR)
///
/// POST /api/v1/privacy/request
///
/// Creates a new data access request (export, delete, rectify, restrict).
/// Uses session blacklist checking to ensure revoked sessions are rejected.
pub async fn create_data_request(
    req: HttpRequest,
    body: web::Json<CreateDataRequestRequest>,
    factory: web::Data<ServiceFactory>,
) -> HttpResponse {
    // Use get_verified_user for mutation operations - checks blacklist
    let user = match get_verified_user(&req, &factory.session_service).await {
        Ok(user) => user,
        Err(e) => {
            return HttpResponse::Unauthorized().json(ConsentResponse::error(e.to_string()));
        }
    };

    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ConsentResponse::error("Invalid user ID"));
        }
    };

    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|s| s.parse().ok());

    match factory
        .consent_service
        .create_data_request(
            user_id,
            &body.request_type,
            body.reason.as_deref(),
            ip_address,
        )
        .await
    {
        Ok(request) => {
            tracing::info!(
                user_id = %user_id,
                request_id = %request.id,
                request_type = %body.request_type,
                "Data access request created"
            );
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "request": request,
            }))
        }
        Err(e) => {
            tracing::warn!(
                user_id = %user_id,
                request_type = %body.request_type,
                error = %e,
                "Failed to create data request"
            );
            HttpResponse::BadRequest().json(ConsentResponse::error(e.to_string()))
        }
    }
}

/// Get user's data access requests
///
/// GET /api/v1/privacy/requests
///
/// Returns all data access requests for the current user.
pub async fn get_data_requests(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
) -> HttpResponse {
    let user = match get_authenticated_user(&req) {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Unauthorized().json(ConsentResponse::error("Not authenticated"));
        }
    };

    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ConsentResponse::error("Invalid user ID"));
        }
    };

    match factory.consent_service.get_user_data_requests(user_id).await {
        Ok(requests) => HttpResponse::Ok().json(requests),
        Err(e) => {
            tracing::error!(user_id = %user_id, error = %e, "Failed to get data requests");
            HttpResponse::InternalServerError()
                .json(ConsentResponse::error("Failed to get data requests"))
        }
    }
}

/// Get data retention policies
///
/// GET /api/v1/privacy/retention-policies
///
/// Returns all data retention policies (public information).
pub async fn get_retention_policies(factory: web::Data<ServiceFactory>) -> HttpResponse {
    match factory.consent_service.get_retention_policies().await {
        Ok(policies) => HttpResponse::Ok().json(policies),
        Err(e) => {
            tracing::error!(error = %e, "Failed to get retention policies");
            HttpResponse::InternalServerError()
                .json(ConsentResponse::error("Failed to get retention policies"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consent_response_success() {
        let response = ConsentResponse::success();
        assert!(response.success);
        assert!(response.message.is_none());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_consent_response_error() {
        let response = ConsentResponse::error("Test error");
        assert!(!response.success);
        assert!(response.message.is_none());
        assert_eq!(response.error, Some("Test error".to_string()));
    }

    #[test]
    fn test_default_consent_method() {
        assert_eq!(default_consent_method(), "explicit_click");
    }
}
