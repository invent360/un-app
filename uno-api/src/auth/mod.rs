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
//!
//! # Usage
//!
//! ## Signing Requests (Client Side)
//!
//! ```ignore
//! use uno_api::auth::{sign_request, SignedRequest};
//!
//! let payload = PublishLicensesRequest::new(licenses);
//! let signed = sign_request(
//!     "client_id",
//!     b"secret_key",
//!     &payload,
//! );
//! // Send `signed` to the server
//! ```
//!
//! ## Verifying Requests (Server Side)
//!
//! ```ignore
//! use uno_api::auth::{verify_request, ClientRegistry};
//!
//! let registry = ClientRegistry::new();
//! registry.register("client_id", b"secret_key", &["admin"]);
//!
//! // In request handler
//! verify_request(&signed_request, &registry, 300)?;
//! ```

mod client_registry;
mod hmac;
mod nonce_registry;
mod preview_token;
mod signed_request;

pub use client_registry::{ClientCredentials, ClientRegistry};
pub use hmac::{sign_payload, verify_signature, verify_with_replay_protection};
pub use nonce_registry::NonceRegistry;
pub use preview_token::{
    generate_preview_token, validate_preview_token, PreviewTokenPayload,
    DEFAULT_PREVIEW_DURATION_SECS,
};
pub use signed_request::SignedRequest;

/// Sign a request with HMAC-SHA256.
///
/// Creates a signed request wrapper with timestamp, nonce, and signature.
pub fn sign_request<T: serde::Serialize>(
    client_id: &str,
    secret_key: &[u8],
    payload: &T,
) -> SignedRequest<T>
where
    T: Clone,
{
    SignedRequest::sign(client_id, secret_key, payload.clone())
}

/// Verify a signed request.
///
/// Checks timestamp freshness and signature validity.
/// Returns the client credentials if verification succeeds.
///
/// Note: This does not include nonce-based replay protection.
/// For production use, prefer `verify_request_with_replay_protection`.
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

/// Verify a signed request with full replay protection.
///
/// This is the recommended verification function for production use.
/// It combines signature verification with nonce tracking to prevent
/// replay attacks even within the timestamp validity window.
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
