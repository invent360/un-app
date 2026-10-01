//! Background worker runner for processing jobs
//!
//! Provides a polling-based worker that:
//! - Registers with the system and maintains heartbeat
//! - Claims and processes jobs with lease management
//! - Handles retries and dead letter queue
//! - Supports graceful shutdown

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::{interval, timeout};
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::server::repositories::{DynOutboxRepository, DynImmutableAuditRepository};
use crate::server::services::{JobService, Job, JobResult};
use crate::types::AppError;

/// Configuration for the background worker
#[derive(Debug, Clone)]
pub struct WorkerConfig {
    /// Worker name for identification
    pub worker_name: String,
    /// Worker type (e.g., "job_processor", "outbox_publisher")
    pub worker_type: String,
    /// Job types this worker handles (empty = all)
    pub job_types: Vec<String>,
    /// Poll interval for checking jobs
    pub poll_interval: Duration,
    /// Heartbeat interval (should be less than lease duration)
    pub heartbeat_interval: Duration,
    /// Lease duration for held jobs
    pub lease_duration_secs: i64,
    /// Maximum jobs to process before recycling
    pub max_jobs_before_recycle: Option<u64>,
    /// Timeout for processing a single job
    pub job_timeout: Duration,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            worker_name: format!("worker-{}", Uuid::new_v4()),
            worker_type: "job_processor".to_string(),
            job_types: vec![],
            poll_interval: Duration::from_secs(5),
            heartbeat_interval: Duration::from_secs(60),
            lease_duration_secs: 300, // 5 minutes
            max_jobs_before_recycle: None,
            job_timeout: Duration::from_secs(240), // 4 minutes
        }
    }
}

/// Job handler function type
pub type JobHandler = Box<dyn Fn(&Job) -> std::pin::Pin<Box<dyn std::future::Future<Output = JobResult> + Send>> + Send + Sync>;

/// Background worker for processing jobs
pub struct WorkerRunner {
    worker_id: Uuid,
    config: WorkerConfig,
    job_service: JobService,
    outbox_repo: DynOutboxRepository,
    shutdown: Arc<AtomicBool>,
    jobs_processed: u64,
    jobs_failed: u64,
}

impl WorkerRunner {
    /// Create a new worker runner
    pub fn new(
        config: WorkerConfig,
        pool: ConnectionPool,
        outbox_repo: DynOutboxRepository,
        audit_repo: DynImmutableAuditRepository,
    ) -> Self {
        let job_service = JobService::new(pool, outbox_repo.clone(), audit_repo);

        Self {
            worker_id: Uuid::new_v4(),
            config,
            job_service,
            outbox_repo,
            shutdown: Arc::new(AtomicBool::new(false)),
            jobs_processed: 0,
            jobs_failed: 0,
        }
    }

    /// Get a shutdown handle for graceful termination
    pub fn shutdown_handle(&self) -> Arc<AtomicBool> {
        self.shutdown.clone()
    }

