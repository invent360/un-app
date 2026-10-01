//! Reservation service
//!
//! Provides unified API for license reservations with:
//! - Eligibility checking
//! - Publication status validation
//! - Atomic reservation with locking
//! - Issuance state tracking
//! - R3-05: Capacity ceiling enforcement

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::sync::Arc;

use crate::server::db::ConnectionPool;
use crate::server::repositories::{
    ClaimRepository, ClaimResult, ReservationResult, RESERVATION_EXPIRY_SECS,
    EligibilityRepository, EligibilityContext, EligibilityResult, CheckType, DeviceType,
};
use crate::types::AppError;

/// R3-05: Maximum number of occupied licenses (reserved + issued + active + pending release)
/// Prevents system from over-allocating beyond capacity.
pub const MAX_OCCUPIED_LICENSES: i64 = 2500;

// ============================================
// SERVICE TYPES
// ============================================

/// Input for reservation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationRequest {
    /// Preferred split type (optional)
    pub split_type: Option<String>,
    /// Referral code (optional)
    pub referral_code: Option<String>,
    /// User ID (for tracking)
    pub user_id: Option<String>,
    /// Country code (ISO 3166-1 alpha-2)
    pub country_code: Option<String>,
    /// Device type
    pub device_type: Option<DeviceType>,
    /// App version
    pub app_version: Option<String>,
    /// OS version
    pub os_version: Option<String>,
    /// Task type (for eligibility)
    pub task_type: Option<String>,
    /// Whether user is verified
    pub is_verified: bool,
    /// Client IP address
    pub ip_address: Option<IpAddr>,
}

impl Default for ReservationRequest {
    fn default() -> Self {
        Self {
            split_type: None,
            referral_code: None,
            user_id: None,
            country_code: None,
            device_type: None,
            app_version: None,
            os_version: None,
            task_type: None,
            is_verified: false,
            ip_address: None,
        }
    }
}

/// Extended reservation result with eligibility info
///
/// R5-05: Does NOT include lease_code - credential revealed only at confirmation
/// when ownership is established.
#[derive(Debug, Clone, Serialize)]
pub struct ExtendedReservationResult {
    pub license_id: String,
    /// R5-05: lease_code intentionally omitted - revealed only at confirm
    pub session_token: String,
    pub expires_at: DateTime<Utc>,
    pub split_type: String,
    pub referral_validated: bool,
    /// Eligibility ruleset that was applied (if any)
    pub eligibility_ruleset: Option<String>,
    /// R5-05: Remaining capacity after this reservation
    pub capacity_remaining: Option<i64>,
}

/// Input for confirming a reservation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmRequest {
    pub license_id: String,
    pub session_token: String,
    /// F2: Owner ID (authenticated user) for verification against reservation
    pub owner_id: String,
    pub device_id: Option<String>,
    pub referral_code: Option<String>,
}

/// Reservation error with specific failure reason
#[derive(Debug, Clone, Serialize)]
pub struct ReservationError {
    pub code: String,
    pub message: String,
    pub eligibility_failure: Option<String>,
}

impl ReservationError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            eligibility_failure: None,
        }
    }

    pub fn eligibility(failure: &crate::server::repositories::EligibilityFailure) -> Self {
        Self {
            code: failure.code().to_string(),
            message: failure.message().to_string(),
            eligibility_failure: Some(failure.code().to_string()),
        }
    }

    pub fn no_licenses() -> Self {
        Self::new("no_licenses_available", "No licenses are currently available")
    }

    pub fn invalid_token() -> Self {
        Self::new("invalid_token", "Invalid or expired reservation token")
    }

    pub fn expired() -> Self {
        Self::new("reservation_expired", "Reservation has expired. Please reserve again.")
    }
}

// ============================================
// SERVICE TRAIT
// ============================================

pub type DynReservationService = Arc<dyn ReservationService + Send + Sync>;

#[async_trait]
pub trait ReservationService: Send + Sync {
    /// Reserve a license with eligibility checking
    ///
    /// 1. Checks eligibility based on request context
    /// 2. Finds an available published license
    /// 3. Atomically reserves it with session binding
    async fn reserve(
        &self,
        request: ReservationRequest,
    ) -> Result<ExtendedReservationResult, ReservationError>;

