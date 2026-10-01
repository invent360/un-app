//! Outbox repository for transactional event publishing
//!
//! Implements the outbox pattern for reliable event delivery:
//! - Events are persisted atomically with business transactions
//! - Background workers poll and publish events
//! - Supports retries with exponential backoff
//! - Dead-letter queue for failed events

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Outbox event status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboxStatus {
    Pending,
    Publishing,
    Published,
    Failed,
    DeadLetter,
}

impl OutboxStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Publishing => "publishing",
            Self::Published => "published",
            Self::Failed => "failed",
            Self::DeadLetter => "dead_letter",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "publishing" => Some(Self::Publishing),
            "published" => Some(Self::Published),
            "failed" => Some(Self::Failed),
            "dead_letter" => Some(Self::DeadLetter),
            _ => None,
        }
    }
}

/// Outbox event entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OutboxEvent {
    pub id: i64,
    pub event_id: Uuid,
    pub event_type: String,
    pub event_version: i32,
    pub aggregate_type: String,
    pub aggregate_id: String,
    #[sqlx(json)]
    pub payload: JsonValue,
    #[sqlx(json)]
    pub metadata: Option<JsonValue>,
    pub idempotency_key: Option<String>,
    pub status: String,
    pub published_at: Option<DateTime<Utc>>,
    pub publish_attempts: i32,
    pub last_error: Option<String>,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl OutboxEvent {
    pub fn status_enum(&self) -> Option<OutboxStatus> {
        OutboxStatus::from_str(&self.status)
    }
}

/// Input for creating an outbox event
#[derive(Debug, Clone)]
pub struct CreateOutboxEvent {
    pub event_type: String,
    pub event_version: i32,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub payload: JsonValue,
    pub metadata: Option<JsonValue>,
    pub idempotency_key: Option<String>,
}

/// Inbox event status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboxStatus {
    Received,
    Processing,
    Processed,
    Failed,
    Skipped,
}

impl InboxStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Received => "received",
            Self::Processing => "processing",
            Self::Processed => "processed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
        }
    }
}

/// Inbox event entry for deduplication
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InboxEvent {
    pub id: i64,
    pub event_id: String,
    pub source: String,
    pub status: String,
    pub processed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub event_type: String,
    #[sqlx(json)]
    pub payload: JsonValue,
    pub received_at: DateTime<Utc>,
}

/// Worker lease entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkerLease {
    pub worker_id: Uuid,
    pub worker_name: String,
    pub worker_type: String,
    pub heartbeat_at: DateTime<Utc>,
    pub lease_expires_at: DateTime<Utc>,
    pub host: Option<String>,
    pub process_id: Option<i32>,
    pub started_at: DateTime<Utc>,
    pub jobs_processed: i64,
    pub jobs_failed: i64,
    pub last_job_at: Option<DateTime<Utc>>,
}

// ============================================
// TRAIT DEFINITION
// ============================================

pub type DynOutboxRepository = Arc<dyn OutboxRepository + Send + Sync>;

#[async_trait]
pub trait OutboxRepository: Send + Sync {
    // --- Outbox Operations ---

    /// Create an outbox event (call within same transaction as business mutation)
    async fn create_event(&self, event: CreateOutboxEvent) -> Result<OutboxEvent, AppError>;

    /// Claim events for publishing (sets status to 'publishing')
    async fn claim_events(&self, limit: i32, worker_id: Uuid) -> Result<Vec<OutboxEvent>, AppError>;

    /// Mark event as published
    #[deprecated(since = "0.2.0", note = "Use mark_published_fenced instead for proper lease validation")]
    async fn mark_published(&self, event_id: Uuid) -> Result<(), AppError>;

    /// R5-09: Mark event as published with fenced lease validation
    /// Returns false if lease expired or worker doesn't match
    async fn mark_published_fenced(
        &self,
        event_id: Uuid,
        worker_id: Uuid,
    ) -> Result<bool, AppError>;

