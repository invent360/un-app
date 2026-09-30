//! Exit repository for voluntary exit workflow management
//!
//! Implements CRUD operations for:
//! - Participant exits (voluntary, admin, system, expiry)
//! - Exit feedback surveys
//! - Exit audit log
//! - Market waitlist

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENUMS
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitType {
    Voluntary,
    Admin,
    System,
    Expiry,
}

impl ExitType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExitType::Voluntary => "voluntary",
            ExitType::Admin => "admin",
            ExitType::System => "system",
            ExitType::Expiry => "expiry",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "voluntary" => Some(ExitType::Voluntary),
            "admin" => Some(ExitType::Admin),
            "system" => Some(ExitType::System),
            "expiry" => Some(ExitType::Expiry),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitStatus {
    Requested,
    PendingPayout,
    Processing,
    Completed,
    Cancelled,
    Rejected,
}

impl ExitStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExitStatus::Requested => "requested",
            ExitStatus::PendingPayout => "pending_payout",
            ExitStatus::Processing => "processing",
            ExitStatus::Completed => "completed",
            ExitStatus::Cancelled => "cancelled",
            ExitStatus::Rejected => "rejected",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "requested" => Some(ExitStatus::Requested),
            "pending_payout" => Some(ExitStatus::PendingPayout),
            "processing" => Some(ExitStatus::Processing),
            "completed" => Some(ExitStatus::Completed),
            "cancelled" => Some(ExitStatus::Cancelled),
            "rejected" => Some(ExitStatus::Rejected),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    NotRequired,
}

impl PayoutStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PayoutStatus::Pending => "pending",
            PayoutStatus::Processing => "processing",
            PayoutStatus::Completed => "completed",
            PayoutStatus::Failed => "failed",
            PayoutStatus::Cancelled => "cancelled",
            PayoutStatus::NotRequired => "not_required",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(PayoutStatus::Pending),
            "processing" => Some(PayoutStatus::Processing),
            "completed" => Some(PayoutStatus::Completed),
            "failed" => Some(PayoutStatus::Failed),
            "cancelled" => Some(PayoutStatus::Cancelled),
            "not_required" => Some(PayoutStatus::NotRequired),
            _ => None,
        }
    }
}

// ============================================
// ENTITY STRUCTS
// ============================================

/// Participant exit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantExit {
    pub id: Uuid,
    pub user_id: String,
    pub license_id: String,
    pub exit_type: ExitType,
    pub exit_reason: Option<String>,
    pub exit_details: Option<String>,
    pub final_balance_micros: i64,
    pub pending_earnings_micros: i64,
    pub deductions_micros: i64,
    pub net_payout_micros: i64,
    pub payout_requested: bool,
    pub payout_status: PayoutStatus,
    pub payout_method: Option<String>,
    pub payout_details: Option<serde_json::Value>,
    pub payout_reference: Option<String>,
    pub payout_initiated_at: Option<DateTime<Utc>>,
    pub payout_completed_at: Option<DateTime<Utc>>,
    pub payout_failed_reason: Option<String>,
    pub has_pending_tasks: bool,
    pub has_pending_support: bool,
    pub cooldown_ends_at: Option<DateTime<Utc>>,
    pub requested_by: String,
    pub requested_by_type: String,
    pub processed_by: Option<String>,
    pub processed_by_type: Option<String>,
    pub status: ExitStatus,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)]
