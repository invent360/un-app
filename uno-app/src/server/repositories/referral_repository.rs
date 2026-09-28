//! Referral repository for database operations

use async_trait::async_trait;
use std::sync::Arc;
use crate::server::db::ConnectionPool;
use crate::types::{Referral, ReferralStatus, AppError};

/// Dynamic type alias for ReferralRepository trait object
pub type DynReferralRepository = Arc<dyn ReferralRepository + Send + Sync>;

/// Input for upserting a referral from external system
#[derive(Debug, Clone)]
pub struct ReferralInput {
    pub username: String,
    pub email: String,
    pub country_code: String,
    pub referral_code: String,
    pub status: String,
}

/// Referral repository trait defining database operations
#[async_trait]
pub trait ReferralRepository {
    /// Get a referral by its unique code
    async fn get_by_code(&self, code: &str) -> Result<Option<Referral>, AppError>;

    /// Get an active referral by code (for validation during claims)
    async fn get_active_by_code(&self, code: &str) -> Result<Option<Referral>, AppError>;

    /// Get a referral by ID
    async fn get_by_id(&self, id: i32) -> Result<Option<Referral>, AppError>;

    /// Get a referral by email
    async fn get_by_email(&self, email: &str) -> Result<Option<Referral>, AppError>;

    /// Check if a referral code already exists
    async fn code_exists(&self, code: &str) -> Result<bool, AppError>;

    /// Create a new referral application
    async fn create(
        &self,
        username: &str,
        email: &str,
        country_code: &str,
        referral_code: &str,
    ) -> Result<Referral, AppError>;

    /// List all referrals (for admin sync)
    async fn list_all(&self, limit: i32) -> Result<Vec<Referral>, AppError>;

    /// Upsert a referral from external system (by referral_code)
    async fn upsert(&self, input: ReferralInput) -> Result<Referral, AppError>;
}

/// Concrete implementation of ReferralRepository
pub struct ReferralRepositoryImpl {
    db_pool: ConnectionPool,
}

impl ReferralRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl ReferralRepository for ReferralRepositoryImpl {
    async fn get_by_code(&self, code: &str) -> Result<Option<Referral>, AppError> {
        let referral = sqlx::query_as::<_, Referral>(
            r#"
            SELECT id, username, email, country_code, referral_code, status,
                   joined_at, created_at, updated_at
            FROM referrals
            WHERE referral_code = $1
            "#
        )
        .bind(code)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(referral)
    }

    async fn get_active_by_code(&self, code: &str) -> Result<Option<Referral>, AppError> {
        let referral = sqlx::query_as::<_, Referral>(
            r#"
            SELECT id, username, email, country_code, referral_code, status,
                   joined_at, created_at, updated_at
            FROM referrals
            WHERE referral_code = $1 AND status = 'active'
            "#
        )
        .bind(code)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(referral)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Referral>, AppError> {
        let referral = sqlx::query_as::<_, Referral>(
            r#"
            SELECT id, username, email, country_code, referral_code, status,
                   joined_at, created_at, updated_at
            FROM referrals
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(referral)
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<Referral>, AppError> {
        let referral = sqlx::query_as::<_, Referral>(
            r#"
            SELECT id, username, email, country_code, referral_code, status,
                   joined_at, created_at, updated_at
            FROM referrals
            WHERE email = $1
            "#
        )
        .bind(email)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(referral)
    }

    async fn code_exists(&self, code: &str) -> Result<bool, AppError> {
        let result: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(SELECT 1 FROM referrals WHERE referral_code = $1)
            "#
        )
        .bind(code)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result.0)
    }

    async fn create(
        &self,
        username: &str,
        email: &str,
        country_code: &str,
        referral_code: &str,
    ) -> Result<Referral, AppError> {
        let referral = sqlx::query_as::<_, Referral>(
            r#"
            INSERT INTO referrals (username, email, country_code, referral_code, status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, username, email, country_code, referral_code, status,
                      joined_at, created_at, updated_at
            "#
        )
        .bind(username)
        .bind(email)
        .bind(country_code)
        .bind(referral_code)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(referral)
    }

    async fn list_all(&self, limit: i32) -> Result<Vec<Referral>, AppError> {
        let referrals = sqlx::query_as::<_, Referral>(
            r#"
            SELECT id, username, email, country_code, referral_code, status,
                   joined_at, created_at, updated_at
            FROM referrals
            ORDER BY created_at DESC
            LIMIT $1
            "#
        )
        .bind(limit)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(referrals)
    }

    async fn upsert(&self, input: ReferralInput) -> Result<Referral, AppError> {
        // Convert status string to ReferralStatus enum for the query
        let status_value: ReferralStatus = match input.status.as_str() {
            "active" => ReferralStatus::Active,
            "suspended" => ReferralStatus::Suspended,
            "rejected" => ReferralStatus::Rejected,
            _ => ReferralStatus::Pending,
        };

        let referral = sqlx::query_as::<_, Referral>(
            r#"
            INSERT INTO referrals (username, email, country_code, referral_code, status)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (referral_code) DO UPDATE SET
                username = EXCLUDED.username,
                email = EXCLUDED.email,
                country_code = EXCLUDED.country_code,
                status = EXCLUDED.status,
                updated_at = NOW()
            RETURNING id, username, email, country_code, referral_code, status,
                      joined_at, created_at, updated_at
            "#
        )
        .bind(&input.username)
        .bind(&input.email)
        .bind(&input.country_code)
        .bind(&input.referral_code)
        .bind(status_value)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(referral)
    }
}
