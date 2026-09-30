//! Session management handlers
//!
//! Provides HTTP endpoints for:
//! - Logout (revoke current session)
//! - Session validation (check if session is valid)
//! - Session activity (touch session timestamp)

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::app::ServiceFactory;
use crate::server::extractors::auth::{extract_session_token, get_authenticated_user};

/// Response for session operations
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl SessionResponse {
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

/// Session info response
#[derive(Debug, Serialize)]
pub struct SessionInfo {
    pub user_id: String,
    pub role: String,
    pub has_mfa: bool,
    pub auth_time: Option<u64>,
    pub is_valid: bool,
}

/// Logout request (optional, for revoking all sessions)
#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    /// If true, revoke all sessions for this user
    #[serde(default)]
    pub all_sessions: bool,
}

/// Logout endpoint - revokes the current session
///
/// POST /api/v1/auth/logout
///
/// This endpoint:
/// 1. Validates the current session
/// 2. Adds the token to the blacklist
/// 3. Marks the session as revoked
pub async fn logout(
    req: HttpRequest,
    body: Option<web::Json<LogoutRequest>>,
    factory: web::Data<ServiceFactory>,
) -> HttpResponse {
    // Get authenticated user
    let user = match get_authenticated_user(&req) {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Unauthorized().json(SessionResponse::error("Not authenticated"));
        }
    };

    // Get the token for blacklisting
    let token = match extract_session_token(&req) {
        Some(token) => token,
        None => {
            return HttpResponse::BadRequest()
                .json(SessionResponse::error("No session token found"));
        }
    };

    // Parse user_id
    let user_id = match Uuid::parse_str(&user.id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest()
                .json(SessionResponse::error("Invalid user ID format"));
        }
    };

    // Get client IP for audit
    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|s| s.parse().ok());

    // Check if we should revoke all sessions
    let revoke_all = body.as_ref().map(|b| b.all_sessions).unwrap_or(false);

    if revoke_all {
        // Revoke all sessions for this user
        match factory
            .session_service
            .revoke_all_sessions(user_id, "logout_all", None)
            .await
        {
            Ok(count) => {
                tracing::info!(
                    user_id = %user_id,
                    sessions_revoked = count,
                    "User logged out from all sessions"
                );
                HttpResponse::Ok().json(SessionResponse::success_with_message(format!(
                    "Logged out from {} sessions",
                    count
                )))
            }
            Err(e) => {
                tracing::error!(user_id = %user_id, error = %e, "Failed to revoke all sessions");
                HttpResponse::InternalServerError()
                    .json(SessionResponse::error("Failed to logout from all sessions"))
            }
        }
    } else {
        // Revoke just the current session
        match factory
            .session_service
            .logout(&token, user_id, ip_address)
            .await
        {
            Ok(()) => {
                tracing::info!(user_id = %user_id, "User logged out");
                HttpResponse::Ok().json(SessionResponse::success())
            }
            Err(e) => {
                tracing::error!(user_id = %user_id, error = %e, "Failed to logout");
                HttpResponse::InternalServerError()
                    .json(SessionResponse::error("Failed to logout"))
            }
        }
    }
}

/// Check if the current session is valid (not blacklisted)
///
/// POST /api/v1/auth/validate
///
/// Returns session info if valid, 401 if not.
pub async fn validate_session(
    req: HttpRequest,
    factory: web::Data<ServiceFactory>,
) -> HttpResponse {
    // Get authenticated user
    let user = match get_authenticated_user(&req) {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Unauthorized().json(SessionResponse::error("Not authenticated"));
        }
    };

    // Get the token for blacklist check
    let token = match extract_session_token(&req) {
        Some(token) => token,
        None => {
            return HttpResponse::BadRequest()
                .json(SessionResponse::error("No session token found"));
        }
    };

    // Check if session is still valid (not blacklisted)
    // R3-03: Fail closed on database errors - security over availability
    let is_valid = match factory.session_service.is_session_valid(&token).await {
        Ok(valid) => valid,
        Err(e) => {
            tracing::error!(error = %e, "Session validation unavailable - failing closed");
            return HttpResponse::ServiceUnavailable()
                .json(SessionResponse::error("Auth service unavailable"));
        }
    };

    if !is_valid {
        return HttpResponse::Unauthorized()
            .json(SessionResponse::error("Session has been revoked"));
    }

    // Return session info - extract values before moving strings
    let has_mfa = user.has_mfa();
    let auth_time = user.auth_time();
    let user_id = user.id.clone();
    let role = user.role.clone();
    HttpResponse::Ok().json(SessionInfo {
        user_id,
        role,
        has_mfa,
        auth_time,
        is_valid: true,
    })
}

/// Touch session - update last activity timestamp
///
/// POST /api/v1/auth/touch
///
/// Called periodically to keep session activity updated.
pub async fn touch_session(req: HttpRequest, factory: web::Data<ServiceFactory>) -> HttpResponse {
    // Get authenticated user
    let _user = match get_authenticated_user(&req) {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Unauthorized().json(SessionResponse::error("Not authenticated"));
        }
    };

    // Get the token
    let token = match extract_session_token(&req) {
        Some(token) => token,
        None => {
            return HttpResponse::BadRequest()
                .json(SessionResponse::error("No session token found"));
        }
    };

    // Update activity
    match factory.session_service.touch_session(&token).await {
        Ok(()) => HttpResponse::Ok().json(SessionResponse::success()),
        Err(e) => {
            tracing::warn!(error = %e, "Failed to touch session");
            // Don't fail the request, just log
            HttpResponse::Ok().json(SessionResponse::success())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_response_success() {
        let response = SessionResponse::success();
        assert!(response.success);
        assert!(response.message.is_none());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_session_response_error() {
        let response = SessionResponse::error("Test error");
        assert!(!response.success);
        assert!(response.message.is_none());
        assert_eq!(response.error, Some("Test error".to_string()));
    }
}
