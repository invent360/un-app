//! License publication repository
//!
//! Provides:
//! - Publication batch management with correlation IDs
//! - Per-item acknowledgement tracking
//! - Withdrawal with reservation blocking
//! - Audit trail for all actions

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::net::IpAddr;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Publication operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationOperation {
    Publish,
    Withdraw,
    DryRun,
}

impl PublicationOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Publish => "publish",
            Self::Withdraw => "withdraw",
            Self::DryRun => "dry_run",
        }
    }
}

/// Publication batch status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationStatus {
    Pending,
    Acknowledged,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

impl PublicationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Acknowledged => "acknowledged",
            Self::Processing => "processing",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

/// Item status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    Pending,
    Acknowledged,
    Processing,
    Completed,
    Failed,
}

impl ItemStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Acknowledged => "acknowledged",
            Self::Processing => "processing",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }
}

/// Item result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemResult {
    Success,
    Failed,
    Skipped,
}

impl ItemResult {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
        }
    }
}

/// License publication status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicensePublicationStatus {
    Draft,
    Pending,
    Published,
    Withdrawn,
}

impl LicensePublicationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Pending => "pending",
            Self::Published => "published",
            Self::Withdrawn => "withdrawn",
        }
    }
}

/// Publication batch entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PublicationBatch {
    pub id: i64,
    pub correlation_id: Uuid,
    pub publisher_id: String,
    pub source_import_batch_id: Option<Uuid>,
    pub operation: String,
    pub total_items: i32,
    pub acknowledged_count: i32,
    pub published_count: i32,
    pub failed_count: i32,
    pub pending_count: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub error_message: Option<String>,
}

/// Publication item entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PublicationItem {
    pub id: i64,
    pub correlation_id: Uuid,
    pub license_id: String,
    pub status: String,
    pub acknowledged: bool,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub result: Option<String>,
    pub error_message: Option<String>,
    pub affected_reservations: Option<i32>,
    pub reservations_blocked: Option<bool>,
    pub processed_at: Option<DateTime<Utc>>,
}

/// Input for creating a publication batch
#[derive(Debug, Clone)]
pub struct CreatePublicationBatchInput {
    pub publisher_id: String,
    pub source_import_batch_id: Option<Uuid>,
    pub operation: PublicationOperation,
    pub license_ids: Vec<String>,
    pub notes: Option<String>,
}

/// Input for acknowledging items
#[derive(Debug, Clone)]
pub struct AcknowledgeItemsInput {
    pub correlation_id: Uuid,
    pub license_ids: Vec<String>,
    pub acknowledger_id: String,
}

/// Publication summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicationSummary {
    pub correlation_id: Uuid,
    pub operation: String,
    pub status: String,
    pub total_items: i32,
    pub acknowledged: i32,
    pub published: i32,
    pub failed: i32,
    pub pending: i32,
    pub failed_items: Vec<FailedItem>,
}

/// Failed item details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedItem {
    pub license_id: String,
    pub error: String,
}

// ============================================
// R5-06: COMPOUND CURSOR TYPES
// ============================================

/// Compound cursor for deterministic pagination across restarts
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SyncCursor {
    pub sync_type: String,
    pub cursor_timestamp: Option<DateTime<Utc>>,
    pub cursor_id: Option<String>,
    pub cursor_sequence: Option<i64>,
    pub processed_count: i64,
    pub status: String,
}

/// Input for updating sync cursor
#[derive(Debug, Clone)]
pub struct UpdateCursorInput {
    pub sync_type: String,
    pub cursor_timestamp: DateTime<Utc>,
    pub cursor_id: String,
    pub cursor_sequence: Option<i64>,
    pub batch_size: Option<i32>,
}

/// License row for sync with compound cursor
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SyncLicenseRow {
    pub license_id: String,
    pub lease_code: String,
    pub claimed_at: Option<DateTime<Utc>>,
    pub split_type: String,
    pub uno_share_pct: Option<f64>,
    pub ulo_share_pct: Option<f64>,
    pub agent_share_pct: Option<f64>,
    pub issued_to: Option<String>,
    pub referral_id: Option<i32>,
    pub is_last_batch: bool,
}

