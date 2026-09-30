//! Cohort repository for D1/D7/D30 tracking
//!
//! Implements CRUD operations for:
//! - Participant cohorts (activation tracking)
//! - Daily activity records
//! - Cohort notifications
//! - Analytics and reporting

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Participant cohort tracking D1/D3/D7/D30 milestones
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ParticipantCohort {
    pub id: Uuid,
    pub user_id: String,
    pub license_id: String,
    pub cohort_date: NaiveDate,

    // D1: Installation/activation
    pub d1_completed: bool,
    pub d1_completed_at: Option<DateTime<Utc>>,

    // D3: Activity/data-cost
    pub d3_completed: bool,
    pub d3_completed_at: Option<DateTime<Utc>>,
    pub d3_activity_count: i32,

    // D7: Productivity (4 of 7 days active)
    pub d7_completed: bool,
    pub d7_completed_at: Option<DateTime<Utc>>,
    pub d7_active_days: i32,
    pub d7_active_dates: Vec<NaiveDate>,

    // D30: Retention
    pub d30_completed: bool,
    pub d30_completed_at: Option<DateTime<Utc>>,
    pub d30_active_days: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Daily activity record for a cohort participant
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CohortDailyActivity {
    pub id: Uuid,
    pub cohort_id: Uuid,
    pub activity_date: NaiveDate,
    pub day_number: i32,
    pub sessions_count: i32,
    pub tasks_completed: i32,
    pub earnings_micros: i64,
    pub data_collected_bytes: i64,
    pub device_type: Option<String>,
    pub app_version: Option<String>,
    pub recorded_at: DateTime<Utc>,
}

/// Notification scheduled for cohort milestone
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CohortNotification {
    pub id: Uuid,
    pub cohort_id: Uuid,
    pub notification_type: String,
    pub scheduled_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
    pub channel: String,
    pub status: String,
    pub skip_reason: Option<String>,
    pub external_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Aggregated cohort analytics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CohortAnalytics {
    pub id: Uuid,
    pub cohort_date: NaiveDate,
    pub total_participants: i32,
    pub d1_completed_count: i32,
    pub d3_completed_count: i32,
    pub d7_completed_count: i32,
    pub d30_completed_count: i32,
    pub d1_rate: f64,
    pub d3_rate: f64,
    pub d7_rate: f64,
    pub d30_rate: f64,
    pub avg_d7_active_days: f64,
    pub avg_d30_active_days: f64,
    pub total_earnings_micros: i64,
    pub country_code: Option<String>,
    pub calculated_at: DateTime<Utc>,
}

// ============================================
// INPUT STRUCTS
// ============================================

/// Input for creating a new cohort entry
#[derive(Debug, Clone)]
pub struct CreateCohortInput {
    pub user_id: String,
    pub license_id: String,
    pub cohort_date: NaiveDate,
}

/// Input for recording daily activity
#[derive(Debug, Clone)]
pub struct RecordActivityInput {
    pub cohort_id: Uuid,
    pub activity_date: NaiveDate,
    pub sessions_count: i32,
    pub tasks_completed: i32,
    pub earnings_micros: i64,
    pub data_collected_bytes: i64,
    pub device_type: Option<String>,
    pub app_version: Option<String>,
}

/// Input for scheduling a notification
#[derive(Debug, Clone)]
pub struct ScheduleNotificationInput {
    pub cohort_id: Uuid,
    pub notification_type: String,
    pub scheduled_at: DateTime<Utc>,
    pub channel: String,
}

/// Progress summary for D7 milestone
#[derive(Debug, Clone, Serialize)]
pub struct D7Progress {
    pub cohort_id: Uuid,
    pub active_days: i32,
    pub active_dates: Vec<NaiveDate>,
    pub days_remaining: i32,
    pub is_complete: bool,
    pub deadline: NaiveDate,
}

/// Progress summary for D30 milestone
#[derive(Debug, Clone, Serialize)]
pub struct D30Progress {
    pub cohort_id: Uuid,
    pub active_days: i32,
    pub days_elapsed: i32,
    pub is_complete: bool,
    pub deadline: NaiveDate,
}

/// Cohort statistics summary
#[derive(Debug, Clone, Serialize)]
pub struct CohortStats {
    pub cohort_date: NaiveDate,
    pub total_participants: i64,
    pub d1_completed: i64,
    pub d3_completed: i64,
    pub d7_completed: i64,
    pub d30_completed: i64,
    pub d1_rate_pct: f64,
    pub d3_rate_pct: f64,
    pub d7_rate_pct: f64,
    pub d30_rate_pct: f64,
}

// ============================================
// TRAIT DEFINITION
// ============================================

/// Dynamic type alias for CohortRepository trait object
pub type DynCohortRepository = Arc<dyn CohortRepository + Send + Sync>;

/// Cohort repository trait defining database operations
#[async_trait]
pub trait CohortRepository: Send + Sync {
    // --- Cohort CRUD ---

    /// Create a new cohort entry for a participant
    async fn create_cohort(&self, input: CreateCohortInput) -> Result<ParticipantCohort, AppError>;

    /// Get cohort by ID
    async fn get_cohort(&self, id: Uuid) -> Result<Option<ParticipantCohort>, AppError>;

    /// Get cohort by user and license
    async fn get_cohort_by_user_license(
        &self,
        user_id: &str,
        license_id: &str,
    ) -> Result<Option<ParticipantCohort>, AppError>;

    /// Get all cohorts for a user
    async fn get_user_cohorts(&self, user_id: &str) -> Result<Vec<ParticipantCohort>, AppError>;

    /// Get cohorts by date range
    async fn get_cohorts_by_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<ParticipantCohort>, AppError>;

    // --- Milestone Completion ---

    /// Mark D1 completed
    async fn complete_d1(&self, cohort_id: Uuid) -> Result<ParticipantCohort, AppError>;

    /// Mark D3 completed with activity count
    async fn complete_d3(&self, cohort_id: Uuid, activity_count: i32) -> Result<ParticipantCohort, AppError>;

    /// Update D7 progress and check completion
    async fn update_d7_progress(
        &self,
        cohort_id: Uuid,
        active_date: NaiveDate,
    ) -> Result<ParticipantCohort, AppError>;

    /// Check and mark D7 completion if criteria met
    async fn check_d7_completion(&self, cohort_id: Uuid) -> Result<bool, AppError>;

    /// Update D30 progress
    async fn update_d30_progress(
        &self,
        cohort_id: Uuid,
        active_days: i32,
    ) -> Result<ParticipantCohort, AppError>;

    // --- Daily Activity ---

    /// Record daily activity
    async fn record_activity(&self, input: RecordActivityInput) -> Result<CohortDailyActivity, AppError>;

    /// Get daily activities for cohort
    async fn get_cohort_activities(&self, cohort_id: Uuid) -> Result<Vec<CohortDailyActivity>, AppError>;

    /// Get activity for specific date
    async fn get_activity_for_date(
        &self,
        cohort_id: Uuid,
        date: NaiveDate,
    ) -> Result<Option<CohortDailyActivity>, AppError>;

    // --- Notifications ---

    /// Schedule a notification
    async fn schedule_notification(&self, input: ScheduleNotificationInput) -> Result<CohortNotification, AppError>;

    /// Get pending notifications
    async fn get_pending_notifications(&self, limit: i32) -> Result<Vec<CohortNotification>, AppError>;

    /// Mark notification sent
    async fn mark_notification_sent(
        &self,
        notification_id: Uuid,
        external_id: Option<&str>,
    ) -> Result<CohortNotification, AppError>;

    /// Skip notification with reason
    async fn skip_notification(
        &self,
        notification_id: Uuid,
        reason: &str,
    ) -> Result<CohortNotification, AppError>;

    /// Get notifications for cohort
    async fn get_cohort_notifications(&self, cohort_id: Uuid) -> Result<Vec<CohortNotification>, AppError>;

    // --- Progress Queries ---

    /// Calculate D7 progress
    async fn calculate_d7_progress(&self, cohort_id: Uuid) -> Result<D7Progress, AppError>;

    /// Calculate D30 progress
    async fn calculate_d30_progress(&self, cohort_id: Uuid) -> Result<D30Progress, AppError>;

    // --- Analytics ---

    /// Get cohort statistics for a date
    async fn get_cohort_stats(&self, cohort_date: NaiveDate) -> Result<CohortStats, AppError>;

    /// Calculate and store cohort analytics
    async fn calculate_analytics(
        &self,
        cohort_date: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<CohortAnalytics, AppError>;

    /// Get stored analytics
    async fn get_analytics(
        &self,
        cohort_date: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<Option<CohortAnalytics>, AppError>;

    // --- Batch Operations ---

    /// Get cohorts needing D1 check
    async fn get_cohorts_pending_d1(&self, limit: i32) -> Result<Vec<ParticipantCohort>, AppError>;

    /// Get cohorts needing D7 check (between day 1-7)
    async fn get_cohorts_pending_d7(&self, limit: i32) -> Result<Vec<ParticipantCohort>, AppError>;

    /// Get cohorts needing D30 check (between day 1-30)
    async fn get_cohorts_pending_d30(&self, limit: i32) -> Result<Vec<ParticipantCohort>, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct CohortRepositoryImpl {
    pool: ConnectionPool,
}

impl CohortRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CohortRepository for CohortRepositoryImpl {
    async fn create_cohort(&self, input: CreateCohortInput) -> Result<ParticipantCohort, AppError> {
        let cohort = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            INSERT INTO participant_cohorts (user_id, license_id, cohort_date)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, license_id, cohort_date,
                      d1_completed, d1_completed_at,
                      d3_completed, d3_completed_at, d3_activity_count,
                      d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                      d30_completed, d30_completed_at, d30_active_days,
                      created_at, updated_at
            "#,
        )
        .bind(&input.user_id)
        .bind(&input.license_id)
        .bind(input.cohort_date)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(cohort)
    }

    async fn get_cohort(&self, id: Uuid) -> Result<Option<ParticipantCohort>, AppError> {
        let cohort = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            SELECT id, user_id, license_id, cohort_date,
                   d1_completed, d1_completed_at,
                   d3_completed, d3_completed_at, d3_activity_count,
                   d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                   d30_completed, d30_completed_at, d30_active_days,
                   created_at, updated_at
            FROM participant_cohorts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(cohort)
    }

    async fn get_cohort_by_user_license(
        &self,
        user_id: &str,
        license_id: &str,
    ) -> Result<Option<ParticipantCohort>, AppError> {
        let cohort = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            SELECT id, user_id, license_id, cohort_date,
                   d1_completed, d1_completed_at,
                   d3_completed, d3_completed_at, d3_activity_count,
                   d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                   d30_completed, d30_completed_at, d30_active_days,
                   created_at, updated_at
            FROM participant_cohorts
            WHERE user_id = $1 AND license_id = $2
            "#,
        )
        .bind(user_id)
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(cohort)
    }

    async fn get_user_cohorts(&self, user_id: &str) -> Result<Vec<ParticipantCohort>, AppError> {
        let cohorts = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            SELECT id, user_id, license_id, cohort_date,
                   d1_completed, d1_completed_at,
                   d3_completed, d3_completed_at, d3_activity_count,
                   d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                   d30_completed, d30_completed_at, d30_active_days,
                   created_at, updated_at
            FROM participant_cohorts
            WHERE user_id = $1
            ORDER BY cohort_date DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(cohorts)
    }

    async fn get_cohorts_by_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<ParticipantCohort>, AppError> {
        let cohorts = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            SELECT id, user_id, license_id, cohort_date,
                   d1_completed, d1_completed_at,
                   d3_completed, d3_completed_at, d3_activity_count,
                   d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                   d30_completed, d30_completed_at, d30_active_days,
                   created_at, updated_at
            FROM participant_cohorts
            WHERE cohort_date >= $1 AND cohort_date <= $2
            ORDER BY cohort_date DESC
            "#,
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(cohorts)
    }

    async fn complete_d1(&self, cohort_id: Uuid) -> Result<ParticipantCohort, AppError> {
        let cohort = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            UPDATE participant_cohorts
            SET d1_completed = TRUE,
                d1_completed_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, cohort_date,
                      d1_completed, d1_completed_at,
                      d3_completed, d3_completed_at, d3_activity_count,
                      d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                      d30_completed, d30_completed_at, d30_active_days,
                      created_at, updated_at
            "#,
        )
        .bind(cohort_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(cohort)
    }

    async fn complete_d3(&self, cohort_id: Uuid, activity_count: i32) -> Result<ParticipantCohort, AppError> {
        let cohort = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            UPDATE participant_cohorts
            SET d3_completed = TRUE,
                d3_completed_at = NOW(),
                d3_activity_count = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, cohort_date,
                      d1_completed, d1_completed_at,
                      d3_completed, d3_completed_at, d3_activity_count,
                      d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                      d30_completed, d30_completed_at, d30_active_days,
                      created_at, updated_at
            "#,
        )
        .bind(cohort_id)
        .bind(activity_count)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(cohort)
    }

    async fn update_d7_progress(
        &self,
        cohort_id: Uuid,
        active_date: NaiveDate,
    ) -> Result<ParticipantCohort, AppError> {
        // Add date to array if not already present, and increment count
        let cohort = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            UPDATE participant_cohorts
            SET d7_active_dates = CASE
                    WHEN $2 = ANY(d7_active_dates) THEN d7_active_dates
                    ELSE array_append(d7_active_dates, $2)
                END,
                d7_active_days = CASE
                    WHEN $2 = ANY(d7_active_dates) THEN d7_active_days
                    ELSE d7_active_days + 1
                END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, cohort_date,
                      d1_completed, d1_completed_at,
                      d3_completed, d3_completed_at, d3_activity_count,
                      d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                      d30_completed, d30_completed_at, d30_active_days,
                      created_at, updated_at
            "#,
        )
        .bind(cohort_id)
        .bind(active_date)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Check if D7 should be marked complete (4+ days active in first 7 days)
        if cohort.d7_active_days >= 4 && !cohort.d7_completed {
            return self.check_d7_completion(cohort_id).await.map(|_| cohort);
        }

        Ok(cohort)
    }

    async fn check_d7_completion(&self, cohort_id: Uuid) -> Result<bool, AppError> {
        // Use database function for atomic check
        let (completed,): (bool,) = sqlx::query_as("SELECT check_d7_completion($1)")
            .bind(cohort_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(completed)
    }

    async fn update_d30_progress(
        &self,
        cohort_id: Uuid,
        active_days: i32,
    ) -> Result<ParticipantCohort, AppError> {
        // Check if should be marked complete (any activity within 30 days counts)
        let cohort = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            UPDATE participant_cohorts
            SET d30_active_days = $2,
                d30_completed = CASE WHEN $2 >= 1 AND cohort_date + INTERVAL '30 days' <= NOW() THEN TRUE ELSE d30_completed END,
                d30_completed_at = CASE WHEN $2 >= 1 AND cohort_date + INTERVAL '30 days' <= NOW() AND d30_completed = FALSE THEN NOW() ELSE d30_completed_at END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, license_id, cohort_date,
                      d1_completed, d1_completed_at,
                      d3_completed, d3_completed_at, d3_activity_count,
                      d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                      d30_completed, d30_completed_at, d30_active_days,
                      created_at, updated_at
            "#,
        )
        .bind(cohort_id)
        .bind(active_days)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(cohort)
    }

    async fn record_activity(&self, input: RecordActivityInput) -> Result<CohortDailyActivity, AppError> {
        // Get cohort to calculate day number
        let cohort = self.get_cohort(input.cohort_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Cohort {} not found", input.cohort_id)))?;

        let day_number = (input.activity_date - cohort.cohort_date).num_days() as i32 + 1;

        // R4-03: Derive is_productive from earnings (any earnings = productive day)
        let is_productive = input.earnings_micros > 0;

        let activity = sqlx::query_as::<_, CohortDailyActivity>(
            r#"
            INSERT INTO cohort_daily_activity (
                cohort_id, activity_date, day_number, sessions_count,
                tasks_completed, earnings_micros, data_collected_bytes,
                device_type, app_version, is_productive
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (cohort_id, activity_date)
            DO UPDATE SET
                sessions_count = cohort_daily_activity.sessions_count + EXCLUDED.sessions_count,
                tasks_completed = cohort_daily_activity.tasks_completed + EXCLUDED.tasks_completed,
                earnings_micros = cohort_daily_activity.earnings_micros + EXCLUDED.earnings_micros,
                data_collected_bytes = cohort_daily_activity.data_collected_bytes + EXCLUDED.data_collected_bytes,
                device_type = COALESCE(EXCLUDED.device_type, cohort_daily_activity.device_type),
                app_version = COALESCE(EXCLUDED.app_version, cohort_daily_activity.app_version),
                is_productive = cohort_daily_activity.is_productive OR EXCLUDED.is_productive,
                updated_at = NOW()
            RETURNING id, cohort_id, activity_date, day_number, sessions_count,
                      tasks_completed, earnings_micros, data_collected_bytes,
                      device_type, app_version, recorded_at
            "#,
        )
        .bind(input.cohort_id)
        .bind(input.activity_date)
        .bind(day_number)
        .bind(input.sessions_count)
        .bind(input.tasks_completed)
        .bind(input.earnings_micros)
        .bind(input.data_collected_bytes)
        .bind(&input.device_type)
        .bind(&input.app_version)
        .bind(is_productive)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Update D7 progress if within first 7 days
        if day_number <= 7 {
            self.update_d7_progress(input.cohort_id, input.activity_date).await?;
        }

        Ok(activity)
    }

    async fn get_cohort_activities(&self, cohort_id: Uuid) -> Result<Vec<CohortDailyActivity>, AppError> {
        let activities = sqlx::query_as::<_, CohortDailyActivity>(
            r#"
            SELECT id, cohort_id, activity_date, day_number, sessions_count,
                   tasks_completed, earnings_micros, data_collected_bytes,
                   device_type, app_version, recorded_at
            FROM cohort_daily_activity
            WHERE cohort_id = $1
            ORDER BY activity_date ASC
            "#,
        )
        .bind(cohort_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(activities)
    }

    async fn get_activity_for_date(
        &self,
        cohort_id: Uuid,
        date: NaiveDate,
    ) -> Result<Option<CohortDailyActivity>, AppError> {
        let activity = sqlx::query_as::<_, CohortDailyActivity>(
            r#"
            SELECT id, cohort_id, activity_date, day_number, sessions_count,
                   tasks_completed, earnings_micros, data_collected_bytes,
                   device_type, app_version, recorded_at
            FROM cohort_daily_activity
            WHERE cohort_id = $1 AND activity_date = $2
            "#,
        )
        .bind(cohort_id)
        .bind(date)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(activity)
    }

    async fn schedule_notification(&self, input: ScheduleNotificationInput) -> Result<CohortNotification, AppError> {
        let notification = sqlx::query_as::<_, CohortNotification>(
            r#"
            INSERT INTO cohort_notifications (cohort_id, notification_type, scheduled_at, channel)
            VALUES ($1, $2, $3, $4)
            RETURNING id, cohort_id, notification_type, scheduled_at, sent_at, channel, status, skip_reason, external_id, created_at
            "#,
        )
        .bind(input.cohort_id)
        .bind(&input.notification_type)
        .bind(input.scheduled_at)
        .bind(&input.channel)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(notification)
    }

    async fn get_pending_notifications(&self, limit: i32) -> Result<Vec<CohortNotification>, AppError> {
        let notifications = sqlx::query_as::<_, CohortNotification>(
            r#"
            SELECT id, cohort_id, notification_type, scheduled_at, sent_at, channel, status, skip_reason, external_id, created_at
            FROM cohort_notifications
            WHERE status = 'pending' AND scheduled_at <= NOW()
            ORDER BY scheduled_at ASC
            LIMIT $1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(notifications)
    }

    async fn mark_notification_sent(
        &self,
        notification_id: Uuid,
        external_id: Option<&str>,
    ) -> Result<CohortNotification, AppError> {
        let notification = sqlx::query_as::<_, CohortNotification>(
            r#"
            UPDATE cohort_notifications
            SET status = 'sent', sent_at = NOW(), external_id = $2
            WHERE id = $1
            RETURNING id, cohort_id, notification_type, scheduled_at, sent_at, channel, status, skip_reason, external_id, created_at
            "#,
        )
        .bind(notification_id)
        .bind(external_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(notification)
    }

    async fn skip_notification(
        &self,
        notification_id: Uuid,
        reason: &str,
    ) -> Result<CohortNotification, AppError> {
        let notification = sqlx::query_as::<_, CohortNotification>(
            r#"
            UPDATE cohort_notifications
            SET status = 'skipped', skip_reason = $2
            WHERE id = $1
            RETURNING id, cohort_id, notification_type, scheduled_at, sent_at, channel, status, skip_reason, external_id, created_at
            "#,
        )
        .bind(notification_id)
        .bind(reason)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(notification)
    }

    async fn get_cohort_notifications(&self, cohort_id: Uuid) -> Result<Vec<CohortNotification>, AppError> {
        let notifications = sqlx::query_as::<_, CohortNotification>(
            r#"
            SELECT id, cohort_id, notification_type, scheduled_at, sent_at, channel, status, skip_reason, external_id, created_at
            FROM cohort_notifications
            WHERE cohort_id = $1
            ORDER BY scheduled_at ASC
            "#,
        )
        .bind(cohort_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(notifications)
    }

    async fn calculate_d7_progress(&self, cohort_id: Uuid) -> Result<D7Progress, AppError> {
        let cohort = self.get_cohort(cohort_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Cohort {} not found", cohort_id)))?;

        let deadline = cohort.cohort_date + chrono::Duration::days(7);
        let today = Utc::now().date_naive();
        let days_remaining = (deadline - today).num_days().max(0) as i32;

        Ok(D7Progress {
            cohort_id,
            active_days: cohort.d7_active_days,
            active_dates: cohort.d7_active_dates,
            days_remaining,
            is_complete: cohort.d7_completed,
            deadline,
        })
    }

    async fn calculate_d30_progress(&self, cohort_id: Uuid) -> Result<D30Progress, AppError> {
        let cohort = self.get_cohort(cohort_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Cohort {} not found", cohort_id)))?;

        let deadline = cohort.cohort_date + chrono::Duration::days(30);
        let today = Utc::now().date_naive();
        let days_elapsed = (today - cohort.cohort_date).num_days().min(30) as i32;

        Ok(D30Progress {
            cohort_id,
            active_days: cohort.d30_active_days,
            days_elapsed,
            is_complete: cohort.d30_completed,
            deadline,
        })
    }

    async fn get_cohort_stats(&self, cohort_date: NaiveDate) -> Result<CohortStats, AppError> {
        let (total,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM participant_cohorts WHERE cohort_date = $1",
        )
        .bind(cohort_date)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let (d1_completed,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM participant_cohorts WHERE cohort_date = $1 AND d1_completed = TRUE",
        )
        .bind(cohort_date)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let (d3_completed,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM participant_cohorts WHERE cohort_date = $1 AND d3_completed = TRUE",
        )
        .bind(cohort_date)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let (d7_completed,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM participant_cohorts WHERE cohort_date = $1 AND d7_completed = TRUE",
        )
        .bind(cohort_date)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let (d30_completed,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM participant_cohorts WHERE cohort_date = $1 AND d30_completed = TRUE",
        )
        .bind(cohort_date)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let total_f64 = total.max(1) as f64;

        Ok(CohortStats {
            cohort_date,
            total_participants: total,
            d1_completed,
            d3_completed,
            d7_completed,
            d30_completed,
            d1_rate_pct: (d1_completed as f64 / total_f64) * 100.0,
            d3_rate_pct: (d3_completed as f64 / total_f64) * 100.0,
            d7_rate_pct: (d7_completed as f64 / total_f64) * 100.0,
            d30_rate_pct: (d30_completed as f64 / total_f64) * 100.0,
        })
    }

    async fn calculate_analytics(
        &self,
        cohort_date: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<CohortAnalytics, AppError> {
        // Use database function for calculation
        let analytics = sqlx::query_as::<_, CohortAnalytics>(
            r#"
            SELECT * FROM calculate_cohort_analytics($1, $2)
            "#,
        )
        .bind(cohort_date)
        .bind(country_code)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(analytics)
    }

    async fn get_analytics(
        &self,
        cohort_date: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<Option<CohortAnalytics>, AppError> {
        let analytics = if let Some(cc) = country_code {
            sqlx::query_as::<_, CohortAnalytics>(
                r#"
                SELECT id, cohort_date, total_participants, d1_completed_count, d3_completed_count,
                       d7_completed_count, d30_completed_count, d1_rate, d3_rate, d7_rate, d30_rate,
                       avg_d7_active_days, avg_d30_active_days, total_earnings_micros, country_code, calculated_at
                FROM cohort_analytics
                WHERE cohort_date = $1 AND country_code = $2
                ORDER BY calculated_at DESC
                LIMIT 1
                "#,
            )
            .bind(cohort_date)
            .bind(cc)
            .fetch_optional(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, CohortAnalytics>(
                r#"
                SELECT id, cohort_date, total_participants, d1_completed_count, d3_completed_count,
                       d7_completed_count, d30_completed_count, d1_rate, d3_rate, d7_rate, d30_rate,
                       avg_d7_active_days, avg_d30_active_days, total_earnings_micros, country_code, calculated_at
                FROM cohort_analytics
                WHERE cohort_date = $1 AND country_code IS NULL
                ORDER BY calculated_at DESC
                LIMIT 1
                "#,
            )
            .bind(cohort_date)
            .fetch_optional(&self.pool)
            .await
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(analytics)
    }

    async fn get_cohorts_pending_d1(&self, limit: i32) -> Result<Vec<ParticipantCohort>, AppError> {
        let cohorts = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            SELECT id, user_id, license_id, cohort_date,
                   d1_completed, d1_completed_at,
                   d3_completed, d3_completed_at, d3_activity_count,
                   d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                   d30_completed, d30_completed_at, d30_active_days,
                   created_at, updated_at
            FROM participant_cohorts
            WHERE d1_completed = FALSE
              AND cohort_date >= CURRENT_DATE - INTERVAL '1 day'
            ORDER BY created_at ASC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(cohorts)
    }

    async fn get_cohorts_pending_d7(&self, limit: i32) -> Result<Vec<ParticipantCohort>, AppError> {
        let cohorts = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            SELECT id, user_id, license_id, cohort_date,
                   d1_completed, d1_completed_at,
                   d3_completed, d3_completed_at, d3_activity_count,
                   d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                   d30_completed, d30_completed_at, d30_active_days,
                   created_at, updated_at
            FROM participant_cohorts
            WHERE d7_completed = FALSE
              AND d1_completed = TRUE
              AND cohort_date >= CURRENT_DATE - INTERVAL '7 days'
              AND cohort_date <= CURRENT_DATE - INTERVAL '1 day'
            ORDER BY cohort_date ASC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(cohorts)
    }

    async fn get_cohorts_pending_d30(&self, limit: i32) -> Result<Vec<ParticipantCohort>, AppError> {
        let cohorts = sqlx::query_as::<_, ParticipantCohort>(
            r#"
            SELECT id, user_id, license_id, cohort_date,
                   d1_completed, d1_completed_at,
                   d3_completed, d3_completed_at, d3_activity_count,
                   d7_completed, d7_completed_at, d7_active_days, d7_active_dates,
                   d30_completed, d30_completed_at, d30_active_days,
                   created_at, updated_at
            FROM participant_cohorts
            WHERE d30_completed = FALSE
              AND d7_completed = TRUE
              AND cohort_date >= CURRENT_DATE - INTERVAL '30 days'
              AND cohort_date <= CURRENT_DATE - INTERVAL '7 days'
            ORDER BY cohort_date ASC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(cohorts)
    }
}
