//! Claim repository for database operations
//!
//! Implements atomic reservation and claim operations with:
//! - Session-bound reservations with unguessable tokens
//! - Automatic expiry of stale reservations
//! - Immutable referral attribution
//! - Owner verification for confirmations

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use std::sync::Arc;
use uuid::Uuid;
use crate::server::db::ConnectionPool;
use crate::types::AppError;

/// Dynamic type alias for ClaimRepository trait object
pub type DynClaimRepository = Arc<dyn ClaimRepository + Send + Sync>;

/// Default reservation expiry time in seconds (2 minutes)
pub const RESERVATION_EXPIRY_SECS: i64 = 120;

/// Claimed license data for sync
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ClaimedLicense {
    pub license_id: String,
    pub claimed_at: Option<DateTime<Utc>>,
    pub referral_code: Option<String>,
    pub claim_token: String,
}

/// Reservation result returned by atomic_reserve
#[derive(Debug, Clone)]
pub struct ReservationResult {
    /// License ID (can be UUID or blockchain hex format)
    pub license_id: String,
    pub lease_code: String,
    pub session_token: String,
    pub expires_at: DateTime<Utc>,
    pub split_type: String,
    pub referral_validated: bool,
}

/// Claim result returned by atomic_confirm
#[derive(Debug, Clone)]
pub struct ClaimResult {
    /// License ID (can be UUID or blockchain hex format)
    pub license_id: String,
    pub lease_code: String,
    pub claimed_at: DateTime<Utc>,
    pub referral_id: Option<i32>,
    pub referral_attributed_at: Option<DateTime<Utc>>,
    pub agreement_version: Option<i32>,
}

/// Claim repository trait defining database operations
#[async_trait]
pub trait ClaimRepository: Send + Sync {
    /// Set the referral_id for a claim (by license_id)
    /// DEPRECATED: Use atomic_confirm with referral_id instead
    async fn set_referral(&self, license_id: &str, referral_id: i32) -> Result<(), AppError>;

    /// Get claimed licenses since a given timestamp (for uno-admin sync)
    async fn get_claimed_since(
        &self,
        since: Option<DateTime<Utc>>,
        limit: i32,
    ) -> Result<Vec<ClaimedLicense>, AppError>;

    /// Atomically reserve a license with session binding
    ///
    /// This performs in a single transaction:
    /// 1. SELECT first available license FOR UPDATE SKIP LOCKED
    /// 2. INSERT into license_reservations with session token
    /// 3. UPDATE licenses SET reserved_until, reservation_token
    /// 4. Optionally validate referral code
    ///
    /// Returns reservation with session token for confirmation
    async fn atomic_reserve(
        &self,
        split_type: Option<&str>,
        referral_code: Option<&str>,
    ) -> Result<ReservationResult, AppError>;

    /// Atomically confirm a reservation and claim the license
    ///
    /// This performs in a single transaction:
    /// 1. Verify session_token matches reservation
    /// 2. Verify reservation not expired
    /// 3. UPDATE licenses SET claimed=true, referral_id (immutable), etc.
    /// 4. UPDATE license_reservations SET status='claimed'
    ///
    /// Referral attribution is immutable - once set, cannot be changed
    async fn atomic_confirm(
        &self,
        license_id: &str,
        session_token: &str,
        device_id: Option<&str>,
        referral_id: Option<i32>,
    ) -> Result<ClaimResult, AppError>;

    /// Release an expired or abandoned reservation
    async fn release_reservation(
        &self,
        license_id: &str,
        session_token: &str,
    ) -> Result<(), AppError>;

    /// Clean up all expired reservations
    /// Should be called periodically by a background task
    async fn cleanup_expired_reservations(&self) -> Result<u64, AppError>;

    /// Check if a license has an active (non-expired) reservation
    async fn has_active_reservation(&self, license_id: &str) -> Result<bool, AppError>;

