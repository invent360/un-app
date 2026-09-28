//! Admin content API handlers with HMAC authentication

use actix_web::{HttpResponse, web};

use uno_api::auth::{verify_request, SignedRequest};

use crate::server::app::ServiceFactory;
use crate::types::{
    UpsertContentRequest, ContentListParams, PublishContentRequest, RevertContentRequest,
    UpdateScheduleRequest,
};

/// Maximum request age in seconds (5 minutes)
const MAX_REQUEST_AGE_SECS: i64 = 300;

/// POST /api/v1/admin/contents
/// Create new content
pub async fn create_content(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<UpsertContentRequest>>,
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

    match factory.content_service.create_content(body.payload.clone(), Some(&body.client_id)).await {
        Ok(content) => HttpResponse::Created().json(serde_json::json!({
            "id": content.id,
            "content_type": content.content_type,
            "slug": content.slug,
            "status": content.status,
            "version": content.version,
            "created_at": content.created_at
        })),
        Err(e) => {
            tracing::error!("Failed to create content: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "CREATE_ERROR"
            }))
        }
    }
}

/// PUT /api/v1/admin/contents/{id}
/// Update existing content
pub async fn update_content(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    body: web::Json<SignedRequest<UpsertContentRequest>>,
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

    let id = path.into_inner();

    match factory.content_service.update_content(id, body.payload.clone(), Some(&body.client_id)).await {
        Ok(content) => HttpResponse::Ok().json(serde_json::json!({
            "id": content.id,
            "content_type": content.content_type,
            "slug": content.slug,
            "status": content.status,
            "version": content.version,
            "updated_at": content.updated_at
        })),
        Err(e) => {
            tracing::error!("Failed to update content: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "UPDATE_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/contents/list
/// List all content with filters
pub async fn list_contents(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<ContentListParams>>,
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

    match factory.content_service.list_contents(body.payload.clone()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            tracing::error!("Failed to list contents: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/contents/{id}/get
/// Get single content detail
pub async fn get_content(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
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

    let id = path.into_inner();

    match factory.content_service.get_content(id).await {
        Ok(Some(content)) => HttpResponse::Ok().json(content),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Content not found",
            "code": "NOT_FOUND"
        })),
        Err(e) => {
            tracing::error!("Failed to get content: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/contents/{id}/delete
/// Delete content (soft delete)
pub async fn delete_content(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
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

    let id = path.into_inner();

    match factory.content_service.delete_content(id).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Content deleted"
        })),
        Err(e) => {
            tracing::error!("Failed to delete content: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/contents/{id}/publish
/// Publish content
pub async fn publish_content(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    body: web::Json<SignedRequest<PublishContentRequest>>,
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

    let id = path.into_inner();
    let published_by = body.payload.published_by.as_deref().or(Some(&body.client_id));

    match factory.content_service.publish_content(id, published_by).await {
        Ok(content) => HttpResponse::Ok().json(serde_json::json!({
            "id": content.id,
            "status": content.status,
            "version": content.version,
            "published_version": content.published_version,
            "published_at": content.published_at
        })),
        Err(e) => {
            tracing::error!("Failed to publish content: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/contents/{id}/archive
/// Archive content
pub async fn archive_content(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
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

    let id = path.into_inner();

    match factory.content_service.archive_content(id).await {
        Ok(content) => HttpResponse::Ok().json(serde_json::json!({
            "id": content.id,
            "status": content.status
        })),
        Err(e) => {
            tracing::error!("Failed to archive content: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/contents/{id}/revert
/// Revert content to version
pub async fn revert_content(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    body: web::Json<SignedRequest<RevertContentRequest>>,
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

    let id = path.into_inner();
    let reverted_by = body.payload.reverted_by.as_deref().or(Some(&body.client_id));

    match factory.content_service.revert_content(id, body.payload.version, reverted_by).await {
        Ok(content) => HttpResponse::Ok().json(serde_json::json!({
            "id": content.id,
            "status": content.status,
            "version": content.version,
            "reverted_to": body.payload.version
        })),
        Err(e) => {
            tracing::error!("Failed to revert content: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// PUT /api/v1/admin/contents/{id}/schedule
/// Update content schedule (publish_at, unpublish_at)
pub async fn update_content_schedule(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    body: web::Json<SignedRequest<UpdateScheduleRequest>>,
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

    let id = path.into_inner();

    match factory.content_service.update_schedule_with_actor(
        id,
        body.payload.publish_at,
        body.payload.unpublish_at,
        Some(&body.client_id),
    ).await {
        Ok(content) => HttpResponse::Ok().json(serde_json::json!({
            "id": content.id,
            "publish_at": content.publish_at,
            "unpublish_at": content.unpublish_at,
            "updated_at": content.updated_at
        })),
        Err(e) => {
            tracing::error!("Failed to update content schedule: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "SCHEDULE_ERROR"
            }))
        }
    }
}

/// Configure admin content API routes
pub fn configure_content_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin/contents")
            .route("", web::post().to(create_content))
            .route("/list", web::post().to(list_contents))
            .route("/{id}", web::put().to(update_content))
            .route("/{id}/get", web::post().to(get_content))
            .route("/{id}/delete", web::post().to(delete_content))
            .route("/{id}/publish", web::post().to(publish_content))
            .route("/{id}/archive", web::post().to(archive_content))
            .route("/{id}/revert", web::post().to(revert_content))
            .route("/{id}/schedule", web::put().to(update_content_schedule))
    );
}
