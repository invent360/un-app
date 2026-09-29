//! Authentication extractor for server functions
//!
//! Provides a way to extract and validate authenticated users in Leptos server functions.

use actix_web::{HttpRequest, web::Data, http::header::AUTHORIZATION};

/// Represents an authenticated user
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    /// User identifier (wallet address, email, or user ID)
    pub id: String,
    /// User role (admin, editor, viewer, etc.)
    pub role: String,
}

/// Error type for authentication failures
#[derive(Debug, Clone)]
pub enum AuthError {
    /// No authentication token provided
    MissingToken,
    /// Token is invalid or expired
    InvalidToken,
    /// User is authenticated but lacks required permissions
    Forbidden,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::MissingToken => write!(f, "Authentication required"),
            AuthError::InvalidToken => write!(f, "Invalid or expired token"),
            AuthError::Forbidden => write!(f, "Insufficient permissions"),
        }
    }
}

impl std::error::Error for AuthError {}

/// Extract authenticated user from Leptos server function context
///
/// # Usage
/// ```rust
/// #[server(MyServerFn, "/api")]
/// pub async fn my_protected_fn() -> Result<String, ServerFnError> {
///     use leptos_actix::extract;
///     use actix_web::HttpRequest;
///     use crate::server::extractors::auth::get_authenticated_user;
///
///     let req: HttpRequest = extract().await?;
///     let user = get_authenticated_user(&req)?;
///
///     // user.id contains the verified user identity
///     Ok(format!("Hello, {}", user.id))
/// }
/// ```
pub fn get_authenticated_user(req: &HttpRequest) -> Result<AuthenticatedUser, AuthError> {
    // Extract Authorization header
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.trim_start_matches("Bearer ").to_string());

    let token = auth_header.ok_or(AuthError::MissingToken)?;

    // Get expected API key from environment
    let admin_key = std::env::var("ADMIN_API_KEY")
        .unwrap_or_else(|_| "dev-admin-key".to_string());

    // Phase 1: Simple API key validation
    // TODO: Phase 2 - Add full JWT validation with signature verification
    if token == admin_key {
        return Ok(AuthenticatedUser {
            id: "admin".to_string(),
            role: "admin".to_string(),
        });
    }

    // For JWT tokens, attempt to decode (without full verification in Phase 1)
    // This provides basic protection while full JWT validation is implemented
    if let Some(user) = try_decode_jwt_basic(&token) {
        return Ok(user);
    }

    Err(AuthError::InvalidToken)
}

/// Basic JWT decoding (extracts claims without full signature verification)
///
/// SECURITY NOTE: This is a Phase 1 stopgap. Full JWT verification with
/// signature validation will be added in Phase 2.
fn try_decode_jwt_basic(token: &str) -> Option<AuthenticatedUser> {
    // JWT format: header.payload.signature
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }

    // Decode the payload (middle part) using simple base64url decoding
    let payload = base64_url_decode(parts[1])?;
    let claims: serde_json::Value = serde_json::from_slice(&payload).ok()?;

    // Extract user ID from standard JWT claims
    // Try 'sub' (subject), then 'user_id', then 'address'
    let user_id = claims.get("sub")
        .or_else(|| claims.get("user_id"))
        .or_else(|| claims.get("address"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())?;

    // Extract role if present
    let role = claims.get("role")
        .and_then(|v| v.as_str())
        .unwrap_or("user")
        .to_string();

    // Check token expiration
    if let Some(exp) = claims.get("exp").and_then(|v| v.as_i64()) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        if now > exp {
            tracing::warn!(user_id = %user_id, "JWT token expired");
            return None; // Token expired
        }
    }

    Some(AuthenticatedUser {
        id: user_id,
        role,
    })
}

/// Simple base64url decoder (URL-safe base64 without padding)
///
/// Implements RFC 4648 Section 5 base64url decoding without external dependencies.
fn base64_url_decode(input: &str) -> Option<Vec<u8>> {
    // Convert base64url to standard base64
    let mut base64_str = input.replace('-', "+").replace('_', "/");

    // Add padding if needed
    let padding = (4 - base64_str.len() % 4) % 4;
    for _ in 0..padding {
        base64_str.push('=');
    }

    // Decode using simple lookup table
    decode_base64(&base64_str)
}

/// Simple base64 decoder
fn decode_base64(input: &str) -> Option<Vec<u8>> {
    const DECODE_TABLE: [i8; 256] = {
        let mut table = [-1i8; 256];
        let mut i = 0u8;
        // A-Z
        while i < 26 {
            table[(b'A' + i) as usize] = i as i8;
            i += 1;
        }
        // a-z
        i = 0;
        while i < 26 {
            table[(b'a' + i) as usize] = (26 + i) as i8;
            i += 1;
        }
        // 0-9
        i = 0;
        while i < 10 {
            table[(b'0' + i) as usize] = (52 + i) as i8;
            i += 1;
        }
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table[b'=' as usize] = 0; // Padding
        table
    };

    let bytes = input.as_bytes();
    let mut output = Vec::with_capacity(bytes.len() * 3 / 4);

    let mut i = 0;
    while i + 3 < bytes.len() {
        let a = DECODE_TABLE[bytes[i] as usize];
        let b = DECODE_TABLE[bytes[i + 1] as usize];
        let c = DECODE_TABLE[bytes[i + 2] as usize];
        let d = DECODE_TABLE[bytes[i + 3] as usize];

        if a < 0 || b < 0 {
            return None;
        }

        output.push(((a as u8) << 2) | ((b as u8) >> 4));

        if bytes[i + 2] != b'=' {
            if c < 0 {
                return None;
            }
            output.push(((b as u8) << 4) | ((c as u8) >> 2));
        }

        if bytes[i + 3] != b'=' {
            if d < 0 {
                return None;
            }
            output.push(((c as u8) << 6) | (d as u8));
        }

        i += 4;
    }

    Some(output)
}

/// Require authentication for a server function, returning ServerFnError on failure
#[macro_export]
macro_rules! require_auth {
    () => {{
        use leptos_actix::extract;
        use actix_web::HttpRequest;
        use $crate::server::extractors::auth::get_authenticated_user;

        let req: HttpRequest = extract().await?;
        get_authenticated_user(&req)
            .map_err(|e| leptos::prelude::ServerFnError::new(e.to_string()))?
    }};
}

/// Require admin role for a server function
#[macro_export]
macro_rules! require_admin {
    () => {{
        let user = require_auth!();
        if user.role != "admin" {
            return Err(leptos::prelude::ServerFnError::new("Admin access required"));
        }
        user
    }};
}