    /// Get the current agreement version for referral attribution
    async fn get_current_agreement_version(&self) -> Result<i32, AppError>;
}

/// Concrete implementation of ClaimRepository
pub struct ClaimRepositoryImpl {
    pub db_pool: ConnectionPool,
}

impl ClaimRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }

    /// Generate a cryptographically secure session token
    fn generate_session_token() -> String {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};

        // Use UUID v4 for the token (128 bits of randomness)
        let uuid = Uuid::new_v4();
        // Add extra entropy from system hasher
        let state = RandomState::new();
        let mut hasher = state.build_hasher();
        hasher.write(uuid.as_bytes());
        let extra = hasher.finish();

        format!("{}_{:016x}", uuid, extra)
    }
}

#[async_trait]
impl ClaimRepository for ClaimRepositoryImpl {
    async fn set_referral(&self, license_id: &str, referral_id: i32) -> Result<(), AppError> {
        // SECURITY: Only set referral if not already set (immutable attribution)
        let result = sqlx::query(r#"
            UPDATE licenses
            SET referral_id = $1,
                referral_attributed_at = NOW()
            WHERE id = $2::uuid
              AND referral_id IS NULL
        "#)
        .bind(referral_id)
        .bind(license_id)
        .execute(&self.db_pool)
        .await?;

        if result.rows_affected() == 0 {
            // Check if referral was already set
            let existing: Option<(Option<i32>,)> = sqlx::query_as(
                "SELECT referral_id FROM licenses WHERE id = $1::uuid"
            )
            .bind(license_id)
            .fetch_optional(&self.db_pool)
            .await?;

            if let Some((Some(_),)) = existing {
                // Referral already set - this is expected for immutability
                tracing::debug!("Referral already set for license {}, ignoring update", license_id);
            }
        }

        Ok(())
    }

    async fn get_claimed_since(
        &self,
        since: Option<DateTime<Utc>>,
        limit: i32,
    ) -> Result<Vec<ClaimedLicense>, AppError> {
        let query = match since {
            Some(ts) => {
                sqlx::query_as::<_, ClaimedLicense>(r#"
                    SELECT
                        l.id::text as license_id,
                        l.claimed_at,
                        r.referral_code,
                        l.lease_code as claim_token
                    FROM licenses l
                    LEFT JOIN referrals r ON l.referral_id = r.id
                    WHERE l.claimed = true
                      AND l.claimed_at > $1
                    ORDER BY l.claimed_at ASC
                    LIMIT $2
                "#)
                .bind(ts)
                .bind(limit)
                .fetch_all(&self.db_pool)
                .await?
            }
            None => {
                sqlx::query_as::<_, ClaimedLicense>(r#"
                    SELECT
                        l.id::text as license_id,
                        l.claimed_at,
                        r.referral_code,
                        l.lease_code as claim_token
                    FROM licenses l
                    LEFT JOIN referrals r ON l.referral_id = r.id
                    WHERE l.claimed = true
                    ORDER BY l.claimed_at ASC
                    LIMIT $1
                "#)
                .bind(limit)
                .fetch_all(&self.db_pool)
                .await?
            }
        };

        Ok(query)
    }

    async fn atomic_reserve(
        &self,
        split_type: Option<&str>,
        referral_code: Option<&str>,
    ) -> Result<ReservationResult, AppError> {
        let now = Utc::now();
        let expires_at = now + Duration::seconds(RESERVATION_EXPIRY_SECS);
        let session_token = Self::generate_session_token();

        // Start transaction
        let mut tx = self.db_pool.begin().await
            .map_err(|e| AppError::Database(format!("Failed to start transaction: {}", e)))?;

        // First, clean up any expired reservations to free licenses
        sqlx::query(r#"
            UPDATE licenses
            SET reserved_until = NULL, reservation_token = NULL
            WHERE reserved_until IS NOT NULL AND reserved_until < $1
        "#)
        .bind(now)
        .execute(&mut *tx)
        .await?;

        // Also mark expired reservations in the reservations table
        sqlx::query(r#"
            UPDATE license_reservations
            SET status = 'expired', released_at = $1
            WHERE status = 'active' AND expires_at < $1
        "#)
        .bind(now)
        .execute(&mut *tx)
        .await?;

        // Select first available license with row-level lock
        let license: Option<LicenseReserveRow> = if let Some(st) = split_type {
            sqlx::query_as::<_, LicenseReserveRow>(r#"
                SELECT id, lease_code, split_type::text as split_type
                FROM licenses
                WHERE claimed = false
                  AND (reserved_until IS NULL OR reserved_until < $1)
                  AND valid_from <= $1
                  AND valid_to > $1
                  AND split_type = $2::split_type
                ORDER BY created_at ASC
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            "#)
            .bind(now)
            .bind(st)
            .fetch_optional(&mut *tx)
            .await?
        } else {
            sqlx::query_as::<_, LicenseReserveRow>(r#"
                SELECT id, lease_code, split_type::text as split_type
                FROM licenses
                WHERE claimed = false
                  AND (reserved_until IS NULL OR reserved_until < $1)
                  AND valid_from <= $1
                  AND valid_to > $1
                ORDER BY created_at ASC
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            "#)
            .bind(now)
            .fetch_optional(&mut *tx)
            .await?
        };

        let license = license.ok_or_else(|| {
            AppError::NotFound("No licenses available for reservation".into())
        })?;

        // Validate referral code if provided (but don't attribute yet)
        let referral_validated = if let Some(code) = referral_code {
            let exists: (bool,) = sqlx::query_as(
                "SELECT EXISTS(SELECT 1 FROM referrals WHERE referral_code = $1 AND status = 'active')"
            )
            .bind(code.to_uppercase())
            .fetch_one(&mut *tx)
            .await?;
            exists.0
        } else {
            false
        };

        // Update license with reservation
        sqlx::query(r#"
            UPDATE licenses
            SET reserved_until = $2, reservation_token = $3
            WHERE id = $1
        "#)
        .bind(&license.id)
        .bind(expires_at)
        .bind(&session_token)
        .execute(&mut *tx)
        .await?;

        // Insert into reservations table for audit trail
        sqlx::query(r#"
            INSERT INTO license_reservations (license_id, session_token, reserved_at, expires_at, status)
            VALUES ($1, $2, $3, $4, 'active')
            ON CONFLICT (license_id) DO UPDATE
            SET session_token = $2, reserved_at = $3, expires_at = $4, status = 'active', released_at = NULL
        "#)
        .bind(&license.id)
        .bind(&session_token)
        .bind(now)
        .bind(expires_at)
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await
            .map_err(|e| AppError::Database(format!("Failed to commit reservation: {}", e)))?;

        Ok(ReservationResult {
            license_id: license.id,
            lease_code: license.lease_code,
            session_token,
            expires_at,
            split_type: license.split_type,
            referral_validated,
        })
    }

    async fn atomic_confirm(
        &self,
        license_id: &str,
        session_token: &str,
        device_id: Option<&str>,
        referral_id: Option<i32>,
    ) -> Result<ClaimResult, AppError> {
        let now = Utc::now();

        // Start transaction
        let mut tx = self.db_pool.begin().await
            .map_err(|e| AppError::Database(format!("Failed to start transaction: {}", e)))?;

        // Verify reservation exists and is valid
        let reservation: Option<ReservationRow> = sqlx::query_as::<_, ReservationRow>(r#"
            SELECT session_token, expires_at, status
            FROM license_reservations
            WHERE license_id = $1
        "#)
        .bind(license_id)
        .fetch_optional(&mut *tx)
        .await?;

        // Also check license-level reservation
        let license_check: Option<LicenseCheckRow> = sqlx::query_as::<_, LicenseCheckRow>(r#"
            SELECT claimed, reservation_token, reserved_until, referral_id
            FROM licenses
            WHERE id = $1
            FOR UPDATE
        "#)
        .bind(license_id)
        .fetch_optional(&mut *tx)
        .await?;

        let license = license_check.ok_or_else(|| {
            AppError::NotFound(format!("License {} not found", license_id))
        })?;

        // If already claimed, return success (idempotent)
        if license.claimed {
            // Fetch the claimed license data
            let result: ClaimResultRow = sqlx::query_as::<_, ClaimResultRow>(r#"
                SELECT id, lease_code, claimed_at, referral_id, referral_attributed_at, referral_agreement_version
                FROM licenses WHERE id = $1
            "#)
            .bind(license_id)
            .fetch_one(&mut *tx)
            .await?;

            tx.commit().await?;

            return Ok(ClaimResult {
                license_id: result.id,
                lease_code: result.lease_code,
                claimed_at: result.claimed_at.unwrap_or(now),
                referral_id: result.referral_id,
                referral_attributed_at: result.referral_attributed_at,
                agreement_version: result.referral_agreement_version,
            });
        }

        // Verify session token matches
        let token_valid = if let Some(ref res) = reservation {
            res.session_token == session_token && res.status == "active"
        } else {
            false
        };

        let license_token_valid = license.reservation_token.as_deref() == Some(session_token);

        if !token_valid && !license_token_valid {
            return Err(AppError::Unauthorized(
                "Invalid or expired reservation token".into()
            ));
        }

        // Check expiry
        if let Some(expires) = license.reserved_until {
            if expires < now {
                return Err(AppError::ValidationError(
                    "Reservation has expired. Please reserve again.".into()
                ));
            }
        }

        // Get current agreement version for referral attribution
        let agreement_version: Option<i32> = if referral_id.is_some() {
            let version: Option<(i32,)> = sqlx::query_as(
                "SELECT version FROM agreement_versions WHERE is_active = true ORDER BY version DESC LIMIT 1"
            )
            .fetch_optional(&mut *tx)
            .await?;
            version.map(|(v,)| v)
        } else {
            None
        };

        // SECURITY: Referral is immutable - only set if not already set
        let (final_referral_id, referral_attributed_at) = if license.referral_id.is_some() {
            // Already has referral - keep existing (immutable)
            (license.referral_id, None)
        } else if let Some(ref_id) = referral_id {
            // Set new referral with attribution timestamp
            (Some(ref_id), Some(now))
        } else {
            (None, None)
        };

        // Claim the license atomically
        let bound_to_device = device_id.is_some();
        sqlx::query(r#"
            UPDATE licenses
            SET claimed = true,
                claimed_at = $2,
                bound_to_device = $3,
                device_id = $4,
                reserved_until = NULL,
                reservation_token = NULL,
                referral_id = COALESCE(referral_id, $5),
                referral_attributed_at = COALESCE(referral_attributed_at, $6),
                referral_agreement_version = COALESCE(referral_agreement_version, $7)
            WHERE id = $1 AND claimed = false
        "#)
        .bind(license_id)
        .bind(now)
        .bind(bound_to_device)
        .bind(device_id)
        .bind(final_referral_id)
        .bind(referral_attributed_at)
        .bind(agreement_version)
        .execute(&mut *tx)
        .await?;

        // Update reservation status
        sqlx::query(r#"
            UPDATE license_reservations
            SET status = 'claimed', claimed_at = $2
            WHERE license_id = $1 AND session_token = $3
        "#)
        .bind(license_id)
        .bind(now)
        .bind(session_token)
        .execute(&mut *tx)
        .await?;

        // Fetch the final result
        let result: ClaimResultRow = sqlx::query_as::<_, ClaimResultRow>(r#"
            SELECT id, lease_code, claimed_at, referral_id, referral_attributed_at, referral_agreement_version
            FROM licenses WHERE id = $1
        "#)
        .bind(license_id)
        .fetch_one(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await
            .map_err(|e| AppError::Database(format!("Failed to commit claim: {}", e)))?;

        Ok(ClaimResult {
            license_id: result.id,
            lease_code: result.lease_code,
            claimed_at: result.claimed_at.unwrap_or(now),
            referral_id: result.referral_id,
            referral_attributed_at: result.referral_attributed_at,
            agreement_version: result.referral_agreement_version,
        })
    }

    async fn release_reservation(
        &self,
        license_id: &str,
        session_token: &str,
    ) -> Result<(), AppError> {
        let now = Utc::now();

        // Clear reservation on license
        sqlx::query(r#"
            UPDATE licenses
            SET reserved_until = NULL, reservation_token = NULL
            WHERE id = $1 AND reservation_token = $2
        "#)
        .bind(license_id)
        .bind(session_token)
        .execute(&self.db_pool)
        .await?;

        // Mark reservation as released
        sqlx::query(r#"
            UPDATE license_reservations
            SET status = 'released', released_at = $3
            WHERE license_id = $1 AND session_token = $2
        "#)
        .bind(license_id)
        .bind(session_token)
        .bind(now)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    async fn cleanup_expired_reservations(&self) -> Result<u64, AppError> {
        let now = Utc::now();

        // Clear expired reservations from licenses
        let result = sqlx::query(r#"
            UPDATE licenses
            SET reserved_until = NULL, reservation_token = NULL
            WHERE reserved_until IS NOT NULL AND reserved_until < $1
        "#)
        .bind(now)
        .execute(&self.db_pool)
        .await?;

        let cleared = result.rows_affected();

        // Mark expired reservations
        sqlx::query(r#"
            UPDATE license_reservations
            SET status = 'expired', released_at = $1
            WHERE status = 'active' AND expires_at < $1
        "#)
        .bind(now)
        .execute(&self.db_pool)
        .await?;

        Ok(cleared)
    }

    async fn has_active_reservation(&self, license_id: &str) -> Result<bool, AppError> {
        let now = Utc::now();

        let result: (bool,) = sqlx::query_as(r#"
            SELECT EXISTS(
                SELECT 1 FROM licenses
                WHERE id = $1
                  AND reserved_until IS NOT NULL
                  AND reserved_until > $2
            )
        "#)
        .bind(license_id)
        .bind(now)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result.0)
    }

    async fn get_current_agreement_version(&self) -> Result<i32, AppError> {
        let result: Option<(i32,)> = sqlx::query_as(
            "SELECT version FROM agreement_versions WHERE is_active = true ORDER BY version DESC LIMIT 1"
        )
        .fetch_optional(&self.db_pool)
        .await?;

        result
            .map(|(v,)| v)
            .ok_or_else(|| AppError::NotFound("No active agreement version found".into()))
    }
}

// Internal row types for SQLx mapping

#[derive(Debug, sqlx::FromRow)]
struct LicenseReserveRow {
    // Use String for id to handle both UUID and blockchain hex formats
    id: String,
    lease_code: String,
    split_type: String,
}

#[derive(Debug, sqlx::FromRow)]
struct ReservationRow {
    session_token: String,
    expires_at: DateTime<Utc>,
    status: String,
}

#[derive(Debug, sqlx::FromRow)]
struct LicenseCheckRow {
    claimed: bool,
    reservation_token: Option<String>,
    reserved_until: Option<DateTime<Utc>>,
    referral_id: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
struct ClaimResultRow {
    // Use String for id to handle both UUID and blockchain hex formats
    id: String,
    lease_code: String,
    claimed_at: Option<DateTime<Utc>>,
    referral_id: Option<i32>,
    referral_attributed_at: Option<DateTime<Utc>>,
    referral_agreement_version: Option<i32>,
}
