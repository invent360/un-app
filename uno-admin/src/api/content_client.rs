//! Content API client for CMS operations with uno-app.
//!
//! This module provides HMAC-authenticated client for managing CMS content
//! in uno-app's PostgreSQL database.

use serde::{Deserialize, Serialize};

// ========================================
// Shared Types (available in both SSR and hydrate)
// ========================================

/// Content status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentStatus {
    Draft,
    PendingReview,
    Approved,
    Published,
    Archived,
}

/// Review decision options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecision {
    Approve,
    RequestChanges,
    Reject,
}

/// Content review record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentReview {
    pub id: i32,
    pub version_id: i32,
    pub status: String,
    pub submitted_by: String,
    pub submitted_at: String,
    pub submitted_notes: Option<String>,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<String>,
    pub review_notes: Option<String>,
}

/// Preview token for viewing draft content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewToken {
    pub id: i32,
    pub version_id: i32,
    pub token: String,
    pub created_by: String,
    pub created_at: String,
    pub expires_at: String,
}

/// Review with associated content information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewWithContent {
    pub review: ContentReview,
    pub content_id: i32,
    pub content_type: String,
    pub slug: String,
    pub title: String,
    pub version: i32,
    pub change_summary: Option<String>,
}

/// Response for pending reviews list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingReviewsResponse {
    pub reviews: Vec<ReviewWithContent>,
    pub total: i64,
}

/// Result of publish operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResult {
    pub success: bool,
    pub published_count: i32,
    pub failed_count: i32,
    pub errors: Vec<PublishError>,
}

/// Error details for failed publish
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishError {
    pub content_id: i32,
    pub error: String,
}

/// Full content version record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentVersion {
    pub id: i32,
    pub content_id: i32,
    pub version: i32,
    pub content: serde_json::Value,
    pub translations: Option<serde_json::Value>,
    pub change_summary: Option<String>,
    pub created_at: String,
    pub created_by: Option<String>,
}

/// Diff between two versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDiff {
    pub version_a: i32,
    pub version_b: i32,
    pub content_id: i32,
    pub changes: Vec<ContentChange>,
    pub total_changes: i32,
}

/// Individual field change in version diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentChange {
    pub field: String,
    pub locale: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub change_type: String,
}

/// Content summary in list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSummary {
    pub id: i32,
    pub content_type: String,
    pub slug: String,
    pub title: String,
    pub status: String,
    pub version: i32,
    pub published_version: Option<i32>,
    pub is_featured: bool,
    pub translation_coverage: i32,
    pub updated_at: String,
}

/// Paginated content list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentListResponse {
    pub items: Vec<ContentSummary>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

/// Version summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionSummary {
    pub version: i32,
    pub change_summary: Option<String>,
    pub created_at: String,
    pub created_by: Option<String>,
}

/// Full content detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentDetailResponse {
    pub id: i32,
    pub content_type: String,
    pub slug: String,
    pub status: String,
    pub content: serde_json::Value,
    pub translations: serde_json::Value,
    pub translation_status: std::collections::HashMap<String, String>,
    pub translation_coverage: i32,
    pub display_order: i32,
    pub is_featured: bool,
    pub version: i32,
    pub published_version: Option<i32>,
    pub published_at: Option<String>,
    pub versions: Vec<VersionSummary>,
    pub created_at: String,
    pub updated_at: String,
}

// ========================================
// SSR-only types and implementation
// ========================================

#[cfg(feature = "ssr")]
pub use ssr_impl::*;

#[cfg(feature = "ssr")]
mod ssr_impl {
    use super::*;
    use thiserror::Error;

