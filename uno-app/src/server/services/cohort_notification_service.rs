//! Cohort Notification Service for D1/D3/D7/D30 milestone notifications
//!
//! Handles:
//! - Creating notification jobs for cohort milestones
//! - Processing notification jobs with consent checks
//! - Scheduling notifications at appropriate local times
//! - Deduplication and current state validation

use std::sync::Arc;
use chrono::{DateTime, Duration, NaiveDate, NaiveTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::repositories::{
    DynCohortRepository, DynConsentRepository,
    ParticipantCohort, CohortNotification,
};
use crate::server::services::{
    JobService, CreateJobInput, JobPriority, JobResult,
};
use crate::types::AppError;

// ============================================
// NOTIFICATION TYPES
// ============================================

/// Notification type for cohort milestones
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CohortNotificationType {
    /// D1: Installation reminder (sent day after activation if not installed)
    D1Reminder,
    /// D3: Activity reminder (sent if no activity by day 3)
    D3Reminder,
    /// D7: Productivity check (sent on day 5 if < 3 days active)
    D7Warning,
    /// D7: Success notification (sent on day 7 if 4+ days active)
    D7Success,
    /// D30: Retention reminder (sent on day 25 if < 20 days active)
    D30Warning,
    /// D30: Success/renewal notification
    D30Success,
}

impl CohortNotificationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::D1Reminder => "d1_reminder",
            Self::D3Reminder => "d3_reminder",
            Self::D7Warning => "d7_warning",
            Self::D7Success => "d7_success",
            Self::D30Warning => "d30_warning",
            Self::D30Success => "d30_success",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "d1_reminder" => Some(Self::D1Reminder),
            "d3_reminder" => Some(Self::D3Reminder),
            "d7_warning" => Some(Self::D7Warning),
            "d7_success" => Some(Self::D7Success),
            "d30_warning" => Some(Self::D30Warning),
            "d30_success" => Some(Self::D30Success),
            _ => None,
        }
    }

    /// Get the day offset from cohort date when this notification should be sent
    pub fn day_offset(&self) -> i64 {
        match self {
            Self::D1Reminder => 1,
            Self::D3Reminder => 3,
            Self::D7Warning => 5,
            Self::D7Success => 7,
            Self::D30Warning => 25,
            Self::D30Success => 30,
        }
    }

    /// Get preferred send time (local time)
    pub fn preferred_hour(&self) -> u32 {
        match self {
            Self::D1Reminder => 10, // 10 AM
            Self::D3Reminder => 10,
            Self::D7Warning => 9,   // 9 AM
            Self::D7Success => 11,  // 11 AM
            Self::D30Warning => 9,
            Self::D30Success => 11,
        }
    }

    /// Get email template ID
    pub fn email_template(&self) -> &'static str {
        match self {
            Self::D1Reminder => "cohort_d1_reminder",
            Self::D3Reminder => "cohort_d3_reminder",
            Self::D7Warning => "cohort_d7_warning",
            Self::D7Success => "cohort_d7_success",
            Self::D30Warning => "cohort_d30_warning",
            Self::D30Success => "cohort_d30_success",
        }
    }

    /// Get push notification template ID
    pub fn push_template(&self) -> &'static str {
        match self {
            Self::D1Reminder => "cohort_d1_push",
            Self::D3Reminder => "cohort_d3_push",
            Self::D7Warning => "cohort_d7_push",
            Self::D7Success => "cohort_d7_push",
            Self::D30Warning => "cohort_d30_push",
            Self::D30Success => "cohort_d30_push",
        }
    }
}

/// Notification channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannel {
    Email,
    Push,
    Sms,
}

impl NotificationChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Push => "push",
            Self::Sms => "sms",
        }
    }
}

// ============================================
// JOB PAYLOADS
// ============================================

/// Job type constants
pub const JOB_TYPE_COHORT_NOTIFICATION: &str = "cohort_notification";
pub const JOB_TYPE_COHORT_MILESTONE_CHECK: &str = "cohort_milestone_check";
pub const JOB_TYPE_COHORT_SCHEDULE_NOTIFICATIONS: &str = "cohort_schedule_notifications";

/// Payload for a cohort notification job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortNotificationPayload {
    pub notification_id: Uuid,
    pub cohort_id: Uuid,
    pub user_id: String,
    pub notification_type: String,
    pub channel: String,
    pub attempt: i32,
}

