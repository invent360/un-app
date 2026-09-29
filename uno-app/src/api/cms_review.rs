//! CMS Review Workflow API endpoints
//!
//! Server functions for content review, approval, and publishing workflow.
//!
//! SECURITY: All mutating operations require authentication. User identity is
//! extracted from the request context, not from client-provided parameters.

use leptos::prelude::*;
use crate::types::{
    ContentReview, PreviewToken, ReviewWithContent, PendingReviewsResponse,
    PublishResult, PublishError, VersionDiff, ContentChange, ChangeType,
    ContentVersion,
};

/// Submit content version for review
///
/// # Security
/// The `submitted_by` identity is validated against the authenticated user.
/// If provided value doesn't match authenticated user, the server uses the authenticated identity.
#[server(SubmitForReview, "/api/cms")]
pub async fn submit_for_review(
    version_id: i32,
    submitted_by: String,
    notes: Option<String>,
) -> Result<ContentReview, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_authenticated_user;
    use crate::types::SubmitReviewRequest;

    // SECURITY: Extract authenticated user from request context
    let req: HttpRequest = extract().await?;
    let auth_user = get_authenticated_user(&req)
        .map_err(|e| ServerFnError::new(format!("Authentication required: {}", e)))?;

    // Use authenticated user ID, not client-provided value
    let verified_submitter = auth_user.id;
    tracing::info!(
        authenticated_user = %verified_submitter,
        claimed_user = %submitted_by,
        "Submit for review - using authenticated identity"
    );

    let factory: Data<ServiceFactory> = extract().await?;

    let request = SubmitReviewRequest {
        version_id,
        submitted_by: verified_submitter,
        notes,
    };

    let review = factory.review_repository
        .submit_for_review(&request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(review)
}

/// Approve a content review
///
/// # Security
/// Requires authentication. Reviewer identity is taken from authenticated user.
#[server(ApproveReview, "/api/cms")]
pub async fn approve_review(
    review_id: i32,
    reviewed_by: String,
    notes: Option<String>,
) -> Result<ContentReview, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_authenticated_user;
    use crate::types::{ReviewDecisionRequest, ReviewDecision};

    // SECURITY: Extract authenticated user
    let req: HttpRequest = extract().await?;
    let auth_user = get_authenticated_user(&req)
        .map_err(|e| ServerFnError::new(format!("Authentication required: {}", e)))?;

    let verified_reviewer = auth_user.id;
    tracing::info!(
        authenticated_user = %verified_reviewer,
        claimed_user = %reviewed_by,
        review_id = review_id,
        "Approve review - using authenticated identity"
    );

    let factory: Data<ServiceFactory> = extract().await?;

    let request = ReviewDecisionRequest {
        review_id,
        reviewed_by: verified_reviewer,
        decision: ReviewDecision::Approve,
        notes,
    };

    let review = factory.review_repository
        .process_decision(&request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(review)
}

/// Request changes on a review
///
/// # Security
/// Requires authentication. Reviewer identity is taken from authenticated user.
#[server(RequestChanges, "/api/cms")]
pub async fn request_changes(
    review_id: i32,
    reviewed_by: String,
    notes: String,
) -> Result<ContentReview, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_authenticated_user;
    use crate::types::{ReviewDecisionRequest, ReviewDecision};

    if notes.trim().is_empty() {
        return Err(ServerFnError::new("Notes are required when requesting changes"));
    }

    // SECURITY: Extract authenticated user
    let req: HttpRequest = extract().await?;
    let auth_user = get_authenticated_user(&req)
        .map_err(|e| ServerFnError::new(format!("Authentication required: {}", e)))?;

    let verified_reviewer = auth_user.id;

    let factory: Data<ServiceFactory> = extract().await?;

    let request = ReviewDecisionRequest {
        review_id,
        reviewed_by: verified_reviewer,
        decision: ReviewDecision::RequestChanges,
        notes: Some(notes),
    };

    let review = factory.review_repository
        .process_decision(&request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(review)
}

/// Reject a review
///
/// # Security
/// Requires authentication. Reviewer identity is taken from authenticated user.
#[server(RejectReview, "/api/cms")]
pub async fn reject_review(
    review_id: i32,
    reviewed_by: String,
    notes: Option<String>,
) -> Result<ContentReview, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_authenticated_user;
    use crate::types::{ReviewDecisionRequest, ReviewDecision};

    // SECURITY: Extract authenticated user
    let req: HttpRequest = extract().await?;
    let auth_user = get_authenticated_user(&req)
        .map_err(|e| ServerFnError::new(format!("Authentication required: {}", e)))?;

    let verified_reviewer = auth_user.id;

    let factory: Data<ServiceFactory> = extract().await?;

    let request = ReviewDecisionRequest {
        review_id,
        reviewed_by: verified_reviewer,
        decision: ReviewDecision::Reject,
        notes,
    };

    let review = factory.review_repository
        .process_decision(&request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(review)
}

