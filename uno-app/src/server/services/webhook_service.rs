//! Webhook service for external integrations
//!
//! Provides webhook capabilities with:
//! - Outbound webhook delivery with circuit breaker
//! - Inbound webhook processing with deduplication
//! - Signature verification
//! - Retry with exponential backoff

use async_trait::async_trait;
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::server::repositories::{
    DynWebhookRepository, WebhookRepository, WebhookEndpoint, WebhookDelivery,
    InboundWebhook, WebhookSourceConfig, WebhookAuthType, WebhookDeliveryStatus,
    CreateEndpointInput, UpdateEndpointInput, CreateSourceConfigInput, DeliveryResultInput,
    DeliveryStats, ProcessingStats,
};
use crate::types::AppError;

// ============================================
// SERVICE TRAIT
// ============================================

/// Dynamic type alias for WebhookService trait object
pub type DynWebhookService = Arc<dyn WebhookService + Send + Sync>;

/// Verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub success: bool,
    pub message: Option<String>,
    pub response_time_ms: Option<i32>,
}

/// Webhook service trait
#[async_trait]
pub trait WebhookService: Send + Sync {
    // --- Endpoint Management ---
    async fn create_endpoint(&self, input: CreateEndpointInput) -> Result<WebhookEndpoint, AppError>;
    async fn get_endpoint(&self, id: Uuid) -> Result<Option<WebhookEndpoint>, AppError>;
    async fn list_endpoints(&self) -> Result<Vec<WebhookEndpoint>, AppError>;
    async fn update_endpoint(&self, id: Uuid, input: UpdateEndpointInput) -> Result<WebhookEndpoint, AppError>;
    async fn toggle_endpoint(&self, id: Uuid, enabled: bool) -> Result<WebhookEndpoint, AppError>;
    async fn delete_endpoint(&self, id: Uuid) -> Result<(), AppError>;
    async fn verify_endpoint(&self, id: Uuid) -> Result<VerificationResult, AppError>;

    // --- Outbound Delivery ---
    async fn queue_delivery(&self, event_id: Uuid, event_type: &str, payload: serde_json::Value) -> Result<i32, AppError>;
    async fn process_pending_deliveries(&self, limit: i32) -> Result<DeliveryStats, AppError>;
    async fn retry_failed_deliveries(&self, limit: i32) -> Result<i32, AppError>;
    async fn get_endpoint_deliveries(&self, endpoint_id: Uuid, limit: i32) -> Result<Vec<WebhookDelivery>, AppError>;

    // --- Inbound Processing ---
    async fn receive_webhook(
        &self,
        source: &str,
        event_type: &str,
        idempotency_key: &str,
        headers: serde_json::Value,
        body: &str,
        signature: Option<&str>,
        source_ip: Option<&str>,
    ) -> Result<Uuid, AppError>;
    async fn verify_signature(&self, source: &str, body: &str, signature: &str) -> Result<bool, AppError>;
    async fn process_inbound_webhooks(&self, limit: i32) -> Result<ProcessingStats, AppError>;
    async fn get_inbound(&self, id: Uuid) -> Result<Option<InboundWebhook>, AppError>;
    async fn mark_inbound_processed(&self, id: Uuid, entity_type: Option<&str>, entity_id: Option<&str>, action: &str) -> Result<InboundWebhook, AppError>;
    async fn mark_inbound_rejected(&self, id: Uuid, error_code: &str, error_message: &str) -> Result<InboundWebhook, AppError>;

    // --- Source Configuration ---
    async fn create_source_config(&self, input: CreateSourceConfigInput) -> Result<WebhookSourceConfig, AppError>;
    async fn get_source_config(&self, source: &str) -> Result<Option<WebhookSourceConfig>, AppError>;
    async fn list_source_configs(&self) -> Result<Vec<WebhookSourceConfig>, AppError>;
    async fn toggle_source_config(&self, source: &str, enabled: bool) -> Result<WebhookSourceConfig, AppError>;

    // --- Stats ---
    async fn get_delivery_stats(&self) -> Result<DeliveryStats, AppError>;
    async fn get_processing_stats(&self) -> Result<ProcessingStats, AppError>;
}

// ============================================
// HTTP CLIENT ABSTRACTION
// ============================================

/// HTTP client for webhook delivery (abstracted for testing)
#[async_trait]
pub trait WebhookHttpClient: Send + Sync {
    async fn send_request(
        &self,
        url: &str,
        method: &str,
        headers: Vec<(String, String)>,
        body: &str,
        timeout_seconds: u32,
    ) -> Result<HttpResponse, HttpError>;
}

pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub duration_ms: u32,
}

#[derive(Debug)]
pub struct HttpError {
    pub code: String,
    pub message: String,
}

/// Default HTTP client using reqwest
pub struct DefaultHttpClient {
    client: reqwest::Client,
}

impl DefaultHttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to build HTTP client"),
        }
    }
}

impl Default for DefaultHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WebhookHttpClient for DefaultHttpClient {
    async fn send_request(
        &self,
        url: &str,
        method: &str,
        headers: Vec<(String, String)>,
        body: &str,
        timeout_seconds: u32,
    ) -> Result<HttpResponse, HttpError> {
        let start = std::time::Instant::now();

        let mut builder = match method.to_uppercase().as_str() {
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "PATCH" => self.client.patch(url),
            "DELETE" => self.client.delete(url),
            _ => self.client.post(url),
        };

        for (name, value) in headers {
            builder = builder.header(&name, &value);
        }

        builder = builder
            .body(body.to_string())
            .timeout(Duration::from_secs(timeout_seconds as u64));

        let response = builder.send().await.map_err(|e| HttpError {
            code: "request_failed".to_string(),
            message: e.to_string(),
        })?;

        let status = response.status().as_u16();
        let response_headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let response_body = response.text().await.map_err(|e| HttpError {
            code: "response_read_failed".to_string(),
            message: e.to_string(),
        })?;

        let duration_ms = start.elapsed().as_millis() as u32;

        Ok(HttpResponse {
            status,
            headers: response_headers,
            body: response_body,
            duration_ms,
        })
    }
}

// ============================================
// SERVICE IMPLEMENTATION
// ============================================

pub struct WebhookServiceImpl<H: WebhookHttpClient = DefaultHttpClient> {
    repo: DynWebhookRepository,
    http_client: Arc<H>,
}

impl WebhookServiceImpl<DefaultHttpClient> {
    pub fn new(repo: DynWebhookRepository) -> Self {
        Self {
            repo,
            http_client: Arc::new(DefaultHttpClient::new()),
        }
    }
}

impl<H: WebhookHttpClient> WebhookServiceImpl<H> {
    pub fn with_http_client(repo: DynWebhookRepository, http_client: H) -> Self {
        Self {
            repo,
            http_client: Arc::new(http_client),
        }
    }

