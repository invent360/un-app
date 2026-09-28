//! File upload handler for cloud storage integration
//!
//! Provides endpoints for:
//! - POST /api/files/upload - Upload files to cloud storage
//! - GET /api/files/display-url - Convert storage URL to display URL
//! - DELETE /api/files/{resource_id} - Delete all files for a resource

#[cfg(feature = "ssr")]
use actix_multipart::Multipart;
#[cfg(feature = "ssr")]
use actix_web::{web, HttpResponse};
#[cfg(feature = "ssr")]
use futures_util::StreamExt;
#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use tracing::{error, info};

/// Response from file upload
#[cfg(feature = "ssr")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResponse {
    /// Storage URLs (compact format for database)
    pub storage_urls: Vec<String>,
    /// Display URLs (browser-loadable, signed)
    pub display_urls: Vec<String>,
    /// Number of files uploaded
    pub uploaded_count: usize,
    /// Total bytes uploaded
    pub total_bytes: u64,
    /// Errors for failed uploads
    pub errors: Vec<String>,
}

/// Request to convert storage URL to display URL
#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct DisplayUrlQuery {
    pub storage_url: String,
}

/// Response with display URL
#[cfg(feature = "ssr")]
#[derive(Debug, Serialize)]
pub struct DisplayUrlResponse {
    pub display_url: String,
}

/// Request to convert multiple storage URLs
#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct BatchDisplayUrlRequest {
    pub storage_urls: Vec<String>,
}

/// Response with multiple display URLs
#[cfg(feature = "ssr")]
#[derive(Debug, Serialize)]
pub struct BatchDisplayUrlResponse {
    pub display_urls: Vec<String>,
}

/// Upload files to cloud storage
///
/// Accepts multipart form data with:
/// - `resource_id`: Resource identifier (used as folder prefix)
/// - `file`: One or more files to upload
#[cfg(feature = "ssr")]
pub async fn upload_files(mut payload: Multipart) -> HttpResponse {
    use file_storage::{create_client_from_env, FileStorageClient};

    // Create storage client
    let client = match create_client_from_env() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create storage client: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Storage not configured: {}", e)
            }));
        }
    };

    let mut resource_id: Option<String> = None;
    let mut files: Vec<(Vec<u8>, String, String)> = Vec::new();

    // Process multipart fields
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => {
                error!("Error reading multipart field: {}", e);
                continue;
            }
        };

        let content_disposition = field.content_disposition();
        let field_name = content_disposition
            .and_then(|cd| cd.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        match field_name.as_str() {
            "resource_id" => {
                // Read resource_id as text
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(bytes) = chunk {
                        data.extend_from_slice(&bytes);
                    }
                }
                if let Ok(id) = String::from_utf8(data) {
                    resource_id = Some(id.trim().to_string());
                }
            }
            "file" | "files" => {
                // Read file data
                let filename = content_disposition
                    .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
                    .unwrap_or_else(|| "unknown".to_string());

                let content_type = field
                    .content_type()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "application/octet-stream".to_string());

                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(bytes) = chunk {
                        data.extend_from_slice(&bytes);
                    }
                }

                if !data.is_empty() {
                    files.push((data, filename, content_type));
                }
            }
            _ => {
                // Skip unknown fields
            }
        }
    }

    // Validate resource_id
    let resource_id = match resource_id {
        Some(id) if !id.is_empty() => id,
        _ => {
            // Generate a UUID if no resource_id provided
            uuid::Uuid::new_v4().to_string()
        }
    };

    if files.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No files provided"
        }));
    }

    info!(
        "Uploading {} files for resource {}",
        files.len(),
        resource_id
    );

    // Upload files
    match client.upload_files(&resource_id, files, None).await {
        Ok(result) => {
            info!(
                "Upload complete: {} files, {} bytes",
                result.uploaded_count, result.total_bytes
            );

            HttpResponse::Ok().json(FileUploadResponse {
                storage_urls: result.storage_urls,
                display_urls: result.display_urls,
                uploaded_count: result.uploaded_count,
                total_bytes: result.total_bytes,
                errors: result.errors.into_iter().map(|e| e.error).collect(),
            })
        }
        Err(e) => {
            error!("Upload failed: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Upload failed: {}", e)
            }))
        }
    }
}

/// Convert a storage URL to a display URL
#[cfg(feature = "ssr")]
pub async fn get_display_url(query: web::Query<DisplayUrlQuery>) -> HttpResponse {
    use file_storage::{create_client_from_env, FileStorageClient};

    let client = match create_client_from_env() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create storage client: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Storage not configured: {}", e)
            }));
        }
    };

    let display_url = client.storage_to_display_url(&query.storage_url);

    HttpResponse::Ok().json(DisplayUrlResponse { display_url })
}

/// Convert multiple storage URLs to display URLs
#[cfg(feature = "ssr")]
pub async fn batch_display_urls(body: web::Json<BatchDisplayUrlRequest>) -> HttpResponse {
    use file_storage::{create_client_from_env, FileStorageClient};

    let client = match create_client_from_env() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create storage client: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Storage not configured: {}", e)
            }));
        }
    };

    let display_urls = client.storage_to_display_urls(&body.storage_urls);

    HttpResponse::Ok().json(BatchDisplayUrlResponse { display_urls })
}

/// Delete all files for a resource
#[cfg(feature = "ssr")]
pub async fn delete_files(path: web::Path<String>) -> HttpResponse {
    use file_storage::{create_client_from_env, FileStorageClient};

    let resource_id = path.into_inner();

    let client = match create_client_from_env() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create storage client: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Storage not configured: {}", e)
            }));
        }
    };

    match client.delete_files(&resource_id).await {
        Ok(_) => {
            info!("Deleted files for resource {}", resource_id);
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "resource_id": resource_id
            }))
        }
        Err(e) => {
            error!("Delete failed for {}: {}", resource_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Delete failed: {}", e)
            }))
        }
    }
}

/// Configure file upload routes
#[cfg(feature = "ssr")]
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/files")
            .route("/upload", web::post().to(upload_files))
            .route("/display-url", web::get().to(get_display_url))
            .route("/display-urls", web::post().to(batch_display_urls))
            .route("/{resource_id}", web::delete().to(delete_files)),
    );
}