    /// Run the worker (blocks until shutdown)
    pub async fn run<F>(&mut self, handler: F) -> Result<WorkerStats, AppError>
    where
        F: Fn(&Job) -> std::pin::Pin<Box<dyn std::future::Future<Output = JobResult> + Send>> + Send + Sync,
    {
        // Register worker
        self.register().await?;

        tracing::info!(
            worker_id = %self.worker_id,
            worker_name = %self.config.worker_name,
            worker_type = %self.config.worker_type,
            "Worker started"
        );

        // Create channels for coordination
        let (heartbeat_tx, mut heartbeat_rx) = mpsc::channel::<()>(1);

        // Start heartbeat task
        let heartbeat_handle = self.start_heartbeat_task(heartbeat_tx);

        // Main processing loop
        let mut poll_timer = interval(self.config.poll_interval);

        loop {
            tokio::select! {
                _ = poll_timer.tick() => {
                    if self.shutdown.load(Ordering::Relaxed) {
                        tracing::info!(worker_id = %self.worker_id, "Shutdown signal received");
                        break;
                    }

                    // Check for max jobs limit
                    if let Some(max_jobs) = self.config.max_jobs_before_recycle {
                        if self.jobs_processed >= max_jobs {
                            tracing::info!(
                                worker_id = %self.worker_id,
                                jobs_processed = self.jobs_processed,
                                "Max jobs reached, recycling"
                            );
                            break;
                        }
                    }

                    // Try to claim and process a job
                    match self.process_one_job(&handler).await {
                        Ok(processed) => {
                            if processed {
                                // Process more jobs immediately if available
                                while !self.shutdown.load(Ordering::Relaxed) {
                                    match self.process_one_job(&handler).await {
                                        Ok(true) => continue,
                                        Ok(false) => break,
                                        Err(e) => {
                                            tracing::error!(error = %e, "Error processing job");
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!(error = %e, "Error in job processing");
                        }
                    }
                }
                _ = heartbeat_rx.recv() => {
                    // Heartbeat task died, this is fatal
                    tracing::error!(worker_id = %self.worker_id, "Heartbeat task died");
                    break;
                }
            }
        }

        // Cleanup
        drop(heartbeat_handle);
        self.unregister().await?;

        Ok(WorkerStats {
            worker_id: self.worker_id,
            jobs_processed: self.jobs_processed,
            jobs_failed: self.jobs_failed,
        })
    }

    async fn process_one_job<F>(&mut self, handler: &F) -> Result<bool, AppError>
    where
        F: Fn(&Job) -> std::pin::Pin<Box<dyn std::future::Future<Output = JobResult> + Send>> + Send + Sync,
    {
        let job_types: Vec<&str> = self.config.job_types.iter().map(|s| s.as_str()).collect();

        // Try to claim a job
        let job = match self.job_service.claim_job(self.worker_id, &job_types).await? {
            Some(job) => job,
            None => return Ok(false),
        };

        tracing::info!(
            worker_id = %self.worker_id,
            job_id = %job.id,
            job_type = %job.job_type,
            attempt = job.attempts,
            "Processing job"
        );

        // Process with timeout
        let result = match timeout(self.config.job_timeout, handler(&job)).await {
            Ok(result) => result,
            Err(_) => {
                tracing::warn!(
                    job_id = %job.id,
                    timeout_secs = self.config.job_timeout.as_secs(),
                    "Job timed out"
                );
                JobResult::failure("Job execution timed out")
            }
        };

        // R3-10: Complete or fail the job with fencing verification
        if result.success {
            let completed = self.job_service.complete_job_fenced(&job, result).await?;
            if completed {
                self.jobs_processed += 1;
                tracing::info!(job_id = %job.id, "Job completed successfully");
            } else {
                // Lease was lost - another worker may have taken over
                tracing::warn!(
                    job_id = %job.id,
                    worker_id = %self.worker_id,
                    "Job completion rejected - lease expired or job reassigned"
                );
            }
        } else {
            let error = result.error.as_deref().unwrap_or("Unknown error");
            let will_retry = self.job_service.fail_job_fenced(&job, error).await?;
            self.jobs_failed += 1;

            if will_retry {
                tracing::warn!(job_id = %job.id, error = %error, "Job failed, will retry");
            } else {
                tracing::error!(job_id = %job.id, error = %error, "Job failed, moved to dead letter");
            }
        }

        // Update worker stats
        let _ = self.outbox_repo.update_worker_stats(
            self.worker_id,
            self.jobs_processed as i64,
            self.jobs_failed as i64,
        ).await;

        Ok(true)
    }

    async fn register(&self) -> Result<(), AppError> {
        self.outbox_repo.register_worker(
            self.worker_id,
            &self.config.worker_name,
            &self.config.worker_type,
            self.config.lease_duration_secs,
        ).await?;

        Ok(())
    }

    async fn unregister(&self) -> Result<(), AppError> {
        self.outbox_repo.unregister_worker(self.worker_id).await?;
        tracing::info!(worker_id = %self.worker_id, "Worker unregistered");
        Ok(())
    }

    fn start_heartbeat_task(&self, shutdown_signal: mpsc::Sender<()>) -> tokio::task::JoinHandle<()> {
        let worker_id = self.worker_id;
        let outbox_repo = self.outbox_repo.clone();
        let lease_duration = self.config.lease_duration_secs;
        let heartbeat_interval = self.config.heartbeat_interval;
        let shutdown = self.shutdown.clone();

        tokio::spawn(async move {
            let mut timer = interval(heartbeat_interval);

            loop {
                timer.tick().await;

                if shutdown.load(Ordering::Relaxed) {
                    break;
                }

                match outbox_repo.heartbeat_worker(worker_id, lease_duration).await {
                    Ok(true) => {
                        tracing::trace!(worker_id = %worker_id, "Heartbeat sent");
                    }
                    Ok(false) => {
                        tracing::warn!(worker_id = %worker_id, "Worker lease not found");
                        let _ = shutdown_signal.send(()).await;
                        break;
                    }
                    Err(e) => {
                        tracing::error!(error = %e, "Failed to send heartbeat");
                        let _ = shutdown_signal.send(()).await;
                        break;
                    }
                }
            }
        })
    }
}

/// Statistics from a worker run
#[derive(Debug, Clone)]
pub struct WorkerStats {
    pub worker_id: Uuid,
    pub jobs_processed: u64,
    pub jobs_failed: u64,
}

/// Background task for reclaiming expired job leases
pub async fn run_lease_reclaimer(
    job_service: JobService,
    check_interval: Duration,
    shutdown: Arc<AtomicBool>,
) {
    let mut timer = interval(check_interval);

    loop {
        timer.tick().await;

        if shutdown.load(Ordering::Relaxed) {
            break;
        }

        match job_service.reclaim_expired_leases().await {
            Ok(count) if count > 0 => {
                tracing::info!(count = count, "Reclaimed expired job leases");
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error = %e, "Failed to reclaim expired leases");
            }
        }
    }
}

/// Background task for cleanup of old events
pub async fn run_event_cleanup(
    outbox_repo: DynOutboxRepository,
    cleanup_interval: Duration,
    retention_days: i32,
    shutdown: Arc<AtomicBool>,
) {
    let mut timer = interval(cleanup_interval);

    loop {
        timer.tick().await;

        if shutdown.load(Ordering::Relaxed) {
            break;
        }

        // Cleanup old outbox events
        match outbox_repo.cleanup_old_outbox_events(retention_days).await {
            Ok(count) if count > 0 => {
                tracing::info!(count = count, "Cleaned up old outbox events");
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error = %e, "Failed to cleanup outbox events");
            }
        }

        // Cleanup old inbox events
        match outbox_repo.cleanup_old_inbox_events(retention_days).await {
            Ok(count) if count > 0 => {
                tracing::info!(count = count, "Cleaned up old inbox events");
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error = %e, "Failed to cleanup inbox events");
            }
        }
    }
}

/// R3-10: Background task for recovering stale publishing events
pub async fn run_stale_event_recovery(
    job_service: JobService,
    check_interval: Duration,
    stale_threshold_secs: i32,
    shutdown: Arc<AtomicBool>,
) {
    let mut timer = interval(check_interval);

    loop {
        timer.tick().await;

        if shutdown.load(Ordering::Relaxed) {
            break;
        }

        match job_service.recover_stale_publishing_events(stale_threshold_secs).await {
            Ok(count) if count > 0 => {
                tracing::info!(count = count, "Recovered stale publishing events");
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error = %e, "Failed to recover stale events");
            }
        }
    }
}

