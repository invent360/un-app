//! License lifecycle service
//!
//! Provides:
//! - License cancellation
//! - Expiry handling
//! - Release workflow
//! - Exposure tracking

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::sync::Arc;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// TYPES
// ============================================

/// Lifecycle event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleEvent {
    Created,
    Published,
    Reserved,
    Issued,
    Claimed,
    Released,
    Cancelled,
    Expired,
    Withdrawn,
    Reactivated,
}

impl LifecycleEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Published => "published",
            Self::Reserved => "reserved",
            Self::Issued => "issued",
            Self::Claimed => "claimed",
            Self::Released => "released",
            Self::Cancelled => "cancelled",
            Self::Expired => "expired",
            Self::Withdrawn => "withdrawn",
            Self::Reactivated => "reactivated",
        }
    }
}

/// Actor types for lifecycle events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    User,
    Admin,
    System,
    Scheduler,
}

impl ActorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Admin => "admin",
            Self::System => "system",
            Self::Scheduler => "scheduler",
        }
    }
}

/// Lifecycle log entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LifecycleLogEntry {
    pub id: i64,
    pub license_id: String,
    pub event: String,
    pub from_state: Option<String>,
    pub to_state: Option<String>,
    pub actor_type: String,
    pub actor_id: Option<String>,
    pub reason: Option<String>,
    pub exposure_snapshot: Option<JsonValue>,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}

/// Exposure metrics for a license
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposureMetrics {
    pub license_id: String,
    pub published_secs: i64,
    pub reserved_secs: i64,
    pub claimed_secs: i64,
    pub total_secs: i64,
    pub time_to_claim_secs: Option<i64>,
    pub remaining_validity_secs: i64,
}

/// Input for cancelling license
#[derive(Debug, Clone)]
pub struct CancelLicenseInput {
    pub license_id: String,
    pub canceller_id: String,
    pub reason: String,
}

/// Input for releasing license
#[derive(Debug, Clone)]
pub struct ReleaseLicenseInput {
    pub license_id: String,
    pub releaser_id: String,
    pub reason: Option<String>,
}

/// Input for reactivating license
#[derive(Debug, Clone)]
pub struct ReactivateLicenseInput {
    pub license_id: String,
    pub reactivator_id: String,
    pub new_valid_to: DateTime<Utc>,
    pub reason: Option<String>,
}

/// Expiry notification types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpiryNotificationType {
    ThirtyDay,
    SevenDay,
    OneDay,
    Expired,
}

impl ExpiryNotificationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ThirtyDay => "30_day",
            Self::SevenDay => "7_day",
            Self::OneDay => "1_day",
            Self::Expired => "expired",
        }
    }

    pub fn days(&self) -> i32 {
        match self {
            Self::ThirtyDay => 30,
            Self::SevenDay => 7,
            Self::OneDay => 1,
            Self::Expired => 0,
        }
    }
}

/// License pending expiry
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct LicensePendingExpiry {
    pub license_id: String,
    pub owner_id: Option<String>,
    pub valid_to: DateTime<Utc>,
    pub days_until_expiry: i32,
}

// ============================================
// R5-05: RELEASE WORKFLOW TYPES
// ============================================

/// R5-05: Release status for upstream workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseStatus {
    None,
    Requested,
    PendingUpstream,
    Confirmed,
    Cooldown,
    Available,
}

impl ReleaseStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Requested => "requested",
            Self::PendingUpstream => "pending_upstream",
            Self::Confirmed => "confirmed",
            Self::Cooldown => "cooldown",
            Self::Available => "available",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "requested" => Some(Self::Requested),
            "pending_upstream" => Some(Self::PendingUpstream),
            "confirmed" => Some(Self::Confirmed),
            "cooldown" => Some(Self::Cooldown),
            "available" => Some(Self::Available),
            _ => None,
        }
    }
}

/// R5-05: Input for requesting release
#[derive(Debug, Clone)]
pub struct RequestReleaseInput {
    pub license_id: String,
    pub user_id: String,
    pub reason: Option<String>,
}

