//! HMAC signing and verification.

use async_trait::async_trait;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::AuthError;

/// Async trait for nonce consumption in distributed environments.
///
/// R3-04: This trait allows different backends (in-memory, PostgreSQL, Redis)
/// to be used for nonce tracking. For production use with multiple replicas,
/// use a PostgreSQL-backed implementation to ensure cross-replica sharing.
#[async_trait]
pub trait AsyncNonceChecker: Send + Sync {
    /// Atomically consume a nonce, returning false if already consumed.
    ///
    /// This must be atomic: if two requests arrive simultaneously with the
    /// same nonce, exactly one must succeed and one must fail.
    async fn consume_nonce(
        &self,
        client_id: &str,
        nonce: &str,
        timestamp: i64,
    ) -> Result<bool, AuthError>;
}

type HmacSha256 = Hmac<Sha256>;

/// Sign a payload with HMAC-SHA256 (legacy, no method/path binding).
///
/// The signature is computed over: `client_id:timestamp:nonce:payload_json`
///
/// DEPRECATED: Use `sign_payload_with_binding` for new code.
/// This exists for backward compatibility during migration.
pub fn sign_payload<T: serde::Serialize>(
    client_id: &str,
    secret_key: &[u8],
    payload: &T,
    timestamp: i64,
    nonce: &str,
) -> String {
    let payload_json = serde_json::to_string(payload).unwrap_or_default();
    let message = format!("{}:{}:{}:{}", client_id, timestamp, nonce, payload_json);

    let mut mac =
        HmacSha256::new_from_slice(secret_key).expect("HMAC can take key of any size");
    mac.update(message.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

/// Sign a payload with HMAC-SHA256 including method/path binding.
///
/// R5-04: The signature is computed over:
/// `method:path:client_id:timestamp:nonce:payload_json`
///
/// This prevents replay attacks where a signed request body is sent to
/// a different endpoint (e.g., a DELETE signed for /api/v1/user/123
/// replayed to /api/v1/admin/delete-all).
pub fn sign_payload_with_binding<T: serde::Serialize>(
    client_id: &str,
    secret_key: &[u8],
    method: &str,
    path: &str,
    payload: &T,
    timestamp: i64,
    nonce: &str,
) -> String {
    let payload_json = serde_json::to_string(payload).unwrap_or_default();
    // R5-04: Include method and path in signature to bind to specific endpoint
    let message = format!(
        "{}:{}:{}:{}:{}:{}",
        method.to_uppercase(),
        canonicalize_path(path),
        client_id,
        timestamp,
        nonce,
        payload_json
    );

    let mut mac =
        HmacSha256::new_from_slice(secret_key).expect("HMAC can take key of any size");
    mac.update(message.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

/// Canonicalize a path for consistent signature verification.
///
/// R5-04: Ensures path comparisons are consistent:
/// - Removes trailing slashes (except for root "/")
/// - Lowercases the path
/// - Does not include query parameters (those are in the payload)
fn canonicalize_path(path: &str) -> String {
    let path = path.split('?').next().unwrap_or(path);
    let path = path.trim_end_matches('/');
    if path.is_empty() {
        "/".to_string()
    } else {
        path.to_lowercase()
    }
}

/// Maximum allowed clock skew for future timestamps (seconds).
/// Allows small clock drift but rejects obvious attacks.
const MAX_FUTURE_SKEW_SECS: i64 = 5;

/// Verify a signature (legacy, without method/path binding).
///
/// DEPRECATED: Use `verify_signature_with_binding` for new code.
///
/// Returns an error if:
/// - The timestamp is in the future (beyond clock skew tolerance)
/// - The timestamp is too old (replay protection)
/// - The signature doesn't match
pub fn verify_signature<T: serde::Serialize>(
    request: &super::SignedRequest<T>,
    secret_key: &[u8],
    max_age_secs: i64,
) -> Result<(), AuthError> {
    verify_timestamp(request.timestamp, max_age_secs)?;

    // Compute expected signature (legacy: no method/path)
    let expected = sign_payload(
        &request.client_id,
        secret_key,
        &request.payload,
        request.timestamp,
        &request.nonce,
    );

    // Constant-time comparison
    if !constant_time_eq(expected.as_bytes(), request.signature.as_bytes()) {
        return Err(AuthError::InvalidSignature);
    }

    Ok(())
}

/// Verify a signature with method/path binding.
///
/// R5-04: This is the recommended verification function for production use.
/// It ensures the signature is bound to the specific HTTP method and path,
/// preventing replay attacks to different endpoints.
///
/// The `actual_method` and `actual_path` parameters are the method and path
/// from the incoming HTTP request, NOT from the signed request body.
///
/// Returns an error if:
/// - The timestamp is in the future (beyond clock skew tolerance)
/// - The timestamp is too old (replay protection)
/// - The method doesn't match what was signed
/// - The path doesn't match what was signed
/// - The signature doesn't match
pub fn verify_signature_with_binding<T: serde::Serialize>(
    request: &super::SignedRequest<T>,
    secret_key: &[u8],
    actual_method: &str,
    actual_path: &str,
    max_age_secs: i64,
) -> Result<(), AuthError> {
    verify_timestamp(request.timestamp, max_age_secs)?;

    // R5-04: Verify method matches (case-insensitive)
    if request.method.to_uppercase() != actual_method.to_uppercase() {
        return Err(AuthError::MethodMismatch {
            expected: request.method.clone(),
            actual: actual_method.to_string(),
        });
    }

    // R5-04: Verify path matches (canonical comparison)
    let expected_path = canonicalize_path(&request.path);
    let actual_canonical = canonicalize_path(actual_path);
    if expected_path != actual_canonical {
        return Err(AuthError::PathMismatch {
            expected: request.path.clone(),
            actual: actual_path.to_string(),
        });
    }

    // Compute expected signature with method/path binding
    let expected = sign_payload_with_binding(
        &request.client_id,
        secret_key,
        &request.method,
        &request.path,
        &request.payload,
        request.timestamp,
        &request.nonce,
    );

    // Constant-time comparison
    if !constant_time_eq(expected.as_bytes(), request.signature.as_bytes()) {
        return Err(AuthError::InvalidSignature);
    }

    Ok(())
}

/// Verify timestamp constraints.
fn verify_timestamp(timestamp: i64, max_age_secs: i64) -> Result<(), AuthError> {
    let now = chrono::Utc::now().timestamp();
    let age = now - timestamp;

    // Reject timestamps in the future (allowing small clock skew)
    if age < -MAX_FUTURE_SKEW_SECS {
        return Err(AuthError::TimestampInFuture);
    }

    // Reject timestamps that are too old
    if age > max_age_secs {
        return Err(AuthError::TimestampExpired);
    }

    Ok(())
}

/// Verify a signature with full replay protection (sync version, legacy).
///
/// NOTE: This uses in-memory nonce tracking which doesn't work across replicas.
/// For multi-replica deployments, use `verify_with_replay_protection_async` instead.
///
/// DEPRECATED: Use `verify_with_replay_protection_binding` for new code.
///
/// Returns an error if:
/// - The timestamp is in the future (beyond clock skew tolerance)
/// - The timestamp is too old (replay protection)
/// - The nonce has already been used (replay attack)
/// - The signature doesn't match
pub fn verify_with_replay_protection<T: serde::Serialize>(
    request: &super::SignedRequest<T>,
    secret_key: &[u8],
    max_age_secs: i64,
    nonce_registry: &super::NonceRegistry,
) -> Result<(), AuthError> {
    // First verify the signature (checks timestamp too)
    verify_signature(request, secret_key, max_age_secs)?;

    // Then check and register the nonce
    // This prevents replays within the validity window
    nonce_registry.check_and_register(
        &request.client_id,
        &request.nonce,
        request.timestamp,
    )?;

    Ok(())
}

/// Verify a signature with method/path binding and replay protection (sync version).
///
/// R5-04: This is the recommended verification function for production use.
/// It combines signature verification with method/path binding and nonce tracking.
///
/// NOTE: This uses in-memory nonce tracking which doesn't work across replicas.
/// For multi-replica deployments, use `verify_with_replay_protection_binding_async`.
pub fn verify_with_replay_protection_binding<T: serde::Serialize>(
    request: &super::SignedRequest<T>,
    secret_key: &[u8],
    actual_method: &str,
    actual_path: &str,
    max_age_secs: i64,
    nonce_registry: &super::NonceRegistry,
) -> Result<(), AuthError> {
    // First verify the signature with method/path binding
    verify_signature_with_binding(request, secret_key, actual_method, actual_path, max_age_secs)?;

    // Then check and register the nonce
    nonce_registry.check_and_register(
        &request.client_id,
        &request.nonce,
        request.timestamp,
    )?;

    Ok(())
}

/// Verify a signature with full replay protection (async version, legacy).
///
/// R3-04: This verification function supports multi-replica deployments
/// using an async nonce checker backed by PostgreSQL or other distributed stores.
///
/// DEPRECATED: Use `verify_with_replay_protection_binding_async` for new code.
///
/// Returns an error if:
/// - The timestamp is in the future (beyond clock skew tolerance)
/// - The timestamp is too old (replay protection)
/// - The nonce has already been used (replay attack)
/// - The signature doesn't match
/// - The nonce backend is unavailable (fail-closed for security)
pub async fn verify_with_replay_protection_async<T: serde::Serialize>(
    request: &super::SignedRequest<T>,
    secret_key: &[u8],
    max_age_secs: i64,
    nonce_checker: &dyn AsyncNonceChecker,
) -> Result<(), AuthError> {
    // First verify the signature (checks timestamp too)
    verify_signature(request, secret_key, max_age_secs)?;

    // Then atomically consume the nonce
    // This prevents replays within the validity window
    match nonce_checker
        .consume_nonce(&request.client_id, &request.nonce, request.timestamp)
        .await
    {
        Ok(true) => Ok(()),          // Nonce was fresh, now consumed
        Ok(false) => Err(AuthError::NonceReused), // Nonce was already used
        Err(e) => Err(e),            // Backend error (fail-closed)
    }
}

/// Verify a signature with method/path binding and replay protection (async version).
///
/// R5-04: This is the recommended verification function for production use
/// in multi-replica deployments. It combines signature verification with
/// method/path binding and async nonce checking.
///
/// The `actual_method` and `actual_path` parameters are from the incoming
/// HTTP request, NOT from the signed request body.
pub async fn verify_with_replay_protection_binding_async<T: serde::Serialize>(
    request: &super::SignedRequest<T>,
    secret_key: &[u8],
    actual_method: &str,
    actual_path: &str,
    max_age_secs: i64,
    nonce_checker: &dyn AsyncNonceChecker,
) -> Result<(), AuthError> {
    // First verify the signature with method/path binding
    verify_signature_with_binding(request, secret_key, actual_method, actual_path, max_age_secs)?;

    // Then atomically consume the nonce
    match nonce_checker
        .consume_nonce(&request.client_id, &request.nonce, request.timestamp)
        .await
    {
        Ok(true) => Ok(()),
        Ok(false) => Err(AuthError::NonceReused),
        Err(e) => Err(e),
    }
}

/// Constant-time byte comparison to prevent timing attacks.
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
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestPayload {
        data: String,
    }

    fn make_legacy_request(
        client_id: &str,
        secret_key: &[u8],
        payload: TestPayload,
        timestamp: i64,
        nonce: &str,
    ) -> super::super::SignedRequest<TestPayload> {
        let signature = sign_payload(client_id, secret_key, &payload, timestamp, nonce);
        super::super::SignedRequest {
            client_id: client_id.to_string(),
            timestamp,
            nonce: nonce.to_string(),
            signature,
            method: String::new(),
            path: String::new(),
            payload,
        }
    }

    fn make_bound_request(
        client_id: &str,
        secret_key: &[u8],
        method: &str,
        path: &str,
        payload: TestPayload,
        timestamp: i64,
        nonce: &str,
    ) -> super::super::SignedRequest<TestPayload> {
        let signature = sign_payload_with_binding(
            client_id, secret_key, method, path, &payload, timestamp, nonce,
        );
        super::super::SignedRequest {
            client_id: client_id.to_string(),
            timestamp,
            nonce: nonce.to_string(),
            signature,
            method: method.to_string(),
            path: path.to_string(),
            payload,
        }
    }

    #[test]
    fn test_sign_and_verify_legacy() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp();
        let request = make_legacy_request("test_client", b"test_secret", payload, timestamp, "test_nonce");
        assert!(verify_signature(&request, b"test_secret", 300).is_ok());
    }

    #[test]
    fn test_sign_and_verify_with_binding() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp();
        let request = make_bound_request(
            "test_client",
            b"test_secret",
            "POST",
            "/api/v1/licenses/claim",
            payload,
            timestamp,
            "test_nonce",
        );

        // Should succeed with matching method/path
        assert!(verify_signature_with_binding(
            &request,
            b"test_secret",
            "POST",
            "/api/v1/licenses/claim",
            300,
        ).is_ok());
    }

    #[test]
    fn test_method_mismatch_rejected() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp();
        let request = make_bound_request(
            "test_client",
            b"test_secret",
            "POST",
            "/api/v1/licenses/claim",
            payload,
            timestamp,
            "test_nonce",
        );

        // Should fail with different method
        let result = verify_signature_with_binding(
            &request,
            b"test_secret",
            "DELETE", // Wrong method!
            "/api/v1/licenses/claim",
            300,
        );
        assert!(matches!(result, Err(AuthError::MethodMismatch { .. })));
    }

    #[test]
    fn test_path_mismatch_rejected() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp();
        let request = make_bound_request(
            "test_client",
            b"test_secret",
            "POST",
            "/api/v1/licenses/claim",
            payload,
            timestamp,
            "test_nonce",
        );

        // Should fail with different path (replay to different endpoint)
        let result = verify_signature_with_binding(
            &request,
            b"test_secret",
            "POST",
            "/api/v1/admin/delete-all", // Wrong path!
            300,
        );
        assert!(matches!(result, Err(AuthError::PathMismatch { .. })));
    }

    #[test]
    fn test_path_canonicalization() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp();

        // Signed with trailing slash
        let request = make_bound_request(
            "test_client",
            b"test_secret",
            "POST",
            "/api/v1/licenses/",
            payload,
            timestamp,
            "test_nonce",
        );

        // Should match without trailing slash (canonicalization)
        assert!(verify_signature_with_binding(
            &request,
            b"test_secret",
            "POST",
            "/api/v1/licenses",
            300,
        ).is_ok());
    }

    #[test]
    fn test_method_case_insensitive() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp();

        // Signed with lowercase
        let request = make_bound_request(
            "test_client",
            b"test_secret",
            "post",
            "/api/v1/licenses",
            payload,
            timestamp,
            "test_nonce",
        );

        // Should match uppercase (case-insensitive)
        assert!(verify_signature_with_binding(
            &request,
            b"test_secret",
            "POST",
            "/api/v1/licenses",
            300,
        ).is_ok());
    }

    #[test]
    fn test_expired_timestamp() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp() - 600; // 10 minutes ago
        let request = make_legacy_request("test_client", b"test_secret", payload, timestamp, "test_nonce");

        let result = verify_signature(&request, b"test_secret", 300);
        assert!(matches!(result, Err(AuthError::TimestampExpired)));
    }

    #[test]
    fn test_invalid_signature() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp();

        let request = super::super::SignedRequest {
            client_id: "test_client".to_string(),
            timestamp,
            nonce: "test_nonce".to_string(),
            signature: "invalid_signature".to_string(),
            method: String::new(),
            path: String::new(),
            payload,
        };

        let result = verify_signature(&request, b"test_secret", 300);
        assert!(matches!(result, Err(AuthError::InvalidSignature)));
    }

    #[test]
    fn test_future_timestamp_rejected() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        // 10 seconds in the future (beyond MAX_FUTURE_SKEW_SECS)
        let timestamp = chrono::Utc::now().timestamp() + 10;
        let request = make_legacy_request("test_client", b"test_secret", payload, timestamp, "test_nonce");

        let result = verify_signature(&request, b"test_secret", 300);
        assert!(matches!(result, Err(AuthError::TimestampInFuture)));
    }

    #[test]
    fn test_small_future_skew_allowed() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        // 3 seconds in the future (within MAX_FUTURE_SKEW_SECS)
        let timestamp = chrono::Utc::now().timestamp() + 3;
        let request = make_legacy_request("test_client", b"test_secret", payload, timestamp, "test_nonce");

        // Should succeed - small clock skew is tolerated
        assert!(verify_signature(&request, b"test_secret", 300).is_ok());
    }

    #[test]
    fn test_replay_protection_first_use() {
        let registry = super::super::NonceRegistry::new(300);
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp();
        let request = make_legacy_request("test_client", b"test_secret", payload, timestamp, "unique_nonce_1");

        // First use should succeed
        assert!(verify_with_replay_protection(&request, b"test_secret", 300, &registry).is_ok());
    }

    #[test]
    fn test_replay_protection_rejects_replay() {
        let registry = super::super::NonceRegistry::new(300);
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp();
        let request = make_legacy_request("test_client", b"test_secret", payload, timestamp, "replayed_nonce");

        // First use succeeds
        assert!(verify_with_replay_protection(&request, b"test_secret", 300, &registry).is_ok());

        // Replay is rejected
        let result = verify_with_replay_protection(&request, b"test_secret", 300, &registry);
        assert!(matches!(result, Err(AuthError::NonceReused)));
    }

    #[test]
    fn test_replay_protection_with_binding() {
        let registry = super::super::NonceRegistry::new(300);
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp();
        let request = make_bound_request(
            "test_client",
            b"test_secret",
            "POST",
            "/api/v1/claim",
            payload,
            timestamp,
            "bound_nonce",
        );

        // First use succeeds
        assert!(verify_with_replay_protection_binding(
            &request,
            b"test_secret",
            "POST",
            "/api/v1/claim",
            300,
            &registry,
        ).is_ok());

        // Replay is rejected
        let result = verify_with_replay_protection_binding(
            &request,
            b"test_secret",
            "POST",
            "/api/v1/claim",
            300,
            &registry,
        );
        assert!(matches!(result, Err(AuthError::NonceReused)));
    }
}
