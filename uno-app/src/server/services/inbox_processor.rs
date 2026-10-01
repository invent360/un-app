//! Inbox processor for handling inbound events with deduplication
//!
//! Implements the inbox pattern for exactly-once processing semantics:
//! - Events are deduplicated by source + event_id
//! - Handlers are registered by event type
//! - Failed events are tracked for retry/manual intervention

use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;

use crate::server::repositories::{DynOutboxRepository, InboxEvent};
use crate::types::AppError;

/// Result of processing an inbound event
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InboxOutcome {
    /// Event was processed successfully
    Processed,
    /// Event was a duplicate (already processed)
    Duplicate,
    /// Event was skipped (no handler registered)
    Skipped,
    /// Event processing failed
    Failed { error: String },
}

/// Context passed to inbox handlers
#[derive(Debug, Clone)]
pub struct InboxContext {
    pub source: String,
    pub event_id: String,
    pub event_type: String,
    pub received_at: chrono::DateTime<chrono::Utc>,
}

impl From<&InboxEvent> for InboxContext {
    fn from(event: &InboxEvent) -> Self {
        Self {
            source: event.source.clone(),
            event_id: event.event_id.clone(),
            event_type: event.event_type.clone(),
            received_at: event.received_at,
        }
    }
}

/// Trait for inbox event handlers
#[async_trait]
pub trait InboxHandler: Send + Sync {
    /// Event types this handler processes
    fn event_types(&self) -> &[&str];

    /// Handle an inbound event
    async fn handle(&self, ctx: &InboxContext, payload: &JsonValue) -> Result<(), AppError>;

    /// Name of this handler (for logging)
    fn name(&self) -> &str;
}

pub type DynInboxHandler = Arc<dyn InboxHandler>;

/// Inbox processor with handler registry
pub struct InboxProcessor {
    outbox_repo: DynOutboxRepository,
    handlers: HashMap<String, DynInboxHandler>,
}

impl InboxProcessor {
    pub fn new(outbox_repo: DynOutboxRepository) -> Self {
        Self {
            outbox_repo,
            handlers: HashMap::new(),
        }
    }

    /// Register a handler for its event types
    pub fn register(&mut self, handler: DynInboxHandler) {
        for event_type in handler.event_types() {
            tracing::debug!(
                handler = handler.name(),
                event_type = event_type,
                "Registering inbox handler"
            );
            self.handlers.insert(event_type.to_string(), handler.clone());
        }
    }

    /// Register a handler (builder pattern)
    pub fn with_handler(mut self, handler: DynInboxHandler) -> Self {
        self.register(handler);
        self
    }

    /// Process an inbound event with deduplication
    pub async fn process(
        &self,
        source: &str,
        event_id: &str,
        event_type: &str,
        payload: JsonValue,
    ) -> Result<InboxOutcome, AppError> {
        // 1. Check if already processed (deduplication)
        if self.outbox_repo.is_event_processed(source, event_id).await? {
            tracing::debug!(
                source = source,
                event_id = event_id,
                "Duplicate event received, skipping"
            );
            return Ok(InboxOutcome::Duplicate);
        }

        // 2. Record the inbound event (also serves as dedup marker)
        let recorded = self.outbox_repo
            .record_inbound_event(source, event_id, event_type, payload.clone())
            .await?;

        if !recorded {
            // Race condition: another worker recorded it first
            return Ok(InboxOutcome::Duplicate);
        }

        // 3. Find handler for this event type
        let handler = match self.handlers.get(event_type) {
            Some(h) => h,
            None => {
                tracing::debug!(
                    source = source,
                    event_id = event_id,
                    event_type = event_type,
                    "No handler registered for event type, skipping"
                );
                // Mark as skipped (not failed)
                // The inbox record already exists, just update status
                return Ok(InboxOutcome::Skipped);
            }
        };

        // 4. Build context and dispatch to handler
        let ctx = InboxContext {
            source: source.to_string(),
            event_id: event_id.to_string(),
            event_type: event_type.to_string(),
            received_at: chrono::Utc::now(),
        };

        tracing::debug!(
            source = source,
            event_id = event_id,
            event_type = event_type,
            handler = handler.name(),
            "Processing inbox event"
        );

        match handler.handle(&ctx, &payload).await {
            Ok(()) => {
                self.outbox_repo.mark_inbound_processed(source, event_id).await?;
                tracing::info!(
                    source = source,
                    event_id = event_id,
                    handler = handler.name(),
                    "Inbox event processed successfully"
                );
                Ok(InboxOutcome::Processed)
            }
            Err(e) => {
                let error_msg = e.to_string();
                self.outbox_repo.mark_inbound_failed(source, event_id, &error_msg).await?;
                tracing::warn!(
                    source = source,
                    event_id = event_id,
                    handler = handler.name(),
                    error = %error_msg,
                    "Inbox event processing failed"
                );
                Ok(InboxOutcome::Failed { error: error_msg })
            }
        }
    }

    /// Get registered event types
    pub fn registered_event_types(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }

    /// Check if a handler is registered for an event type
    pub fn has_handler(&self, event_type: &str) -> bool {
        self.handlers.contains_key(event_type)
    }
}

/// Example handler for logging events (useful for debugging)
pub struct LoggingHandler {
    event_types: Vec<&'static str>,
}

impl LoggingHandler {
    pub fn new(event_types: Vec<&'static str>) -> Self {
        Self { event_types }
    }

    pub fn all() -> Self {
        Self {
            event_types: vec!["*"],
        }
    }
}

