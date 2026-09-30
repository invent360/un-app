//! File API handlers for storage (local filesystem and cloud)
//!
//! Provides endpoints for uploading files and serving them.
//! For local storage, files are served directly with proper security headers.
//! For cloud storage, files are redirected to signed URLs.

use crate::server::middleware::AdminAuth;
use crate::server::services::{
    DynMediaAssetService, UploadRequest as MediaUploadRequest,
};
use actix_web::{http::header, web, HttpResponse};
use file_storage::create_client_from_env;
use uuid::Uuid;

/// MIME type mapping for file extensions
fn mime_from_extension(path: &str) -> &'static str {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "avif" => "image/avif",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mov" => "video/quicktime",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "json" => "application/json",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "csv" => "text/csv",
        _ => "application/octet-stream",
    }
}

/// GET /files/{resource_id}/{filename}
/// Serves local files directly with security headers
pub async fn serve_local_file(path: web::Path<(String, String)>) -> HttpResponse {
    let (resource_id, filename) = path.into_inner();

    // Basic path validation (additional checks in file-storage)
    if resource_id.contains("..")
        || filename.contains("..")
        || resource_id.contains('/')
        || resource_id.contains('\\')
    {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Path traversal rejected",
            "code": "FORBIDDEN"
        }));
    }

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

    let storage_url = format!("local://{resource_id}/{filename}");

    // Get file content from storage
    match client.get_file(&storage_url).await {
        Ok(data) => {
            let mime_type = mime_from_extension(&filename);

            HttpResponse::Ok()
                .insert_header((header::CONTENT_TYPE, mime_type))
                .insert_header((header::CACHE_CONTROL, "public, max-age=31536000, immutable"))
                .insert_header((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
                // Prevent XSS via uploaded content
                .insert_header((
                    header::CONTENT_SECURITY_POLICY,
                    "default-src 'none'; style-src 'unsafe-inline'",
                ))
                // Prevent files from being framed
                .insert_header((header::X_FRAME_OPTIONS, "DENY"))
                .body(data)
        }
        Err(e) => {
            let status_code = e.status_code();
            match status_code {
                404 => HttpResponse::NotFound().json(serde_json::json!({
                    "error": "File not found",
                    "code": "NOT_FOUND"
                })),
                403 => HttpResponse::Forbidden().json(serde_json::json!({
                    "error": "Access denied",
                    "code": "FORBIDDEN"
                })),
                _ => {
                    tracing::error!("Failed to serve file: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to retrieve file",
                        "code": "INTERNAL_ERROR"
                    }))
                }
            }
        }
    }
}

/// GET /api/files/{storage_url}
/// Redirects to a signed display URL for the file (cloud storage)
/// or serves directly for local storage
pub async fn get_file(path: web::Path<String>) -> HttpResponse {
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

    // For local files, serve directly instead of redirecting
    if storage_url.starts_with("local://") {
        match client.get_file(&storage_url).await {
            Ok(data) => {
                let mime_type = mime_from_extension(&storage_url);
                return HttpResponse::Ok()
                    .insert_header((header::CONTENT_TYPE, mime_type))
                    .insert_header((header::CACHE_CONTROL, "public, max-age=31536000, immutable"))
                    .insert_header((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
                    .body(data);
            }
            Err(e) => {
                tracing::error!("Failed to serve local file: {}", e);
                return HttpResponse::NotFound().json(serde_json::json!({
                    "error": "File not found",
                    "code": "NOT_FOUND"
                }));
            }
        }
    }

    // For cloud storage, redirect to display URL
    let display_url = client.storage_to_display_url(&storage_url);
    if display_url.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::TemporaryRedirect()
        .insert_header((header::LOCATION, display_url))
        .insert_header((header::CACHE_CONTROL, "private, max-age=3600"))
        .finish()
}

// ============================================
// R4-05: VISIBILITY-CHECKED FILE ACCESS
// ============================================

use crate::server::repositories::{DynMediaAssetRepository, AssetVisibility};

/// Query parameters for visibility-checked file access
#[derive(Debug, serde::Deserialize)]
pub struct VisibilityQuery {
    /// Optional accessor ID (user ID) for visibility check
    pub accessor_id: Option<String>,
    /// Set to true if the accessor is the owner
    #[serde(default)]
    pub is_owner: bool,
}

/// GET /files/v/{resource_id}/{filename}
/// R4-05: Serves files with visibility policy enforcement
pub async fn serve_file_with_visibility(
    path: web::Path<(String, String)>,
    query: web::Query<VisibilityQuery>,
    media_repo: web::Data<DynMediaAssetRepository>,
) -> HttpResponse {
    let (resource_id, filename) = path.into_inner();

    // Basic path validation
    if resource_id.contains("..")
        || filename.contains("..")
        || resource_id.contains('/')
        || resource_id.contains('\\')
    {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Path traversal rejected",
            "code": "FORBIDDEN"
        }));
    }

    let storage_url = format!("local://{resource_id}/{filename}");

    // R4-05: Look up asset and check visibility
    let asset = match media_repo.get_asset_by_storage_url(&storage_url).await {
        Ok(Some(asset)) => asset,
        Ok(None) => {
            // Asset not tracked - fall back to public access for backwards compatibility
            tracing::debug!(storage_url = %storage_url, "Asset not tracked, serving as public");
            return serve_untracked_file(&resource_id, &filename).await;
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to look up asset");
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check asset visibility",
                "code": "INTERNAL_ERROR"
            }));
        }
    };

    // R4-05: Check visibility policy
    let visibility = asset.visibility_enum();
    let accessor_id = query.accessor_id.as_deref();
    let is_owner = query.is_owner || accessor_id == asset.owner_id.as_deref();

    if !asset.is_accessible_by(accessor_id, is_owner) {
        // R4-05: Return 401 for anonymous access to non-public files
        if accessor_id.is_none() && !visibility.allows_anonymous() {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }

        // R4-05: Return 403 for authenticated but unauthorized access
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied",
            "code": "FORBIDDEN"
        }));
    }

    // Asset is accessible, serve the file
    serve_untracked_file(&resource_id, &filename).await
}

