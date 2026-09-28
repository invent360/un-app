//! Claim repository for database operations

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::server::db::ConnectionPool;
use crate::types::AppError;

/// Dynamic type alias for ClaimRepository trait object
pub type DynClaimRepository = Arc<dyn ClaimRepository + Send + Sync>;

/// Claimed license data for sync
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ClaimedLicense {
    pub license_id: String,
    pub claimed_at: Option<DateTime<Utc>>,
    pub referral_code: Option<String>,
    pub claim_token: String,
}

/// Claim repository trait defining database operations
#[async_trait]
pub trait ClaimRepository {
    /// Set the referral_id for a claim (by license_id)
    async fn set_referral(&self, license_id: &str, referral_id: i32) -> Result<(), AppError>;

    /// Get claimed licenses since a given timestamp (for uno-admin sync)
    async fn get_claimed_since(
        &self,
        since: Option<DateTime<Utc>>,
        limit: i32,
    ) -> Result<Vec<ClaimedLicense>, AppError>;
}

/// Concrete implementation of ClaimRepository
pub struct ClaimRepositoryImpl {
    pub db_pool: ConnectionPool,
}

impl ClaimRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl ClaimRepository for ClaimRepositoryImpl {
    async fn set_referral(&self, license_id: &str, referral_id: i32) -> Result<(), AppError> {
        sqlx::query(r#"
            UPDATE licenses
            SET referral_id = $1
            WHERE id = $2::uuid
        "#)
        .bind(referral_id)
        .bind(license_id)
        .execute(&self.db_pool)
        .await?;

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
}