    /// Confirm a reservation and claim the license
    ///
    /// 1. Validates session token
    /// 2. Checks reservation hasn't expired
    /// 3. Atomically claims the license
    async fn confirm(
        &self,
        request: ConfirmRequest,
    ) -> Result<ClaimResult, ReservationError>;

    /// Release an active reservation
    async fn release(
        &self,
        license_id: &str,
        session_token: &str,
    ) -> Result<(), AppError>;

    /// Check if a license can be reserved by given context
    async fn can_reserve(
        &self,
        license_id: &str,
        request: &ReservationRequest,
    ) -> Result<EligibilityResult, AppError>;

    /// Get active reservation for a user
    async fn get_user_reservation(
        &self,
        user_id: &str,
    ) -> Result<Option<ReservationResult>, AppError>;

    /// Cleanup expired reservations
    async fn cleanup_expired(&self) -> Result<u64, AppError>;
}

// ============================================
// SERVICE IMPLEMENTATION
// ============================================

pub struct ReservationServiceImpl {
    pool: ConnectionPool,
    claim_repo: Arc<dyn ClaimRepository + Send + Sync>,
    eligibility_repo: Arc<dyn EligibilityRepository + Send + Sync>,
}

impl ReservationServiceImpl {
    pub fn new(
        pool: ConnectionPool,
        claim_repo: Arc<dyn ClaimRepository + Send + Sync>,
        eligibility_repo: Arc<dyn EligibilityRepository + Send + Sync>,
    ) -> Self {
        Self {
            pool,
            claim_repo,
            eligibility_repo,
        }
    }

    /// Convert request to eligibility context
    fn to_eligibility_context(request: &ReservationRequest) -> EligibilityContext {
        EligibilityContext {
            country_code: request.country_code.clone(),
            device_type: request.device_type,
            task_type: request.task_type.clone(),
            app_version: request.app_version.clone(),
            os_version: request.os_version.clone(),
            is_verified: request.is_verified,
            ip_address: request.ip_address,
        }
    }

    // R5-05: Capacity check moved inside atomic_reserve
    // The old check_capacity() method created a TOCTOU race condition where
    // two users could both pass the capacity check, then both try to reserve.
    // Now the capacity check is performed atomically inside the transaction.

    /// Find license ID and check eligibility atomically
    async fn find_eligible_license(
        &self,
        request: &ReservationRequest,
    ) -> Result<String, ReservationError> {
        let now = Utc::now();
        let context = Self::to_eligibility_context(request);

        // Find first available license matching criteria
        let license_row: Option<LicenseSearchRow> = if let Some(ref st) = request.split_type {
            sqlx::query_as::<_, LicenseSearchRow>(r#"
                SELECT id, eligibility_ruleset_id
                FROM licenses
                WHERE claimed = false
                  AND (reserved_until IS NULL OR reserved_until < $1)
                  AND valid_from <= $1
                  AND valid_to > $1
                  AND split_type = $2::split_type
                  AND (publication_status = 'published' OR publication_status IS NULL)
                  AND (is_quarantined = false OR is_quarantined IS NULL)
                ORDER BY created_at ASC
                LIMIT 1
            "#)
            .bind(now)
            .bind(st)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ReservationError::new("database_error", &e.to_string()))?
        } else {
            sqlx::query_as::<_, LicenseSearchRow>(r#"
                SELECT id, eligibility_ruleset_id
                FROM licenses
                WHERE claimed = false
                  AND (reserved_until IS NULL OR reserved_until < $1)
                  AND valid_from <= $1
                  AND valid_to > $1
                  AND (publication_status = 'published' OR publication_status IS NULL)
                  AND (is_quarantined = false OR is_quarantined IS NULL)
                ORDER BY created_at ASC
                LIMIT 1
            "#)
            .bind(now)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ReservationError::new("database_error", &e.to_string()))?
        };

        let license = license_row.ok_or_else(ReservationError::no_licenses)?;

        // Check eligibility if license has rules
        if license.eligibility_ruleset_id.is_some() {
            let result = self.eligibility_repo
                .check_eligibility(&license.id, &context, CheckType::Reservation, request.user_id.as_deref())
                .await
                .map_err(|e| ReservationError::new("eligibility_check_failed", &e.to_string()))?;

            if !result.is_eligible {
                if let Some(ref failure) = result.failure {
                    return Err(ReservationError::eligibility(failure));
                }
                return Err(ReservationError::new("ineligible", "License eligibility check failed"));
            }
        }

        Ok(license.id)
    }
}