/// Internal helper to serve a file without visibility checks
async fn serve_untracked_file(resource_id: &str, filename: &str) -> HttpResponse {
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

    let storage_url = format!("local://{resource_id}/{filename}");

    match client.get_file(&storage_url).await {
        Ok(data) => {
            let mime_type = mime_from_extension(filename);

            HttpResponse::Ok()
                .insert_header((header::CONTENT_TYPE, mime_type))
                .insert_header((header::CACHE_CONTROL, "public, max-age=31536000, immutable"))
                .insert_header((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
                .insert_header((
                    header::CONTENT_SECURITY_POLICY,
                    "default-src 'none'; style-src 'unsafe-inline'",
                ))
                .insert_header((header::X_FRAME_OPTIONS, "DENY"))
                .body(data)
        }
        Err(e) => {
            let status_code = e.status_code();
            match status_code {
                404 => HttpResponse::NotFound().json(serde_json::json!({
                    "error": "File not found",
                    "code": "NOT_FOUND"
                })),
                403 => HttpResponse::Forbidden().json(serde_json::json!({
                    "error": "Access denied",
                    "code": "FORBIDDEN"
                })),
                _ => {
                    tracing::error!("Failed to serve file: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to retrieve file",
                        "code": "INTERNAL_ERROR"
                    }))
                }
            }
        }
    }
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
pub async fn get_display_url(query: web::Query<DisplayUrlQuery>) -> HttpResponse {
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
pub async fn batch_display_urls(body: web::Json<BatchDisplayUrlRequest>) -> HttpResponse {
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
pub async fn upload_files(mut payload: actix_multipart::Multipart) -> HttpResponse {
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

    let mut total_bytes = 0usize;
    let mut field_count = 0usize;

    // Enforce limits before extending buffers, including resource-id fields.
    while let Some(item) = payload.next().await {
        field_count += 1;
        if field_count > 16 {
            return HttpResponse::PayloadTooLarge().finish();
        }
        let mut field = match item {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Failed to read multipart field: {}", e);
                return HttpResponse::BadRequest().finish();
            }
        };

        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "resource_id" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    let bytes = match chunk {
                        Ok(bytes) => bytes,
                        Err(_) => return HttpResponse::BadRequest().finish(),
                    };
                    if bytes.len() > 128 - data.len()
                        || bytes.len() > 20 * 1024 * 1024 - total_bytes
                    {
                        return HttpResponse::PayloadTooLarge().finish();
                    }
                    total_bytes += bytes.len();
                    data.extend_from_slice(&bytes);
                }
                if resource_id.is_some() {
                    return HttpResponse::BadRequest().finish();
                }
                match String::from_utf8(data) {
                    Ok(id) => resource_id = Some(id.trim().to_string()),
                    Err(_) => return HttpResponse::BadRequest().finish(),
                }
            }
            "file" | "files" => {
                if files.len() >= 8 {
                    return HttpResponse::PayloadTooLarge().finish();
                }
                let filename = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
                    .unwrap_or_else(|| "unknown".to_string());

                let content_type = field
                    .content_type()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "application/octet-stream".to_string());

                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    let bytes = match chunk {
                        Ok(bytes) => bytes,
                        Err(_) => return HttpResponse::BadRequest().finish(),
                    };
                    if bytes.len() > 10 * 1024 * 1024 - data.len()
                        || bytes.len() > 20 * 1024 * 1024 - total_bytes
                    {
                        return HttpResponse::PayloadTooLarge().finish();
                    }
                    total_bytes += bytes.len();
                    data.extend_from_slice(&bytes);
                }

                if !data.is_empty() {
                    files.push((data, filename, content_type));
                }
            }
            _ => {
                return HttpResponse::BadRequest().finish();
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

    tracing::info!(
        "Uploading {} files for resource {}",
        files.len(),
        resource_id
    );

    match client.upload_files(&resource_id, files, None).await {
        Ok(result) => {
            tracing::info!(
                "Upload complete: {} files, {} bytes",
                result.uploaded_count,
                result.total_bytes
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
pub async fn delete_files(path: web::Path<String>) -> HttpResponse {
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

// ============================================
// TRACKED ASSET ENDPOINTS (P6-01/P6-02)
// ============================================

/// Request for tracked upload
#[derive(Debug, serde::Deserialize)]
pub struct TrackedUploadQuery {
    pub resource_id: Option<String>,
    pub owner_id: Option<String>,
    pub owner_type: Option<String>,
    pub category: Option<String>,
    pub alt_text: Option<String>,
    /// R4-05: Visibility setting for new uploads (defaults to draft)
    pub visibility: Option<String>,
}

/// Response for tracked upload
#[derive(Debug, serde::Serialize)]
pub struct TrackedUploadResponse {
    pub asset_id: String,
    pub storage_url: String,
    pub display_url: String,
    pub file_size: i64,
    pub sha256_hash: String,
}

/// POST /api/admin/files/tracked-upload
/// Upload with DB tracking, quota enforcement, and hash verification
pub async fn tracked_upload(
    mut payload: actix_multipart::Multipart,
    query: web::Query<TrackedUploadQuery>,
    media_service: web::Data<DynMediaAssetService>,
) -> HttpResponse {
    use futures_util::StreamExt;

    // Extract file from multipart
    let mut file_data: Option<(Vec<u8>, String, String)> = None;
    let mut total_bytes = 0usize;

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Failed to read multipart field: {}", e);
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid multipart data",
                    "code": "INVALID_REQUEST"
                }));
            }
        };

        let field_name = field.name().unwrap_or("").to_string();
        if field_name == "file" {
            let filename = field
                .content_disposition()
                .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
                .unwrap_or_else(|| "unknown".to_string());

            let content_type = field
                .content_type()
                .map(|m| m.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_string());

            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                let bytes = match chunk {
                    Ok(bytes) => bytes,
                    Err(_) => return HttpResponse::BadRequest().finish(),
                };
                // 50MB limit for tracked uploads
                if bytes.len() > 50 * 1024 * 1024 - data.len() {
                    return HttpResponse::PayloadTooLarge().json(serde_json::json!({
                        "error": "File too large",
                        "code": "FILE_TOO_LARGE"
                    }));
                }
                total_bytes += bytes.len();
                data.extend_from_slice(&bytes);
            }

            if !data.is_empty() {
                file_data = Some((data, filename, content_type));
            }
        }
    }

    let (data, filename, content_type) = match file_data {
        Some(f) => f,
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "No file provided",
                "code": "MISSING_FILE"
            }));
        }
    };

    let resource_id = query.resource_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());

    // R4-05: Parse visibility from query parameter
    let visibility = query.visibility.as_deref()
        .and_then(AssetVisibility::from_str);

    let request = MediaUploadRequest {
        resource_id,
        original_filename: filename,
        content_type,
        data,
        owner_id: query.owner_id.clone(),
        owner_type: query.owner_type.clone(),
        category: query.category.clone(),
        alt_text: query.alt_text.clone(),
        created_by: None, // Could be extracted from auth
        visibility,
    };

    match media_service.upload_with_tracking(request).await {
        Ok(result) => {
            HttpResponse::Created().json(TrackedUploadResponse {
                asset_id: result.asset_id.to_string(),
                storage_url: result.storage_url,
                display_url: result.display_url,
                file_size: result.file_size,
                sha256_hash: result.sha256_hash,
            })
        }
        Err(e) => {
            tracing::error!("Tracked upload failed: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Upload failed: {}", e),
                "code": "UPLOAD_FAILED"
            }))
        }
    }
}

