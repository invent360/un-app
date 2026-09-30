//! License import repository with provenance tracking
//!
//! Provides:
//! - Import batch management
//! - Per-row validation result storage
//! - Original file retention references
//! - Quarantine workflow support

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

/// Import mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportMode {
    ValidateOnly,
    Execute,
}

impl ImportMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ValidateOnly => "validate_only",
            Self::Execute => "execute",
        }
    }
}

/// Import batch status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportStatus {
    Pending,
    Validating,
    Executing,
    Completed,
    Failed,
    Cancelled,
}

impl ImportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Validating => "validating",
            Self::Executing => "executing",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

/// Row validation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowStatus {
    Pending,
    Valid,
    Invalid,
    Quarantined,
}

impl RowStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
            Self::Quarantined => "quarantined",
        }
    }
}

/// Row action taken
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowAction {
    Created,
    Updated,
    Skipped,
    Quarantined,
    Rejected,
}

impl RowAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Skipped => "skipped",
            Self::Quarantined => "quarantined",
            Self::Rejected => "rejected",
        }
    }
}

/// Import batch entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ImportBatch {
    pub id: i64,
    pub batch_id: Uuid,
    pub imported_by: String,
    pub source_system: String,
    pub source_ip: Option<String>,
    pub provider_id: String,
    pub provider_name: Option<String>,
    pub provider_reference: Option<String>,
    pub import_mode: String,
    pub has_header: bool,
    pub contract_version: String,
    pub original_filename: Option<String>,
    pub original_content_hash: String,
    pub original_row_count: i32,
    pub original_stored_path: Option<String>,
    pub status: String,
    pub total_rows: i32,
    pub accepted_count: i32,
    pub rejected_count: i32,
    pub unchanged_count: i32,
    pub quarantined_count: i32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub processing_time_ms: Option<i32>,
    pub error_message: Option<String>,
}

/// Import row result entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ImportRow {
    pub id: i64,
    pub batch_id: Uuid,
    pub row_number: i32,
    #[sqlx(json)]
    pub original_row_data: JsonValue,
    pub status: String,
    pub license_id: Option<String>,
    pub lease_code: Option<String>,
    pub parsed_valid_from: Option<DateTime<Utc>>,
    pub parsed_valid_to: Option<DateTime<Utc>>,
    pub parsed_split_type: Option<String>,
    pub agreement_version: Option<i32>,
    #[sqlx(json)]
    pub agreement_snapshot: Option<JsonValue>,
    #[sqlx(json)]
    pub validation_errors: Option<JsonValue>,
    #[sqlx(json)]
    pub validation_warnings: Option<JsonValue>,
    pub action: Option<String>,
    pub action_reason: Option<String>,
    pub processed_at: Option<DateTime<Utc>>,
}

/// Input for creating an import batch
#[derive(Debug, Clone)]
pub struct CreateImportBatchInput {
    pub imported_by: String,
    pub source_system: String,
    pub source_ip: Option<IpAddr>,
    pub provider_id: String,
    pub provider_name: Option<String>,
    pub provider_reference: Option<String>,
    pub import_mode: ImportMode,
    pub has_header: bool,
    pub contract_version: String,
    pub original_filename: Option<String>,
    pub original_content_hash: String,
    pub original_row_count: i32,
    pub original_stored_path: Option<String>,
}

/// Input for creating an import row
#[derive(Debug, Clone)]
pub struct CreateImportRowInput {
    pub batch_id: Uuid,
    pub row_number: i32,
    pub original_row_data: JsonValue,
}

/// Input for updating row validation result
#[derive(Debug, Clone)]
pub struct UpdateRowValidationInput {
    pub row_id: i64,
    pub status: RowStatus,
    pub lease_code: Option<String>,
    pub parsed_valid_from: Option<DateTime<Utc>>,
    pub parsed_valid_to: Option<DateTime<Utc>>,
    pub parsed_split_type: Option<String>,
    pub agreement_version: Option<i32>,
    pub agreement_snapshot: Option<JsonValue>,
    pub validation_errors: Option<Vec<String>>,
    pub validation_warnings: Option<Vec<String>>,
}

/// Input for updating row action
#[derive(Debug, Clone)]
pub struct UpdateRowActionInput {
    pub row_id: i64,
    pub license_id: Option<String>,
    pub action: RowAction,
    pub action_reason: Option<String>,
}

