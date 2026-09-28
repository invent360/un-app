//! Role-Based Access Control type definitions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Role entity from database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Permission entity from database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// User role assignment with role name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserRole {
    pub user_id: String,
    pub role_id: i32,
    pub role_name: String,
    pub assigned_by: Option<String>,
    pub assigned_at: DateTime<Utc>,
}

/// User permissions summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissions {
    pub user_id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

/// Request to assign a role to a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleRequest {
    pub user_id: String,
    pub role_name: String,
}

/// Request to remove a role from a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRoleRequest {
    pub user_id: String,
    pub role_name: String,
}

/// Permission constants
pub mod permissions {
    // Content permissions
    pub const CONTENT_CREATE: &str = "content:create";
    pub const CONTENT_READ: &str = "content:read";
    pub const CONTENT_UPDATE: &str = "content:update";
    pub const CONTENT_DELETE: &str = "content:delete";
    pub const CONTENT_PUBLISH: &str = "content:publish";
    pub const CONTENT_REVERT: &str = "content:revert";
    pub const CONTENT_SCHEDULE: &str = "content:schedule";

    // Review permissions
    pub const REVIEW_SUBMIT: &str = "review:submit";
    pub const REVIEW_APPROVE: &str = "review:approve";

    // Admin permissions
    pub const USER_MANAGE: &str = "user:manage";
    pub const AUDIT_READ: &str = "audit:read";
    pub const SETTINGS_MANAGE: &str = "settings:manage";
}

/// Role constants
pub mod roles {
    pub const AUTHOR: &str = "author";
    pub const REVIEWER: &str = "reviewer";
    pub const PUBLISHER: &str = "publisher";
    pub const ADMIN: &str = "admin";
}
