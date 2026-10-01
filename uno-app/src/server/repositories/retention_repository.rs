//! R3-17: Data Retention Repository
//!
//! Manages retention policies and tracks cleanup operations.
//! Policies are stored in the `retention_policies` table and define
//! how long data should be kept in each table.
//!
//! R5-16: Safe table allowlist ensures that critical tables cannot be
//! accidentally or maliciously purged.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// R5-16: SAFE TABLE ALLOWLIST
// ============================================
// Only tables in this list can have retention policies applied.
// NEVER add tables containing:
// - Financial records (allocation_ledger, settlement_*, allocation_*)
// - Audit trails (audit_logs, immutable_audit_events)
// - License ownership (licenses, license_claims)
// - Consent/legal records (consent_records, data_subject_requests)
// - Active CMS content (content_items, faqs, testimonials)

/// Tables that are ALLOWED to have retention cleanup
/// All other tables are protected by default
pub const RETENTION_ALLOWED_TABLES: &[&str] = &[
    // Session/temporary data
    "visitor_sessions",
    "rate_limit_entries",
    "nonces",
    "device_fingerprints",
    // Job queues (completed jobs only via deletion_condition)
    "job_queue",
    "outbox",
    "inbox",
    // Analytics aggregates (not source data)
    "cohort_daily_activity",
    "pilot_daily_snapshots",
    // Notifications (after delivery confirmation)
    "cohort_notifications",
    "notification_queue",
    // Support tickets (after resolution + retention period)
    "support_tickets",
    "support_messages",
    // Sync checkpoints (old checkpoints only)
    "sync_checkpoints",
];

// ============================================
// TYPES
// ============================================

/// A retention policy for a database table
#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    pub id: i32,
    pub table_name: String,
    pub retention_days: Option<i32>,
    pub archive_before_delete: bool,
    pub deletion_condition: String,
    pub last_cleanup_at: Option<DateTime<Utc>>,
    pub rows_deleted_last: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Result of a cleanup operation for a single policy
#[derive(Debug, Clone)]
pub struct RetentionCleanupResult {
    pub table_name: String,
    pub rows_deleted: i64,
    pub rows_archived: i64,
    pub duration_ms: u64,
    pub error: Option<String>,
}

impl RetentionCleanupResult {
    pub fn success(table_name: &str, rows_deleted: i64, rows_archived: i64, duration_ms: u64) -> Self {
        Self {
            table_name: table_name.to_string(),
            rows_deleted,
            rows_archived,
            duration_ms,
            error: None,
        }
    }

    pub fn failure(table_name: &str, error: impl Into<String>) -> Self {
        Self {
            table_name: table_name.to_string(),
            rows_deleted: 0,
            rows_archived: 0,
            duration_ms: 0,
            error: Some(error.into()),
        }
    }
}

/// Dynamic type alias for dependency injection
pub type DynRetentionRepository = Arc<dyn RetentionRepository + Send + Sync>;

// ============================================
// TRAIT
// ============================================

#[async_trait]
pub trait RetentionRepository: Send + Sync {
    /// Get all active retention policies
    async fn list_policies(&self) -> Result<Vec<RetentionPolicy>, AppError>;

    /// Get a specific retention policy by table name
    async fn get_policy(&self, table_name: &str) -> Result<Option<RetentionPolicy>, AppError>;

    /// Update cleanup stats for a policy after execution
    async fn record_cleanup(
        &self,
        table_name: &str,
        rows_deleted: i64,
    ) -> Result<(), AppError>;

    /// Execute cleanup for a policy and return rows deleted
    /// Returns the number of rows deleted, or 0 if the condition is empty
    async fn execute_cleanup(&self, policy: &RetentionPolicy) -> Result<i64, AppError>;

    /// Archive rows before deletion (for policies with archive_before_delete = true)
    async fn archive_before_cleanup(&self, policy: &RetentionPolicy) -> Result<i64, AppError>;

    /// Update or create a retention policy
    async fn upsert_policy(
        &self,
        table_name: &str,
        retention_days: Option<i32>,
        archive_before_delete: bool,
        deletion_condition: &str,
    ) -> Result<RetentionPolicy, AppError>;
}

