//! ScyllaDB implementation of SyncJobRepositoryTrait

use async_trait::async_trait;
use chrono::Utc;
use scylla::frame::value::CqlTimestamp;
use scylla::DeserializeRow;
use scylla::Session;
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::entity::{NewSyncJob, SyncJobEntity};
use crate::repository::traits::SyncJobRepositoryTrait;

/// Row struct for deserializing sync_jobs table
/// Using struct instead of tuple to support >16 fields
#[derive(DeserializeRow)]
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
    started_at: Option<CqlTimestamp>,
    completed_at: Option<CqlTimestamp>,
    next_retry_at: Option<CqlTimestamp>,
    created_at: CqlTimestamp,
    updated_at: CqlTimestamp,
    job_context: Option<String>,
    duration_ms: Option<i64>,
    job_logs: Option<String>,
}

/// ScyllaDB implementation of the SyncJob repository
pub struct SyncJobRepository {
    session: Arc<Session>,
}

impl SyncJobRepository {
    /// Create a new SyncJobRepository with the given session
    pub fn new(session: DbPool) -> Self {
        Self { session }
    }

    fn timestamp_to_string(ts: CqlTimestamp) -> String {
        chrono::DateTime::from_timestamp_millis(ts.0)
            .map(|d| d.to_rfc3339())
            .unwrap_or_else(|| Utc::now().to_rfc3339())
    }

    fn optional_timestamp_to_string(ts: Option<CqlTimestamp>) -> Option<String> {
        ts.map(Self::timestamp_to_string)
    }

    fn parse_job(result: scylla::QueryResult) -> Result<Option<SyncJobEntity>, String> {
        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        let rows = match rows_result.rows::<SyncJobRow>() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        for row_result in rows {
            if let Ok(row) = row_result {
                return Ok(Some(SyncJobEntity {
                    id: row.id,
                    job_type: row.job_type,
                    target_date: row.target_date,
                    status: row.status,
                    attempt_count: row.attempt_count,
                    max_attempts: row.max_attempts,
                    records_fetched: row.records_fetched,
                    records_inserted: row.records_inserted,
                    error_message: row.error_message,
                    started_at: Self::optional_timestamp_to_string(row.started_at),
                    completed_at: Self::optional_timestamp_to_string(row.completed_at),
                    next_retry_at: Self::optional_timestamp_to_string(row.next_retry_at),
                    created_at: Self::timestamp_to_string(row.created_at),
                    updated_at: Self::timestamp_to_string(row.updated_at),
                    job_context: row.job_context,
                    duration_ms: row.duration_ms,
                    job_logs: row.job_logs,
                }));
            }
        }

        Ok(None)
    }

    fn parse_jobs(result: scylla::QueryResult) -> Result<Vec<SyncJobEntity>, String> {
        let mut jobs = Vec::new();

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(jobs),
        };

        let rows = match rows_result.rows::<SyncJobRow>() {
            Ok(r) => r,
            Err(_) => return Ok(jobs),
        };

        for row_result in rows {
            if let Ok(row) = row_result {
                jobs.push(SyncJobEntity {
                    id: row.id,
                    job_type: row.job_type,
                    target_date: row.target_date,
                    status: row.status,
                    attempt_count: row.attempt_count,
                    max_attempts: row.max_attempts,
                    records_fetched: row.records_fetched,
                    records_inserted: row.records_inserted,
                    error_message: row.error_message,
                    started_at: Self::optional_timestamp_to_string(row.started_at),
                    completed_at: Self::optional_timestamp_to_string(row.completed_at),
                    next_retry_at: Self::optional_timestamp_to_string(row.next_retry_at),
                    created_at: Self::timestamp_to_string(row.created_at),
                    updated_at: Self::timestamp_to_string(row.updated_at),
                    job_context: row.job_context,
                    duration_ms: row.duration_ms,
                    job_logs: row.job_logs,
                });
            }
        }

        Ok(jobs)
    }
}

#[async_trait]
impl SyncJobRepositoryTrait for SyncJobRepository {
    async fn create_job(&self, new_job: NewSyncJob) -> Result<SyncJobEntity, String> {
        let now = Utc::now();
        let now_ts = CqlTimestamp(now.timestamp_millis());
        let id = Uuid::new_v4().to_string();

        // Only insert essential fields - other fields will default to NULL
        let query = "INSERT INTO sync_jobs (id, job_type, target_date, status, attempt_count, max_attempts, records_fetched, records_inserted, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        self.session
            .query_unpaged(
                query,
                (
                    &id,
                    &new_job.job_type,
                    &new_job.target_date,
                    "pending",
                    0i32,
                    new_job.max_attempts,
                    0i32,
                    0i32,
                    now_ts,
                    now_ts,
                ),
            )
            .await
            .map_err(|e| {
                error!("Failed to create sync job: {}", e);
                e.to_string()
            })?;

        debug!("Created sync job: {}", id);

        Ok(SyncJobEntity {
            id,
            job_type: new_job.job_type,
            target_date: new_job.target_date,
            status: "pending".to_string(),
            attempt_count: 0,
            max_attempts: new_job.max_attempts,
            records_fetched: 0,
            records_inserted: 0,
            error_message: None,
            started_at: None,
            completed_at: None,
            next_retry_at: None,
            created_at: now.to_rfc3339(),
            updated_at: now.to_rfc3339(),
            job_context: None,
            duration_ms: None,
            job_logs: None,
        })
    }

