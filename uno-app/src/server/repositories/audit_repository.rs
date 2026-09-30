//! Audit log repository for database operations
//!
//! This module provides two audit repository implementations:
//! 1. AuditRepository - Legacy interface for the audit_logs table (CMS-focused)
//! 2. ImmutableAuditRepository - New interface for the audit_log table from migration 00020
//!    (compliance-focused, append-only, with enhanced event tracking)

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::net::IpAddr;
use std::sync::Arc;

use crate::server::db::ConnectionPool;
use crate::types::{AppError, AuditLogEntry, CreateAuditLog, AuditLogFilter, AuditLogResponse};

// ============================================
// LEGACY AUDIT REPOSITORY (audit_logs table)
// ============================================

/// Dynamic type alias for legacy AuditRepository trait object
pub type DynAuditRepository = Arc<dyn AuditRepository + Send + Sync>;

/// Legacy audit repository trait for CMS operations (audit_logs table)
#[async_trait]
pub trait AuditRepository: Send + Sync {
    /// Create a new audit log entry
    async fn create(&self, entry: CreateAuditLog) -> Result<AuditLogEntry, AppError>;

    /// List audit logs with filtering and pagination
    async fn list(&self, filter: AuditLogFilter) -> Result<AuditLogResponse, AppError>;

    /// Get audit logs for a specific entity
    async fn get_by_entity(&self, entity_type: &str, entity_id: i32) -> Result<Vec<AuditLogEntry>, AppError>;
}

/// Concrete implementation of legacy AuditRepository
pub struct AuditRepositoryImpl {
    db_pool: ConnectionPool,
}

impl AuditRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AuditRepository for AuditRepositoryImpl {
    async fn create(&self, entry: CreateAuditLog) -> Result<AuditLogEntry, AppError> {
        let item = sqlx::query_as::<_, AuditLogEntry>(
            r#"
            INSERT INTO audit_logs
                (entity_type, entity_id, action, actor_id, actor_name, old_values, new_values, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, entity_type, entity_id, action, actor_id, actor_name,
                      old_values, new_values, metadata, created_at
            "#
        )
        .bind(&entry.entity_type)
        .bind(entry.entity_id)
        .bind(&entry.action)
        .bind(&entry.actor_id)
        .bind(&entry.actor_name)
        .bind(&entry.old_values)
        .bind(&entry.new_values)
        .bind(&entry.metadata)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn list(&self, filter: AuditLogFilter) -> Result<AuditLogResponse, AppError> {
        let offset = ((filter.page - 1) * filter.per_page) as i64;
        let limit = filter.per_page as i64;

        // Count query with filters
        let total: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM audit_logs
            WHERE ($1::text IS NULL OR entity_type = $1)
              AND ($2::int IS NULL OR entity_id = $2)
              AND ($3::text IS NULL OR action = $3)
              AND ($4::text IS NULL OR actor_id = $4)
              AND ($5::timestamptz IS NULL OR created_at >= $5)
              AND ($6::timestamptz IS NULL OR created_at <= $6)
            "#
        )
        .bind(&filter.entity_type)
        .bind(filter.entity_id)
        .bind(&filter.action)
        .bind(&filter.actor_id)
        .bind(filter.from_date)
        .bind(filter.to_date)
        .fetch_one(&self.db_pool)
        .await?;

        // Data query with filters
        let entries = sqlx::query_as::<_, AuditLogEntry>(
            r#"
            SELECT id, entity_type, entity_id, action, actor_id, actor_name,
                   old_values, new_values, metadata, created_at
            FROM audit_logs
            WHERE ($1::text IS NULL OR entity_type = $1)
              AND ($2::int IS NULL OR entity_id = $2)
              AND ($3::text IS NULL OR action = $3)
              AND ($4::text IS NULL OR actor_id = $4)
              AND ($5::timestamptz IS NULL OR created_at >= $5)
              AND ($6::timestamptz IS NULL OR created_at <= $6)
            ORDER BY created_at DESC
            LIMIT $7 OFFSET $8
            "#
        )
        .bind(&filter.entity_type)
        .bind(filter.entity_id)
        .bind(&filter.action)
        .bind(&filter.actor_id)
        .bind(filter.from_date)
        .bind(filter.to_date)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(AuditLogResponse {
            entries,
            total: total.0,
            page: filter.page,
            per_page: filter.per_page,
        })
    }

    async fn get_by_entity(&self, entity_type: &str, entity_id: i32) -> Result<Vec<AuditLogEntry>, AppError> {
        let entries = sqlx::query_as::<_, AuditLogEntry>(
            r#"
            SELECT id, entity_type, entity_id, action, actor_id, actor_name,
                   old_values, new_values, metadata, created_at
            FROM audit_logs
            WHERE entity_type = $1 AND entity_id = $2
            ORDER BY created_at DESC
            LIMIT 100
            "#
        )
        .bind(entity_type)
        .bind(entity_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(entries)
    }
}

// ============================================
// NEW IMMUTABLE AUDIT REPOSITORY (audit_log table)
// Migration 00020_identity_sessions.up.sql
// ============================================

/// Dynamic type alias for ImmutableAuditRepository trait object
pub type DynImmutableAuditRepository = Arc<dyn ImmutableAuditRepository + Send + Sync>;

// ============================================
// ENUMS
// ============================================

/// Audit event categories for classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditCategory {
    /// Authentication events (login, logout, session management)
    Auth,
    /// User account events (profile updates, account changes)
    User,
    /// Permission and access control events
    Permission,
    /// Content management events (create, update, publish)
    Content,
    /// Financial and transaction events
    Finance,
    /// Privacy and consent events
    Privacy,
    /// System events (maintenance, configuration changes)
    System,
    /// Security events (auth failures, suspicious activity)
    Security,
    /// Compliance events (regulatory, audit trail)
    Compliance,
}

impl AuditCategory {
    /// Convert to database string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditCategory::Auth => "auth",
            AuditCategory::User => "user",
            AuditCategory::Permission => "permission",
            AuditCategory::Content => "content",
            AuditCategory::Finance => "finance",
            AuditCategory::System => "system",
            AuditCategory::Privacy => "privacy",
            AuditCategory::Security => "security",
            AuditCategory::Compliance => "compliance",
        }
    }

    /// Parse from database string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "auth" => Some(AuditCategory::Auth),
            "user" => Some(AuditCategory::User),
            "permission" => Some(AuditCategory::Permission),
            "content" => Some(AuditCategory::Content),
            "finance" => Some(AuditCategory::Finance),
            "system" => Some(AuditCategory::System),
            "privacy" => Some(AuditCategory::Privacy),
            "security" => Some(AuditCategory::Security),
            "compliance" => Some(AuditCategory::Compliance),
            _ => None,
        }
    }
}