struct ParticipantExitRow {
    id: Uuid,
    user_id: String,
    license_id: String,
    exit_type: String,
    exit_reason: Option<String>,
    exit_details: Option<String>,
    final_balance_micros: i64,
    pending_earnings_micros: i64,
    deductions_micros: i64,
    net_payout_micros: i64,
    payout_requested: bool,
    payout_status: Option<String>,
    payout_method: Option<String>,
    payout_details: Option<serde_json::Value>,
    payout_reference: Option<String>,
    payout_initiated_at: Option<DateTime<Utc>>,
    payout_completed_at: Option<DateTime<Utc>>,
    payout_failed_reason: Option<String>,
    has_pending_tasks: bool,
    has_pending_support: bool,
    cooldown_ends_at: Option<DateTime<Utc>>,
    requested_by: String,
    requested_by_type: String,
    processed_by: Option<String>,
    processed_by_type: Option<String>,
    status: String,
    rejection_reason: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

impl From<ParticipantExitRow> for ParticipantExit {
    fn from(row: ParticipantExitRow) -> Self {
        ParticipantExit {
            id: row.id,
            user_id: row.user_id,
            license_id: row.license_id,
            exit_type: ExitType::from_str(&row.exit_type).unwrap_or(ExitType::Voluntary),
            exit_reason: row.exit_reason,
            exit_details: row.exit_details,
            final_balance_micros: row.final_balance_micros,
            pending_earnings_micros: row.pending_earnings_micros,
            deductions_micros: row.deductions_micros,
            net_payout_micros: row.net_payout_micros,
            payout_requested: row.payout_requested,
            payout_status: row.payout_status
                .and_then(|s| PayoutStatus::from_str(&s))
                .unwrap_or(PayoutStatus::Pending),
            payout_method: row.payout_method,
            payout_details: row.payout_details,
            payout_reference: row.payout_reference,
            payout_initiated_at: row.payout_initiated_at,
            payout_completed_at: row.payout_completed_at,
            payout_failed_reason: row.payout_failed_reason,
            has_pending_tasks: row.has_pending_tasks,
            has_pending_support: row.has_pending_support,
            cooldown_ends_at: row.cooldown_ends_at,
            requested_by: row.requested_by,
            requested_by_type: row.requested_by_type,
            processed_by: row.processed_by,
            processed_by_type: row.processed_by_type,
            status: ExitStatus::from_str(&row.status).unwrap_or(ExitStatus::Requested),
            rejection_reason: row.rejection_reason,
            created_at: row.created_at,
            updated_at: row.updated_at,
            completed_at: row.completed_at,
        }
    }
}

/// Exit feedback survey
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExitFeedback {
    pub id: Uuid,
    pub exit_id: Uuid,
    pub primary_reason: Option<String>,
    pub satisfaction_score: Option<i32>,
    pub would_recommend: Option<bool>,
    pub earnings_met_expectations: Option<bool>,
    pub support_quality_score: Option<i32>,
    pub comments: Option<String>,
    pub improvement_suggestions: Option<String>,
    pub submitted_at: DateTime<Utc>,
}

/// Exit audit log entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExitAuditLog {
    pub id: Uuid,
    pub exit_id: Uuid,
    pub action: String,
    pub old_status: Option<String>,
    pub new_status: Option<String>,
    pub actor_id: String,
    pub actor_type: String,
    pub details: Option<serde_json::Value>,
    pub notes: Option<String>,
    pub logged_at: DateTime<Utc>,
}

/// Market waitlist entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WaitlistEntry {
    pub id: Uuid,
    pub user_id: Option<String>,
    pub email: String,
    pub country_code: String,
    pub consent_given: bool,
    pub consent_given_at: Option<DateTime<Utc>>,
    pub consent_version: Option<String>,
    pub status: String,
    pub notified_at: Option<DateTime<Utc>>,
    pub notification_reference: Option<String>,
    pub source: Option<String>,
    pub campaign_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================
// INPUT STRUCTS
// ============================================

/// Input for initiating an exit
#[derive(Debug, Clone)]
pub struct InitiateExitInput {
    pub user_id: String,
    pub license_id: String,
    pub exit_type: ExitType,
    pub exit_reason: Option<String>,
    pub exit_details: Option<String>,
    pub requested_by: String,
    pub requested_by_type: String,
}

/// Input for submitting feedback
#[derive(Debug, Clone)]
pub struct SubmitFeedbackInput {
    pub exit_id: Uuid,
    pub primary_reason: Option<String>,
    pub satisfaction_score: Option<i32>,
    pub would_recommend: Option<bool>,
    pub earnings_met_expectations: Option<bool>,
    pub support_quality_score: Option<i32>,
    pub comments: Option<String>,
    pub improvement_suggestions: Option<String>,
}

/// Input for adding to waitlist
#[derive(Debug, Clone)]
pub struct AddToWaitlistInput {
    pub user_id: Option<String>,
    pub email: String,
    pub country_code: String,
    pub consent_given: bool,
    pub consent_version: Option<String>,
    pub source: Option<String>,
    pub campaign_id: Option<Uuid>,
}

