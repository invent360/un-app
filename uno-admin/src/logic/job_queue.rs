//! Background job queue for async task execution
//!
//! This module provides a simple mpsc channel-based job queue that allows
//! HTTP handlers to enqueue jobs and return immediately, while a background
//! worker processes them asynchronously.

use std::sync::OnceLock;
use tokio::sync::mpsc;

/// Job commands that can be queued for background execution
#[derive(Debug, Clone)]
pub enum JobCommand {
    /// Run rewards sync for a specific date
    /// Contains: (job_id, target_date)
    RewardsSync { job_id: String, target_date: String },

    /// Run license sync (syncs all licenses from API)
    /// Contains: (job_id)
    LicenseSync { job_id: String },

    /// Re-run an existing job
    /// Contains: (job_id, job_type, target_date)
    RerunJob {
        job_id: String,
        job_type: String,
        target_date: String,
    },
}

/// Global job queue sender
static JOB_QUEUE: OnceLock<mpsc::Sender<JobCommand>> = OnceLock::new();

/// Initialize the job queue and return the receiver
///
/// This must be called exactly once at startup. The receiver should be
/// passed to the job worker for processing.
///
/// # Panics
/// Panics if called more than once.
pub fn init_job_queue() -> mpsc::Receiver<JobCommand> {
    let (tx, rx) = mpsc::channel(100);
    JOB_QUEUE
        .set(tx)
        .expect("Job queue already initialized - init_job_queue called twice");
    tracing::info!("[JobQueue] Initialized with capacity 100");
    rx
}

/// Enqueue a job command for background processing
///
/// Returns Ok(()) if the job was successfully queued, or an error if:
/// - The queue has not been initialized
/// - The queue is full (should not happen with capacity 100)
///
/// # Example
/// ```ignore
/// use crate::logic::job_queue::{enqueue, JobCommand};
///
/// enqueue(JobCommand::RewardsSync {
///     job_id: "abc123".to_string(),
///     target_date: "2026-06-10".to_string(),
/// })?;
/// ```
pub fn enqueue(cmd: JobCommand) -> Result<(), String> {
    let sender = JOB_QUEUE
        .get()
        .ok_or("Job queue not initialized - call init_job_queue first")?;

    sender
        .try_send(cmd)
        .map_err(|e| format!("Failed to enqueue job: {}", e))
}

/// Check if the job queue has been initialized
pub fn is_initialized() -> bool {
    JOB_QUEUE.get().is_some()
}
