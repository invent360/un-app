//! Media backup repository
//!
//! Provides:
//! - Backup job tracking
//! - Manifest entries
//! - Restore operations
//! - Storage health metrics

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Backup status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl BackupStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "running" => Some(Self::Running),
            "completed" => Some(Self::Completed),
            "failed" => Some(Self::Failed),
            "cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }
}

/// Backup type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
}

impl BackupType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Incremental => "incremental",
            Self::Differential => "differential",
        }
    }
}

/// Restore status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreStatus {
    Pending,
    Running,
    Verifying,
    Completed,
    Failed,
    Partial,
}

impl RestoreStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Verifying => "verifying",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Partial => "partial",
        }
    }
}

/// Media backup entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaBackup {
    pub id: Uuid,
    pub backup_name: String,
    pub backup_type: String,
    pub source_path: String,
    pub destination_path: String,
    pub encryption_key_id: Option<String>,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub total_files: i32,
    pub processed_files: i32,
    pub total_bytes: i64,
    pub processed_bytes: i64,
    pub skipped_files: i32,
    pub manifest_hash: Option<String>,
    pub archive_hash: Option<String>,
    pub base_backup_id: Option<Uuid>,
    pub rpo_seconds: Option<i32>,
    pub estimated_rto_seconds: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

/// Backup entry entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaBackupEntry {
    pub id: Uuid,
    pub backup_id: Uuid,
    pub asset_id: Uuid,
    pub relative_path: String,
    pub file_size: i64,
    pub sha256_hash: String,
    pub mime_type: Option<String>,
    pub compressed_size: Option<i64>,
    pub encrypted: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub verification_status: Option<String>,
    pub backed_up_at: DateTime<Utc>,
    // B4 FIX: Fields for manifest hash verification
    pub original_filename: Option<String>,
    pub original_storage_url: Option<String>,
}

/// Media restore entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaRestore {
    pub id: Uuid,
    pub backup_id: Uuid,
    pub restore_path: String,
    pub restore_mode: String,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub total_files: i32,
    pub restored_files: i32,
    pub failed_files: i32,
    pub total_bytes: i64,
    pub restored_bytes: i64,
    pub rto_seconds: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

/// Storage health metrics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StorageHealthMetric {
    pub id: Uuid,
    pub mount_path: String,
    pub volume_label: Option<String>,
    pub total_bytes: i64,
    pub used_bytes: i64,
    pub available_bytes: i64,
    pub inode_total: Option<i64>,
    pub inode_used: Option<i64>,
    pub read_latency_ms: Option<i32>,
    pub write_latency_ms: Option<i32>,
    pub is_healthy: bool,
    pub health_notes: Option<String>,
    pub recorded_at: DateTime<Utc>,
}

// ============================================
// INPUT STRUCTS
// ============================================

/// Input for creating a backup job
#[derive(Debug, Clone)]
pub struct CreateBackupInput {
    pub backup_name: String,
    pub backup_type: BackupType,
    pub source_path: String,
    pub destination_path: String,
    pub encryption_key_id: Option<String>,
    pub base_backup_id: Option<Uuid>,
    pub created_by: Option<String>,
}

/// Input for creating a backup entry
#[derive(Debug, Clone)]
pub struct CreateBackupEntryInput {
    pub backup_id: Uuid,
    pub asset_id: Uuid,
    pub relative_path: String,
    pub file_size: i64,
    pub sha256_hash: String,
    pub mime_type: Option<String>,
    pub compressed_size: Option<i64>,
    pub encrypted: bool,
    // B4 FIX: Fields for manifest hash verification
    pub original_filename: Option<String>,
    pub original_storage_url: Option<String>,
}

/// Input for creating a restore job
#[derive(Debug, Clone)]
pub struct CreateRestoreInput {
    pub backup_id: Uuid,
    pub restore_path: String,
    pub restore_mode: String,
    pub created_by: Option<String>,
}

/// Input for recording storage metrics
#[derive(Debug, Clone)]
pub struct RecordStorageMetricsInput {
    pub mount_path: String,
    pub volume_label: Option<String>,
    pub total_bytes: i64,
    pub used_bytes: i64,
    pub available_bytes: i64,
    pub inode_total: Option<i64>,
    pub inode_used: Option<i64>,
    pub read_latency_ms: Option<i32>,
    pub write_latency_ms: Option<i32>,
    pub is_healthy: bool,
    pub health_notes: Option<String>,
}

// ============================================
// TRAIT DEFINITION
// ============================================