#[async_trait]
impl ReservationService for ReservationServiceImpl {
    async fn reserve(
        &self,
        request: ReservationRequest,
    ) -> Result<ExtendedReservationResult, ReservationError> {
        // B3 FIX: Require authenticated user - no more anonymous reservations
        let user_id = request.user_id.as_deref()
            .ok_or_else(|| ReservationError::new("authentication_required", "User ID is required for reservation"))?;

        // R5-05: Use atomic_reserve with capacity ceiling INSIDE the transaction
        // This prevents TOCTOU race conditions where two users could both pass a
        // non-locked capacity check and then both attempt to reserve.
        let result = self.claim_repo
            .atomic_reserve(
                user_id,
                request.split_type.as_deref(),
                request.referral_code.as_deref(),
                MAX_OCCUPIED_LICENSES,
            )
            .await
            .map_err(|e| {
                match e {
                    AppError::NotFound(_) => ReservationError::no_licenses(),
                    AppError::ValidationError(msg) if msg.contains("capacity") => {
                        ReservationError::new("capacity_exceeded", &msg)
                    }
                    AppError::Unauthorized(_) => {
                        ReservationError::new("authentication_required", "Authentication required for reservation")
                    }
                    _ => ReservationError::new("reservation_failed", &e.to_string()),
                }
            })?;

        // Check eligibility for the reserved license
        let context = Self::to_eligibility_context(&request);
        let eligibility = self.eligibility_repo
            .check_eligibility(&result.license_id, &context, CheckType::Reservation, request.user_id.as_deref())
            .await
            .map_err(|e| ReservationError::new("eligibility_check_failed", &e.to_string()))?;

        if !eligibility.is_eligible {
            // Release the reservation since eligibility failed
            let _ = self.claim_repo.release_reservation(&result.license_id, &result.session_token).await;

            if let Some(ref failure) = eligibility.failure {
                return Err(ReservationError::eligibility(failure));
            }
            return Err(ReservationError::new("ineligible", "License eligibility check failed"));
        }

        // Store context in reservation (for audit)
        let _ = sqlx::query(r#"
            UPDATE license_reservations
            SET user_id = $3, country_code = $4, device_type = $5, app_version = $6,
                os_version = $7, ip_address = $8, referral_code = $9
            WHERE license_id = $1 AND session_token = $2
        "#)
        .bind(&result.license_id)
        .bind(&result.session_token)
        .bind(&request.user_id)
        .bind(request.country_code.as_ref().map(|c| &c[..2.min(c.len())]))
        .bind(request.device_type.map(|d| d.as_str()))
        .bind(&request.app_version)
        .bind(&request.os_version)
        .bind(request.ip_address.map(|ip| ip.to_string()))
        .bind(&request.referral_code)
        .execute(&self.pool)
        .await;

        // Get ruleset name if applied
        let ruleset_name = if let Some(ruleset_id) = eligibility.ruleset_id {
            self.eligibility_repo
                .get_ruleset(ruleset_id)
                .await
                .ok()
                .flatten()
                .map(|r| r.name)
        } else {
            None
        };

        // R5-05: Do NOT include lease_code - credential revealed only at confirm
        Ok(ExtendedReservationResult {
            license_id: result.license_id,
            session_token: result.session_token,
            expires_at: result.expires_at,
            split_type: result.split_type,
            referral_validated: result.referral_validated,
            eligibility_ruleset: ruleset_name,
            capacity_remaining: result.capacity_remaining,
        })
    }

    async fn confirm(
        &self,
        request: ConfirmRequest,
    ) -> Result<ClaimResult, ReservationError> {
        // Get referral ID if code was provided
        let referral_id = if let Some(ref code) = request.referral_code {
            let result: Option<(i32,)> = sqlx::query_as(
                "SELECT id FROM referrals WHERE referral_code = $1 AND status = 'active'"
            )
            .bind(code.to_uppercase())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ReservationError::new("referral_lookup_failed", &e.to_string()))?;
            result.map(|(id,)| id)
        } else {
            // Check if referral was stored at reservation time
            let result: Option<(Option<String>,)> = sqlx::query_as(
                "SELECT referral_code FROM license_reservations WHERE license_id = $1 AND session_token = $2"
            )
            .bind(&request.license_id)
            .bind(&request.session_token)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ReservationError::new("reservation_lookup_failed", &e.to_string()))?;

            if let Some((Some(code),)) = result {
                let ref_result: Option<(i32,)> = sqlx::query_as(
                    "SELECT id FROM referrals WHERE referral_code = $1 AND status = 'active'"
                )
                .bind(code.to_uppercase())
                .fetch_optional(&self.pool)
                .await
                .ok()
                .flatten();
                ref_result.map(|(id,)| id)
            } else {
                None
            }
        };

