//! License claiming service for end users
//!
//! Implements atomic reservation and claim operations with:
//! - Session-bound reservations with unguessable tokens
//! - Immutable referral attribution
//! - Owner verification for confirmations

use crate::server::repositories::{ClaimRepository, ClaimResult, ReservationResult};
use crate::types::{
    AppError, ClaimPageData, ClaimRequest, ClaimResponse, License, LicenseDto, LicenseSummary,
    SplitType,
};
use std::sync::Arc;
use uno_api::traits::LicenseRepository;

/// Extended claim response with session token for two-phase flow
///
/// R5-05: Does NOT include lease_code - credential is revealed only at confirmation
/// when ownership is established. This prevents credential exposure before the
/// user has committed to claiming.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReservationResponse {
    pub success: bool,
    pub message: Option<String>,
    pub license_id: Option<String>,
    /// R5-05: lease_code intentionally omitted - revealed only at confirm
    pub session_token: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub split_type: Option<SplitType>,
    pub referral_validated: bool,
    /// R5-05: Remaining capacity after this reservation
    pub capacity_remaining: Option<i64>,
}

impl ReservationResponse {
    pub fn success(result: ReservationResult) -> Self {
        Self {
            success: true,
            message: None,
            license_id: Some(result.license_id.to_string()),
            session_token: Some(result.session_token),
            expires_at: Some(result.expires_at),
            split_type: SplitType::from_str(&result.split_type),
            referral_validated: result.referral_validated,
            capacity_remaining: result.capacity_remaining,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: Some(message.to_string()),
            license_id: None,
            session_token: None,
            expires_at: None,
            split_type: None,
            referral_validated: false,
            capacity_remaining: None,
        }
    }
}

/// License service for claim operations
#[derive(Clone)]
pub struct LicenseService {
    license_repo: Arc<dyn LicenseRepository + Send + Sync>,
    claim_repo: Option<Arc<dyn ClaimRepository>>,
}

impl LicenseService {
    /// Create a new license service.
    pub fn new(license_repo: Arc<dyn LicenseRepository + Send + Sync>) -> Self {
        Self {
            license_repo,
            claim_repo: None,
        }
    }

    /// Create a new license service with claim repository for atomic operations.
    pub fn with_claim_repo(
        license_repo: Arc<dyn LicenseRepository + Send + Sync>,
        claim_repo: Arc<dyn ClaimRepository>,
    ) -> Self {
        Self {
            license_repo,
            claim_repo: Some(claim_repo),
        }
    }

    // Fail closed until Phase 4 replaces every legacy claim path with verified,
    // owner-bound issuance.
    //
    // F9: Changed from environment variable to feature flag for better security.
    // To run tests with issuance enabled: cargo test --features ssr,test-issuance
    // In production (without feature flag), this ALWAYS fails closed.
    fn ensure_issuance_ready() -> Result<(), AppError> {
        #[cfg(feature = "test-issuance")]
        {
            // Feature flag enabled - allow issuance for testing
            return Ok(());
        }

        // Production (no feature flag): fail closed
        #[allow(unreachable_code)]
        Err(AppError::LicenseUnavailable(
            "Licence issuance is paused pending secure allocation".into(),
        ))
    }

    /// Atomically reserve a license with session binding (Phase 1)
    ///
    /// Returns a session token that must be used to confirm the claim.
    /// The reservation expires after 2 minutes.
    pub async fn atomic_reserve(
        &self,
        split_type: Option<SplitType>,
        referral_code: Option<String>,
    ) -> Result<ReservationResponse, AppError> {
        Self::ensure_issuance_ready()?;
        let claim_repo = self
            .claim_repo
            .as_ref()
            .ok_or_else(|| AppError::ConfigError("Claim repository not configured".into()))?;

        let split_str = split_type.map(|st| convert_to_db_split(st));
        let result = claim_repo
            .atomic_reserve(split_str.as_deref(), referral_code.as_deref())
            .await?;

        Ok(ReservationResponse::success(result))
    }

