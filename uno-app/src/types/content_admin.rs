//! Admin CMS request/response types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Create/Update content request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertContentRequest {
    pub content_type: String,
    pub slug: String,
    pub content: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translations: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_featured: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_summary: Option<String>,
}

/// Publish content request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PublishContentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_by: Option<String>,
}

/// Revert content request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevertContentRequest {
    pub version: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reverted_by: Option<String>,
}

/// Content list query parameters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContentListParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_drafts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

/// Content list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentListResponse {
    pub items: Vec<ContentSummary>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
}

/// Content summary for list view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSummary {
    pub id: i32,
    pub content_type: String,
    pub slug: String,
    pub title: String,  // Extracted from content JSON
    pub status: String,
    pub version: i32,
    pub published_version: Option<i32>,
    pub is_featured: bool,
    pub translation_coverage: i32,  // Percentage
    pub updated_at: DateTime<Utc>,
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
    pub translation_status: HashMap<String, String>,
    pub translation_coverage: i32,
    pub display_order: i32,
    pub is_featured: bool,
    pub version: i32,
    pub published_version: Option<i32>,
    pub published_at: Option<DateTime<Utc>>,
    pub versions: Vec<VersionSummary>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Version summary for history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionSummary {
    pub version: i32,
    pub change_summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

/// Content creation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentCreateResponse {
    pub id: i32,
    pub content_type: String,
    pub slug: String,
    pub status: String,
    pub version: i32,
    pub created_at: DateTime<Utc>,
}

/// Bulk content operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkContentRequest {
    pub contents: Vec<UpsertContentRequest>,
}

/// Bulk content operation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkContentResponse {
    pub created: i32,
    pub updated: i32,
    pub failed: i32,
    pub errors: Vec<BulkContentError>,
}

/// Error detail for bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkContentError {
    pub slug: String,
    pub error: String,
}

/// Update content schedule request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScheduleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpublish_at: Option<DateTime<Utc>>,
}
