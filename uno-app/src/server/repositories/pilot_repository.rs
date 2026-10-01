//! Pilot participant repository for controlled rollout tracking
//!
//! Phase 9: Release Validation - tracks pilot participants
//! through their lifecycle milestones (D7, D30).

use crate::server::db::ConnectionPool;
use crate::types::AppError;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

/// Dynamic type alias for dependency injection
pub type DynPilotRepository = Arc<dyn PilotRepository + Send + Sync>;

/// Pilot participant state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "pilot_participant_state", rename_all = "snake_case")]
pub enum PilotParticipantState {
    Invited,
    Registered,
    LicenseClaimed,
    D7Active,
    D7Inactive,
    D30Active,
    D30Inactive,
    Graduated,
    Dropped,
    Withdrawn,
}

impl std::fmt::Display for PilotParticipantState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invited => write!(f, "invited"),
            Self::Registered => write!(f, "registered"),
            Self::LicenseClaimed => write!(f, "license_claimed"),
            Self::D7Active => write!(f, "d7_active"),
            Self::D7Inactive => write!(f, "d7_inactive"),
            Self::D30Active => write!(f, "d30_active"),
            Self::D30Inactive => write!(f, "d30_inactive"),
            Self::Graduated => write!(f, "graduated"),
            Self::Dropped => write!(f, "dropped"),
            Self::Withdrawn => write!(f, "withdrawn"),
        }
    }
}