pub type DynMediaBackupRepository = Arc<dyn MediaBackupRepository + Send + Sync>;

#[async_trait]
pub trait MediaBackupRepository: Send + Sync {
    // Backup jobs
    async fn create_backup(&self, input: CreateBackupInput) -> Result<MediaBackup, AppError>;
    async fn get_backup(&self, id: Uuid) -> Result<Option<MediaBackup>, AppError>;
    async fn start_backup(&self, id: Uuid) -> Result<(), AppError>;
    async fn update_backup_progress(&self, id: Uuid, processed_files: i32, processed_bytes: i64) -> Result<(), AppError>;
    async fn complete_backup(&self, id: Uuid, manifest_hash: &str, archive_hash: &str, rpo_seconds: i32) -> Result<(), AppError>;
    async fn fail_backup(&self, id: Uuid, error: &str) -> Result<(), AppError>;
    async fn list_backups(&self, limit: i32) -> Result<Vec<MediaBackup>, AppError>;
    async fn get_latest_full_backup(&self) -> Result<Option<MediaBackup>, AppError>;

    // Backup entries
    async fn create_entry(&self, input: CreateBackupEntryInput) -> Result<MediaBackupEntry, AppError>;
    async fn get_entries(&self, backup_id: Uuid) -> Result<Vec<MediaBackupEntry>, AppError>;
    async fn verify_entry(&self, entry_id: Uuid, status: &str) -> Result<(), AppError>;
    async fn get_entry_count(&self, backup_id: Uuid) -> Result<i64, AppError>;

    // Restore jobs
    async fn create_restore(&self, input: CreateRestoreInput) -> Result<MediaRestore, AppError>;
    async fn get_restore(&self, id: Uuid) -> Result<Option<MediaRestore>, AppError>;
    async fn start_restore(&self, id: Uuid) -> Result<(), AppError>;
    async fn update_restore_progress(&self, id: Uuid, restored_files: i32, restored_bytes: i64, failed_files: i32) -> Result<(), AppError>;
    async fn complete_restore(&self, id: Uuid, rto_seconds: i32) -> Result<(), AppError>;
    async fn fail_restore(&self, id: Uuid, error: &str) -> Result<(), AppError>;