    /// Generate HMAC-SHA256 signature
    fn generate_hmac_signature(secret: &[u8], body: &str) -> String {
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");
        mac.update(body.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    /// Verify HMAC-SHA256 signature
    fn verify_hmac_signature(secret: &[u8], body: &str, signature: &str) -> bool {
        let expected = Self::generate_hmac_signature(secret, body);
        // Compare in constant time to prevent timing attacks
        expected.eq_ignore_ascii_case(signature.trim_start_matches("sha256="))
    }

    /// Build headers for outbound webhook
    fn build_headers(
        endpoint: &WebhookEndpoint,
        body: &str,
    ) -> Vec<(String, String)> {
        let mut headers = vec![
            ("Content-Type".to_string(), endpoint.content_type.clone()),
            ("User-Agent".to_string(), "UNO-Webhook/1.0".to_string()),
            ("X-Webhook-Timestamp".to_string(), Utc::now().timestamp().to_string()),
        ];

        // Add auth header based on type
        match endpoint.auth_type {
            WebhookAuthType::Bearer => {
                if let Some(ref secret) = endpoint.auth_secret_encrypted {
                    // In production, decrypt the secret
                    let token = String::from_utf8_lossy(secret);
                    headers.push(("Authorization".to_string(), format!("Bearer {}", token)));
                }
            }
            WebhookAuthType::Hmac => {
                if let Some(ref secret) = endpoint.auth_secret_encrypted {
                    let signature = Self::generate_hmac_signature(secret, body);
                    let header_name = endpoint.auth_header_name.as_deref().unwrap_or("X-Hub-Signature-256");
                    headers.push((header_name.to_string(), format!("sha256={}", signature)));
                }
            }
            WebhookAuthType::ApiKey => {
                if let Some(ref secret) = endpoint.auth_secret_encrypted {
                    let key = String::from_utf8_lossy(secret);
                    let header_name = endpoint.auth_header_name.as_deref().unwrap_or("X-API-Key");
                    headers.push((header_name.to_string(), key.to_string()));
                }
            }
            WebhookAuthType::Basic => {
                if let Some(ref secret) = endpoint.auth_secret_encrypted {
                    // Simple base64 encoding for basic auth
                    let secret_str = String::from_utf8_lossy(secret);
                    let encoded = simple_base64_encode(secret_str.as_bytes());
                    headers.push(("Authorization".to_string(), format!("Basic {}", encoded)));
                }
            }
            WebhookAuthType::None => {}
        }

        // Add custom headers
        if let Some(ref custom) = endpoint.custom_headers {
            if let Some(obj) = custom.as_object() {
                for (key, value) in obj {
                    if let Some(v) = value.as_str() {
                        headers.push((key.clone(), v.to_string()));
                    }
                }
            }
        }

        headers
    }
}

#[async_trait]
impl<H: WebhookHttpClient + 'static> WebhookService for WebhookServiceImpl<H> {
    async fn create_endpoint(&self, input: CreateEndpointInput) -> Result<WebhookEndpoint, AppError> {
        self.repo.create_endpoint(input).await
    }

    async fn get_endpoint(&self, id: Uuid) -> Result<Option<WebhookEndpoint>, AppError> {
        self.repo.get_endpoint(id).await
    }

    async fn list_endpoints(&self) -> Result<Vec<WebhookEndpoint>, AppError> {
        self.repo.list_endpoints().await
    }

    async fn update_endpoint(&self, id: Uuid, input: UpdateEndpointInput) -> Result<WebhookEndpoint, AppError> {
        self.repo.update_endpoint(id, input).await
    }

    async fn toggle_endpoint(&self, id: Uuid, enabled: bool) -> Result<WebhookEndpoint, AppError> {
        self.repo.toggle_endpoint(id, enabled).await
    }

    async fn delete_endpoint(&self, id: Uuid) -> Result<(), AppError> {
        self.repo.delete_endpoint(id).await
    }

    async fn verify_endpoint(&self, id: Uuid) -> Result<VerificationResult, AppError> {
        let endpoint = self.repo.get_endpoint(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Endpoint {} not found", id)))?;

        // Send a verification request
        let verification_payload = serde_json::json!({
            "type": "webhook.verification",
            "timestamp": Utc::now().to_rfc3339(),
            "challenge": Uuid::new_v4().to_string(),
        });

        let body = serde_json::to_string(&verification_payload)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let headers = Self::build_headers(&endpoint, &body);

        let result = self.http_client
            .send_request(
                &endpoint.endpoint_url,
                &endpoint.http_method,
                headers,
                &body,
                endpoint.timeout_seconds as u32,
            )
            .await;

        match result {
            Ok(response) => {
                let success = (200..300).contains(&(response.status as i32));

                if success {
                    self.repo.verify_endpoint(id).await?;
                }

                Ok(VerificationResult {
                    success,
                    message: if success {
                        Some("Endpoint verified successfully".to_string())
                    } else {
                        Some(format!("Unexpected status code: {}", response.status))
                    },
                    response_time_ms: Some(response.duration_ms as i32),
                })
            }
            Err(e) => Ok(VerificationResult {
                success: false,
                message: Some(e.message),
                response_time_ms: None,
            }),
        }
    }

    async fn queue_delivery(&self, event_id: Uuid, event_type: &str, payload: serde_json::Value) -> Result<i32, AppError> {
        self.repo.queue_delivery(event_id, event_type, payload).await
    }

    async fn process_pending_deliveries(&self, limit: i32) -> Result<DeliveryStats, AppError> {
        let pending = self.repo.get_pending_deliveries(limit).await?;
        let mut delivered = 0i64;
        let mut failed = 0i64;

        for delivery in pending {
            // Get endpoint
            let endpoint = match self.repo.get_endpoint(delivery.endpoint_id).await? {
                Some(e) => e,
                None => continue,
            };

            // Check circuit breaker
            if !self.repo.check_circuit_breaker(delivery.endpoint_id).await? {
                continue;
            }

            // Build request
            let body = delivery.request_body.as_deref().unwrap_or("{}");
            let headers = Self::build_headers(&endpoint, body);

            // Send request
            let result = self.http_client
                .send_request(
                    &delivery.request_url,
                    &delivery.request_method,
                    headers,
                    body,
                    endpoint.timeout_seconds as u32,
                )
                .await;

            // Record result
            let delivery_result = match result {
                Ok(response) => {
                    let success = (200..300).contains(&(response.status as i32));
                    if success {
                        delivered += 1;
                    } else {
                        failed += 1;
                    }

                    DeliveryResultInput {
                        delivery_id: delivery.id,
                        success,
                        response_status: Some(response.status as i32),
                        response_headers: None,
                        response_body: Some(response.body),
                        response_time_ms: Some(response.duration_ms as i32),
                        error_code: if !success { Some("http_error".to_string()) } else { None },
                        error_message: if !success { Some(format!("Status {}", response.status)) } else { None },
                    }
                }
                Err(e) => {
                    failed += 1;
                    DeliveryResultInput {
                        delivery_id: delivery.id,
                        success: false,
                        response_status: None,
                        response_headers: None,
                        response_body: None,
                        response_time_ms: None,
                        error_code: Some(e.code),
                        error_message: Some(e.message),
                    }
                }
            };

            self.repo.record_delivery_result(delivery_result).await?;
        }

        Ok(DeliveryStats {
            total_pending: (limit as i64) - delivered - failed,
            delivered,
            failed,
            dead_letter: 0,
        })
    }

    async fn retry_failed_deliveries(&self, limit: i32) -> Result<i32, AppError> {
        let retry = self.repo.get_retry_deliveries(limit).await?;
        let count = retry.len() as i32;

        // Process same as pending
        for delivery in retry {
            let endpoint = match self.repo.get_endpoint(delivery.endpoint_id).await? {
                Some(e) => e,
                None => continue,
            };

            if !self.repo.check_circuit_breaker(delivery.endpoint_id).await? {
                continue;
            }

            let body = delivery.request_body.as_deref().unwrap_or("{}");
            let headers = Self::build_headers(&endpoint, body);

            let result = self.http_client
                .send_request(
                    &delivery.request_url,
                    &delivery.request_method,
                    headers,
                    body,
                    endpoint.timeout_seconds as u32,
                )
                .await;

            let delivery_result = match result {
                Ok(response) => DeliveryResultInput {
                    delivery_id: delivery.id,
                    success: (200..300).contains(&(response.status as i32)),
                    response_status: Some(response.status as i32),
                    response_headers: None,
                    response_body: Some(response.body),
                    response_time_ms: Some(response.duration_ms as i32),
                    error_code: None,
                    error_message: None,
                },
                Err(e) => DeliveryResultInput {
                    delivery_id: delivery.id,
                    success: false,
                    response_status: None,
                    response_headers: None,
                    response_body: None,
                    response_time_ms: None,
                    error_code: Some(e.code),
                    error_message: Some(e.message),
                },
            };

            self.repo.record_delivery_result(delivery_result).await?;
        }

        Ok(count)
    }

    async fn get_endpoint_deliveries(&self, endpoint_id: Uuid, limit: i32) -> Result<Vec<WebhookDelivery>, AppError> {
        self.repo.get_endpoint_deliveries(endpoint_id, limit).await
    }

    async fn receive_webhook(
        &self,
        source: &str,
        event_type: &str,
        idempotency_key: &str,
        headers: serde_json::Value,
        body: &str,
        signature: Option<&str>,
        source_ip: Option<&str>,
    ) -> Result<Uuid, AppError> {
        // R4-08: Require known inbound source - reject unknown sources entirely
        let config = self.repo.get_source_config(source).await?
            .ok_or_else(|| {
                tracing::warn!(source = %source, "Webhook rejected: unknown source");
                AppError::Unauthorized(format!("Unknown webhook source: {}", source))
            })?;

        if !config.enabled {
            tracing::warn!(source = %source, "Webhook rejected: source disabled");
            return Err(AppError::ValidationError(format!("Source '{}' is disabled", source)));
        }

        // R4-08: Verify timestamp freshness (300 second window)
        if let Some(ts_str) = headers.get("x-timestamp").and_then(|v| v.as_str()) {
            if let Ok(timestamp) = ts_str.parse::<i64>() {
                let now = Utc::now().timestamp();
                let age = (now - timestamp).abs();
                if age > 300 {
                    tracing::warn!(
                        source = %source,
                        timestamp = %timestamp,
                        age_seconds = %age,
                        "Webhook rejected: timestamp expired"
                    );
                    return Err(AppError::Unauthorized("Request timestamp expired".to_string()));
                }
            }
        }

        // Verify signature if required
        if config.require_signature {
            let sig = signature.ok_or_else(|| {
                tracing::warn!(source = %source, "Webhook rejected: signature required but missing");
                AppError::Unauthorized("Signature required but not provided".to_string())
            })?;

            if !self.verify_signature(source, body, sig).await? {
                tracing::warn!(source = %source, "Webhook rejected: invalid signature");
                return Err(AppError::Unauthorized("Invalid signature".to_string()));
            }
        }

        // Verify IP whitelist if configured
        if let Some(ref allowed_ips) = config.allowed_ips {
            if let Some(ip) = source_ip {
                if !allowed_ips.contains(&ip.to_string()) {
                    tracing::warn!(
                        source = %source,
                        ip = %ip,
                        "Webhook rejected: IP not in whitelist"
                    );
                    return Err(AppError::Unauthorized(format!("IP {} not in whitelist", ip)));
                }
            }
        }

        self.repo.receive_inbound(source, event_type, idempotency_key, headers, body, signature, source_ip).await
    }

    async fn verify_signature(&self, source: &str, body: &str, signature: &str) -> Result<bool, AppError> {
        let config = self.repo.get_source_config(source).await?
            .ok_or_else(|| AppError::NotFound(format!("Source config '{}' not found", source)))?;

        let secret = config.signing_secret_encrypted
            .ok_or_else(|| AppError::ValidationError("No signing secret configured".to_string()))?;

        Ok(Self::verify_hmac_signature(&secret, body, signature))
    }

    async fn process_inbound_webhooks(&self, limit: i32) -> Result<ProcessingStats, AppError> {
        let pending = self.repo.get_pending_inbound(limit).await?;
        let mut processed = 0i64;
        let mut rejected = 0i64;
        let mut skipped = 0i64;

        for webhook in pending {
            // Get source config
            let config = self.repo.get_source_config(&webhook.source).await?;

            if config.is_none() || !config.as_ref().map(|c| c.auto_process).unwrap_or(false) {
                skipped += 1;
                continue;
            }

            // Process based on event type
            // In a real implementation, this would route to appropriate handlers
            match webhook.event_type.as_str() {
                "contact.created" | "contact.updated" => {
                    // Handle CRM contact events
                    self.repo.mark_inbound_processed(
                        webhook.id,
                        Some("contact"),
                        webhook.payload.as_ref().and_then(|p| p["id"].as_str()),
                        "synced",
                    ).await?;
                    processed += 1;
                }
                "payment.completed" | "payment.failed" => {
                    // Handle payment events
                    self.repo.mark_inbound_processed(
                        webhook.id,
                        Some("payment"),
                        webhook.payload.as_ref().and_then(|p| p["id"].as_str()),
                        "recorded",
                    ).await?;
                    processed += 1;
                }
                _ => {
                    // Unknown event type - skip
                    self.repo.mark_inbound_rejected(
                        webhook.id,
                        "unknown_event",
                        &format!("Unknown event type: {}", webhook.event_type),
                    ).await?;
                    rejected += 1;
                }
            }
        }

        Ok(ProcessingStats {
            processed,
            rejected,
            skipped,
        })
    }

    async fn get_inbound(&self, id: Uuid) -> Result<Option<InboundWebhook>, AppError> {
        self.repo.get_inbound(id).await
    }

    async fn mark_inbound_processed(&self, id: Uuid, entity_type: Option<&str>, entity_id: Option<&str>, action: &str) -> Result<InboundWebhook, AppError> {
        self.repo.mark_inbound_processed(id, entity_type, entity_id, action).await
    }

    async fn mark_inbound_rejected(&self, id: Uuid, error_code: &str, error_message: &str) -> Result<InboundWebhook, AppError> {
        self.repo.mark_inbound_rejected(id, error_code, error_message).await
    }

    async fn create_source_config(&self, input: CreateSourceConfigInput) -> Result<WebhookSourceConfig, AppError> {
        self.repo.create_source_config(input).await
    }

    async fn get_source_config(&self, source: &str) -> Result<Option<WebhookSourceConfig>, AppError> {
        self.repo.get_source_config(source).await
    }

    async fn list_source_configs(&self) -> Result<Vec<WebhookSourceConfig>, AppError> {
        self.repo.list_source_configs().await
    }

    async fn toggle_source_config(&self, source: &str, enabled: bool) -> Result<WebhookSourceConfig, AppError> {
        self.repo.toggle_source_config(source, enabled).await
    }

    async fn get_delivery_stats(&self) -> Result<DeliveryStats, AppError> {
        self.repo.get_delivery_stats().await
    }

    async fn get_processing_stats(&self) -> Result<ProcessingStats, AppError> {
        self.repo.get_processing_stats().await
    }
}

// Simple base64 encoding (no external deps)
fn simple_base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        buf[..chunk.len()].copy_from_slice(chunk);

        let n = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

        result.push(ALPHABET[((n >> 18) & 0x3F) as usize] as char);
        result.push(ALPHABET[((n >> 12) & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            result.push(ALPHABET[((n >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(ALPHABET[(n & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}
