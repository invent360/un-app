//! Authentication module for uno-api.
//!
//! Provides HMAC-based request signing and verification for secure
//! communication between admin clients and the API server.
//!
//! # Security Features
//!
//! - HMAC-SHA256 request signing
//! - Timestamp-based replay protection
//! - Nonce for additional uniqueness
//! - Client credential management
//! - R5-04: Method/path binding to prevent endpoint replay attacks
//!
//! # Usage
//!
//! ## Signing Requests (Client Side)
//!
//! ```ignore
//! use uno_api::auth::{sign_request, SignedRequest};
//!
//! let payload = PublishLicensesRequest::new(licenses);
//! // R5-04: Include method and path in signature
//! let signed = sign_request(
//!     "client_id",
//!     b"secret_key",
//!     "POST",
//!     "/api/v1/licenses/publish",
//!     &payload,
//! );
//! // Send `signed` to the server
//! ```
//!
//! ## Verifying Requests (Server Side)
//!
//! ```ignore
//! use uno_api::auth::{verify_request_with_binding, ClientRegistry};
//!
//! let registry = ClientRegistry::new();
//! registry.register("client_id", b"secret_key", &["admin"]);
//!
//! // In request handler - verify method/path matches what was signed
//! verify_request_with_binding(
//!     &signed_request,
//!     &registry,
//!     "POST",                          // actual HTTP method
//!     "/api/v1/licenses/publish",      // actual request path
//!     300,                             // max age in seconds
//! )?;
//! ```

mod client_registry;
mod hmac;
mod nonce_registry;
mod preview_token;
pub mod session;
mod signed_request;
#[cfg(feature = "web-auth")]
pub mod web;

pub use client_registry::{ClientCredentials, ClientRegistry};
pub use hmac::{
    sign_payload, sign_payload_with_binding, verify_signature, verify_signature_with_binding,
    verify_with_replay_protection, verify_with_replay_protection_async,
    verify_with_replay_protection_binding, verify_with_replay_protection_binding_async,
    AsyncNonceChecker,
};
pub use nonce_registry::NonceRegistry;
pub use preview_token::{
    generate_preview_token, validate_preview_token, PreviewTokenPayload,
    DEFAULT_PREVIEW_DURATION_SECS,
};
pub use signed_request::SignedRequest;

/// Sign a request with HMAC-SHA256 including method/path binding.
///
/// R5-04: Creates a signed request wrapper with timestamp, nonce, signature,
/// and method/path binding to prevent replay attacks to different endpoints.
///
/// This is the recommended signing function for new code.
pub fn sign_request<T: serde::Serialize>(
    client_id: &str,
    secret_key: &[u8],
    method: &str,
    path: &str,
    payload: &T,
) -> SignedRequest<T>
where
    T: Clone,
{
    SignedRequest::sign(client_id, secret_key, method, path, payload.clone())
}

/// Verify a signed request (legacy, no method/path binding).
///
/// DEPRECATED: Use `verify_request_with_binding` for new code.
///
/// Checks timestamp freshness and signature validity.
/// Returns the client credentials if verification succeeds.
///
/// Note: This does not include nonce-based replay protection.
/// For production use, prefer `verify_request_with_replay_protection_binding`.
pub fn verify_request<T: serde::Serialize>(
    request: &SignedRequest<T>,
    registry: &ClientRegistry,
    max_age_secs: i64,
) -> Result<ClientCredentials, crate::error::AuthError> {
    // Get client credentials
    let credentials = registry.get(&request.client_id)?;

    // Verify the signature
    hmac::verify_signature(request, &credentials.secret_key, max_age_secs)?;

    Ok(credentials)
}

/// Verify a signed request with method/path binding.
///
/// R5-04: This is the recommended verification function. It ensures the
/// signature is bound to the specific HTTP method and path, preventing
/// replay attacks to different endpoints.
///
/// The `actual_method` and `actual_path` parameters are the method and path
/// from the incoming HTTP request, NOT from the signed request body.
///
/// Note: This does not include nonce-based replay protection.
/// For production use, prefer `verify_request_with_replay_protection_binding`.
pub fn verify_request_with_binding<T: serde::Serialize>(
    request: &SignedRequest<T>,
    registry: &ClientRegistry,
    actual_method: &str,
    actual_path: &str,
    max_age_secs: i64,
) -> Result<ClientCredentials, crate::error::AuthError> {
    // Get client credentials
    let credentials = registry.get(&request.client_id)?;

    // Verify the signature with method/path binding
    hmac::verify_signature_with_binding(
        request,
        &credentials.secret_key,
        actual_method,
        actual_path,
        max_age_secs,
    )?;

    Ok(credentials)
}

