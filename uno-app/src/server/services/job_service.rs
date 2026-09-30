//! Job Queue Service for durable work processing
//!
//! Provides high-level API for:
//! - Creating jobs with transactional outbox support
//! - Processing jobs with lease management
//! - Retry handling with exponential backoff
//! - Dead letter queue management
//! - Worker coordination

use chrono::{DateTime, Duration, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::server::repositories::{
    DynOutboxRepository, OutboxRepository, CreateOutboxEvent, OutboxEvent,
    DynImmutableAuditRepository, AuditEventBuilder, AuditCategory, AuditEventType, AuditOutcome, ActorType,
};
use crate::types::AppError;

/// Job priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl JobPriority {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

/// Job status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    DeadLetter,
}

impl JobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::DeadLetter => "dead_letter",
        }
    }
}

/// Job outcome classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobOutcome {
    Success,
    Failure,
    Timeout,
    Unknown,
}

/// Configuration for job execution
#[derive(Debug, Clone)]
pub struct JobConfig {
    pub max_attempts: i32,
    pub lease_duration_secs: i64,
    pub initial_retry_delay_secs: i64,
    pub max_retry_delay_secs: i64,
    pub backoff_multiplier: f64,
}

impl Default for JobConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            lease_duration_secs: 300,       // 5 minutes
            initial_retry_delay_secs: 10,   // 10 seconds
            max_retry_delay_secs: 3600,     // 1 hour max
            backoff_multiplier: 3.0,
        }
    }
}

/// Input for creating a new job
#[derive(Debug, Clone)]
pub struct CreateJobInput<T: Serialize> {
    pub job_type: String,
    pub payload: T,
    pub priority: JobPriority,
    pub idempotency_key: Option<String>,
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
    pub config: Option<JobConfig>,
}

/// Job entity with full details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub job_type: String,
    pub payload: JsonValue,
    pub status: String,
    pub priority: i32,
    pub attempts: i32,
    pub max_attempts: i32,
    pub worker_id: Option<String>,
    pub lease_expires_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub retry_at: Option<DateTime<Utc>>,
    pub idempotency_key: Option<String>,
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
    pub outcome: Option<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Result of job processing
#[derive(Debug)]
pub struct JobResult {
    pub success: bool,
    pub result: Option<JsonValue>,
    pub error: Option<String>,
}

impl JobResult {
    pub fn success(result: impl Serialize) -> Self {
        Self {
            success: true,
            result: serde_json::to_value(result).ok(),
            error: None,
        }
    }

    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            success: false,
            result: None,
            error: Some(error.into()),
        }
    }
}

/// Job service for durable work processing
#[derive(Clone)]
pub struct JobService {
    pool: ConnectionPool,
    outbox_repo: DynOutboxRepository,
    audit_repo: DynImmutableAuditRepository,
    config: JobConfig,
}

impl JobService {
    pub fn new(
        pool: ConnectionPool,
        outbox_repo: DynOutboxRepository,
        audit_repo: DynImmutableAuditRepository,
    ) -> Self {
        Self {
            pool,
            outbox_repo,
            audit_repo,
            config: JobConfig::default(),
        }
    }

    pub fn with_config(mut self, config: JobConfig) -> Self {
        self.config = config;
        self
    }