    /// Mark event as failed with retry
    #[deprecated(since = "0.2.0", note = "Use mark_failed_fenced instead for proper lease validation")]
    async fn mark_failed(&self, event_id: Uuid, error: &str, retry_at: Option<DateTime<Utc>>) -> Result<(), AppError>;

    /// R5-09: Mark event as failed with fenced lease validation
    /// Returns false if lease expired or worker doesn't match
    async fn mark_failed_fenced(
        &self,
        event_id: Uuid,
        error: &str,
        retry_at: Option<DateTime<Utc>>,
        worker_id: Uuid,
    ) -> Result<bool, AppError>;

    /// Mark event as dead letter (no more retries)
    #[deprecated(since = "0.2.0", note = "Use mark_dead_letter_fenced instead for proper lease validation")]
    async fn mark_dead_letter(&self, event_id: Uuid, error: &str) -> Result<(), AppError>;

    /// R5-09: Mark event as dead letter with fenced lease validation
    /// Returns false if lease expired or worker doesn't match
    async fn mark_dead_letter_fenced(
        &self,
        event_id: Uuid,
        error: &str,
        worker_id: Uuid,
    ) -> Result<bool, AppError>;

    /// Get events ready for retry
    async fn get_retry_events(&self, limit: i32) -> Result<Vec<OutboxEvent>, AppError>;

    /// Get dead letter events for inspection
    async fn get_dead_letter_events(&self, limit: i32, offset: i32) -> Result<Vec<OutboxEvent>, AppError>;

    /// Replay a dead letter event (reset to pending)
    async fn replay_event(&self, event_id: Uuid) -> Result<OutboxEvent, AppError>;

    // --- Inbox Operations ---

    /// Check if an event has already been processed
    async fn is_event_processed(&self, source: &str, event_id: &str) -> Result<bool, AppError>;

    /// Record an inbound event (returns false if already exists)
    async fn record_inbound_event(
        &self,
        source: &str,
        event_id: &str,
        event_type: &str,
        payload: JsonValue,
    ) -> Result<bool, AppError>;

    /// Mark inbound event as processed
    async fn mark_inbound_processed(&self, source: &str, event_id: &str) -> Result<(), AppError>;

    /// Mark inbound event as failed
    async fn mark_inbound_failed(&self, source: &str, event_id: &str, error: &str) -> Result<(), AppError>;

    // --- Worker Operations ---

    /// Register a worker and acquire lease
    async fn register_worker(
        &self,
        worker_id: Uuid,
        worker_name: &str,
        worker_type: &str,
        lease_duration_secs: i64,
    ) -> Result<WorkerLease, AppError>;

    /// Heartbeat to maintain worker lease
    async fn heartbeat_worker(&self, worker_id: Uuid, lease_duration_secs: i64) -> Result<bool, AppError>;

    /// Update worker statistics
    async fn update_worker_stats(&self, worker_id: Uuid, jobs_processed: i64, jobs_failed: i64) -> Result<(), AppError>;

    /// Remove worker registration
    async fn unregister_worker(&self, worker_id: Uuid) -> Result<(), AppError>;

    /// Get expired worker leases (for reclaiming jobs)
    async fn get_expired_workers(&self) -> Result<Vec<WorkerLease>, AppError>;

    /// Cleanup old processed inbox events
    async fn cleanup_old_inbox_events(&self, older_than_days: i32) -> Result<i64, AppError>;

    /// Cleanup old published outbox events
    async fn cleanup_old_outbox_events(&self, older_than_days: i32) -> Result<i64, AppError>;

    /// R5-09: Recover events stuck in 'publishing' state with expired claims
    async fn recover_stale_publishing_events(&self) -> Result<i64, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct OutboxRepositoryImpl {
    pool: ConnectionPool,
}

impl OutboxRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }

    fn calculate_retry_delay(attempts: i32) -> Duration {
        // Exponential backoff: 10s, 30s, 90s, 270s, 810s (max ~13 min)
        let base_delay_secs = 10i64;
        let multiplier = 3i64.pow(attempts.min(5) as u32);
        Duration::seconds(base_delay_secs * multiplier)
    }
}

