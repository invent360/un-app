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
///
/// R5-05: The lease_code is NOT included in the reservation result.
/// Credentials are only revealed after confirmation to prevent
/// credential exposure before ownership is established.
#[derive(Debug, Clone)]
pub struct ReservationResult {
    /// License ID (can be UUID or blockchain hex format)
    pub license_id: String,
    /// Session token for confirming the reservation
    pub session_token: String,
    /// When the reservation expires
    pub expires_at: DateTime<Utc>,
    /// Split type for this license
    pub split_type: String,
    /// Whether the referral code was validated (not yet attributed)
    pub referral_validated: bool,
    /// R5-05: Remaining capacity after this reservation
    pub capacity_remaining: Option<i64>,
}

/// Claim result returned by atomic_confirm
///
/// R5-05: The lease_code IS included here - credential revealed only after
/// successful confirmation when ownership is established.
#[derive(Debug, Clone)]
pub struct ClaimResult {
    /// License ID (can be UUID or blockchain hex format)
    pub license_id: String,
    /// The credential - revealed only at confirmation time
    pub lease_code: String,
    /// When the license was claimed
    pub claimed_at: DateTime<Utc>,
    /// Referral ID if attribution was made
    pub referral_id: Option<i32>,
    /// When referral was attributed
    pub referral_attributed_at: Option<DateTime<Utc>>,
    /// Agreement version at time of referral attribution
    pub agreement_version: Option<i32>,
    /// R5-05: Referral attribution is now frozen (immutable, including no-referral)
    pub referral_frozen: bool,
}

/// Claim repository trait defining database operations
#[async_trait]
pub trait ClaimRepository: Send + Sync {
    /// Set the referral_id for a claim (by license_id)
    /// DEPRECATED: Use atomic_confirm with referral_id instead
    async fn set_referral(&self, license_id: &str, referral_id: i32) -> Result<(), AppError>;

    /// Get claimed licenses since a given timestamp (for uno-admin sync)
    ///
    /// DEPRECATED: Use `get_claimed_since_compound` for correct cursor-based pagination.
    /// This method can skip records with identical timestamps.
    async fn get_claimed_since(
        &self,
        since: Option<DateTime<Utc>>,
        limit: i32,
    ) -> Result<Vec<ClaimedLicense>, AppError>;

    /// Get claimed licenses using compound cursor (R5-06)
    ///
    /// Uses compound key (claimed_at, license_id) for correct pagination.
    /// This prevents skipping records that have the same timestamp.
    ///
    /// - `since_ts`: Start after this timestamp (exclusive if high_water_id matches)
    /// - `high_water_id`: Last processed license_id at `since_ts` (for disambiguation)
    async fn get_claimed_since_compound(
        &self,
        since_ts: Option<DateTime<Utc>>,
        high_water_id: Option<&str>,
        limit: i32,
    ) -> Result<Vec<ClaimedLicense>, AppError>;

    /// Atomically reserve a license with session binding and capacity enforcement
    ///
    /// B3 FIX: This is now the ONLY reservation method. Anonymous reservations
    /// are no longer supported - all reservations must have an authenticated user.
    ///
    /// This performs in a single transaction:
    /// 1. Check capacity ceiling (prevents over-allocation)
    /// 2. SELECT first available license FOR UPDATE SKIP LOCKED
    /// 3. INSERT into license_reservations with session token and user_id
    /// 4. UPDATE licenses SET reserved_until, reservation_token
    /// 5. Optionally validate referral code
    ///
    /// F2: user_id is stored to bind reservation to owner for confirmation.
    /// R5-05: capacity_ceiling is enforced INSIDE the transaction to prevent TOCTOU.
    ///
    /// Returns reservation with session token for confirmation
    async fn atomic_reserve(
        &self,
        user_id: &str,
        split_type: Option<&str>,
        referral_code: Option<&str>,
        capacity_ceiling: i64,
    ) -> Result<ReservationResult, AppError>;