    /// Atomically confirm a reservation and claim the license (Phase 2)
    ///
    /// Requires the session token from the reservation.
    /// F2: Now requires owner_id - the authenticated user who is claiming.
    /// This is verified against the user_id stored during reservation.
    /// Referral attribution is immutable - once set, cannot be changed.
    pub async fn atomic_confirm(
        &self,
        license_id: &str,
        session_token: &str,
        owner_id: &str,
        device_id: Option<String>,
        referral_id: Option<i32>,
    ) -> Result<ClaimResponse, AppError> {
        Self::ensure_issuance_ready()?;
        let claim_repo = self
            .claim_repo
            .as_ref()
            .ok_or_else(|| AppError::ConfigError("Claim repository not configured".into()))?;

        let result = claim_repo
            .atomic_confirm(license_id, session_token, owner_id, device_id.as_deref(), referral_id)
            .await?;

        // Convert to ClaimResponse
        Ok(ClaimResponse {
            success: true,
            message: None,
            license_id: Some(result.license_id.to_string()),
            license_key: Some(result.lease_code.clone()),
            claimed_at: Some(result.claimed_at),
            license: None, // Full license details not included in atomic confirm
            error: None,
        })
    }

    /// Release a reservation (allows other users to claim)
    pub async fn release_reservation(
        &self,
        license_id: &str,
        session_token: &str,
    ) -> Result<(), AppError> {
        Self::ensure_issuance_ready()?;
        let claim_repo = self
            .claim_repo
            .as_ref()
            .ok_or_else(|| AppError::ConfigError("Claim repository not configured".into()))?;

        claim_repo.release_reservation(license_id, session_token).await
    }

