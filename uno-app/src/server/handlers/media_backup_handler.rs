//! Media Backup/Restore API handlers
//!
//! R4-06: Provides endpoints for:
//! - Creating backups
//! - Restoring from backups
//! - Verifying backup integrity
//! - Listing and querying backups

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::middleware::AdminAuth;
use crate::server::services::{
    DynMediaBackupService, BackupRequest, RestoreRequest,
};

// ============================================
// REQUEST/RESPONSE TYPES
// ============================================

/// Request to create a backup
#[derive(Debug, Deserialize)]
pub struct CreateBackupRequest {
    /// Descriptive name for the backup
    pub name: Option<String>,
    /// Backup type: full, incremental, differential
    #[serde(default = "default_backup_type")]
    pub backup_type: String,
    /// Destination path for the backup
    pub destination_path: String,
}

fn default_backup_type() -> String {
    "full".to_string()
}

/// Response from backup creation
#[derive(Debug, Serialize)]
pub struct CreateBackupResponse {
    pub backup_id: String,
    pub status: String,
    pub message: String,
}

/// Request to restore from a backup
#[derive(Debug, Deserialize)]
pub struct CreateRestoreRequest {
    /// Actor initiating the restore
    pub initiated_by: String,
}

/// Response from restore operation
#[derive(Debug, Serialize)]
pub struct RestoreResponse {
    pub restore_id: String,
    pub backup_id: String,
    pub status: String,
    pub message: String,
}

/// Backup status response
#[derive(Debug, Serialize)]
pub struct BackupStatusResponse {
    pub id: String,
    pub name: String,
    pub backup_type: String,
    pub status: String,
    pub total_files: i32,
    pub total_bytes: i64,
    pub processed_files: i32,
    pub archive_hash: Option<String>,
    pub rpo_seconds: Option<i32>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub error_message: Option<String>,
}

/// Verification response
#[derive(Debug, Serialize)]
pub struct VerifyBackupResponse {
    pub is_valid: bool,
    pub entries_verified: i32,
    pub entries_corrupted: i32,
    pub errors: Vec<String>,
}

/// List backups query parameters
#[derive(Debug, Deserialize)]
pub struct ListBackupsQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ============================================
// HANDLERS
// ============================================

/// POST /api/v1/admin/backups
/// Create a new backup
pub async fn create_backup(
    body: web::Json<CreateBackupRequest>,
    backup_service: web::Data<DynMediaBackupService>,
) -> HttpResponse {
    let request = BackupRequest {
        name: body.name.clone().unwrap_or_else(|| {
            format!("backup-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"))
        }),
        backup_type: match body.backup_type.as_str() {
            "incremental" => crate::server::repositories::BackupType::Incremental,
            "differential" => crate::server::repositories::BackupType::Differential,
            _ => crate::server::repositories::BackupType::Full,
        },
        destination_path: body.destination_path.clone(),
        encrypt: false,
        encryption_key_id: None,
        created_by: Some("admin".to_string()),
    };

    match backup_service.create_backup(request).await {
        Ok(result) => {
            HttpResponse::Accepted().json(CreateBackupResponse {
                backup_id: result.backup_id.to_string(),
                status: "created".to_string(),
                message: format!(
                    "Backup created with {} files ({} bytes)",
                    result.total_files, result.total_bytes
                ),
            })
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to create backup");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Backup failed: {}", e),
                "code": "BACKUP_FAILED"
            }))
        }
    }
}

/// POST /api/v1/admin/backups/{id}/restore
/// Restore from a backup
pub async fn restore_backup(
    path: web::Path<String>,
    body: web::Json<CreateRestoreRequest>,
    backup_service: web::Data<DynMediaBackupService>,
) -> HttpResponse {
    let backup_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid backup ID",
                "code": "INVALID_ID"
            }));
        }
    };

    let request = RestoreRequest {
        backup_id,
        restore_path: String::new(), // Empty = restore to original location
        verify_hashes: true,
        created_by: Some(body.initiated_by.clone()),
    };

    match backup_service.restore_backup(request).await {
        Ok(result) => {
            HttpResponse::Accepted().json(RestoreResponse {
                restore_id: result.restore_id.to_string(),
                backup_id: backup_id.to_string(),
                status: if result.failed_files == 0 { "completed" } else { "partial" }.to_string(),
                message: format!(
                    "Restored {} files ({} bytes) in {} seconds",
                    result.restored_files, result.restored_bytes, result.rto_seconds
                ),
            })
        }
        Err(e) => {
            tracing::error!(backup_id = %backup_id, error = %e, "Failed to restore backup");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Restore failed: {}", e),
                "code": "RESTORE_FAILED"
            }))
        }
    }
}