/// Calculated exit balance
#[derive(Debug, Clone, Serialize)]
pub struct ExitBalance {
    pub final_balance_micros: i64,
    pub pending_earnings_micros: i64,
    pub deductions_micros: i64,
    pub net_payout_micros: i64,
    pub has_pending_tasks: bool,
    pub cooldown_ends_at: Option<DateTime<Utc>>,
}

/// Payout result
#[derive(Debug, Clone, Serialize)]
pub struct PayoutResult {
    pub success: bool,
    pub reference: Option<String>,
    pub error_message: Option<String>,
}

// ============================================
// TRAIT DEFINITION
// ============================================

/// Dynamic type alias for ExitRepository trait object
pub type DynExitRepository = Arc<dyn ExitRepository + Send + Sync>;

/// Exit repository trait defining database operations
#[async_trait]
pub trait ExitRepository: Send + Sync {
    // --- Exit CRUD ---

    /// Initiate an exit
    async fn initiate_exit(&self, input: InitiateExitInput) -> Result<ParticipantExit, AppError>;

    /// Get exit by ID
    async fn get_exit(&self, id: Uuid) -> Result<Option<ParticipantExit>, AppError>;

    /// Get exit by license ID
    async fn get_exit_by_license(&self, license_id: &str) -> Result<Option<ParticipantExit>, AppError>;

    /// Get exits for user
    async fn get_user_exits(&self, user_id: &str) -> Result<Vec<ParticipantExit>, AppError>;

    /// Get exits by status
    async fn get_exits_by_status(&self, status: ExitStatus, limit: i32) -> Result<Vec<ParticipantExit>, AppError>;

    /// Get exits pending payout
    async fn get_exits_pending_payout(&self, limit: i32) -> Result<Vec<ParticipantExit>, AppError>;

    // --- Exit Status Updates ---

    /// Update exit status
    async fn update_status(
        &self,
        id: Uuid,
        status: ExitStatus,
        actor_id: &str,
        actor_type: &str,
        notes: Option<&str>,
    ) -> Result<ParticipantExit, AppError>;

    /// Approve exit
    async fn approve_exit(
        &self,
        id: Uuid,
        processed_by: &str,
        processed_by_type: &str,
    ) -> Result<ParticipantExit, AppError>;

    /// Reject exit
    async fn reject_exit(
        &self,
        id: Uuid,
        reason: &str,
        rejected_by: &str,
        rejected_by_type: &str,
    ) -> Result<ParticipantExit, AppError>;

    /// Cancel exit
    async fn cancel_exit(
        &self,
        id: Uuid,
        cancelled_by: &str,
        cancelled_by_type: &str,
    ) -> Result<ParticipantExit, AppError>;

    // --- Payout Operations ---

    /// Request payout
    async fn request_payout(
        &self,
        id: Uuid,
        method: &str,
        details: Option<serde_json::Value>,
    ) -> Result<ParticipantExit, AppError>;

    /// Initiate payout processing
    async fn initiate_payout(
        &self,
        id: Uuid,
        reference: &str,
    ) -> Result<ParticipantExit, AppError>;

    /// Complete payout
    async fn complete_payout(
        &self,
        id: Uuid,
        reference: &str,
    ) -> Result<ParticipantExit, AppError>;

    /// Fail payout
    async fn fail_payout(
        &self,
        id: Uuid,
        reason: &str,
    ) -> Result<ParticipantExit, AppError>;

    // --- Balance Calculation ---

    /// Calculate exit balance using database function
    async fn calculate_exit_balance(
        &self,
        user_id: &str,
        license_id: &str,
    ) -> Result<ExitBalance, AppError>;

    // --- Feedback Operations ---

    /// Submit exit feedback
    async fn submit_feedback(&self, input: SubmitFeedbackInput) -> Result<ExitFeedback, AppError>;

    /// Get feedback for exit
    async fn get_exit_feedback(&self, exit_id: Uuid) -> Result<Option<ExitFeedback>, AppError>;

    // --- Audit Log ---

    /// Get exit audit log
    async fn get_exit_audit_log(&self, exit_id: Uuid) -> Result<Vec<ExitAuditLog>, AppError>;

    /// Add audit log entry
    async fn add_audit_log(
        &self,
        exit_id: Uuid,
        action: &str,
        old_status: Option<&str>,
        new_status: Option<&str>,
        actor_id: &str,
        actor_type: &str,
        details: Option<serde_json::Value>,
        notes: Option<&str>,
    ) -> Result<ExitAuditLog, AppError>;