/// Import summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSummary {
    pub batch_id: Uuid,
    pub status: String,
    pub total_rows: i32,
    pub accepted: i32,
    pub rejected: i32,
    pub unchanged: i32,
    pub quarantined: i32,
    pub processing_time_ms: Option<i32>,
    pub errors: Vec<ImportRowError>,
}

/// Row error for summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRowError {
    pub row_number: i32,
    pub errors: Vec<String>,
}

// ============================================
// TRAIT DEFINITION
// ============================================

pub type DynImportRepository = Arc<dyn ImportRepository + Send + Sync>;

#[async_trait]
pub trait ImportRepository: Send + Sync {
    // Batch operations
    async fn create_batch(&self, input: CreateImportBatchInput) -> Result<ImportBatch, AppError>;
    async fn get_batch(&self, batch_id: Uuid) -> Result<Option<ImportBatch>, AppError>;
    async fn get_batch_by_hash(&self, content_hash: &str) -> Result<Option<ImportBatch>, AppError>;
    async fn update_batch_status(&self, batch_id: Uuid, status: ImportStatus, error: Option<&str>) -> Result<(), AppError>;
    async fn update_batch_counts(&self, batch_id: Uuid, accepted: i32, rejected: i32, unchanged: i32, quarantined: i32) -> Result<(), AppError>;
    async fn complete_batch(&self, batch_id: Uuid, processing_time_ms: i32) -> Result<(), AppError>;
    async fn list_recent_batches(&self, limit: i32) -> Result<Vec<ImportBatch>, AppError>;

    // Row operations
    async fn create_row(&self, input: CreateImportRowInput) -> Result<ImportRow, AppError>;
    async fn create_rows_bulk(&self, inputs: Vec<CreateImportRowInput>) -> Result<i64, AppError>;
    async fn update_row_validation(&self, input: UpdateRowValidationInput) -> Result<(), AppError>;
    async fn update_row_action(&self, input: UpdateRowActionInput) -> Result<(), AppError>;
    async fn get_rows_by_batch(&self, batch_id: Uuid, status: Option<RowStatus>) -> Result<Vec<ImportRow>, AppError>;
    async fn get_row_errors(&self, batch_id: Uuid) -> Result<Vec<ImportRowError>, AppError>;

    // Summary
    async fn get_summary(&self, batch_id: Uuid) -> Result<Option<ImportSummary>, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct ImportRepositoryImpl {
    pool: ConnectionPool,
}

impl ImportRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ImportRepository for ImportRepositoryImpl {
    async fn create_batch(&self, input: CreateImportBatchInput) -> Result<ImportBatch, AppError> {
        let result = sqlx::query_as::<_, ImportBatch>(
            r#"
            INSERT INTO license_import_batches (
                imported_by, source_system, source_ip, provider_id, provider_name,
                provider_reference, import_mode, has_header, contract_version,
                original_filename, original_content_hash, original_row_count,
                original_stored_path, status, total_rows
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, 'pending', $12)
            RETURNING id, batch_id, imported_by, source_system, source_ip::TEXT,
                      provider_id, provider_name, provider_reference, import_mode,
                      has_header, contract_version, original_filename, original_content_hash,
                      original_row_count, original_stored_path, status, total_rows,
                      accepted_count, rejected_count, unchanged_count, quarantined_count,
                      started_at, completed_at, processing_time_ms, error_message
            "#,
        )
        .bind(&input.imported_by)
        .bind(&input.source_system)
        .bind(input.source_ip.map(|ip| ip.to_string()))
        .bind(&input.provider_id)
        .bind(input.provider_name.as_deref())
        .bind(input.provider_reference.as_deref())
        .bind(input.import_mode.as_str())
        .bind(input.has_header)
        .bind(&input.contract_version)
        .bind(input.original_filename.as_deref())
        .bind(&input.original_content_hash)
        .bind(input.original_row_count)
        .bind(input.original_stored_path.as_deref())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            batch_id = %result.batch_id,
            provider_id = %input.provider_id,
            rows = input.original_row_count,
            "Import batch created"
        );

        Ok(result)
    }

