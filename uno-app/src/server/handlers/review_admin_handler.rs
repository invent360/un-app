//! Admin Review API handlers with HMAC authentication
//!
//! REST API endpoints for CMS review workflow operations.
//! These endpoints mirror the Leptos server functions in api/cms_review.rs
//! but are accessible via standard HTTP/JSON with HMAC authentication.

use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use uno_api::auth::{verify_request, SignedRequest};

use crate::server::app::ServiceFactory;
use crate::types::{
    SubmitReviewRequest, ReviewDecisionRequest, ReviewDecision,
    CreatePreviewTokenRequest, DirectPublishRequest, PublishRequest,
    PublishResult, PublishError,
};

/// Maximum request age in seconds (5 minutes)
const MAX_REQUEST_AGE_SECS: i64 = 300;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to get pending reviews
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPendingReviewsRequest {
    #[serde(default)]
    pub limit: Option<i32>,
}

/// Request to get submissions by user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMySubmissionsRequest {
    pub submitter: String,
    #[serde(default)]
    pub limit: Option<i32>,
}

/// Request to get version history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionHistoryRequest {
    #[serde(default)]
    pub limit: Option<i32>,
}

/// Request to compare versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareVersionsRequest {
    pub version_a: i32,
    pub version_b: i32,
}

/// Request to revert to a version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevertToVersionRequest {
    pub version: i32,
    pub reverted_by: String,
}

// ============================================================================
// Review Workflow Handlers
// ============================================================================