/// Quarantine reason codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineReason {
    MissingLeaseCode,
    MissingValidityDates,
    InvalidDateFormat,
    MissingShares,
    ValidationFailed,
}

impl QuarantineReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingLeaseCode => "missing_lease_code",
            Self::MissingValidityDates => "missing_validity_dates",
            Self::InvalidDateFormat => "invalid_date_format",
            Self::MissingShares => "missing_shares",
            Self::ValidationFailed => "validation_failed",
        }
    }
}

/// Input for quarantining a license
#[derive(Debug, Clone)]
pub struct QuarantineInput {
    pub source_license_id: String,
    pub source_system: String,
    pub reason_code: QuarantineReason,
    pub reason_message: String,
    pub source_record: Option<JsonValue>,
    pub validation_errors: Option<JsonValue>,
}

/// Quarantined license record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuarantinedLicense {
    pub id: i64,
    pub source_license_id: String,
    pub source_system: String,
    pub reason_code: String,
    pub reason_message: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// Per-item batch outcome for R5-06 correlation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchItemOutcome {
    pub input_index: i32,
    pub input_license_id: String,
    pub result: ItemResult,
    pub error_message: Option<String>,
    pub quarantine_id: Option<i64>,
}

// ============================================
// TRAIT DEFINITION
// ============================================

pub type DynPublicationRepository = Arc<dyn PublicationRepository + Send + Sync>;

#[async_trait]
pub trait PublicationRepository: Send + Sync {
    // Batch operations
    async fn create_batch(&self, input: CreatePublicationBatchInput) -> Result<PublicationBatch, AppError>;
    async fn get_batch(&self, correlation_id: Uuid) -> Result<Option<PublicationBatch>, AppError>;
    async fn update_batch_status(&self, correlation_id: Uuid, status: PublicationStatus, error: Option<&str>) -> Result<(), AppError>;
    async fn complete_batch(&self, correlation_id: Uuid) -> Result<(), AppError>;
    async fn list_recent_batches(&self, limit: i32) -> Result<Vec<PublicationBatch>, AppError>;

    // Item operations
    async fn acknowledge_items(&self, input: AcknowledgeItemsInput) -> Result<i32, AppError>;
    async fn acknowledge_all_items(&self, correlation_id: Uuid, acknowledger_id: &str) -> Result<i32, AppError>;
    async fn get_acknowledged_items(&self, correlation_id: Uuid) -> Result<Vec<PublicationItem>, AppError>;
    async fn get_pending_items(&self, correlation_id: Uuid) -> Result<Vec<PublicationItem>, AppError>;
    async fn update_item_result(&self, item_id: i64, result: ItemResult, error: Option<&str>) -> Result<(), AppError>;

    // Publication execution
    async fn publish_license(&self, license_id: &str, publisher_id: &str, correlation_id: Uuid) -> Result<(), AppError>;
    async fn withdraw_license(&self, license_id: &str, withdrawer_id: &str, reason: &str, correlation_id: Uuid) -> Result<i32, AppError>;

    // Summary
    async fn get_summary(&self, correlation_id: Uuid) -> Result<Option<PublicationSummary>, AppError>;

    // Audit
    async fn log_audit(
        &self,
        correlation_id: Uuid,
        license_id: Option<&str>,
        actor_id: &str,
        action: &str,
        details: Option<JsonValue>,
        ip_address: Option<IpAddr>,
    ) -> Result<(), AppError>;

    // R5-06: Compound cursor operations
    async fn get_sync_cursor(&self, sync_type: &str) -> Result<Option<SyncCursor>, AppError>;
    async fn update_sync_cursor(&self, input: UpdateCursorInput) -> Result<(), AppError>;
    async fn fetch_licenses_for_sync(
        &self,
        cursor_timestamp: Option<DateTime<Utc>>,
        cursor_id: Option<&str>,
        limit: i32,
    ) -> Result<Vec<SyncLicenseRow>, AppError>;