    /// Content client error types
    #[derive(Debug, Error)]
    pub enum ContentClientError {
        #[error("HTTP error: {0}")]
        Http(#[from] reqwest::Error),
        #[error("JSON error: {0}")]
        Json(#[from] serde_json::Error),
        #[error("API error: {message}")]
        Api { message: String, code: String },
        #[error("Configuration error: {0}")]
        Config(String),
    }

    /// Signed request wrapper for HMAC authentication
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SignedRequest<T> {
        pub client_id: String,
        pub timestamp: i64,
        pub nonce: String,
        pub signature: String,
        pub payload: T,
    }

    // ========================================
    // Review Workflow Request Types
    // ========================================

    /// Request to submit content for review
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SubmitReviewRequest {
        pub version_id: i32,
        pub submitted_by: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notes: Option<String>,
    }

    /// Request to process a review decision
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ReviewDecisionRequest {
        pub review_id: i32,
        pub reviewed_by: String,
        pub decision: ReviewDecision,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notes: Option<String>,
    }

    /// Request to create a preview token
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreatePreviewTokenRequest {
        pub version_id: i32,
        pub created_by: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub expires_in_hours: Option<i32>,
    }

    /// Request to directly publish content
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PublishDirectRequest {
        pub content_id: i32,
        pub published_by: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub commit_message: Option<String>,
    }

    /// Request to batch publish approved content
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PublishApprovedRequest {
        pub content_ids: Vec<i32>,
        pub published_by: String,
    }

    /// Request to compare versions
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CompareVersionsRequest {
        pub content_id: i32,
        pub version_a: i32,
        pub version_b: i32,
    }

    /// Request to get version history
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetVersionHistoryRequest {
        pub content_id: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i32>,
    }

    /// Request to revert to a version
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RevertToVersionRequest {
        pub content_id: i32,
        pub version: i32,
        pub reverted_by: String,
    }

    /// Request to create or update content
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpsertContentRequest {
        pub content_type: String,
        pub slug: String,
        pub content: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub translations: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub display_order: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_featured: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub change_summary: Option<String>,
    }

    /// Parameters for listing content
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct ContentListParams {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub search: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<i32>,
    }

    /// Request to publish content
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct PublishContentRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub published_by: Option<String>,
    }

    /// Request to revert content
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RevertContentRequest {
        pub version: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reverted_by: Option<String>,
    }

    /// Request to update content schedule
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateScheduleRequest {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub publish_at: Option<chrono::DateTime<chrono::Utc>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unpublish_at: Option<chrono::DateTime<chrono::Utc>>,
    }

    /// Response for schedule update
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ScheduleResponse {
        pub id: i32,
        pub publish_at: Option<String>,
        pub unpublish_at: Option<String>,
        pub updated_at: String,
    }

    /// Audit log filter parameters
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct AuditLogFilter {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub entity_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub entity_id: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub actor_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to_date: Option<String>,
        #[serde(default = "default_page")]
        pub page: i32,
        #[serde(default = "default_per_page")]
        pub per_page: i32,
    }

    fn default_page() -> i32 {
        1
    }
    fn default_per_page() -> i32 {
        50
    }

    /// Audit log entry
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AuditLogEntry {
        pub id: i32,
        pub entity_type: String,
        pub entity_id: Option<i32>,
        pub action: String,
        pub actor_id: String,
        pub actor_name: Option<String>,
        pub old_values: Option<serde_json::Value>,
        pub new_values: Option<serde_json::Value>,
        pub metadata: Option<serde_json::Value>,
        pub created_at: String,
    }

    /// Audit log list response
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AuditLogResponse {
        pub entries: Vec<AuditLogEntry>,
        pub total: i64,
        pub page: i32,
        pub per_page: i32,
    }

    /// User permissions response
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserPermissions {
        pub user_id: String,
        pub roles: Vec<String>,
        pub permissions: Vec<String>,
    }

    /// Role definition
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Role {
        pub id: i32,
        pub name: String,
        pub description: Option<String>,
        pub is_system: bool,
        pub created_at: String,
        pub updated_at: String,
    }

    /// Request to assign/remove role
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RoleAssignmentRequest {
        pub user_id: String,
        pub role_name: String,
    }

    /// API response wrapper for create/update operations
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ContentResponse {
        pub id: i32,
        pub content_type: String,
        pub slug: String,
        pub status: String,
        pub version: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub published_version: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub published_at: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
    }

    /// API error response
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ApiErrorResponse {
        pub error: String,
        pub code: String,
    }

    /// Content API client
    #[derive(Clone)]
    pub struct ContentClient {
        http_client: reqwest::Client,
        base_url: String,
        client_id: String,
        secret_key: Vec<u8>,
    }