#[async_trait]
impl OutboxRepository for OutboxRepositoryImpl {
    async fn create_event(&self, event: CreateOutboxEvent) -> Result<OutboxEvent, AppError> {
        let result = sqlx::query_as::<_, OutboxEvent>(
            r#"
            INSERT INTO outbox (
                event_type, event_version, aggregate_type, aggregate_id,
                payload, metadata, idempotency_key, status, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending', NOW())
            RETURNING id, event_id, event_type, event_version, aggregate_type, aggregate_id,
                      payload, metadata, idempotency_key, status, published_at,
                      publish_attempts, last_error, next_retry_at, created_at
            "#,
        )
        .bind(&event.event_type)
        .bind(event.event_version)
        .bind(&event.aggregate_type)
        .bind(&event.aggregate_id)
        .bind(&event.payload)
        .bind(&event.metadata)
        .bind(event.idempotency_key.as_deref())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("Event with this idempotency key already exists".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(result)
    }

    async fn claim_events(&self, limit: i32, worker_id: Uuid) -> Result<Vec<OutboxEvent>, AppError> {
        // R5-09: Claim with proper lease tracking for fenced acknowledgement
        // Sets claimed_by, claimed_at, claim_expires_at, and increments claim_generation
        let events = sqlx::query_as::<_, OutboxEvent>(
            r#"
            UPDATE outbox
            SET status = 'publishing',
                publish_attempts = publish_attempts + 1,
                claimed_by = $2,
                claimed_at = NOW(),
                claim_expires_at = NOW() + INTERVAL '5 minutes',
                claim_generation = COALESCE(claim_generation, 0) + 1
            WHERE id IN (
                SELECT id FROM outbox
                WHERE (status = 'pending')
                   OR (status = 'failed' AND next_retry_at IS NOT NULL AND next_retry_at <= NOW())
                ORDER BY created_at ASC
                LIMIT $1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING id, event_id, event_type, event_version, aggregate_type, aggregate_id,
                      payload, metadata, idempotency_key, status, published_at,
                      publish_attempts, last_error, next_retry_at, created_at
            "#,
        )
        .bind(limit)
        .bind(worker_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(events)
    }

    async fn mark_published(&self, event_id: Uuid) -> Result<(), AppError> {
        // R5-09: Clear claim tracking and mark as published
        // Note: Fenced version would check claimed_by matches, but since we claimed
        // within same process flow, we trust the event_id. The claim_expires_at
        // provides protection against stale completion from crashed workers.
        let result = sqlx::query(
            r#"
            UPDATE outbox
            SET status = 'published',
                published_at = NOW(),
                last_error = NULL,
                next_retry_at = NULL,
                claimed_by = NULL,
                claimed_at = NULL,
                claim_expires_at = NULL
            WHERE event_id = $1
              AND status = 'publishing'
              AND claim_expires_at > NOW()
            "#,
        )
        .bind(event_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            // Check if already published or claim expired
            let current: Option<(String,)> = sqlx::query_as(
                "SELECT status FROM outbox WHERE event_id = $1"
            )
            .bind(event_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            match current {
                Some((status,)) if status == "published" => {
                    // Already published, idempotent success
                    tracing::debug!(event_id = %event_id, "Event already published");
                }
                Some((status,)) => {
                    // Lease expired or status changed
                    tracing::warn!(
                        event_id = %event_id,
                        status = %status,
                        "Failed to mark published - lease may have expired"
                    );
                    return Err(AppError::Conflict(
                        format!("Event {} claim expired or status changed to {}", event_id, status)
                    ));
                }
                None => {
                    return Err(AppError::NotFound(format!("Event {} not found", event_id)));
                }
            }
        }

        Ok(())
    }

    async fn mark_published_fenced(
        &self,
        event_id: Uuid,
        worker_id: Uuid,
    ) -> Result<bool, AppError> {
        // R5-09: Only allow publish if lease is still valid and worker matches
        let result = sqlx::query(
            r#"
            UPDATE outbox
            SET status = 'published',
                published_at = NOW(),
                last_error = NULL,
                next_retry_at = NULL,
                claimed_by = NULL,
                claimed_at = NULL,
                claim_expires_at = NULL
            WHERE event_id = $1
              AND status = 'publishing'
              AND claimed_by = $2
              AND claim_expires_at > NOW()
            "#,
        )
        .bind(event_id)
        .bind(worker_id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            // Check if already published (idempotent success) or stale
            let current: Option<(String,)> = sqlx::query_as(
                "SELECT status FROM outbox WHERE event_id = $1"
            )
            .bind(event_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            match current {
                Some((status,)) if status == "published" => {
                    // Already published, idempotent success
                    tracing::debug!(
                        event_id = %event_id,
                        worker_id = %worker_id,
                        "Event already published (idempotent success)"
                    );
                    return Ok(true);
                }
                Some((status,)) => {
                    // Lease expired or worker mismatch
                    tracing::warn!(
                        event_id = %event_id,
                        worker_id = %worker_id,
                        status = %status,
                        "mark_published_fenced: lease expired or worker mismatch - stale worker detected"
                    );
                    return Ok(false);
                }
                None => {
                    tracing::error!(event_id = %event_id, "Event not found");
                    return Ok(false);
                }
            }
        }

        tracing::debug!(
            event_id = %event_id,
            worker_id = %worker_id,
            "Event published successfully (fenced)"
        );
        Ok(true)
    }

    async fn mark_failed(&self, event_id: Uuid, error: &str, retry_at: Option<DateTime<Utc>>) -> Result<(), AppError> {
        // R5-09: Clear claim tracking when failing
        sqlx::query(
            r#"
            UPDATE outbox
            SET status = 'failed',
                last_error = $2,
                next_retry_at = $3,
                claimed_by = NULL,
                claimed_at = NULL,
                claim_expires_at = NULL
            WHERE event_id = $1
            "#,
        )
        .bind(event_id)
        .bind(error)
        .bind(retry_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn mark_dead_letter(&self, event_id: Uuid, error: &str) -> Result<(), AppError> {
        // R5-09: Clear claim tracking when moving to dead letter
        sqlx::query(
            r#"
            UPDATE outbox
            SET status = 'dead_letter',
                last_error = $2,
                next_retry_at = NULL,
                claimed_by = NULL,
                claimed_at = NULL,
                claim_expires_at = NULL
            WHERE event_id = $1
            "#,
        )
        .bind(event_id)
        .bind(error)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn mark_failed_fenced(
        &self,
        event_id: Uuid,
        error: &str,
        retry_at: Option<DateTime<Utc>>,
        worker_id: Uuid,
    ) -> Result<bool, AppError> {
        // R5-09: Only allow fail if lease is still valid and worker matches
        let result = sqlx::query(
            r#"
            UPDATE outbox
            SET status = 'failed',
                last_error = $2,
                next_retry_at = $3,
                claimed_by = NULL,
                claimed_at = NULL,
                claim_expires_at = NULL
            WHERE event_id = $1
              AND status = 'publishing'
              AND claimed_by = $4
              AND claim_expires_at > NOW()
            "#,
        )
        .bind(event_id)
        .bind(error)
        .bind(retry_at)
        .bind(worker_id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            tracing::warn!(
                event_id = %event_id,
                worker_id = %worker_id,
                "mark_failed_fenced: lease expired or worker mismatch - stale worker detected"
            );
            return Ok(false);
        }

        Ok(true)
    }

    async fn mark_dead_letter_fenced(
        &self,
        event_id: Uuid,
        error: &str,
        worker_id: Uuid,
    ) -> Result<bool, AppError> {
        // R5-09: Only allow dead-letter if lease is still valid and worker matches
        let result = sqlx::query(
            r#"
            UPDATE outbox
            SET status = 'dead_letter',
                last_error = $2,
                next_retry_at = NULL,
                claimed_by = NULL,
                claimed_at = NULL,
                claim_expires_at = NULL
            WHERE event_id = $1
              AND status = 'publishing'
              AND claimed_by = $3
              AND claim_expires_at > NOW()
            "#,
        )
        .bind(event_id)
        .bind(error)
        .bind(worker_id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            tracing::warn!(
                event_id = %event_id,
                worker_id = %worker_id,
                "mark_dead_letter_fenced: lease expired or worker mismatch - stale worker detected"
            );
            return Ok(false);
        }

        Ok(true)
    }

    async fn get_retry_events(&self, limit: i32) -> Result<Vec<OutboxEvent>, AppError> {
        let events = sqlx::query_as::<_, OutboxEvent>(
            r#"
            SELECT id, event_id, event_type, event_version, aggregate_type, aggregate_id,
                   payload, metadata, idempotency_key, status, published_at,
                   publish_attempts, last_error, next_retry_at, created_at
            FROM outbox
            WHERE status = 'failed'
              AND next_retry_at IS NOT NULL
              AND next_retry_at <= NOW()
            ORDER BY next_retry_at ASC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(events)
    }

    async fn get_dead_letter_events(&self, limit: i32, offset: i32) -> Result<Vec<OutboxEvent>, AppError> {
        let events = sqlx::query_as::<_, OutboxEvent>(
            r#"
            SELECT id, event_id, event_type, event_version, aggregate_type, aggregate_id,
                   payload, metadata, idempotency_key, status, published_at,
                   publish_attempts, last_error, next_retry_at, created_at
            FROM outbox
            WHERE status = 'dead_letter'
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(events)
    }

    async fn replay_event(&self, event_id: Uuid) -> Result<OutboxEvent, AppError> {
        let event = sqlx::query_as::<_, OutboxEvent>(
            r#"
            UPDATE outbox
            SET status = 'pending',
                publish_attempts = 0,
                last_error = NULL,
                next_retry_at = NULL
            WHERE event_id = $1 AND status = 'dead_letter'
            RETURNING id, event_id, event_type, event_version, aggregate_type, aggregate_id,
                      payload, metadata, idempotency_key, status, published_at,
                      publish_attempts, last_error, next_retry_at, created_at
            "#,
        )
        .bind(event_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(event)
    }

    async fn is_event_processed(&self, source: &str, event_id: &str) -> Result<bool, AppError> {
        let result: Option<(String,)> = sqlx::query_as(
            r#"
            SELECT status FROM inbox
            WHERE source = $1 AND event_id = $2
            "#,
        )
        .bind(source)
        .bind(event_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.is_some())
    }

    async fn record_inbound_event(
        &self,
        source: &str,
        event_id: &str,
        event_type: &str,
        payload: JsonValue,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            INSERT INTO inbox (source, event_id, event_type, payload, status, received_at)
            VALUES ($1, $2, $3, $4, 'received', NOW())
            ON CONFLICT (source, event_id) DO NOTHING
            "#,
        )
        .bind(source)
        .bind(event_id)
        .bind(event_type)
        .bind(&payload)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn mark_inbound_processed(&self, source: &str, event_id: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE inbox
            SET status = 'processed',
                processed_at = NOW(),
                error_message = NULL
            WHERE source = $1 AND event_id = $2
            "#,
        )
        .bind(source)
        .bind(event_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn mark_inbound_failed(&self, source: &str, event_id: &str, error: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE inbox
            SET status = 'failed',
                error_message = $3
            WHERE source = $1 AND event_id = $2
            "#,
        )
        .bind(source)
        .bind(event_id)
        .bind(error)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn register_worker(
        &self,
        worker_id: Uuid,
        worker_name: &str,
        worker_type: &str,
        lease_duration_secs: i64,
    ) -> Result<WorkerLease, AppError> {
        let lease_expires = Utc::now() + Duration::seconds(lease_duration_secs);
        let host = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok());
        let process_id = std::process::id() as i32;

        let lease = sqlx::query_as::<_, WorkerLease>(
            r#"
            INSERT INTO worker_leases (
                worker_id, worker_name, worker_type, heartbeat_at, lease_expires_at,
                host, process_id, started_at
            )
            VALUES ($1, $2, $3, NOW(), $4, $5, $6, NOW())
            ON CONFLICT (worker_id) DO UPDATE SET
                heartbeat_at = NOW(),
                lease_expires_at = $4
            RETURNING worker_id, worker_name, worker_type, heartbeat_at, lease_expires_at,
                      host, process_id, started_at, jobs_processed, jobs_failed, last_job_at
            "#,
        )
        .bind(worker_id)
        .bind(worker_name)
        .bind(worker_type)
        .bind(lease_expires)
        .bind(host.as_deref())
        .bind(process_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(lease)
    }

    async fn heartbeat_worker(&self, worker_id: Uuid, lease_duration_secs: i64) -> Result<bool, AppError> {
        let lease_expires = Utc::now() + Duration::seconds(lease_duration_secs);

        let result = sqlx::query(
            r#"
            UPDATE worker_leases
            SET heartbeat_at = NOW(),
                lease_expires_at = $2
            WHERE worker_id = $1
            "#,
        )
        .bind(worker_id)
        .bind(lease_expires)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_worker_stats(&self, worker_id: Uuid, jobs_processed: i64, jobs_failed: i64) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE worker_leases
            SET jobs_processed = jobs_processed + $2,
                jobs_failed = jobs_failed + $3,
                last_job_at = NOW()
            WHERE worker_id = $1
            "#,
        )
        .bind(worker_id)
        .bind(jobs_processed)
        .bind(jobs_failed)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn unregister_worker(&self, worker_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM worker_leases WHERE worker_id = $1")
            .bind(worker_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_expired_workers(&self) -> Result<Vec<WorkerLease>, AppError> {
        let workers = sqlx::query_as::<_, WorkerLease>(
            r#"
            SELECT worker_id, worker_name, worker_type, heartbeat_at, lease_expires_at,
                   host, process_id, started_at, jobs_processed, jobs_failed, last_job_at
            FROM worker_leases
            WHERE lease_expires_at < NOW()
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(workers)
    }

    async fn cleanup_old_inbox_events(&self, older_than_days: i32) -> Result<i64, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM inbox
            WHERE status = 'processed'
              AND processed_at < NOW() - ($1 || ' days')::interval
            "#,
        )
        .bind(older_than_days)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    async fn cleanup_old_outbox_events(&self, older_than_days: i32) -> Result<i64, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM outbox
            WHERE status = 'published'
              AND published_at < NOW() - ($1 || ' days')::interval
            "#,
        )
        .bind(older_than_days)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    async fn recover_stale_publishing_events(&self) -> Result<i64, AppError> {
        // R5-09: Recover events stuck in 'publishing' with expired claims
        // These are events where the worker crashed or timed out
        let result = sqlx::query(
            r#"
            UPDATE outbox
            SET status = 'pending',
                claimed_by = NULL,
                claimed_at = NULL,
                claim_expires_at = NULL,
                last_error = 'Recovered from stale publishing state (worker claim expired)'
            WHERE status = 'publishing'
              AND claim_expires_at IS NOT NULL
              AND claim_expires_at < NOW()
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let count = result.rows_affected() as i64;
        if count > 0 {
            tracing::info!(count = count, "Recovered stale publishing events");
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outbox_status_conversion() {
        assert_eq!(OutboxStatus::Pending.as_str(), "pending");
        assert_eq!(OutboxStatus::from_str("published"), Some(OutboxStatus::Published));
        assert_eq!(OutboxStatus::from_str("invalid"), None);
    }

    #[test]
    fn test_retry_delay_calculation() {
        // First retry: 10s
        assert_eq!(OutboxRepositoryImpl::calculate_retry_delay(0).num_seconds(), 10);
        // Second retry: 30s
        assert_eq!(OutboxRepositoryImpl::calculate_retry_delay(1).num_seconds(), 30);
        // Third retry: 90s
        assert_eq!(OutboxRepositoryImpl::calculate_retry_delay(2).num_seconds(), 90);
        // Fourth retry: 270s
        assert_eq!(OutboxRepositoryImpl::calculate_retry_delay(3).num_seconds(), 270);
    }
}