/// R5-09: Background task for recovering stale outbox events
/// Resets events stuck in 'publishing' state back to 'pending'
pub async fn run_outbox_stale_recovery(
    outbox_repo: DynOutboxRepository,
    check_interval: Duration,
    shutdown: Arc<AtomicBool>,
) {
    let mut timer = interval(check_interval);

    tracing::info!(
        check_interval_secs = check_interval.as_secs(),
        "Outbox stale event recovery task started"
    );

    loop {
        timer.tick().await;

        if shutdown.load(Ordering::Relaxed) {
            break;
        }

        match outbox_repo.recover_stale_publishing_events().await {
            Ok(count) if count > 0 => {
                tracing::info!(count = count, "Recovered stale outbox events");
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error = %e, "Failed to recover stale outbox events");
            }
        }
    }
}

/// R5-09: Background task for publishing outbox events
/// Polls and publishes events using the configured publisher (webhook, etc.)
pub async fn run_outbox_publisher(
    outbox_repo: DynOutboxRepository,
    config: super::OutboxPublisherConfig,
    poll_interval: Duration,
    shutdown: Arc<AtomicBool>,
) {
    use super::OutboxPublisher;

    let publisher = OutboxPublisher::from_config(outbox_repo.clone(), config);
    let mut timer = interval(poll_interval);

    tracing::info!(
        poll_interval_secs = poll_interval.as_secs(),
        "Outbox publisher started"
    );

    loop {
        timer.tick().await;

        if shutdown.load(Ordering::Relaxed) {
            tracing::info!("Outbox publisher shutting down");
            break;
        }

        match publisher.publish_batch().await {
            Ok(stats) => {
                if stats.claimed > 0 {
                    tracing::debug!(
                        claimed = stats.claimed,
                        published = stats.published,
                        failed = stats.failed,
                        dead_lettered = stats.dead_lettered,
                        "Outbox publish batch completed"
                    );
                }
            }
            Err(e) => {
                tracing::error!(error = %e, "Outbox publish batch failed");
            }
        }
    }
}