    impl ContentClient {
        /// Create a new client from environment configuration.
        ///
        /// Required environment variables:
        /// - UNO_API_URL: Base URL of the uno-app API (default: http://localhost:3000)
        /// - ADMIN_CLIENT_ID or UNO_CLIENT_ID: Client identifier for HMAC auth
        /// - ADMIN_SECRET_KEY or UNO_SECRET_KEY: Secret key for HMAC signing
        pub fn from_env() -> Result<Self, ContentClientError> {
            let api_url = std::env::var("UNO_API_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string());

            // ADMIN_CLIENT_ID takes precedence over UNO_CLIENT_ID
            let client_id = std::env::var("ADMIN_CLIENT_ID")
                .or_else(|_| std::env::var("UNO_CLIENT_ID"))
                .map_err(|_| {
                    ContentClientError::Config("ADMIN_CLIENT_ID or UNO_CLIENT_ID not set".into())
                })?;

            // ADMIN_SECRET_KEY takes precedence over UNO_SECRET_KEY
            let secret_key = std::env::var("ADMIN_SECRET_KEY")
                .or_else(|_| std::env::var("UNO_SECRET_KEY"))
                .map_err(|_| {
                    ContentClientError::Config("ADMIN_SECRET_KEY or UNO_SECRET_KEY not set".into())
                })?;

            Ok(Self::new(&api_url, &client_id, secret_key.as_bytes()))
        }

        /// Create a new client with explicit configuration.
        pub fn new(base_url: &str, client_id: &str, secret_key: &[u8]) -> Self {
            Self {
                http_client: reqwest::Client::new(),
                base_url: base_url.trim_end_matches('/').to_string(),
                client_id: client_id.to_string(),
                secret_key: secret_key.to_vec(),
            }
        }

        /// Sign a payload with HMAC-SHA256.
        fn sign<T: Serialize + Clone>(&self, payload: &T) -> SignedRequest<T> {
            use hmac::{Hmac, Mac};
            use sha2::Sha256;

            let timestamp = chrono::Utc::now().timestamp();
            let nonce = uuid::Uuid::new_v4().to_string();
            let payload_json = serde_json::to_string(payload).unwrap_or_default();

            let message = format!(
                "{}:{}:{}:{}",
                self.client_id, timestamp, nonce, payload_json
            );

            let mut mac = Hmac::<Sha256>::new_from_slice(&self.secret_key)
                .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let signature = hex::encode(mac.finalize().into_bytes());

            SignedRequest {
                client_id: self.client_id.clone(),
                timestamp,
                nonce,
                signature,
                payload: payload.clone(),
            }
        }

        /// Create new content.
        pub async fn create_content(
            &self,
            request: UpsertContentRequest,
        ) -> Result<ContentResponse, ContentClientError> {
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/contents", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Update existing content.
        pub async fn update_content(
            &self,
            id: i32,
            request: UpsertContentRequest,
        ) -> Result<ContentResponse, ContentClientError> {
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/contents/{}", self.base_url, id);

            let response = self.http_client.put(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// List all content with filters.
        pub async fn list_contents(
            &self,
            params: ContentListParams,
        ) -> Result<ContentListResponse, ContentClientError> {
            let signed = self.sign(&params);
            let url = format!("{}/api/v1/admin/contents/list", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Get single content detail.
        pub async fn get_content(
            &self,
            id: i32,
        ) -> Result<ContentDetailResponse, ContentClientError> {
            let signed = self.sign(&serde_json::json!({}));
            let url = format!("{}/api/v1/admin/contents/{}/get", self.base_url, id);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Delete content (soft delete).
        pub async fn delete_content(&self, id: i32) -> Result<(), ContentClientError> {
            let signed = self.sign(&serde_json::json!({}));
            let url = format!("{}/api/v1/admin/contents/{}/delete", self.base_url, id);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            let _: serde_json::Value = self.handle_response(response).await?;
            Ok(())
        }

        /// Publish content.
        pub async fn publish_content(
            &self,
            id: i32,
            request: PublishContentRequest,
        ) -> Result<ContentResponse, ContentClientError> {
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/contents/{}/publish", self.base_url, id);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Archive content.
        pub async fn archive_content(
            &self,
            id: i32,
        ) -> Result<ContentResponse, ContentClientError> {
            let signed = self.sign(&serde_json::json!({}));
            let url = format!("{}/api/v1/admin/contents/{}/archive", self.base_url, id);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Revert content to a previous version.
        pub async fn revert_content(
            &self,
            id: i32,
            request: RevertContentRequest,
        ) -> Result<ContentResponse, ContentClientError> {
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/contents/{}/revert", self.base_url, id);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        // ========================================
        // Review Workflow Methods
        // ========================================

        /// Submit a content version for review.
        pub async fn submit_for_review(
            &self,
            version_id: i32,
            submitted_by: &str,
            notes: Option<&str>,
        ) -> Result<ContentReview, ContentClientError> {
            let request = SubmitReviewRequest {
                version_id,
                submitted_by: submitted_by.to_string(),
                notes: notes.map(String::from),
            };
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/reviews/submit", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Approve a pending review.
        pub async fn approve_review(
            &self,
            review_id: i32,
            reviewed_by: &str,
            notes: Option<&str>,
        ) -> Result<ContentReview, ContentClientError> {
            // REST API uses path parameter for review_id
            let request = serde_json::json!({
                "reviewed_by": reviewed_by,
                "notes": notes
            });
            let signed = self.sign(&request);
            let url = format!(
                "{}/api/v1/admin/reviews/{}/approve",
                self.base_url, review_id
            );

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Request changes on a review.
        pub async fn request_changes(
            &self,
            review_id: i32,
            reviewed_by: &str,
            notes: &str,
        ) -> Result<ContentReview, ContentClientError> {
            // REST API uses path parameter for review_id
            let request = serde_json::json!({
                "reviewed_by": reviewed_by,
                "notes": notes
            });
            let signed = self.sign(&request);
            let url = format!(
                "{}/api/v1/admin/reviews/{}/request-changes",
                self.base_url, review_id
            );

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Reject a review.
        pub async fn reject_review(
            &self,
            review_id: i32,
            reviewed_by: &str,
            notes: Option<&str>,
        ) -> Result<ContentReview, ContentClientError> {
            // REST API uses path parameter for review_id
            let request = serde_json::json!({
                "reviewed_by": reviewed_by,
                "notes": notes
            });
            let signed = self.sign(&request);
            let url = format!(
                "{}/api/v1/admin/reviews/{}/reject",
                self.base_url, review_id
            );

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Get pending reviews for the reviewer dashboard.
        pub async fn get_pending_reviews(
            &self,
            limit: Option<i32>,
        ) -> Result<PendingReviewsResponse, ContentClientError> {
            let request = serde_json::json!({ "limit": limit });
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/reviews/pending", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Get reviews submitted by a specific user.
        pub async fn get_my_submissions(
            &self,
            submitter: &str,
            limit: Option<i32>,
        ) -> Result<Vec<ReviewWithContent>, ContentClientError> {
            let request = serde_json::json!({
                "submitter": submitter,
                "limit": limit
            });
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/reviews/my-submissions", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Create a preview token for viewing draft content.
        pub async fn create_preview_token(
            &self,
            version_id: i32,
            created_by: &str,
            expires_in_hours: Option<i32>,
        ) -> Result<PreviewToken, ContentClientError> {
            let request = CreatePreviewTokenRequest {
                version_id,
                created_by: created_by.to_string(),
                expires_in_hours,
            };
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/preview-tokens", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Publish content directly, skipping review.
        pub async fn publish_direct(
            &self,
            content_id: i32,
            published_by: &str,
            commit_message: Option<&str>,
        ) -> Result<PublishResult, ContentClientError> {
            let request = PublishDirectRequest {
                content_id,
                published_by: published_by.to_string(),
                commit_message: commit_message.map(String::from),
            };
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/publish/direct", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Batch publish multiple approved content items.
        pub async fn publish_approved(
            &self,
            content_ids: Vec<i32>,
            published_by: &str,
        ) -> Result<PublishResult, ContentClientError> {
            let request = PublishApprovedRequest {
                content_ids,
                published_by: published_by.to_string(),
            };
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/publish/batch", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Get version history for content.
        pub async fn get_version_history(
            &self,
            content_id: i32,
            limit: Option<i32>,
        ) -> Result<Vec<ContentVersion>, ContentClientError> {
            // REST API uses path parameter for content_id
            let request = serde_json::json!({ "limit": limit });
            let signed = self.sign(&request);
            let url = format!(
                "{}/api/v1/admin/versions/{}/history",
                self.base_url, content_id
            );

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Compare two versions of content.
        pub async fn compare_versions(
            &self,
            content_id: i32,
            version_a: i32,
            version_b: i32,
        ) -> Result<VersionDiff, ContentClientError> {
            // REST API uses path parameter for content_id
            let request = serde_json::json!({
                "version_a": version_a,
                "version_b": version_b
            });
            let signed = self.sign(&request);
            let url = format!(
                "{}/api/v1/admin/versions/{}/compare",
                self.base_url, content_id
            );

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Revert to a specific version.
        pub async fn revert_to_version(
            &self,
            content_id: i32,
            version: i32,
            reverted_by: &str,
        ) -> Result<ContentVersion, ContentClientError> {
            // REST API uses path parameter for content_id
            let request = serde_json::json!({
                "version": version,
                "reverted_by": reverted_by
            });
            let signed = self.sign(&request);
            let url = format!(
                "{}/api/v1/admin/versions/{}/revert",
                self.base_url, content_id
            );

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        // ========================================
        // Scheduling Methods
        // ========================================

        /// Update content schedule (publish_at, unpublish_at)
        pub async fn update_schedule(
            &self,
            content_id: i32,
            publish_at: Option<chrono::DateTime<chrono::Utc>>,
            unpublish_at: Option<chrono::DateTime<chrono::Utc>>,
        ) -> Result<ScheduleResponse, ContentClientError> {
            let request = UpdateScheduleRequest {
                publish_at,
                unpublish_at,
            };
            let signed = self.sign(&request);
            let url = format!(
                "{}/api/v1/admin/contents/{}/schedule",
                self.base_url, content_id
            );

            let response = self.http_client.put(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        // ========================================
        // Audit Log Methods
        // ========================================

        /// List audit logs with filtering
        pub async fn list_audit_logs(
            &self,
            filter: AuditLogFilter,
        ) -> Result<AuditLogResponse, ContentClientError> {
            let signed = self.sign(&filter);
            let url = format!("{}/api/v1/admin/audit-logs", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Get audit history for a specific content item
        pub async fn get_content_audit_history(
            &self,
            content_id: i32,
        ) -> Result<Vec<AuditLogEntry>, ContentClientError> {
            let signed = self.sign(&serde_json::json!({}));
            let url = format!(
                "{}/api/v1/admin/contents/{}/audit",
                self.base_url, content_id
            );

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        // ========================================
        // RBAC Methods
        // ========================================

        /// Get current user's permissions
        pub async fn get_my_permissions(&self) -> Result<UserPermissions, ContentClientError> {
            let signed = self.sign(&serde_json::json!({}));
            let url = format!("{}/api/v1/admin/users/me/permissions", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Get permissions for a specific user
        pub async fn get_user_permissions(
            &self,
            user_id: &str,
        ) -> Result<UserPermissions, ContentClientError> {
            let signed = self.sign(&serde_json::json!({}));
            let url = format!(
                "{}/api/v1/admin/users/{}/permissions",
                self.base_url, user_id
            );

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// List all available roles
        pub async fn list_roles(&self) -> Result<Vec<Role>, ContentClientError> {
            let signed = self.sign(&serde_json::json!({}));
            let url = format!("{}/api/v1/admin/roles", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Assign a role to a user
        pub async fn assign_role(
            &self,
            user_id: &str,
            role_name: &str,
        ) -> Result<serde_json::Value, ContentClientError> {
            let request = RoleAssignmentRequest {
                user_id: user_id.to_string(),
                role_name: role_name.to_string(),
            };
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/users/roles/assign", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Remove a role from a user
        pub async fn remove_role(
            &self,
            user_id: &str,
            role_name: &str,
        ) -> Result<serde_json::Value, ContentClientError> {
            let request = RoleAssignmentRequest {
                user_id: user_id.to_string(),
                role_name: role_name.to_string(),
            };
            let signed = self.sign(&request);
            let url = format!("{}/api/v1/admin/users/roles/remove", self.base_url);

            let response = self.http_client.post(&url).json(&signed).send().await?;

            self.handle_response(response).await
        }

        /// Handle API response, parsing success or error.
        async fn handle_response<T: serde::de::DeserializeOwned>(
            &self,
            response: reqwest::Response,
        ) -> Result<T, ContentClientError> {
            let status = response.status();
            let body = response.text().await?;

            if status.is_success() {
                serde_json::from_str(&body).map_err(Into::into)
            } else {
                // Try to parse as API error response
                if let Ok(error) = serde_json::from_str::<ApiErrorResponse>(&body) {
                    Err(ContentClientError::Api {
                        message: error.error,
                        code: error.code,
                    })
                } else {
                    Err(ContentClientError::Api {
                        message: body,
                        code: status.as_str().to_string(),
                    })
                }
            }
        }
    }
}