/// GET /api/v1/admin/backups/{id}/verify
/// Verify backup integrity without restoring
pub async fn verify_backup(
    path: web::Path<String>,
    backup_service: web::Data<DynMediaBackupService>,
) -> HttpResponse {
    let backup_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid backup ID",
                "code": "INVALID_ID"
            }));
        }
    };

    match backup_service.verify_backup(backup_id).await {
        Ok(result) => {
            HttpResponse::Ok().json(VerifyBackupResponse {
                is_valid: result.is_valid,
                entries_verified: result.entries_verified,
                entries_corrupted: result.entries_corrupted,
                errors: result.errors,
            })
        }
        Err(e) => {
            tracing::error!(backup_id = %backup_id, error = %e, "Failed to verify backup");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Verification failed: {}", e),
                "code": "VERIFY_FAILED"
            }))
        }
    }
}

/// GET /api/v1/admin/backups/{id}
/// Get backup status
pub async fn get_backup(
    path: web::Path<String>,
    backup_service: web::Data<DynMediaBackupService>,
) -> HttpResponse {
    let backup_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid backup ID",
                "code": "INVALID_ID"
            }));
        }
    };

    match backup_service.get_backup(backup_id).await {
        Ok(Some(backup)) => {
            HttpResponse::Ok().json(BackupStatusResponse {
                id: backup.id.to_string(),
                name: backup.backup_name,
                backup_type: backup.backup_type,
                status: backup.status,
                total_files: backup.total_files,
                total_bytes: backup.total_bytes,
                processed_files: backup.processed_files,
                archive_hash: backup.archive_hash,
                rpo_seconds: backup.rpo_seconds,
                started_at: backup.started_at.map(|t| t.to_rfc3339()),
                completed_at: backup.completed_at.map(|t| t.to_rfc3339()),
                error_message: backup.error_message,
            })
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Backup not found",
                "code": "NOT_FOUND"
            }))
        }
        Err(e) => {
            tracing::error!(backup_id = %backup_id, error = %e, "Failed to get backup");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to get backup: {}", e),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/admin/backups
/// List recent backups
pub async fn list_backups(
    query: web::Query<ListBackupsQuery>,
    backup_service: web::Data<DynMediaBackupService>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(20).min(100);

    match backup_service.list_backups(limit).await {
        Ok(backups) => {
            let response: Vec<BackupStatusResponse> = backups
                .into_iter()
                .map(|b| BackupStatusResponse {
                    id: b.id.to_string(),
                    name: b.backup_name,
                    backup_type: b.backup_type,
                    status: b.status,
                    total_files: b.total_files,
                    total_bytes: b.total_bytes,
                    processed_files: b.processed_files,
                    archive_hash: b.archive_hash,
                    rpo_seconds: b.rpo_seconds,
                    started_at: b.started_at.map(|t| t.to_rfc3339()),
                    completed_at: b.completed_at.map(|t| t.to_rfc3339()),
                    error_message: b.error_message,
                })
                .collect();

            HttpResponse::Ok().json(serde_json::json!({
                "backups": response,
                "total": response.len()
            }))
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to list backups");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to list backups: {}", e),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/admin/restores/{id}
/// Get restore status
pub async fn get_restore(
    path: web::Path<String>,
    backup_service: web::Data<DynMediaBackupService>,
) -> HttpResponse {
    let restore_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid restore ID",
                "code": "INVALID_ID"
            }));
        }
    };

    match backup_service.get_restore(restore_id).await {
        Ok(Some(restore)) => {
            HttpResponse::Ok().json(serde_json::json!({
                "id": restore.id.to_string(),
                "backup_id": restore.backup_id.to_string(),
                "status": restore.status,
                "total_files": restore.total_files,
                "restored_files": restore.restored_files,
                "failed_files": restore.failed_files,
                "rto_seconds": restore.rto_seconds,
                "started_at": restore.started_at.map(|t| t.to_rfc3339()),
                "completed_at": restore.completed_at.map(|t| t.to_rfc3339()),
                "error_message": restore.error_message,
            }))
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Restore not found",
                "code": "NOT_FOUND"
            }))
        }
        Err(e) => {
            tracing::error!(restore_id = %restore_id, error = %e, "Failed to get restore");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to get restore: {}", e),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

// ============================================
// ROUTE CONFIGURATION
// ============================================

/// Configure backup/restore routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin/backups")
            .wrap(AdminAuth::from_env().expect("ADMIN_API_KEY validated at startup"))
            .route("", web::post().to(create_backup))
            .route("", web::get().to(list_backups))
            .route("/{id}", web::get().to(get_backup))
            .route("/{id}/restore", web::post().to(restore_backup))
            .route("/{id}/verify", web::get().to(verify_backup)),
    );

    cfg.service(
        web::scope("/api/v1/admin/restores")
            .wrap(AdminAuth::from_env().expect("ADMIN_API_KEY validated at startup"))
            .route("/{id}", web::get().to(get_restore)),
    );
}