/// R3-17: Background task for data retention cleanup
/// Enforces retention policies from the retention_policies table
pub async fn run_retention_cleanup(
    retention_service: super::DynRetentionService,
    cleanup_interval: Duration,
    shutdown: Arc<AtomicBool>,
) {
    let mut timer = interval(cleanup_interval);

    loop {
        timer.tick().await;

        if shutdown.load(Ordering::Relaxed) {
            break;
        }

        // Check if any policies are overdue (haven't been cleaned in 24+ hours)
        match retention_service.has_overdue_policies(24).await {
            Ok(true) => {
                tracing::info!("Starting retention cleanup (overdue policies detected)");
            }
            Ok(false) => {
                // Skip this cycle if no policies are overdue
                continue;
            }
            Err(e) => {
                tracing::error!(error = %e, "Failed to check overdue policies");
                continue;
            }
        }

        match retention_service.run_full_cleanup().await {
            Ok(summary) => {
                tracing::info!(
                    policies_processed = summary.policies_processed,
                    rows_deleted = summary.total_rows_deleted,
                    rows_archived = summary.total_rows_archived,
                    duration_ms = summary.total_duration_ms,
                    errors = summary.errors.len(),
                    "Retention cleanup completed"
                );

                if !summary.errors.is_empty() {
                    for error in &summary.errors {
                        tracing::warn!(error = %error, "Retention cleanup partial failure");
                    }
                }
            }
            Err(e) => {
                tracing::error!(error = %e, "Retention cleanup failed");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_config_defaults() {
        let config = WorkerConfig::default();
        assert_eq!(config.worker_type, "job_processor");
        assert_eq!(config.poll_interval, Duration::from_secs(5));
        assert_eq!(config.heartbeat_interval, Duration::from_secs(60));
        assert_eq!(config.lease_duration_secs, 300);
    }

    #[test]
    fn test_worker_stats() {
        let stats = WorkerStats {
            worker_id: Uuid::new_v4(),
            jobs_processed: 100,
            jobs_failed: 5,
        };
        assert_eq!(stats.jobs_processed, 100);
        assert_eq!(stats.jobs_failed, 5);
    }
}
