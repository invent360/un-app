//! Communication repository for user preferences and messaging
//!
//! Implements CRUD operations for:
//! - User communication preferences
//! - Suppression lists
//! - Message templates
//! - Scheduled messages
//! - Device tokens

use async_trait::async_trait;
use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Communication channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "comm_channel", rename_all = "snake_case")]
pub enum CommChannel {
    Email,
    Push,
    Sms,
    InApp,
}

impl std::fmt::Display for CommChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommChannel::Email => write!(f, "email"),
            CommChannel::Push => write!(f, "push"),
            CommChannel::Sms => write!(f, "sms"),
            CommChannel::InApp => write!(f, "in_app"),
        }
    }
}

/// Message category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "message_category", rename_all = "lowercase")]
pub enum MessageCategory {
    Transactional,
    Marketing,
    Support,
    Cohort,
    System,
}

/// Suppression type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "suppression_type", rename_all = "lowercase")]
pub enum SuppressionType {
    Bounce,
    Complaint,
    Unsubscribe,
    Manual,
    Invalid,
}

/// Template status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "template_status", rename_all = "snake_case")]
pub enum TemplateStatus {
    Draft,
    PendingApproval,
    Approved,
    Archived,
}

/// Scheduled message status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "scheduled_message_status", rename_all = "lowercase")]
pub enum ScheduledMessageStatus {
    Scheduled,
    Sending,
    Sent,
    Failed,
    Cancelled,
}

