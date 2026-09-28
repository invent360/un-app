//! Trait for sync job repository operations

use async_trait::async_trait;
use crate::models::entity::{NewSyncJob, SyncJobEntity};

/// Trait defining sync job repository operations
#[async_trait]
pub trait SyncJobRepositoryTrait: Send + Sync {
    /// Create a new sync job
    async fn create_job(&self, new_job: NewSyncJob) -> Result<SyncJobEntity, String>;

    /// Get a sync job by ID
    async fn get_job_by_id(&self, id: &str) -> Result<Option<SyncJobEntity>, String>;

    /// Get a sync job by target date and job type
    async fn get_job_by_date(&self, job_type: &str, target_date: &str) -> Result<Option<SyncJobEntity>, String>;

    /// List all sync jobs, ordered by created_at descending
    async fn list_jobs(&self) -> Result<Vec<SyncJobEntity>, String>;

    /// List jobs by status
    async fn list_jobs_by_status(&self, status: &str) -> Result<Vec<SyncJobEntity>, String>;

    /// List jobs that are ready to retry (next_retry_at <= now)
    async fn list_pending_retries(&self) -> Result<Vec<SyncJobEntity>, String>;

    /// Update job status
    async fn update_status(&self, id: &str, status: &str) -> Result<(), String>;

    /// Mark job as running
    async fn mark_running(&self, id: &str) -> Result<(), String>;

    /// Mark job as completed with record counts
    async fn mark_completed(&self, id: &str, records_fetched: i32, records_inserted: i32) -> Result<(), String>;

    /// Mark job as completed with context, duration, and logs
    async fn mark_completed_with_context(
        &self,
        id: &str,
        records_fetched: i32,
        records_inserted: i32,
        context: Option<&str>,
        duration_ms: i64,
        logs: Option<&str>,
    ) -> Result<(), String>;

    /// Mark job as failed
    async fn mark_failed(&self, id: &str, error_message: &str) -> Result<(), String>;

    /// Mark job as retrying with next retry time
    async fn mark_retrying(&self, id: &str, attempt_count: i32, error_message: &str, next_retry_at: &str) -> Result<(), String>;

    /// Delete a sync job
    async fn delete_job(&self, id: &str) -> Result<bool, String>;

    /// Reset a job for re-execution (sets status to pending, resets attempt count)
    async fn reset_job(&self, id: &str) -> Result<(), String>;

    /// Update job context (for progress tracking during sync)
    async fn update_job_context(&self, id: &str, context: &str) -> Result<(), String>;
}