    async fn get_batch(&self, batch_id: Uuid) -> Result<Option<ImportBatch>, AppError> {
        let result = sqlx::query_as::<_, ImportBatch>(
            r#"
            SELECT id, batch_id, imported_by, source_system, source_ip::TEXT,
                   provider_id, provider_name, provider_reference, import_mode,
                   has_header, contract_version, original_filename, original_content_hash,
                   original_row_count, original_stored_path, status, total_rows,
                   accepted_count, rejected_count, unchanged_count, quarantined_count,
                   started_at, completed_at, processing_time_ms, error_message
            FROM license_import_batches
            WHERE batch_id = $1
            "#,
        )
        .bind(batch_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn get_batch_by_hash(&self, content_hash: &str) -> Result<Option<ImportBatch>, AppError> {
        let result = sqlx::query_as::<_, ImportBatch>(
            r#"
            SELECT id, batch_id, imported_by, source_system, source_ip::TEXT,
                   provider_id, provider_name, provider_reference, import_mode,
                   has_header, contract_version, original_filename, original_content_hash,
                   original_row_count, original_stored_path, status, total_rows,
                   accepted_count, rejected_count, unchanged_count, quarantined_count,
                   started_at, completed_at, processing_time_ms, error_message
            FROM license_import_batches
            WHERE original_content_hash = $1
            ORDER BY started_at DESC
            LIMIT 1
            "#,
        )
        .bind(content_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn update_batch_status(&self, batch_id: Uuid, status: ImportStatus, error: Option<&str>) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE license_import_batches
            SET status = $2, error_message = $3
            WHERE batch_id = $1
            "#,
        )
        .bind(batch_id)
        .bind(status.as_str())
        .bind(error)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn update_batch_counts(&self, batch_id: Uuid, accepted: i32, rejected: i32, unchanged: i32, quarantined: i32) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE license_import_batches
            SET accepted_count = $2, rejected_count = $3, unchanged_count = $4, quarantined_count = $5
            WHERE batch_id = $1
            "#,
        )
        .bind(batch_id)
        .bind(accepted)
        .bind(rejected)
        .bind(unchanged)
        .bind(quarantined)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn complete_batch(&self, batch_id: Uuid, processing_time_ms: i32) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE license_import_batches
            SET status = 'completed', completed_at = NOW(), processing_time_ms = $2
            WHERE batch_id = $1
            "#,
        )
        .bind(batch_id)
        .bind(processing_time_ms)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(batch_id = %batch_id, processing_time_ms = processing_time_ms, "Import batch completed");

        Ok(())
    }

