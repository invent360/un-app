//! Base64 to GCS conversion utilities
//!
//! This module provides functions to detect and convert base64 data URLs
//! in JSON content to GCS storage URLs during save operations.

use base64::Engine;
use file_storage::FileStorageClient;
use serde_json::Value;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Check if a string is a base64 data URL
pub fn is_base64_data_url(s: &str) -> bool {
    s.starts_with("data:image/") || s.starts_with("data:video/") || s.starts_with("data:audio/")
}

/// Extract MIME type, extension, and bytes from a base64 data URL
///
/// Format: data:image/png;base64,iVBORw0KGgo...
pub fn parse_base64_data_url(data_url: &str) -> Option<(String, String, Vec<u8>)> {
    // Split by comma to separate header from data
    let parts: Vec<&str> = data_url.splitn(2, ',').collect();
    if parts.len() != 2 {
        return None;
    }

    let header = parts[0]; // data:image/png;base64
    let base64_data = parts[1];

    // Extract MIME type from header
    // Header format: data:image/png;base64 or data:image/png
    let mime_type = header
        .strip_prefix("data:")?
        .split(';')
        .next()?
        .to_string();

    // Get file extension from MIME type
    let extension = mime_type_to_extension(&mime_type);

    // Decode base64 data
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_data)
        .ok()?;

    Some((mime_type, extension, bytes))
}

/// Convert MIME type to file extension
fn mime_type_to_extension(mime_type: &str) -> String {
    match mime_type {
        "image/png" => "png".to_string(),
        "image/jpeg" | "image/jpg" => "jpg".to_string(),
        "image/gif" => "gif".to_string(),
        "image/webp" => "webp".to_string(),
        "image/svg+xml" => "svg".to_string(),
        "image/avif" => "avif".to_string(),
        "video/mp4" => "mp4".to_string(),
        "video/webm" => "webm".to_string(),
        "video/quicktime" => "mov".to_string(),
        "audio/mpeg" | "audio/mp3" => "mp3".to_string(),
        "audio/wav" => "wav".to_string(),
        "audio/ogg" => "ogg".to_string(),
        _ => "bin".to_string(),
    }
}

/// Recursively convert base64 data URLs in JSON to GCS storage URLs
///
/// Returns the count of converted images.
pub async fn convert_base64_to_gcs(
    value: &mut Value,
    content_type: &str,
    storage_client: &dyn FileStorageClient,
) -> Result<usize, String> {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    let mut converted_count = 0;

    match value {
        Value::String(s) => {
            if is_base64_data_url(s) {
                // Parse and upload
                if let Some((mime_type, extension, bytes)) = parse_base64_data_url(s) {
                    // Generate unique filename
                    let index = COUNTER.fetch_add(1, Ordering::SeqCst);
                    let timestamp = chrono::Utc::now().timestamp_millis();
                    let filename = format!("{}_{}.{}", index, timestamp, extension);

                    // Upload to GCS
                    match storage_client
                        .upload_file(content_type, bytes, &filename, &mime_type)
                        .await
                    {
                        Ok(storage_url) => {
                            tracing::info!(
                                "Converted base64 to GCS: {} -> {}",
                                &s[..50.min(s.len())],
                                storage_url
                            );
                            *s = storage_url;
                            converted_count += 1;
                        }
                        Err(e) => {
                            tracing::error!("Failed to upload base64 to GCS: {}", e);
                            return Err(format!("Failed to upload image: {}", e));
                        }
                    }
                }
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                converted_count += Box::pin(convert_base64_to_gcs(item, content_type, storage_client)).await?;
            }
        }
        Value::Object(obj) => {
            for (_, v) in obj.iter_mut() {
                converted_count += Box::pin(convert_base64_to_gcs(v, content_type, storage_client)).await?;
            }
        }
        _ => {}
    }

    Ok(converted_count)
}

/// Check if a JSON value contains any base64 data URLs
pub fn contains_base64_data(value: &Value) -> bool {
    match value {
        Value::String(s) => is_base64_data_url(s),
        Value::Array(arr) => arr.iter().any(contains_base64_data),
        Value::Object(obj) => obj.values().any(contains_base64_data),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_base64_data_url() {
        assert!(is_base64_data_url("data:image/png;base64,iVBORw0KGgo="));
        assert!(is_base64_data_url("data:image/jpeg;base64,/9j/4AAQ"));
        assert!(is_base64_data_url("data:video/mp4;base64,AAAA"));
        assert!(!is_base64_data_url("gcs://bucket/path/file.png"));
        assert!(!is_base64_data_url("https://example.com/image.png"));
        assert!(!is_base64_data_url(""));
    }

    #[test]
    fn test_parse_base64_data_url() {
        let data_url = "data:image/png;base64,SGVsbG8="; // "Hello" in base64
        let result = parse_base64_data_url(data_url);
        assert!(result.is_some());
        let (mime, ext, bytes) = result.unwrap();
        assert_eq!(mime, "image/png");
        assert_eq!(ext, "png");
        assert_eq!(bytes, b"Hello");
    }

    #[test]
    fn test_contains_base64_data() {
        let json_with_base64 = serde_json::json!({
            "title": "Test",
            "images": ["data:image/png;base64,iVBORw0KGgo="]
        });
        assert!(contains_base64_data(&json_with_base64));

        let json_without_base64 = serde_json::json!({
            "title": "Test",
            "images": ["gcs://bucket/image.png"]
        });
        assert!(!contains_base64_data(&json_without_base64));
    }
}