    // Storage health
    async fn record_storage_metrics(&self, input: RecordStorageMetricsInput) -> Result<StorageHealthMetric, AppError>;
    async fn get_latest_storage_metrics(&self, mount_path: &str) -> Result<Option<StorageHealthMetric>, AppError>;
    async fn get_unhealthy_mounts(&self) -> Result<Vec<StorageHealthMetric>, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct MediaBackupRepositoryImpl {
    pool: ConnectionPool,
}

impl MediaBackupRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MediaBackupRepository for MediaBackupRepositoryImpl {
    async fn create_backup(&self, input: CreateBackupInput) -> Result<MediaBackup, AppError> {
        let backup = sqlx::query_as::<_, MediaBackup>(
            r#"
            INSERT INTO media_backups (
                backup_name, backup_type, source_path, destination_path,
                encryption_key_id, base_backup_id, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, backup_name, backup_type, source_path, destination_path,
                      encryption_key_id, status, started_at, completed_at, error_message,
                      total_files, processed_files, total_bytes, processed_bytes, skipped_files,
                      manifest_hash, archive_hash, base_backup_id, rpo_seconds, estimated_rto_seconds,
                      created_at, created_by
            "#,
        )
        .bind(&input.backup_name)
        .bind(input.backup_type.as_str())
        .bind(&input.source_path)
        .bind(&input.destination_path)
        .bind(&input.encryption_key_id)
        .bind(input.base_backup_id)
        .bind(&input.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(backup_id = %backup.id, name = %backup.backup_name, "Backup job created");
        Ok(backup)
    }

    async fn get_backup(&self, id: Uuid) -> Result<Option<MediaBackup>, AppError> {
        let result = sqlx::query_as::<_, MediaBackup>(
            r#"
            SELECT id, backup_name, backup_type, source_path, destination_path,
                   encryption_key_id, status, started_at, completed_at, error_message,
                   total_files, processed_files, total_bytes, processed_bytes, skipped_files,
                   manifest_hash, archive_hash, base_backup_id, rpo_seconds, estimated_rto_seconds,
                   created_at, created_by
            FROM media_backups WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn start_backup(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE media_backups SET status = 'running', started_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn update_backup_progress(&self, id: Uuid, processed_files: i32, processed_bytes: i64) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE media_backups SET processed_files = $2, processed_bytes = $3 WHERE id = $1"
        )
        .bind(id)
        .bind(processed_files)
        .bind(processed_bytes)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn complete_backup(&self, id: Uuid, manifest_hash: &str, archive_hash: &str, rpo_seconds: i32) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE media_backups SET
                status = 'completed',
                completed_at = NOW(),
                manifest_hash = $2,
                archive_hash = $3,
                rpo_seconds = $4
            WHERE id = $1
            "#
        )
        .bind(id)
        .bind(manifest_hash)
        .bind(archive_hash)
        .bind(rpo_seconds)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(backup_id = %id, rpo_seconds = rpo_seconds, "Backup completed");
        Ok(())
    }

    async fn fail_backup(&self, id: Uuid, error: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE media_backups SET status = 'failed', error_message = $2 WHERE id = $1")
            .bind(id)
            .bind(error)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::error!(backup_id = %id, error = %error, "Backup failed");
        Ok(())
    }

    async fn list_backups(&self, limit: i32) -> Result<Vec<MediaBackup>, AppError> {
        let results = sqlx::query_as::<_, MediaBackup>(
            r#"
            SELECT id, backup_name, backup_type, source_path, destination_path,
                   encryption_key_id, status, started_at, completed_at, error_message,
                   total_files, processed_files, total_bytes, processed_bytes, skipped_files,
                   manifest_hash, archive_hash, base_backup_id, rpo_seconds, estimated_rto_seconds,
                   created_at, created_by
            FROM media_backups
            ORDER BY created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn get_latest_full_backup(&self) -> Result<Option<MediaBackup>, AppError> {
        let result = sqlx::query_as::<_, MediaBackup>(
            r#"
            SELECT id, backup_name, backup_type, source_path, destination_path,
                   encryption_key_id, status, started_at, completed_at, error_message,
                   total_files, processed_files, total_bytes, processed_bytes, skipped_files,
                   manifest_hash, archive_hash, base_backup_id, rpo_seconds, estimated_rto_seconds,
                   created_at, created_by
            FROM media_backups
            WHERE backup_type = 'full' AND status = 'completed'
            ORDER BY completed_at DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn create_entry(&self, input: CreateBackupEntryInput) -> Result<MediaBackupEntry, AppError> {
        // B4 FIX: Include original_filename and original_storage_url for manifest verification
        let entry = sqlx::query_as::<_, MediaBackupEntry>(
            r#"
            INSERT INTO media_backup_entries (
                backup_id, asset_id, relative_path, file_size, sha256_hash,
                mime_type, compressed_size, encrypted, original_filename, original_storage_url
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, backup_id, asset_id, relative_path, file_size, sha256_hash,
                      mime_type, compressed_size, encrypted, verified_at, verification_status, backed_up_at,
                      original_filename, original_storage_url
            "#,
        )
        .bind(input.backup_id)
        .bind(input.asset_id)
        .bind(&input.relative_path)
        .bind(input.file_size)
        .bind(&input.sha256_hash)
        .bind(&input.mime_type)
        .bind(input.compressed_size)
        .bind(input.encrypted)
        .bind(&input.original_filename)
        .bind(&input.original_storage_url)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(entry)
    }

    async fn get_entries(&self, backup_id: Uuid) -> Result<Vec<MediaBackupEntry>, AppError> {
        // B4 FIX: Include original_filename and original_storage_url for manifest verification
        let results = sqlx::query_as::<_, MediaBackupEntry>(
            r#"
            SELECT id, backup_id, asset_id, relative_path, file_size, sha256_hash,
                   mime_type, compressed_size, encrypted, verified_at, verification_status, backed_up_at,
                   original_filename, original_storage_url
            FROM media_backup_entries WHERE backup_id = $1
            ORDER BY relative_path
            "#,
        )
        .bind(backup_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn verify_entry(&self, entry_id: Uuid, status: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE media_backup_entries SET verified_at = NOW(), verification_status = $2 WHERE id = $1")
            .bind(entry_id)
            .bind(status)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn get_entry_count(&self, backup_id: Uuid) -> Result<i64, AppError> {
        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM media_backup_entries WHERE backup_id = $1")
            .bind(backup_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(result.0)
    }

    async fn create_restore(&self, input: CreateRestoreInput) -> Result<MediaRestore, AppError> {
        let restore = sqlx::query_as::<_, MediaRestore>(
            r#"
            INSERT INTO media_restores (backup_id, restore_path, restore_mode, created_by)
            VALUES ($1, $2, $3, $4)
            RETURNING id, backup_id, restore_path, restore_mode, status, started_at, completed_at,
                      error_message, total_files, restored_files, failed_files, total_bytes,
                      restored_bytes, rto_seconds, created_at, created_by
            "#,
        )
        .bind(input.backup_id)
        .bind(&input.restore_path)
        .bind(&input.restore_mode)
        .bind(&input.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(restore_id = %restore.id, backup_id = %input.backup_id, "Restore job created");
        Ok(restore)
    }

    async fn get_restore(&self, id: Uuid) -> Result<Option<MediaRestore>, AppError> {
        let result = sqlx::query_as::<_, MediaRestore>(
            r#"
            SELECT id, backup_id, restore_path, restore_mode, status, started_at, completed_at,
                   error_message, total_files, restored_files, failed_files, total_bytes,
                   restored_bytes, rto_seconds, created_at, created_by
            FROM media_restores WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn start_restore(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE media_restores SET status = 'running', started_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn update_restore_progress(&self, id: Uuid, restored_files: i32, restored_bytes: i64, failed_files: i32) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE media_restores SET restored_files = $2, restored_bytes = $3, failed_files = $4 WHERE id = $1"
        )
        .bind(id)
        .bind(restored_files)
        .bind(restored_bytes)
        .bind(failed_files)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn complete_restore(&self, id: Uuid, rto_seconds: i32) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE media_restores SET status = 'completed', completed_at = NOW(), rto_seconds = $2 WHERE id = $1"
        )
        .bind(id)
        .bind(rto_seconds)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(restore_id = %id, rto_seconds = rto_seconds, "Restore completed");
        Ok(())
    }

    async fn fail_restore(&self, id: Uuid, error: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE media_restores SET status = 'failed', error_message = $2 WHERE id = $1")
            .bind(id)
            .bind(error)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::error!(restore_id = %id, error = %error, "Restore failed");
        Ok(())
    }

    async fn record_storage_metrics(&self, input: RecordStorageMetricsInput) -> Result<StorageHealthMetric, AppError> {
        let metric = sqlx::query_as::<_, StorageHealthMetric>(
            r#"
            INSERT INTO storage_health_metrics (
                mount_path, volume_label, total_bytes, used_bytes, available_bytes,
                inode_total, inode_used, read_latency_ms, write_latency_ms,
                is_healthy, health_notes
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, mount_path, volume_label, total_bytes, used_bytes, available_bytes,
                      inode_total, inode_used, read_latency_ms, write_latency_ms,
                      is_healthy, health_notes, recorded_at
            "#,
        )
        .bind(&input.mount_path)
        .bind(&input.volume_label)
        .bind(input.total_bytes)
        .bind(input.used_bytes)
        .bind(input.available_bytes)
        .bind(input.inode_total)
        .bind(input.inode_used)
        .bind(input.read_latency_ms)
        .bind(input.write_latency_ms)
        .bind(input.is_healthy)
        .bind(&input.health_notes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(metric)
    }

    async fn get_latest_storage_metrics(&self, mount_path: &str) -> Result<Option<StorageHealthMetric>, AppError> {
        let result = sqlx::query_as::<_, StorageHealthMetric>(
            r#"
            SELECT id, mount_path, volume_label, total_bytes, used_bytes, available_bytes,
                   inode_total, inode_used, read_latency_ms, write_latency_ms,
                   is_healthy, health_notes, recorded_at
            FROM storage_health_metrics
            WHERE mount_path = $1
            ORDER BY recorded_at DESC
            LIMIT 1
            "#,
        )
        .bind(mount_path)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn get_unhealthy_mounts(&self) -> Result<Vec<StorageHealthMetric>, AppError> {
        let results = sqlx::query_as::<_, StorageHealthMetric>(
            r#"
            SELECT DISTINCT ON (mount_path)
                   id, mount_path, volume_label, total_bytes, used_bytes, available_bytes,
                   inode_total, inode_used, read_latency_ms, write_latency_ms,
                   is_healthy, health_notes, recorded_at
            FROM storage_health_metrics
            WHERE is_healthy = FALSE
            ORDER BY mount_path, recorded_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_status() {
        assert_eq!(BackupStatus::Completed.as_str(), "completed");
        assert_eq!(BackupStatus::from_str("completed"), Some(BackupStatus::Completed));
    }

    #[test]
    fn test_backup_type() {
        assert_eq!(BackupType::Full.as_str(), "full");
        assert_eq!(BackupType::Incremental.as_str(), "incremental");
    }
}