/// POST /api/v1/admin/reviews/submit
/// Submit content version for review
pub async fn submit_for_review(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<SubmitReviewRequest>>,
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

    match factory.review_repository.submit_for_review(&body.payload).await {
        Ok(review) => HttpResponse::Created().json(review),
        Err(e) => {
            tracing::error!("Failed to submit for review: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "SUBMIT_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/reviews/{id}/approve
/// Approve a content review
pub async fn approve_review(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    body: web::Json<SignedRequest<ReviewDecisionPayload>>,
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

    let review_id = path.into_inner();
    let request = ReviewDecisionRequest {
        review_id,
        reviewed_by: body.payload.reviewed_by.clone(),
        decision: ReviewDecision::Approve,
        notes: body.payload.notes.clone(),
    };

    match factory.review_repository.process_decision(&request).await {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(e) => {
            tracing::error!("Failed to approve review: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "APPROVE_ERROR"
            }))
        }
    }
}

/// Payload for review decision (approve/reject/request-changes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewDecisionPayload {
    pub reviewed_by: String,
    #[serde(default)]
    pub notes: Option<String>,
}

/// POST /api/v1/admin/reviews/{id}/request-changes
/// Request changes on a review
pub async fn request_changes(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    body: web::Json<SignedRequest<ReviewDecisionPayload>>,
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

    // Notes are required for request changes
    let notes = match &body.payload.notes {
        Some(n) if !n.trim().is_empty() => n.clone(),
        _ => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Notes are required when requesting changes",
                "code": "VALIDATION_ERROR"
            }));
        }
    };

    let review_id = path.into_inner();
    let request = ReviewDecisionRequest {
        review_id,
        reviewed_by: body.payload.reviewed_by.clone(),
        decision: ReviewDecision::RequestChanges,
        notes: Some(notes),
    };

    match factory.review_repository.process_decision(&request).await {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(e) => {
            tracing::error!("Failed to request changes: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "REQUEST_CHANGES_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/reviews/{id}/reject
/// Reject a review
pub async fn reject_review(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    body: web::Json<SignedRequest<ReviewDecisionPayload>>,
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

    let review_id = path.into_inner();
    let request = ReviewDecisionRequest {
        review_id,
        reviewed_by: body.payload.reviewed_by.clone(),
        decision: ReviewDecision::Reject,
        notes: body.payload.notes.clone(),
    };

    match factory.review_repository.process_decision(&request).await {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(e) => {
            tracing::error!("Failed to reject review: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "REJECT_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/reviews/pending
/// Get pending reviews
pub async fn get_pending_reviews(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<GetPendingReviewsRequest>>,
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

    match factory.review_repository.get_pending_reviews(body.payload.limit).await {
        Ok(reviews) => {
            let total = reviews.len() as i64;
            HttpResponse::Ok().json(serde_json::json!({
                "reviews": reviews,
                "total": total
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get pending reviews: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/reviews/my-submissions
/// Get reviews submitted by a user
pub async fn get_my_submissions(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<GetMySubmissionsRequest>>,
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

    match factory.review_repository
        .get_reviews_by_submitter(&body.payload.submitter, body.payload.limit)
        .await
    {
        Ok(reviews) => HttpResponse::Ok().json(reviews),
        Err(e) => {
            tracing::error!("Failed to get submissions: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

// ============================================================================
// Preview Token Handlers
// ============================================================================

/// POST /api/v1/admin/preview-tokens
/// Create a preview token
pub async fn create_preview_token(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<CreatePreviewTokenRequest>>,
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

    let req = &body.payload;
    match factory.review_repository
        .create_preview_token(req.version_id, &req.created_by, req.expires_in_hours)
        .await
    {
        Ok(token) => HttpResponse::Created().json(token),
        Err(e) => {
            tracing::error!("Failed to create preview token: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/preview/{token}
/// Get preview content by token (public endpoint, no HMAC required)
pub async fn get_preview_content(
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

    let token = path.into_inner();

    // Validate the token
    match factory.review_repository.validate_preview_token(&token).await {
        Ok(Some(preview_token)) => {
            // Get the version content
            match factory.content_service.get_version_by_id(preview_token.version_id).await {
                Ok(Some(version)) => HttpResponse::Ok().json(version),
                Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Content version not found",
                    "code": "NOT_FOUND"
                })),
                Err(e) => {
                    tracing::error!("Failed to get preview content: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": e.to_string(),
                        "code": "INTERNAL_ERROR"
                    }))
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Invalid or expired preview token",
            "code": "INVALID_TOKEN"
        })),
        Err(e) => {
            tracing::error!("Failed to validate preview token: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

// ============================================================================
// Publish Handlers
// ============================================================================

/// POST /api/v1/admin/publish/direct
/// Publish content directly (skip review)
pub async fn publish_direct(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<DirectPublishRequest>>,
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

    let req = &body.payload;

    // Get the content to verify it exists
    match factory.content_service.get_by_id(req.content_id).await {
        Ok(Some(_)) => {
            // Publish the content
            match factory.content_service.publish(req.content_id, Some(&req.published_by)).await {
                Ok(_) => {
                    if let Some(msg) = &req.commit_message {
                        tracing::info!("Direct publish of content {} by {}: {}", req.content_id, req.published_by, msg);
                    }
                    HttpResponse::Ok().json(PublishResult {
                        success: true,
                        published_count: 1,
                        failed_count: 0,
                        errors: vec![],
                    })
                }
                Err(e) => {
                    tracing::error!("Failed to publish content: {}", e);
                    HttpResponse::Ok().json(PublishResult {
                        success: false,
                        published_count: 0,
                        failed_count: 1,
                        errors: vec![PublishError {
                            content_id: req.content_id,
                            error: e.to_string(),
                        }],
                    })
                }
            }
        }
        Ok(None) => {
            HttpResponse::Ok().json(PublishResult {
                success: false,
                published_count: 0,
                failed_count: 1,
                errors: vec![PublishError {
                    content_id: req.content_id,
                    error: "Content not found".to_string(),
                }],
            })
        }
        Err(e) => {
            tracing::error!("Failed to get content: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/publish/batch
/// Publish multiple approved content items
pub async fn publish_batch(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<PublishRequest>>,
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

    let req = &body.payload;
    let mut published_count = 0;
    let mut failed_count = 0;
    let mut errors = Vec::new();

    for content_id in &req.content_ids {
        match factory.content_service.get_by_id(*content_id).await {
            Ok(Some(content)) => {
                // Check if approved or draft (direct publish allowed)
                if content.status != "approved" && content.status != "draft" {
                    errors.push(PublishError {
                        content_id: *content_id,
                        error: format!("Content status is '{}', expected 'approved' or 'draft'", content.status),
                    });
                    failed_count += 1;
                    continue;
                }

                match factory.content_service.publish(*content_id, Some(&req.published_by)).await {
                    Ok(_) => published_count += 1,
                    Err(e) => {
                        errors.push(PublishError {
                            content_id: *content_id,
                            error: e.to_string(),
                        });
                        failed_count += 1;
                    }
                }
            }
            Ok(None) => {
                errors.push(PublishError {
                    content_id: *content_id,
                    error: "Content not found".to_string(),
                });
                failed_count += 1;
            }
            Err(e) => {
                errors.push(PublishError {
                    content_id: *content_id,
                    error: e.to_string(),
                });
                failed_count += 1;
            }
        }
    }

    HttpResponse::Ok().json(PublishResult {
        success: failed_count == 0,
        published_count,
        failed_count,
        errors,
    })
}

// ============================================================================
// Version History Handlers
// ============================================================================

/// POST /api/v1/admin/versions/{content_id}/history
/// Get version history for content
pub async fn get_version_history(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    body: web::Json<SignedRequest<GetVersionHistoryRequest>>,
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

    let content_id = path.into_inner();

    match factory.content_service.get_versions(content_id).await {
        Ok(mut versions) => {
            if let Some(limit) = body.payload.limit {
                versions.truncate(limit as usize);
            }
            HttpResponse::Ok().json(versions)
        }
        Err(e) => {
            tracing::error!("Failed to get version history: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/versions/{content_id}/compare
/// Compare two versions
pub async fn compare_versions(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    body: web::Json<SignedRequest<CompareVersionsRequest>>,
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

    let content_id = path.into_inner();
    let req = &body.payload;

    // Get both versions
    let ver_a = match factory.content_service.get_version(content_id, req.version_a).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": format!("Version {} not found", req.version_a),
                "code": "NOT_FOUND"
            }));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }));
        }
    };

    let ver_b = match factory.content_service.get_version(content_id, req.version_b).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": format!("Version {} not found", req.version_b),
                "code": "NOT_FOUND"
            }));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }));
        }
    };

    // Compute diff
    let changes = compute_diff(&ver_a.content, &ver_b.content, "en");
    let total_changes = changes.len() as i32;

    HttpResponse::Ok().json(serde_json::json!({
        "version_a": req.version_a,
        "version_b": req.version_b,
        "content_id": content_id,
        "changes": changes,
        "total_changes": total_changes
    }))
}

/// POST /api/v1/admin/versions/{content_id}/revert
/// Revert content to a previous version
pub async fn revert_to_version(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    body: web::Json<SignedRequest<RevertToVersionRequest>>,
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

    let content_id = path.into_inner();
    let req = &body.payload;

    match factory.content_service
        .revert_to_version(content_id, req.version, Some(&req.reverted_by))
        .await
    {
        Ok(content) => {
            // Get the new version that was created
            match factory.content_service.get_versions(content_id).await {
                Ok(versions) => {
                    if let Some(version) = versions.first() {
                        HttpResponse::Ok().json(version)
                    } else {
                        HttpResponse::Ok().json(content)
                    }
                }
                Err(_) => HttpResponse::Ok().json(content),
            }
        }
        Err(e) => {
            tracing::error!("Failed to revert to version: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

use crate::types::{ContentChange, ChangeType};

/// Compute diff between two JSON values
fn compute_diff(a: &serde_json::Value, b: &serde_json::Value, locale: &str) -> Vec<ContentChange> {
    let mut changes = Vec::new();

    match (a.as_object(), b.as_object()) {
        (Some(obj_a), Some(obj_b)) => {
            let mut all_keys: std::collections::HashSet<&String> = obj_a.keys().collect();
            all_keys.extend(obj_b.keys());

            for key in all_keys {
                let val_a = obj_a.get(key);
                let val_b = obj_b.get(key);

                match (val_a, val_b) {
                    (None, Some(new)) => {
                        changes.push(ContentChange {
                            field: key.clone(),
                            locale: locale.to_string(),
                            old_value: None,
                            new_value: Some(value_to_string(new)),
                            change_type: ChangeType::Added,
                        });
                    }
                    (Some(old), None) => {
                        changes.push(ContentChange {
                            field: key.clone(),
                            locale: locale.to_string(),
                            old_value: Some(value_to_string(old)),
                            new_value: None,
                            change_type: ChangeType::Removed,
                        });
                    }
                    (Some(old), Some(new)) if old != new => {
                        changes.push(ContentChange {
                            field: key.clone(),
                            locale: locale.to_string(),
                            old_value: Some(value_to_string(old)),
                            new_value: Some(value_to_string(new)),
                            change_type: ChangeType::Modified,
                        });
                    }
                    _ => {}
                }
            }
        }
        _ => {
            if a != b {
                changes.push(ContentChange {
                    field: "content".to_string(),
                    locale: locale.to_string(),
                    old_value: Some(value_to_string(a)),
                    new_value: Some(value_to_string(b)),
                    change_type: ChangeType::Modified,
                });
            }
        }
    }

    changes
}

/// Convert JSON value to display string
fn value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => "null".to_string(),
        _ => value.to_string(),
    }
}

// ============================================================================
// Route Configuration
// ============================================================================

/// Configure admin review API routes
pub fn configure_review_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin")
            // Review workflow routes
            .route("/reviews/submit", web::post().to(submit_for_review))
            .route("/reviews/pending", web::post().to(get_pending_reviews))
            .route("/reviews/my-submissions", web::post().to(get_my_submissions))
            .route("/reviews/{id}/approve", web::post().to(approve_review))
            .route("/reviews/{id}/request-changes", web::post().to(request_changes))
            .route("/reviews/{id}/reject", web::post().to(reject_review))
            // Preview tokens
            .route("/preview-tokens", web::post().to(create_preview_token))
            // Publish routes
            .route("/publish/direct", web::post().to(publish_direct))
            .route("/publish/batch", web::post().to(publish_batch))
            // Version history routes
            .route("/versions/{content_id}/history", web::post().to(get_version_history))
            .route("/versions/{content_id}/compare", web::post().to(compare_versions))
            .route("/versions/{content_id}/revert", web::post().to(revert_to_version))
    );

    // Public preview endpoint (no HMAC required)
    cfg.route("/api/v1/preview/{token}", web::get().to(get_preview_content));
}
