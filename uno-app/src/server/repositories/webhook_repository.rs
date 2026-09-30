//! Webhook repository for outbound/inbound webhook management
//!
//! Implements CRUD operations for:
//! - Webhook endpoint configurations
//! - Outbound webhook deliveries
//! - Inbound webhook events
//! - Source configurations

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Webhook authentication types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "webhook_auth_type", rename_all = "lowercase")]
pub enum WebhookAuthType {
    None,
    Bearer,
    Hmac,
    Basic,
    ApiKey,
}

impl std::fmt::Display for WebhookAuthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhookAuthType::None => write!(f, "none"),
            WebhookAuthType::Bearer => write!(f, "bearer"),
            WebhookAuthType::Hmac => write!(f, "hmac"),
            WebhookAuthType::Basic => write!(f, "basic"),
            WebhookAuthType::ApiKey => write!(f, "api_key"),
        }
    }
}

/// Webhook delivery status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "webhook_delivery_status", rename_all = "snake_case")]
pub enum WebhookDeliveryStatus {
    Pending,
    Delivered,
    Failed,
    Skipped,
    DeadLetter,
}

/// Inbound webhook status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "inbound_webhook_status", rename_all = "lowercase")]
pub enum InboundWebhookStatus {
    Pending,
    Processed,
    Rejected,
    Skipped,
    Duplicate,
}

/// Webhook endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WebhookEndpoint {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub endpoint_url: String,

    // Authentication
    pub auth_type: WebhookAuthType,
    pub auth_header_name: Option<String>,
    #[serde(skip_serializing)]
    pub auth_secret_encrypted: Option<Vec<u8>>,
    pub auth_secret_version: i32,

    // Event filtering
    pub event_types: Vec<String>,
    pub country_codes: Option<Vec<String>>,

    // Request configuration
    pub http_method: String,
    pub content_type: String,
    pub custom_headers: Option<serde_json::Value>,
    pub payload_template: Option<String>,

    // Status and verification
    pub enabled: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub verification_token: Option<String>,
    pub last_success_at: Option<DateTime<Utc>>,
    pub last_failure_at: Option<DateTime<Utc>>,
    pub consecutive_failures: i32,

    // Circuit breaker
    pub circuit_open: bool,
    pub circuit_opened_at: Option<DateTime<Utc>>,
    pub circuit_failure_threshold: i32,
    pub circuit_reset_seconds: i32,

    // Retry configuration
    pub max_retries: i32,
    pub retry_delay_seconds: i32,
    pub retry_backoff_multiplier: f64,
    pub timeout_seconds: i32,

    // Rate limiting
    pub rate_limit_per_minute: Option<i32>,
    pub rate_limit_burst: Option<i32>,

    // Ownership
    pub created_by: String,
    pub updated_by: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Webhook delivery record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WebhookDelivery {
    pub id: Uuid,
    pub endpoint_id: Uuid,

    // Event reference
    pub event_id: Uuid,
    pub event_type: String,
    pub event_sequence: Option<i64>,

    // Request details
    pub request_url: String,
    pub request_method: String,
    pub request_headers: Option<serde_json::Value>,
    pub request_body: Option<String>,
    pub request_body_hash: Option<String>,

    // Delivery status
    pub status: WebhookDeliveryStatus,
    pub attempt_count: i32,
    pub max_attempts: i32,

    // Response details
    pub response_status: Option<i32>,
    pub response_headers: Option<serde_json::Value>,
    pub response_body: Option<String>,
    pub response_time_ms: Option<i32>,

    // Error tracking
    pub error_code: Option<String>,
    pub error_message: Option<String>,

    // Timing
    pub scheduled_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub next_retry_at: Option<DateTime<Utc>>,

    // Idempotency
    pub idempotency_key: String,

    pub created_at: DateTime<Utc>,
}