    // --- Waitlist Operations ---

    /// Add to waitlist
    async fn add_to_waitlist(&self, input: AddToWaitlistInput) -> Result<WaitlistEntry, AppError>;

    /// Get waitlist entry by email and country
    async fn get_waitlist_entry(
        &self,
        email: &str,
        country_code: &str,
    ) -> Result<Option<WaitlistEntry>, AppError>;

    /// Get waitlist for country
    async fn get_country_waitlist(&self, country_code: &str) -> Result<Vec<WaitlistEntry>, AppError>;

    /// Update waitlist status
    async fn update_waitlist_status(
        &self,
        id: Uuid,
        status: &str,
        notification_reference: Option<&str>,
    ) -> Result<WaitlistEntry, AppError>;

    /// Get waitlist entries to notify
    async fn get_waitlist_to_notify(&self, country_code: &str, limit: i32) -> Result<Vec<WaitlistEntry>, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct ExitRepositoryImpl {
    pool: ConnectionPool,
}

impl ExitRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExitRepository for ExitRepositoryImpl {
    async fn initiate_exit(&self, input: InitiateExitInput) -> Result<ParticipantExit, AppError> {
        // Use database function for atomic operation with balance calculation
        let (exit_id,): (Uuid,) = sqlx::query_as(
            "SELECT initiate_exit($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(&input.user_id)
        .bind(&input.license_id)
        .bind(input.exit_type.as_str())
        .bind(&input.exit_reason)
        .bind(&input.exit_details)
        .bind(&input.requested_by)
        .bind(&input.requested_by_type)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("Exit already initiated") {
                AppError::Conflict("Exit already initiated for this license".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        self.get_exit(exit_id).await?
            .ok_or_else(|| AppError::InternalServerError("Failed to retrieve created exit".to_string()))
    }

    async fn get_exit(&self, id: Uuid) -> Result<Option<ParticipantExit>, AppError> {
        let row = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            SELECT id, user_id, license_id, exit_type, exit_reason, exit_details,
                   final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                   payout_requested, payout_status, payout_method, payout_details, payout_reference,
                   payout_initiated_at, payout_completed_at, payout_failed_reason,
                   has_pending_tasks, has_pending_support, cooldown_ends_at,
                   requested_by, requested_by_type, processed_by, processed_by_type,
                   status, rejection_reason, created_at, updated_at, completed_at
            FROM participant_exits
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(ParticipantExit::from))
    }

    async fn get_exit_by_license(&self, license_id: &str) -> Result<Option<ParticipantExit>, AppError> {
        let row = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            SELECT id, user_id, license_id, exit_type, exit_reason, exit_details,
                   final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                   payout_requested, payout_status, payout_method, payout_details, payout_reference,
                   payout_initiated_at, payout_completed_at, payout_failed_reason,
                   has_pending_tasks, has_pending_support, cooldown_ends_at,
                   requested_by, requested_by_type, processed_by, processed_by_type,
                   status, rejection_reason, created_at, updated_at, completed_at
            FROM participant_exits
            WHERE license_id = $1
            "#,
        )
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(row.map(ParticipantExit::from))
    }

    async fn get_user_exits(&self, user_id: &str) -> Result<Vec<ParticipantExit>, AppError> {
        let rows = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            SELECT id, user_id, license_id, exit_type, exit_reason, exit_details,
                   final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                   payout_requested, payout_status, payout_method, payout_details, payout_reference,
                   payout_initiated_at, payout_completed_at, payout_failed_reason,
                   has_pending_tasks, has_pending_support, cooldown_ends_at,
                   requested_by, requested_by_type, processed_by, processed_by_type,
                   status, rejection_reason, created_at, updated_at, completed_at
            FROM participant_exits
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(ParticipantExit::from).collect())
    }