/// Common audit event types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    // Auth events
    Login,
    Logout,
    LoginFailed,
    AuthFailure,
    SessionCreated,
    SessionRevoked,
    PasswordChanged,
    MfaEnabled,
    MfaDisabled,
    MfaVerified,

    // CRUD events
    Create,
    Read,
    Update,
    Delete,

    // Permission events
    PermissionGranted,
    PermissionRevoked,
    RoleAssigned,
    RoleRemoved,

    // Content events
    Publish,
    Unpublish,
    Archive,
    Restore,

    // Finance events
    PaymentInitiated,
    PaymentCompleted,
    PaymentFailed,
    RefundInitiated,
    RefundCompleted,

    // System events
    ConfigChanged,
    MaintenanceStarted,
    MaintenanceEnded,
    FeatureToggled,

    // Privacy/Consent events
    ConsentRecorded,
    ConsentWithdrawn,
    DataRequestCreated,
    DataRequestUpdated,
    DataRequestCompleted,

    // Custom event type
    Custom(String),
}

impl AuditEventType {
    /// Convert to database string representation
    pub fn as_str(&self) -> String {
        match self {
            AuditEventType::Login => "auth.login".to_string(),
            AuditEventType::Logout => "auth.logout".to_string(),
            AuditEventType::LoginFailed => "auth.login_failed".to_string(),
            AuditEventType::AuthFailure => "security.auth_failure".to_string(),
            AuditEventType::SessionCreated => "auth.session_created".to_string(),
            AuditEventType::SessionRevoked => "auth.session_revoked".to_string(),
            AuditEventType::PasswordChanged => "auth.password_changed".to_string(),
            AuditEventType::MfaEnabled => "auth.mfa_enabled".to_string(),
            AuditEventType::MfaDisabled => "auth.mfa_disabled".to_string(),
            AuditEventType::MfaVerified => "auth.mfa_verified".to_string(),
            AuditEventType::Create => "crud.create".to_string(),
            AuditEventType::Read => "crud.read".to_string(),
            AuditEventType::Update => "crud.update".to_string(),
            AuditEventType::Delete => "crud.delete".to_string(),
            AuditEventType::PermissionGranted => "permission.granted".to_string(),
            AuditEventType::PermissionRevoked => "permission.revoked".to_string(),
            AuditEventType::RoleAssigned => "permission.role_assigned".to_string(),
            AuditEventType::RoleRemoved => "permission.role_removed".to_string(),
            AuditEventType::Publish => "content.publish".to_string(),
            AuditEventType::Unpublish => "content.unpublish".to_string(),
            AuditEventType::Archive => "content.archive".to_string(),
            AuditEventType::Restore => "content.restore".to_string(),
            AuditEventType::PaymentInitiated => "finance.payment_initiated".to_string(),
            AuditEventType::PaymentCompleted => "finance.payment_completed".to_string(),
            AuditEventType::PaymentFailed => "finance.payment_failed".to_string(),
            AuditEventType::RefundInitiated => "finance.refund_initiated".to_string(),
            AuditEventType::RefundCompleted => "finance.refund_completed".to_string(),
            AuditEventType::ConfigChanged => "system.config_changed".to_string(),
            AuditEventType::MaintenanceStarted => "system.maintenance_started".to_string(),
            AuditEventType::MaintenanceEnded => "system.maintenance_ended".to_string(),
            AuditEventType::FeatureToggled => "system.feature_toggled".to_string(),
            AuditEventType::ConsentRecorded => "privacy.consent_recorded".to_string(),
            AuditEventType::ConsentWithdrawn => "privacy.consent_withdrawn".to_string(),
            AuditEventType::DataRequestCreated => "privacy.data_request_created".to_string(),
            AuditEventType::DataRequestUpdated => "privacy.data_request_updated".to_string(),
            AuditEventType::DataRequestCompleted => "privacy.data_request_completed".to_string(),
            AuditEventType::Custom(s) => s.clone(),
        }
    }

