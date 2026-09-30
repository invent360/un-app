//! Webhook adapter handlers (Phase 8)
//!
//! Provides endpoints for managing webhook endpoints, deliveries,
//! and inbound webhook processing.

use actix_web::{web, HttpRequest, HttpResponse, Responder, web::Bytes};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::repositories::{
    DynWebhookRepository, CreateEndpointInput, UpdateEndpointInput,
    CreateSourceConfigInput, WebhookAuthType,
};
use crate::server::services::DynWebhookService;
use crate::types::AppError;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct ListEndpointsQuery {
    pub enabled: Option<bool>,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEndpointRequest {
    pub name: String,
    pub endpoint_url: String,
    pub auth_type: String,
    pub auth_secret: Option<String>,
    pub event_types: Vec<String>,
    pub country_codes: Option<Vec<String>>,
    pub max_retries: Option<i32>,
    pub retry_delay_seconds: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub created_by: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEndpointRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub endpoint_url: Option<String>,
    pub auth_type: Option<String>,
    pub auth_secret: Option<String>,
    pub event_types: Option<Vec<String>>,
    pub country_codes: Option<Vec<String>>,
    pub max_retries: Option<i32>,
    pub retry_delay_seconds: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub updated_by: String,
}

#[derive(Debug, Deserialize)]
pub struct ToggleEndpointRequest {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct DeliveriesQuery {
    pub status: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSourceConfigRequest {
    pub source: String,
    pub display_name: Option<String>,
    pub secret_key: Option<String>,
    pub auto_process: Option<bool>,
    pub require_signature: Option<bool>,
    pub created_by: String,
}

#[derive(Debug, Serialize)]
pub struct EndpointResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub endpoint_url: String,
    pub auth_type: String,
    pub event_types: Vec<String>,
    pub country_codes: Option<Vec<String>>,
    pub enabled: bool,
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_success_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_failure_at: Option<chrono::DateTime<chrono::Utc>>,
    pub consecutive_failures: i32,
    pub circuit_open: bool,
    pub max_retries: i32,
    pub retry_delay_seconds: i32,
    pub timeout_seconds: i32,
    pub created_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct DeliveryResponse {
    pub id: Uuid,
    pub endpoint_id: Uuid,
    pub event_id: Uuid,
    pub event_type: String,
    pub status: String,
    pub attempt_count: i32,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub error_message: Option<String>,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub next_retry_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct InboundWebhookResponse {
    pub id: Uuid,
    pub source: String,
    pub event_type: String,
    pub idempotency_key: String,
    pub status: String,
    pub signature_valid: Option<bool>,
    pub scope_valid: Option<bool>,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
    pub received_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct SourceConfigResponse {
    pub id: Uuid,
    pub source: String,
    pub display_name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub auto_process: bool,
    pub require_signature: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct VerificationResponse {
    pub success: bool,
    pub message: Option<String>,
    pub response_time_ms: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct DeliveryStatsResponse {
    pub total_pending: i64,
    pub delivered: i64,
    pub failed: i64,
    pub dead_letter: i64,
}

#[derive(Debug, Serialize)]
pub struct ProcessingStatsResponse {
    pub processed: i64,
    pub rejected: i64,
    pub skipped: i64,
}

#[derive(Debug, Serialize)]
pub struct ReceiveWebhookResponse {
    pub id: Uuid,
    pub status: String,
}

// ============================================================================
// Admin Handlers
// ============================================================================

/// GET /api/v1/admin/webhooks/endpoints
/// List all webhook endpoints
pub async fn list_endpoints(
    service: web::Data<DynWebhookService>,
    _query: web::Query<ListEndpointsQuery>,
) -> Result<impl Responder, AppError> {
    let endpoints = service.list_endpoints().await?;

    let response: Vec<EndpointResponse> = endpoints
        .into_iter()
        .map(|e| EndpointResponse {
            id: e.id,
            name: e.name,
            description: e.description,
            endpoint_url: e.endpoint_url,
            auth_type: format!("{:?}", e.auth_type).to_lowercase(),
            event_types: e.event_types,
            country_codes: e.country_codes,
            enabled: e.enabled,
            verified_at: e.verified_at,
            last_success_at: e.last_success_at,
            last_failure_at: e.last_failure_at,
            consecutive_failures: e.consecutive_failures,
            circuit_open: e.circuit_open,
            max_retries: e.max_retries,
            retry_delay_seconds: e.retry_delay_seconds,
            timeout_seconds: e.timeout_seconds,
            created_by: e.created_by,
            created_at: e.created_at,
            updated_at: e.updated_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/admin/webhooks/endpoints
/// Create a new webhook endpoint
pub async fn create_endpoint(
    service: web::Data<DynWebhookService>,
    body: web::Json<CreateEndpointRequest>,
) -> Result<impl Responder, AppError> {
    let auth_type = match body.auth_type.to_lowercase().as_str() {
        "bearer" => WebhookAuthType::Bearer,
        "hmac" => WebhookAuthType::Hmac,
        "basic" => WebhookAuthType::Basic,
        "api_key" | "apikey" => WebhookAuthType::ApiKey,
        _ => WebhookAuthType::None,
    };

    let endpoint = service
        .create_endpoint(CreateEndpointInput {
            name: body.name.clone(),
            description: None,
            endpoint_url: body.endpoint_url.clone(),
            auth_type,
            auth_header_name: None,
            auth_secret: body.auth_secret.clone(),
            event_types: body.event_types.clone(),
            country_codes: body.country_codes.clone(),
            http_method: None,
            content_type: None,
            custom_headers: None,
            payload_template: None,
            max_retries: body.max_retries,
            retry_delay_seconds: body.retry_delay_seconds,
            timeout_seconds: body.timeout_seconds,
            rate_limit_per_minute: None,
            created_by: body.created_by.clone(),
        })
        .await?;

    Ok(HttpResponse::Created().json(EndpointResponse {
        id: endpoint.id,
        name: endpoint.name,
        description: endpoint.description,
        endpoint_url: endpoint.endpoint_url,
        auth_type: format!("{:?}", endpoint.auth_type).to_lowercase(),
        event_types: endpoint.event_types,
        country_codes: endpoint.country_codes,
        enabled: endpoint.enabled,
        verified_at: endpoint.verified_at,
        last_success_at: endpoint.last_success_at,
        last_failure_at: endpoint.last_failure_at,
        consecutive_failures: endpoint.consecutive_failures,
        circuit_open: endpoint.circuit_open,
        max_retries: endpoint.max_retries,
        retry_delay_seconds: endpoint.retry_delay_seconds,
        timeout_seconds: endpoint.timeout_seconds,
        created_by: endpoint.created_by,
        created_at: endpoint.created_at,
        updated_at: endpoint.updated_at,
    }))
}

/// GET /api/v1/admin/webhooks/endpoints/{id}
/// Get a specific webhook endpoint
pub async fn get_endpoint(
    service: web::Data<DynWebhookService>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let endpoint = service.get_endpoint(id).await?;

    match endpoint {
        Some(e) => Ok(HttpResponse::Ok().json(EndpointResponse {
            id: e.id,
            name: e.name,
            description: e.description,
            endpoint_url: e.endpoint_url,
            auth_type: format!("{:?}", e.auth_type).to_lowercase(),
            event_types: e.event_types,
            country_codes: e.country_codes,
            enabled: e.enabled,
            verified_at: e.verified_at,
            last_success_at: e.last_success_at,
            last_failure_at: e.last_failure_at,
            consecutive_failures: e.consecutive_failures,
            circuit_open: e.circuit_open,
            max_retries: e.max_retries,
            retry_delay_seconds: e.retry_delay_seconds,
            timeout_seconds: e.timeout_seconds,
            created_by: e.created_by,
            created_at: e.created_at,
            updated_at: e.updated_at,
        })),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

/// PUT /api/v1/admin/webhooks/endpoints/{id}
/// Update a webhook endpoint
pub async fn update_endpoint(
    service: web::Data<DynWebhookService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateEndpointRequest>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let auth_type = body.auth_type.as_ref().map(|at| match at.to_lowercase().as_str() {
        "bearer" => WebhookAuthType::Bearer,
        "hmac" => WebhookAuthType::Hmac,
        "basic" => WebhookAuthType::Basic,
        _ => WebhookAuthType::None,
    });

    let endpoint = service
        .update_endpoint(
            id,
            UpdateEndpointInput {
                name: body.name.clone(),
                description: body.description.clone(),
                endpoint_url: body.endpoint_url.clone(),
                auth_type,
                auth_secret: body.auth_secret.clone(),
                event_types: body.event_types.clone(),
                country_codes: body.country_codes.clone(),
                max_retries: body.max_retries,
                retry_delay_seconds: body.retry_delay_seconds,
                timeout_seconds: body.timeout_seconds,
                updated_by: body.updated_by.clone(),
            },
        )
        .await?;

    Ok(HttpResponse::Ok().json(EndpointResponse {
        id: endpoint.id,
        name: endpoint.name,
        description: endpoint.description,
        endpoint_url: endpoint.endpoint_url,
        auth_type: format!("{:?}", endpoint.auth_type).to_lowercase(),
        event_types: endpoint.event_types,
        country_codes: endpoint.country_codes,
        enabled: endpoint.enabled,
        verified_at: endpoint.verified_at,
        last_success_at: endpoint.last_success_at,
        last_failure_at: endpoint.last_failure_at,
        consecutive_failures: endpoint.consecutive_failures,
        circuit_open: endpoint.circuit_open,
        max_retries: endpoint.max_retries,
        retry_delay_seconds: endpoint.retry_delay_seconds,
        timeout_seconds: endpoint.timeout_seconds,
        created_by: endpoint.created_by,
        created_at: endpoint.created_at,
        updated_at: endpoint.updated_at,
    }))
}

/// DELETE /api/v1/admin/webhooks/endpoints/{id}
/// Delete a webhook endpoint
pub async fn delete_endpoint(
    service: web::Data<DynWebhookService>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    service.delete_endpoint(id).await?;
    Ok(HttpResponse::NoContent().finish())
}

/// POST /api/v1/admin/webhooks/endpoints/{id}/verify
/// Verify a webhook endpoint
pub async fn verify_endpoint(
    service: web::Data<DynWebhookService>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let result = service.verify_endpoint(id).await?;

    Ok(HttpResponse::Ok().json(VerificationResponse {
        success: result.success,
        message: result.message,
        response_time_ms: result.response_time_ms,
    }))
}

/// POST /api/v1/admin/webhooks/endpoints/{id}/toggle
/// Enable or disable a webhook endpoint
pub async fn toggle_endpoint(
    service: web::Data<DynWebhookService>,
    path: web::Path<Uuid>,
    body: web::Json<ToggleEndpointRequest>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let endpoint = service.toggle_endpoint(id, body.enabled).await?;

    Ok(HttpResponse::Ok().json(EndpointResponse {
        id: endpoint.id,
        name: endpoint.name,
        description: endpoint.description,
        endpoint_url: endpoint.endpoint_url,
        auth_type: format!("{:?}", endpoint.auth_type).to_lowercase(),
        event_types: endpoint.event_types,
        country_codes: endpoint.country_codes,
        enabled: endpoint.enabled,
        verified_at: endpoint.verified_at,
        last_success_at: endpoint.last_success_at,
        last_failure_at: endpoint.last_failure_at,
        consecutive_failures: endpoint.consecutive_failures,
        circuit_open: endpoint.circuit_open,
        max_retries: endpoint.max_retries,
        retry_delay_seconds: endpoint.retry_delay_seconds,
        timeout_seconds: endpoint.timeout_seconds,
        created_by: endpoint.created_by,
        created_at: endpoint.created_at,
        updated_at: endpoint.updated_at,
    }))
}

// NOTE: reset_circuit_breaker not yet implemented in service

/// GET /api/v1/admin/webhooks/endpoints/{id}/deliveries
/// Get delivery history for an endpoint
pub async fn get_deliveries(
    service: web::Data<DynWebhookService>,
    path: web::Path<Uuid>,
    query: web::Query<DeliveriesQuery>,
) -> Result<impl Responder, AppError> {
    let endpoint_id = path.into_inner();
    let limit = query.limit.unwrap_or(50);
    let deliveries = service.get_endpoint_deliveries(endpoint_id, limit).await?;

    let response: Vec<DeliveryResponse> = deliveries
        .into_iter()
        .map(|d| DeliveryResponse {
            id: d.id,
            endpoint_id: d.endpoint_id,
            event_id: d.event_id,
            event_type: d.event_type,
            status: format!("{:?}", d.status).to_lowercase(),
            attempt_count: d.attempt_count,
            response_status: d.response_status,
            response_body: d.response_body,
            error_message: d.error_message,
            scheduled_at: d.scheduled_at,
            completed_at: d.completed_at,
            next_retry_at: d.next_retry_at,
            created_at: d.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/admin/webhooks/process
/// Process pending webhook deliveries
pub async fn process_deliveries(
    service: web::Data<DynWebhookService>,
) -> Result<impl Responder, AppError> {
    let stats = service.process_pending_deliveries(100).await?;

    Ok(HttpResponse::Ok().json(DeliveryStatsResponse {
        total_pending: stats.total_pending,
        delivered: stats.delivered,
        failed: stats.failed,
        dead_letter: stats.dead_letter,
    }))
}

/// POST /api/v1/admin/webhooks/retry
/// Retry failed webhook deliveries
pub async fn retry_deliveries(
    service: web::Data<DynWebhookService>,
) -> Result<impl Responder, AppError> {
    let count = service.retry_failed_deliveries(100).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "retried": count })))
}

// NOTE: list_inbound_webhooks not yet implemented in service

/// POST /api/v1/admin/webhooks/inbound/process
/// Process pending inbound webhooks
pub async fn process_inbound(
    service: web::Data<DynWebhookService>,
) -> Result<impl Responder, AppError> {
    let stats = service.process_inbound_webhooks(100).await?;

    Ok(HttpResponse::Ok().json(ProcessingStatsResponse {
        processed: stats.processed,
        rejected: stats.rejected,
        skipped: stats.skipped,
    }))
}

/// GET /api/v1/admin/webhooks/sources
/// List source configurations
pub async fn list_sources(
    service: web::Data<DynWebhookService>,
) -> Result<impl Responder, AppError> {
    let sources = service.list_source_configs().await?;

    let response: Vec<SourceConfigResponse> = sources
        .into_iter()
        .map(|s| SourceConfigResponse {
            id: s.id,
            source: s.source,
            display_name: s.display_name,
            description: s.description,
            enabled: s.enabled,
            auto_process: s.auto_process,
            require_signature: s.require_signature,
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/admin/webhooks/sources
/// Create a source configuration
pub async fn create_source(
    service: web::Data<DynWebhookService>,
    body: web::Json<CreateSourceConfigRequest>,
) -> Result<impl Responder, AppError> {
    let source = service
        .create_source_config(CreateSourceConfigInput {
            source: body.source.clone(),
            display_name: body.display_name.clone().unwrap_or_else(|| body.source.clone()),
            description: None,
            signing_secret: body.secret_key.clone(),
            signature_header: None,
            signature_algorithm: None,
            allowed_ips: None,
            event_type_path: None,
            idempotency_key_path: None,
            auto_process: body.auto_process.unwrap_or(false),
            require_signature: body.require_signature.unwrap_or(false),
            rate_limit_per_minute: None,
            created_by: body.created_by.clone(),
        })
        .await?;

    Ok(HttpResponse::Created().json(SourceConfigResponse {
        id: source.id,
        source: source.source,
        display_name: source.display_name,
        description: source.description,
        enabled: source.enabled,
        auto_process: source.auto_process,
        require_signature: source.require_signature,
        created_at: source.created_at,
        updated_at: source.updated_at,
    }))
}

// ============================================================================
// Inbound Webhook Handlers (Public)
// ============================================================================

/// POST /api/v1/webhooks/inbound/{source}
/// Receive an inbound webhook from an external source
pub async fn receive_webhook(
    service: web::Data<DynWebhookService>,
    path: web::Path<String>,
    req: HttpRequest,
    body_bytes: Bytes,
) -> Result<impl Responder, AppError> {
    let source = path.into_inner();
    let body_str = String::from_utf8_lossy(&body_bytes).to_string();

    // Parse JSON payload
    let payload: serde_json::Value = serde_json::from_str(&body_str)
        .unwrap_or_else(|_| serde_json::json!({}));

    // Extract event type from payload (common paths)
    let event_type = payload["type"].as_str()
        .or_else(|| payload["event_type"].as_str())
        .or_else(|| payload["event"].as_str())
        .unwrap_or("unknown");

    // Extract idempotency key from payload or headers
    let idempotency_key = payload["id"].as_str()
        .or_else(|| payload["event_id"].as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    // Extract signature from headers
    let signature = req
        .headers()
        .get("X-Webhook-Signature")
        .or_else(|| req.headers().get("X-Signature"))
        .or_else(|| req.headers().get("X-Hub-Signature-256"))
        .and_then(|v| v.to_str().ok());

    // Collect headers as JSON
    let headers: serde_json::Value = req
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    // Get source IP
    let source_ip = req.connection_info().peer_addr().map(|s| s.to_string());

    let id = service
        .receive_webhook(
            &source,
            event_type,
            &idempotency_key,
            headers,
            &body_str,
            signature,
            source_ip.as_deref(),
        )
        .await?;

    Ok(HttpResponse::Accepted().json(ReceiveWebhookResponse {
        id,
        status: "pending".to_string(),
    }))
}