// ============================================
// IMPLEMENTATION
// ============================================

pub struct RetentionRepositoryImpl {
    pool: ConnectionPool,
}

impl RetentionRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RetentionRepository for RetentionRepositoryImpl {
    async fn list_policies(&self) -> Result<Vec<RetentionPolicy>, AppError> {
        let rows = sqlx::query_as::<_, (
            i32, String, Option<i32>, bool, String,
            Option<DateTime<Utc>>, i32, DateTime<Utc>, DateTime<Utc>
        )>(
            r#"
            SELECT id, table_name, retention_days, archive_before_delete, deletion_condition,
                   last_cleanup_at, rows_deleted_last, created_at, updated_at
            FROM retention_policies
            ORDER BY table_name
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|(
            id, table_name, retention_days, archive_before_delete, deletion_condition,
            last_cleanup_at, rows_deleted_last, created_at, updated_at
        )| RetentionPolicy {
            id,
            table_name,
            retention_days,
            archive_before_delete,
            deletion_condition,
            last_cleanup_at,
            rows_deleted_last,
            created_at,
            updated_at,
        }).collect())
    }

    async fn get_policy(&self, table_name: &str) -> Result<Option<RetentionPolicy>, AppError> {
        let row = sqlx::query_as::<_, (
            i32, String, Option<i32>, bool, String,
            Option<DateTime<Utc>>, i32, DateTime<Utc>, DateTime<Utc>
        )>(
            r#"
            SELECT id, table_name, retention_days, archive_before_delete, deletion_condition,
                   last_cleanup_at, rows_deleted_last, created_at, updated_at
            FROM retention_policies
            WHERE table_name = $1
            "#
        )
        .bind(table_name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.map(|(
            id, table_name, retention_days, archive_before_delete, deletion_condition,
            last_cleanup_at, rows_deleted_last, created_at, updated_at
        )| RetentionPolicy {
            id,
            table_name,
            retention_days,
            archive_before_delete,
            deletion_condition,
            last_cleanup_at,
            rows_deleted_last,
            created_at,
            updated_at,
        }))
    }

    async fn record_cleanup(
        &self,
        table_name: &str,
        rows_deleted: i64,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE retention_policies
            SET last_cleanup_at = NOW(),
                rows_deleted_last = $2,
                updated_at = NOW()
            WHERE table_name = $1
            "#
        )
        .bind(table_name)
        .bind(rows_deleted as i32)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn execute_cleanup(&self, policy: &RetentionPolicy) -> Result<i64, AppError> {
        // R5-16: Validate table is in allowlist before any cleanup
        if !RETENTION_ALLOWED_TABLES.contains(&policy.table_name.as_str()) {
            tracing::error!(
                table = %policy.table_name,
                "SECURITY: Attempted cleanup on protected table - BLOCKED"
            );
            return Err(AppError::BadRequest(format!(
                "Table '{}' is not in the retention allowlist. Protected tables cannot be purged.",
                policy.table_name
            )));
        }

        // R5-16: Validate table name contains only safe characters (prevent SQL injection)
        if !policy.table_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(AppError::BadRequest(format!(
                "Invalid table name '{}': must contain only alphanumeric characters and underscores",
                policy.table_name
            )));
        }

        // Skip if deletion_condition is empty or 'FALSE' (indefinite retention)
        if policy.deletion_condition.is_empty()
            || policy.deletion_condition.to_uppercase() == "FALSE"
        {
            return Ok(0);
        }

        // Build the DELETE query using the policy's deletion condition
        // IMPORTANT: The deletion_condition is stored as trusted SQL from migrations
        // We use a parameterized batch delete with LIMIT to avoid long locks
        let delete_sql = format!(
            r#"
            DELETE FROM {}
            WHERE ctid IN (
                SELECT ctid FROM {}
                WHERE {}
                LIMIT 10000
            )
            "#,
            policy.table_name,
            policy.table_name,
            policy.deletion_condition
        );

        let mut total_deleted: i64 = 0;

        // Execute in batches to avoid long-running transactions
        loop {
            let result = sqlx::query(&delete_sql)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::Database(format!(
                    "Failed to cleanup {}: {}",
                    policy.table_name, e
                )))?;

            let deleted = result.rows_affected() as i64;
            total_deleted += deleted;

            // If we deleted fewer than the batch size, we're done
            if deleted < 10000 {
                break;
            }
        }

        Ok(total_deleted)
    }

    async fn archive_before_cleanup(&self, policy: &RetentionPolicy) -> Result<i64, AppError> {
        // R5-16: Validate table is in allowlist before any archive operation
        if !RETENTION_ALLOWED_TABLES.contains(&policy.table_name.as_str()) {
            tracing::error!(
                table = %policy.table_name,
                "SECURITY: Attempted archive on protected table - BLOCKED"
            );
            return Err(AppError::BadRequest(format!(
                "Table '{}' is not in the retention allowlist",
                policy.table_name
            )));
        }

        // R5-16: Validate table name contains only safe characters
        if !policy.table_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(AppError::BadRequest(format!(
                "Invalid table name '{}': must contain only alphanumeric characters and underscores",
                policy.table_name
            )));
        }

        // Only archive if configured to do so
        if !policy.archive_before_delete {
            return Ok(0);
        }

        // Skip if deletion_condition is empty or 'FALSE'
        if policy.deletion_condition.is_empty()
            || policy.deletion_condition.to_uppercase() == "FALSE"
        {
            return Ok(0);
        }

        // Archive table naming convention: {table_name}_archive
        let archive_table = format!("{}_archive", policy.table_name);

        // Check if archive table exists
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM information_schema.tables
                WHERE table_name = $1
            )
            "#
        )
        .bind(&archive_table)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if !exists {
            tracing::warn!(
                table = %policy.table_name,
                archive_table = %archive_table,
                "Archive table does not exist, skipping archival"
            );
            return Ok(0);
        }

        // Insert into archive with batch processing
        let archive_sql = format!(
            r#"
            INSERT INTO {}
            SELECT * FROM {}
            WHERE {}
            ON CONFLICT DO NOTHING
            "#,
            archive_table,
            policy.table_name,
            policy.deletion_condition
        );

        let result = sqlx::query(&archive_sql)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(format!(
                "Failed to archive {}: {}",
                policy.table_name, e
            )))?;

        Ok(result.rows_affected() as i64)
    }

    async fn upsert_policy(
        &self,
        table_name: &str,
        retention_days: Option<i32>,
        archive_before_delete: bool,
        deletion_condition: &str,
    ) -> Result<RetentionPolicy, AppError> {
        let row = sqlx::query_as::<_, (
            i32, String, Option<i32>, bool, String,
            Option<DateTime<Utc>>, i32, DateTime<Utc>, DateTime<Utc>
        )>(
            r#"
            INSERT INTO retention_policies (table_name, retention_days, archive_before_delete, deletion_condition)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (table_name) DO UPDATE SET
                retention_days = EXCLUDED.retention_days,
                archive_before_delete = EXCLUDED.archive_before_delete,
                deletion_condition = EXCLUDED.deletion_condition,
                updated_at = NOW()
            RETURNING id, table_name, retention_days, archive_before_delete, deletion_condition,
                      last_cleanup_at, rows_deleted_last, created_at, updated_at
            "#
        )
        .bind(table_name)
        .bind(retention_days)
        .bind(archive_before_delete)
        .bind(deletion_condition)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(RetentionPolicy {
            id: row.0,
            table_name: row.1,
            retention_days: row.2,
            archive_before_delete: row.3,
            deletion_condition: row.4,
            last_cleanup_at: row.5,
            rows_deleted_last: row.6,
            created_at: row.7,
            updated_at: row.8,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_result_success() {
        let result = RetentionCleanupResult::success("visitors", 100, 0, 50);
        assert_eq!(result.table_name, "visitors");
        assert_eq!(result.rows_deleted, 100);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_cleanup_result_failure() {
        let result = RetentionCleanupResult::failure("visitors", "Connection error");
        assert_eq!(result.table_name, "visitors");
        assert_eq!(result.rows_deleted, 0);
        assert!(result.error.is_some());
    }
}
