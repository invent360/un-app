//! V4 Signed URL generation for GCS

use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::DecodePrivateKey;
use rsa::signature::{SignatureEncoding, Signer};
use rsa::RsaPrivateKey;
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::error::{Result, StorageError};

use super::auth::ServiceAccountKey;

/// Generate a V4 signed URL for a GCS object
///
/// # Arguments
/// * `bucket` - Bucket name
/// * `object_name` - Object path within the bucket
/// * `service_account` - Service account with private key
/// * `expiration` - URL expiration duration
///
/// # Returns
/// Signed URL that can be used to access the object
pub fn generate_signed_url(
    bucket: &str,
    object_name: &str,
    service_account: &ServiceAccountKey,
    expiration: Duration,
) -> Result<String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| StorageError::InternalError(format!("Time error: {}", e)))?
        .as_secs();

    // Format timestamp for GCS V4 signing
    let datetime = chrono::DateTime::from_timestamp(now as i64, 0)
        .ok_or_else(|| StorageError::InternalError("Invalid timestamp".to_string()))?;
    let date_stamp = datetime.format("%Y%m%d").to_string();
    let request_timestamp = datetime.format("%Y%m%dT%H%M%SZ").to_string();

    // Credential scope
    let credential_scope = format!("{}/auto/storage/goog4_request", date_stamp);
    let credential = format!("{}/{}", service_account.client_email, credential_scope);

    // Canonical headers
    let host = "storage.googleapis.com";
    let canonical_headers = format!("host:{}\n", host);
    let signed_headers = "host";

    // Query parameters for signed URL
    let expiration_secs = expiration.as_secs();
    let canonical_query_string = format!(
        "X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Credential={}&X-Goog-Date={}&X-Goog-Expires={}&X-Goog-SignedHeaders={}",
        urlencoding::encode(&credential),
        request_timestamp,
        expiration_secs,
        signed_headers
    );

    // Canonical resource path
    let canonical_resource = format!("/{}/{}", bucket, object_name);

    // Canonical request
    let canonical_request = format!(
        "GET\n{}\n{}\n{}\n{}\nUNSIGNED-PAYLOAD",
        canonical_resource, canonical_query_string, canonical_headers, signed_headers
    );

    // Hash the canonical request
    let mut hasher = Sha256::new();
    hasher.update(canonical_request.as_bytes());
    let canonical_request_hash = hasher.finalize();
    let canonical_request_hash_hex = hex::encode(&canonical_request_hash);

    // String to sign
    let string_to_sign = format!(
        "GOOG4-RSA-SHA256\n{}\n{}\n{}",
        request_timestamp, credential_scope, canonical_request_hash_hex
    );

    // Parse private key and sign
    let private_key = RsaPrivateKey::from_pkcs8_pem(&service_account.private_key)
        .map_err(|e| StorageError::ConfigError(format!("Invalid private key: {}", e)))?;

    let signing_key: SigningKey<Sha256> = SigningKey::new(private_key);
    let signature = signing_key.sign(string_to_sign.as_bytes());
    let signature_hex = hex::encode(signature.to_bytes());

    // Build the signed URL
    let signed_url = format!(
        "https://{}{}?{}&X-Goog-Signature={}",
        host, canonical_resource, canonical_query_string, signature_hex
    );

    Ok(signed_url)
}

/// Generate a public URL for a GCS object (for public buckets)
pub fn generate_public_url(bucket: &str, object_name: &str) -> String {
    format!(
        "https://storage.googleapis.com/{}/{}",
        bucket,
        urlencoding::encode(object_name)
    )
}

/// Generate a storage URL (compact format for database storage)
pub fn generate_storage_url(bucket: &str, object_name: &str) -> String {
    format!("gcs://{}/{}", bucket, object_name)
}

/// Parse a storage URL to extract bucket and object name
pub fn parse_storage_url(storage_url: &str) -> Option<(String, String)> {
    if storage_url.starts_with("gcs://") {
        let path = &storage_url[6..]; // Remove "gcs://"
        let parts: Vec<&str> = path.splitn(2, '/').collect();
        if parts.len() == 2 {
            return Some((parts[0].to_string(), parts[1].to_string()));
        }
    } else if storage_url.starts_with("https://storage.googleapis.com/") {
        let path = &storage_url[31..]; // Remove "https://storage.googleapis.com/"
        let parts: Vec<&str> = path.splitn(2, '/').collect();
        if parts.len() == 2 {
            return Some((parts[0].to_string(), parts[1].to_string()));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_storage_url_gcs() {
        let (bucket, object) = parse_storage_url("gcs://my-bucket/path/to/file.jpg").unwrap();
        assert_eq!(bucket, "my-bucket");
        assert_eq!(object, "path/to/file.jpg");
    }

    #[test]
    fn test_parse_storage_url_https() {
        let (bucket, object) =
            parse_storage_url("https://storage.googleapis.com/my-bucket/path/to/file.jpg")
                .unwrap();
        assert_eq!(bucket, "my-bucket");
        assert_eq!(object, "path/to/file.jpg");
    }

    #[test]
    fn test_parse_storage_url_invalid() {
        assert!(parse_storage_url("https://example.com/file.jpg").is_none());
        assert!(parse_storage_url("invalid").is_none());
    }

    #[test]
    fn test_generate_storage_url() {
        let url = generate_storage_url("my-bucket", "path/to/file.jpg");
        assert_eq!(url, "gcs://my-bucket/path/to/file.jpg");
    }

    #[test]
    fn test_generate_public_url() {
        let url = generate_public_url("my-bucket", "path/to/file.jpg");
        assert_eq!(
            url,
            "https://storage.googleapis.com/my-bucket/path%2Fto%2Ffile.jpg"
        );
    }
}