    /// Parse from database string
    pub fn from_str(s: &str) -> Self {
        match s {
            "auth.login" => AuditEventType::Login,
            "auth.logout" => AuditEventType::Logout,
            "auth.login_failed" => AuditEventType::LoginFailed,
            "security.auth_failure" => AuditEventType::AuthFailure,
            "auth.session_created" => AuditEventType::SessionCreated,
            "auth.session_revoked" => AuditEventType::SessionRevoked,
            "auth.password_changed" => AuditEventType::PasswordChanged,
            "auth.mfa_enabled" => AuditEventType::MfaEnabled,
            "auth.mfa_disabled" => AuditEventType::MfaDisabled,
            "auth.mfa_verified" => AuditEventType::MfaVerified,
            "crud.create" => AuditEventType::Create,
            "crud.read" => AuditEventType::Read,
            "crud.update" => AuditEventType::Update,
            "crud.delete" => AuditEventType::Delete,
            "permission.granted" => AuditEventType::PermissionGranted,
            "permission.revoked" => AuditEventType::PermissionRevoked,
            "permission.role_assigned" => AuditEventType::RoleAssigned,
            "permission.role_removed" => AuditEventType::RoleRemoved,
            "content.publish" => AuditEventType::Publish,
            "content.unpublish" => AuditEventType::Unpublish,
            "content.archive" => AuditEventType::Archive,
            "content.restore" => AuditEventType::Restore,
            "finance.payment_initiated" => AuditEventType::PaymentInitiated,
            "finance.payment_completed" => AuditEventType::PaymentCompleted,
            "finance.payment_failed" => AuditEventType::PaymentFailed,
            "finance.refund_initiated" => AuditEventType::RefundInitiated,
            "finance.refund_completed" => AuditEventType::RefundCompleted,
            "system.config_changed" => AuditEventType::ConfigChanged,
            "system.maintenance_started" => AuditEventType::MaintenanceStarted,
            "system.maintenance_ended" => AuditEventType::MaintenanceEnded,
            "system.feature_toggled" => AuditEventType::FeatureToggled,
            "privacy.consent_recorded" => AuditEventType::ConsentRecorded,
            "privacy.consent_withdrawn" => AuditEventType::ConsentWithdrawn,
            "privacy.data_request_created" => AuditEventType::DataRequestCreated,
            "privacy.data_request_updated" => AuditEventType::DataRequestUpdated,
            "privacy.data_request_completed" => AuditEventType::DataRequestCompleted,
            other => AuditEventType::Custom(other.to_string()),
        }
    }
}

