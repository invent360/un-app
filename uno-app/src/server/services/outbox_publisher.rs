//! Outbox publisher service for reliable event delivery
//!
//! Polls outbox events and publishes them to external targets with retry logic.
//! Uses the transactional outbox pattern for at-least-once delivery guarantees.

use async_trait::async_trait;
use chrono::{Duration, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::server::repositories::{DynOutboxRepository, OutboxEvent};
use crate::types::AppError;

/// Configuration for outbox publisher
#[derive(Debug, Clone)]
pub struct OutboxPublisherConfig {
    /// Maximum events to claim per batch
    pub batch_size: i32,
    /// Maximum retry attempts before dead-lettering
    pub max_retries: i32,
    /// Base timeout for HTTP requests in seconds
    pub request_timeout_secs: u64,
    /// Webhook URL for event delivery (if configured)
    pub webhook_url: Option<String>,
    /// HMAC secret for signing webhook payloads
    pub webhook_secret: Option<String>,
}

impl Default for OutboxPublisherConfig {
    fn default() -> Self {
        Self {
            batch_size: 50,
            max_retries: 5,
            request_timeout_secs: 30,
            webhook_url: None,
            webhook_secret: None,
        }
    }
}

/// Statistics from a publish batch
#[derive(Debug, Clone, Default, Serialize)]
pub struct PublishStats {
    pub claimed: u32,
    pub published: u32,
    pub failed: u32,
    pub dead_lettered: u32,
}

/// Webhook payload sent to external targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub event_id: Uuid,
    pub event_type: String,
    pub event_version: i32,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub payload: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub timestamp: String,
}

impl From<&OutboxEvent> for WebhookPayload {
    fn from(event: &OutboxEvent) -> Self {
        Self {
            event_id: event.event_id,
            event_type: event.event_type.clone(),
            event_version: event.event_version,
            aggregate_type: event.aggregate_type.clone(),
            aggregate_id: event.aggregate_id.clone(),
            payload: event.payload.clone(),
            metadata: event.metadata.clone(),
            timestamp: event.created_at.to_rfc3339(),
        }
    }
}

/// Trait for publishing events to external targets
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publish an event to the external target
    async fn publish(&self, event: &OutboxEvent) -> Result<(), AppError>;

    /// Name of this publisher (for logging)
    fn name(&self) -> &str;
}

pub type DynEventPublisher = Arc<dyn EventPublisher>;

/// HTTP webhook publisher
pub struct WebhookPublisher {
    client: Client,
    url: String,
    secret: Option<String>,
}

impl WebhookPublisher {
    pub fn new(url: String, secret: Option<String>, timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            url,
            secret,
        }
    }

    /// Compute HMAC signature for payload
    fn compute_signature(&self, payload: &[u8]) -> Option<String> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        self.secret.as_ref().map(|secret| {
            let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
                .expect("HMAC key length is valid");
            mac.update(payload);
            let result = mac.finalize();
            hex::encode(result.into_bytes())
        })
    }
}

#[async_trait]
impl EventPublisher for WebhookPublisher {
    async fn publish(&self, event: &OutboxEvent) -> Result<(), AppError> {
        let payload = WebhookPayload::from(event);
        let body = serde_json::to_vec(&payload)
            .map_err(|e| AppError::InternalServerError(format!("Failed to serialize webhook payload: {}", e)))?;

        let mut request = self.client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .header("X-Event-ID", event.event_id.to_string())
            .header("X-Event-Type", &event.event_type);

        // Add HMAC signature if secret is configured
        if let Some(signature) = self.compute_signature(&body) {
            request = request.header("X-Signature-256", format!("sha256={}", signature));
        }

        // Add idempotency key if present
        if let Some(ref key) = event.idempotency_key {
            request = request.header("X-Idempotency-Key", key);
        }

        let response = request
            .body(body)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Webhook request failed: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(AppError::InternalServerError(format!(
                "Webhook returned {}: {}",
                status,
                body.chars().take(500).collect::<String>()
            )))
        }
    }

    fn name(&self) -> &str {
        "webhook"
    }
}

/// R5-09: No delivery target configured - events cannot be published
///
/// In production, this publisher MUST fail to ensure events are not
/// silently marked as published without actual delivery.
/// Use `TestPublisher` for testing scenarios where you want to capture events.
pub struct NullPublisher;

#[async_trait]
impl EventPublisher for NullPublisher {
    async fn publish(&self, event: &OutboxEvent) -> Result<(), AppError> {
        // R5-09: MUST NOT silently consume events as published
        // This will cause retry/dead-letter, alerting operators to configure a webhook
        tracing::warn!(
            event_id = %event.event_id,
            event_type = %event.event_type,
            "NullPublisher: No delivery target configured, event cannot be delivered"
        );
        Err(AppError::ConfigError(
            "No webhook delivery target configured. Configure WEBHOOK_URL or disable outbox processing.".to_string()
        ))
    }

