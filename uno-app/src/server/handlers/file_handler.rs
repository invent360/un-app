//! File API handlers for storage (local filesystem and cloud)
//!
//! Provides endpoints for uploading files and serving them.
//! For local storage, files are served directly with proper security headers.
//! For cloud storage, files are redirected to signed URLs.
//!
//! R5-10: All visibility checks derive identity from authenticated principal,
//! never from query parameters. Untracked assets default to private.

use crate::server::extractors::auth::get_authenticated_user;
use crate::server::middleware::AdminAuth;
use crate::server::services::{
    DynMediaAssetService, UploadRequest as MediaUploadRequest,
};
use actix_web::{http::header, web, HttpRequest, HttpResponse};
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
/// R5-10: This legacy route now enforces visibility checks.
/// All file serving must go through visibility enforcement - no bypass routes.
/// Redirects to serve_file_with_visibility for proper access control.
pub async fn serve_local_file(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    media_repo: web::Data<DynMediaAssetRepository>,
) -> HttpResponse {
    // R5-10: Delegate to visibility-checked handler - no bypass allowed
    serve_file_with_visibility(req, path, media_repo).await
}

// R5-10: get_file endpoint REMOVED
// This endpoint previously served files at /api/files/serve/{storage_url} with NO visibility checks.
// This was a critical security vulnerability - anyone could access any file by providing a storage URL.
// All file serving must now go through visibility-checked endpoints:
// - /files/v/{resource_id}/{filename} - visibility-enforced serving
// - /files/{resource_id}/{filename} - legacy route that delegates to visibility-checked handler
//
// If you need to serve cloud storage files, use the display-url endpoints to get signed URLs
// which have appropriate time-limited access built in.

// ============================================
// R4-05: VISIBILITY-CHECKED FILE ACCESS
// ============================================

use crate::server::repositories::{DynMediaAssetRepository, AssetVisibility};

/// GET /files/v/{resource_id}/{filename}
/// R4-05, R5-10: Serves files with visibility policy enforcement
/// R5-10: Identity is derived from authenticated principal, never from query params
/// R5-10: Untracked assets default to private, not public
pub async fn serve_file_with_visibility(
    req: HttpRequest,
    path: web::Path<(String, String)>,
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

    // R5-10: Derive accessor identity from authenticated principal, NOT from query params
    let authenticated_user = get_authenticated_user(&req).ok();
    let accessor_id = authenticated_user.as_ref().map(|u| u.id.as_str());

    // R4-05: Look up asset and check visibility
    let asset = match media_repo.get_asset_by_storage_url(&storage_url).await {
        Ok(Some(asset)) => asset,
        Ok(None) => {
            // R5-10: Untracked assets default to PRIVATE, not public
            // Only authenticated owners can access untracked assets
            tracing::warn!(storage_url = %storage_url, "Asset not tracked - treating as private");
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Asset not found or access denied",
                "code": "NOT_FOUND"
            }));
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to look up asset");
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check asset visibility",
                "code": "INTERNAL_ERROR"
            }));
        }
    };

    // R5-10: Check visibility policy - is_owner derived from auth + asset.owner_id
    let visibility = asset.visibility_enum();
    let is_owner = accessor_id.is_some() && accessor_id == asset.owner_id.as_deref();

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

    // Asset is accessible, serve the file with appropriate cache headers
    // R5-10: Private assets get no-store cache headers
    serve_file_with_cache_policy(&resource_id, &filename, &visibility).await
}

/// Internal helper to serve a file with appropriate cache policy
/// R5-10: Private responses must not be cached publicly
async fn serve_file_with_cache_policy(
    resource_id: &str,
    filename: &str,
    visibility: &AssetVisibility,
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

    let storage_url = format!("local://{resource_id}/{filename}");

    match client.get_file(&storage_url).await {
        Ok(data) => {
            let mime = mime_from_extension(filename);

            // R5-10: Use appropriate cache policy based on visibility
            let cache_control = match visibility {
                AssetVisibility::Public => {
                    // Public assets can be cached for 1 year (immutable)
                    "public, max-age=31536000, immutable"
                }
                AssetVisibility::Private | AssetVisibility::Draft => {
                    // Private/draft assets must not be cached in shared caches
                    "private, no-store, no-cache, must-revalidate"
                }
            };

            HttpResponse::Ok()
                .content_type(mime)
                .insert_header((header::CACHE_CONTROL, cache_control))
                .insert_header((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
                .body(data)
        }
        Err(e) => {
            tracing::error!(error = %e, storage_url = %storage_url, "Failed to get file");
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "File not found",
                "code": "NOT_FOUND"
            }))
        }
    }
}

// R5-10: serve_untracked_file removed - all file serving must go through visibility checks

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
/// R5-10: owner_id removed - derived from authenticated principal
#[derive(Debug, serde::Deserialize)]
pub struct TrackedUploadQuery {
    pub resource_id: Option<String>,
    // R5-10: owner_id removed - derived from authenticated principal
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
/// R5-10: owner_id and created_by derived from authenticated principal
pub async fn tracked_upload(
    req: HttpRequest,
    mut payload: actix_multipart::Multipart,
    query: web::Query<TrackedUploadQuery>,
    media_service: web::Data<DynMediaAssetService>,
) -> HttpResponse {
    // R5-10: Extract identity from authenticated principal, not query params
    let authenticated_user = match get_authenticated_user(&req) {
        Ok(user) => user,
        Err(e) => {
            tracing::warn!("Tracked upload requires authentication: {}", e);
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "code": "UNAUTHORIZED"
            }));
        }
    };
    let actor_id = authenticated_user.id.clone();
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

    // R5-10: Owner/creator derived from authenticated principal, not query params
    let request = MediaUploadRequest {
        resource_id,
        original_filename: filename,
        content_type,
        data,
        owner_id: Some(actor_id.clone()),
        owner_type: query.owner_type.clone(),
        category: query.category.clone(),
        alt_text: query.alt_text.clone(),
        created_by: Some(actor_id),
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
    // File serving with visibility enforcement
    // R5-10: Both routes now enforce visibility - no bypass routes allowed
    cfg.service(web::scope("/files")
        // R4-05: Visibility-checked endpoint (explicit path)
        .route("/v/{resource_id}/{filename:.*}", web::get().to(serve_file_with_visibility))
        // R5-10: Legacy endpoint now delegates to visibility-checked handler
        .route("/{resource_id}/{filename:.*}", web::get().to(serve_local_file)),
    );

    // API routes for storage URL management
    // R5-10: /serve/{storage_url} route REMOVED - was a visibility bypass vulnerability
    cfg.service(
        web::scope("/api/files")
            .route("/display-url", web::get().to(get_display_url))
            .route("/display-urls", web::post().to(batch_display_urls)),
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
