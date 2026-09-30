//! Sync checkpoint repository for durable cursor management
//!
//! Provides cursor-based sync with:
//! - Durable checkpoint persistence
//! - High-water mark tracking for equal-timestamp ordering
//! - Error tracking and recovery
//! - Source system tracking

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::sync::Arc;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Sync checkpoint entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SyncCheckpoint {
    pub sync_id: String,
    pub sync_type: String,
    pub cursor_json: Option<String>,
    pub records_processed: i64,
    pub records_reconciled: i64,
    pub is_reconciled: bool,
    pub source_system: Option<String>,
    pub cursor_version: i32,
    pub high_water_mark: Option<String>,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub error_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SyncCheckpoint {
    /// Parse the cursor JSON into a typed cursor
    pub fn parse_cursor<T: DeserializeOwned>(&self) -> Option<T> {
        self.cursor_json.as_ref().and_then(|json| {
            serde_json::from_str(json).ok()
        })
    }

    /// Check if there was a recent error
    pub fn has_error(&self) -> bool {
        self.last_error.is_some()
    }

    /// Check if this sync is in a healthy state
    pub fn is_healthy(&self) -> bool {
        self.error_count < 3 && !self.has_error()
    }
}

/// Input for creating or updating a checkpoint
#[derive(Debug, Clone)]
pub struct SaveCheckpointInput {
    pub sync_id: String,
    pub sync_type: String,
    pub source_system: Option<String>,
    pub cursor: Option<JsonValue>,
    pub cursor_version: i32,
    pub high_water_mark: Option<String>,
    pub records_processed: i64,
}

/// Input for marking a sync as reconciled
#[derive(Debug, Clone)]
pub struct ReconcileCheckpointInput {
    pub sync_id: String,
    pub records_reconciled: i64,
}

/// Input for recording a sync error
#[derive(Debug, Clone)]
pub struct RecordSyncErrorInput {
    pub sync_id: String,
    pub error: String,
}

/// Cursor value wrapper with pagination support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorValue {
    /// Timestamp-based cursor (for time-ordered sync)
    pub timestamp: Option<DateTime<Utc>>,
    /// ID-based cursor (for id-ordered sync)
    pub id: Option<String>,
    /// Offset cursor (for page-based sync)
    pub offset: Option<i64>,
    /// High-water mark for equal-timestamp disambiguation
    pub high_water_mark: Option<String>,
    /// Additional cursor data
    pub extra: Option<JsonValue>,
}

impl CursorValue {
    pub fn from_timestamp(ts: DateTime<Utc>) -> Self {
        Self {
            timestamp: Some(ts),
            id: None,
            offset: None,
            high_water_mark: None,
            extra: None,
        }
    }

    pub fn from_id(id: impl Into<String>) -> Self {
        Self {
            timestamp: None,
            id: Some(id.into()),
            offset: None,
            high_water_mark: None,
            extra: None,
        }
    }

    pub fn from_offset(offset: i64) -> Self {
        Self {
            timestamp: None,
            id: None,
            offset: Some(offset),
            high_water_mark: None,
            extra: None,
        }
    }

    pub fn with_high_water_mark(mut self, hwm: impl Into<String>) -> Self {
        self.high_water_mark = Some(hwm.into());
        self
    }
}

// ============================================
// TRAIT DEFINITION
// ============================================

pub type DynSyncCheckpointRepository = Arc<dyn SyncCheckpointRepository + Send + Sync>;

#[async_trait]
pub trait SyncCheckpointRepository: Send + Sync {
    /// Get a checkpoint by sync_id
    async fn get(&self, sync_id: &str) -> Result<Option<SyncCheckpoint>, AppError>;

    /// Get all checkpoints for a sync type
    async fn get_by_type(&self, sync_type: &str) -> Result<Vec<SyncCheckpoint>, AppError>;

    /// Save a checkpoint (upsert)
    async fn save(&self, input: SaveCheckpointInput) -> Result<SyncCheckpoint, AppError>;