        // F2: Use atomic_confirm with owner_id for owner verification
        let result = self.claim_repo
            .atomic_confirm(
                &request.license_id,
                &request.session_token,
                &request.owner_id,
                request.device_id.as_deref(),
                referral_id,
            )
            .await
            .map_err(|e| {
                match e {
                    AppError::Unauthorized(_) => ReservationError::invalid_token(),
                    AppError::ValidationError(_) => ReservationError::expired(),
                    AppError::NotFound(_) => ReservationError::new("not_found", "License not found"),
                    _ => ReservationError::new("confirm_failed", &e.to_string()),
                }
            })?;

        Ok(result)
    }

    async fn release(
        &self,
        license_id: &str,
        session_token: &str,
    ) -> Result<(), AppError> {
        self.claim_repo.release_reservation(license_id, session_token).await
    }

    async fn can_reserve(
        &self,
        license_id: &str,
        request: &ReservationRequest,
    ) -> Result<EligibilityResult, AppError> {
        let context = Self::to_eligibility_context(request);
        self.eligibility_repo
            .check_eligibility(license_id, &context, CheckType::Reservation, request.user_id.as_deref())
            .await
    }

    async fn get_user_reservation(
        &self,
        user_id: &str,
    ) -> Result<Option<ReservationResult>, AppError> {
        let now = Utc::now();

        // R5-05: Do NOT select lease_code - credential revealed only at confirm
        let result: Option<UserReservationRow> = sqlx::query_as::<_, UserReservationRow>(r#"
            SELECT r.license_id, r.session_token, r.expires_at, l.split_type::text as split_type
            FROM license_reservations r
            JOIN licenses l ON r.license_id = l.id
            WHERE r.user_id = $1
              AND r.status = 'active'
              AND r.expires_at > $2
            ORDER BY r.reserved_at DESC
            LIMIT 1
        "#)
        .bind(user_id)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.map(|r| ReservationResult {
            license_id: r.license_id,
            session_token: r.session_token,
            expires_at: r.expires_at,
            split_type: r.split_type,
            referral_validated: false,
            capacity_remaining: None, // Not tracked for existing reservations
        }))
    }

    async fn cleanup_expired(&self) -> Result<u64, AppError> {
        self.claim_repo.cleanup_expired_reservations().await
    }
}

// ============================================
// HELPER TYPES
// ============================================

#[derive(Debug, sqlx::FromRow)]
struct LicenseSearchRow {
    id: String,
    eligibility_ruleset_id: Option<i32>,
}

/// R5-05: Does NOT include lease_code - credentials revealed only at confirm
#[derive(Debug, sqlx::FromRow)]
struct UserReservationRow {
    license_id: String,
    session_token: String,
    expires_at: DateTime<Utc>,
    split_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reservation_error() {
        let err = ReservationError::no_licenses();
        assert_eq!(err.code, "no_licenses_available");
    }

    #[test]
    fn test_request_defaults() {
        let req = ReservationRequest::default();
        assert!(req.split_type.is_none());
        assert!(!req.is_verified);
    }

    #[test]
    fn test_capacity_ceiling_constant() {
        // R3-05: Verify capacity ceiling is configured correctly
        assert_eq!(MAX_OCCUPIED_LICENSES, 2500);
    }

    #[test]
    fn test_capacity_exceeded_error() {
        let err = ReservationError::new("capacity_exceeded", "License capacity has been reached.");
        assert_eq!(err.code, "capacity_exceeded");
    }
}
