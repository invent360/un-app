//! Durable job queue types and traits
//!
//! Provides types for durable job persistence with:
//! - Worker leases for distributed execution
//! - Bounded retries with exponential backoff
//! - Dead-letter queue for failed jobs
//! - Exactly-once semantics via idempotency keys
//!
//! Note: The actual database implementation uses ScyllaDB via
//! the existing SyncJobRepository infrastructure.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration as StdDuration;

/// Job status in the queue
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    /// Job is waiting to be picked up
    Pending,
    /// Job is being processed by a worker
    Running,
    /// Job completed successfully
    Completed,
    /// Job failed and may be retried
    Failed,
    /// Job is scheduled for retry
    Retrying,
    /// Job exceeded max retries, moved to dead-letter
    DeadLetter,
}

impl JobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Retrying => "retrying",
            Self::DeadLetter => "dead_letter",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "running" => Some(Self::Running),
            "completed" => Some(Self::Completed),
            "failed" => Some(Self::Failed),
            "retrying" => Some(Self::Retrying),
            "dead_letter" => Some(Self::DeadLetter),
            _ => None,
        }
    }

    /// Check if job is in a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::DeadLetter)
    }

    /// Check if job can be retried
    pub fn can_retry(&self) -> bool {
        matches!(self, Self::Failed | Self::Retrying)
    }
}

/// Job priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum JobPriority {
    Low = 0,
    #[default]
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl JobPriority {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::Low,
            2 => Self::High,
            3 => Self::Critical,
            _ => Self::Normal,
        }
    }
}

/// A job in the durable queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurableJob {
    /// Unique job identifier
    pub id: String,
    /// Job type (e.g., "claims_sync", "rewards_sync")
    pub job_type: String,
    /// JSON payload for the job
    pub payload: serde_json::Value,
    /// Current status
    pub status: JobStatus,
    /// Number of execution attempts
    pub attempts: i32,
    /// Maximum allowed attempts before dead-letter
    pub max_attempts: i32,
    /// Priority (higher = picked up first)
    pub priority: JobPriority,
    /// Worker ID that has the lease (None if not running)
    pub worker_id: Option<String>,
    /// When the lease expires (None if not running)
    pub lease_expires_at: Option<DateTime<Utc>>,
    /// Error message from last failure
    pub error_message: Option<String>,
    /// When to retry (for failed jobs with retries remaining)
    pub retry_at: Option<DateTime<Utc>>,
    /// Idempotency key for exactly-once semantics
    pub idempotency_key: Option<String>,
    /// When the job was created
    pub created_at: DateTime<Utc>,
    /// When the job was last updated
    pub updated_at: DateTime<Utc>,
    /// When the job started running
    pub started_at: Option<DateTime<Utc>>,
    /// When the job completed/failed
    pub completed_at: Option<DateTime<Utc>>,
    /// Job result (if completed)
    pub result: Option<serde_json::Value>,
    /// Job context for progress tracking
    pub context: Option<serde_json::Value>,
    /// Execution logs
    pub logs: Option<String>,
}

impl DurableJob {
    /// Create a new pending job
    pub fn new(job_type: impl Into<String>, payload: serde_json::Value) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            job_type: job_type.into(),
            payload,
            status: JobStatus::Pending,
            attempts: 0,
            max_attempts: 3,
            priority: JobPriority::Normal,
            worker_id: None,
            lease_expires_at: None,
            error_message: None,
            retry_at: None,
            idempotency_key: None,
            created_at: now,
            updated_at: now,
            started_at: None,
            completed_at: None,
            result: None,
            context: None,
            logs: None,
        }
    }

    /// Set maximum retry attempts
    pub fn with_max_attempts(mut self, max: i32) -> Self {
        self.max_attempts = max;
        self
    }

    /// Set job priority
    pub fn with_priority(mut self, priority: JobPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set idempotency key for exactly-once semantics
    pub fn with_idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());
        self
    }

    /// Calculate next retry delay using exponential backoff
    ///
    /// Base delay: 30 seconds, doubles with each attempt
    /// Max delay: 30 minutes
    pub fn next_retry_delay(&self) -> Duration {
        let base_secs = 30i64;
        let max_secs = 30 * 60i64;
        let delay_secs = (base_secs * 2i64.pow(self.attempts as u32)).min(max_secs);
        Duration::seconds(delay_secs)
    }

    /// Calculate when job should be retried
    pub fn next_retry_at(&self) -> DateTime<Utc> {
        Utc::now() + self.next_retry_delay()
    }

    /// Check if job has retries remaining
    pub fn has_retries(&self) -> bool {
        self.attempts < self.max_attempts
    }

    /// Check if job should be moved to dead-letter
    pub fn should_dead_letter(&self) -> bool {
        !self.has_retries() && self.status.can_retry()
    }

    /// Mark job as started
    pub fn mark_started(&mut self, worker_id: String, lease_duration: StdDuration) {
        self.status = JobStatus::Running;
        self.worker_id = Some(worker_id);
        self.lease_expires_at = Some(Utc::now() + Duration::from_std(lease_duration).unwrap());
        self.started_at = Some(Utc::now());
        self.attempts += 1;
        self.updated_at = Utc::now();
    }

    /// Mark job as completed
    pub fn mark_completed(&mut self, result: Option<serde_json::Value>) {
        self.status = JobStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.result = result;
        self.worker_id = None;
        self.lease_expires_at = None;
        self.updated_at = Utc::now();
    }

    /// Mark job as failed
    pub fn mark_failed(&mut self, error: impl Into<String>) {
        self.error_message = Some(error.into());
        self.worker_id = None;
        self.lease_expires_at = None;
        self.updated_at = Utc::now();

        if self.has_retries() {
            self.status = JobStatus::Failed;
            self.retry_at = Some(self.next_retry_at());
        } else {
            self.status = JobStatus::DeadLetter;
            self.completed_at = Some(Utc::now());
        }
    }

    /// Update job context (for progress tracking)
    pub fn update_context(&mut self, context: serde_json::Value) {
        self.context = Some(context);
        self.updated_at = Utc::now();
    }

    /// Append to job logs
    pub fn append_log(&mut self, message: impl Into<String>) {
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let line = format!("[{}] {}\n", timestamp, message.into());
        match &mut self.logs {
            Some(logs) => logs.push_str(&line),
            None => self.logs = Some(line),
        }
        self.updated_at = Utc::now();
    }
}