/// Actor type - who performed the action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    User,
    System,
    Worker,
    Admin,
    Service,
}

impl ActorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActorType::User => "user",
            ActorType::System => "system",
            ActorType::Worker => "worker",
            ActorType::Admin => "admin",
            ActorType::Service => "service",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "user" => Some(ActorType::User),
            "system" => Some(ActorType::System),
            "worker" => Some(ActorType::Worker),
            "admin" => Some(ActorType::Admin),
            "service" => Some(ActorType::Service),
            _ => None,
        }
    }
}

/// Outcome of the audit event
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditOutcome {
    Success,
    Failure,
    Denied,
    Error,
}

impl AuditOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditOutcome::Success => "success",
            AuditOutcome::Failure => "failure",
            AuditOutcome::Denied => "denied",
            AuditOutcome::Error => "error",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "success" => Some(AuditOutcome::Success),
            "failure" => Some(AuditOutcome::Failure),
            "denied" => Some(AuditOutcome::Denied),
            "error" => Some(AuditOutcome::Error),
            _ => None,
        }
    }
}

// ============================================
// ENTITY STRUCTS
// ============================================

/// Audit event for creating new log entries (input type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Event type (e.g., "auth.login", "crud.create")
    pub event_type: AuditEventType,
    /// Event category for classification
    pub event_category: AuditCategory,

    /// Type of actor performing the action
    pub actor_type: ActorType,
    /// Actor identifier (user ID, system name, etc.)
    pub actor_id: Option<String>,
    /// Actor's IP address
    pub actor_ip: Option<IpAddr>,
    /// Actor's user agent string
    pub actor_user_agent: Option<String>,

    /// Type of resource being acted upon
    pub resource_type: Option<String>,
    /// Resource identifier
    pub resource_id: Option<String>,

    /// Action performed (e.g., "create", "update", "delete")
    pub action: String,
    /// Outcome of the action
    pub outcome: AuditOutcome,
    /// Reason for the outcome (especially for failures)
    pub outcome_reason: Option<String>,

    /// Additional event data (changes, context, etc.)
    pub event_data: Option<JsonValue>,
}

impl AuditEvent {
    /// Create a new audit event builder
    pub fn builder(event_type: AuditEventType, category: AuditCategory) -> AuditEventBuilder {
        AuditEventBuilder::new(event_type, category)
    }

    /// Quick constructor for successful user action
    pub fn user_action(
        event_type: AuditEventType,
        category: AuditCategory,
        actor_id: &str,
        action: &str,
    ) -> Self {
        Self {
            event_type,
            event_category: category,
            actor_type: ActorType::User,
            actor_id: Some(actor_id.to_string()),
            actor_ip: None,
            actor_user_agent: None,
            resource_type: None,
            resource_id: None,
            action: action.to_string(),
            outcome: AuditOutcome::Success,
            outcome_reason: None,
            event_data: None,
        }
    }

    /// Quick constructor for system event
    pub fn system_event(
        event_type: AuditEventType,
        category: AuditCategory,
        action: &str,
    ) -> Self {
        Self {
            event_type,
            event_category: category,
            actor_type: ActorType::System,
            actor_id: Some("system".to_string()),
            actor_ip: None,
            actor_user_agent: None,
            resource_type: None,
            resource_id: None,
            action: action.to_string(),
            outcome: AuditOutcome::Success,
            outcome_reason: None,
            event_data: None,
        }
    }
}

/// Builder for constructing audit events
#[derive(Debug, Clone)]
pub struct AuditEventBuilder {
    event_type: AuditEventType,
    event_category: AuditCategory,
    actor_type: ActorType,
    actor_id: Option<String>,
    actor_ip: Option<IpAddr>,
    actor_user_agent: Option<String>,
    resource_type: Option<String>,
    resource_id: Option<String>,
    action: String,
    outcome: AuditOutcome,
    outcome_reason: Option<String>,
    event_data: Option<JsonValue>,
}

impl AuditEventBuilder {
    pub fn new(event_type: AuditEventType, category: AuditCategory) -> Self {
        Self {
            event_type,
            event_category: category,
            actor_type: ActorType::User,
            actor_id: None,
            actor_ip: None,
            actor_user_agent: None,
            resource_type: None,
            resource_id: None,
            action: String::new(),
            outcome: AuditOutcome::Success,
            outcome_reason: None,
            event_data: None,
        }
    }