    // R5-06: Quarantine operations
    async fn quarantine_license(&self, input: QuarantineInput) -> Result<i64, AppError>;
    async fn get_quarantined_licenses(&self, status: Option<&str>, limit: i32) -> Result<Vec<QuarantinedLicense>, AppError>;
    async fn resolve_quarantine(&self, id: i64, resolved_by: &str, notes: Option<&str>) -> Result<(), AppError>;

    // R5-06: Per-item batch outcomes
    async fn create_batch_with_correlation(
        &self,
        input: CreatePublicationBatchInput,
        input_license_ids: &[(i32, String)], // (input_index, license_id)
    ) -> Result<PublicationBatch, AppError>;
    async fn get_batch_outcomes(&self, correlation_id: Uuid) -> Result<Vec<BatchItemOutcome>, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct PublicationRepositoryImpl {
    pool: ConnectionPool,
}

impl PublicationRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PublicationRepository for PublicationRepositoryImpl {
    async fn create_batch(&self, input: CreatePublicationBatchInput) -> Result<PublicationBatch, AppError> {
        // Start transaction
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Create batch
        let batch = sqlx::query_as::<_, PublicationBatch>(
            r#"
            INSERT INTO license_publication_batches (
                publisher_id, source_import_batch_id, operation, total_items, notes, status
            )
            VALUES ($1, $2, $3, $4, $5, 'pending')
            RETURNING id, correlation_id, publisher_id, source_import_batch_id, operation,
                      total_items, acknowledged_count, published_count, failed_count, pending_count,
                      status, created_at, acknowledged_at, completed_at, notes, error_message
            "#,
        )
        .bind(&input.publisher_id)
        .bind(input.source_import_batch_id)
        .bind(input.operation.as_str())
        .bind(input.license_ids.len() as i32)
        .bind(input.notes.as_deref())
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Create items
        for license_id in &input.license_ids {
            sqlx::query(
                r#"
                INSERT INTO license_publication_items (correlation_id, license_id, status)
                VALUES ($1, $2, 'pending')
                "#,
            )
            .bind(batch.correlation_id)
            .bind(license_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        // Update pending count
        sqlx::query(
            "UPDATE license_publication_batches SET pending_count = $2 WHERE correlation_id = $1"
        )
        .bind(batch.correlation_id)
        .bind(input.license_ids.len() as i32)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            correlation_id = %batch.correlation_id,
            operation = %input.operation.as_str(),
            items = input.license_ids.len(),
            "Publication batch created"
        );

        Ok(batch)
    }

    async fn get_batch(&self, correlation_id: Uuid) -> Result<Option<PublicationBatch>, AppError> {
        let result = sqlx::query_as::<_, PublicationBatch>(
            r#"
            SELECT id, correlation_id, publisher_id, source_import_batch_id, operation,
                   total_items, acknowledged_count, published_count, failed_count, pending_count,
                   status, created_at, acknowledged_at, completed_at, notes, error_message
            FROM license_publication_batches
            WHERE correlation_id = $1
            "#,
        )
        .bind(correlation_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn update_batch_status(&self, correlation_id: Uuid, status: PublicationStatus, error: Option<&str>) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE license_publication_batches
            SET status = $2, error_message = $3
            WHERE correlation_id = $1
            "#,
        )
        .bind(correlation_id)
        .bind(status.as_str())
        .bind(error)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn complete_batch(&self, correlation_id: Uuid) -> Result<(), AppError> {
        // Update counts from items
        sqlx::query(
            r#"
            UPDATE license_publication_batches b
            SET status = 'completed',
                completed_at = NOW(),
                published_count = (SELECT COUNT(*) FROM license_publication_items WHERE correlation_id = $1 AND result = 'success'),
                failed_count = (SELECT COUNT(*) FROM license_publication_items WHERE correlation_id = $1 AND result = 'failed'),
                pending_count = (SELECT COUNT(*) FROM license_publication_items WHERE correlation_id = $1 AND result IS NULL)
            WHERE correlation_id = $1
            "#,
        )
        .bind(correlation_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(correlation_id = %correlation_id, "Publication batch completed");

        Ok(())
    }

    async fn list_recent_batches(&self, limit: i32) -> Result<Vec<PublicationBatch>, AppError> {
        let results = sqlx::query_as::<_, PublicationBatch>(
            r#"
            SELECT id, correlation_id, publisher_id, source_import_batch_id, operation,
                   total_items, acknowledged_count, published_count, failed_count, pending_count,
                   status, created_at, acknowledged_at, completed_at, notes, error_message
            FROM license_publication_batches
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

    async fn acknowledge_items(&self, input: AcknowledgeItemsInput) -> Result<i32, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE license_publication_items
            SET acknowledged = TRUE, acknowledged_at = NOW(), status = 'acknowledged'
            WHERE correlation_id = $1 AND license_id = ANY($2) AND acknowledged = FALSE
            "#,
        )
        .bind(input.correlation_id)
        .bind(&input.license_ids)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let count = result.rows_affected() as i32;

        // Update batch acknowledged count
        sqlx::query(
            r#"
            UPDATE license_publication_batches
            SET acknowledged_count = (SELECT COUNT(*) FROM license_publication_items WHERE correlation_id = $1 AND acknowledged = TRUE),
                acknowledged_at = CASE WHEN acknowledged_at IS NULL THEN NOW() ELSE acknowledged_at END
            WHERE correlation_id = $1
            "#,
        )
        .bind(input.correlation_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Log audit
        let _ = self.log_audit(
            input.correlation_id,
            None,
            &input.acknowledger_id,
            "acknowledge",
            Some(serde_json::json!({ "count": count, "license_ids": input.license_ids })),
            None,
        ).await;

        Ok(count)
    }

    async fn acknowledge_all_items(&self, correlation_id: Uuid, acknowledger_id: &str) -> Result<i32, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE license_publication_items
            SET acknowledged = TRUE, acknowledged_at = NOW(), status = 'acknowledged'
            WHERE correlation_id = $1 AND acknowledged = FALSE
            "#,
        )
        .bind(correlation_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let count = result.rows_affected() as i32;

        // Update batch
        sqlx::query(
            r#"
            UPDATE license_publication_batches
            SET acknowledged_count = total_items,
                acknowledged_at = NOW(),
                status = 'acknowledged'
            WHERE correlation_id = $1
            "#,
        )
        .bind(correlation_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Log audit
        let _ = self.log_audit(
            correlation_id,
            None,
            acknowledger_id,
            "acknowledge_all",
            Some(serde_json::json!({ "count": count })),
            None,
        ).await;

        tracing::info!(correlation_id = %correlation_id, count = count, "All items acknowledged");

        Ok(count)
    }

    async fn get_acknowledged_items(&self, correlation_id: Uuid) -> Result<Vec<PublicationItem>, AppError> {
        let results = sqlx::query_as::<_, PublicationItem>(
            r#"
            SELECT id, correlation_id, license_id, status, acknowledged, acknowledged_at,
                   result, error_message, affected_reservations, reservations_blocked, processed_at
            FROM license_publication_items
            WHERE correlation_id = $1 AND acknowledged = TRUE
            ORDER BY id
            "#,
        )
        .bind(correlation_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn get_pending_items(&self, correlation_id: Uuid) -> Result<Vec<PublicationItem>, AppError> {
        let results = sqlx::query_as::<_, PublicationItem>(
            r#"
            SELECT id, correlation_id, license_id, status, acknowledged, acknowledged_at,
                   result, error_message, affected_reservations, reservations_blocked, processed_at
            FROM license_publication_items
            WHERE correlation_id = $1 AND result IS NULL
            ORDER BY id
            "#,
        )
        .bind(correlation_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn update_item_result(&self, item_id: i64, result: ItemResult, error: Option<&str>) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE license_publication_items
            SET result = $2, error_message = $3, status = 'completed', processed_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(item_id)
        .bind(result.as_str())
        .bind(error)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn publish_license(&self, license_id: &str, publisher_id: &str, correlation_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE licenses
            SET publication_status = 'published',
                published_at = NOW(),
                published_by = $2,
                publication_correlation_id = $3
            WHERE id = $1
            "#,
        )
        .bind(license_id)
        .bind(publisher_id)
        .bind(correlation_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn withdraw_license(&self, license_id: &str, withdrawer_id: &str, reason: &str, correlation_id: Uuid) -> Result<i32, AppError> {
        // Count affected reservations
        let affected: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM license_reservations
            WHERE license_id = $1 AND status = 'active'
            "#,
        )
        .bind(license_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Withdraw the license
        sqlx::query(
            r#"
            UPDATE licenses
            SET publication_status = 'withdrawn',
                withdrawn_at = NOW(),
                withdrawn_by = $2,
                withdrawal_reason = $3,
                publication_correlation_id = $4
            WHERE id = $1
            "#,
        )
        .bind(license_id)
        .bind(withdrawer_id)
        .bind(reason)
        .bind(correlation_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Block any active reservations
        sqlx::query(
            r#"
            UPDATE license_reservations
            SET status = 'blocked', released_at = NOW()
            WHERE license_id = $1 AND status = 'active'
            "#,
        )
        .bind(license_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::warn!(
            license_id = %license_id,
            affected_reservations = affected.0,
            reason = %reason,
            "License withdrawn"
        );

        Ok(affected.0 as i32)
    }

    async fn get_summary(&self, correlation_id: Uuid) -> Result<Option<PublicationSummary>, AppError> {
        let batch = self.get_batch(correlation_id).await?;
        let batch = match batch {
            Some(b) => b,
            None => return Ok(None),
        };

        // Get failed items
        let failed_items: Vec<(String, Option<String>)> = sqlx::query_as(
            r#"
            SELECT license_id, error_message
            FROM license_publication_items
            WHERE correlation_id = $1 AND result = 'failed'
            "#,
        )
        .bind(correlation_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let failed_items = failed_items
            .into_iter()
            .map(|(license_id, error)| FailedItem {
                license_id,
                error: error.unwrap_or_default(),
            })
            .collect();

        Ok(Some(PublicationSummary {
            correlation_id: batch.correlation_id,
            operation: batch.operation,
            status: batch.status,
            total_items: batch.total_items,
            acknowledged: batch.acknowledged_count,
            published: batch.published_count,
            failed: batch.failed_count,
            pending: batch.pending_count,
            failed_items,
        }))
    }

    async fn log_audit(
        &self,
        correlation_id: Uuid,
        license_id: Option<&str>,
        actor_id: &str,
        action: &str,
        details: Option<JsonValue>,
        ip_address: Option<IpAddr>,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO license_publication_audit (correlation_id, license_id, actor_id, action, details, ip_address)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(correlation_id)
        .bind(license_id)
        .bind(actor_id)
        .bind(action)
        .bind(details)
        .bind(ip_address.map(|ip| ip.to_string()))
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    // ============================================
    // R5-06: Compound Cursor Operations
    // ============================================

    async fn get_sync_cursor(&self, sync_type: &str) -> Result<Option<SyncCursor>, AppError> {
        let result = sqlx::query_as::<_, SyncCursor>(
            r#"
            SELECT sync_type, cursor_timestamp, cursor_id, cursor_sequence, processed_count, status
            FROM sync_cursors
            WHERE sync_type = $1 AND status = 'active'
            "#,
        )
        .bind(sync_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn update_sync_cursor(&self, input: UpdateCursorInput) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO sync_cursors (sync_type, cursor_timestamp, cursor_id, cursor_sequence, last_batch_size, processed_count)
            VALUES ($1, $2, $3, $4, $5, COALESCE($5, 0))
            ON CONFLICT (sync_type, direction) DO UPDATE SET
                cursor_timestamp = EXCLUDED.cursor_timestamp,
                cursor_id = EXCLUDED.cursor_id,
                cursor_sequence = EXCLUDED.cursor_sequence,
                last_batch_size = EXCLUDED.last_batch_size,
                processed_count = sync_cursors.processed_count + COALESCE(EXCLUDED.last_batch_size, 0),
                updated_at = NOW()
            "#,
        )
        .bind(&input.sync_type)
        .bind(input.cursor_timestamp)
        .bind(&input.cursor_id)
        .bind(input.cursor_sequence)
        .bind(input.batch_size)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::debug!(
            sync_type = %input.sync_type,
            cursor_id = %input.cursor_id,
            "R5-06: Sync cursor updated"
        );

        Ok(())
    }

    async fn fetch_licenses_for_sync(
        &self,
        cursor_timestamp: Option<DateTime<Utc>>,
        cursor_id: Option<&str>,
        limit: i32,
    ) -> Result<Vec<SyncLicenseRow>, AppError> {
        // R5-06: Fetch LIMIT + 1 rows to detect if there are more batches
        // This is the correct way to detect last batch for cursor pagination
        let fetch_limit = limit + 1;

        #[derive(Debug, sqlx::FromRow)]
        struct RawSyncRow {
            license_id: String,
            lease_code: String,
            claimed_at: Option<DateTime<Utc>>,
            split_type: String,
            uno_share_pct: Option<f64>,
            ulo_share_pct: Option<f64>,
            agent_share_pct: Option<f64>,
            issued_to: Option<String>,
            referral_id: Option<i32>,
        }

        let raw_results = sqlx::query_as::<_, RawSyncRow>(
            r#"
            SELECT
                l.id::VARCHAR(66) as license_id,
                l.lease_code::VARCHAR(100),
                l.claimed_at,
                l.split_type::text,
                l.uno_share_pct,
                l.ulo_share_pct,
                l.agent_share_pct,
                l.issued_to,
                l.referral_id
            FROM licenses l
            WHERE l.claimed = true
              AND (
                  $1::TIMESTAMPTZ IS NULL
                  OR (l.claimed_at, l.id) > ($1::TIMESTAMPTZ, COALESCE($2, ''))
              )
            ORDER BY l.claimed_at ASC, l.id ASC
            LIMIT $3
            "#,
        )
        .bind(cursor_timestamp)
        .bind(cursor_id)
        .bind(fetch_limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // R5-06: If we got fewer than LIMIT + 1 rows, this is the last batch
        let is_last_batch = (raw_results.len() as i32) <= limit;

        // Take only up to LIMIT rows (discard the extra row if present)
        let results: Vec<SyncLicenseRow> = raw_results
            .into_iter()
            .take(limit as usize)
            .map(|row| SyncLicenseRow {
                license_id: row.license_id,
                lease_code: row.lease_code,
                claimed_at: row.claimed_at,
                split_type: row.split_type,
                uno_share_pct: row.uno_share_pct,
                ulo_share_pct: row.ulo_share_pct,
                agent_share_pct: row.agent_share_pct,
                issued_to: row.issued_to,
                referral_id: row.referral_id,
                is_last_batch,
            })
            .collect();

        tracing::debug!(
            count = results.len(),
            is_last_batch,
            "R5-06: Fetched licenses for sync"
        );

        Ok(results)
    }

    // ============================================
    // R5-06: Quarantine Operations
    // ============================================

    async fn quarantine_license(&self, input: QuarantineInput) -> Result<i64, AppError> {
        let result = sqlx::query_as::<_, (i64,)>(
            r#"
            INSERT INTO publication_quarantine (
                source_license_id, source_system, source_record,
                reason_code, reason_message, validation_errors
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (source_license_id, source_system) DO UPDATE SET
                reason_code = EXCLUDED.reason_code,
                reason_message = EXCLUDED.reason_message,
                validation_errors = EXCLUDED.validation_errors,
                source_record = COALESCE(EXCLUDED.source_record, publication_quarantine.source_record),
                updated_at = NOW()
            RETURNING id
            "#,
        )
        .bind(&input.source_license_id)
        .bind(&input.source_system)
        .bind(&input.source_record)
        .bind(input.reason_code.as_str())
        .bind(&input.reason_message)
        .bind(&input.validation_errors)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::warn!(
            source_license_id = %input.source_license_id,
            reason = %input.reason_code.as_str(),
            "R5-06: License quarantined"
        );

        Ok(result.0)
    }

    async fn get_quarantined_licenses(&self, status: Option<&str>, limit: i32) -> Result<Vec<QuarantinedLicense>, AppError> {
        let status_filter = status.unwrap_or("quarantined");
        let results = sqlx::query_as::<_, QuarantinedLicense>(
            r#"
            SELECT id, source_license_id, source_system, reason_code, reason_message, status, created_at
            FROM publication_quarantine
            WHERE status = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(status_filter)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn resolve_quarantine(&self, id: i64, resolved_by: &str, notes: Option<&str>) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE publication_quarantine
            SET status = 'resolved', resolved_at = NOW(), resolved_by = $2, resolution_notes = $3
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(resolved_by)
        .bind(notes)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(quarantine_id = id, resolved_by = %resolved_by, "R5-06: Quarantine resolved");

        Ok(())
    }

    // ============================================
    // R5-06: Per-Item Batch Outcomes
    // ============================================

    async fn create_batch_with_correlation(
        &self,
        input: CreatePublicationBatchInput,
        input_license_ids: &[(i32, String)], // (input_index, license_id)
    ) -> Result<PublicationBatch, AppError> {
        // Start transaction
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Create batch
        let batch = sqlx::query_as::<_, PublicationBatch>(
            r#"
            INSERT INTO license_publication_batches (
                publisher_id, source_import_batch_id, operation, total_items, notes, status
            )
            VALUES ($1, $2, $3, $4, $5, 'pending')
            RETURNING id, correlation_id, publisher_id, source_import_batch_id, operation,
                      total_items, acknowledged_count, published_count, failed_count, pending_count,
                      status, created_at, acknowledged_at, completed_at, notes, error_message
            "#,
        )
        .bind(&input.publisher_id)
        .bind(input.source_import_batch_id)
        .bind(input.operation.as_str())
        .bind(input_license_ids.len() as i32)
        .bind(input.notes.as_deref())
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // R5-06: Create items with input_index and input_license_id for correlation
        for (input_index, license_id) in input_license_ids {
            sqlx::query(
                r#"
                INSERT INTO license_publication_items (correlation_id, license_id, status, input_index, input_license_id)
                VALUES ($1, $2, 'pending', $3, $4)
                "#,
            )
            .bind(batch.correlation_id)
            .bind(license_id)
            .bind(input_index)
            .bind(license_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        // Update pending count
        sqlx::query(
            "UPDATE license_publication_batches SET pending_count = $2 WHERE correlation_id = $1"
        )
        .bind(batch.correlation_id)
        .bind(input_license_ids.len() as i32)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            correlation_id = %batch.correlation_id,
            operation = %input.operation.as_str(),
            items = input_license_ids.len(),
            "R5-06: Publication batch created with correlation"
        );

        Ok(batch)
    }

    async fn get_batch_outcomes(&self, correlation_id: Uuid) -> Result<Vec<BatchItemOutcome>, AppError> {
        // R5-06: Return per-item outcomes correlated to input IDs
        let results: Vec<(i32, String, Option<String>, Option<String>, Option<i64>)> = sqlx::query_as(
            r#"
            SELECT input_index, input_license_id, result, error_message, quarantine_id
            FROM license_publication_items
            WHERE correlation_id = $1
            ORDER BY input_index
            "#,
        )
        .bind(correlation_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let outcomes = results
            .into_iter()
            .map(|(input_index, input_license_id, result, error_message, quarantine_id)| {
                let result = match result.as_deref() {
                    Some("success") => ItemResult::Success,
                    Some("failed") => ItemResult::Failed,
                    Some("skipped") => ItemResult::Skipped,
                    _ => ItemResult::Skipped, // Default for pending items
                };
                BatchItemOutcome {
                    input_index,
                    input_license_id,
                    result,
                    error_message,
                    quarantine_id,
                }
            })
            .collect();

        Ok(outcomes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publication_operation() {
        assert_eq!(PublicationOperation::Publish.as_str(), "publish");
        assert_eq!(PublicationOperation::Withdraw.as_str(), "withdraw");
        assert_eq!(PublicationOperation::DryRun.as_str(), "dry_run");
    }

    #[test]
    fn test_publication_status() {
        assert_eq!(PublicationStatus::Pending.as_str(), "pending");
        assert_eq!(PublicationStatus::Completed.as_str(), "completed");
    }

    #[test]
    fn test_item_result() {
        assert_eq!(ItemResult::Success.as_str(), "success");
        assert_eq!(ItemResult::Failed.as_str(), "failed");
    }
}