/// Verify a signed request with full replay protection (sync version, legacy).
///
/// DEPRECATED: Use `verify_request_with_replay_protection_binding` for new code.
///
/// NOTE: This uses in-memory nonce tracking which doesn't work across replicas.
/// For multi-replica deployments, use `verify_request_with_replay_protection_async`.
///
/// Returns the client credentials if verification succeeds.
pub fn verify_request_with_replay_protection<T: serde::Serialize>(
    request: &SignedRequest<T>,
    client_registry: &ClientRegistry,
    nonce_registry: &NonceRegistry,
    max_age_secs: i64,
) -> Result<ClientCredentials, crate::error::AuthError> {
    // Get client credentials
    let credentials = client_registry.get(&request.client_id)?;

    // Verify signature with replay protection (timestamp + nonce)
    hmac::verify_with_replay_protection(
        request,
        &credentials.secret_key,
        max_age_secs,
        nonce_registry,
    )?;

    Ok(credentials)
}

/// Verify a signed request with method/path binding and full replay protection (sync).
///
/// R5-04: This is the recommended verification function for production use.
/// It combines signature verification with method/path binding and nonce tracking.
///
/// NOTE: This uses in-memory nonce tracking which doesn't work across replicas.
/// For multi-replica deployments, use `verify_request_with_replay_protection_binding_async`.
pub fn verify_request_with_replay_protection_binding<T: serde::Serialize>(
    request: &SignedRequest<T>,
    client_registry: &ClientRegistry,
    actual_method: &str,
    actual_path: &str,
    nonce_registry: &NonceRegistry,
    max_age_secs: i64,
) -> Result<ClientCredentials, crate::error::AuthError> {
    // Get client credentials
    let credentials = client_registry.get(&request.client_id)?;

    // Verify signature with method/path binding and replay protection
    hmac::verify_with_replay_protection_binding(
        request,
        &credentials.secret_key,
        actual_method,
        actual_path,
        max_age_secs,
        nonce_registry,
    )?;

    Ok(credentials)
}

/// Verify a signed request with full replay protection (async version, legacy).
///
/// DEPRECATED: Use `verify_request_with_replay_protection_binding_async` for new code.
///
/// R3-04: This supports multi-replica deployments using an async nonce checker
/// that can be backed by PostgreSQL or other distributed stores.
///
/// Returns the client credentials if verification succeeds.
pub async fn verify_request_with_replay_protection_async<T: serde::Serialize>(
    request: &SignedRequest<T>,
    client_registry: &ClientRegistry,
    nonce_checker: &dyn AsyncNonceChecker,
    max_age_secs: i64,
) -> Result<ClientCredentials, crate::error::AuthError> {
    // Get client credentials
    let credentials = client_registry.get(&request.client_id)?;

    // Verify signature with replay protection (timestamp + nonce) - async
    hmac::verify_with_replay_protection_async(
        request,
        &credentials.secret_key,
        max_age_secs,
        nonce_checker,
    )
    .await?;

    Ok(credentials)
}

/// Verify a signed request with method/path binding and full replay protection (async).
///
/// R5-04: This is the recommended verification function for production use
/// in multi-replica deployments. It combines signature verification with
/// method/path binding and async nonce checking.
///
/// The `actual_method` and `actual_path` parameters are from the incoming
/// HTTP request, NOT from the signed request body.
pub async fn verify_request_with_replay_protection_binding_async<T: serde::Serialize>(
    request: &SignedRequest<T>,
    client_registry: &ClientRegistry,
    actual_method: &str,
    actual_path: &str,
    nonce_checker: &dyn AsyncNonceChecker,
    max_age_secs: i64,
) -> Result<ClientCredentials, crate::error::AuthError> {
    // Get client credentials
    let credentials = client_registry.get(&request.client_id)?;

    // Verify signature with method/path binding and replay protection - async
    hmac::verify_with_replay_protection_binding_async(
        request,
        &credentials.secret_key,
        actual_method,
        actual_path,
        max_age_secs,
        nonce_checker,
    )
    .await?;

    Ok(credentials)
}