    /// Mark a sync as reconciled
    async fn mark_reconciled(&self, input: ReconcileCheckpointInput) -> Result<SyncCheckpoint, AppError>;

    /// Record a sync error
    async fn record_error(&self, input: RecordSyncErrorInput) -> Result<(), AppError>;

    /// Clear error state after successful sync
    async fn clear_error(&self, sync_id: &str) -> Result<(), AppError>;

    /// Reset a checkpoint (for re-sync)
    async fn reset(&self, sync_id: &str) -> Result<(), AppError>;

    /// Delete a checkpoint
    async fn delete(&self, sync_id: &str) -> Result<bool, AppError>;

    /// Get all checkpoints with errors
    async fn get_errored_checkpoints(&self) -> Result<Vec<SyncCheckpoint>, AppError>;

    /// Get all checkpoints not reconciled
    async fn get_unreconciled_checkpoints(&self) -> Result<Vec<SyncCheckpoint>, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct SyncCheckpointRepositoryImpl {
    pool: ConnectionPool,
}

impl SyncCheckpointRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SyncCheckpointRepository for SyncCheckpointRepositoryImpl {
    async fn get(&self, sync_id: &str) -> Result<Option<SyncCheckpoint>, AppError> {
        let result = sqlx::query_as::<_, SyncCheckpoint>(
            r#"
            SELECT sync_id, sync_type, cursor_json, records_processed, records_reconciled,
                   is_reconciled, source_system, cursor_version, high_water_mark,
                   last_sync_at, last_error, error_count, created_at, updated_at
            FROM sync_checkpoints
            WHERE sync_id = $1
            "#,
        )
        .bind(sync_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn get_by_type(&self, sync_type: &str) -> Result<Vec<SyncCheckpoint>, AppError> {
        let results = sqlx::query_as::<_, SyncCheckpoint>(
            r#"
            SELECT sync_id, sync_type, cursor_json, records_processed, records_reconciled,
                   is_reconciled, source_system, cursor_version, high_water_mark,
                   last_sync_at, last_error, error_count, created_at, updated_at
            FROM sync_checkpoints
            WHERE sync_type = $1
            ORDER BY sync_id
            "#,
        )
        .bind(sync_type)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn save(&self, input: SaveCheckpointInput) -> Result<SyncCheckpoint, AppError> {
        let cursor_json = input.cursor.map(|c| c.to_string());

        let result = sqlx::query_as::<_, SyncCheckpoint>(
            r#"
            INSERT INTO sync_checkpoints (
                sync_id, sync_type, source_system, cursor_json, cursor_version,
                high_water_mark, records_processed, last_sync_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
            ON CONFLICT (sync_id) DO UPDATE SET
                cursor_json = EXCLUDED.cursor_json,
                cursor_version = EXCLUDED.cursor_version,
                high_water_mark = EXCLUDED.high_water_mark,
                records_processed = sync_checkpoints.records_processed + EXCLUDED.records_processed,
                last_sync_at = NOW(),
                updated_at = NOW(),
                -- Clear error on successful sync
                last_error = NULL,
                error_count = 0
            RETURNING sync_id, sync_type, cursor_json, records_processed, records_reconciled,
                      is_reconciled, source_system, cursor_version, high_water_mark,
                      last_sync_at, last_error, error_count, created_at, updated_at
            "#,
        )
        .bind(&input.sync_id)
        .bind(&input.sync_type)
        .bind(input.source_system.as_deref())
        .bind(cursor_json.as_deref())
        .bind(input.cursor_version)
        .bind(input.high_water_mark.as_deref())
        .bind(input.records_processed)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::debug!(
            sync_id = %input.sync_id,
            records = input.records_processed,
            "Checkpoint saved"
        );

        Ok(result)
    }

    async fn mark_reconciled(&self, input: ReconcileCheckpointInput) -> Result<SyncCheckpoint, AppError> {
        let result = sqlx::query_as::<_, SyncCheckpoint>(
            r#"
            UPDATE sync_checkpoints
            SET is_reconciled = TRUE,
                records_reconciled = records_reconciled + $2,
                updated_at = NOW()
            WHERE sync_id = $1
            RETURNING sync_id, sync_type, cursor_json, records_processed, records_reconciled,
                      is_reconciled, source_system, cursor_version, high_water_mark,
                      last_sync_at, last_error, error_count, created_at, updated_at
            "#,
        )
        .bind(&input.sync_id)
        .bind(input.records_reconciled)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            sync_id = %input.sync_id,
            records_reconciled = input.records_reconciled,
            "Checkpoint marked as reconciled"
        );

        Ok(result)
    }

    async fn record_error(&self, input: RecordSyncErrorInput) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE sync_checkpoints
            SET last_error = $2,
                error_count = error_count + 1,
                updated_at = NOW()
            WHERE sync_id = $1
            "#,
        )
        .bind(&input.sync_id)
        .bind(&input.error)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::warn!(
            sync_id = %input.sync_id,
            error = %input.error,
            "Sync error recorded"
        );

        Ok(())
    }

    async fn clear_error(&self, sync_id: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE sync_checkpoints
            SET last_error = NULL,
                error_count = 0,
                updated_at = NOW()
            WHERE sync_id = $1
            "#,
        )
        .bind(sync_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn reset(&self, sync_id: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE sync_checkpoints
            SET cursor_json = NULL,
                high_water_mark = NULL,
                records_processed = 0,
                records_reconciled = 0,
                is_reconciled = FALSE,
                last_error = NULL,
                error_count = 0,
                last_sync_at = NULL,
                updated_at = NOW()
            WHERE sync_id = $1
            "#,
        )
        .bind(sync_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(sync_id = %sync_id, "Checkpoint reset");

        Ok(())
    }

    async fn delete(&self, sync_id: &str) -> Result<bool, AppError> {
        let result = sqlx::query(
            "DELETE FROM sync_checkpoints WHERE sync_id = $1"
        )
        .bind(sync_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_errored_checkpoints(&self) -> Result<Vec<SyncCheckpoint>, AppError> {
        let results = sqlx::query_as::<_, SyncCheckpoint>(
            r#"
            SELECT sync_id, sync_type, cursor_json, records_processed, records_reconciled,
                   is_reconciled, source_system, cursor_version, high_water_mark,
                   last_sync_at, last_error, error_count, created_at, updated_at
            FROM sync_checkpoints
            WHERE last_error IS NOT NULL OR error_count > 0
            ORDER BY error_count DESC, updated_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn get_unreconciled_checkpoints(&self) -> Result<Vec<SyncCheckpoint>, AppError> {
        let results = sqlx::query_as::<_, SyncCheckpoint>(
            r#"
            SELECT sync_id, sync_type, cursor_json, records_processed, records_reconciled,
                   is_reconciled, source_system, cursor_version, high_water_mark,
                   last_sync_at, last_error, error_count, created_at, updated_at
            FROM sync_checkpoints
            WHERE is_reconciled = FALSE AND records_processed > 0
            ORDER BY updated_at DESC
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
    fn test_cursor_value_from_timestamp() {
        let now = Utc::now();
        let cursor = CursorValue::from_timestamp(now);
        assert_eq!(cursor.timestamp, Some(now));
        assert!(cursor.id.is_none());
    }

    #[test]
    fn test_cursor_value_from_id() {
        let cursor = CursorValue::from_id("abc123");
        assert_eq!(cursor.id, Some("abc123".to_string()));
        assert!(cursor.timestamp.is_none());
    }

    #[test]
    fn test_cursor_value_with_hwm() {
        let cursor = CursorValue::from_offset(100)
            .with_high_water_mark("last-id-xyz");
        assert_eq!(cursor.offset, Some(100));
        assert_eq!(cursor.high_water_mark, Some("last-id-xyz".to_string()));
    }
}