/// Pilot cohort definition
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PilotCohort {
    pub id: String,
    pub market_code: String,
    pub cohort_name: String,
    pub target_size: i32,
    pub current_size: i32,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
    pub success_criteria: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Pilot participant
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PilotParticipant {
    pub id: Uuid,
    pub external_user_id: String,
    pub market_code: String,
    pub cohort_id: String,
    pub state: PilotParticipantState,
    pub invitation_code: Option<String>,
    pub invitation_channel: Option<String>,
    pub invited_at: Option<DateTime<Utc>>,
    pub registered_at: Option<DateTime<Utc>>,
    pub license_claimed_at: Option<DateTime<Utc>>,
    pub d7_check_at: Option<DateTime<Utc>>,
    pub d30_check_at: Option<DateTime<Utc>>,
    pub graduated_at: Option<DateTime<Utc>>,
    pub d7_outcome: Option<String>,
    pub d30_outcome: Option<String>,
    pub sessions_count: i32,
    pub activities_count: i32,
    pub support_tickets_count: i32,
    pub license_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Pilot state transition record
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PilotStateTransition {
    pub id: Uuid,
    pub participant_id: Uuid,
    pub from_state: Option<PilotParticipantState>,
    pub to_state: PilotParticipantState,
    pub triggered_by: String,
    pub actor: Option<String>,
    pub reason: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// Pilot activity record
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PilotActivity {
    pub id: Uuid,
    pub participant_id: Uuid,
    pub activity_type: String,
    pub activity_data: Option<serde_json::Value>,
    pub recorded_at: DateTime<Utc>,
}

/// Pilot metrics snapshot
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PilotMetricsSnapshot {
    pub id: Uuid,
    pub cohort_id: String,
    pub snapshot_date: NaiveDate,
    pub total_invited: i32,
    pub total_registered: i32,
    pub total_claimed: i32,
    pub total_d7_active: i32,
    pub total_d7_inactive: i32,
    pub total_d30_active: i32,
    pub total_d30_inactive: i32,
    pub total_graduated: i32,
    pub total_dropped: i32,
    pub avg_sessions_per_user: Option<rust_decimal::Decimal>,
    pub avg_activities_per_user: Option<rust_decimal::Decimal>,
    pub support_ticket_rate: Option<rust_decimal::Decimal>,
    pub d7_retention_rate: Option<rust_decimal::Decimal>,
    pub d30_retention_rate: Option<rust_decimal::Decimal>,
    pub created_at: DateTime<Utc>,
}

/// Input for adding a pilot participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddParticipantInput {
    pub external_user_id: String,
    pub market_code: String,
    pub cohort_id: String,
    pub invitation_code: Option<String>,
    pub invitation_channel: Option<String>,
}

/// Input for transitioning participant state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionStateInput {
    pub participant_id: Uuid,
    pub new_state: PilotParticipantState,
    pub triggered_by: String,
    pub actor: Option<String>,
    pub reason: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Input for recording pilot activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPilotActivityInput {
    pub participant_id: Uuid,
    pub activity_type: String,
    pub activity_data: Option<serde_json::Value>,
}

/// Cohort report with aggregate metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortReport {
    pub cohort: PilotCohort,
    pub total_participants: i64,
    pub by_state: Vec<(PilotParticipantState, i64)>,
    pub d7_retention_rate: Option<f64>,
    pub d30_retention_rate: Option<f64>,
}

/// Pilot repository trait
#[async_trait]
pub trait PilotRepository {
    /// Get all cohorts
    async fn get_cohorts(&self) -> Result<Vec<PilotCohort>, AppError>;

    /// Get a specific cohort
    async fn get_cohort(&self, cohort_id: &str) -> Result<Option<PilotCohort>, AppError>;

    /// Get active cohorts for a market
    async fn get_active_cohorts(&self, market_code: &str) -> Result<Vec<PilotCohort>, AppError>;

    /// Check if cohort has capacity
    async fn cohort_has_capacity(&self, cohort_id: &str) -> Result<bool, AppError>;

    /// Add a pilot participant
    async fn add_participant(&self, input: AddParticipantInput) -> Result<PilotParticipant, AppError>;

    /// Get a participant by ID
    async fn get_participant(&self, participant_id: Uuid) -> Result<Option<PilotParticipant>, AppError>;

    /// Get a participant by user ID
    async fn get_participant_by_user(&self, external_user_id: &str, cohort_id: &str) -> Result<Option<PilotParticipant>, AppError>;

    /// Get all participants in a cohort
    async fn get_cohort_participants(&self, cohort_id: &str) -> Result<Vec<PilotParticipant>, AppError>;

    /// Transition participant state
    async fn transition_state(&self, input: TransitionStateInput) -> Result<PilotParticipant, AppError>;

    /// Get state transition history
    async fn get_state_history(&self, participant_id: Uuid) -> Result<Vec<PilotStateTransition>, AppError>;

    /// Record a pilot activity
    async fn record_activity(&self, input: RecordPilotActivityInput) -> Result<PilotActivity, AppError>;

    /// Get participant activities
    async fn get_activities(&self, participant_id: Uuid) -> Result<Vec<PilotActivity>, AppError>;

    /// Get participants pending D7 check
    async fn get_pending_d7_check(&self) -> Result<Vec<PilotParticipant>, AppError>;

    /// Get participants pending D30 check
    async fn get_pending_d30_check(&self) -> Result<Vec<PilotParticipant>, AppError>;

    /// Generate cohort report
    async fn get_cohort_report(&self, cohort_id: &str) -> Result<CohortReport, AppError>;

    /// Save metrics snapshot
    async fn save_metrics_snapshot(&self, cohort_id: &str) -> Result<PilotMetricsSnapshot, AppError>;

    /// Get metrics snapshots for a cohort
    async fn get_metrics_snapshots(&self, cohort_id: &str) -> Result<Vec<PilotMetricsSnapshot>, AppError>;

    /// F6: Atomic pilot enrollment with gate/quota/capacity checks AND participant insertion
    ///
    /// Returns the created participant on success, or an error with rejection reason.
    /// This is fully atomic - quota consumption, capacity increment, and participant
    /// creation all happen in a single transaction.
    async fn atomic_enroll_with_participant(
        &self,
        input: AddParticipantInput,
    ) -> Result<Result<PilotParticipant, String>, AppError>;

    /// R5-16: Atomic pilot enrollment with gate/quota/capacity checks (legacy)
    ///
    /// Returns Some((cohort_id, None)) on success, Some((_, rejection_reason)) on rejection,
    /// or None if the function returns no rows.
    #[deprecated(note = "Use atomic_enroll_with_participant for true atomicity")]
    async fn atomic_enroll(
        &self,
        user_id: &str,
        market_code: &str,
        cohort_id: Option<&str>,
    ) -> Result<Option<(String, Option<String>)>, AppError>;
}

/// PostgreSQL implementation of PilotRepository
pub struct PilotRepositoryImpl {
    pool: ConnectionPool,
}

impl PilotRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PilotRepository for PilotRepositoryImpl {
    async fn get_cohorts(&self) -> Result<Vec<PilotCohort>, AppError> {
        let cohorts = sqlx::query_as::<_, PilotCohort>(
            r#"
            SELECT id, market_code, cohort_name, target_size, current_size,
                   start_date, end_date, is_active, success_criteria, created_at, updated_at
            FROM pilot_cohorts
            ORDER BY start_date DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(cohorts)
    }

    async fn get_cohort(&self, cohort_id: &str) -> Result<Option<PilotCohort>, AppError> {
        let cohort = sqlx::query_as::<_, PilotCohort>(
            r#"
            SELECT id, market_code, cohort_name, target_size, current_size,
                   start_date, end_date, is_active, success_criteria, created_at, updated_at
            FROM pilot_cohorts
            WHERE id = $1
            "#,
        )
        .bind(cohort_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(cohort)
    }

    async fn get_active_cohorts(&self, market_code: &str) -> Result<Vec<PilotCohort>, AppError> {
        let cohorts = sqlx::query_as::<_, PilotCohort>(
            r#"
            SELECT id, market_code, cohort_name, target_size, current_size,
                   start_date, end_date, is_active, success_criteria, created_at, updated_at
            FROM pilot_cohorts
            WHERE market_code = $1 AND is_active = true
            ORDER BY start_date DESC
            "#,
        )
        .bind(market_code)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(cohorts)
    }

    async fn cohort_has_capacity(&self, cohort_id: &str) -> Result<bool, AppError> {
        let result: (i32, i32) = sqlx::query_as(
            r#"
            SELECT current_size, target_size
            FROM pilot_cohorts
            WHERE id = $1
            "#,
        )
        .bind(cohort_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result.0 < result.1)
    }

    async fn add_participant(&self, input: AddParticipantInput) -> Result<PilotParticipant, AppError> {
        let participant = sqlx::query_as::<_, PilotParticipant>(
            r#"
            INSERT INTO pilot_participants (
                external_user_id, market_code, cohort_id, state,
                invitation_code, invitation_channel, invited_at
            )
            VALUES ($1, $2, $3, 'invited', $4, $5, NOW())
            RETURNING *
            "#,
        )
        .bind(&input.external_user_id)
        .bind(&input.market_code)
        .bind(&input.cohort_id)
        .bind(&input.invitation_code)
        .bind(&input.invitation_channel)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(participant)
    }

    async fn get_participant(&self, participant_id: Uuid) -> Result<Option<PilotParticipant>, AppError> {
        let participant = sqlx::query_as::<_, PilotParticipant>(
            "SELECT * FROM pilot_participants WHERE id = $1",
        )
        .bind(participant_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(participant)
    }

    async fn get_participant_by_user(&self, external_user_id: &str, cohort_id: &str) -> Result<Option<PilotParticipant>, AppError> {
        let participant = sqlx::query_as::<_, PilotParticipant>(
            "SELECT * FROM pilot_participants WHERE external_user_id = $1 AND cohort_id = $2",
        )
        .bind(external_user_id)
        .bind(cohort_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(participant)
    }

    async fn get_cohort_participants(&self, cohort_id: &str) -> Result<Vec<PilotParticipant>, AppError> {
        let participants = sqlx::query_as::<_, PilotParticipant>(
            "SELECT * FROM pilot_participants WHERE cohort_id = $1 ORDER BY created_at",
        )
        .bind(cohort_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(participants)
    }

    async fn transition_state(&self, input: TransitionStateInput) -> Result<PilotParticipant, AppError> {
        // Get current state
        let current = self.get_participant(input.participant_id).await?
            .ok_or_else(|| AppError::NotFound("Participant not found".to_string()))?;

        // Record transition
        sqlx::query(
            r#"
            INSERT INTO pilot_state_transitions (
                participant_id, from_state, to_state, triggered_by, actor, reason, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(input.participant_id)
        .bind(current.state)
        .bind(input.new_state)
        .bind(&input.triggered_by)
        .bind(&input.actor)
        .bind(&input.reason)
        .bind(&input.metadata)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        // Update participant state with appropriate timestamp
        let timestamp_field = match input.new_state {
            PilotParticipantState::Registered => "registered_at",
            PilotParticipantState::LicenseClaimed => "license_claimed_at",
            PilotParticipantState::D7Active | PilotParticipantState::D7Inactive => "d7_check_at",
            PilotParticipantState::D30Active | PilotParticipantState::D30Inactive => "d30_check_at",
            PilotParticipantState::Graduated => "graduated_at",
            _ => "",
        };

        let query = if timestamp_field.is_empty() {
            format!(
                "UPDATE pilot_participants SET state = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
            )
        } else {
            format!(
                "UPDATE pilot_participants SET state = $1, {} = NOW(), updated_at = NOW() WHERE id = $2 RETURNING *",
                timestamp_field
            )
        };

        let participant = sqlx::query_as::<_, PilotParticipant>(&query)
            .bind(input.new_state)
            .bind(input.participant_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(participant)
    }

    async fn get_state_history(&self, participant_id: Uuid) -> Result<Vec<PilotStateTransition>, AppError> {
        let history = sqlx::query_as::<_, PilotStateTransition>(
            r#"
            SELECT id, participant_id, from_state, to_state, triggered_by, actor, reason, metadata, created_at
            FROM pilot_state_transitions
            WHERE participant_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(participant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(history)
    }

    async fn record_activity(&self, input: RecordPilotActivityInput) -> Result<PilotActivity, AppError> {
        let activity = sqlx::query_as::<_, PilotActivity>(
            r#"
            INSERT INTO pilot_activities (participant_id, activity_type, activity_data)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(input.participant_id)
        .bind(&input.activity_type)
        .bind(&input.activity_data)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        // Update activity count
        sqlx::query("UPDATE pilot_participants SET activities_count = activities_count + 1 WHERE id = $1")
            .bind(input.participant_id)
            .execute(&self.pool)
            .await
            .ok();

        Ok(activity)
    }

    async fn get_activities(&self, participant_id: Uuid) -> Result<Vec<PilotActivity>, AppError> {
        let activities = sqlx::query_as::<_, PilotActivity>(
            "SELECT * FROM pilot_activities WHERE participant_id = $1 ORDER BY recorded_at",
        )
        .bind(participant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(activities)
    }

    async fn get_pending_d7_check(&self) -> Result<Vec<PilotParticipant>, AppError> {
        let participants = sqlx::query_as::<_, PilotParticipant>(
            r#"
            SELECT * FROM pilot_participants
            WHERE state = 'license_claimed'
              AND license_claimed_at IS NOT NULL
              AND license_claimed_at < NOW() - INTERVAL '7 days'
              AND d7_check_at IS NULL
            ORDER BY license_claimed_at
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(participants)
    }

    async fn get_pending_d30_check(&self) -> Result<Vec<PilotParticipant>, AppError> {
        let participants = sqlx::query_as::<_, PilotParticipant>(
            r#"
            SELECT * FROM pilot_participants
            WHERE state IN ('d7_active', 'd7_inactive')
              AND license_claimed_at IS NOT NULL
              AND license_claimed_at < NOW() - INTERVAL '30 days'
              AND d30_check_at IS NULL
            ORDER BY license_claimed_at
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(participants)
    }

    async fn get_cohort_report(&self, cohort_id: &str) -> Result<CohortReport, AppError> {
        let cohort = self.get_cohort(cohort_id).await?
            .ok_or_else(|| AppError::NotFound("Cohort not found".to_string()))?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM pilot_participants WHERE cohort_id = $1",
        )
        .bind(cohort_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        // Get counts by state - we'll return simplified counts
        let d7_stats: (i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE state IN ('d7_active', 'd30_active', 'd30_inactive', 'graduated')) as active,
                COUNT(*) FILTER (WHERE d7_check_at IS NOT NULL) as checked
            FROM pilot_participants
            WHERE cohort_id = $1
            "#,
        )
        .bind(cohort_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let d7_retention = if d7_stats.1 > 0 {
            Some(d7_stats.0 as f64 / d7_stats.1 as f64)
        } else {
            None
        };

        let d30_stats: (i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE state IN ('d30_active', 'graduated')) as active,
                COUNT(*) FILTER (WHERE d30_check_at IS NOT NULL) as checked
            FROM pilot_participants
            WHERE cohort_id = $1
            "#,
        )
        .bind(cohort_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let d30_retention = if d30_stats.1 > 0 {
            Some(d30_stats.0 as f64 / d30_stats.1 as f64)
        } else {
            None
        };

        Ok(CohortReport {
            cohort,
            total_participants: total.0,
            by_state: vec![], // Simplified for now
            d7_retention_rate: d7_retention,
            d30_retention_rate: d30_retention,
        })
    }

    async fn save_metrics_snapshot(&self, cohort_id: &str) -> Result<PilotMetricsSnapshot, AppError> {
        let snapshot = sqlx::query_as::<_, PilotMetricsSnapshot>(
            r#"
            INSERT INTO pilot_metrics_snapshots (
                cohort_id, snapshot_date,
                total_invited, total_registered, total_claimed,
                total_d7_active, total_d7_inactive,
                total_d30_active, total_d30_inactive,
                total_graduated, total_dropped
            )
            SELECT
                $1, CURRENT_DATE,
                COUNT(*) FILTER (WHERE state = 'invited'),
                COUNT(*) FILTER (WHERE state IN ('registered', 'license_claimed', 'd7_active', 'd7_inactive', 'd30_active', 'd30_inactive', 'graduated')),
                COUNT(*) FILTER (WHERE state IN ('license_claimed', 'd7_active', 'd7_inactive', 'd30_active', 'd30_inactive', 'graduated')),
                COUNT(*) FILTER (WHERE state = 'd7_active'),
                COUNT(*) FILTER (WHERE state = 'd7_inactive'),
                COUNT(*) FILTER (WHERE state = 'd30_active'),
                COUNT(*) FILTER (WHERE state = 'd30_inactive'),
                COUNT(*) FILTER (WHERE state = 'graduated'),
                COUNT(*) FILTER (WHERE state = 'dropped')
            FROM pilot_participants
            WHERE cohort_id = $1
            ON CONFLICT (cohort_id, snapshot_date) DO UPDATE SET
                total_invited = EXCLUDED.total_invited,
                total_registered = EXCLUDED.total_registered,
                total_claimed = EXCLUDED.total_claimed,
                total_d7_active = EXCLUDED.total_d7_active,
                total_d7_inactive = EXCLUDED.total_d7_inactive,
                total_d30_active = EXCLUDED.total_d30_active,
                total_d30_inactive = EXCLUDED.total_d30_inactive,
                total_graduated = EXCLUDED.total_graduated,
                total_dropped = EXCLUDED.total_dropped
            RETURNING *
            "#,
        )
        .bind(cohort_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(snapshot)
    }

    async fn get_metrics_snapshots(&self, cohort_id: &str) -> Result<Vec<PilotMetricsSnapshot>, AppError> {
        let snapshots = sqlx::query_as::<_, PilotMetricsSnapshot>(
            "SELECT * FROM pilot_metrics_snapshots WHERE cohort_id = $1 ORDER BY snapshot_date DESC",
        )
        .bind(cohort_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(snapshots)
    }

    async fn atomic_enroll_with_participant(
        &self,
        input: AddParticipantInput,
    ) -> Result<Result<PilotParticipant, String>, AppError> {
        // F6: Call the updated atomic_pilot_enroll SQL function that also inserts participant
        // Returns: (enrolled BOOLEAN, cohort_id VARCHAR(50), rejection_reason TEXT, participant_id UUID)
        let result: Option<(bool, Option<String>, Option<String>, Option<Uuid>)> = sqlx::query_as(
            "SELECT enrolled, cohort_id, rejection_reason, participant_id FROM atomic_pilot_enroll($1, $2, $3, $4, $5)",
        )
        .bind(&input.external_user_id)
        .bind(&input.market_code)
        .bind(Some(&input.cohort_id))
        .bind(&input.invitation_code)
        .bind(&input.invitation_channel)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        match result {
            // Success: enrolled=true with participant_id
            Some((true, _, _, Some(participant_id))) => {
                // Success - fetch the created participant
                let participant = self.get_participant(participant_id).await?
                    .ok_or_else(|| AppError::InternalServerError(
                        "Participant created but not found".to_string()
                    ))?;
                Ok(Ok(participant))
            }
            // Success but no participant ID (shouldn't happen with F6 fix)
            Some((true, _, _, None)) => {
                Err(AppError::InternalServerError(
                    "Enrollment succeeded but no participant ID returned".to_string()
                ))
            }
            // Rejected with reason
            Some((false, _, Some(rejection_reason), _)) => {
                Ok(Err(rejection_reason))
            }
            // Rejected without specific reason
            Some((false, _, None, _)) => {
                Ok(Err("Enrollment not available".to_string()))
            }
            // No result returned
            None => {
                Ok(Err("No enrollment result returned".to_string()))
            }
        }
    }

    #[allow(deprecated)]
    async fn atomic_enroll(
        &self,
        user_id: &str,
        market_code: &str,
        cohort_id: Option<&str>,
    ) -> Result<Option<(String, Option<String>)>, AppError> {
        // R5-16: Call the atomic_pilot_enroll SQL function (legacy signature)
        // This still works but doesn't create the participant atomically
        // Returns: (enrolled BOOLEAN, cohort_id VARCHAR(50), rejection_reason TEXT, participant_id UUID)
        let result: Option<(bool, Option<String>, Option<String>, Option<Uuid>)> = sqlx::query_as(
            "SELECT enrolled, cohort_id, rejection_reason, participant_id FROM atomic_pilot_enroll($1, $2, $3, NULL, NULL)",
        )
        .bind(user_id)
        .bind(market_code)
        .bind(cohort_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        match result {
            Some((enrolled, Some(assigned_cohort_id), rejection_reason, _)) => {
                if enrolled {
                    Ok(Some((assigned_cohort_id, None)))
                } else {
                    Ok(Some((assigned_cohort_id, rejection_reason)))
                }
            }
            Some((_, None, rejection_reason, _)) => {
                // enrolled=false with no cohort_id means rejection
                Ok(Some((String::new(), rejection_reason)))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_display() {
        assert_eq!(PilotParticipantState::Invited.to_string(), "invited");
        assert_eq!(PilotParticipantState::D7Active.to_string(), "d7_active");
        assert_eq!(PilotParticipantState::Graduated.to_string(), "graduated");
    }
}