/// Query parameters for quota lookup
#[derive(Debug, serde::Deserialize)]
pub struct QuotaQuery {
    pub owner_type: Option<String>,
}

/// GET /api/admin/files/quota/{owner_id}
/// Get quota status for an owner
pub async fn get_quota(
    path: web::Path<String>,
    query: web::Query<QuotaQuery>,
    media_service: web::Data<DynMediaAssetService>,
) -> HttpResponse {
    let owner_id = path.into_inner();
    let owner_type = query.owner_type.as_deref().unwrap_or("user");

    match media_service.get_quota(&owner_id, owner_type).await {
        Ok(Some(quota)) => {
            HttpResponse::Ok().json(serde_json::json!({
                "owner_id": quota.owner_id,
                "max_total_bytes": quota.max_total_bytes,
                "max_file_size": quota.max_file_size,
                "max_file_count": quota.max_file_count,
                "current_bytes": quota.current_bytes,
                "current_count": quota.current_count,
                "usage_percent": if quota.max_total_bytes > 0 {
                    (quota.current_bytes as f64 / quota.max_total_bytes as f64 * 100.0).round() as i32
                } else {
                    0
                }
            }))
        }
        Ok(None) => {
            HttpResponse::Ok().json(serde_json::json!({
                "owner_id": owner_id,
                "max_total_bytes": 104857600, // 100MB default
                "max_file_size": 10485760,    // 10MB default
                "max_file_count": 50,
                "current_bytes": 0,
                "current_count": 0,
                "usage_percent": 0
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get quota: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to get quota: {}", e),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/admin/files/disk-status
/// Get disk threshold status (P6-08)
pub async fn get_disk_status(
    media_service: web::Data<DynMediaAssetService>,
) -> HttpResponse {
    match media_service.check_disk_thresholds().await {
        Ok(status) => {
            HttpResponse::Ok().json(serde_json::json!({
                "mount_path": status.mount_path,
                "total_bytes": status.total_bytes,
                "available_bytes": status.available_bytes,
                "used_percent": status.used_percent,
                "status": format!("{:?}", status.status),
                "is_healthy": status.status == crate::server::services::ThresholdLevel::Ok
            }))
        }
        Err(e) => {
            tracing::error!("Failed to check disk status: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to check disk status: {}", e),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/admin/files/reconcile
/// Reconcile DB with filesystem (find missing/orphaned files)
pub async fn reconcile_assets(
    media_service: web::Data<DynMediaAssetService>,
) -> HttpResponse {
    match media_service.reconcile_assets().await {
        Ok(result) => {
            HttpResponse::Ok().json(serde_json::json!({
                "assets_checked": result.assets_checked,
                "marked_missing": result.marked_missing,
                "marked_ready": result.marked_ready,
                "orphaned_files": result.orphaned_files,
            }))
        }
        Err(e) => {
            tracing::error!("Reconciliation failed: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Reconciliation failed: {}", e),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// Configure file storage routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Public file serving for local storage (legacy, no visibility check)
    // GET /files/{resource_id}/{filename}
    cfg.service(web::scope("/files")
        // R4-05: Visibility-checked endpoint (preferred)
        .route("/v/{resource_id}/{filename:.*}", web::get().to(serve_file_with_visibility))
        // Legacy endpoint (backwards compatibility)
        .route("/{resource_id}/{filename:.*}", web::get().to(serve_local_file)),
    );

    // API routes for storage URL management
    cfg.service(
        web::scope("/api/files")
            .route("/display-url", web::get().to(get_display_url))
            .route("/display-urls", web::post().to(batch_display_urls))
            .route("/serve/{storage_url:.*}", web::get().to(get_file)),
    );

    // Admin file routes - protected by AdminAuth
    cfg.service(
        web::scope("/api/admin/files")
            .wrap(AdminAuth::from_env().expect("ADMIN_API_KEY validated at startup"))
            .route("/upload", web::post().to(upload_files))
            .route("/tracked-upload", web::post().to(tracked_upload))
            .route("/quota/{owner_id}", web::get().to(get_quota))
            .route("/disk-status", web::get().to(get_disk_status))
            .route("/reconcile", web::post().to(reconcile_assets))
            .route("/{resource_id}", web::delete().to(delete_files)),
    );
}
