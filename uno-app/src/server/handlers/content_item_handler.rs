//! Content Item API handlers
//!
//! Provides endpoints for managing generic content items.
//! Admin endpoints require HMAC authentication.

use actix_web::{HttpResponse, web};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uno_api::auth::{verify_request_with_replay_protection_async, SignedRequest};
use crate::server::app::ServiceFactory;
use crate::types::{
    ContentItemListParams, UpsertContentItemRequest, ContentItemStatus,
};

/// Maximum request age in seconds (5 minutes)
const MAX_REQUEST_AGE_SECS: i64 = 300;

// ============================================
// PUBLIC ENDPOINTS
// ============================================

/// GET /api/v1/items/{schema_id}
/// Get all published content items for a schema (public)
pub async fn get_items_by_schema(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    query: web::Query<PublicListQuery>,
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
    let locale = query.locale.as_deref().unwrap_or("en");

    match factory.content_item_service.get_published_by_schema(&schema_id, locale).await {
        Ok(items) => HttpResponse::Ok().json(serde_json::json!({
            "items": items,
            "schema_id": schema_id,
            "locale": locale
        })),
        Err(e) => {
            tracing::error!("Failed to get items for schema {}: {}", schema_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PublicListQuery {
    pub locale: Option<String>,
}

/// GET /api/v1/items/{schema_id}/{slug}
/// Get a single published content item by slug (public)
pub async fn get_item_by_slug(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<(String, String)>,
    query: web::Query<PublicListQuery>,
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

    let (schema_id, slug) = path.into_inner();
    let locale = query.locale.as_deref().unwrap_or("en");

    match factory.content_item_service.get_published_by_slug(&schema_id, &slug, locale).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Item with slug '{}' not found", slug),
            "code": "NOT_FOUND"
        })),
        Err(e) => {
            tracing::error!("Failed to get item by slug: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/items/{schema_id}/featured
/// Get featured content items for a schema (public)
pub async fn get_featured_items(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    query: web::Query<PublicListQuery>,
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
    let locale = query.locale.as_deref().unwrap_or("en");

    match factory.content_item_service.get_featured(&schema_id, locale).await {
        Ok(items) => HttpResponse::Ok().json(serde_json::json!({
            "items": items,
            "schema_id": schema_id,
            "locale": locale
        })),
        Err(e) => {
            tracing::error!("Failed to get featured items: {}", e);
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

/// POST /api/v1/admin/items/list
/// List all content items with filters (admin)
pub async fn list_items(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<ContentItemListParams>>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    match factory.content_item_service.list(&body.payload).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            tracing::error!("Failed to list items: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/items
/// Create a new content item (admin)
pub async fn create_item(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<UpsertContentItemRequest>>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    match factory.content_item_service.create(body.payload.clone(), Some(&body.client_id)).await {
        Ok(item) => HttpResponse::Created().json(serde_json::json!({
            "id": item.id,
            "schema_id": item.schema_id,
            "slug": item.slug,
            "status": item.status,
            "version": item.version,
            "created_at": item.created_at
        })),
        Err(e) => {
            tracing::error!("Failed to create item: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "CREATE_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/items/{id}/get
/// Get a single content item detail (admin)
pub async fn get_item(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<Uuid>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let id = path.into_inner();

    match factory.content_item_service.get_detail(id).await {
        Ok(Some(detail)) => HttpResponse::Ok().json(detail),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Content item not found",
            "code": "NOT_FOUND"
        })),
        Err(e) => {
            tracing::error!("Failed to get item: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// PUT /api/v1/admin/items/{id}
/// Update an existing content item (admin)
pub async fn update_item(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<Uuid>,
    body: web::Json<SignedRequest<UpsertContentItemRequest>>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let id = path.into_inner();

    match factory.content_item_service.update(id, body.payload.clone(), Some(&body.client_id)).await {
        Ok(item) => HttpResponse::Ok().json(serde_json::json!({
            "id": item.id,
            "schema_id": item.schema_id,
            "slug": item.slug,
            "status": item.status,
            "version": item.version,
            "updated_at": item.updated_at
        })),
        Err(e) => {
            tracing::error!("Failed to update item: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "UPDATE_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/items/{id}/delete
/// Delete a content item (admin)
pub async fn delete_item(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<Uuid>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let id = path.into_inner();

    match factory.content_item_service.delete(id, Some(&body.client_id)).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Content item deleted"
        })),
        Err(e) => {
            tracing::error!("Failed to delete item: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "DELETE_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/items/{id}/publish
/// Publish a content item (admin)
pub async fn publish_item(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<Uuid>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let id = path.into_inner();

    match factory.content_item_service.publish(id, Some(&body.client_id)).await {
        Ok(item) => HttpResponse::Ok().json(serde_json::json!({
            "id": item.id,
            "status": item.status,
            "version": item.version,
            "published_version": item.published_version,
            "published_at": item.published_at
        })),
        Err(e) => {
            tracing::error!("Failed to publish item: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "PUBLISH_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/items/{id}/archive
/// Archive a content item (admin)
pub async fn archive_item(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<Uuid>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let id = path.into_inner();

    match factory.content_item_service.archive(id, Some(&body.client_id)).await {
        Ok(item) => HttpResponse::Ok().json(serde_json::json!({
            "id": item.id,
            "status": item.status
        })),
        Err(e) => {
            tracing::error!("Failed to archive item: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "ARCHIVE_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/items/{id}/versions
/// Get version history for a content item (admin)
pub async fn get_item_versions(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<Uuid>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let id = path.into_inner();

    match factory.content_item_service.get_versions(id).await {
        Ok(versions) => HttpResponse::Ok().json(serde_json::json!({
            "content_id": id,
            "versions": versions
        })),
        Err(e) => {
            tracing::error!("Failed to get versions: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// Revert request payload
#[derive(Debug, Serialize, Deserialize)]
pub struct RevertItemRequest {
    pub version: i32,
}

/// POST /api/v1/admin/items/{id}/revert
/// Revert a content item to a previous version (admin)
pub async fn revert_item(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<Uuid>,
    body: web::Json<SignedRequest<RevertItemRequest>>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let id = path.into_inner();

    match factory.content_item_service.revert_to_version(id, body.payload.version, Some(&body.client_id)).await {
        Ok(item) => HttpResponse::Ok().json(serde_json::json!({
            "id": item.id,
            "status": item.status,
            "version": item.version,
            "reverted_to": body.payload.version
        })),
        Err(e) => {
            tracing::error!("Failed to revert item: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "REVERT_ERROR"
            }))
        }
    }
}

/// Schedule request payload
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleItemRequest {
    pub publish_at: Option<DateTime<Utc>>,
    pub unpublish_at: Option<DateTime<Utc>>,
}

/// PUT /api/v1/admin/items/{id}/schedule
/// Update content item schedule (admin)
pub async fn update_item_schedule(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<Uuid>,
    body: web::Json<SignedRequest<ScheduleItemRequest>>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let id = path.into_inner();

    match factory.content_item_service.update_schedule(
        id,
        body.payload.publish_at,
        body.payload.unpublish_at,
        Some(&body.client_id),
    ).await {
        Ok(item) => HttpResponse::Ok().json(serde_json::json!({
            "id": item.id,
            "status": item.status,
            "publish_at": item.publish_at,
            "unpublish_at": item.unpublish_at,
            "updated_at": item.updated_at
        })),
        Err(e) => {
            tracing::error!("Failed to update schedule: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "SCHEDULE_ERROR"
            }))
        }
    }
}

/// Translation update request
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTranslationRequest {
    pub locale: String,
    pub data: serde_json::Value,
    pub status: Option<String>,
}

/// PUT /api/v1/admin/items/{id}/translations
/// Update translations for a content item (admin)
pub async fn update_item_translation(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<Uuid>,
    body: web::Json<SignedRequest<UpdateTranslationRequest>>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let id = path.into_inner();

    match factory.content_item_service.update_translation(
        id,
        &body.payload.locale,
        body.payload.data.clone(),
        body.payload.status.as_deref(),
        Some(&body.client_id),
    ).await {
        Ok(item) => HttpResponse::Ok().json(serde_json::json!({
            "id": item.id,
            "locale": body.payload.locale,
            "translation_status": item.translation_status,
            "updated_at": item.updated_at
        })),
        Err(e) => {
            tracing::error!("Failed to update translation: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "TRANSLATION_ERROR"
            }))
        }
    }
}

/// Bulk status update request
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkUpdateStatusRequest {
    pub ids: Vec<Uuid>,
    pub status: ContentItemStatus,
}

/// POST /api/v1/admin/items/bulk/status
/// Update status for multiple content items (admin)
pub async fn bulk_update_status(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<BulkUpdateStatusRequest>>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let mut successes = Vec::new();
    let mut failures = Vec::new();

    for id in &body.payload.ids {
        let result = match body.payload.status {
            ContentItemStatus::Published => {
                factory.content_item_service.publish(*id, Some(&body.client_id)).await
            }
            ContentItemStatus::Archived => {
                factory.content_item_service.archive(*id, Some(&body.client_id)).await
            }
            _ => {
                failures.push(serde_json::json!({
                    "id": id,
                    "error": format!("Cannot bulk update to status {:?}", body.payload.status)
                }));
                continue;
            }
        };

        match result {
            Ok(_) => successes.push(*id),
            Err(e) => failures.push(serde_json::json!({
                "id": id,
                "error": e.to_string()
            })),
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "success_count": successes.len(),
        "failure_count": failures.len(),
        "successes": successes,
        "failures": failures
    }))
}

/// Reorder request
#[derive(Debug, Serialize, Deserialize)]
pub struct ReorderItemsRequest {
    pub schema_id: String,
    pub order: Vec<Uuid>,
}

/// POST /api/v1/admin/items/reorder
/// Reorder content items (admin)
pub async fn reorder_items(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<ReorderItemsRequest>>,
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

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    match factory.content_item_service.reorder(&body.payload.schema_id, &body.payload.order, Some(&body.client_id)).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Items reordered"
        })),
        Err(e) => {
            tracing::error!("Failed to reorder items: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "REORDER_ERROR"
            }))
        }
    }
}