    /// Atomically confirm a reservation and claim the license
    ///
    /// This performs in a single transaction:
    /// 1. Verify session_token matches reservation
    /// 2. Verify owner_id matches the user who made the reservation (F2)
    /// 3. Verify reservation not expired
    /// 4. UPDATE licenses SET claimed=true, issued_to, referral_id (immutable), etc.
    /// 5. UPDATE license_reservations SET status='claimed'
    ///
    /// F2: The owner_id MUST match the user_id stored during reservation.
    /// This prevents credential theft where an attacker who knows the
    /// license_id could claim someone else's reservation.
    ///
    /// Referral attribution is immutable - once set, cannot be changed
    async fn atomic_confirm(
        &self,
        license_id: &str,
        session_token: &str,
        owner_id: &str,
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

    /// Internal method that performs the actual reservation attempt.
    /// Called by atomic_reserve with retry logic.
    async fn try_atomic_reserve(
        &self,
        user_id: &str,
        split_type: Option<&str>,
        referral_code: Option<&str>,
        capacity_ceiling: Option<i64>,
    ) -> Result<ReservationResult, AppError> {
        let now = Utc::now();
        let expires_at = now + Duration::seconds(RESERVATION_EXPIRY_SECS);
        let session_token = Self::generate_session_token();

        // F2: Start transaction and SET SERIALIZABLE isolation level
        // This ensures capacity checks and reservations are serialized
        let mut tx = self.db_pool.begin().await
            .map_err(|e| AppError::Database(format!("Failed to start transaction: {}", e)))?;

        // F2: Actually set SERIALIZABLE isolation (not just default READ COMMITTED)
        sqlx::query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Database(format!("Failed to set isolation level: {}", e)))?;

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

        // R5-05: Check capacity ceiling INSIDE the transaction
        // This count happens after cleanup and before we lock our target license
        // INCLUDES: claimed licenses, active reservations, AND pending releases
        let capacity_remaining = if let Some(ceiling) = capacity_ceiling {
            let count: (i64,) = sqlx::query_as(
                r#"
                SELECT COUNT(*) as count
                FROM licenses
                WHERE claimed = true
                   OR (reserved_until IS NOT NULL AND reserved_until > $1)
                   OR (is_pending_release = true)
                "#,
            )
            .bind(now)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| AppError::Database(format!("Capacity check failed: {}", e)))?;

            if count.0 >= ceiling {
                tracing::warn!(
                    occupied = count.0,
                    ceiling = ceiling,
                    "Capacity ceiling reached (inside transaction)"
                );
                return Err(AppError::ValidationError(
                    "License capacity has been reached. Please try again later.".into()
                ));
            }

            Some(ceiling - count.0 - 1) // -1 because we're about to reserve one
        } else {
            None
        };