    pub fn actor(mut self, actor_type: ActorType, actor_id: impl Into<String>) -> Self {
        self.actor_type = actor_type;
        self.actor_id = Some(actor_id.into());
        self
    }

    pub fn actor_ip(mut self, ip: IpAddr) -> Self {
        self.actor_ip = Some(ip);
        self
    }

    /// Set actor IP from optional string (convenience method)
    pub fn ip_address(mut self, ip: Option<String>) -> Self {
        self.actor_ip = ip.and_then(|s| s.parse().ok());
        self
    }

    pub fn actor_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.actor_user_agent = Some(user_agent.into());
        self
    }

    pub fn resource(mut self, resource_type: impl Into<String>, resource_id: impl Into<String>) -> Self {
        self.resource_type = Some(resource_type.into());
        self.resource_id = Some(resource_id.into());
        self
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = action.into();
        self
    }

    pub fn outcome(mut self, outcome: AuditOutcome) -> Self {
        self.outcome = outcome;
        self
    }

    pub fn outcome_with_reason(mut self, outcome: AuditOutcome, reason: impl Into<String>) -> Self {
        self.outcome = outcome;
        self.outcome_reason = Some(reason.into());
        self
    }

    pub fn event_data(mut self, data: JsonValue) -> Self {
        self.event_data = Some(data);
        self
    }

    pub fn build(self) -> AuditEvent {
        AuditEvent {
            event_type: self.event_type,
            event_category: self.event_category,
            actor_type: self.actor_type,
            actor_id: self.actor_id,
            actor_ip: self.actor_ip,
            actor_user_agent: self.actor_user_agent,
            resource_type: self.resource_type,
            resource_id: self.resource_id,
            action: self.action,
            outcome: self.outcome,
            outcome_reason: self.outcome_reason,
            event_data: self.event_data,
        }
    }
}

/// Internal row type for database mapping (with IpNetwork)
#[derive(Debug, Clone, sqlx::FromRow)]
struct ImmutableAuditLogRow {
    id: i64,
    event_type: String,
    event_category: String,
    actor_type: String,
    actor_id: Option<String>,
    actor_ip: Option<ipnetwork::IpNetwork>,
    actor_user_agent: Option<String>,
    resource_type: Option<String>,
    resource_id: Option<String>,
    action: String,
    outcome: String,
    outcome_reason: Option<String>,
    #[sqlx(json)]
    event_data: Option<JsonValue>,
    occurred_at: DateTime<Utc>,
    is_immutable: bool,
}

/// Immutable audit log entry read from the database (output type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmutableAuditLogEntry {
    pub id: i64,
    pub event_type: String,
    pub event_category: String,
    pub actor_type: String,
    pub actor_id: Option<String>,
    pub actor_ip: Option<IpAddr>,
    pub actor_user_agent: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub action: String,
    pub outcome: String,
    pub outcome_reason: Option<String>,
    pub event_data: Option<JsonValue>,
    pub occurred_at: DateTime<Utc>,
    pub is_immutable: bool,
}

impl From<ImmutableAuditLogRow> for ImmutableAuditLogEntry {
    fn from(row: ImmutableAuditLogRow) -> Self {
        Self {
            id: row.id,
            event_type: row.event_type,
            event_category: row.event_category,
            actor_type: row.actor_type,
            actor_id: row.actor_id,
            actor_ip: row.actor_ip.map(|n| n.ip()),
            actor_user_agent: row.actor_user_agent,
            resource_type: row.resource_type,
            resource_id: row.resource_id,
            action: row.action,
            outcome: row.outcome,
            outcome_reason: row.outcome_reason,
            event_data: row.event_data,
            occurred_at: row.occurred_at,
            is_immutable: row.is_immutable,
        }
    }
}

impl ImmutableAuditLogEntry {
    /// Parse event type enum
    pub fn event_type_enum(&self) -> AuditEventType {
        AuditEventType::from_str(&self.event_type)
    }

    /// Parse event category enum
    pub fn event_category_enum(&self) -> Option<AuditCategory> {
        AuditCategory::from_str(&self.event_category)
    }

    /// Parse actor type enum
    pub fn actor_type_enum(&self) -> Option<ActorType> {
        ActorType::from_str(&self.actor_type)
    }