/// Payload for a milestone check job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneCheckPayload {
    pub cohort_id: Uuid,
    pub milestone: String, // "d1", "d3", "d7", "d30"
}

/// Payload for scheduling notifications job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleNotificationsPayload {
    pub cohort_id: Uuid,
    pub user_timezone: Option<String>,
}

// ============================================
// SERVICE
// ============================================

/// Configuration for the cohort notification service
#[derive(Debug, Clone)]
pub struct CohortNotificationConfig {
    /// Default timezone for users without preference
    pub default_timezone: String,
    /// Maximum retries for failed notifications
    pub max_retries: i32,
    /// Minimum hours between duplicate notification attempts
    pub dedup_window_hours: i64,
}

impl Default for CohortNotificationConfig {
    fn default() -> Self {
        Self {
            default_timezone: "UTC".to_string(),
            max_retries: 3,
            dedup_window_hours: 24,
        }
    }
}

/// Cohort notification service for managing D1/D3/D7/D30 notifications
#[derive(Clone)]
pub struct CohortNotificationService {
    cohort_repo: DynCohortRepository,
    consent_repo: DynConsentRepository,
    job_service: JobService,
    config: CohortNotificationConfig,
}

impl CohortNotificationService {
    pub fn new(
        cohort_repo: DynCohortRepository,
        consent_repo: DynConsentRepository,
        job_service: JobService,
    ) -> Self {
        Self {
            cohort_repo,
            consent_repo,
            job_service,
            config: CohortNotificationConfig::default(),
        }
    }

    pub fn with_config(mut self, config: CohortNotificationConfig) -> Self {
        self.config = config;
        self
    }

    // ============================================
    // SCHEDULING
    // ============================================