        // Select first available license with row-level lock
        // R5-05: FOR UPDATE SKIP LOCKED ensures atomic acquisition
        // R5-05: Do NOT select lease_code - credentials not revealed until confirm
        let license: Option<LicenseReserveRow> = if let Some(st) = split_type {
            sqlx::query_as::<_, LicenseReserveRow>(r#"
                SELECT id, split_type::text as split_type
                FROM licenses
                WHERE claimed = false
                  AND (reserved_until IS NULL OR reserved_until < $1)
                  AND valid_from <= $1
                  AND valid_to > $1
                  AND split_type = $2::split_type
                  AND (publication_status = 'published' OR publication_status IS NULL)
                  AND (is_quarantined = false OR is_quarantined IS NULL)
                  AND (is_pending_release = false OR is_pending_release IS NULL)
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
                SELECT id, split_type::text as split_type
                FROM licenses
                WHERE claimed = false
                  AND (reserved_until IS NULL OR reserved_until < $1)
                  AND valid_from <= $1
                  AND valid_to > $1
                  AND (publication_status = 'published' OR publication_status IS NULL)
                  AND (is_quarantined = false OR is_quarantined IS NULL)
                  AND (is_pending_release = false OR is_pending_release IS NULL)
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

        // R3-07: Validate referral code if provided (but don't attribute yet)
        // Check that both the referral is active AND the agent is approved
        let referral_validated = if let Some(code) = referral_code {
            let valid: (bool,) = sqlx::query_as(
                r#"
                SELECT EXISTS(
                    SELECT 1 FROM referrals r
                    JOIN agents a ON r.agent_id = a.id
                    WHERE r.referral_code = $1
                      AND r.status = 'active'
                      AND a.status = 'approved'
                )
                "#
            )
            .bind(code.to_uppercase())
            .fetch_one(&mut *tx)
            .await?;
            valid.0
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

        // F2: Insert into reservations table with user_id for owner binding
        sqlx::query(r#"
            INSERT INTO license_reservations (license_id, session_token, user_id, reserved_at, expires_at, status)
            VALUES ($1, $2, $3, $4, $5, 'active')
            ON CONFLICT (license_id) DO UPDATE
            SET session_token = $2, user_id = $3, reserved_at = $4, expires_at = $5, status = 'active', released_at = NULL
        "#)
        .bind(&license.id)
        .bind(&session_token)
        .bind(user_id)
        .bind(now)
        .bind(expires_at)
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await
            .map_err(|e| AppError::Database(format!("Failed to commit reservation: {}", e)))?;

        // R5-05: Return result WITHOUT lease_code - credential revealed only at confirm
        Ok(ReservationResult {
            license_id: license.id,
            session_token,
            expires_at,
            split_type: license.split_type,
            referral_validated,
            capacity_remaining,
        })
    }
}

#[async_trait]
impl ClaimRepository for ClaimRepositoryImpl {
    async fn set_referral(&self, license_id: &str, referral_id: i32) -> Result<(), AppError> {
        // R5-05: SECURITY: Only set referral if not already set AND not frozen
        // referral_frozen is set to true during atomic_confirm() to prevent changes
        let result = sqlx::query(r#"
            UPDATE licenses
            SET referral_id = $1,
                referral_attributed_at = NOW()
            WHERE id = $2::uuid
              AND referral_id IS NULL
              AND (referral_frozen = false OR referral_frozen IS NULL)
        "#)
        .bind(referral_id)
        .bind(license_id)
        .execute(&self.db_pool)
        .await?;

        if result.rows_affected() == 0 {
            // Check why update failed - either referral already set or frozen
            let existing: Option<(Option<i32>, Option<bool>)> = sqlx::query_as(
                "SELECT referral_id, referral_frozen FROM licenses WHERE id = $1::uuid"
            )
            .bind(license_id)
            .fetch_optional(&self.db_pool)
            .await?;

            match existing {
                Some((Some(_), _)) => {
                    // Referral already set - this is expected for immutability
                    tracing::debug!("Referral already set for license {}, ignoring update", license_id);
                }
                Some((None, Some(true))) => {
                    // R5-05: Referral frozen as "no referral" - cannot change
                    tracing::warn!(
                        "Attempted to set referral on frozen license {} - immutability enforced",
                        license_id
                    );
                }
                _ => {
                    tracing::debug!("License {} not found or other condition prevented update", license_id);
                }
            }
        }

        Ok(())
    }

    async fn get_claimed_since(
        &self,
        since: Option<DateTime<Utc>>,
        limit: i32,
    ) -> Result<Vec<ClaimedLicense>, AppError> {
        // Delegate to compound version without high-water mark
        self.get_claimed_since_compound(since, None, limit).await
    }

    async fn get_claimed_since_compound(
        &self,
        since_ts: Option<DateTime<Utc>>,
        high_water_id: Option<&str>,
        limit: i32,
    ) -> Result<Vec<ClaimedLicense>, AppError> {
        let results = match (since_ts, high_water_id) {
            // R5-06: Full compound cursor - excludes previously processed records
            // with the same timestamp by using (timestamp, id) > (cursor_ts, cursor_id)
            (Some(ts), Some(hwm_id)) => {
                sqlx::query_as::<_, ClaimedLicense>(r#"
                    SELECT
                        l.id::text as license_id,
                        l.claimed_at,
                        r.referral_code,
                        l.lease_code as claim_token
                    FROM licenses l
                    LEFT JOIN referrals r ON l.referral_id = r.id
                    WHERE l.claimed = true
                      AND (l.claimed_at, l.id::text) > ($1, $2)
                    ORDER BY l.claimed_at ASC, l.id ASC
                    LIMIT $3
                "#)
                .bind(ts)
                .bind(hwm_id)
                .bind(limit)
                .fetch_all(&self.db_pool)
                .await?
            }
            // Timestamp only - use greater-than for timestamp
            (Some(ts), None) => {
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
                    ORDER BY l.claimed_at ASC, l.id ASC
                    LIMIT $2
                "#)
                .bind(ts)
                .bind(limit)
                .fetch_all(&self.db_pool)
                .await?
            }
            // No cursor - start from beginning
            (None, _) => {
                sqlx::query_as::<_, ClaimedLicense>(r#"
                    SELECT
                        l.id::text as license_id,
                        l.claimed_at,
                        r.referral_code,
                        l.lease_code as claim_token
                    FROM licenses l
                    LEFT JOIN referrals r ON l.referral_id = r.id
                    WHERE l.claimed = true
                    ORDER BY l.claimed_at ASC, l.id ASC
                    LIMIT $1
                "#)
                .bind(limit)
                .fetch_all(&self.db_pool)
                .await?
            }
        };

        Ok(results)
    }

    /// B3 FIX: Unified atomic_reserve - no more anonymous path or optional capacity
    ///
    /// R5-05: The capacity check is now inside the transaction with proper locking.
    /// This prevents TOCTOU race conditions where two users could both pass a
    /// non-locked capacity check and then both attempt to reserve.
    ///
    /// F2: Uses SERIALIZABLE isolation with retry logic for serialization failures.
    /// Stores user_id to bind reservation to owner for confirmation.
    async fn atomic_reserve(
        &self,
        user_id: &str,
        split_type: Option<&str>,
        referral_code: Option<&str>,
        capacity_ceiling: i64,
    ) -> Result<ReservationResult, AppError> {
        // B3 FIX: Reject anonymous users - all reservations must be authenticated
        if user_id.is_empty() || user_id == "anonymous" {
            return Err(AppError::Unauthorized(
                "Authentication required for license reservation".into()
            ));
        }

        const MAX_SERIALIZATION_RETRIES: u32 = 3;

        for attempt in 0..MAX_SERIALIZATION_RETRIES {
            match self.try_atomic_reserve(user_id, split_type, referral_code, Some(capacity_ceiling)).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    // Check if this is a serialization failure (PostgreSQL error code 40001)
                    let error_str = e.to_string();
                    if error_str.contains("40001") || error_str.contains("serialization") {
                        if attempt < MAX_SERIALIZATION_RETRIES - 1 {
                            tracing::warn!(
                                attempt = attempt + 1,
                                max_retries = MAX_SERIALIZATION_RETRIES,
                                "Serialization failure in reservation, retrying"
                            );
                            // Brief backoff before retry
                            tokio::time::sleep(tokio::time::Duration::from_millis(10 * (attempt as u64 + 1))).await;
                            continue;
                        }
                    }
                    return Err(e);
                }
            }
        }

        Err(AppError::Database("Failed to reserve after max retries due to serialization conflicts".into()))
    }

    async fn atomic_confirm(
        &self,
        license_id: &str,
        session_token: &str,
        owner_id: &str,
        device_id: Option<&str>,
        referral_id: Option<i32>,
    ) -> Result<ClaimResult, AppError> {
        let now = Utc::now();

        // Start transaction
        let mut tx = self.db_pool.begin().await
            .map_err(|e| AppError::Database(format!("Failed to start transaction: {}", e)))?;

        // F2: Verify reservation exists and is valid - includes user_id for owner check
        let reservation: Option<ReservationRow> = sqlx::query_as::<_, ReservationRow>(r#"
            SELECT session_token, user_id, expires_at, status
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

        // F2: SECURITY FIX - Verify session token FIRST before revealing any credentials
        // This prevents credential theft where an attacker with knowledge of license_id
        // could obtain lease_code without valid session_token
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

        // F2: SECURITY - Verify owner_id matches the user who made the reservation
        // This prevents a different authenticated user from claiming someone else's reservation
        // B3 FIX: Removed "anonymous" exception - all reservations now require authentication
        if let Some(ref res) = reservation {
            if let Some(ref stored_user_id) = res.user_id {
                if stored_user_id != owner_id {
                    tracing::warn!(
                        license_id = %license_id,
                        stored_user = %stored_user_id,
                        claiming_user = %owner_id,
                        "SECURITY: Owner mismatch in reservation confirmation"
                    );
                    return Err(AppError::Unauthorized(
                        "This reservation belongs to a different user".into()
                    ));
                }
            }
        }

        // Check expiry
        if let Some(expires) = license.reserved_until {
            if expires < now {
                return Err(AppError::ValidationError(
                    "Reservation has expired. Please reserve again.".into()
                ));
            }
        }

        // F2: NOW safe to check idempotency - token was verified above
        // If already claimed, return success (idempotent)
        if license.claimed {
            // B3 FIX: Still update reservation as claimed (idempotent but ensures consistency)
            // This ensures retries update the reservation even if claim already succeeded
            sqlx::query(r#"
                UPDATE license_reservations
                SET status = 'claimed', claimed_at = COALESCE(claimed_at, $2)
                WHERE license_id = $1 AND session_token = $3 AND status = 'active'
            "#)
            .bind(license_id)
            .bind(now)
            .bind(session_token)
            .execute(&mut *tx)
            .await
            .ok();  // Ignore errors - reservation may already be claimed

            // B3 FIX: Also mark credential revealed on retry
            sqlx::query(r#"
                UPDATE license_reservations
                SET credential_revealed = true, credential_revealed_at = COALESCE(credential_revealed_at, $2)
                WHERE license_id = $1 AND session_token = $3
            "#)
            .bind(license_id)
            .bind(now)
            .bind(session_token)
            .execute(&mut *tx)
            .await
            .ok();  // Ignore errors - just audit tracking

            // Fetch the claimed license data - credentials can be revealed since token was valid
            let result: ClaimResultRow = sqlx::query_as::<_, ClaimResultRow>(r#"
                SELECT id, lease_code, claimed_at, referral_id, referral_attributed_at,
                       referral_agreement_version, COALESCE(referral_frozen, false) as referral_frozen
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
                referral_frozen: result.referral_frozen,
            });
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

        // R3-07: Verify agent is approved before allowing attribution
        // Suspended/rejected/terminated agents cannot receive new attributions
        if let Some(ref_id) = referral_id {
            // Only check if this is a NEW attribution (license doesn't already have referral)
            if license.referral_id.is_none() {
                let agent_check: Option<(String,)> = sqlx::query_as(
                    r#"
                    SELECT a.status::text
                    FROM referrals r
                    JOIN agents a ON r.agent_id = a.id
                    WHERE r.id = $1
                    "#
                )
                .bind(ref_id)
                .fetch_optional(&mut *tx)
                .await?;

                if let Some((status,)) = agent_check {
                    if status != "approved" {
                        tracing::warn!(
                            referral_id = %ref_id,
                            agent_status = %status,
                            license_id = %license_id,
                            "Attribution rejected: agent not approved"
                        );
                        return Err(AppError::ValidationError(format!(
                            "Cannot attribute to referral: agent status is '{}'",
                            status
                        )));
                    }
                }
            }
        }

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
        // R5-05: Set referral_frozen = true to make attribution immutable (including no-referral)
        // F2: Set issued_to = owner_id to bind license to authenticated user
        let bound_to_device = device_id.is_some();
        sqlx::query(r#"
            UPDATE licenses
            SET claimed = true,
                claimed_at = $2,
                issued_to = $3,
                bound_to_device = $4,
                device_id = $5,
                reserved_until = NULL,
                reservation_token = NULL,
                referral_id = COALESCE(referral_id, $6),
                referral_attributed_at = COALESCE(referral_attributed_at, $7),
                referral_agreement_version = COALESCE(referral_agreement_version, $8),
                referral_frozen = true,
                referral_frozen_at = COALESCE(referral_frozen_at, $2)
            WHERE id = $1 AND claimed = false
        "#)
        .bind(license_id)
        .bind(now)
        .bind(owner_id)
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
            SELECT id, lease_code, claimed_at, referral_id, referral_attributed_at,
                   referral_agreement_version, COALESCE(referral_frozen, true) as referral_frozen
            FROM licenses WHERE id = $1
        "#)
        .bind(license_id)
        .fetch_one(&mut *tx)
        .await?;

        // Mark credential as revealed in reservation record
        sqlx::query(r#"
            UPDATE license_reservations
            SET credential_revealed = true, credential_revealed_at = $2
            WHERE license_id = $1 AND session_token = $3
        "#)
        .bind(license_id)
        .bind(now)
        .bind(session_token)
        .execute(&mut *tx)
        .await
        .ok(); // Ignore errors - this is just audit tracking

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
            referral_frozen: result.referral_frozen,
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

/// R5-05: Does NOT include lease_code - credentials revealed only at confirm
#[derive(Debug, sqlx::FromRow)]
struct LicenseReserveRow {
    // Use String for id to handle both UUID and blockchain hex formats
    id: String,
    split_type: String,
}

#[derive(Debug, sqlx::FromRow)]
struct ReservationRow {
    session_token: String,
    user_id: Option<String>,
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
    /// R5-05: Whether referral attribution is frozen
    referral_frozen: bool,
}
