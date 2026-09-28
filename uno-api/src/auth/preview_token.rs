//! Preview token generation and validation.
//!
//! Preview tokens allow admin users to view draft content through the actual
//! content pages without publishing. The token encodes the content ID, schema,
//! and expiry time with an HMAC signature for security.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::error::AuthError;

type HmacSha256 = Hmac<Sha256>;

/// Default preview token validity duration (1 hour)
pub const DEFAULT_PREVIEW_DURATION_SECS: i64 = 3600;

/// Preview token payload - the data encoded in the token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewTokenPayload {
    /// Content item ID (UUID string)
    pub content_id: String,
    /// Schema ID (e.g., "guide", "task", "faq")
    pub schema_id: String,
    /// Token expiry timestamp (Unix timestamp)
    pub expires_at: i64,
    /// Optional specific version to preview (None = latest draft)
    pub version: Option<i32>,
}

impl PreviewTokenPayload {
    /// Create a new preview token payload
    pub fn new(content_id: &str, schema_id: &str) -> Self {
        let expires_at = Utc::now().timestamp() + DEFAULT_PREVIEW_DURATION_SECS;
        Self {
            content_id: content_id.to_string(),
            schema_id: schema_id.to_string(),
            expires_at,
            version: None,
        }
    }

    /// Create a payload with custom expiry duration
    pub fn with_duration(content_id: &str, schema_id: &str, duration: Duration) -> Self {
        let expires_at = Utc::now().timestamp() + duration.num_seconds();
        Self {
            content_id: content_id.to_string(),
            schema_id: schema_id.to_string(),
            expires_at,
            version: None,
        }
    }

    /// Set a specific version to preview
    pub fn with_version(mut self, version: i32) -> Self {
        self.version = Some(version);
        self
    }

    /// Check if the token has expired
    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.expires_at
    }

    /// Get expiry as DateTime
    pub fn expires_at_datetime(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.expires_at, 0)
    }
}

/// Generate a preview token
///
/// The token format is: base64(json_payload).signature
/// where signature is HMAC-SHA256 of the base64 payload
pub fn generate_preview_token(payload: &PreviewTokenPayload, secret_key: &[u8]) -> String {
    // Serialize payload to JSON
    let json = serde_json::to_string(payload).expect("Failed to serialize payload");

    // Encode as base64
    let encoded = URL_SAFE_NO_PAD.encode(json.as_bytes());

    // Generate HMAC signature of the encoded payload
    let mut mac = HmacSha256::new_from_slice(secret_key).expect("HMAC can take key of any size");
    mac.update(encoded.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    // Return token as: encoded_payload.signature
    format!("{}.{}", encoded, signature)
}

/// Validate and decode a preview token
///
/// Returns the payload if the token is valid and not expired
pub fn validate_preview_token(
    token: &str,
    secret_key: &[u8],
) -> Result<PreviewTokenPayload, AuthError> {
    // Split token into payload and signature
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 2 {
        return Err(AuthError::MalformedAuth("Invalid token format".into()));
    }

    let encoded_payload = parts[0];
    let provided_signature = parts[1];

    // Verify signature
    let mut mac = HmacSha256::new_from_slice(secret_key).expect("HMAC can take key of any size");
    mac.update(encoded_payload.as_bytes());
    let expected_signature = hex::encode(mac.finalize().into_bytes());

    // Constant-time comparison
    if !constant_time_eq(expected_signature.as_bytes(), provided_signature.as_bytes()) {
        return Err(AuthError::InvalidSignature);
    }

    // Decode payload
    let json_bytes = URL_SAFE_NO_PAD
        .decode(encoded_payload)
        .map_err(|_| AuthError::MalformedAuth("Invalid base64 encoding".into()))?;

    let payload: PreviewTokenPayload = serde_json::from_slice(&json_bytes)
        .map_err(|_| AuthError::MalformedAuth("Invalid JSON payload".into()))?;

    // Check expiry
    if payload.is_expired() {
        return Err(AuthError::TimestampExpired);
    }

    Ok(payload)
}

/// Constant-time byte comparison to prevent timing attacks
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &[u8] = b"test-preview-secret-key-12345";

    #[test]
    fn test_generate_and_validate_token() {
        let payload = PreviewTokenPayload::new("content-123", "guide");
        let token = generate_preview_token(&payload, TEST_SECRET);

        // Token should have two parts
        assert!(token.contains('.'));

        // Should validate successfully
        let validated = validate_preview_token(&token, TEST_SECRET).unwrap();
        assert_eq!(validated.content_id, "content-123");
        assert_eq!(validated.schema_id, "guide");
        assert!(!validated.is_expired());
    }

    #[test]
    fn test_token_with_version() {
        let payload = PreviewTokenPayload::new("content-123", "guide").with_version(5);
        let token = generate_preview_token(&payload, TEST_SECRET);

        let validated = validate_preview_token(&token, TEST_SECRET).unwrap();
        assert_eq!(validated.version, Some(5));
    }

    #[test]
    fn test_expired_token() {
        let mut payload = PreviewTokenPayload::new("content-123", "guide");
        payload.expires_at = Utc::now().timestamp() - 100; // Expired 100 seconds ago

        let token = generate_preview_token(&payload, TEST_SECRET);

        let result = validate_preview_token(&token, TEST_SECRET);
        assert!(matches!(result, Err(AuthError::TimestampExpired)));
    }

    #[test]
    fn test_invalid_signature() {
        let payload = PreviewTokenPayload::new("content-123", "guide");
        let token = generate_preview_token(&payload, TEST_SECRET);

        // Try to validate with wrong secret
        let result = validate_preview_token(&token, b"wrong-secret");
        assert!(matches!(result, Err(AuthError::InvalidSignature)));
    }

    #[test]
    fn test_tampered_token() {
        let payload = PreviewTokenPayload::new("content-123", "guide");
        let token = generate_preview_token(&payload, TEST_SECRET);

        // Tamper with the payload
        let parts: Vec<&str> = token.split('.').collect();
        let tampered_token = format!("tampered{}.{}", parts[0], parts[1]);

        let result = validate_preview_token(&tampered_token, TEST_SECRET);
        assert!(matches!(result, Err(AuthError::InvalidSignature)));
    }

    #[test]
    fn test_malformed_token() {
        let result = validate_preview_token("not-a-valid-token", TEST_SECRET);
        assert!(matches!(result, Err(AuthError::MalformedAuth(_))));
    }
}