/// User communication preferences
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommunicationPreferences {
    pub id: Uuid,
    pub user_id: String,

    // Channel preferences
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub sms_enabled: bool,
    pub in_app_enabled: bool,

    // Category preferences
    pub transactional_enabled: bool,
    pub marketing_enabled: bool,
    pub support_enabled: bool,
    pub cohort_reminders_enabled: bool,
    pub system_enabled: bool,

    // Frequency preferences
    pub digest_frequency: Option<String>,
    pub max_messages_per_day: Option<i32>,

    // Timezone and quiet hours
    pub timezone: String,
    pub quiet_hours_enabled: bool,
    pub quiet_hours_start: Option<NaiveTime>,
    pub quiet_hours_end: Option<NaiveTime>,

    // Locale
    pub preferred_locale: String,

    // Global opt-out
    pub global_opt_out: bool,
    pub opt_out_reason: Option<String>,
    pub opted_out_at: Option<DateTime<Utc>>,

    // Verification
    pub email_verified: bool,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub phone_verified: bool,
    pub phone_verified_at: Option<DateTime<Utc>>,

    // Contact info
    pub notification_email: Option<String>,
    pub notification_phone: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Communication suppression entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommunicationSuppression {
    pub id: Uuid,
    pub channel: CommChannel,
    pub identifier: String,
    pub identifier_hash: String,

    pub suppression_type: SuppressionType,
    pub source: String,

    pub reason: Option<String>,
    pub bounce_type: Option<String>,
    pub complaint_type: Option<String>,
    pub diagnostic_code: Option<String>,

    pub permanent: bool,
    pub user_id: Option<String>,

    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,

    pub external_id: Option<String>,
    pub external_timestamp: Option<DateTime<Utc>>,
}

/// Message template
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessageTemplate {
    pub id: Uuid,
    pub template_code: String,
    pub version: i32,
    pub channel: CommChannel,
    pub locale: String,

    // Content
    pub subject: Option<String>,
    pub title: Option<String>,
    pub body_template: String,
    pub html_template: Option<String>,
    pub action_url: Option<String>,

    // Template engine
    pub template_engine: String,

    // Metadata
    pub category: MessageCategory,
    pub description: Option<String>,
    pub variables: Vec<String>,
    pub sample_data: Option<serde_json::Value>,

    // Status and approval
    pub status: TemplateStatus,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,

    // Usage tracking
    pub usage_count: i32,
    pub last_used_at: Option<DateTime<Utc>>,

    // Ownership
    pub created_by: String,
    pub updated_by: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Scheduled message
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ScheduledMessage {
    pub id: Uuid,
    pub user_id: String,
    pub channel: CommChannel,

    // Template reference
    pub template_id: Option<Uuid>,
    pub template_code: Option<String>,
    pub template_version: Option<i32>,

    // Recipient
    pub recipient_address: String,
    pub recipient_name: Option<String>,

    // Content
    pub subject: Option<String>,
    pub title: Option<String>,
    pub body: String,
    pub html_body: Option<String>,
    pub action_url: Option<String>,

    // Template data
    pub template_data: Option<serde_json::Value>,

    // Scheduling
    pub scheduled_at: DateTime<Utc>,
    pub local_time_preference: bool,
    pub user_timezone: Option<String>,

    // Category
    pub category: MessageCategory,

    // Priority
    pub priority: i32,

    // Status
    pub status: ScheduledMessageStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub external_id: Option<String>,

    // Error handling
    pub attempt_count: i32,
    pub max_attempts: i32,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub next_retry_at: Option<DateTime<Utc>>,

    // Correlation
    pub correlation_id: Option<String>,
    pub triggered_by: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,

    pub created_at: DateTime<Utc>,
}

/// Device token for push notifications
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeviceToken {
    pub id: Uuid,
    pub user_id: String,

    pub token: String,
    pub token_hash: String,
    pub platform: String,
    pub platform_version: Option<String>,

    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub app_version: Option<String>,

    pub active: bool,
    pub invalidated_at: Option<DateTime<Utc>>,
    pub invalidation_reason: Option<String>,

    pub last_used_at: Option<DateTime<Utc>>,
    pub last_success_at: Option<DateTime<Utc>>,
    pub last_failure_at: Option<DateTime<Utc>>,
    pub consecutive_failures: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================
// INPUT STRUCTS
// ============================================

/// Input for updating preferences
#[derive(Debug, Clone)]
pub struct UpdatePreferencesInput {
    pub email_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,
    pub in_app_enabled: Option<bool>,
    pub marketing_enabled: Option<bool>,
    pub support_enabled: Option<bool>,
    pub cohort_reminders_enabled: Option<bool>,
    pub digest_frequency: Option<String>,
    pub max_messages_per_day: Option<i32>,
    pub timezone: Option<String>,
    pub quiet_hours_enabled: Option<bool>,
    pub quiet_hours_start: Option<NaiveTime>,
    pub quiet_hours_end: Option<NaiveTime>,
    pub preferred_locale: Option<String>,
    pub notification_email: Option<String>,
    pub notification_phone: Option<String>,
}

/// Input for adding suppression
#[derive(Debug, Clone)]
pub struct AddSuppressionInput {
    pub channel: CommChannel,
    pub identifier: String,
    pub suppression_type: SuppressionType,
    pub source: String,
    pub reason: Option<String>,
    pub bounce_type: Option<String>,
    pub complaint_type: Option<String>,
    pub permanent: bool,
    pub user_id: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Input for creating a template
#[derive(Debug, Clone)]
pub struct CreateTemplateInput {
    pub template_code: String,
    pub channel: CommChannel,
    pub locale: String,
    pub subject: Option<String>,
    pub title: Option<String>,
    pub body_template: String,
    pub html_template: Option<String>,
    pub action_url: Option<String>,
    pub template_engine: String,
    pub category: MessageCategory,
    pub description: Option<String>,
    pub variables: Vec<String>,
    pub sample_data: Option<serde_json::Value>,
    pub created_by: String,
}

/// Input for updating a template
#[derive(Debug, Clone)]
pub struct UpdateTemplateInput {
    pub subject: Option<String>,
    pub title: Option<String>,
    pub body_template: Option<String>,
    pub html_template: Option<String>,
    pub action_url: Option<String>,
    pub description: Option<String>,
    pub variables: Option<Vec<String>>,
    pub sample_data: Option<serde_json::Value>,
    pub updated_by: String,
}

/// Input for scheduling a message
#[derive(Debug, Clone)]
pub struct ScheduleMessageInput {
    pub user_id: String,
    pub channel: CommChannel,
    pub category: MessageCategory,
    pub recipient_address: String,
    pub recipient_name: Option<String>,
    pub subject: Option<String>,
    pub title: Option<String>,
    pub body: String,
    pub html_body: Option<String>,
    pub action_url: Option<String>,
    pub template_id: Option<Uuid>,
    pub template_code: Option<String>,
    pub template_data: Option<serde_json::Value>,
    pub scheduled_at: DateTime<Utc>,
    pub local_time_preference: bool,
    pub priority: i32,
    pub correlation_id: Option<String>,
    pub triggered_by: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
}

/// Input for registering device token
#[derive(Debug, Clone)]
pub struct RegisterDeviceInput {
    pub user_id: String,
    pub token: String,
    pub platform: String,
    pub platform_version: Option<String>,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub app_version: Option<String>,
}

/// Can receive result
#[derive(Debug, Clone, Serialize)]
pub struct CanReceiveResult {
    pub can_send: bool,
    pub reason: Option<String>,
    pub retry_after: Option<NaiveTime>,
}

/// Send statistics
#[derive(Debug, Clone, Serialize)]
pub struct SendStats {
    pub scheduled: i64,
    pub sent: i64,
    pub failed: i64,
    pub cancelled: i64,
}

// ============================================
// TRAIT DEFINITION
// ============================================

/// Dynamic type alias for CommunicationRepository trait object
pub type DynCommunicationRepository = Arc<dyn CommunicationRepository + Send + Sync>;

/// Communication repository trait
#[async_trait]
pub trait CommunicationRepository: Send + Sync {
    // --- Preferences ---

    /// Get or create preferences for user
    async fn get_or_create_preferences(&self, user_id: &str) -> Result<CommunicationPreferences, AppError>;

    /// Get preferences
    async fn get_preferences(&self, user_id: &str) -> Result<Option<CommunicationPreferences>, AppError>;

    /// Update preferences
    async fn update_preferences(&self, user_id: &str, input: UpdatePreferencesInput) -> Result<CommunicationPreferences, AppError>;

    /// Opt out globally
    async fn opt_out(&self, user_id: &str, reason: Option<&str>) -> Result<CommunicationPreferences, AppError>;

    /// Verify email
    async fn verify_email(&self, user_id: &str) -> Result<CommunicationPreferences, AppError>;

    /// Verify phone
    async fn verify_phone(&self, user_id: &str) -> Result<CommunicationPreferences, AppError>;

    /// Check if user can receive message (uses database function)
    async fn can_receive_message(&self, user_id: &str, channel: CommChannel, category: MessageCategory,
                                  recipient: &str) -> Result<CanReceiveResult, AppError>;

    // --- Suppressions ---

    /// Add suppression (uses database function)
    async fn add_suppression(&self, input: AddSuppressionInput) -> Result<CommunicationSuppression, AppError>;

    /// Check if identifier is suppressed
    async fn is_suppressed(&self, channel: CommChannel, identifier: &str) -> Result<bool, AppError>;

    /// Get suppression for identifier
    async fn get_suppression(&self, channel: CommChannel, identifier: &str) -> Result<Option<CommunicationSuppression>, AppError>;

    /// Remove suppression
    async fn remove_suppression(&self, channel: CommChannel, identifier: &str) -> Result<(), AppError>;

    /// List suppressions
    async fn list_suppressions(&self, channel: Option<CommChannel>, limit: i32) -> Result<Vec<CommunicationSuppression>, AppError>;

    // --- Templates ---

    /// Create template
    async fn create_template(&self, input: CreateTemplateInput) -> Result<MessageTemplate, AppError>;

    /// Get template by code/channel/locale
    async fn get_template(&self, code: &str, channel: CommChannel, locale: &str) -> Result<Option<MessageTemplate>, AppError>;

    /// Get template by ID
    async fn get_template_by_id(&self, id: Uuid) -> Result<Option<MessageTemplate>, AppError>;

    /// List templates
    async fn list_templates(&self, channel: Option<CommChannel>, category: Option<MessageCategory>) -> Result<Vec<MessageTemplate>, AppError>;

    /// Update template
    async fn update_template(&self, id: Uuid, input: UpdateTemplateInput) -> Result<MessageTemplate, AppError>;

    /// Submit for approval
    async fn submit_template_for_approval(&self, id: Uuid) -> Result<MessageTemplate, AppError>;

    /// Approve template
    async fn approve_template(&self, id: Uuid, approved_by: &str) -> Result<MessageTemplate, AppError>;

    /// Reject template
    async fn reject_template(&self, id: Uuid, reason: &str) -> Result<MessageTemplate, AppError>;

    /// Archive template
    async fn archive_template(&self, id: Uuid) -> Result<MessageTemplate, AppError>;

    /// Increment usage count
    async fn increment_template_usage(&self, id: Uuid) -> Result<(), AppError>;

    // --- Scheduled Messages ---

    /// Schedule message
    async fn schedule_message(&self, input: ScheduleMessageInput) -> Result<ScheduledMessage, AppError>;

    /// Get pending messages
    async fn get_pending_messages(&self, limit: i32) -> Result<Vec<ScheduledMessage>, AppError>;

    /// Get message by ID
    async fn get_message(&self, id: Uuid) -> Result<Option<ScheduledMessage>, AppError>;

    /// Get user messages
    async fn get_user_messages(&self, user_id: &str, limit: i32) -> Result<Vec<ScheduledMessage>, AppError>;

    /// Mark message sending
    async fn mark_message_sending(&self, id: Uuid) -> Result<ScheduledMessage, AppError>;

    /// Mark message sent
    async fn mark_message_sent(&self, id: Uuid, external_id: Option<&str>) -> Result<ScheduledMessage, AppError>;

    /// Mark message failed
    async fn mark_message_failed(&self, id: Uuid, error_code: &str, error_message: &str) -> Result<ScheduledMessage, AppError>;

    /// Cancel message
    async fn cancel_message(&self, id: Uuid) -> Result<ScheduledMessage, AppError>;

    /// Get send stats
    async fn get_send_stats(&self) -> Result<SendStats, AppError>;

    // --- Device Tokens ---

    /// Register device token
    async fn register_device(&self, input: RegisterDeviceInput) -> Result<DeviceToken, AppError>;

    /// Get user device tokens
    async fn get_user_devices(&self, user_id: &str) -> Result<Vec<DeviceToken>, AppError>;

    /// Invalidate device token
    async fn invalidate_device(&self, token_hash: &str, reason: &str) -> Result<(), AppError>;

    /// Record push success
    async fn record_push_success(&self, token_hash: &str) -> Result<(), AppError>;

    /// Record push failure
    async fn record_push_failure(&self, token_hash: &str) -> Result<(), AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct CommunicationRepositoryImpl {
    pool: ConnectionPool,
}

impl CommunicationRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }

    /// Hash an identifier for lookup
    fn hash_identifier(identifier: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(identifier.to_lowercase().as_bytes());
        hex::encode(hasher.finalize())
    }
}

#[async_trait]
impl CommunicationRepository for CommunicationRepositoryImpl {
    async fn get_or_create_preferences(&self, user_id: &str) -> Result<CommunicationPreferences, AppError> {
        let prefs = sqlx::query_as::<_, CommunicationPreferences>(
            r#"
            INSERT INTO communication_preferences (user_id)
            VALUES ($1)
            ON CONFLICT (user_id) DO UPDATE SET updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(prefs)
    }

    async fn get_preferences(&self, user_id: &str) -> Result<Option<CommunicationPreferences>, AppError> {
        let prefs = sqlx::query_as::<_, CommunicationPreferences>(
            "SELECT * FROM communication_preferences WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(prefs)
    }

    async fn update_preferences(&self, user_id: &str, input: UpdatePreferencesInput) -> Result<CommunicationPreferences, AppError> {
        let prefs = sqlx::query_as::<_, CommunicationPreferences>(
            r#"
            UPDATE communication_preferences SET
                email_enabled = COALESCE($2, email_enabled),
                push_enabled = COALESCE($3, push_enabled),
                sms_enabled = COALESCE($4, sms_enabled),
                in_app_enabled = COALESCE($5, in_app_enabled),
                marketing_enabled = COALESCE($6, marketing_enabled),
                support_enabled = COALESCE($7, support_enabled),
                cohort_reminders_enabled = COALESCE($8, cohort_reminders_enabled),
                digest_frequency = COALESCE($9, digest_frequency),
                max_messages_per_day = COALESCE($10, max_messages_per_day),
                timezone = COALESCE($11, timezone),
                quiet_hours_enabled = COALESCE($12, quiet_hours_enabled),
                quiet_hours_start = COALESCE($13, quiet_hours_start),
                quiet_hours_end = COALESCE($14, quiet_hours_end),
                preferred_locale = COALESCE($15, preferred_locale),
                notification_email = COALESCE($16, notification_email),
                notification_phone = COALESCE($17, notification_phone),
                updated_at = NOW()
            WHERE user_id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(input.email_enabled)
        .bind(input.push_enabled)
        .bind(input.sms_enabled)
        .bind(input.in_app_enabled)
        .bind(input.marketing_enabled)
        .bind(input.support_enabled)
        .bind(input.cohort_reminders_enabled)
        .bind(&input.digest_frequency)
        .bind(input.max_messages_per_day)
        .bind(&input.timezone)
        .bind(input.quiet_hours_enabled)
        .bind(input.quiet_hours_start)
        .bind(input.quiet_hours_end)
        .bind(&input.preferred_locale)
        .bind(&input.notification_email)
        .bind(&input.notification_phone)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(prefs)
    }

    async fn opt_out(&self, user_id: &str, reason: Option<&str>) -> Result<CommunicationPreferences, AppError> {
        let prefs = sqlx::query_as::<_, CommunicationPreferences>(
            r#"
            UPDATE communication_preferences
            SET global_opt_out = TRUE, opt_out_reason = $2, opted_out_at = NOW(), updated_at = NOW()
            WHERE user_id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(reason)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(prefs)
    }

    async fn verify_email(&self, user_id: &str) -> Result<CommunicationPreferences, AppError> {
        let prefs = sqlx::query_as::<_, CommunicationPreferences>(
            r#"
            UPDATE communication_preferences
            SET email_verified = TRUE, email_verified_at = NOW(), updated_at = NOW()
            WHERE user_id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(prefs)
    }

    async fn verify_phone(&self, user_id: &str) -> Result<CommunicationPreferences, AppError> {
        let prefs = sqlx::query_as::<_, CommunicationPreferences>(
            r#"
            UPDATE communication_preferences
            SET phone_verified = TRUE, phone_verified_at = NOW(), updated_at = NOW()
            WHERE user_id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(prefs)
    }

    async fn can_receive_message(
        &self,
        user_id: &str,
        channel: CommChannel,
        category: MessageCategory,
        recipient: &str,
    ) -> Result<CanReceiveResult, AppError> {
        let (result,): (serde_json::Value,) = sqlx::query_as("SELECT can_receive_message($1, $2, $3, $4)")
            .bind(user_id)
            .bind(channel)
            .bind(category)
            .bind(recipient)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(CanReceiveResult {
            can_send: result["can_send"].as_bool().unwrap_or(false),
            reason: result["reason"].as_str().map(|s| s.to_string()),
            retry_after: None, // Parse from result if present
        })
    }

    async fn add_suppression(&self, input: AddSuppressionInput) -> Result<CommunicationSuppression, AppError> {
        let (id,): (Uuid,) = sqlx::query_as("SELECT add_suppression($1, $2, $3, $4, $5, $6, $7)")
            .bind(input.channel)
            .bind(&input.identifier)
            .bind(input.suppression_type)
            .bind(&input.source)
            .bind(&input.reason)
            .bind(input.permanent)
            .bind(&input.user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.get_suppression(input.channel, &input.identifier)
            .await?
            .ok_or_else(|| AppError::InternalServerError("Failed to add suppression".to_string()))
    }

    async fn is_suppressed(&self, channel: CommChannel, identifier: &str) -> Result<bool, AppError> {
        let hash = Self::hash_identifier(identifier);

        let (exists,): (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM communication_suppressions
                WHERE channel = $1 AND identifier_hash = $2
                  AND (permanent = TRUE OR expires_at IS NULL OR expires_at > NOW())
            )
            "#,
        )
        .bind(channel)
        .bind(&hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(exists)
    }

    async fn get_suppression(&self, channel: CommChannel, identifier: &str) -> Result<Option<CommunicationSuppression>, AppError> {
        let hash = Self::hash_identifier(identifier);

        let suppression = sqlx::query_as::<_, CommunicationSuppression>(
            "SELECT * FROM communication_suppressions WHERE channel = $1 AND identifier_hash = $2",
        )
        .bind(channel)
        .bind(&hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(suppression)
    }

    async fn remove_suppression(&self, channel: CommChannel, identifier: &str) -> Result<(), AppError> {
        let hash = Self::hash_identifier(identifier);

        sqlx::query("DELETE FROM communication_suppressions WHERE channel = $1 AND identifier_hash = $2")
            .bind(channel)
            .bind(&hash)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn list_suppressions(&self, channel: Option<CommChannel>, limit: i32) -> Result<Vec<CommunicationSuppression>, AppError> {
        let suppressions = if let Some(ch) = channel {
            sqlx::query_as::<_, CommunicationSuppression>(
                r#"
                SELECT * FROM communication_suppressions
                WHERE channel = $1
                ORDER BY created_at DESC
                LIMIT $2
                "#,
            )
            .bind(ch)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, CommunicationSuppression>(
                r#"
                SELECT * FROM communication_suppressions
                ORDER BY created_at DESC
                LIMIT $1
                "#,
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(suppressions)
    }

    async fn create_template(&self, input: CreateTemplateInput) -> Result<MessageTemplate, AppError> {
        let template = sqlx::query_as::<_, MessageTemplate>(
            r#"
            INSERT INTO message_templates (
                template_code, channel, locale, subject, title,
                body_template, html_template, action_url, template_engine,
                category, description, variables, sample_data, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING *
            "#,
        )
        .bind(&input.template_code)
        .bind(input.channel)
        .bind(&input.locale)
        .bind(&input.subject)
        .bind(&input.title)
        .bind(&input.body_template)
        .bind(&input.html_template)
        .bind(&input.action_url)
        .bind(&input.template_engine)
        .bind(input.category)
        .bind(&input.description)
        .bind(&input.variables)
        .bind(&input.sample_data)
        .bind(&input.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(template)
    }

    async fn get_template(&self, code: &str, channel: CommChannel, locale: &str) -> Result<Option<MessageTemplate>, AppError> {
        let template = sqlx::query_as::<_, MessageTemplate>(
            r#"
            SELECT * FROM message_templates
            WHERE template_code = $1 AND channel = $2 AND locale = $3 AND status = 'approved'
            ORDER BY version DESC
            LIMIT 1
            "#,
        )
        .bind(code)
        .bind(channel)
        .bind(locale)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(template)
    }

    async fn get_template_by_id(&self, id: Uuid) -> Result<Option<MessageTemplate>, AppError> {
        let template = sqlx::query_as::<_, MessageTemplate>(
            "SELECT * FROM message_templates WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(template)
    }

    async fn list_templates(&self, channel: Option<CommChannel>, category: Option<MessageCategory>) -> Result<Vec<MessageTemplate>, AppError> {
        let templates = match (channel, category) {
            (Some(ch), Some(cat)) => {
                sqlx::query_as::<_, MessageTemplate>(
                    r#"
                    SELECT * FROM message_templates
                    WHERE channel = $1 AND category = $2
                    ORDER BY template_code, version DESC
                    "#,
                )
                .bind(ch)
                .bind(cat)
                .fetch_all(&self.pool)
                .await
            }
            (Some(ch), None) => {
                sqlx::query_as::<_, MessageTemplate>(
                    r#"
                    SELECT * FROM message_templates
                    WHERE channel = $1
                    ORDER BY template_code, version DESC
                    "#,
                )
                .bind(ch)
                .fetch_all(&self.pool)
                .await
            }
            (None, Some(cat)) => {
                sqlx::query_as::<_, MessageTemplate>(
                    r#"
                    SELECT * FROM message_templates
                    WHERE category = $1
                    ORDER BY template_code, version DESC
                    "#,
                )
                .bind(cat)
                .fetch_all(&self.pool)
                .await
            }
            (None, None) => {
                sqlx::query_as::<_, MessageTemplate>(
                    r#"
                    SELECT * FROM message_templates
                    ORDER BY template_code, version DESC
                    "#,
                )
                .fetch_all(&self.pool)
                .await
            }
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(templates)
    }

    async fn update_template(&self, id: Uuid, input: UpdateTemplateInput) -> Result<MessageTemplate, AppError> {
        let template = sqlx::query_as::<_, MessageTemplate>(
            r#"
            UPDATE message_templates SET
                subject = COALESCE($2, subject),
                title = COALESCE($3, title),
                body_template = COALESCE($4, body_template),
                html_template = COALESCE($5, html_template),
                action_url = COALESCE($6, action_url),
                description = COALESCE($7, description),
                variables = COALESCE($8, variables),
                sample_data = COALESCE($9, sample_data),
                updated_by = $10,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&input.subject)
        .bind(&input.title)
        .bind(&input.body_template)
        .bind(&input.html_template)
        .bind(&input.action_url)
        .bind(&input.description)
        .bind(&input.variables)
        .bind(&input.sample_data)
        .bind(&input.updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(template)
    }

    async fn submit_template_for_approval(&self, id: Uuid) -> Result<MessageTemplate, AppError> {
        let template = sqlx::query_as::<_, MessageTemplate>(
            r#"
            UPDATE message_templates
            SET status = 'pending_approval', updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(template)
    }

    async fn approve_template(&self, id: Uuid, approved_by: &str) -> Result<MessageTemplate, AppError> {
        let template = sqlx::query_as::<_, MessageTemplate>(
            r#"
            UPDATE message_templates
            SET status = 'approved', approved_by = $2, approved_at = NOW(), updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(approved_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(template)
    }

    async fn reject_template(&self, id: Uuid, reason: &str) -> Result<MessageTemplate, AppError> {
        let template = sqlx::query_as::<_, MessageTemplate>(
            r#"
            UPDATE message_templates
            SET status = 'draft', rejection_reason = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(reason)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(template)
    }

    async fn archive_template(&self, id: Uuid) -> Result<MessageTemplate, AppError> {
        let template = sqlx::query_as::<_, MessageTemplate>(
            r#"
            UPDATE message_templates
            SET status = 'archived', updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(template)
    }

    async fn increment_template_usage(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE message_templates SET usage_count = usage_count + 1, last_used_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn schedule_message(&self, input: ScheduleMessageInput) -> Result<ScheduledMessage, AppError> {
        let message = sqlx::query_as::<_, ScheduledMessage>(
            r#"
            INSERT INTO scheduled_messages (
                user_id, channel, category, recipient_address, recipient_name,
                subject, title, body, html_body, action_url,
                template_id, template_code, template_data,
                scheduled_at, local_time_preference, priority,
                correlation_id, triggered_by, entity_type, entity_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            RETURNING *
            "#,
        )
        .bind(&input.user_id)
        .bind(input.channel)
        .bind(input.category)
        .bind(&input.recipient_address)
        .bind(&input.recipient_name)
        .bind(&input.subject)
        .bind(&input.title)
        .bind(&input.body)
        .bind(&input.html_body)
        .bind(&input.action_url)
        .bind(input.template_id)
        .bind(&input.template_code)
        .bind(&input.template_data)
        .bind(input.scheduled_at)
        .bind(input.local_time_preference)
        .bind(input.priority)
        .bind(&input.correlation_id)
        .bind(&input.triggered_by)
        .bind(&input.entity_type)
        .bind(&input.entity_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(message)
    }

    async fn get_pending_messages(&self, limit: i32) -> Result<Vec<ScheduledMessage>, AppError> {
        let messages = sqlx::query_as::<_, ScheduledMessage>(
            r#"
            SELECT * FROM scheduled_messages
            WHERE status = 'scheduled' AND scheduled_at <= NOW()
            ORDER BY priority DESC, scheduled_at
            LIMIT $1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(messages)
    }

    async fn get_message(&self, id: Uuid) -> Result<Option<ScheduledMessage>, AppError> {
        let message = sqlx::query_as::<_, ScheduledMessage>(
            "SELECT * FROM scheduled_messages WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(message)
    }

    async fn get_user_messages(&self, user_id: &str, limit: i32) -> Result<Vec<ScheduledMessage>, AppError> {
        let messages = sqlx::query_as::<_, ScheduledMessage>(
            r#"
            SELECT * FROM scheduled_messages
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(messages)
    }

    async fn mark_message_sending(&self, id: Uuid) -> Result<ScheduledMessage, AppError> {
        let message = sqlx::query_as::<_, ScheduledMessage>(
            r#"
            UPDATE scheduled_messages
            SET status = 'sending', attempt_count = attempt_count + 1
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(message)
    }

    async fn mark_message_sent(&self, id: Uuid, external_id: Option<&str>) -> Result<ScheduledMessage, AppError> {
        let message = sqlx::query_as::<_, ScheduledMessage>(
            r#"
            UPDATE scheduled_messages
            SET status = 'sent', sent_at = NOW(), external_id = $2
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(external_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(message)
    }

    async fn mark_message_failed(&self, id: Uuid, error_code: &str, error_message: &str) -> Result<ScheduledMessage, AppError> {
        let message = sqlx::query_as::<_, ScheduledMessage>(
            r#"
            UPDATE scheduled_messages
            SET status = 'failed', error_code = $2, error_message = $3,
                next_retry_at = CASE
                    WHEN attempt_count < max_attempts THEN NOW() + INTERVAL '5 minutes'
                    ELSE NULL
                END
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(error_code)
        .bind(error_message)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(message)
    }

    async fn cancel_message(&self, id: Uuid) -> Result<ScheduledMessage, AppError> {
        let message = sqlx::query_as::<_, ScheduledMessage>(
            r#"
            UPDATE scheduled_messages
            SET status = 'cancelled'
            WHERE id = $1 AND status = 'scheduled'
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(message)
    }

    async fn get_send_stats(&self) -> Result<SendStats, AppError> {
        let stats = sqlx::query_as::<_, (i64, i64, i64, i64)>(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE status = 'scheduled') as scheduled,
                COUNT(*) FILTER (WHERE status = 'sent') as sent,
                COUNT(*) FILTER (WHERE status = 'failed') as failed,
                COUNT(*) FILTER (WHERE status = 'cancelled') as cancelled
            FROM scheduled_messages
            WHERE created_at > NOW() - INTERVAL '24 hours'
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(SendStats {
            scheduled: stats.0,
            sent: stats.1,
            failed: stats.2,
            cancelled: stats.3,
        })
    }

    async fn register_device(&self, input: RegisterDeviceInput) -> Result<DeviceToken, AppError> {
        let token_hash = Self::hash_identifier(&input.token);

        let device = sqlx::query_as::<_, DeviceToken>(
            r#"
            INSERT INTO device_tokens (
                user_id, token, token_hash, platform, platform_version,
                device_id, device_name, app_version
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (token_hash) DO UPDATE SET
                user_id = EXCLUDED.user_id,
                platform_version = EXCLUDED.platform_version,
                device_name = EXCLUDED.device_name,
                app_version = EXCLUDED.app_version,
                active = TRUE,
                invalidated_at = NULL,
                invalidation_reason = NULL,
                last_used_at = NOW(),
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(&input.user_id)
        .bind(&input.token)
        .bind(&token_hash)
        .bind(&input.platform)
        .bind(&input.platform_version)
        .bind(&input.device_id)
        .bind(&input.device_name)
        .bind(&input.app_version)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(device)
    }

    async fn get_user_devices(&self, user_id: &str) -> Result<Vec<DeviceToken>, AppError> {
        let devices = sqlx::query_as::<_, DeviceToken>(
            r#"
            SELECT * FROM device_tokens
            WHERE user_id = $1 AND active = TRUE
            ORDER BY last_used_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(devices)
    }

    async fn invalidate_device(&self, token_hash: &str, reason: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE device_tokens
            SET active = FALSE, invalidated_at = NOW(), invalidation_reason = $2, updated_at = NOW()
            WHERE token_hash = $1
            "#,
        )
        .bind(token_hash)
        .bind(reason)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn record_push_success(&self, token_hash: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE device_tokens
            SET last_success_at = NOW(), last_used_at = NOW(), consecutive_failures = 0, updated_at = NOW()
            WHERE token_hash = $1
            "#,
        )
        .bind(token_hash)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn record_push_failure(&self, token_hash: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE device_tokens
            SET last_failure_at = NOW(), consecutive_failures = consecutive_failures + 1, updated_at = NOW(),
                active = CASE WHEN consecutive_failures >= 4 THEN FALSE ELSE active END,
                invalidated_at = CASE WHEN consecutive_failures >= 4 THEN NOW() ELSE invalidated_at END,
                invalidation_reason = CASE WHEN consecutive_failures >= 4 THEN 'consecutive_failures' ELSE invalidation_reason END
            WHERE token_hash = $1
            "#,
        )
        .bind(token_hash)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