/// Configuration for the durable job queue
#[derive(Debug, Clone)]
pub struct DurableQueueConfig {
    /// How long a worker lease lasts
    pub lease_duration: StdDuration,
    /// How often to check for expired leases
    pub lease_check_interval: StdDuration,
    /// How often to check for jobs to retry
    pub retry_check_interval: StdDuration,
    /// Maximum jobs to dequeue in one batch
    pub batch_size: i32,
    /// Default max retry attempts
    pub default_max_attempts: i32,
}

impl Default for DurableQueueConfig {
    fn default() -> Self {
        Self {
            lease_duration: StdDuration::from_secs(300), // 5 minutes
            lease_check_interval: StdDuration::from_secs(60), // 1 minute
            retry_check_interval: StdDuration::from_secs(30), // 30 seconds
            batch_size: 10,
            default_max_attempts: 3,
        }
    }
}

/// Worker lease for distributed job execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerLease {
    /// Job ID this lease is for
    pub job_id: String,
    /// Worker ID holding the lease
    pub worker_id: String,
    /// When the lease expires
    pub expires_at: DateTime<Utc>,
    /// When the lease was acquired
    pub acquired_at: DateTime<Utc>,
}

impl WorkerLease {
    /// Create a new lease
    pub fn new(job_id: impl Into<String>, worker_id: impl Into<String>, duration: StdDuration) -> Self {
        let now = Utc::now();
        Self {
            job_id: job_id.into(),
            worker_id: worker_id.into(),
            expires_at: now + Duration::from_std(duration).unwrap(),
            acquired_at: now,
        }
    }

    /// Check if lease is still valid
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }

    /// Extend the lease
    pub fn extend(&mut self, duration: StdDuration) {
        self.expires_at = Utc::now() + Duration::from_std(duration).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_creation() {
        let job = DurableJob::new("test_job", serde_json::json!({"key": "value"}))
            .with_max_attempts(5)
            .with_priority(JobPriority::High)
            .with_idempotency_key("test-key-123");

        assert_eq!(job.job_type, "test_job");
        assert_eq!(job.max_attempts, 5);
        assert!(matches!(job.priority, JobPriority::High));
        assert_eq!(job.idempotency_key, Some("test-key-123".to_string()));
    }

    #[test]
    fn test_retry_delay_exponential_backoff() {
        let mut job = DurableJob::new("test", serde_json::Value::Null);

        // First attempt: 30 * 2^1 = 60 seconds
        job.attempts = 1;
        assert_eq!(job.next_retry_delay().num_seconds(), 60);

        // Second attempt: 30 * 2^2 = 120 seconds
        job.attempts = 2;
        assert_eq!(job.next_retry_delay().num_seconds(), 120);

        // Third attempt: 30 * 2^3 = 240 seconds
        job.attempts = 3;
        assert_eq!(job.next_retry_delay().num_seconds(), 240);

        // Max delay is 30 minutes
        job.attempts = 20;
        assert_eq!(job.next_retry_delay().num_seconds(), 30 * 60);
    }

    #[test]
    fn test_has_retries() {
        let mut job = DurableJob::new("test", serde_json::Value::Null).with_max_attempts(3);

        job.attempts = 0;
        assert!(job.has_retries());

        job.attempts = 2;
        assert!(job.has_retries());

        job.attempts = 3;
        assert!(!job.has_retries());
    }

    #[test]
    fn test_job_status_conversion() {
        assert_eq!(JobStatus::Pending.as_str(), "pending");
        assert_eq!(JobStatus::from_str("running"), Some(JobStatus::Running));
        assert_eq!(JobStatus::from_str("dead_letter"), Some(JobStatus::DeadLetter));
        assert_eq!(JobStatus::from_str("invalid"), None);
    }

    #[test]
    fn test_job_lifecycle() {
        let mut job = DurableJob::new("test", serde_json::Value::Null);
        assert!(matches!(job.status, JobStatus::Pending));

        job.mark_started("worker-1".to_string(), StdDuration::from_secs(300));
        assert!(matches!(job.status, JobStatus::Running));
        assert_eq!(job.attempts, 1);

        job.mark_completed(Some(serde_json::json!({"success": true})));
        assert!(matches!(job.status, JobStatus::Completed));
        assert!(job.result.is_some());
    }

    #[test]
    fn test_job_failure_and_retry() {
        let mut job = DurableJob::new("test", serde_json::Value::Null).with_max_attempts(3);

        job.attempts = 1;
        job.mark_failed("First failure");
        assert!(matches!(job.status, JobStatus::Failed));
        assert!(job.retry_at.is_some());

        job.attempts = 3;
        job.mark_failed("Final failure");
        assert!(matches!(job.status, JobStatus::DeadLetter));
    }

    #[test]
    fn test_worker_lease() {
        let lease = WorkerLease::new("job-1", "worker-1", StdDuration::from_secs(300));
        assert!(lease.is_valid());
        assert_eq!(lease.job_id, "job-1");
    }
}