    /// Claim a license by split type (auto-assigns an available license).
    ///
    /// # R5-05 DEPRECATED
    /// This method uses non-atomic operations and should NOT be used for new code.
    /// Use `atomic_reserve_with_capacity()` + `atomic_confirm()` from ClaimRepository instead.
    /// Issues with this method:
    /// - No transactional guarantee between get and claim
    /// - No capacity ceiling enforcement
    /// - No referral attribution (use atomic_confirm for frozen attribution)
    /// - Credential (lease_code) immediately exposed
    #[deprecated(since = "2.0.0", note = "Use ClaimRepository::atomic_reserve_with_capacity() + atomic_confirm() instead")]
    pub async fn claim_by_split_type(
        &self,
        split_type: SplitType,
        device_id: Option<String>,
    ) -> Result<ClaimResponse, AppError> {
        Self::ensure_issuance_ready()?;

        tracing::warn!(
            "R5-05: Using deprecated claim_by_split_type - migrate to atomic_reserve + atomic_confirm"
        );

        let api_split = convert_to_api_split(split_type);

        // Get the first unclaimed license of this type
        let license = self
            .license_repo
            .get_first_unclaimed(api_split)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let license = match license {
            Some(l) => l,
            None => {
                return Ok(ClaimResponse::error(
                    "No licenses available for this split type",
                ));
            }
        };

        // Claim the license
        let claimed_license = self
            .license_repo
            .claim(&license.id, device_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(ClaimResponse::success(convert_to_local_license(
            claimed_license,
        )))
    }

    /// Reserve a license by split type (returns license info WITHOUT marking as claimed).
    /// This is phase 1 of the two-phase claim process.
    ///
    /// # R5-05 DEPRECATED
    /// This method uses non-atomic operations and should NOT be used for new code.
    /// Use `atomic_reserve_with_capacity()` from ClaimRepository instead.
    /// Issues with this method:
    /// - No session binding for reservation
    /// - No capacity ceiling enforcement
    /// - No publication/quarantine status checking
    #[deprecated(since = "2.0.0", note = "Use ClaimRepository::atomic_reserve_with_capacity() instead")]
    pub async fn reserve_by_split_type(
        &self,
        split_type: SplitType,
    ) -> Result<ClaimResponse, AppError> {
        Self::ensure_issuance_ready()?;

        tracing::warn!(
            "R5-05: Using deprecated reserve_by_split_type - migrate to atomic_reserve_with_capacity"
        );

        let api_split = convert_to_api_split(split_type);

        // Get the first unclaimed license of this type (without claiming)
        let license = self
            .license_repo
            .get_first_unclaimed(api_split)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let license = match license {
            Some(l) => l,
            None => {
                return Ok(ClaimResponse::error(
                    "No licenses available for this split type",
                ));
            }
        };

        // Return license info without marking as claimed
        Ok(ClaimResponse::success(convert_to_local_license(license)))
    }

    /// Reserve the next available license (any split type).
    /// Returns license info WITHOUT marking as claimed.
    /// This is phase 1 of the simplified two-phase claim process.
    pub async fn reserve_next_available(&self) -> Result<ClaimResponse, AppError> {
        Self::ensure_issuance_ready()?;
        // Get the first unclaimed license (any split type)
        let license = self
            .license_repo
            .get_first_unclaimed_any()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let license = match license {
            Some(l) => l,
            None => {
                return Ok(ClaimResponse::error("No licenses available"));
            }
        };

        // Return license info without marking as claimed
        Ok(ClaimResponse::success(convert_to_local_license(license)))
    }

    /// Check if any licenses are available for claiming.
    pub async fn has_available_licenses(&self) -> Result<bool, AppError> {
        if Self::ensure_issuance_ready().is_err() {
            return Ok(false);
        }
        let count = self
            .license_repo
            .count_unclaimed()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    /// Get the count of available (unclaimed) licenses.
    pub async fn get_available_count(&self) -> Result<i64, AppError> {
        if Self::ensure_issuance_ready().is_err() {
            return Ok(0);
        }
        self.license_repo
            .count_unclaimed()
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Confirm a license claim by ID (marks as claimed).
    /// This is phase 2 of the two-phase claim process - called when user copies the key.
    ///
    /// # R5-05 DEPRECATED
    /// This method uses non-atomic operations and should NOT be used for new code.
    /// Use `atomic_confirm()` from ClaimRepository instead.
    /// Issues with this method:
    /// - No session token verification (anyone with license_id can confirm)
    /// - No referral attribution or freezing
    /// - No agent approval validation
    /// - Separate check-then-claim creates race condition
    #[deprecated(since = "2.0.0", note = "Use ClaimRepository::atomic_confirm() instead")]
    pub async fn confirm_claim(
        &self,
        license_id: &str,
        device_id: Option<String>,
    ) -> Result<ClaimResponse, AppError> {
        Self::ensure_issuance_ready()?;

        tracing::warn!(
            "R5-05: Using deprecated confirm_claim - migrate to atomic_confirm"
        );

        // Validate license ID is not empty
        if license_id.is_empty() {
            return Err(AppError::ValidationError("License ID cannot be empty".to_string()));
        }

        // Check if license exists and is not already claimed
        let license = self
            .license_repo
            .get_by_id(license_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let license = match license {
            Some(l) => l,
            None => {
                return Ok(ClaimResponse::error("License not found"));
            }
        };

        // Check if already claimed
        if license.claimed {
            // If already claimed, just return success (idempotent)
            return Ok(ClaimResponse::success(convert_to_local_license(license)));
        }

        // Claim the license
        let claimed_license = self
            .license_repo
            .claim(license_id, device_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(ClaimResponse::success(convert_to_local_license(
            claimed_license,
        )))
    }

    /// Claim a license by lease code.
    pub async fn claim_license(&self, request: ClaimRequest) -> Result<ClaimResponse, AppError> {
        Self::ensure_issuance_ready()?;
        // Find the license by lease code
        let license = self
            .license_repo
            .get_by_lease_code(&request.lease_code)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let license = match license {
            Some(l) => l,
            None => {
                return Ok(ClaimResponse::error("License not found"));
            }
        };

        // Check if already claimed
        if license.claimed {
            return Ok(ClaimResponse::error("License has already been claimed"));
        }

        // Check if valid
        if !license.is_valid() {
            if license.is_expired() {
                return Ok(ClaimResponse::error("License has expired"));
            } else {
                return Ok(ClaimResponse::error("License is not yet valid"));
            }
        }

        // Claim the license
        let claimed_license = self
            .license_repo
            .claim(&license.id, request.device_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(ClaimResponse::success(convert_to_local_license(
            claimed_license,
        )))
    }

    /// Get a license by lease code (for display).
    pub async fn get_license_by_code(
        &self,
        lease_code: &str,
    ) -> Result<Option<ClaimPageData>, AppError> {
        Self::ensure_issuance_ready()?;
        let license = self
            .license_repo
            .get_by_lease_code(lease_code)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(license.map(|l| ClaimPageData::from(convert_to_local_license(l))))
    }

    /// Get available license count by split type.
    pub async fn get_available_by_split(&self, split_type: SplitType) -> Result<i64, AppError> {
        if Self::ensure_issuance_ready().is_err() {
            return Ok(0);
        }
        let api_split = convert_to_api_split(split_type);
        self.license_repo
            .count_unclaimed_by_split(api_split)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Get license summary statistics.
    pub async fn get_summary(&self) -> Result<LicenseSummary, AppError> {
        let api_summary = self
            .license_repo
            .get_summary()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(convert_to_local_summary(api_summary))
    }

    /// Get available licenses by split type.
    pub async fn get_available_splits(&self) -> Result<Vec<SplitTypeAvailability>, AppError> {
        // Use summary to get both available and claimed counts
        let summary = self
            .license_repo
            .get_summary()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = summary
            .by_split_type
            .into_iter()
            .map(|s| SplitTypeAvailability {
                split_type: convert_to_local_split(s.split_type),
                available: if Self::ensure_issuance_ready().is_err() {
                    0
                } else {
                    s.available
                },
                claimed: s.claimed,
                total: s.total,
            })
            .collect();

        Ok(result)
    }
}

/// Availability information for a split type.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SplitTypeAvailability {
    pub split_type: SplitType,
    pub available: i64,
    pub claimed: i64,
    pub total: i64,
}

// Conversion functions between local and uno-api types

fn convert_to_local_license(api_license: uno_api::models::License) -> License {
    License {
        id: api_license.id,
        lease_code: api_license.lease_code,
        valid_from: api_license.valid_from,
        valid_to: api_license.valid_to,
        split_type: convert_to_local_split(api_license.split_type),
        claimed: api_license.claimed,
        bound_to_device: api_license.bound_to_device,
        device_id: api_license.device_id,
        claimed_at: api_license.claimed_at,
        created_at: api_license.created_at,
    }
}

fn convert_to_local_split(api_split: uno_api::models::SplitType) -> SplitType {
    match api_split {
        uno_api::models::SplitType::Split5050 => SplitType::Split5050,
        uno_api::models::SplitType::Split5545 => SplitType::Split5545,
        uno_api::models::SplitType::Split6040 => SplitType::Split6040,
    }
}

fn convert_to_api_split(local_split: SplitType) -> uno_api::models::SplitType {
    match local_split {
        SplitType::Split5050 => uno_api::models::SplitType::Split5050,
        SplitType::Split5545 => uno_api::models::SplitType::Split5545,
        SplitType::Split6040 => uno_api::models::SplitType::Split6040,
    }
}

fn convert_to_db_split(local_split: SplitType) -> String {
    match local_split {
        SplitType::Split5050 => "50:50".to_string(),
        SplitType::Split5545 => "55:45".to_string(),
        SplitType::Split6040 => "60:40".to_string(),
    }
}

fn convert_to_local_summary(api_summary: uno_api::models::LicenseSummary) -> LicenseSummary {
    use crate::types::SplitTypeSummary;

    LicenseSummary {
        total: api_summary.total,
        claimed: api_summary.claimed,
        unclaimed: api_summary.available, // uno-api uses 'available', uno-app uses 'unclaimed'
        expired: api_summary.expired,
        by_split_type: api_summary
            .by_split_type
            .into_iter()
            .map(|s| SplitTypeSummary {
                split_type: convert_to_local_split(s.split_type),
                total: s.total,
                claimed: s.claimed,
                unclaimed: s.available, // uno-api uses 'available', uno-app uses 'unclaimed'
            })
            .collect(),
    }
}