    async fn get_exits_by_status(&self, status: ExitStatus, limit: i32) -> Result<Vec<ParticipantExit>, AppError> {
        let rows = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            SELECT id, user_id, license_id, exit_type, exit_reason, exit_details,
                   final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                   payout_requested, payout_status, payout_method, payout_details, payout_reference,
                   payout_initiated_at, payout_completed_at, payout_failed_reason,
                   has_pending_tasks, has_pending_support, cooldown_ends_at,
                   requested_by, requested_by_type, processed_by, processed_by_type,
                   status, rejection_reason, created_at, updated_at, completed_at
            FROM participant_exits
            WHERE status = $1
            ORDER BY created_at ASC
            LIMIT $2
            "#,
        )
        .bind(status.as_str())
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(ParticipantExit::from).collect())
    }

    async fn get_exits_pending_payout(&self, limit: i32) -> Result<Vec<ParticipantExit>, AppError> {
        let rows = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            SELECT id, user_id, license_id, exit_type, exit_reason, exit_details,
                   final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                   payout_requested, payout_status, payout_method, payout_details, payout_reference,
                   payout_initiated_at, payout_completed_at, payout_failed_reason,
                   has_pending_tasks, has_pending_support, cooldown_ends_at,
                   requested_by, requested_by_type, processed_by, processed_by_type,
                   status, rejection_reason, created_at, updated_at, completed_at
            FROM participant_exits
            WHERE payout_status IN ('pending', 'processing')
              AND payout_requested = TRUE
              AND net_payout_micros > 0
            ORDER BY created_at ASC
            LIMIT $1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(ParticipantExit::from).collect())
    }

    async fn update_status(
        &self,
        id: Uuid,
        status: ExitStatus,
        actor_id: &str,
        actor_type: &str,
        notes: Option<&str>,
    ) -> Result<ParticipantExit, AppError> {
        // Get current exit for audit log
        let current = self.get_exit(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Exit {} not found", id)))?;

        let row = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            UPDATE participant_exits
            SET status = $2,
                completed_at = CASE WHEN $2 = 'completed' THEN NOW() ELSE completed_at END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, exit_type, exit_reason, exit_details,
                      final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                      payout_requested, payout_status, payout_method, payout_details, payout_reference,
                      payout_initiated_at, payout_completed_at, payout_failed_reason,
                      has_pending_tasks, has_pending_support, cooldown_ends_at,
                      requested_by, requested_by_type, processed_by, processed_by_type,
                      status, rejection_reason, created_at, updated_at, completed_at
            "#,
        )
        .bind(id)
        .bind(status.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Add audit log
        self.add_audit_log(
            id,
            "status_change",
            Some(current.status.as_str()),
            Some(status.as_str()),
            actor_id,
            actor_type,
            None,
            notes,
        ).await?;

        Ok(ParticipantExit::from(row))
    }

    async fn approve_exit(
        &self,
        id: Uuid,
        processed_by: &str,
        processed_by_type: &str,
    ) -> Result<ParticipantExit, AppError> {
        let row = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            UPDATE participant_exits
            SET status = CASE
                    WHEN net_payout_micros > 0 THEN 'pending_payout'
                    ELSE 'completed'
                END,
                processed_by = $2,
                processed_by_type = $3,
                completed_at = CASE WHEN net_payout_micros <= 0 THEN NOW() ELSE NULL END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, exit_type, exit_reason, exit_details,
                      final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                      payout_requested, payout_status, payout_method, payout_details, payout_reference,
                      payout_initiated_at, payout_completed_at, payout_failed_reason,
                      has_pending_tasks, has_pending_support, cooldown_ends_at,
                      requested_by, requested_by_type, processed_by, processed_by_type,
                      status, rejection_reason, created_at, updated_at, completed_at
            "#,
        )
        .bind(id)
        .bind(processed_by)
        .bind(processed_by_type)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let exit = ParticipantExit::from(row);

        // Add audit log
        self.add_audit_log(
            id,
            "approved",
            Some("requested"),
            Some(exit.status.as_str()),
            processed_by,
            processed_by_type,
            None,
            None,
        ).await?;

        Ok(exit)
    }

    async fn reject_exit(
        &self,
        id: Uuid,
        reason: &str,
        rejected_by: &str,
        rejected_by_type: &str,
    ) -> Result<ParticipantExit, AppError> {
        let row = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            UPDATE participant_exits
            SET status = 'rejected',
                rejection_reason = $2,
                processed_by = $3,
                processed_by_type = $4,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, exit_type, exit_reason, exit_details,
                      final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                      payout_requested, payout_status, payout_method, payout_details, payout_reference,
                      payout_initiated_at, payout_completed_at, payout_failed_reason,
                      has_pending_tasks, has_pending_support, cooldown_ends_at,
                      requested_by, requested_by_type, processed_by, processed_by_type,
                      status, rejection_reason, created_at, updated_at, completed_at
            "#,
        )
        .bind(id)
        .bind(reason)
        .bind(rejected_by)
        .bind(rejected_by_type)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Add audit log
        self.add_audit_log(
            id,
            "rejected",
            None,
            Some("rejected"),
            rejected_by,
            rejected_by_type,
            Some(serde_json::json!({ "reason": reason })),
            None,
        ).await?;

        Ok(ParticipantExit::from(row))
    }

    async fn cancel_exit(
        &self,
        id: Uuid,
        cancelled_by: &str,
        cancelled_by_type: &str,
    ) -> Result<ParticipantExit, AppError> {
        let row = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            UPDATE participant_exits
            SET status = 'cancelled',
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, exit_type, exit_reason, exit_details,
                      final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                      payout_requested, payout_status, payout_method, payout_details, payout_reference,
                      payout_initiated_at, payout_completed_at, payout_failed_reason,
                      has_pending_tasks, has_pending_support, cooldown_ends_at,
                      requested_by, requested_by_type, processed_by, processed_by_type,
                      status, rejection_reason, created_at, updated_at, completed_at
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Add audit log
        self.add_audit_log(
            id,
            "cancelled",
            None,
            Some("cancelled"),
            cancelled_by,
            cancelled_by_type,
            None,
            None,
        ).await?;

        Ok(ParticipantExit::from(row))
    }

    async fn request_payout(
        &self,
        id: Uuid,
        method: &str,
        details: Option<serde_json::Value>,
    ) -> Result<ParticipantExit, AppError> {
        let row = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            UPDATE participant_exits
            SET payout_requested = TRUE,
                payout_method = $2,
                payout_details = $3,
                payout_status = 'pending',
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, exit_type, exit_reason, exit_details,
                      final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                      payout_requested, payout_status, payout_method, payout_details, payout_reference,
                      payout_initiated_at, payout_completed_at, payout_failed_reason,
                      has_pending_tasks, has_pending_support, cooldown_ends_at,
                      requested_by, requested_by_type, processed_by, processed_by_type,
                      status, rejection_reason, created_at, updated_at, completed_at
            "#,
        )
        .bind(id)
        .bind(method)
        .bind(details)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(ParticipantExit::from(row))
    }

    async fn initiate_payout(
        &self,
        id: Uuid,
        reference: &str,
    ) -> Result<ParticipantExit, AppError> {
        let row = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            UPDATE participant_exits
            SET payout_status = 'processing',
                payout_reference = $2,
                payout_initiated_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, exit_type, exit_reason, exit_details,
                      final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                      payout_requested, payout_status, payout_method, payout_details, payout_reference,
                      payout_initiated_at, payout_completed_at, payout_failed_reason,
                      has_pending_tasks, has_pending_support, cooldown_ends_at,
                      requested_by, requested_by_type, processed_by, processed_by_type,
                      status, rejection_reason, created_at, updated_at, completed_at
            "#,
        )
        .bind(id)
        .bind(reference)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(ParticipantExit::from(row))
    }

    async fn complete_payout(
        &self,
        id: Uuid,
        reference: &str,
    ) -> Result<ParticipantExit, AppError> {
        let row = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            UPDATE participant_exits
            SET payout_status = 'completed',
                payout_reference = $2,
                payout_completed_at = NOW(),
                status = 'completed',
                completed_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, exit_type, exit_reason, exit_details,
                      final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                      payout_requested, payout_status, payout_method, payout_details, payout_reference,
                      payout_initiated_at, payout_completed_at, payout_failed_reason,
                      has_pending_tasks, has_pending_support, cooldown_ends_at,
                      requested_by, requested_by_type, processed_by, processed_by_type,
                      status, rejection_reason, created_at, updated_at, completed_at
            "#,
        )
        .bind(id)
        .bind(reference)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Add audit log
        self.add_audit_log(
            id,
            "payout_completed",
            None,
            None,
            "system",
            "system",
            Some(serde_json::json!({ "reference": reference })),
            None,
        ).await?;

        Ok(ParticipantExit::from(row))
    }

    async fn fail_payout(
        &self,
        id: Uuid,
        reason: &str,
    ) -> Result<ParticipantExit, AppError> {
        let row = sqlx::query_as::<_, ParticipantExitRow>(
            r#"
            UPDATE participant_exits
            SET payout_status = 'failed',
                payout_failed_reason = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, exit_type, exit_reason, exit_details,
                      final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
                      payout_requested, payout_status, payout_method, payout_details, payout_reference,
                      payout_initiated_at, payout_completed_at, payout_failed_reason,
                      has_pending_tasks, has_pending_support, cooldown_ends_at,
                      requested_by, requested_by_type, processed_by, processed_by_type,
                      status, rejection_reason, created_at, updated_at, completed_at
            "#,
        )
        .bind(id)
        .bind(reason)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Add audit log
        self.add_audit_log(
            id,
            "payout_failed",
            None,
            None,
            "system",
            "system",
            Some(serde_json::json!({ "reason": reason })),
            None,
        ).await?;

        Ok(ParticipantExit::from(row))
    }

    async fn calculate_exit_balance(
        &self,
        user_id: &str,
        license_id: &str,
    ) -> Result<ExitBalance, AppError> {
        let row = sqlx::query_as::<_, (i64, i64, i64, i64, bool, Option<DateTime<Utc>>)>(
            "SELECT * FROM calculate_exit_balance($1, $2)",
        )
        .bind(user_id)
        .bind(license_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(ExitBalance {
            final_balance_micros: row.0,
            pending_earnings_micros: row.1,
            deductions_micros: row.2,
            net_payout_micros: row.3,
            has_pending_tasks: row.4,
            cooldown_ends_at: row.5,
        })
    }

    async fn submit_feedback(&self, input: SubmitFeedbackInput) -> Result<ExitFeedback, AppError> {
        let feedback = sqlx::query_as::<_, ExitFeedback>(
            r#"
            INSERT INTO exit_feedback (
                exit_id, primary_reason, satisfaction_score, would_recommend,
                earnings_met_expectations, support_quality_score, comments, improvement_suggestions
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (exit_id) DO UPDATE SET
                primary_reason = EXCLUDED.primary_reason,
                satisfaction_score = EXCLUDED.satisfaction_score,
                would_recommend = EXCLUDED.would_recommend,
                earnings_met_expectations = EXCLUDED.earnings_met_expectations,
                support_quality_score = EXCLUDED.support_quality_score,
                comments = EXCLUDED.comments,
                improvement_suggestions = EXCLUDED.improvement_suggestions,
                submitted_at = NOW()
            RETURNING id, exit_id, primary_reason, satisfaction_score, would_recommend,
                      earnings_met_expectations, support_quality_score, comments,
                      improvement_suggestions, submitted_at
            "#,
        )
        .bind(input.exit_id)
        .bind(&input.primary_reason)
        .bind(input.satisfaction_score)
        .bind(input.would_recommend)
        .bind(input.earnings_met_expectations)
        .bind(input.support_quality_score)
        .bind(&input.comments)
        .bind(&input.improvement_suggestions)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(feedback)
    }

    async fn get_exit_feedback(&self, exit_id: Uuid) -> Result<Option<ExitFeedback>, AppError> {
        let feedback = sqlx::query_as::<_, ExitFeedback>(
            r#"
            SELECT id, exit_id, primary_reason, satisfaction_score, would_recommend,
                   earnings_met_expectations, support_quality_score, comments,
                   improvement_suggestions, submitted_at
            FROM exit_feedback
            WHERE exit_id = $1
            "#,
        )
        .bind(exit_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(feedback)
    }

    async fn get_exit_audit_log(&self, exit_id: Uuid) -> Result<Vec<ExitAuditLog>, AppError> {
        let logs = sqlx::query_as::<_, ExitAuditLog>(
            r#"
            SELECT id, exit_id, action, old_status, new_status, actor_id, actor_type,
                   details, notes, logged_at
            FROM exit_audit_log
            WHERE exit_id = $1
            ORDER BY logged_at DESC
            "#,
        )
        .bind(exit_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(logs)
    }

    async fn add_audit_log(
        &self,
        exit_id: Uuid,
        action: &str,
        old_status: Option<&str>,
        new_status: Option<&str>,
        actor_id: &str,
        actor_type: &str,
        details: Option<serde_json::Value>,
        notes: Option<&str>,
    ) -> Result<ExitAuditLog, AppError> {
        let log = sqlx::query_as::<_, ExitAuditLog>(
            r#"
            INSERT INTO exit_audit_log (exit_id, action, old_status, new_status, actor_id, actor_type, details, notes)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, exit_id, action, old_status, new_status, actor_id, actor_type, details, notes, logged_at
            "#,
        )
        .bind(exit_id)
        .bind(action)
        .bind(old_status)
        .bind(new_status)
        .bind(actor_id)
        .bind(actor_type)
        .bind(details)
        .bind(notes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(log)
    }

    async fn add_to_waitlist(&self, input: AddToWaitlistInput) -> Result<WaitlistEntry, AppError> {
        let entry = sqlx::query_as::<_, WaitlistEntry>(
            r#"
            INSERT INTO market_waitlist (
                user_id, email, country_code, consent_given, consent_given_at,
                consent_version, source, campaign_id
            )
            VALUES ($1, $2, $3, $4, CASE WHEN $4 THEN NOW() ELSE NULL END, $5, $6, $7)
            ON CONFLICT (email, country_code) DO UPDATE SET
                user_id = COALESCE(EXCLUDED.user_id, market_waitlist.user_id),
                consent_given = EXCLUDED.consent_given,
                consent_given_at = CASE WHEN EXCLUDED.consent_given THEN NOW() ELSE market_waitlist.consent_given_at END,
                consent_version = COALESCE(EXCLUDED.consent_version, market_waitlist.consent_version),
                updated_at = NOW()
            RETURNING id, user_id, email, country_code, consent_given, consent_given_at,
                      consent_version, status, notified_at, notification_reference,
                      source, campaign_id, created_at, updated_at
            "#,
        )
        .bind(&input.user_id)
        .bind(&input.email)
        .bind(&input.country_code)
        .bind(input.consent_given)
        .bind(&input.consent_version)
        .bind(&input.source)
        .bind(input.campaign_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(entry)
    }

    async fn get_waitlist_entry(
        &self,
        email: &str,
        country_code: &str,
    ) -> Result<Option<WaitlistEntry>, AppError> {
        let entry = sqlx::query_as::<_, WaitlistEntry>(
            r#"
            SELECT id, user_id, email, country_code, consent_given, consent_given_at,
                   consent_version, status, notified_at, notification_reference,
                   source, campaign_id, created_at, updated_at
            FROM market_waitlist
            WHERE email = $1 AND country_code = $2
            "#,
        )
        .bind(email)
        .bind(country_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(entry)
    }

    async fn get_country_waitlist(&self, country_code: &str) -> Result<Vec<WaitlistEntry>, AppError> {
        let entries = sqlx::query_as::<_, WaitlistEntry>(
            r#"
            SELECT id, user_id, email, country_code, consent_given, consent_given_at,
                   consent_version, status, notified_at, notification_reference,
                   source, campaign_id, created_at, updated_at
            FROM market_waitlist
            WHERE country_code = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(country_code)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(entries)
    }

    async fn update_waitlist_status(
        &self,
        id: Uuid,
        status: &str,
        notification_reference: Option<&str>,
    ) -> Result<WaitlistEntry, AppError> {
        let entry = sqlx::query_as::<_, WaitlistEntry>(
            r#"
            UPDATE market_waitlist
            SET status = $2,
                notified_at = CASE WHEN $2 = 'notified' THEN NOW() ELSE notified_at END,
                notification_reference = COALESCE($3, notification_reference),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, email, country_code, consent_given, consent_given_at,
                      consent_version, status, notified_at, notification_reference,
                      source, campaign_id, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(notification_reference)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(entry)
    }

    async fn get_waitlist_to_notify(&self, country_code: &str, limit: i32) -> Result<Vec<WaitlistEntry>, AppError> {
        let entries = sqlx::query_as::<_, WaitlistEntry>(
            r#"
            SELECT id, user_id, email, country_code, consent_given, consent_given_at,
                   consent_version, status, notified_at, notification_reference,
                   source, campaign_id, created_at, updated_at
            FROM market_waitlist
            WHERE country_code = $1
              AND status = 'waiting'
              AND consent_given = TRUE
            ORDER BY created_at ASC
            LIMIT $2
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(country_code)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(entries)
    }
}
