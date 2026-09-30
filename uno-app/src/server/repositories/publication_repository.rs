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