    /// Schedule all notifications for a new cohort
    pub async fn schedule_cohort_notifications(
        &self,
        cohort_id: Uuid,
        user_timezone: Option<&str>,
    ) -> Result<Vec<CohortNotification>, AppError> {
        let cohort = self.cohort_repo.get_cohort(cohort_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Cohort {} not found", cohort_id)))?;

        let tz = user_timezone.unwrap_or(&self.config.default_timezone);
        let mut notifications = Vec::new();

        // Schedule D1 reminder
        if !cohort.d1_completed {
            let notif = self.schedule_notification(
                &cohort,
                CohortNotificationType::D1Reminder,
                NotificationChannel::Email,
                tz,
            ).await?;
            notifications.push(notif);
        }

        // Schedule D3 reminder
        if !cohort.d3_completed {
            let notif = self.schedule_notification(
                &cohort,
                CohortNotificationType::D3Reminder,
                NotificationChannel::Email,
                tz,
            ).await?;
            notifications.push(notif);
        }

        // Schedule D7 warning (day 5)
        if !cohort.d7_completed {
            let notif = self.schedule_notification(
                &cohort,
                CohortNotificationType::D7Warning,
                NotificationChannel::Email,
                tz,
            ).await?;
            notifications.push(notif);
        }

        // Schedule D30 warning (day 25)
        if !cohort.d30_completed {
            let notif = self.schedule_notification(
                &cohort,
                CohortNotificationType::D30Warning,
                NotificationChannel::Email,
                tz,
            ).await?;
            notifications.push(notif);
        }

        Ok(notifications)
    }

    /// Schedule a single notification
    async fn schedule_notification(
        &self,
        cohort: &ParticipantCohort,
        notification_type: CohortNotificationType,
        channel: NotificationChannel,
        timezone: &str,
    ) -> Result<CohortNotification, AppError> {
        let scheduled_at = self.calculate_send_time(
            cohort.cohort_date,
            notification_type.day_offset(),
            notification_type.preferred_hour(),
            timezone,
        )?;

        let input = crate::server::repositories::ScheduleNotificationInput {
            cohort_id: cohort.id,
            notification_type: notification_type.as_str().to_string(),
            scheduled_at,
            channel: channel.as_str().to_string(),
        };

        self.cohort_repo.schedule_notification(input).await
    }

    /// Calculate send time in UTC for a notification
    fn calculate_send_time(
        &self,
        cohort_date: NaiveDate,
        day_offset: i64,
        preferred_hour: u32,
        _timezone: &str,
    ) -> Result<DateTime<Utc>, AppError> {
        // Calculate target date
        let target_date = cohort_date + chrono::Duration::days(day_offset);

        // Create time at preferred hour (simplified - assumes UTC for now)
        let target_time = NaiveTime::from_hms_opt(preferred_hour, 0, 0)
            .ok_or_else(|| AppError::ValidationError("Invalid time".to_string()))?;

        let naive_dt = target_date.and_time(target_time);

        // Convert to UTC (simplified - full implementation would use chrono-tz)
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc))
    }

    // ============================================
    // JOB CREATION
    // ============================================

    /// Create a job to send a pending notification
    pub async fn create_notification_job(
        &self,
        notification: &CohortNotification,
        cohort: &ParticipantCohort,
    ) -> Result<(), AppError> {
        let payload = CohortNotificationPayload {
            notification_id: notification.id,
            cohort_id: cohort.id,
            user_id: cohort.user_id.clone(),
            notification_type: notification.notification_type.clone(),
            channel: notification.channel.clone(),
            attempt: 1,
        };

        let idempotency_key = format!(
            "cohort-notif-{}-{}",
            notification.id,
            notification.notification_type
        );

        self.job_service.create_job(CreateJobInput {
            job_type: JOB_TYPE_COHORT_NOTIFICATION.to_string(),
            payload,
            priority: JobPriority::Normal,
            idempotency_key: Some(idempotency_key),
            correlation_id: Some(notification.id.to_string()),
            causation_id: None,
            config: None,
        }).await?;

        Ok(())
    }

    /// Create a job to check milestone completion
    pub async fn create_milestone_check_job(
        &self,
        cohort_id: Uuid,
        milestone: &str,
    ) -> Result<(), AppError> {
        let payload = MilestoneCheckPayload {
            cohort_id,
            milestone: milestone.to_string(),
        };

        let idempotency_key = format!(
            "milestone-check-{}-{}-{}",
            cohort_id,
            milestone,
            Utc::now().format("%Y%m%d")
        );

        self.job_service.create_job(CreateJobInput {
            job_type: JOB_TYPE_COHORT_MILESTONE_CHECK.to_string(),
            payload,
            priority: JobPriority::Normal,
            idempotency_key: Some(idempotency_key),
            correlation_id: Some(cohort_id.to_string()),
            causation_id: None,
            config: None,
        }).await?;

        Ok(())
    }

    // ============================================
    // JOB PROCESSING
    // ============================================

    /// Process a cohort notification job
    pub async fn process_notification_job(
        &self,
        payload: CohortNotificationPayload,
    ) -> JobResult {
        // 1. Get the cohort and check current state
        let cohort = match self.cohort_repo.get_cohort(payload.cohort_id).await {
            Ok(Some(c)) => c,
            Ok(None) => {
                return JobResult::success(serde_json::json!({
                    "skipped": true,
                    "reason": "cohort_not_found"
                }));
            }
            Err(e) => return JobResult::failure(format!("Failed to get cohort: {}", e)),
        };

        // 2. Check if notification is still relevant
        let notification_type = match CohortNotificationType::from_str(&payload.notification_type) {
            Some(t) => t,
            None => return JobResult::failure("Invalid notification type"),
        };

        if !self.is_notification_relevant(&cohort, notification_type) {
            // Mark as skipped - milestone already completed
            let _ = self.cohort_repo.skip_notification(
                payload.notification_id,
                "Milestone already completed",
            ).await;

            return JobResult::success(serde_json::json!({
                "skipped": true,
                "reason": "milestone_completed"
            }));
        }

        // 3. Check consent
        let has_consent = self.check_notification_consent(
            &payload.user_id,
            &payload.channel,
        ).await.unwrap_or(false);

        if !has_consent {
            let _ = self.cohort_repo.skip_notification(
                payload.notification_id,
                "No consent for channel",
            ).await;

            return JobResult::success(serde_json::json!({
                "skipped": true,
                "reason": "no_consent"
            }));
        }

        // 4. Send the notification
        match self.send_notification(&payload, &cohort).await {
            Ok(external_id) => {
                // Mark as sent
                let _ = self.cohort_repo.mark_notification_sent(
                    payload.notification_id,
                    external_id.as_deref(),
                ).await;

                JobResult::success(serde_json::json!({
                    "sent": true,
                    "external_id": external_id
                }))
            }
            Err(e) => {
                tracing::error!(
                    notification_id = %payload.notification_id,
                    error = %e,
                    "Failed to send notification"
                );
                JobResult::failure(format!("Failed to send: {}", e))
            }
        }
    }

    /// Process a milestone check job
    pub async fn process_milestone_check_job(
        &self,
        payload: MilestoneCheckPayload,
    ) -> JobResult {
        let cohort = match self.cohort_repo.get_cohort(payload.cohort_id).await {
            Ok(Some(c)) => c,
            Ok(None) => {
                return JobResult::success(serde_json::json!({
                    "skipped": true,
                    "reason": "cohort_not_found"
                }));
            }
            Err(e) => return JobResult::failure(format!("Failed to get cohort: {}", e)),
        };

        match payload.milestone.as_str() {
            "d1" => self.check_d1_milestone(&cohort).await,
            "d3" => self.check_d3_milestone(&cohort).await,
            "d7" => self.check_d7_milestone(&cohort).await,
            "d30" => self.check_d30_milestone(&cohort).await,
            _ => JobResult::failure("Unknown milestone"),
        }
    }

    // ============================================
    // MILESTONE CHECKS
    // ============================================

    async fn check_d1_milestone(&self, cohort: &ParticipantCohort) -> JobResult {
        if cohort.d1_completed {
            return JobResult::success(serde_json::json!({
                "milestone": "d1",
                "status": "already_completed"
            }));
        }

        // Check if they've had any activity (indicating installation)
        let activities = match self.cohort_repo.get_cohort_activities(cohort.id).await {
            Ok(a) => a,
            Err(e) => return JobResult::failure(format!("Failed to get activities: {}", e)),
        };

        if !activities.is_empty() {
            // Mark D1 complete
            if let Err(e) = self.cohort_repo.complete_d1(cohort.id).await {
                return JobResult::failure(format!("Failed to complete D1: {}", e));
            }

            JobResult::success(serde_json::json!({
                "milestone": "d1",
                "status": "completed",
                "activity_count": activities.len()
            }))
        } else {
            JobResult::success(serde_json::json!({
                "milestone": "d1",
                "status": "pending",
                "activity_count": 0
            }))
        }
    }

    async fn check_d3_milestone(&self, cohort: &ParticipantCohort) -> JobResult {
        if cohort.d3_completed {
            return JobResult::success(serde_json::json!({
                "milestone": "d3",
                "status": "already_completed"
            }));
        }

        // Count activities in first 3 days
        let today = Utc::now().date_naive();
        let d3_date = cohort.cohort_date + chrono::Duration::days(3);

        if today < d3_date {
            return JobResult::success(serde_json::json!({
                "milestone": "d3",
                "status": "too_early",
                "check_date": d3_date.to_string()
            }));
        }

        let activities = match self.cohort_repo.get_cohort_activities(cohort.id).await {
            Ok(a) => a,
            Err(e) => return JobResult::failure(format!("Failed to get activities: {}", e)),
        };

        let d3_activities: Vec<_> = activities.iter()
            .filter(|a| a.activity_date <= d3_date)
            .collect();

        if d3_activities.len() >= 2 {
            // Mark D3 complete
            if let Err(e) = self.cohort_repo.complete_d3(cohort.id, d3_activities.len() as i32).await {
                return JobResult::failure(format!("Failed to complete D3: {}", e));
            }

            JobResult::success(serde_json::json!({
                "milestone": "d3",
                "status": "completed",
                "activity_count": d3_activities.len()
            }))
        } else {
            JobResult::success(serde_json::json!({
                "milestone": "d3",
                "status": "pending",
                "activity_count": d3_activities.len(),
                "required": 2
            }))
        }
    }

    async fn check_d7_milestone(&self, cohort: &ParticipantCohort) -> JobResult {
        if cohort.d7_completed {
            return JobResult::success(serde_json::json!({
                "milestone": "d7",
                "status": "already_completed"
            }));
        }

        // Calculate D7 progress
        let progress = match self.cohort_repo.calculate_d7_progress(cohort.id).await {
            Ok(p) => p,
            Err(e) => return JobResult::failure(format!("Failed to calculate D7 progress: {}", e)),
        };

        // D7 requires 4 of 7 days active
        if progress.active_days >= 4 {
            // D7 complete - this would be handled by activity recording
            JobResult::success(serde_json::json!({
                "milestone": "d7",
                "status": "completed",
                "active_days": progress.active_days,
                "required": 4
            }))
        } else {
            JobResult::success(serde_json::json!({
                "milestone": "d7",
                "status": "pending",
                "active_days": progress.active_days,
                "required": 4,
                "days_remaining": progress.days_remaining
            }))
        }
    }

    async fn check_d30_milestone(&self, cohort: &ParticipantCohort) -> JobResult {
        if cohort.d30_completed {
            return JobResult::success(serde_json::json!({
                "milestone": "d30",
                "status": "already_completed"
            }));
        }

        // Calculate D30 progress
        let progress = match self.cohort_repo.calculate_d30_progress(cohort.id).await {
            Ok(p) => p,
            Err(e) => return JobResult::failure(format!("Failed to calculate D30 progress: {}", e)),
        };

        // D30 requires 20 of 30 days active
        if progress.active_days >= 20 {
            JobResult::success(serde_json::json!({
                "milestone": "d30",
                "status": "completed",
                "active_days": progress.active_days,
                "required": 20
            }))
        } else {
            let days_remaining = 30i32.saturating_sub(progress.days_elapsed);
            JobResult::success(serde_json::json!({
                "milestone": "d30",
                "status": "pending",
                "active_days": progress.active_days,
                "required": 20,
                "days_remaining": days_remaining
            }))
        }
    }

    // ============================================
    // HELPERS
    // ============================================

    /// Check if a notification is still relevant given current cohort state
    fn is_notification_relevant(
        &self,
        cohort: &ParticipantCohort,
        notification_type: CohortNotificationType,
    ) -> bool {
        match notification_type {
            CohortNotificationType::D1Reminder => !cohort.d1_completed,
            CohortNotificationType::D3Reminder => !cohort.d3_completed,
            CohortNotificationType::D7Warning => !cohort.d7_completed && cohort.d7_active_days < 3,
            CohortNotificationType::D7Success => cohort.d7_completed,
            CohortNotificationType::D30Warning => !cohort.d30_completed && cohort.d30_active_days < 15,
            CohortNotificationType::D30Success => cohort.d30_completed,
        }
    }

    /// Check if user has consented to notifications on the given channel
    async fn check_notification_consent(
        &self,
        user_id: &str,
        channel: &str,
    ) -> Result<bool, AppError> {
        // Map channel to consent type
        let consent_type = match channel {
            "email" => "marketing_email",
            "push" => "push_notifications",
            "sms" => "sms_notifications",
            _ => return Ok(false),
        };

        // Parse user_id as Uuid
        let user_uuid = uuid::Uuid::parse_str(user_id)
            .map_err(|_| AppError::BadRequest(format!("Invalid user_id: {}", user_id)))?;

        // Check consent status
        let status = self.consent_repo.get_user_consent_status(user_uuid).await?;

        Ok(status.consents.iter().any(|c| {
            c.consent_type == consent_type && c.consented
        }))
    }

    /// Send a notification (placeholder - integrate with actual notification service)
    async fn send_notification(
        &self,
        payload: &CohortNotificationPayload,
        cohort: &ParticipantCohort,
    ) -> Result<Option<String>, AppError> {
        let notification_type = CohortNotificationType::from_str(&payload.notification_type)
            .ok_or_else(|| AppError::ValidationError("Invalid notification type".to_string()))?;

        tracing::info!(
            user_id = %payload.user_id,
            notification_type = %payload.notification_type,
            channel = %payload.channel,
            cohort_id = %cohort.id,
            "Sending cohort notification"
        );

        // TODO: Integrate with actual email/push service
        // For now, just log and return a mock ID
        let external_id = format!("notif-{}", Uuid::new_v4());

        Ok(Some(external_id))
    }
}

