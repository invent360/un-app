//! Schema API handlers
//!
//! Provides endpoints for managing content schemas.
//! Admin endpoints require HMAC authentication.

use actix_web::{HttpResponse, web};
use uno_api::auth::{verify_request, SignedRequest};
use crate::server::app::ServiceFactory;
use crate::types::{ContentSchema, SchemaListResponse};

/// Maximum request age in seconds (5 minutes)
const MAX_REQUEST_AGE_SECS: i64 = 300;

// ============================================
// PUBLIC ENDPOINTS
// ============================================

/// GET /api/v1/schemas
/// Get all available schemas (public)
pub async fn get_schemas(
    factory: Option<web::Data<ServiceFactory>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    match factory.schema_service.get_all_schemas().await {
        Ok(schemas) => HttpResponse::Ok().json(SchemaListResponse { schemas }),
        Err(e) => {
            tracing::error!("Failed to get schemas: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/schemas/{id}
/// Get a single schema by ID (public)
pub async fn get_schema(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    let schema_id = path.into_inner();

    match factory.schema_service.get_schema(&schema_id).await {
        Ok(Some(schema)) => HttpResponse::Ok().json(schema),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Schema '{}' not found", schema_id),
            "code": "NOT_FOUND"
        })),
        Err(e) => {
            tracing::error!("Failed to get schema: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

// ============================================
// ADMIN ENDPOINTS
// ============================================

/// POST /api/v1/admin/schemas
/// Create a new schema (admin only)
pub async fn create_schema(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<ContentSchema>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature
    if let Err(e) = verify_request(&body, &factory.client_registry, MAX_REQUEST_AGE_SECS) {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    match factory.schema_service.create_schema(body.payload.clone(), Some(&body.client_id)).await {
        Ok(schema) => HttpResponse::Created().json(serde_json::json!({
            "id": schema.id,
            "name": schema.name,
            "version": schema.version,
            "created_at": schema.created_at
        })),
        Err(e) => {
            tracing::error!("Failed to create schema: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "CREATE_ERROR"
            }))
        }
    }
}

/// PUT /api/v1/admin/schemas/{id}
/// Update an existing schema (admin only)
pub async fn update_schema(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    body: web::Json<SignedRequest<ContentSchema>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature
    if let Err(e) = verify_request(&body, &factory.client_registry, MAX_REQUEST_AGE_SECS) {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let schema_id = path.into_inner();

    match factory.schema_service.update_schema(&schema_id, body.payload.clone(), Some(&body.client_id)).await {
        Ok(schema) => HttpResponse::Ok().json(serde_json::json!({
            "id": schema.id,
            "name": schema.name,
            "version": schema.version,
            "updated_at": schema.updated_at
        })),
        Err(e) => {
            tracing::error!("Failed to update schema: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "UPDATE_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/schemas/{id}/delete
/// Delete a schema (admin only)
pub async fn delete_schema(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    body: web::Json<SignedRequest<serde_json::Value>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature
    if let Err(e) = verify_request(&body, &factory.client_registry, MAX_REQUEST_AGE_SECS) {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let schema_id = path.into_inner();

    match factory.schema_service.delete_schema(&schema_id, Some(&body.client_id)).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": format!("Schema '{}' deleted", schema_id)
        })),
        Err(e) => {
            tracing::error!("Failed to delete schema: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "DELETE_ERROR"
            }))
        }
    }
}
