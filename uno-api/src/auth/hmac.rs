//! HMAC signing and verification.

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::AuthError;

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

/// Verify a signature.
///
/// Returns an error if:
/// - The timestamp is too old (replay protection)
/// - The signature doesn't match
pub fn verify_signature<T: serde::Serialize>(
    request: &super::SignedRequest<T>,
    secret_key: &[u8],
    max_age_secs: i64,
) -> Result<(), AuthError> {
    // Check timestamp freshness
    let now = chrono::Utc::now().timestamp();
    let age = (now - request.timestamp).abs();

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
}
