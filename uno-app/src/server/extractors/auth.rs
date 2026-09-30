//! Authentication for Leptos functions uses the shared verified session boundary.
//!
//! This module provides:
//! - `get_authenticated_user()` - Extract authenticated user from request
//! - `extract_session_token()` - Extract raw JWT token for session operations
//! - `require_auth!()` macro - Require authentication in server functions
//! - `require_admin!()` macro - Require operator permission in server functions

use actix_web::http::header;
use actix_web::HttpRequest;
use uno_api::auth::session::Principal;
pub use uno_api::auth::session::{Permission, SessionError as AuthError};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
    pub role: String,
    principal: Principal,
    /// The raw JWT token (for session operations like logout)
    token: Option<String>,
}

impl AuthenticatedUser {
    pub fn require(&self, permission: Permission) -> Result<(), AuthError> {
        self.principal.require(permission)
    }

    /// Get the raw JWT token for session operations (e.g., logout, revocation)
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// Get the principal's auth time (for freshness checks)
    pub fn auth_time(&self) -> Option<u64> {
        self.principal.auth_time
    }

    /// Check if the session has MFA verification
    pub fn has_mfa(&self) -> bool {
        self.principal.amr.iter().any(|m| m == "mfa")
    }
}

/// Extract the raw JWT session token from the request.
///
/// This is useful for session operations like logout/revocation.
/// Returns None if no token is present.
pub fn extract_session_token(req: &HttpRequest) -> Option<String> {
    // Try Authorization header first
    if let Some(value) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(value) = value.to_str() {
            if let Some(token) = value.strip_prefix("Bearer ") {
                let token = token.trim();
                if !token.is_empty() && !token.contains(char::is_whitespace) {
                    return Some(token.to_string());
                }
            }
        }
    }

    // Fall back to cookie
    if let Some(cookie) = req.cookie("uno_session") {
        return Some(cookie.value().to_string());
    }

    None
}

pub fn get_authenticated_user(req: &HttpRequest) -> Result<AuthenticatedUser, AuthError> {
    // Extract token for session operations
    let token = extract_session_token(req);

    // Verify and get principal
    let principal = uno_api::auth::web::authenticate(req)?;

    Ok(AuthenticatedUser {
        id: principal.sub.clone(),
        role: principal.role.clone(),
        principal,
        token,
    })
}

/// Get an authenticated user with session blacklist verification.
///
/// This function performs both JWT validation and checks that the session
/// has not been revoked/blacklisted. Use this for handlers that perform
/// sensitive operations where session revocation must be enforced immediately.
///
/// # Arguments
/// * `req` - The HTTP request containing the session token
/// * `session_service` - The session service for blacklist checking
///
/// # Returns
/// * `Ok(AuthenticatedUser)` - The authenticated user with valid session
/// * `Err(AppError::Unauthorized)` - If not authenticated or session is revoked
pub async fn get_verified_user(
    req: &actix_web::HttpRequest,
    session_service: &crate::server::services::SessionService,
) -> Result<AuthenticatedUser, crate::types::AppError> {
    // First, validate JWT and get authenticated user
    let user = get_authenticated_user(req)
        .map_err(|e| crate::types::AppError::Unauthorized(e.to_string()))?;

    // Check session blacklist for revoked tokens
    // R3-03: Fail closed on database errors for security
    if let Some(token) = user.token() {
        match session_service.is_session_valid(token).await {
            Ok(false) => {
                return Err(crate::types::AppError::Unauthorized(
                    "Session has been revoked".to_string(),
                ));
            }
            Err(e) => {
                // R3-03: Fail closed - service unavailable is safer than fail open
                tracing::error!(error = %e, "Session validation unavailable - failing closed");
                return Err(crate::types::AppError::ServiceUnavailable(
                    "Auth service unavailable".to_string(),
                ));
            }
            Ok(true) => {}
        }
    }

    Ok(user)
}

/// Get the authenticated actor ID for audit trails.
///
/// R4-02: This replaces the insecure X-Admin-Id header pattern.
/// Actor ID is always derived from the verified JWT token, never from headers.
pub fn get_actor_id(req: &actix_web::HttpRequest) -> Result<String, crate::types::AppError> {
    let user = get_authenticated_user(req)
        .map_err(|e| crate::types::AppError::Unauthorized(e.to_string()))?;
    Ok(user.id)
}

/// Get the authenticated actor ID, falling back to "system" for internal operations.
///
/// R4-02: Use this only for internal/scheduled operations where no user context exists.
/// For user-initiated operations, always use `get_actor_id()` which requires authentication.
pub fn get_actor_id_or_system(req: &actix_web::HttpRequest) -> String {
    get_actor_id(req).unwrap_or_else(|_| "system".to_string())
}

#[macro_export]
macro_rules! require_auth {
    () => {{
        let req: actix_web::HttpRequest = leptos_actix::extract().await?;
        $crate::server::extractors::auth::get_authenticated_user(&req)
            .map_err(|e| leptos::prelude::ServerFnError::new(e.to_string()))?
    }};
}

#[macro_export]
macro_rules! require_admin {
    () => {{
        let user = $crate::require_auth!();
        user.require($crate::server::extractors::auth::Permission::Operator)
            .map_err(|e| leptos::prelude::ServerFnError::new(e.to_string()))?;
        user
    }};
}
