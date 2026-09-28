//! CMS Review Workflow Types
//!
//! Types for the content review and approval workflow system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Review status for content submissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    #[default]
    Pending,
    Approved,
    ChangesRequested,
    Rejected,
}

impl ReviewStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReviewStatus::Pending => "pending",
            ReviewStatus::Approved => "approved",
            ReviewStatus::ChangesRequested => "changes_requested",
            ReviewStatus::Rejected => "rejected",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(ReviewStatus::Pending),
            "approved" => Some(ReviewStatus::Approved),
            "changes_requested" => Some(ReviewStatus::ChangesRequested),
            "rejected" => Some(ReviewStatus::Rejected),
            _ => None,
        }
    }

    /// Check if the review is in a final state
    pub fn is_final(&self) -> bool {
        matches!(self, ReviewStatus::Approved | ReviewStatus::Rejected)
    }

    /// Check if the review can be updated
    pub fn can_update(&self) -> bool {
        matches!(self, ReviewStatus::Pending)
    }
}

impl std::fmt::Display for ReviewStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Content review entity from database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ContentReview {
    pub id: i32,
    pub version_id: i32,
    pub status: String,  // Maps to ReviewStatus

    // Submission details
    pub submitted_by: String,
    pub submitted_at: DateTime<Utc>,
    pub submitted_notes: Option<String>,

    // Review details
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub review_notes: Option<String>,
}

impl ContentReview {
    /// Get the review status as an enum
    pub fn status_enum(&self) -> Option<ReviewStatus> {
        ReviewStatus::from_str(&self.status)
    }

    /// Check if this review is pending
    pub fn is_pending(&self) -> bool {
        self.status_enum() == Some(ReviewStatus::Pending)
    }

    /// Check if this review was approved
    pub fn is_approved(&self) -> bool {
        self.status_enum() == Some(ReviewStatus::Approved)
    }
}

/// Preview token for viewing draft content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct PreviewToken {
    pub id: i32,
    pub version_id: i32,
    pub token: String,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl PreviewToken {
    /// Check if the token has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if the token is valid
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}

/// Submit for review request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitReviewRequest {
    pub version_id: i32,
    pub submitted_by: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Approve/reject review request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewDecisionRequest {
    pub review_id: i32,
    pub reviewed_by: String,
    pub decision: ReviewDecision,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Review decision type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecision {
    Approve,
    RequestChanges,
    Reject,
}

/// Create preview token request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePreviewTokenRequest {
    pub version_id: i32,
    pub created_by: String,
    #[serde(default = "default_preview_hours")]
    pub expires_in_hours: i32,
}

fn default_preview_hours() -> i32 {
    24
}

/// Publish request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRequest {
    pub content_ids: Vec<i32>,
    pub published_by: String,
}

/// Direct publish request (skip review)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectPublishRequest {
    pub content_id: i32,
    pub published_by: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
}

/// Publish result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResult {
    pub success: bool,
    pub published_count: i32,
    pub failed_count: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<PublishError>,
}

/// Publish error detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishError {
    pub content_id: i32,
    pub error: String,
}

/// Content change in a version diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentChange {
    pub field: String,
    pub locale: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub change_type: ChangeType,
}

/// Type of change
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Added,
    Modified,
    Removed,
}

/// Version diff result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDiff {
    pub version_a: i32,
    pub version_b: i32,
    pub content_id: i32,
    pub changes: Vec<ContentChange>,
    pub total_changes: i32,
}

/// Review with content details for dashboard
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

/// Pending reviews response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingReviewsResponse {
    pub reviews: Vec<ReviewWithContent>,
    pub total: i64,
}

/// Publish queue item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishQueueItem {
    pub content_id: i32,
    pub content_type: String,
    pub slug: String,
    pub title: String,
    pub version: i32,
    pub approved_at: DateTime<Utc>,
    pub approved_by: Option<String>,
}

/// Publish queue response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishQueueResponse {
    pub items: Vec<PublishQueueItem>,
    pub total: i64,
}
