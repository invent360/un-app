//! Audit log type definitions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Audit log entry from database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct AuditLogEntry {
    pub id: i32,
    pub entity_type: String,
    pub entity_id: Option<i32>,
    pub action: String,
    pub actor_id: String,
    pub actor_name: Option<String>,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub old_values: Option<JsonValue>,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub new_values: Option<JsonValue>,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}

/// Request to create a new audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuditLog {
    pub entity_type: String,
    pub entity_id: Option<i32>,
    pub action: String,
    pub actor_id: String,
    pub actor_name: Option<String>,
    pub old_values: Option<JsonValue>,
    pub new_values: Option<JsonValue>,
    pub metadata: Option<JsonValue>,
}

/// Filter parameters for listing audit logs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLogFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_date: Option<DateTime<Utc>>,
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

/// Response for listing audit logs with pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogResponse {
    pub entries: Vec<AuditLogEntry>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

/// Common audit actions
pub mod actions {
    pub const CREATE: &str = "create";
    pub const UPDATE: &str = "update";
    pub const DELETE: &str = "delete";
    pub const PUBLISH: &str = "publish";
    pub const UNPUBLISH: &str = "unpublish";
    pub const ARCHIVE: &str = "archive";
    pub const REVERT: &str = "revert";
    pub const SCHEDULE: &str = "schedule";
    pub const SUBMIT_REVIEW: &str = "submit_review";
    pub const APPROVE: &str = "approve";
    pub const REJECT: &str = "reject";
}

/// Common entity types
pub mod entities {
    pub const CONTENT: &str = "content";
    pub const REVIEW: &str = "review";
    pub const USER: &str = "user";
    pub const ROLE: &str = "role";
}