    async fn get_job_by_id(&self, id: &str) -> Result<Option<SyncJobEntity>, String> {
        let query = "SELECT id, job_type, target_date, status, attempt_count, max_attempts, records_fetched, records_inserted, error_message, started_at, completed_at, next_retry_at, created_at, updated_at, job_context, duration_ms, job_logs FROM sync_jobs WHERE id = ?";

        let result = self
            .session
            .query_unpaged(query, (id,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_job(result)
    }

    async fn get_job_by_date(&self, job_type: &str, target_date: &str) -> Result<Option<SyncJobEntity>, String> {
        let query = "SELECT id, job_type, target_date, status, attempt_count, max_attempts, records_fetched, records_inserted, error_message, started_at, completed_at, next_retry_at, created_at, updated_at, job_context, duration_ms, job_logs FROM sync_jobs WHERE job_type = ? AND target_date = ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (job_type, target_date))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_job(result)
    }

    async fn list_jobs(&self) -> Result<Vec<SyncJobEntity>, String> {
        let query = "SELECT id, job_type, target_date, status, attempt_count, max_attempts, records_fetched, records_inserted, error_message, started_at, completed_at, next_retry_at, created_at, updated_at, job_context, duration_ms, job_logs FROM sync_jobs";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        let mut jobs = Self::parse_jobs(result)?;
        // Sort by created_at descending (most recent first)
        jobs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(jobs)
    }

    async fn list_jobs_by_status(&self, status: &str) -> Result<Vec<SyncJobEntity>, String> {
        let query = "SELECT id, job_type, target_date, status, attempt_count, max_attempts, records_fetched, records_inserted, error_message, started_at, completed_at, next_retry_at, created_at, updated_at, job_context, duration_ms, job_logs FROM sync_jobs WHERE status = ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (status,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_jobs(result)
    }

    async fn list_pending_retries(&self) -> Result<Vec<SyncJobEntity>, String> {
        // Get all retrying jobs and filter by next_retry_at in memory
        let jobs = self.list_jobs_by_status("retrying").await?;
        let now = Utc::now().to_rfc3339();

        Ok(jobs
            .into_iter()
            .filter(|j| {
                j.next_retry_at
                    .as_ref()
                    .map(|t| t <= &now)
                    .unwrap_or(false)
            })
            .collect())
    }

    async fn update_status(&self, id: &str, status: &str) -> Result<(), String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        let query = "UPDATE sync_jobs SET status = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(query, (status, now_ts, id))
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn mark_running(&self, id: &str) -> Result<(), String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        let query = "UPDATE sync_jobs SET status = ?, started_at = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(query, ("running", now_ts, now_ts, id))
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn mark_completed(&self, id: &str, records_fetched: i32, records_inserted: i32) -> Result<(), String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        let query = "UPDATE sync_jobs SET status = ?, records_fetched = ?, records_inserted = ?, completed_at = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(query, ("completed", records_fetched, records_inserted, now_ts, now_ts, id))
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
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        // Clear error_message on successful completion
        let query = "UPDATE sync_jobs SET status = ?, records_fetched = ?, records_inserted = ?, completed_at = ?, updated_at = ?, job_context = ?, duration_ms = ?, error_message = ?, job_logs = ? WHERE id = ?";

        self.session
            .query_unpaged(
                query,
                (
                    "completed",
                    records_fetched,
                    records_inserted,
                    now_ts,
                    now_ts,
                    context,
                    duration_ms,
                    None::<String>, // Clear error_message on success
                    logs,
                    id,
                ),
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn mark_failed(&self, id: &str, error_message: &str) -> Result<(), String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        let query = "UPDATE sync_jobs SET status = ?, error_message = ?, completed_at = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(query, ("failed", error_message, now_ts, now_ts, id))
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn mark_retrying(&self, id: &str, attempt_count: i32, error_message: &str, next_retry_at: &str) -> Result<(), String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        // Parse the next_retry_at string to timestamp
        let next_retry_ts = chrono::DateTime::parse_from_rfc3339(next_retry_at)
            .map(|d| CqlTimestamp(d.timestamp_millis()))
            .map_err(|e| e.to_string())?;

        let query = "UPDATE sync_jobs SET status = ?, attempt_count = ?, error_message = ?, next_retry_at = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(query, ("retrying", attempt_count, error_message, next_retry_ts, now_ts, id))
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete_job(&self, id: &str) -> Result<bool, String> {
        let query = "DELETE FROM sync_jobs WHERE id = ?";

        self.session
            .query_unpaged(query, (id,))
            .await
            .map_err(|e| e.to_string())?;

        Ok(true)
    }

    async fn reset_job(&self, id: &str) -> Result<(), String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        let query = "UPDATE sync_jobs SET status = ?, attempt_count = ?, error_message = ?, started_at = ?, completed_at = ?, next_retry_at = ?, records_fetched = ?, records_inserted = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(
                query,
                (
                    "pending",
                    0i32,
                    None::<String>,
                    None::<CqlTimestamp>,
                    None::<CqlTimestamp>,
                    None::<CqlTimestamp>,
                    0i32,
                    0i32,
                    now_ts,
                    id,
                ),
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn update_job_context(&self, id: &str, context: &str) -> Result<(), String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        let query = "UPDATE sync_jobs SET job_context = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(query, (context, now_ts, id))
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
