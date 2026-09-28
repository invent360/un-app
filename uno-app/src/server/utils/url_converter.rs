//! URL conversion utilities for cloud storage
//!
//! Converts storage URLs (gcs://bucket/path) to browser-loadable signed URLs
//! when serving content to clients.

use file_storage::{create_client_from_env, FileStorageClient};
use serde_json::Value;

/// Convert all storage URLs in a JSON value to display URLs
///
/// Recursively scans the JSON structure and converts any string values
/// that start with "gcs://" or "s3://" to signed display URLs.
pub fn convert_storage_urls(value: &mut Value) {
    match value {
        Value::String(s) => {
            if s.starts_with("gcs://") || s.starts_with("s3://") {
                if let Ok(client) = create_client_from_env() {
                    *s = client.storage_to_display_url(s);
                }
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                convert_storage_urls(item);
            }
        }
        Value::Object(obj) => {
            for (_, v) in obj.iter_mut() {
                convert_storage_urls(v);
            }
        }
        _ => {}
    }
}

/// Convert storage URLs in a JSON string, returning the modified string
pub fn convert_storage_urls_in_json(json_str: &str) -> Result<String, serde_json::Error> {
    let mut value: Value = serde_json::from_str(json_str)?;
    convert_storage_urls(&mut value);
    serde_json::to_string(&value)
}

/// Convert a single storage URL to a display URL
pub fn storage_to_display_url(storage_url: &str) -> String {
    if storage_url.starts_with("gcs://") || storage_url.starts_with("s3://") {
        if let Ok(client) = create_client_from_env() {
            return client.storage_to_display_url(storage_url);
        }
    }
    storage_url.to_string()
}

/// Convert multiple storage URLs to display URLs
pub fn storage_to_display_urls(storage_urls: &[String]) -> Vec<String> {
    if let Ok(client) = create_client_from_env() {
        client.storage_to_display_urls(storage_urls)
    } else {
        storage_urls.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_convert_storage_urls_in_object() {
        // Without actual GCS client, URLs won't be converted
        // This test just verifies the recursive traversal works
        let mut value = json!({
            "title": "Test",
            "cover_image": "https://example.com/image.jpg",
            "sections": [
                {
                    "title": "Section 1",
                    "images": ["https://example.com/a.jpg", "https://example.com/b.jpg"]
                }
            ]
        });

        convert_storage_urls(&mut value);

        // URLs should remain unchanged since they're not gcs:// URLs
        assert_eq!(value["cover_image"], "https://example.com/image.jpg");
    }

    #[test]
    fn test_storage_to_display_url_passthrough() {
        // Non-storage URLs should pass through unchanged
        let url = "https://example.com/image.jpg";
        assert_eq!(storage_to_display_url(url), url);
    }
}
