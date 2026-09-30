//! PostgreSQL implementation of SyncJobRepositoryTrait

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

use crate::models::entity::{NewSyncJob, SyncJobEntity};
use crate::repository::traits::SyncJobRepositoryTrait;

/// Row struct for deserializing sync_jobs table from PostgreSQL
#[derive(sqlx::FromRow)]
struct SyncJobRow {
    id: String,
    job_type: String,
    target_date: String,
    status: String,
    attempt_count: i32,
    max_attempts: i32,
    records_fetched: i32,
    records_inserted: i32,
    error_message: Option<String>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    next_retry_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    job_context: Option<String>,
    duration_ms: Option<i64>,
    job_logs: Option<String>,
}

impl From<SyncJobRow> for SyncJobEntity {
    fn from(row: SyncJobRow) -> Self {
        SyncJobEntity {
            id: row.id,
            job_type: row.job_type,
            target_date: row.target_date,
            status: row.status,
            attempt_count: row.attempt_count,
            max_attempts: row.max_attempts,
            records_fetched: row.records_fetched,
            records_inserted: row.records_inserted,
            error_message: row.error_message,
            started_at: row.started_at.map(|dt| dt.to_rfc3339()),
            completed_at: row.completed_at.map(|dt| dt.to_rfc3339()),
            next_retry_at: row.next_retry_at.map(|dt| dt.to_rfc3339()),
            created_at: row.created_at.to_rfc3339(),
            updated_at: row.updated_at.to_rfc3339(),
            job_context: row.job_context,
            duration_ms: row.duration_ms,
            job_logs: row.job_logs,
        }
    }
}

/// PostgreSQL implementation of the SyncJob repository
pub struct PgSyncJobRepository {
    pool: Arc<PgPool>,
}

impl PgSyncJobRepository {
    /// Create a new PgSyncJobRepository with the given connection pool
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SyncJobRepositoryTrait for PgSyncJobRepository {
    async fn create_job(&self, new_job: NewSyncJob) -> Result<SyncJobEntity, String> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();

        let row = sqlx::query_as::<_, SyncJobRow>(
            r#"
            INSERT INTO sync_jobs (
                id, job_type, target_date, status, attempt_count, max_attempts,
                records_fetched, records_inserted, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, job_type, target_date, status, attempt_count, max_attempts,
                      records_fetched, records_inserted, error_message, started_at,
                      completed_at, next_retry_at, created_at, updated_at,
                      job_context, duration_ms, job_logs
            "#,
        )
        .bind(&id)
        .bind(&new_job.job_type)
        .bind(&new_job.target_date)
        .bind("pending")
        .bind(0i32)
        .bind(new_job.max_attempts)
        .bind(0i32)
        .bind(0i32)
        .bind(now)
        .bind(now)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to create sync job: {}", e);
            e.to_string()
        })?;

        debug!("Created sync job: {}", id);

