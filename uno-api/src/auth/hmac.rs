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

/// Sign a payload with HMAC-SHA256.
///
/// The signature is computed over: `client_id:timestamp:nonce:payload_json`
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

/// Maximum allowed clock skew for future timestamps (seconds).
/// Allows small clock drift but rejects obvious attacks.
const MAX_FUTURE_SKEW_SECS: i64 = 5;

/// Verify a signature.
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
    // Check timestamp - reject future timestamps (prevents pre-signed attacks)
    let now = chrono::Utc::now().timestamp();
    let age = now - request.timestamp;

    // Reject timestamps in the future (allowing small clock skew)
    if age < -MAX_FUTURE_SKEW_SECS {
        return Err(AuthError::TimestampInFuture);
    }

    // Reject timestamps that are too old
    if age > max_age_secs {
        return Err(AuthError::TimestampExpired);
    }

    // Compute expected signature
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

/// Verify a signature with full replay protection (sync version).
///
/// This is the recommended verification function for production use.
/// It combines signature verification with nonce tracking to prevent
/// replay attacks even within the timestamp validity window.
///
/// NOTE: This uses in-memory nonce tracking which doesn't work across replicas.
/// For multi-replica deployments, use `verify_with_replay_protection_async` instead.
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

/// Verify a signature with full replay protection (async version).
///
/// R3-04: This is the recommended verification function for production use
/// in multi-replica deployments. It uses an async nonce checker that can
/// be backed by PostgreSQL or other distributed stores.
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

    #[test]
    fn test_sign_and_verify() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let client_id = "test_client";
        let secret_key = b"test_secret";
        let timestamp = chrono::Utc::now().timestamp();
        let nonce = "test_nonce";

        let signature = sign_payload(client_id, secret_key, &payload, timestamp, nonce);

        let request = super::super::SignedRequest {
            client_id: client_id.to_string(),
            timestamp,
            nonce: nonce.to_string(),
            signature,
            payload,
        };

        assert!(verify_signature(&request, secret_key, 300).is_ok());
    }

    #[test]
    fn test_expired_timestamp() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let client_id = "test_client";
        let secret_key = b"test_secret";
        let timestamp = chrono::Utc::now().timestamp() - 600; // 10 minutes ago
        let nonce = "test_nonce";

        let signature = sign_payload(client_id, secret_key, &payload, timestamp, nonce);

        let request = super::super::SignedRequest {
            client_id: client_id.to_string(),
            timestamp,
            nonce: nonce.to_string(),
            signature,
            payload,
        };

        let result = verify_signature(&request, secret_key, 300);
        assert!(matches!(result, Err(AuthError::TimestampExpired)));
    }

    #[test]
    fn test_invalid_signature() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let client_id = "test_client";
        let timestamp = chrono::Utc::now().timestamp();
        let nonce = "test_nonce";

        let request = super::super::SignedRequest {
            client_id: client_id.to_string(),
            timestamp,
            nonce: nonce.to_string(),
            signature: "invalid_signature".to_string(),
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
        let client_id = "test_client";
        let secret_key = b"test_secret";
        // 10 seconds in the future (beyond MAX_FUTURE_SKEW_SECS)
        let timestamp = chrono::Utc::now().timestamp() + 10;
        let nonce = "test_nonce";

        let signature = sign_payload(client_id, secret_key, &payload, timestamp, nonce);

        let request = super::super::SignedRequest {
            client_id: client_id.to_string(),
            timestamp,
            nonce: nonce.to_string(),
            signature,
            payload,
        };

        let result = verify_signature(&request, secret_key, 300);
        assert!(matches!(result, Err(AuthError::TimestampInFuture)));
    }

    #[test]
    fn test_small_future_skew_allowed() {
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let client_id = "test_client";
        let secret_key = b"test_secret";
        // 3 seconds in the future (within MAX_FUTURE_SKEW_SECS)
        let timestamp = chrono::Utc::now().timestamp() + 3;
        let nonce = "test_nonce";

        let signature = sign_payload(client_id, secret_key, &payload, timestamp, nonce);

        let request = super::super::SignedRequest {
            client_id: client_id.to_string(),
            timestamp,
            nonce: nonce.to_string(),
            signature,
            payload,
        };

        // Should succeed - small clock skew is tolerated
        assert!(verify_signature(&request, secret_key, 300).is_ok());
    }

    #[test]
    fn test_replay_protection_first_use() {
        let registry = super::super::NonceRegistry::new(300);
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let client_id = "test_client";
        let secret_key = b"test_secret";
        let timestamp = chrono::Utc::now().timestamp();
        let nonce = "unique_nonce_1";

        let signature = sign_payload(client_id, secret_key, &payload, timestamp, nonce);

        let request = super::super::SignedRequest {
            client_id: client_id.to_string(),
            timestamp,
            nonce: nonce.to_string(),
            signature,
            payload,
        };

        // First use should succeed
        assert!(verify_with_replay_protection(&request, secret_key, 300, &registry).is_ok());
    }

    #[test]
    fn test_replay_protection_rejects_replay() {
        let registry = super::super::NonceRegistry::new(300);
        let payload = TestPayload {
            data: "test".to_string(),
        };
        let client_id = "test_client";
        let secret_key = b"test_secret";
        let timestamp = chrono::Utc::now().timestamp();
        let nonce = "replayed_nonce";

        let signature = sign_payload(client_id, secret_key, &payload, timestamp, nonce);

        let request = super::super::SignedRequest {
            client_id: client_id.to_string(),
            timestamp,
            nonce: nonce.to_string(),
            signature,
            payload,
        };

        // First use succeeds
        assert!(verify_with_replay_protection(&request, secret_key, 300, &registry).is_ok());

        // Replay is rejected
        let result = verify_with_replay_protection(&request, secret_key, 300, &registry);
        assert!(matches!(result, Err(AuthError::NonceReused)));
    }
}
