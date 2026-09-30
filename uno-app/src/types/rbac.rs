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

/// Permission constants (resource:action format)
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

    // License permissions
    pub const LICENSE_READ: &str = "license:read";
    pub const LICENSE_CLAIM: &str = "license:claim";
    pub const LICENSE_ADMIN: &str = "license:admin";
    pub const LICENSE_IMPORT: &str = "license:import";
    pub const LICENSE_REVOKE: &str = "license:revoke";

    // Finance permissions
    pub const FINANCE_READ: &str = "finance:read";
    pub const FINANCE_WRITE: &str = "finance:write";
    pub const FINANCE_APPROVE: &str = "finance:approve";
    pub const FINANCE_RECONCILE: &str = "finance:reconcile";

    // Support permissions
    pub const SUPPORT_READ: &str = "support:read";
    pub const SUPPORT_WRITE: &str = "support:write";
    pub const SUPPORT_ESCALATE: &str = "support:escalate";

    // Agent permissions
    pub const AGENT_READ: &str = "agent:read";
    pub const AGENT_MANAGE: &str = "agent:manage";
    pub const REFERRAL_READ: &str = "referral:read";
    pub const REFERRAL_APPROVE: &str = "referral:approve";

    // User management permissions
    pub const USER_READ: &str = "user:read";
    pub const USER_MANAGE: &str = "user:manage";
    pub const USER_SUSPEND: &str = "user:suspend";

    // System permissions
    pub const AUDIT_READ: &str = "audit:read";
    pub const SETTINGS_MANAGE: &str = "settings:manage";
    pub const GATE_MANAGE: &str = "gate:manage";

    // Worker/integration permissions
    pub const WORKER_EXECUTE: &str = "worker:execute";
    pub const INTEGRATION_READ: &str = "integration:read";
    pub const INTEGRATION_MANAGE: &str = "integration:manage";
}

/// Role constants matching JWT role claims
pub mod roles {
    // Content management roles
    pub const AUTHOR: &str = "content_author";
    pub const REVIEWER: &str = "reviewer";
    pub const PUBLISHER: &str = "publisher";

    // Administrative roles
    pub const ADMIN: &str = "admin";
    pub const OPERATOR: &str = "operator";

    // User-facing roles
    pub const PARTICIPANT: &str = "participant";

    // Support roles
    pub const SUPPORT: &str = "support";
    pub const AGENT: &str = "agent";

    // Country-level agent role with geographic scope
    pub const COUNTRY_AGENT: &str = "country_agent";

    // Finance roles
    pub const FINANCE: &str = "finance";

    // System/background worker roles
    pub const WORKER: &str = "worker";
    pub const INTEGRATION_WORKER: &str = "integration_worker";

    /// All available roles
    pub const ALL_ROLES: &[&str] = &[
        AUTHOR,
        REVIEWER,
        PUBLISHER,
        ADMIN,
        OPERATOR,
        PARTICIPANT,
        SUPPORT,
        AGENT,
        COUNTRY_AGENT,
        FINANCE,
        WORKER,
        INTEGRATION_WORKER,
    ];

    /// Roles that require MFA for authentication
    pub const MFA_REQUIRED_ROLES: &[&str] = &[
        OPERATOR, AUTHOR, REVIEWER, PUBLISHER, FINANCE, AGENT, COUNTRY_AGENT,
    ];

    /// Roles that can access the admin dashboard
    pub const ADMIN_DASHBOARD_ROLES: &[&str] = &[
        OPERATOR, AUTHOR, REVIEWER, PUBLISHER, SUPPORT, FINANCE, AGENT, COUNTRY_AGENT,
    ];
}