    /// Parse outcome enum
    pub fn outcome_enum(&self) -> Option<AuditOutcome> {
        AuditOutcome::from_str(&self.outcome)
    }
}

// ============================================
// IMMUTABLE AUDIT REPOSITORY TRAIT
// ============================================

/// Immutable audit repository trait for compliance-focused audit logging
/// Uses the audit_log table from migration 00020_identity_sessions.up.sql
/// Events are append-only - no updates or deletes are supported.
#[async_trait]
pub trait ImmutableAuditRepository: Send + Sync {
    /// Log a new audit event (append-only)
    async fn log_event(&self, event: AuditEvent) -> Result<ImmutableAuditLogEntry, AppError>;

    /// Get audit events by actor ID with pagination
    async fn get_events_by_actor(
        &self,
        actor_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ImmutableAuditLogEntry>, AppError>;

    /// Get audit events by resource with pagination
    async fn get_events_by_resource(
        &self,
        resource_type: &str,
        resource_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ImmutableAuditLogEntry>, AppError>;

    /// Get audit events by event type within a time range
    async fn get_events_by_type(
        &self,
        event_type: &AuditEventType,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<ImmutableAuditLogEntry>, AppError>;

    /// Get audit events by category within a time range
    async fn get_events_by_category(
        &self,
        category: AuditCategory,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<ImmutableAuditLogEntry>, AppError>;
}

// ============================================
// IMMUTABLE AUDIT REPOSITORY IMPLEMENTATION
// ============================================

/// PostgreSQL implementation of the immutable audit repository
pub struct ImmutableAuditRepositoryImpl {
    pool: ConnectionPool,
}

impl ImmutableAuditRepositoryImpl {
    /// Create a new immutable audit repository instance
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ImmutableAuditRepository for ImmutableAuditRepositoryImpl {
    async fn log_event(&self, event: AuditEvent) -> Result<ImmutableAuditLogEntry, AppError> {
        let row = sqlx::query_as::<_, ImmutableAuditLogRow>(
            r#"
            INSERT INTO audit_log (
                event_type, event_category,
                actor_type, actor_id, actor_ip, actor_user_agent,
                resource_type, resource_id,
                action, outcome, outcome_reason,
                event_data
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING
                id, event_type, event_category,
                actor_type, actor_id, actor_ip, actor_user_agent,
                resource_type, resource_id,
                action, outcome, outcome_reason,
                event_data, occurred_at, is_immutable
            "#,
        )
        .bind(event.event_type.as_str())
        .bind(event.event_category.as_str())
        .bind(event.actor_type.as_str())
        .bind(&event.actor_id)
        .bind(event.actor_ip.map(ipnetwork::IpNetwork::from))
        .bind(&event.actor_user_agent)
        .bind(&event.resource_type)
        .bind(&event.resource_id)
        .bind(&event.action)
        .bind(event.outcome.as_str())
        .bind(&event.outcome_reason)
        .bind(&event.event_data)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn get_events_by_actor(
        &self,
        actor_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ImmutableAuditLogEntry>, AppError> {
        let rows = sqlx::query_as::<_, ImmutableAuditLogRow>(
            r#"
            SELECT
                id, event_type, event_category,
                actor_type, actor_id, actor_ip, actor_user_agent,
                resource_type, resource_id,
                action, outcome, outcome_reason,
                event_data, occurred_at, is_immutable
            FROM audit_log
            WHERE actor_id = $1
            ORDER BY occurred_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(actor_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn get_events_by_resource(
        &self,
        resource_type: &str,
        resource_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ImmutableAuditLogEntry>, AppError> {
        let rows = sqlx::query_as::<_, ImmutableAuditLogRow>(
            r#"
            SELECT
                id, event_type, event_category,
                actor_type, actor_id, actor_ip, actor_user_agent,
                resource_type, resource_id,
                action, outcome, outcome_reason,
                event_data, occurred_at, is_immutable
            FROM audit_log
            WHERE resource_type = $1 AND resource_id = $2
            ORDER BY occurred_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(resource_type)
        .bind(resource_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn get_events_by_type(
        &self,
        event_type: &AuditEventType,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<ImmutableAuditLogEntry>, AppError> {
        let rows = sqlx::query_as::<_, ImmutableAuditLogRow>(
            r#"
            SELECT
                id, event_type, event_category,
                actor_type, actor_id, actor_ip, actor_user_agent,
                resource_type, resource_id,
                action, outcome, outcome_reason,
                event_data, occurred_at, is_immutable
            FROM audit_log
            WHERE event_type = $1
              AND occurred_at >= $2
              AND occurred_at <= $3
            ORDER BY occurred_at DESC
            LIMIT $4
            "#,
        )
        .bind(event_type.as_str())
        .bind(start_time)
        .bind(end_time)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn get_events_by_category(
        &self,
        category: AuditCategory,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<ImmutableAuditLogEntry>, AppError> {
        let rows = sqlx::query_as::<_, ImmutableAuditLogRow>(
            r#"
            SELECT
                id, event_type, event_category,
                actor_type, actor_id, actor_ip, actor_user_agent,
                resource_type, resource_id,
                action, outcome, outcome_reason,
                event_data, occurred_at, is_immutable
            FROM audit_log
            WHERE event_category = $1
              AND occurred_at >= $2
              AND occurred_at <= $3
            ORDER BY occurred_at DESC
            LIMIT $4
            "#,
        )
        .bind(category.as_str())
        .bind(start_time)
        .bind(end_time)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_category_roundtrip() {
        let categories = [
            AuditCategory::Auth,
            AuditCategory::User,
            AuditCategory::Permission,
            AuditCategory::Content,
            AuditCategory::Finance,
            AuditCategory::System,
        ];

        for cat in categories {
            let s = cat.as_str();
            let parsed = AuditCategory::from_str(s);
            assert_eq!(parsed, Some(cat));
        }
    }

    #[test]
    fn test_audit_event_type_roundtrip() {
        let types = [
            AuditEventType::Login,
            AuditEventType::Logout,
            AuditEventType::Create,
            AuditEventType::Update,
            AuditEventType::Delete,
            AuditEventType::PermissionGranted,
        ];

        for t in types {
            let s = t.as_str();
            let parsed = AuditEventType::from_str(&s);
            assert_eq!(parsed, t);
        }
    }

    #[test]
    fn test_custom_event_type() {
        let custom = AuditEventType::Custom("custom.event".to_string());
        let s = custom.as_str();
        assert_eq!(s, "custom.event");

        let parsed = AuditEventType::from_str("custom.event");
        assert_eq!(parsed, AuditEventType::Custom("custom.event".to_string()));
    }

    #[test]
    fn test_audit_event_builder() {
        let event = AuditEvent::builder(AuditEventType::Login, AuditCategory::Auth)
            .actor(ActorType::User, "user-123")
            .action("login")
            .outcome(AuditOutcome::Success)
            .build();

        assert_eq!(event.event_type, AuditEventType::Login);
        assert_eq!(event.event_category, AuditCategory::Auth);
        assert_eq!(event.actor_type, ActorType::User);
        assert_eq!(event.actor_id, Some("user-123".to_string()));
        assert_eq!(event.action, "login");
        assert_eq!(event.outcome, AuditOutcome::Success);
    }

    #[test]
    fn test_audit_event_builder_with_resource() {
        let event = AuditEvent::builder(AuditEventType::Update, AuditCategory::Content)
            .actor(ActorType::Admin, "admin-456")
            .resource("article", "article-789")
            .action("update")
            .outcome_with_reason(AuditOutcome::Success, "Content approved")
            .event_data(serde_json::json!({"field": "title", "old": "Old Title", "new": "New Title"}))
            .build();

        assert_eq!(event.resource_type, Some("article".to_string()));
        assert_eq!(event.resource_id, Some("article-789".to_string()));
        assert_eq!(event.outcome_reason, Some("Content approved".to_string()));
        assert!(event.event_data.is_some());
    }

    #[test]
    fn test_user_action_helper() {
        let event = AuditEvent::user_action(
            AuditEventType::Create,
            AuditCategory::Content,
            "user-123",
            "create",
        );

        assert_eq!(event.actor_type, ActorType::User);
        assert_eq!(event.actor_id, Some("user-123".to_string()));
        assert_eq!(event.outcome, AuditOutcome::Success);
    }

    #[test]
    fn test_system_event_helper() {
        let event = AuditEvent::system_event(
            AuditEventType::MaintenanceStarted,
            AuditCategory::System,
            "maintenance_start",
        );

        assert_eq!(event.actor_type, ActorType::System);
        assert_eq!(event.actor_id, Some("system".to_string()));
    }
}