    async fn list_recent_batches(&self, limit: i32) -> Result<Vec<ImportBatch>, AppError> {
        let results = sqlx::query_as::<_, ImportBatch>(
            r#"
            SELECT id, batch_id, imported_by, source_system, source_ip::TEXT,
                   provider_id, provider_name, provider_reference, import_mode,
                   has_header, contract_version, original_filename, original_content_hash,
                   original_row_count, original_stored_path, status, total_rows,
                   accepted_count, rejected_count, unchanged_count, quarantined_count,
                   started_at, completed_at, processing_time_ms, error_message
            FROM license_import_batches
            ORDER BY started_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn create_row(&self, input: CreateImportRowInput) -> Result<ImportRow, AppError> {
        let result = sqlx::query_as::<_, ImportRow>(
            r#"
            INSERT INTO license_import_rows (batch_id, row_number, original_row_data, status)
            VALUES ($1, $2, $3, 'pending')
            RETURNING id, batch_id, row_number, original_row_data, status, license_id,
                      lease_code, parsed_valid_from, parsed_valid_to, parsed_split_type,
                      agreement_version, agreement_snapshot, validation_errors,
                      validation_warnings, action, action_reason, processed_at
            "#,
        )
        .bind(input.batch_id)
        .bind(input.row_number)
        .bind(&input.original_row_data)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn create_rows_bulk(&self, inputs: Vec<CreateImportRowInput>) -> Result<i64, AppError> {
        if inputs.is_empty() {
            return Ok(0);
        }

        // Build bulk insert
        let mut query = String::from(
            "INSERT INTO license_import_rows (batch_id, row_number, original_row_data, status) VALUES "
        );

        let mut values = Vec::new();
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                query.push_str(", ");
            }
            let base = i * 3;
            query.push_str(&format!(
                "(${}, ${}, ${}, 'pending')",
                base + 1, base + 2, base + 3
            ));
            values.push(input.batch_id.to_string());
            values.push(input.row_number.to_string());
            values.push(input.original_row_data.to_string());
        }

        // Execute with raw query since we can't easily bind dynamically
        let mut query_builder = sqlx::query(&query);
        for input in &inputs {
            query_builder = query_builder
                .bind(input.batch_id)
                .bind(input.row_number)
                .bind(&input.original_row_data);
        }

        let result = query_builder
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    async fn update_row_validation(&self, input: UpdateRowValidationInput) -> Result<(), AppError> {
        let errors_json = input.validation_errors.map(|e| serde_json::json!(e));
        let warnings_json = input.validation_warnings.map(|w| serde_json::json!(w));

        sqlx::query(
            r#"
            UPDATE license_import_rows
            SET status = $2, lease_code = $3, parsed_valid_from = $4, parsed_valid_to = $5,
                parsed_split_type = $6, agreement_version = $7, agreement_snapshot = $8,
                validation_errors = $9, validation_warnings = $10, processed_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(input.row_id)
        .bind(input.status.as_str())
        .bind(input.lease_code.as_deref())
        .bind(input.parsed_valid_from)
        .bind(input.parsed_valid_to)
        .bind(input.parsed_split_type.as_deref())
        .bind(input.agreement_version)
        .bind(&input.agreement_snapshot)
        .bind(&errors_json)
        .bind(&warnings_json)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn update_row_action(&self, input: UpdateRowActionInput) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE license_import_rows
            SET license_id = $2, action = $3, action_reason = $4, processed_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(input.row_id)
        .bind(input.license_id.as_deref())
        .bind(input.action.as_str())
        .bind(input.action_reason.as_deref())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_rows_by_batch(&self, batch_id: Uuid, status: Option<RowStatus>) -> Result<Vec<ImportRow>, AppError> {
        let results = if let Some(s) = status {
            sqlx::query_as::<_, ImportRow>(
                r#"
                SELECT id, batch_id, row_number, original_row_data, status, license_id,
                       lease_code, parsed_valid_from, parsed_valid_to, parsed_split_type,
                       agreement_version, agreement_snapshot, validation_errors,
                       validation_warnings, action, action_reason, processed_at
                FROM license_import_rows
                WHERE batch_id = $1 AND status = $2
                ORDER BY row_number
                "#,
            )
            .bind(batch_id)
            .bind(s.as_str())
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, ImportRow>(
                r#"
                SELECT id, batch_id, row_number, original_row_data, status, license_id,
                       lease_code, parsed_valid_from, parsed_valid_to, parsed_split_type,
                       agreement_version, agreement_snapshot, validation_errors,
                       validation_warnings, action, action_reason, processed_at
                FROM license_import_rows
                WHERE batch_id = $1
                ORDER BY row_number
                "#,
            )
            .bind(batch_id)
            .fetch_all(&self.pool)
            .await
        };

        results.map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn get_row_errors(&self, batch_id: Uuid) -> Result<Vec<ImportRowError>, AppError> {
        let rows: Vec<(i32, Option<JsonValue>)> = sqlx::query_as(
            r#"
            SELECT row_number, validation_errors
            FROM license_import_rows
            WHERE batch_id = $1 AND status = 'invalid'
            ORDER BY row_number
            "#,
        )
        .bind(batch_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let errors = rows
            .into_iter()
            .map(|(row_number, errors_json)| {
                let errors = errors_json
                    .and_then(|v| serde_json::from_value::<Vec<String>>(v).ok())
                    .unwrap_or_default();
                ImportRowError { row_number, errors }
            })
            .collect();

        Ok(errors)
    }

    async fn get_summary(&self, batch_id: Uuid) -> Result<Option<ImportSummary>, AppError> {
        let batch = self.get_batch(batch_id).await?;
        let batch = match batch {
            Some(b) => b,
            None => return Ok(None),
        };

        let errors = self.get_row_errors(batch_id).await?;

        Ok(Some(ImportSummary {
            batch_id: batch.batch_id,
            status: batch.status,
            total_rows: batch.total_rows,
            accepted: batch.accepted_count,
            rejected: batch.rejected_count,
            unchanged: batch.unchanged_count,
            quarantined: batch.quarantined_count,
            processing_time_ms: batch.processing_time_ms,
            errors,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_mode() {
        assert_eq!(ImportMode::ValidateOnly.as_str(), "validate_only");
        assert_eq!(ImportMode::Execute.as_str(), "execute");
    }

    #[test]
    fn test_import_status() {
        assert_eq!(ImportStatus::Pending.as_str(), "pending");
        assert_eq!(ImportStatus::Completed.as_str(), "completed");
    }

    #[test]
    fn test_row_action() {
        assert_eq!(RowAction::Created.as_str(), "created");
        assert_eq!(RowAction::Quarantined.as_str(), "quarantined");
    }
}