/// Inbound webhook event
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InboundWebhook {
    pub id: Uuid,

    // Source identification
    pub source: String,
    pub source_ip: Option<String>,

    // Event details
    pub event_type: String,
    pub idempotency_key: String,

    // Payload
    pub raw_headers: serde_json::Value,
    pub raw_body: String,
    pub payload: Option<serde_json::Value>,

    // Signature verification
    pub signature: Option<String>,
    pub signature_valid: Option<bool>,
    pub signature_algorithm: Option<String>,

    // Processing status
    pub status: InboundWebhookStatus,
    pub processed_at: Option<DateTime<Utc>>,

    // Validation results
    pub validation_errors: Option<Vec<String>>,
    pub scope_valid: Option<bool>,

    // Processing results
    pub result_entity_type: Option<String>,
    pub result_entity_id: Option<String>,
    pub result_action: Option<String>,

    // Error tracking
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub max_retries: i32,

    // Timing
    pub received_at: DateTime<Utc>,
    pub next_retry_at: Option<DateTime<Utc>>,
}

/// Webhook source configuration
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WebhookSourceConfig {
    pub id: Uuid,
    pub source: String,
    pub display_name: String,
    pub description: Option<String>,

    // Verification
    #[serde(skip_serializing)]
    pub signing_secret_encrypted: Option<Vec<u8>>,
    pub signature_header: Option<String>,
    pub signature_algorithm: Option<String>,

    // Allowed IPs
    pub allowed_ips: Option<Vec<String>>,

    // Event mapping
    pub event_type_path: Option<String>,
    pub idempotency_key_path: Option<String>,

    // Processing configuration
    pub enabled: bool,
    pub auto_process: bool,
    pub require_signature: bool,

    // Rate limiting
    pub rate_limit_per_minute: Option<i32>,

    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================
// INPUT STRUCTS
// ============================================

/// Input for creating webhook endpoint
#[derive(Debug, Clone)]
pub struct CreateEndpointInput {
    pub name: String,
    pub description: Option<String>,
    pub endpoint_url: String,
    pub auth_type: WebhookAuthType,
    pub auth_header_name: Option<String>,
    pub auth_secret: Option<String>,
    pub event_types: Vec<String>,
    pub country_codes: Option<Vec<String>>,
    pub http_method: Option<String>,
    pub content_type: Option<String>,
    pub custom_headers: Option<serde_json::Value>,
    pub payload_template: Option<String>,
    pub max_retries: Option<i32>,
    pub retry_delay_seconds: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub rate_limit_per_minute: Option<i32>,
    pub created_by: String,
}

/// Input for updating webhook endpoint
#[derive(Debug, Clone)]
pub struct UpdateEndpointInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub endpoint_url: Option<String>,
    pub auth_type: Option<WebhookAuthType>,
    pub auth_secret: Option<String>,
    pub event_types: Option<Vec<String>>,
    pub country_codes: Option<Vec<String>>,
    pub max_retries: Option<i32>,
    pub retry_delay_seconds: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub updated_by: String,
}

/// Input for creating inbound webhook source config
#[derive(Debug, Clone)]
pub struct CreateSourceConfigInput {
    pub source: String,
    pub display_name: String,
    pub description: Option<String>,
    pub signing_secret: Option<String>,
    pub signature_header: Option<String>,
    pub signature_algorithm: Option<String>,
    pub allowed_ips: Option<Vec<String>>,
    pub event_type_path: Option<String>,
    pub idempotency_key_path: Option<String>,
    pub auto_process: bool,
    pub require_signature: bool,
    pub rate_limit_per_minute: Option<i32>,
    pub created_by: String,
}

/// Delivery result input
#[derive(Debug, Clone)]
pub struct DeliveryResultInput {
    pub delivery_id: Uuid,
    pub success: bool,
    pub response_status: Option<i32>,
    pub response_headers: Option<serde_json::Value>,
    pub response_body: Option<String>,
    pub response_time_ms: Option<i32>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

/// Delivery statistics
#[derive(Debug, Clone, Serialize)]
pub struct DeliveryStats {
    pub total_pending: i64,
    pub delivered: i64,
    pub failed: i64,
    pub dead_letter: i64,
}

/// Processing statistics
#[derive(Debug, Clone, Serialize)]
pub struct ProcessingStats {
    pub processed: i64,
    pub rejected: i64,
    pub skipped: i64,
}

// ============================================
// TRAIT DEFINITION
// ============================================

/// Dynamic type alias for WebhookRepository trait object
pub type DynWebhookRepository = Arc<dyn WebhookRepository + Send + Sync>;

/// Webhook repository trait
#[async_trait]
pub trait WebhookRepository: Send + Sync {
    // --- Endpoint Management ---

