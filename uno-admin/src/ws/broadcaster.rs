//! Job event broadcaster using tokio::sync::broadcast

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tokio::sync::broadcast;

use crate::models::entity::SyncJobEntity;

/// Global broadcaster instance
static BROADCASTER: OnceLock<broadcast::Sender<JobEvent>> = OnceLock::new();

/// Job status change event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum JobEvent {
    /// Job created or status changed
    JobUpdated(JobUpdatePayload),
    /// Job deleted
    JobDeleted { job_id: String },
}

/// Payload for job update events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobUpdatePayload {
    pub id: String,
    pub job_type: String,
    pub status: String,
    pub target_date: String,
    pub records_fetched: i32,
    pub records_inserted: i32,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Initialize the global broadcaster (call once at startup)
pub fn init_broadcaster() -> broadcast::Receiver<JobEvent> {
    let (tx, rx) = broadcast::channel(100);
    BROADCASTER.set(tx).expect("Broadcaster already initialized");
    tracing::info!("WebSocket broadcaster initialized");
    rx
}

/// Get a new receiver subscription
pub fn subscribe() -> Option<broadcast::Receiver<JobEvent>> {
    BROADCASTER.get().map(|tx| tx.subscribe())
}

/// Broadcast a job event to all connected clients
pub fn broadcast(event: JobEvent) {
    if let Some(tx) = BROADCASTER.get() {
        // Ignore errors (no receivers connected)
        let _ = tx.send(event);
    }
}

/// Helper to create JobUpdatePayload from SyncJobEntity
impl From<&SyncJobEntity> for JobUpdatePayload {
    fn from(job: &SyncJobEntity) -> Self {
        Self {
            id: job.id.clone(),
            job_type: job.job_type.clone(),
            status: job.status.clone(),
            target_date: job.target_date.clone(),
            records_fetched: job.records_fetched,
            records_inserted: job.records_inserted,
            duration_ms: job.duration_ms,
            error_message: job.error_message.clone(),
            created_at: job.created_at.clone(),
            updated_at: job.updated_at.clone(),
        }
    }
}

/// Convenience function to broadcast a job update
pub fn broadcast_job_update(job: &SyncJobEntity) {
    broadcast(JobEvent::JobUpdated(JobUpdatePayload::from(job)));
}

/// Convenience function to broadcast a job deletion
pub fn broadcast_job_deleted(job_id: &str) {
    broadcast(JobEvent::JobDeleted {
        job_id: job_id.to_string(),
    });
}