/// Get pending reviews for reviewer dashboard
#[server(GetPendingReviews, "/api/cms")]
pub async fn get_pending_reviews(
    limit: Option<i32>,
) -> Result<PendingReviewsResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    let reviews = factory.review_repository
        .get_pending_reviews(limit)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let total = reviews.len() as i64;

    Ok(PendingReviewsResponse { reviews, total })
}

/// Get reviews submitted by a specific user
#[server(GetMySubmissions, "/api/cms")]
pub async fn get_my_submissions(
    submitter: String,
    limit: Option<i32>,
) -> Result<Vec<ReviewWithContent>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    let reviews = factory.review_repository
        .get_reviews_by_submitter(&submitter, limit)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(reviews)
}

/// Create a preview token for a content version
///
/// # Security
/// Requires authentication. Creator identity is taken from authenticated user.
#[server(CreatePreviewToken, "/api/cms")]
pub async fn create_preview_token(
    version_id: i32,
    created_by: String,
    expires_in_hours: Option<i32>,
) -> Result<PreviewToken, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_authenticated_user;

    // SECURITY: Extract authenticated user
    let req: HttpRequest = extract().await?;
    let auth_user = get_authenticated_user(&req)
        .map_err(|e| ServerFnError::new(format!("Authentication required: {}", e)))?;

    let verified_creator = auth_user.id;

    let factory: Data<ServiceFactory> = extract().await?;

    let hours = expires_in_hours.unwrap_or(24);

    let token = factory.review_repository
        .create_preview_token(version_id, &verified_creator, hours)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(token)
}

/// Get preview content by token
#[server(GetPreviewContent, "/api")]
pub async fn get_preview_content(
    token: String,
) -> Result<Option<ContentVersion>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // Validate the token
    let preview_token = factory.review_repository
        .validate_preview_token(&token)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    match preview_token {
        Some(pt) => {
            // Get the version content by version ID
            let version = factory.content_service
                .get_version_by_id(pt.version_id)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;

            Ok(version)
        }
        None => Ok(None),
    }
}

/// Publish approved content directly (skip review)
///
/// # Security
/// Requires authentication. Publisher identity is taken from authenticated user.
#[server(PublishDirect, "/api/cms")]
pub async fn publish_direct(
    content_id: i32,
    _published_by: String,
    commit_message: Option<String>,
) -> Result<PublishResult, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_authenticated_user;

    // SECURITY: Extract authenticated user
    let req: HttpRequest = extract().await?;
    let auth_user = get_authenticated_user(&req)
        .map_err(|e| ServerFnError::new(format!("Authentication required: {}", e)))?;

    let verified_publisher = auth_user.id;
    tracing::info!(
        authenticated_user = %verified_publisher,
        content_id = content_id,
        "Direct publish - using authenticated identity"
    );

    let factory: Data<ServiceFactory> = extract().await?;

    // Get the content to verify it exists
    let content = factory.content_service
        .get_by_id(content_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if content.is_none() {
        return Ok(PublishResult {
            success: false,
            published_count: 0,
            failed_count: 1,
            errors: vec![PublishError {
                content_id,
                error: "Content not found".to_string(),
            }],
        });
    }

    // Publish the content
    factory.content_service
        .publish(content_id, Some(&verified_publisher))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // If commit message provided, create a version note
    if let Some(_msg) = commit_message {
        // The publish already updates the version, so we just log it
        tracing::info!("Direct publish of content {} by {}", content_id, verified_publisher);
    }

    Ok(PublishResult {
        success: true,
        published_count: 1,
        failed_count: 0,
        errors: vec![],
    })
}