    /// Create a new endpoint
    async fn create_endpoint(&self, input: CreateEndpointInput) -> Result<WebhookEndpoint, AppError>;

    /// Get endpoint by ID
    async fn get_endpoint(&self, id: Uuid) -> Result<Option<WebhookEndpoint>, AppError>;

    /// List all endpoints
    async fn list_endpoints(&self) -> Result<Vec<WebhookEndpoint>, AppError>;

    /// List enabled endpoints
    async fn list_enabled_endpoints(&self) -> Result<Vec<WebhookEndpoint>, AppError>;

    /// Update endpoint
    async fn update_endpoint(&self, id: Uuid, input: UpdateEndpointInput) -> Result<WebhookEndpoint, AppError>;

    /// Enable/disable endpoint
    async fn toggle_endpoint(&self, id: Uuid, enabled: bool) -> Result<WebhookEndpoint, AppError>;

    /// Mark endpoint verified
    async fn verify_endpoint(&self, id: Uuid) -> Result<WebhookEndpoint, AppError>;

    /// Delete endpoint
    async fn delete_endpoint(&self, id: Uuid) -> Result<(), AppError>;

    /// Check circuit breaker
    async fn check_circuit_breaker(&self, id: Uuid) -> Result<bool, AppError>;

    // --- Delivery Management ---

    /// Queue a delivery (uses database function)
    async fn queue_delivery(&self, event_id: Uuid, event_type: &str, payload: serde_json::Value) -> Result<i32, AppError>;

    /// Get pending deliveries
    async fn get_pending_deliveries(&self, limit: i32) -> Result<Vec<WebhookDelivery>, AppError>;

    /// Get deliveries for retry
    async fn get_retry_deliveries(&self, limit: i32) -> Result<Vec<WebhookDelivery>, AppError>;

    /// Get delivery by ID
    async fn get_delivery(&self, id: Uuid) -> Result<Option<WebhookDelivery>, AppError>;

    /// Get deliveries for endpoint
    async fn get_endpoint_deliveries(&self, endpoint_id: Uuid, limit: i32) -> Result<Vec<WebhookDelivery>, AppError>;

    /// Record delivery result (uses database function)
    async fn record_delivery_result(&self, input: DeliveryResultInput) -> Result<(), AppError>;

    /// Get delivery stats
    async fn get_delivery_stats(&self) -> Result<DeliveryStats, AppError>;

    // --- Inbound Webhook Management ---

    /// Receive inbound webhook (uses database function)
    async fn receive_inbound(&self, source: &str, event_type: &str, idempotency_key: &str,
                             headers: serde_json::Value, body: &str, signature: Option<&str>,
                             source_ip: Option<&str>) -> Result<Uuid, AppError>;

    /// Get pending inbound webhooks
    async fn get_pending_inbound(&self, limit: i32) -> Result<Vec<InboundWebhook>, AppError>;

    /// Get inbound webhook by ID
    async fn get_inbound(&self, id: Uuid) -> Result<Option<InboundWebhook>, AppError>;

    /// Mark inbound processed
    async fn mark_inbound_processed(&self, id: Uuid, entity_type: Option<&str>,
                                     entity_id: Option<&str>, action: &str) -> Result<InboundWebhook, AppError>;

    /// Mark inbound rejected
    async fn mark_inbound_rejected(&self, id: Uuid, error_code: &str, error_message: &str) -> Result<InboundWebhook, AppError>;

    /// Get processing stats
    async fn get_processing_stats(&self) -> Result<ProcessingStats, AppError>;

    // --- Source Configuration ---

    /// Create source config
    async fn create_source_config(&self, input: CreateSourceConfigInput) -> Result<WebhookSourceConfig, AppError>;

    /// Get source config
    async fn get_source_config(&self, source: &str) -> Result<Option<WebhookSourceConfig>, AppError>;

    /// List source configs
    async fn list_source_configs(&self) -> Result<Vec<WebhookSourceConfig>, AppError>;

    /// Toggle source config
    async fn toggle_source_config(&self, source: &str, enabled: bool) -> Result<WebhookSourceConfig, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct WebhookRepositoryImpl {
    pool: ConnectionPool,
}

impl WebhookRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }

