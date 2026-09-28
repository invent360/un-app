//! File API handlers for cloud storage (GCS/S3)
//!
//! Provides endpoints for uploading files to cloud storage and
//! serving them via signed URLs.

use actix_web::{HttpResponse, web, http::header};
use file_storage::{create_client_from_env, FileStorageClient};
use uuid::Uuid;

/// GET /api/files/{storage_url}
/// Redirects to a signed display URL for the file
pub async fn get_file(
    path: web::Path<String>,
) -> HttpResponse {
    let storage_url = path.into_inner();

    // Decode URL-encoded storage URL (e.g., gcs%3A%2F%2Fbucket%2Fpath -> gcs://bucket/path)
    let storage_url = match urlencoding::decode(&storage_url) {
        Ok(url) => url.to_string(),
        Err(_) => storage_url,
    };

    let client = match create_client_from_env() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to create storage client: {}", e);
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Storage service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    let display_url = client.storage_to_display_url(&storage_url);

    HttpResponse::TemporaryRedirect()
        .insert_header((header::LOCATION, display_url))
        .insert_header((header::CACHE_CONTROL, "private, max-age=3600"))
        .finish()
}

/// Request to convert storage URL to display URL
#[derive(Debug, serde::Deserialize)]
pub struct DisplayUrlQuery {
    pub storage_url: String,
}

/// Response with display URL
#[derive(Debug, serde::Serialize)]
pub struct DisplayUrlResponse {
    pub display_url: String,
}

/// GET /api/files/display-url?storage_url=gcs://...
/// Converts a storage URL to a browser-loadable signed URL
pub async fn get_display_url(
    query: web::Query<DisplayUrlQuery>,
) -> HttpResponse {
    let client = match create_client_from_env() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to create storage client: {}", e);
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Storage service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    let display_url = client.storage_to_display_url(&query.storage_url);

    HttpResponse::Ok().json(DisplayUrlResponse { display_url })
}

/// Request to convert multiple storage URLs
#[derive(Debug, serde::Deserialize)]
pub struct BatchDisplayUrlRequest {
    pub storage_urls: Vec<String>,
}

/// Response with multiple display URLs
#[derive(Debug, serde::Serialize)]
pub struct BatchDisplayUrlResponse {
    pub display_urls: Vec<String>,
}

/// POST /api/files/display-urls
/// Converts multiple storage URLs to display URLs
pub async fn batch_display_urls(
    body: web::Json<BatchDisplayUrlRequest>,
) -> HttpResponse {
    let client = match create_client_from_env() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to create storage client: {}", e);
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Storage service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    let display_urls = client.storage_to_display_urls(&body.storage_urls);

    HttpResponse::Ok().json(BatchDisplayUrlResponse { display_urls })
}

/// Response from file upload
#[derive(Debug, serde::Serialize)]
pub struct FileUploadResponse {
    pub storage_urls: Vec<String>,
    pub display_urls: Vec<String>,
    pub uploaded_count: usize,
    pub total_bytes: u64,
}

/// POST /api/admin/files/upload
/// Upload files to cloud storage
pub async fn upload_files(
    mut payload: actix_multipart::Multipart,
) -> HttpResponse {
    use futures_util::StreamExt;

    let client = match create_client_from_env() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to create storage client: {}", e);
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Storage service unavailable",
                "code": "SERVICE_UNAVAILABLE"
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
                tracing::error!("Failed to read multipart field: {}", e);
                continue;
            }
        };

        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "resource_id" => {
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
                let filename = field.content_disposition()
                    .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
                    .unwrap_or_else(|| "unknown".to_string());

                let content_type = field.content_type()
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
                while field.next().await.is_some() {}
            }
        }
    }

    // Generate resource ID if not provided
    let resource_id = resource_id.unwrap_or_else(|| Uuid::new_v4().to_string());

    if files.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No files provided",
            "code": "MISSING_FILES"
        }));
    }

    tracing::info!("Uploading {} files for resource {}", files.len(), resource_id);

    match client.upload_files(&resource_id, files, None).await {
        Ok(result) => {
            tracing::info!(
                "Upload complete: {} files, {} bytes",
                result.uploaded_count, result.total_bytes
            );

            HttpResponse::Created().json(FileUploadResponse {
                storage_urls: result.storage_urls,
                display_urls: result.display_urls,
                uploaded_count: result.uploaded_count,
                total_bytes: result.total_bytes,
            })
        }
        Err(e) => {
            tracing::error!("Upload failed: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Upload failed: {}", e),
                "code": "UPLOAD_FAILED"
            }))
        }
    }
}

/// DELETE /api/admin/files/{resource_id}
/// Delete all files for a resource from cloud storage
pub async fn delete_files(
    path: web::Path<String>,
) -> HttpResponse {
    let resource_id = path.into_inner();

    let client = match create_client_from_env() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to create storage client: {}", e);
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Storage service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    match client.delete_files(&resource_id).await {
        Ok(_) => {
            tracing::info!("Deleted files for resource {}", resource_id);
            HttpResponse::NoContent().finish()
        }
        Err(e) => {
            tracing::error!("Delete failed for {}: {}", resource_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Delete failed: {}", e),
                "code": "DELETE_FAILED"
            }))
        }
    }
}

/// Configure file storage routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/files")
            .route("/display-url", web::get().to(get_display_url))
            .route("/display-urls", web::post().to(batch_display_urls))
            .route("/serve/{storage_url:.*}", web::get().to(get_file))
    );

    cfg.service(
        web::scope("/api/admin/files")
            .route("/upload", web::post().to(upload_files))
            .route("/{resource_id}", web::delete().to(delete_files))
    );
}