// ============================================
// BATCH PROCESSING
// ============================================

/// Process pending notifications in batch
pub async fn process_pending_notifications(
    service: &CohortNotificationService,
    cohort_repo: &DynCohortRepository,
    batch_size: i32,
) -> Result<ProcessingStats, AppError> {
    let pending = cohort_repo.get_pending_notifications(batch_size).await?;

    let mut stats = ProcessingStats::default();

    for notification in pending {
        // Get the cohort
        let cohort = match cohort_repo.get_cohort(notification.cohort_id).await? {
            Some(c) => c,
            None => {
                stats.skipped += 1;
                continue;
            }
        };

        // Create job for processing
        match service.create_notification_job(&notification, &cohort).await {
            Ok(()) => stats.queued += 1,
            Err(e) => {
                tracing::error!(
                    notification_id = %notification.id,
                    error = %e,
                    "Failed to create notification job"
                );
                stats.errors += 1;
            }
        }
    }

    Ok(stats)
}

/// Check milestones for pending cohorts
pub async fn check_pending_milestones(
    service: &CohortNotificationService,
    cohort_repo: &DynCohortRepository,
    limit: i32,
) -> Result<MilestoneCheckStats, AppError> {
    let mut stats = MilestoneCheckStats::default();

    // Check D1 pending
    let d1_pending = cohort_repo.get_cohorts_pending_d1(limit).await?;
    for cohort in d1_pending {
        if service.create_milestone_check_job(cohort.id, "d1").await.is_ok() {
            stats.d1_checked += 1;
        }
    }

    // Check D7 pending
    let d7_pending = cohort_repo.get_cohorts_pending_d7(limit).await?;
    for cohort in d7_pending {
        if service.create_milestone_check_job(cohort.id, "d7").await.is_ok() {
            stats.d7_checked += 1;
        }
    }

    // Check D30 pending
    let d30_pending = cohort_repo.get_cohorts_pending_d30(limit).await?;
    for cohort in d30_pending {
        if service.create_milestone_check_job(cohort.id, "d30").await.is_ok() {
            stats.d30_checked += 1;
        }
    }

    Ok(stats)
}