    /// Encrypt secret (placeholder - use proper encryption in production)
    fn encrypt_secret(&self, secret: &str) -> Vec<u8> {
        // In production, use proper encryption with a key management service
        secret.as_bytes().to_vec()
    }
}

#[async_trait]
impl WebhookRepository for WebhookRepositoryImpl {
    async fn create_endpoint(&self, input: CreateEndpointInput) -> Result<WebhookEndpoint, AppError> {
        let encrypted_secret = input.auth_secret.as_ref().map(|s| self.encrypt_secret(s));

        let endpoint = sqlx::query_as::<_, WebhookEndpoint>(
            r#"
            INSERT INTO webhook_endpoints (
                name, description, endpoint_url, auth_type, auth_header_name, auth_secret_encrypted,
                event_types, country_codes, http_method, content_type, custom_headers, payload_template,
                max_retries, retry_delay_seconds, timeout_seconds, rate_limit_per_minute, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING *
            "#,
        )
        .bind(&input.name)
        .bind(&input.description)
        .bind(&input.endpoint_url)
        .bind(input.auth_type)
        .bind(&input.auth_header_name)
        .bind(&encrypted_secret)
        .bind(&input.event_types)
        .bind(&input.country_codes)
        .bind(input.http_method.as_deref().unwrap_or("POST"))
        .bind(input.content_type.as_deref().unwrap_or("application/json"))
        .bind(&input.custom_headers)
        .bind(&input.payload_template)
        .bind(input.max_retries.unwrap_or(3))
        .bind(input.retry_delay_seconds.unwrap_or(60))
        .bind(input.timeout_seconds.unwrap_or(30))
        .bind(input.rate_limit_per_minute)
        .bind(&input.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(endpoint)
    }

    async fn get_endpoint(&self, id: Uuid) -> Result<Option<WebhookEndpoint>, AppError> {
        let endpoint = sqlx::query_as::<_, WebhookEndpoint>(
            "SELECT * FROM webhook_endpoints WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(endpoint)
    }

    async fn list_endpoints(&self) -> Result<Vec<WebhookEndpoint>, AppError> {
        let endpoints = sqlx::query_as::<_, WebhookEndpoint>(
            "SELECT * FROM webhook_endpoints ORDER BY name",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(endpoints)
    }

    async fn list_enabled_endpoints(&self) -> Result<Vec<WebhookEndpoint>, AppError> {
        let endpoints = sqlx::query_as::<_, WebhookEndpoint>(
            "SELECT * FROM webhook_endpoints WHERE enabled = TRUE AND circuit_open = FALSE ORDER BY name",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(endpoints)
    }

    async fn update_endpoint(&self, id: Uuid, input: UpdateEndpointInput) -> Result<WebhookEndpoint, AppError> {
        let encrypted_secret = input.auth_secret.as_ref().map(|s| self.encrypt_secret(s));

        let endpoint = sqlx::query_as::<_, WebhookEndpoint>(
            r#"
            UPDATE webhook_endpoints SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                endpoint_url = COALESCE($4, endpoint_url),
                auth_type = COALESCE($5, auth_type),
                auth_secret_encrypted = COALESCE($6, auth_secret_encrypted),
                event_types = COALESCE($7, event_types),
                country_codes = COALESCE($8, country_codes),
                max_retries = COALESCE($9, max_retries),
                retry_delay_seconds = COALESCE($10, retry_delay_seconds),
                timeout_seconds = COALESCE($11, timeout_seconds),
                updated_by = $12,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&input.name)
        .bind(&input.description)
        .bind(&input.endpoint_url)
        .bind(input.auth_type)
        .bind(&encrypted_secret)
        .bind(&input.event_types)
        .bind(&input.country_codes)
        .bind(input.max_retries)
        .bind(input.retry_delay_seconds)
        .bind(input.timeout_seconds)
        .bind(&input.updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(endpoint)
    }

    async fn toggle_endpoint(&self, id: Uuid, enabled: bool) -> Result<WebhookEndpoint, AppError> {
        let endpoint = sqlx::query_as::<_, WebhookEndpoint>(
            r#"
            UPDATE webhook_endpoints
            SET enabled = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(enabled)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(endpoint)
    }

    async fn verify_endpoint(&self, id: Uuid) -> Result<WebhookEndpoint, AppError> {
        let endpoint = sqlx::query_as::<_, WebhookEndpoint>(
            r#"
            UPDATE webhook_endpoints
            SET verified_at = NOW(), updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(endpoint)
    }

    async fn delete_endpoint(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM webhook_endpoints WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn check_circuit_breaker(&self, id: Uuid) -> Result<bool, AppError> {
        let (is_closed,): (bool,) = sqlx::query_as("SELECT check_circuit_breaker($1)")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(is_closed)
    }

    async fn queue_delivery(&self, event_id: Uuid, event_type: &str, payload: serde_json::Value) -> Result<i32, AppError> {
        let (count,): (i32,) = sqlx::query_as("SELECT queue_webhook_delivery($1, $2, $3)")
            .bind(event_id)
            .bind(event_type)
            .bind(&payload)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(count)
    }

    async fn get_pending_deliveries(&self, limit: i32) -> Result<Vec<WebhookDelivery>, AppError> {
        let deliveries = sqlx::query_as::<_, WebhookDelivery>(
            r#"
            SELECT * FROM webhook_deliveries
            WHERE status = 'pending' AND scheduled_at <= NOW()
            ORDER BY scheduled_at
            LIMIT $1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(deliveries)
    }

    async fn get_retry_deliveries(&self, limit: i32) -> Result<Vec<WebhookDelivery>, AppError> {
        let deliveries = sqlx::query_as::<_, WebhookDelivery>(
            r#"
            SELECT * FROM webhook_deliveries
            WHERE status = 'failed' AND next_retry_at IS NOT NULL AND next_retry_at <= NOW()
            ORDER BY next_retry_at
            LIMIT $1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(deliveries)
    }

    async fn get_delivery(&self, id: Uuid) -> Result<Option<WebhookDelivery>, AppError> {
        let delivery = sqlx::query_as::<_, WebhookDelivery>(
            "SELECT * FROM webhook_deliveries WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(delivery)
    }

    async fn get_endpoint_deliveries(&self, endpoint_id: Uuid, limit: i32) -> Result<Vec<WebhookDelivery>, AppError> {
        let deliveries = sqlx::query_as::<_, WebhookDelivery>(
            r#"
            SELECT * FROM webhook_deliveries
            WHERE endpoint_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(endpoint_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(deliveries)
    }

    async fn record_delivery_result(&self, input: DeliveryResultInput) -> Result<(), AppError> {
        sqlx::query("SELECT record_delivery_result($1, $2, $3, $4, $5, $6, $7)")
            .bind(input.delivery_id)
            .bind(input.success)
            .bind(input.response_status)
            .bind(input.response_body.as_deref())
            .bind(input.error_code.as_deref())
            .bind(input.error_message.as_deref())
            .bind(input.response_time_ms)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_delivery_stats(&self) -> Result<DeliveryStats, AppError> {
        let stats = sqlx::query_as::<_, (i64, i64, i64, i64)>(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE status = 'pending') as pending,
                COUNT(*) FILTER (WHERE status = 'delivered') as delivered,
                COUNT(*) FILTER (WHERE status = 'failed') as failed,
                COUNT(*) FILTER (WHERE status = 'dead_letter') as dead_letter
            FROM webhook_deliveries
            WHERE created_at > NOW() - INTERVAL '24 hours'
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(DeliveryStats {
            total_pending: stats.0,
            delivered: stats.1,
            failed: stats.2,
            dead_letter: stats.3,
        })
    }

    async fn receive_inbound(
        &self,
        source: &str,
        event_type: &str,
        idempotency_key: &str,
        headers: serde_json::Value,
        body: &str,
        signature: Option<&str>,
        source_ip: Option<&str>,
    ) -> Result<Uuid, AppError> {
        let (id,): (Uuid,) = sqlx::query_as("SELECT receive_inbound_webhook($1, $2, $3, $4, $5, $6, $7)")
            .bind(source)
            .bind(event_type)
            .bind(idempotency_key)
            .bind(&headers)
            .bind(body)
            .bind(signature)
            .bind(source_ip)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(id)
    }

    async fn get_pending_inbound(&self, limit: i32) -> Result<Vec<InboundWebhook>, AppError> {
        let webhooks = sqlx::query_as::<_, InboundWebhook>(
            r#"
            SELECT * FROM inbound_webhooks
            WHERE status = 'pending'
            ORDER BY received_at
            LIMIT $1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(webhooks)
    }

    async fn get_inbound(&self, id: Uuid) -> Result<Option<InboundWebhook>, AppError> {
        let webhook = sqlx::query_as::<_, InboundWebhook>(
            "SELECT * FROM inbound_webhooks WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(webhook)
    }

    async fn mark_inbound_processed(
        &self,
        id: Uuid,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
        action: &str,
    ) -> Result<InboundWebhook, AppError> {
        let webhook = sqlx::query_as::<_, InboundWebhook>(
            r#"
            UPDATE inbound_webhooks
            SET status = 'processed', processed_at = NOW(),
                result_entity_type = $2, result_entity_id = $3, result_action = $4
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(entity_type)
        .bind(entity_id)
        .bind(action)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(webhook)
    }

    async fn mark_inbound_rejected(
        &self,
        id: Uuid,
        error_code: &str,
        error_message: &str,
    ) -> Result<InboundWebhook, AppError> {
        let webhook = sqlx::query_as::<_, InboundWebhook>(
            r#"
            UPDATE inbound_webhooks
            SET status = 'rejected', error_code = $2, error_message = $3,
                retry_count = retry_count + 1,
                next_retry_at = CASE
                    WHEN retry_count < max_retries - 1 THEN NOW() + INTERVAL '5 minutes'
                    ELSE NULL
                END
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(error_code)
        .bind(error_message)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(webhook)
    }

    async fn get_processing_stats(&self) -> Result<ProcessingStats, AppError> {
        let stats = sqlx::query_as::<_, (i64, i64, i64)>(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE status = 'processed') as processed,
                COUNT(*) FILTER (WHERE status = 'rejected') as rejected,
                COUNT(*) FILTER (WHERE status = 'skipped') as skipped
            FROM inbound_webhooks
            WHERE received_at > NOW() - INTERVAL '24 hours'
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(ProcessingStats {
            processed: stats.0,
            rejected: stats.1,
            skipped: stats.2,
        })
    }

    async fn create_source_config(&self, input: CreateSourceConfigInput) -> Result<WebhookSourceConfig, AppError> {
        let encrypted_secret = input.signing_secret.as_ref().map(|s| self.encrypt_secret(s));

        let config = sqlx::query_as::<_, WebhookSourceConfig>(
            r#"
            INSERT INTO webhook_source_configs (
                source, display_name, description, signing_secret_encrypted,
                signature_header, signature_algorithm, allowed_ips,
                event_type_path, idempotency_key_path, auto_process,
                require_signature, rate_limit_per_minute, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#,
        )
        .bind(&input.source)
        .bind(&input.display_name)
        .bind(&input.description)
        .bind(&encrypted_secret)
        .bind(&input.signature_header)
        .bind(&input.signature_algorithm)
        .bind(&input.allowed_ips)
        .bind(&input.event_type_path)
        .bind(&input.idempotency_key_path)
        .bind(input.auto_process)
        .bind(input.require_signature)
        .bind(input.rate_limit_per_minute)
        .bind(&input.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(config)
    }

    async fn get_source_config(&self, source: &str) -> Result<Option<WebhookSourceConfig>, AppError> {
        let config = sqlx::query_as::<_, WebhookSourceConfig>(
            "SELECT * FROM webhook_source_configs WHERE source = $1",
        )
        .bind(source)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(config)
    }

    async fn list_source_configs(&self) -> Result<Vec<WebhookSourceConfig>, AppError> {
        let configs = sqlx::query_as::<_, WebhookSourceConfig>(
            "SELECT * FROM webhook_source_configs ORDER BY source",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(configs)
    }

    async fn toggle_source_config(&self, source: &str, enabled: bool) -> Result<WebhookSourceConfig, AppError> {
        let config = sqlx::query_as::<_, WebhookSourceConfig>(
            r#"
            UPDATE webhook_source_configs
            SET enabled = $2, updated_at = NOW()
            WHERE source = $1
            RETURNING *
            "#,
        )
        .bind(source)
        .bind(enabled)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(config)
    }
}