/// R5-05: Result of requesting release
#[derive(Debug, Clone, Serialize)]
pub struct RequestReleaseResult {
    pub success: bool,
    pub release_status: ReleaseStatus,
    pub cooldown_until: Option<DateTime<Utc>>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

/// R5-05: Input for confirming upstream release
#[derive(Debug, Clone)]
pub struct ConfirmUpstreamReleaseInput {
    pub license_id: String,
    pub admin_id: String,
    pub upstream_ref: Option<String>,
}

// ============================================
// SERVICE TRAIT
// ============================================

pub type DynLifecycleService = Arc<dyn LifecycleService + Send + Sync>;

#[async_trait]
pub trait LifecycleService: Send + Sync {
    // Cancellation
    async fn cancel(&self, input: CancelLicenseInput) -> Result<(), AppError>;
    async fn is_cancellable(&self, license_id: &str) -> Result<bool, AppError>;

    // Release
    async fn release(&self, input: ReleaseLicenseInput) -> Result<(), AppError>;
    async fn can_release(&self, license_id: &str, user_id: &str) -> Result<bool, AppError>;

    // Reactivation
    async fn reactivate(&self, input: ReactivateLicenseInput) -> Result<(), AppError>;
    async fn can_reactivate(&self, license_id: &str) -> Result<bool, AppError>;

    // Expiry
    async fn process_expired(&self) -> Result<i32, AppError>;
    async fn get_pending_expiry(&self, days: i32, limit: i32) -> Result<Vec<LicensePendingExpiry>, AppError>;
    async fn record_expiry_notification(&self, license_id: &str, notification_type: ExpiryNotificationType) -> Result<(), AppError>;

    // Exposure
    async fn get_exposure(&self, license_id: &str) -> Result<ExposureMetrics, AppError>;
    async fn update_exposure(&self, license_id: &str) -> Result<(), AppError>;

    // Lifecycle history
    async fn get_lifecycle_history(&self, license_id: &str) -> Result<Vec<LifecycleLogEntry>, AppError>;
    async fn log_event(
        &self,
        license_id: &str,
        event: LifecycleEvent,
        from_state: Option<&str>,
        to_state: &str,
        actor_type: ActorType,
        actor_id: Option<&str>,
        reason: Option<&str>,
    ) -> Result<i64, AppError>;

    // R5-05: Upstream release workflow
    /// Request release with upstream tracking
    async fn request_release(&self, input: RequestReleaseInput) -> Result<RequestReleaseResult, AppError>;

    /// Admin confirms upstream release
    async fn confirm_upstream_release(&self, input: ConfirmUpstreamReleaseInput) -> Result<bool, AppError>;

    /// Complete release after cooldown (called by scheduler)
    async fn complete_pending_releases(&self) -> Result<i32, AppError>;

    /// Get release status for a license
    async fn get_release_status(&self, license_id: &str) -> Result<ReleaseStatus, AppError>;
}

// ============================================
// SERVICE IMPLEMENTATION
// ============================================

pub struct LifecycleServiceImpl {
    pool: ConnectionPool,
}

impl LifecycleServiceImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LifecycleService for LifecycleServiceImpl {
    async fn cancel(&self, input: CancelLicenseInput) -> Result<(), AppError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT cancel_license($1, $2, $3)"
        )
        .bind(&input.license_id)
        .bind(&input.canceller_id)
        .bind(&input.reason)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !result.0 {
            return Err(AppError::ValidationError("License is already cancelled".into()));
        }

        tracing::info!(
            license_id = %input.license_id,
            canceller = %input.canceller_id,
            reason = %input.reason,
            "License cancelled"
        );