        Ok(row.into())
    }

    async fn get_job_by_id(&self, id: &str) -> Result<Option<SyncJobEntity>, String> {
        let row = sqlx::query_as::<_, SyncJobRow>(
            r#"
            SELECT id, job_type, target_date, status, attempt_count, max_attempts,
                   records_fetched, records_inserted, error_message, started_at,
                   completed_at, next_retry_at, created_at, updated_at,
                   job_context, duration_ms, job_logs
            FROM sync_jobs
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|r| r.into()))
    }

    async fn get_job_by_date(&self, job_type: &str, target_date: &str) -> Result<Option<SyncJobEntity>, String> {
        let row = sqlx::query_as::<_, SyncJobRow>(
            r#"
            SELECT id, job_type, target_date, status, attempt_count, max_attempts,
                   records_fetched, records_inserted, error_message, started_at,
                   completed_at, next_retry_at, created_at, updated_at,
                   job_context, duration_ms, job_logs
            FROM sync_jobs
            WHERE job_type = $1 AND target_date = $2
            "#,
        )
        .bind(job_type)
        .bind(target_date)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|r| r.into()))
    }

    async fn list_jobs(&self) -> Result<Vec<SyncJobEntity>, String> {
        let rows = sqlx::query_as::<_, SyncJobRow>(
            r#"
            SELECT id, job_type, target_date, status, attempt_count, max_attempts,
                   records_fetched, records_inserted, error_message, started_at,
                   completed_at, next_retry_at, created_at, updated_at,
                   job_context, duration_ms, job_logs
            FROM sync_jobs
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn list_jobs_by_status(&self, status: &str) -> Result<Vec<SyncJobEntity>, String> {
        let rows = sqlx::query_as::<_, SyncJobRow>(
            r#"
            SELECT id, job_type, target_date, status, attempt_count, max_attempts,
                   records_fetched, records_inserted, error_message, started_at,
                   completed_at, next_retry_at, created_at, updated_at,
                   job_context, duration_ms, job_logs
            FROM sync_jobs
            WHERE status = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(status)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn list_pending_retries(&self) -> Result<Vec<SyncJobEntity>, String> {
        let now = Utc::now();

        let rows = sqlx::query_as::<_, SyncJobRow>(
            r#"
            SELECT id, job_type, target_date, status, attempt_count, max_attempts,
                   records_fetched, records_inserted, error_message, started_at,
                   completed_at, next_retry_at, created_at, updated_at,
                   job_context, duration_ms, job_logs
            FROM sync_jobs
            WHERE status = 'retrying' AND next_retry_at <= $1
            ORDER BY next_retry_at ASC
            "#,
        )
        .bind(now)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn update_status(&self, id: &str, status: &str) -> Result<(), String> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE sync_jobs
            SET status = $1, updated_at = $2
            WHERE id = $3
            "#,
        )
        .bind(status)
        .bind(now)
        .bind(id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn mark_running(&self, id: &str) -> Result<(), String> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE sync_jobs
            SET status = 'running', started_at = $1, updated_at = $2
            WHERE id = $3
            "#,
        )
        .bind(now)
        .bind(now)
        .bind(id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn mark_completed(&self, id: &str, records_fetched: i32, records_inserted: i32) -> Result<(), String> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE sync_jobs
            SET status = 'completed', records_fetched = $1, records_inserted = $2,
                completed_at = $3, updated_at = $4
            WHERE id = $5
            "#,
        )
        .bind(records_fetched)
        .bind(records_inserted)
        .bind(now)
        .bind(now)
        .bind(id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn mark_completed_with_context(
        &self,
        id: &str,
        records_fetched: i32,
        records_inserted: i32,
        context: Option<&str>,
        duration_ms: i64,
        logs: Option<&str>,
    ) -> Result<(), String> {
        let now = Utc::now();

        // Clear error_message on successful completion
        sqlx::query(
            r#"
            UPDATE sync_jobs
            SET status = 'completed', records_fetched = $1, records_inserted = $2,
                completed_at = $3, updated_at = $4, job_context = $5,
                duration_ms = $6, error_message = NULL, job_logs = $7
            WHERE id = $8
            "#,
        )
        .bind(records_fetched)
        .bind(records_inserted)
        .bind(now)
        .bind(now)
        .bind(context)
        .bind(duration_ms)
        .bind(logs)
        .bind(id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn mark_failed(&self, id: &str, error_message: &str) -> Result<(), String> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE sync_jobs
            SET status = 'failed', error_message = $1, completed_at = $2, updated_at = $3
            WHERE id = $4
            "#,
        )
        .bind(error_message)
        .bind(now)
        .bind(now)
        .bind(id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn mark_retrying(&self, id: &str, attempt_count: i32, error_message: &str, next_retry_at: &str) -> Result<(), String> {
        let now = Utc::now();

        // Parse the next_retry_at string to DateTime<Utc>
        let next_retry_ts = DateTime::parse_from_rfc3339(next_retry_at)
            .map(|d| d.with_timezone(&Utc))
            .map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            UPDATE sync_jobs
            SET status = 'retrying', attempt_count = $1, error_message = $2,
                next_retry_at = $3, updated_at = $4
            WHERE id = $5
            "#,
        )
        .bind(attempt_count)
        .bind(error_message)
        .bind(next_retry_ts)
        .bind(now)
        .bind(id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete_job(&self, id: &str) -> Result<bool, String> {
        let result = sqlx::query(
            r#"
            DELETE FROM sync_jobs
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.rows_affected() > 0)
    }

    async fn reset_job(&self, id: &str) -> Result<(), String> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE sync_jobs
            SET status = 'pending', attempt_count = 0, error_message = NULL,
                started_at = NULL, completed_at = NULL, next_retry_at = NULL,
                records_fetched = 0, records_inserted = 0, updated_at = $1
            WHERE id = $2
            "#,
        )
        .bind(now)
        .bind(id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn update_job_context(&self, id: &str, context: &str) -> Result<(), String> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE sync_jobs
            SET job_context = $1, updated_at = $2
            WHERE id = $3
            "#,
        )
        .bind(context)
        .bind(now)
        .bind(id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Integration tests would require a running PostgreSQL database
    // and should be placed in a separate integration test file.

    #[test]
    fn test_sync_job_row_conversion() {
        let now = Utc::now();
        let row = SyncJobRow {
            id: "test-id".to_string(),
            job_type: "rewards_sync".to_string(),
            target_date: "2024-01-15".to_string(),
            status: "pending".to_string(),
            attempt_count: 0,
            max_attempts: 3,
            records_fetched: 0,
            records_inserted: 0,
            error_message: None,
            started_at: None,
            completed_at: None,
            next_retry_at: None,
            created_at: now,
            updated_at: now,
            job_context: None,
            duration_ms: None,
            job_logs: None,
        };

        let entity: SyncJobEntity = row.into();

        assert_eq!(entity.id, "test-id");
        assert_eq!(entity.job_type, "rewards_sync");
        assert_eq!(entity.target_date, "2024-01-15");
        assert_eq!(entity.status, "pending");
        assert_eq!(entity.attempt_count, 0);
        assert_eq!(entity.max_attempts, 3);
        assert!(entity.started_at.is_none());
        assert!(entity.completed_at.is_none());
    }
}