/// Publish multiple approved content items
///
/// # Security
/// Requires authentication. Publisher identity is taken from authenticated user.
#[server(PublishApproved, "/api/cms")]
pub async fn publish_approved(
    content_ids: Vec<i32>,
    _published_by: String,
) -> Result<PublishResult, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_authenticated_user;

    // SECURITY: Extract authenticated user
    let req: HttpRequest = extract().await?;
    let auth_user = get_authenticated_user(&req)
        .map_err(|e| ServerFnError::new(format!("Authentication required: {}", e)))?;

    let verified_publisher = auth_user.id;
    tracing::info!(
        authenticated_user = %verified_publisher,
        content_count = content_ids.len(),
        "Batch publish - using authenticated identity"
    );

    let factory: Data<ServiceFactory> = extract().await?;

    let mut published_count = 0;
    let mut failed_count = 0;
    let mut errors = Vec::new();

    for content_id in content_ids {
        // Get content to check status
        let content = factory.content_service
            .get_by_id(content_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        match content {
            Some(c) => {
                // Check if approved or draft (direct publish allowed)
                if c.status != "approved" && c.status != "draft" {
                    errors.push(PublishError {
                        content_id,
                        error: format!("Content status is '{}', expected 'approved' or 'draft'", c.status),
                    });
                    failed_count += 1;
                    continue;
                }

                match factory.content_service
                    .publish(content_id, Some(&verified_publisher))
                    .await
                {
                    Ok(_) => published_count += 1,
                    Err(e) => {
                        errors.push(PublishError {
                            content_id,
                            error: e.to_string(),
                        });
                        failed_count += 1;
                    }
                }
            }
            None => {
                errors.push(PublishError {
                    content_id,
                    error: "Content not found".to_string(),
                });
                failed_count += 1;
            }
        }
    }

    Ok(PublishResult {
        success: failed_count == 0,
        published_count,
        failed_count,
        errors,
    })
}

/// Compare two content versions
#[server(CompareVersions, "/api/cms")]
pub async fn compare_versions(
    content_id: i32,
    version_a: i32,
    version_b: i32,
) -> Result<VersionDiff, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // Get both versions
    let ver_a = factory.content_service
        .get_version(content_id, version_a)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new(format!("Version {} not found", version_a)))?;

    let ver_b = factory.content_service
        .get_version(content_id, version_b)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new(format!("Version {} not found", version_b)))?;

    // Compare the versions
    let changes = compute_diff(&ver_a.content, &ver_b.content, "en");

    // Also compare translations
    let mut all_changes = changes;
    if let (Some(trans_a), Some(trans_b)) = (&ver_a.translations, &ver_b.translations) {
        // Get all locales from both versions
        let mut locales: std::collections::HashSet<String> = std::collections::HashSet::new();
        if let Some(obj_a) = trans_a.as_object() {
            locales.extend(obj_a.keys().cloned());
        }
        if let Some(obj_b) = trans_b.as_object() {
            locales.extend(obj_b.keys().cloned());
        }

        for locale in locales {
            let locale_a = trans_a.get(&locale).unwrap_or(&serde_json::Value::Null);
            let locale_b = trans_b.get(&locale).unwrap_or(&serde_json::Value::Null);
            let locale_changes = compute_diff(locale_a, locale_b, &locale);
            all_changes.extend(locale_changes);
        }
    }

    let total_changes = all_changes.len() as i32;

    Ok(VersionDiff {
        version_a,
        version_b,
        content_id,
        changes: all_changes,
        total_changes,
    })
}

/// Compute diff between two JSON values
fn compute_diff(a: &serde_json::Value, b: &serde_json::Value, locale: &str) -> Vec<ContentChange> {
    let mut changes = Vec::new();

    // Compare as objects
    match (a.as_object(), b.as_object()) {
        (Some(obj_a), Some(obj_b)) => {
            // Find all keys
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
            // Not both objects, compare as strings
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

/// Get content version history
#[server(GetVersionHistory, "/api/cms")]
pub async fn get_version_history(
    content_id: i32,
    limit: Option<i32>,
) -> Result<Vec<ContentVersion>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    let mut versions = factory.content_service
        .get_versions(content_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Apply limit if specified
    if let Some(lim) = limit {
        versions.truncate(lim as usize);
    }

    Ok(versions)
}

/// Revert content to a previous version
///
/// # Security
/// Requires authentication. User identity is taken from authenticated user.
#[server(RevertToVersion, "/api/cms")]
pub async fn revert_to_version(
    content_id: i32,
    version: i32,
    _reverted_by: String,
) -> Result<ContentVersion, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::server::extractors::auth::get_authenticated_user;

    // SECURITY: Extract authenticated user
    let req: HttpRequest = extract().await?;
    let auth_user = get_authenticated_user(&req)
        .map_err(|e| ServerFnError::new(format!("Authentication required: {}", e)))?;

    let verified_user = auth_user.id;
    tracing::info!(
        authenticated_user = %verified_user,
        content_id = content_id,
        target_version = version,
        "Revert to version - using authenticated identity"
    );

    let factory: Data<ServiceFactory> = extract().await?;

    let _content = factory.content_service
        .revert_to_version(content_id, version, Some(&verified_user))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Return the new version that was created
    let versions = factory.content_service
        .get_versions(content_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    versions.first().cloned()
        .ok_or_else(|| ServerFnError::new("Failed to get reverted version"))
}