#[async_trait]
impl InboxHandler for LoggingHandler {
    fn event_types(&self) -> &[&str] {
        &self.event_types
    }

    async fn handle(&self, ctx: &InboxContext, payload: &JsonValue) -> Result<(), AppError> {
        tracing::info!(
            source = %ctx.source,
            event_id = %ctx.event_id,
            event_type = %ctx.event_type,
            payload = %payload,
            "Inbox event logged"
        );
        Ok(())
    }

    fn name(&self) -> &str {
        "logging"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler {
        types: Vec<&'static str>,
        should_fail: bool,
    }

    #[async_trait]
    impl InboxHandler for TestHandler {
        fn event_types(&self) -> &[&str] {
            &self.types
        }

        async fn handle(&self, _ctx: &InboxContext, _payload: &JsonValue) -> Result<(), AppError> {
            if self.should_fail {
                Err(AppError::InternalServerError("Test failure".to_string()))
            } else {
                Ok(())
            }
        }

        fn name(&self) -> &str {
            "test"
        }
    }

    #[test]
    fn test_handler_registration() {
        let processor = InboxProcessor::new(Arc::new(MockOutboxRepo))
            .with_handler(Arc::new(TestHandler {
                types: vec!["license.claimed", "license.released"],
                should_fail: false,
            }));

        assert!(processor.has_handler("license.claimed"));
        assert!(processor.has_handler("license.released"));
        assert!(!processor.has_handler("license.expired"));
    }

    #[test]
    fn test_registered_event_types() {
        let processor = InboxProcessor::new(Arc::new(MockOutboxRepo))
            .with_handler(Arc::new(TestHandler {
                types: vec!["event.a", "event.b"],
                should_fail: false,
            }));

        let types = processor.registered_event_types();
        assert_eq!(types.len(), 2);
        assert!(types.contains(&"event.a".to_string()));
        assert!(types.contains(&"event.b".to_string()));
    }

    struct MockOutboxRepo;

    #[async_trait]
    impl crate::server::repositories::OutboxRepository for MockOutboxRepo {
        async fn create_event(&self, _: crate::server::repositories::CreateOutboxEvent) -> Result<crate::server::repositories::OutboxEvent, AppError> {
            unimplemented!()
        }
        async fn claim_events(&self, _: i32, _: uuid::Uuid) -> Result<Vec<crate::server::repositories::OutboxEvent>, AppError> {
            Ok(vec![])
        }
        async fn mark_published(&self, _: uuid::Uuid) -> Result<(), AppError> { Ok(()) }
        async fn mark_published_fenced(&self, _: uuid::Uuid, _: uuid::Uuid) -> Result<bool, AppError> { Ok(true) }
        async fn mark_failed(&self, _: uuid::Uuid, _: &str, _: Option<chrono::DateTime<chrono::Utc>>) -> Result<(), AppError> { Ok(()) }
        async fn mark_dead_letter(&self, _: uuid::Uuid, _: &str) -> Result<(), AppError> { Ok(()) }
        async fn mark_failed_fenced(&self, _: uuid::Uuid, _: &str, _: Option<chrono::DateTime<chrono::Utc>>, _: uuid::Uuid) -> Result<bool, AppError> { Ok(true) }
        async fn mark_dead_letter_fenced(&self, _: uuid::Uuid, _: &str, _: uuid::Uuid) -> Result<bool, AppError> { Ok(true) }
        async fn get_retry_events(&self, _: i32) -> Result<Vec<crate::server::repositories::OutboxEvent>, AppError> { Ok(vec![]) }
        async fn get_dead_letter_events(&self, _: i32, _: i32) -> Result<Vec<crate::server::repositories::OutboxEvent>, AppError> { Ok(vec![]) }
        async fn replay_event(&self, _: uuid::Uuid) -> Result<crate::server::repositories::OutboxEvent, AppError> { unimplemented!() }
        async fn is_event_processed(&self, _: &str, _: &str) -> Result<bool, AppError> { Ok(false) }
        async fn record_inbound_event(&self, _: &str, _: &str, _: &str, _: serde_json::Value) -> Result<bool, AppError> { Ok(true) }
        async fn mark_inbound_processed(&self, _: &str, _: &str) -> Result<(), AppError> { Ok(()) }
        async fn mark_inbound_failed(&self, _: &str, _: &str, _: &str) -> Result<(), AppError> { Ok(()) }
        async fn register_worker(&self, _: uuid::Uuid, _: &str, _: &str, _: i64) -> Result<crate::server::repositories::WorkerLease, AppError> { unimplemented!() }
        async fn heartbeat_worker(&self, _: uuid::Uuid, _: i64) -> Result<bool, AppError> { Ok(true) }
        async fn update_worker_stats(&self, _: uuid::Uuid, _: i64, _: i64) -> Result<(), AppError> { Ok(()) }
        async fn unregister_worker(&self, _: uuid::Uuid) -> Result<(), AppError> { Ok(()) }
        async fn get_expired_workers(&self) -> Result<Vec<crate::server::repositories::WorkerLease>, AppError> { Ok(vec![]) }
        async fn cleanup_old_inbox_events(&self, _: i32) -> Result<i64, AppError> { Ok(0) }
        async fn cleanup_old_outbox_events(&self, _: i32) -> Result<i64, AppError> { Ok(0) }
        async fn recover_stale_publishing_events(&self) -> Result<i64, AppError> { Ok(0) }
    }
}
