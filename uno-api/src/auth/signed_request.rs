//! Signed request wrapper.

use rand::Rng;
use serde::{Deserialize, Serialize};

/// A signed request wrapper.
///
/// Contains the payload along with authentication metadata:
/// - `client_id`: Identifies the client making the request
/// - `timestamp`: Unix timestamp when the request was created (for replay protection)
/// - `nonce`: Random string for additional uniqueness
/// - `signature`: HMAC-SHA256 signature of the request
/// - `method`: HTTP method (R5-04: binds signature to specific method)
/// - `path`: Canonical path (R5-04: binds signature to specific endpoint)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedRequest<T> {
    /// Client identifier.
    pub client_id: String,
    /// Unix timestamp (seconds since epoch).
    pub timestamp: i64,
    /// Random nonce for uniqueness.
    pub nonce: String,
    /// HMAC-SHA256 signature (hex encoded).
    pub signature: String,
    /// R5-04: HTTP method bound to signature (e.g., "POST", "GET").
    #[serde(default)]
    pub method: String,
    /// R5-04: Canonical path bound to signature (e.g., "/api/v1/licenses/claim").
    #[serde(default)]
    pub path: String,
    /// The actual request payload.
    pub payload: T,
}

impl<T: Serialize + Clone> SignedRequest<T> {
    /// Create and sign a new request with method/path binding.
    ///
    /// R5-04: The signature now includes method and path to prevent
    /// replay attacks to different endpoints.
    pub fn sign(
        client_id: &str,
        secret_key: &[u8],
        method: &str,
        path: &str,
        payload: T,
    ) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        let nonce = generate_nonce();
        let signature = super::hmac::sign_payload_with_binding(
            client_id,
            secret_key,
            method,
            path,
            &payload,
            timestamp,
            &nonce,
        );

        Self {
            client_id: client_id.to_string(),
            timestamp,
            nonce,
            signature,
            method: method.to_string(),
            path: path.to_string(),
            payload,
        }
    }

    /// Create a request with a specific timestamp (for testing).
    #[cfg(test)]
    pub fn with_timestamp(
        client_id: &str,
        secret_key: &[u8],
        method: &str,
        path: &str,
        payload: T,
        timestamp: i64,
    ) -> Self {
        let nonce = generate_nonce();
        let signature = super::hmac::sign_payload_with_binding(
            client_id,
            secret_key,
            method,
            path,
            &payload,
            timestamp,
            &nonce,
        );

        Self {
            client_id: client_id.to_string(),
            timestamp,
            nonce,
            signature,
            method: method.to_string(),
            path: path.to_string(),
            payload,
        }
    }
}

/// Generate a random nonce string.
fn generate_nonce() -> String {
    let mut rng = rand::rng();
    let bytes: Vec<u8> = (0..16).map(|_| rng.random()).collect();
    hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestPayload {
        value: i32,
    }

    #[test]
    fn test_sign_request_with_method_path() {
        let payload = TestPayload { value: 42 };
        let signed = SignedRequest::sign(
            "client1",
            b"secret",
            "POST",
            "/api/v1/licenses/claim",
            payload.clone(),
        );

        assert_eq!(signed.client_id, "client1");
        assert_eq!(signed.method, "POST");
        assert_eq!(signed.path, "/api/v1/licenses/claim");
        assert_eq!(signed.payload, payload);
        assert!(!signed.signature.is_empty());
        assert!(!signed.nonce.is_empty());
    }

    #[test]
    fn test_serialization_with_method_path() {
        let payload = TestPayload { value: 42 };
        let signed = SignedRequest::sign(
            "client1",
            b"secret",
            "POST",
            "/api/v1/claim",
            payload,
        );

        let json = serde_json::to_string(&signed).unwrap();
        let deserialized: SignedRequest<TestPayload> = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.client_id, signed.client_id);
        assert_eq!(deserialized.method, signed.method);
        assert_eq!(deserialized.path, signed.path);
        assert_eq!(deserialized.payload, signed.payload);
    }
}