    /// Create a new job (persisted transactionally with business mutation)
    ///
    /// Call this within the same database transaction as your business logic
    /// to ensure atomic persistence of both the domain change and the job.
    pub async fn create_job<T: Serialize>(
        &self,
        input: CreateJobInput<T>,
    ) -> Result<Job, AppError> {
        let job_id = Uuid::new_v4().to_string();
        let config = input.config.unwrap_or_else(|| self.config.clone());
        let payload = serde_json::to_value(&input.payload)
            .map_err(|e| AppError::ValidationError(format!("Invalid payload: {}", e)))?;

        let job = sqlx::query_as::<_, JobRow>(
            r#"
            INSERT INTO job_queue (
                id, job_type, payload, payload_version, status, priority,
                max_attempts, idempotency_key, correlation_id, causation_id,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, 1, 'pending', $4, $5, $6, $7, $8, NOW(), NOW())
            RETURNING id, job_type, payload, status, priority, attempts, max_attempts,
                      worker_id, lease_expires_at, error_message, retry_at,
                      idempotency_key, correlation_id, causation_id, outcome,
                      created_at, started_at, completed_at
            "#,
        )
        .bind(&job_id)
        .bind(&input.job_type)
        .bind(&payload.to_string())
        .bind(input.priority.as_i32())
        .bind(config.max_attempts)
        .bind(input.idempotency_key.as_deref())
        .bind(input.correlation_id.as_deref())
        .bind(input.causation_id.as_deref())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("Job with this idempotency key already exists".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        tracing::info!(
            job_id = %job_id,
            job_type = %input.job_type,
            priority = ?input.priority,
            "Job created"
        );

        Ok(job.into_job())
    }

    /// Claim a job for processing (acquires lease)
    ///
    /// Returns None if no jobs are available.
    pub async fn claim_job(&self, worker_id: Uuid, job_types: &[&str]) -> Result<Option<Job>, AppError> {
        let lease_expires = Utc::now() + Duration::seconds(self.config.lease_duration_secs);

        // Build job type filter
        let type_filter = if job_types.is_empty() {
            "TRUE".to_string()
        } else {
            format!("job_type = ANY($3)")
        };

        let query = format!(
            r#"
            UPDATE job_queue
            SET status = 'running',
                worker_id = $1,
                lease_expires_at = $2,
                started_at = COALESCE(started_at, NOW()),
                attempts = attempts + 1
            WHERE id = (
                SELECT id FROM job_queue
                WHERE (status = 'pending' OR (status = 'failed' AND retry_at IS NOT NULL AND retry_at <= NOW()))
                  AND {}
                ORDER BY priority DESC, created_at ASC
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING id, job_type, payload, status, priority, attempts, max_attempts,
                      worker_id, lease_expires_at, error_message, retry_at,
                      idempotency_key, correlation_id, causation_id, outcome,
                      created_at, started_at, completed_at
            "#,
            type_filter
        );

        let result = if job_types.is_empty() {
            sqlx::query_as::<_, JobRow>(&query)
                .bind(worker_id.to_string())
                .bind(lease_expires)
                .fetch_optional(&self.pool)
                .await
        } else {
            sqlx::query_as::<_, JobRow>(&query)
                .bind(worker_id.to_string())
                .bind(lease_expires)
                .bind(job_types)
                .fetch_optional(&self.pool)
                .await
        };

        let job = result.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(job.map(|j| j.into_job()))
    }

    /// Extend the lease on a running job (heartbeat)
    pub async fn heartbeat_job(&self, job_id: &str, worker_id: Uuid) -> Result<bool, AppError> {
        let lease_expires = Utc::now() + Duration::seconds(self.config.lease_duration_secs);

        let result = sqlx::query(
            r#"
            UPDATE job_queue
            SET lease_expires_at = $2
            WHERE id = $1 AND worker_id = $3 AND status = 'running'
            "#,
        )
        .bind(job_id)
        .bind(lease_expires)
        .bind(worker_id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Complete a job successfully
    pub async fn complete_job(&self, job_id: &str, result: JobResult) -> Result<(), AppError> {
        let outcome = if result.success { "success" } else { "failure" };
        let result_json = result.result.map(|v| v.to_string());

        sqlx::query(
            r#"
            UPDATE job_queue
            SET status = 'completed',
                completed_at = NOW(),
                result = $2,
                outcome = $3,
                outcome_verified_at = NOW(),
                worker_id = NULL,
                lease_expires_at = NULL
            WHERE id = $1
            "#,
        )
        .bind(job_id)
        .bind(result_json)
        .bind(outcome)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(job_id = %job_id, outcome = %outcome, "Job completed");

        Ok(())
    }

    /// Fail a job (will retry if attempts < max_attempts)
    pub async fn fail_job(&self, job_id: &str, error: &str) -> Result<bool, AppError> {
        // Get current attempt count
        let job: Option<(i32, i32)> = sqlx::query_as(
            "SELECT attempts, max_attempts FROM job_queue WHERE id = $1"
        )
        .bind(job_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let (attempts, max_attempts) = match job {
            Some((a, m)) => (a, m),
            None => return Ok(false),
        };

        if attempts >= max_attempts {
            // Move to dead letter queue
            sqlx::query(
                r#"
                UPDATE job_queue
                SET status = 'dead_letter',
                    completed_at = NOW(),
                    error_message = $2,
                    outcome = 'failure',
                    worker_id = NULL,
                    lease_expires_at = NULL
                WHERE id = $1
                "#,
            )
            .bind(job_id)
            .bind(error)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            tracing::warn!(job_id = %job_id, error = %error, "Job moved to dead letter queue");

            // Audit dead letter
            let builder = AuditEventBuilder::new(
                AuditEventType::Custom("job.dead_letter".to_string()),
                AuditCategory::System,
            )
            .resource("job", job_id)
            .action("dead_letter")
            .outcome(AuditOutcome::Failure)
            .event_data(serde_json::json!({
                "error": error,
                "attempts": attempts,
            }));

            let _ = self.audit_repo.log_event(builder.build()).await;

            Ok(false)
        } else {
            // Schedule retry with exponential backoff
            let retry_delay = self.calculate_retry_delay(attempts);
            let retry_at = Utc::now() + retry_delay;

            sqlx::query(
                r#"
                UPDATE job_queue
                SET status = 'failed',
                    error_message = $2,
                    retry_at = $3,
                    worker_id = NULL,
                    lease_expires_at = NULL
                WHERE id = $1
                "#,
            )
            .bind(job_id)
            .bind(error)
            .bind(retry_at)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            tracing::info!(
                job_id = %job_id,
                error = %error,
                retry_at = %retry_at,
                attempt = attempts,
                "Job failed, scheduled for retry"
            );

            Ok(true)
        }
    }

    /// Reclaim expired leases (for jobs whose workers died)
    pub async fn reclaim_expired_leases(&self) -> Result<i64, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE job_queue
            SET status = 'pending',
                worker_id = NULL,
                lease_expires_at = NULL,
                error_message = 'Lease expired (worker may have died)'
            WHERE status = 'running'
              AND lease_expires_at < NOW()
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let count = result.rows_affected() as i64;

        if count > 0 {
            tracing::warn!(count = count, "Reclaimed expired job leases");
        }

        Ok(count)
    }

    /// Get dead letter jobs for manual inspection
    pub async fn get_dead_letter_jobs(&self, limit: i32, offset: i32) -> Result<Vec<Job>, AppError> {
        let jobs = sqlx::query_as::<_, JobRow>(
            r#"
            SELECT id, job_type, payload, status, priority, attempts, max_attempts,
                   worker_id, lease_expires_at, error_message, retry_at,
                   idempotency_key, correlation_id, causation_id, outcome,
                   created_at, started_at, completed_at
            FROM job_queue
            WHERE status = 'dead_letter'
            ORDER BY completed_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(jobs.into_iter().map(|j| j.into_job()).collect())
    }

    /// Replay a dead letter job (reset to pending)
    pub async fn replay_job(&self, job_id: &str, admin_id: &str) -> Result<Job, AppError> {
        let job = sqlx::query_as::<_, JobRow>(
            r#"
            UPDATE job_queue
            SET status = 'pending',
                attempts = 0,
                error_message = NULL,
                retry_at = NULL,
                completed_at = NULL,
                outcome = NULL
            WHERE id = $1 AND status = 'dead_letter'
            RETURNING id, job_type, payload, status, priority, attempts, max_attempts,
                      worker_id, lease_expires_at, error_message, retry_at,
                      idempotency_key, correlation_id, causation_id, outcome,
                      created_at, started_at, completed_at
            "#,
        )
        .bind(job_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Audit the replay
        let builder = AuditEventBuilder::new(
            AuditEventType::Custom("job.replay".to_string()),
            AuditCategory::System,
        )
        .actor(ActorType::Admin, admin_id.to_string())
        .resource("job", job_id)
        .action("replay")
        .outcome(AuditOutcome::Success);

        let _ = self.audit_repo.log_event(builder.build()).await;

        tracing::info!(job_id = %job_id, admin_id = %admin_id, "Job replayed from dead letter queue");

        Ok(job.into_job())
    }

    /// Create an outbox event (transactional with business mutation)
    pub async fn create_outbox_event(
        &self,
        event_type: &str,
        aggregate_type: &str,
        aggregate_id: &str,
        payload: impl Serialize,
        idempotency_key: Option<String>,
    ) -> Result<OutboxEvent, AppError> {
        let payload_json = serde_json::to_value(payload)
            .map_err(|e| AppError::ValidationError(format!("Invalid payload: {}", e)))?;

        self.outbox_repo.create_event(CreateOutboxEvent {
            event_type: event_type.to_string(),
            event_version: 1,
            aggregate_type: aggregate_type.to_string(),
            aggregate_id: aggregate_id.to_string(),
            payload: payload_json,
            metadata: None,
            idempotency_key,
        }).await
    }

    fn calculate_retry_delay(&self, attempts: i32) -> Duration {
        let base = self.config.initial_retry_delay_secs as f64;
        let multiplier = self.config.backoff_multiplier.powi(attempts);
        let delay_secs = (base * multiplier).min(self.config.max_retry_delay_secs as f64) as i64;
        Duration::seconds(delay_secs)
    }
}

// Internal row type for database mapping
#[derive(Debug, Clone, sqlx::FromRow)]
struct JobRow {
    id: String,
    job_type: String,
    payload: String,
    status: String,
    priority: i32,
    attempts: i32,
    max_attempts: i32,
    worker_id: Option<String>,
    lease_expires_at: Option<DateTime<Utc>>,
    error_message: Option<String>,
    retry_at: Option<DateTime<Utc>>,
    idempotency_key: Option<String>,
    correlation_id: Option<String>,
    causation_id: Option<String>,
    outcome: Option<String>,
    created_at: DateTime<Utc>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
}

impl JobRow {
    fn into_job(self) -> Job {
        Job {
            id: self.id,
            job_type: self.job_type,
            payload: serde_json::from_str(&self.payload).unwrap_or(JsonValue::Null),
            status: self.status,
            priority: self.priority,
            attempts: self.attempts,
            max_attempts: self.max_attempts,
            worker_id: self.worker_id,
            lease_expires_at: self.lease_expires_at,
            error_message: self.error_message,
            retry_at: self.retry_at,
            idempotency_key: self.idempotency_key,
            correlation_id: self.correlation_id,
            causation_id: self.causation_id,
            outcome: self.outcome,
            created_at: self.created_at,
            started_at: self.started_at,
            completed_at: self.completed_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_priority_values() {
        assert_eq!(JobPriority::Low.as_i32(), 0);
        assert_eq!(JobPriority::Normal.as_i32(), 1);
        assert_eq!(JobPriority::High.as_i32(), 2);
        assert_eq!(JobPriority::Critical.as_i32(), 3);
    }

    #[test]
    fn test_job_status_conversion() {
        assert_eq!(JobStatus::Pending.as_str(), "pending");
        assert_eq!(JobStatus::Running.as_str(), "running");
        assert_eq!(JobStatus::DeadLetter.as_str(), "dead_letter");
    }

    #[test]
    fn test_job_result_success() {
        let result = JobResult::success(serde_json::json!({"count": 42}));
        assert!(result.success);
        assert!(result.result.is_some());
        assert!(result.error.is_none());
    }

    #[test]
    fn test_job_result_failure() {
        let result = JobResult::failure("Something went wrong");
        assert!(!result.success);
        assert!(result.result.is_none());
        assert_eq!(result.error, Some("Something went wrong".to_string()));
    }
}