    fn name(&self) -> &str {
        "null"
    }
}

/// Test publisher that captures events for verification (use only in tests)
#[cfg(test)]
pub struct TestPublisher {
    events: std::sync::Arc<std::sync::Mutex<Vec<OutboxEvent>>>,
}

#[cfg(test)]
impl TestPublisher {
    pub fn new() -> Self {
        Self {
            events: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn events(&self) -> Vec<OutboxEvent> {
        self.events.lock().unwrap().clone()
    }
}

#[cfg(test)]
#[async_trait]
impl EventPublisher for TestPublisher {
    async fn publish(&self, event: &OutboxEvent) -> Result<(), AppError> {
        self.events.lock().unwrap().push(event.clone());
        Ok(())
    }

    fn name(&self) -> &str {
        "test"
    }
}

/// Outbox publisher service
pub struct OutboxPublisher {
    outbox_repo: DynOutboxRepository,
    publisher: DynEventPublisher,
    config: OutboxPublisherConfig,
    worker_id: Uuid,
}

impl OutboxPublisher {
    pub fn new(
        outbox_repo: DynOutboxRepository,
        publisher: DynEventPublisher,
        config: OutboxPublisherConfig,
    ) -> Self {
        Self {
            outbox_repo,
            publisher,
            config,
            worker_id: Uuid::new_v4(),
        }
    }

    /// Create publisher from environment configuration
    pub fn from_config(
        outbox_repo: DynOutboxRepository,
        config: OutboxPublisherConfig,
    ) -> Self {
        let publisher: DynEventPublisher = if let Some(ref url) = config.webhook_url {
            Arc::new(WebhookPublisher::new(
                url.clone(),
                config.webhook_secret.clone(),
                config.request_timeout_secs,
            ))
        } else {
            Arc::new(NullPublisher)
        };

        Self::new(outbox_repo, publisher, config)
    }

    /// Poll and publish a batch of events
    pub async fn publish_batch(&self) -> Result<PublishStats, AppError> {
        let mut stats = PublishStats::default();

        // Claim events for publishing
        let events = self.outbox_repo.claim_events(self.config.batch_size, self.worker_id).await?;
        stats.claimed = events.len() as u32;

        if events.is_empty() {
            return Ok(stats);
        }

        tracing::debug!(
            publisher = self.publisher.name(),
            claimed = events.len(),
            "Publishing outbox events"
        );

        for event in events {
            match self.publish_single_event(&event).await {
                Ok(()) => {
                    // R5-09: Use fenced version to prevent stale worker writes
                    let success = self.outbox_repo
                        .mark_published_fenced(event.event_id, self.worker_id)
                        .await?;
                    if success {
                        stats.published += 1;
                    } else {
                        tracing::warn!(
                            event_id = %event.event_id,
                            worker_id = %self.worker_id,
                            "Publish fenced: lease expired, event will be retried by another worker"
                        );
                    }
                }
                Err(e) => {
                    let should_dead_letter = event.publish_attempts >= self.config.max_retries;

                    if should_dead_letter {
                        tracing::warn!(
                            event_id = %event.event_id,
                            attempts = event.publish_attempts,
                            error = %e,
                            "Event exceeded max retries, moving to dead letter"
                        );
                        // R5-09: Use fenced version to prevent stale worker writes
                        let success = self.outbox_repo
                            .mark_dead_letter_fenced(event.event_id, &e.to_string(), self.worker_id)
                            .await?;
                        if success {
                            stats.dead_lettered += 1;
                        } else {
                            tracing::warn!(
                                event_id = %event.event_id,
                                worker_id = %self.worker_id,
                                "Dead-letter fenced: lease expired, skipping"
                            );
                        }
                    } else {
                        let retry_at = Utc::now() + self.calculate_retry_delay(event.publish_attempts);
                        tracing::debug!(
                            event_id = %event.event_id,
                            attempts = event.publish_attempts,
                            retry_at = %retry_at,
                            error = %e,
                            "Event publish failed, scheduling retry"
                        );
                        // R5-09: Use fenced version to prevent stale worker writes
                        let success = self.outbox_repo
                            .mark_failed_fenced(event.event_id, &e.to_string(), Some(retry_at), self.worker_id)
                            .await?;
                        if success {
                            stats.failed += 1;
                        } else {
                            tracing::warn!(
                                event_id = %event.event_id,
                                worker_id = %self.worker_id,
                                "Mark-failed fenced: lease expired, skipping"
                            );
                        }
                    }
                }
            }
        }

        tracing::info!(
            publisher = self.publisher.name(),
            published = stats.published,
            failed = stats.failed,
            dead_lettered = stats.dead_lettered,
            "Publish batch completed"
        );

        Ok(stats)
    }

    /// Publish a single event
    async fn publish_single_event(&self, event: &OutboxEvent) -> Result<(), AppError> {
        self.publisher.publish(event).await
    }

    /// Calculate retry delay with exponential backoff
    fn calculate_retry_delay(&self, attempts: i32) -> Duration {
        // Exponential backoff: 10s, 30s, 90s, 270s, 810s (max ~13 min)
        let base_delay_secs = 10i64;
        let multiplier = 3i64.pow(attempts.min(5) as u32);
        Duration::seconds(base_delay_secs * multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_delay_calculation() {
        let publisher = OutboxPublisher {
            outbox_repo: Arc::new(MockOutboxRepo),
            publisher: Arc::new(NullPublisher),
            config: OutboxPublisherConfig::default(),
            worker_id: Uuid::new_v4(),
        };

        // First retry: 10s
        assert_eq!(publisher.calculate_retry_delay(0).num_seconds(), 10);
        // Second retry: 30s
        assert_eq!(publisher.calculate_retry_delay(1).num_seconds(), 30);
        // Third retry: 90s
        assert_eq!(publisher.calculate_retry_delay(2).num_seconds(), 90);
        // Capped at 5 retries for delay calculation
        assert_eq!(publisher.calculate_retry_delay(10).num_seconds(), 2430);
    }

    struct MockOutboxRepo;

    #[async_trait]
    impl crate::server::repositories::OutboxRepository for MockOutboxRepo {
        async fn create_event(&self, _: crate::server::repositories::CreateOutboxEvent) -> Result<OutboxEvent, AppError> {
            unimplemented!()
        }
        async fn claim_events(&self, _: i32, _: Uuid) -> Result<Vec<OutboxEvent>, AppError> {
            Ok(vec![])
        }
        async fn mark_published(&self, _: Uuid) -> Result<(), AppError> { Ok(()) }
        async fn mark_published_fenced(&self, _: Uuid, _: Uuid) -> Result<bool, AppError> { Ok(true) }
        async fn mark_failed(&self, _: Uuid, _: &str, _: Option<chrono::DateTime<Utc>>) -> Result<(), AppError> { Ok(()) }
        async fn mark_dead_letter(&self, _: Uuid, _: &str) -> Result<(), AppError> { Ok(()) }
        async fn mark_failed_fenced(&self, _: Uuid, _: &str, _: Option<chrono::DateTime<Utc>>, _: Uuid) -> Result<bool, AppError> { Ok(true) }
        async fn mark_dead_letter_fenced(&self, _: Uuid, _: &str, _: Uuid) -> Result<bool, AppError> { Ok(true) }
        async fn get_retry_events(&self, _: i32) -> Result<Vec<OutboxEvent>, AppError> { Ok(vec![]) }
        async fn get_dead_letter_events(&self, _: i32, _: i32) -> Result<Vec<OutboxEvent>, AppError> { Ok(vec![]) }
        async fn replay_event(&self, _: Uuid) -> Result<OutboxEvent, AppError> { unimplemented!() }
        async fn is_event_processed(&self, _: &str, _: &str) -> Result<bool, AppError> { Ok(false) }
        async fn record_inbound_event(&self, _: &str, _: &str, _: &str, _: serde_json::Value) -> Result<bool, AppError> { Ok(true) }
        async fn mark_inbound_processed(&self, _: &str, _: &str) -> Result<(), AppError> { Ok(()) }
        async fn mark_inbound_failed(&self, _: &str, _: &str, _: &str) -> Result<(), AppError> { Ok(()) }
        async fn register_worker(&self, _: Uuid, _: &str, _: &str, _: i64) -> Result<crate::server::repositories::WorkerLease, AppError> { unimplemented!() }
        async fn heartbeat_worker(&self, _: Uuid, _: i64) -> Result<bool, AppError> { Ok(true) }
        async fn update_worker_stats(&self, _: Uuid, _: i64, _: i64) -> Result<(), AppError> { Ok(()) }
        async fn unregister_worker(&self, _: Uuid) -> Result<(), AppError> { Ok(()) }
        async fn get_expired_workers(&self) -> Result<Vec<crate::server::repositories::WorkerLease>, AppError> { Ok(vec![]) }
        async fn cleanup_old_inbox_events(&self, _: i32) -> Result<i64, AppError> { Ok(0) }
        async fn cleanup_old_outbox_events(&self, _: i32) -> Result<i64, AppError> { Ok(0) }
        async fn recover_stale_publishing_events(&self) -> Result<i64, AppError> { Ok(0) }
    }

    #[test]
    fn test_publisher_config_defaults() {
        let config = OutboxPublisherConfig::default();
        assert_eq!(config.batch_size, 50);
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.request_timeout_secs, 30);
    }
}