        Ok(())
    }

    async fn is_cancellable(&self, license_id: &str) -> Result<bool, AppError> {
        let result: Option<(String,)> = sqlx::query_as(
            "SELECT issuance_state::text FROM licenses WHERE id = $1"
        )
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some((state,)) => Ok(state != "cancelled" && state != "terminated"),
            None => Err(AppError::NotFound("License not found".into())),
        }
    }

    async fn release(&self, input: ReleaseLicenseInput) -> Result<(), AppError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT release_license($1, $2, $3)"
        )
        .bind(&input.license_id)
        .bind(&input.releaser_id)
        .bind(input.reason.as_deref().unwrap_or("User released"))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !result.0 {
            return Err(AppError::ValidationError("Cannot release this license".into()));
        }

        tracing::info!(
            license_id = %input.license_id,
            releaser = %input.releaser_id,
            "License released"
        );

        Ok(())
    }

    async fn can_release(&self, license_id: &str, user_id: &str) -> Result<bool, AppError> {
        let result: Option<(String, Option<String>)> = sqlx::query_as(
            "SELECT issuance_state::text, issued_to FROM licenses WHERE id = $1"
        )
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some((state, owner)) => {
                let in_releasable_state = state == "issued" || state == "claimed";
                let is_owner = owner.as_deref() == Some(user_id);
                Ok(in_releasable_state && is_owner)
            }
            None => Ok(false),
        }
    }

    async fn reactivate(&self, input: ReactivateLicenseInput) -> Result<(), AppError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT reactivate_license($1, $2, $3, $4)"
        )
        .bind(&input.license_id)
        .bind(&input.reactivator_id)
        .bind(input.new_valid_to)
        .bind(input.reason.as_deref().unwrap_or("License reactivated"))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !result.0 {
            return Err(AppError::ValidationError("Cannot reactivate this license".into()));
        }

        tracing::info!(
            license_id = %input.license_id,
            reactivator = %input.reactivator_id,
            new_valid_to = %input.new_valid_to,
            "License reactivated"
        );

        Ok(())
    }

    async fn can_reactivate(&self, license_id: &str) -> Result<bool, AppError> {
        let result: Option<(String,)> = sqlx::query_as(
            "SELECT issuance_state::text FROM licenses WHERE id = $1"
        )
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some((state,)) => Ok(state == "expired" || state == "cancelled"),
            None => Ok(false),
        }
    }

    async fn process_expired(&self) -> Result<i32, AppError> {
        let result: (i32,) = sqlx::query_as(
            "SELECT process_expired_licenses()"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.0 > 0 {
            tracing::info!(count = result.0, "Processed expired licenses");
        }

        Ok(result.0)
    }

    async fn get_pending_expiry(&self, days: i32, limit: i32) -> Result<Vec<LicensePendingExpiry>, AppError> {
        let results = sqlx::query_as::<_, LicensePendingExpiry>(
            r#"
            SELECT
                id as license_id,
                issued_to as owner_id,
                valid_to,
                EXTRACT(DAY FROM (valid_to - NOW()))::INT as days_until_expiry
            FROM licenses
            WHERE issuance_state IN ('claimed', 'issued')
              AND valid_to > NOW()
              AND valid_to <= NOW() + ($1 || ' days')::INTERVAL
              AND (expiry_notification_sent = FALSE OR expiry_notification_sent IS NULL)
            ORDER BY valid_to ASC
            LIMIT $2
            "#,
        )
        .bind(days)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn record_expiry_notification(
        &self,
        license_id: &str,
        notification_type: ExpiryNotificationType,
    ) -> Result<(), AppError> {
        // Get owner
        let owner: Option<(Option<String>,)> = sqlx::query_as(
            "SELECT issued_to FROM licenses WHERE id = $1"
        )
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let owner_id = owner.and_then(|(o,)| o);

        // Record notification
        sqlx::query(
            r#"
            INSERT INTO license_expiry_notifications (
                license_id, owner_id, notification_type, days_until_expiry, delivery_status
            )
            VALUES ($1, $2, $3, $4, 'sent')
            "#,
        )
        .bind(license_id)
        .bind(&owner_id)
        .bind(notification_type.as_str())
        .bind(notification_type.days())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Mark as notified
        sqlx::query(
            "UPDATE licenses SET expiry_notification_sent = TRUE, expiry_notification_at = NOW() WHERE id = $1"
        )
        .bind(license_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_exposure(&self, license_id: &str) -> Result<ExposureMetrics, AppError> {
        // First update exposure to get accurate numbers
        self.update_exposure(license_id).await?;

        let row: Option<(String, i64, i64, i64, Option<i64>, i64)> = sqlx::query_as(
            r#"
            SELECT
                license_id,
                total_published_secs,
                total_claimed_secs,
                COALESCE(total_published_secs, 0) + COALESCE(total_claimed_secs, 0) as total,
                time_to_claim_secs,
                remaining_validity_secs
            FROM license_exposure_summary
            WHERE license_id = $1
            "#,
        )
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match row {
            Some((id, pub_secs, claim_secs, total, ttc, remaining)) => {
                Ok(ExposureMetrics {
                    license_id: id,
                    published_secs: pub_secs,
                    reserved_secs: 0, // From view
                    claimed_secs: claim_secs,
                    total_secs: total,
                    time_to_claim_secs: ttc,
                    remaining_validity_secs: remaining,
                })
            }
            None => Err(AppError::NotFound("License not found".into())),
        }
    }

    async fn update_exposure(&self, license_id: &str) -> Result<(), AppError> {
        sqlx::query("SELECT update_license_exposure($1)")
            .bind(license_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_lifecycle_history(&self, license_id: &str) -> Result<Vec<LifecycleLogEntry>, AppError> {
        let results = sqlx::query_as::<_, LifecycleLogEntry>(
            r#"
            SELECT id, license_id, event, from_state, to_state,
                   actor_type, actor_id, reason, exposure_snapshot, metadata, created_at
            FROM license_lifecycle_log
            WHERE license_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(license_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn log_event(
        &self,
        license_id: &str,
        event: LifecycleEvent,
        from_state: Option<&str>,
        to_state: &str,
        actor_type: ActorType,
        actor_id: Option<&str>,
        reason: Option<&str>,
    ) -> Result<i64, AppError> {
        let result: (i64,) = sqlx::query_as(
            "SELECT log_lifecycle_event($1, $2::lifecycle_event, $3, $4, $5, $6, $7)"
        )
        .bind(license_id)
        .bind(event.as_str())
        .bind(from_state)
        .bind(to_state)
        .bind(actor_type.as_str())
        .bind(actor_id)
        .bind(reason)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.0)
    }

    // R5-05: Upstream release workflow implementation

    async fn request_release(&self, input: RequestReleaseInput) -> Result<RequestReleaseResult, AppError> {
        // Call the database function for atomic release request
        let row: Option<(bool, String, Option<DateTime<Utc>>, Option<String>, Option<String>)> = sqlx::query_as(
            r#"
            SELECT success, release_status::text, cooldown_until, error_code, error_message
            FROM request_license_release($1, $2, $3)
            "#,
        )
        .bind(&input.license_id)
        .bind(&input.user_id)
        .bind(input.reason.as_deref())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match row {
            Some((success, status_str, cooldown, err_code, err_msg)) => {
                let status = ReleaseStatus::from_str(&status_str).unwrap_or(ReleaseStatus::None);

                if success {
                    tracing::info!(
                        license_id = %input.license_id,
                        user_id = %input.user_id,
                        status = %status_str,
                        "Release requested"
                    );
                }

                Ok(RequestReleaseResult {
                    success,
                    release_status: status,
                    cooldown_until: cooldown,
                    error_code: err_code,
                    error_message: err_msg,
                })
            }
            None => Err(AppError::DatabaseError("Release request failed".into())),
        }
    }

    async fn confirm_upstream_release(&self, input: ConfirmUpstreamReleaseInput) -> Result<bool, AppError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT confirm_upstream_release($1, $2, $3)"
        )
        .bind(&input.license_id)
        .bind(&input.admin_id)
        .bind(input.upstream_ref.as_deref())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.0 {
            tracing::info!(
                license_id = %input.license_id,
                admin_id = %input.admin_id,
                upstream_ref = ?input.upstream_ref,
                "Upstream release confirmed"
            );
        }

        Ok(result.0)
    }

    async fn complete_pending_releases(&self) -> Result<i32, AppError> {
        // Find all licenses in cooldown that have passed their cooldown_until
        let now = chrono::Utc::now();
        let licenses: Vec<(String,)> = sqlx::query_as(
            r#"
            SELECT id FROM licenses
            WHERE release_status = 'cooldown'
              AND cooldown_until <= $1
            "#,
        )
        .bind(now)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut completed = 0;
        for (license_id,) in licenses {
            let result: (bool,) = sqlx::query_as(
                "SELECT complete_license_release($1)"
            )
            .bind(&license_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            if result.0 {
                completed += 1;
                tracing::info!(license_id = %license_id, "Release completed after cooldown");
            }
        }

        if completed > 0 {
            tracing::info!(count = completed, "Completed pending releases");
        }

        Ok(completed)
    }

    async fn get_release_status(&self, license_id: &str) -> Result<ReleaseStatus, AppError> {
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT COALESCE(release_status::text, 'none') FROM licenses WHERE id = $1"
        )
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match row {
            Some((status_str,)) => {
                Ok(ReleaseStatus::from_str(&status_str).unwrap_or(ReleaseStatus::None))
            }
            None => Err(AppError::NotFound("License not found".into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle_event() {
        assert_eq!(LifecycleEvent::Cancelled.as_str(), "cancelled");
        assert_eq!(LifecycleEvent::Released.as_str(), "released");
    }

    #[test]
    fn test_expiry_notification_type() {
        assert_eq!(ExpiryNotificationType::ThirtyDay.days(), 30);
        assert_eq!(ExpiryNotificationType::OneDay.days(), 1);
    }
}