/// Stats from processing notifications
#[derive(Debug, Default, Clone, Serialize)]
pub struct ProcessingStats {
    pub queued: i32,
    pub skipped: i32,
    pub errors: i32,
}

/// Stats from milestone checks
#[derive(Debug, Default, Clone, Serialize)]
pub struct MilestoneCheckStats {
    pub d1_checked: i32,
    pub d7_checked: i32,
    pub d30_checked: i32,
}

// ============================================
// JOB HANDLER
// ============================================

/// Create a job handler function for cohort notifications
pub fn create_cohort_job_handler(
    service: Arc<CohortNotificationService>,
) -> impl Fn(&crate::server::services::Job) -> std::pin::Pin<Box<dyn std::future::Future<Output = JobResult> + Send>> + Send + Sync {
    move |job| {
        let service = service.clone();
        let job_type = job.job_type.clone();
        let payload = job.payload.clone();

        Box::pin(async move {
            match job_type.as_str() {
                JOB_TYPE_COHORT_NOTIFICATION => {
                    let payload: CohortNotificationPayload = match serde_json::from_value(payload) {
                        Ok(p) => p,
                        Err(e) => return JobResult::failure(format!("Invalid payload: {}", e)),
                    };
                    service.process_notification_job(payload).await
                }
                JOB_TYPE_COHORT_MILESTONE_CHECK => {
                    let payload: MilestoneCheckPayload = match serde_json::from_value(payload) {
                        Ok(p) => p,
                        Err(e) => return JobResult::failure(format!("Invalid payload: {}", e)),
                    };
                    service.process_milestone_check_job(payload).await
                }
                _ => JobResult::failure(format!("Unknown job type: {}", job_type)),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_type_conversion() {
        assert_eq!(CohortNotificationType::D1Reminder.as_str(), "d1_reminder");
        assert_eq!(
            CohortNotificationType::from_str("d1_reminder"),
            Some(CohortNotificationType::D1Reminder)
        );
        assert_eq!(CohortNotificationType::from_str("invalid"), None);
    }

    #[test]
    fn test_notification_day_offsets() {
        assert_eq!(CohortNotificationType::D1Reminder.day_offset(), 1);
        assert_eq!(CohortNotificationType::D3Reminder.day_offset(), 3);
        assert_eq!(CohortNotificationType::D7Warning.day_offset(), 5);
        assert_eq!(CohortNotificationType::D7Success.day_offset(), 7);
        assert_eq!(CohortNotificationType::D30Warning.day_offset(), 25);
        assert_eq!(CohortNotificationType::D30Success.day_offset(), 30);
    }

    #[test]
    fn test_notification_templates() {
        assert_eq!(
            CohortNotificationType::D1Reminder.email_template(),
            "cohort_d1_reminder"
        );
        assert_eq!(
            CohortNotificationType::D7Success.push_template(),
            "cohort_d7_push"
        );
    }

    #[test]
    fn test_processing_stats_default() {
        let stats = ProcessingStats::default();
        assert_eq!(stats.queued, 0);
        assert_eq!(stats.skipped, 0);
        assert_eq!(stats.errors, 0);
    }

    #[test]
    fn test_milestone_check_stats_default() {
        let stats = MilestoneCheckStats::default();
        assert_eq!(stats.d1_checked, 0);
        assert_eq!(stats.d7_checked, 0);
        assert_eq!(stats.d30_checked, 0);
    }
}
