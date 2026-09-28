//! Variant repository for database operations

use async_trait::async_trait;
use std::sync::Arc;
use crate::server::db::ConnectionPool;
use crate::types::{LicenseVariant, AppError, PaginationParams, PaginatedResponse, VariantStatus};

/// Dynamic type alias for VariantRepository trait object
pub type DynVariantRepository = Arc<dyn VariantRepository + Send + Sync>;

/// Variant repository trait defining database operations
#[async_trait]
pub trait VariantRepository {
    /// Get all active variants ordered by display order
    async fn get_active(&self) -> Result<Vec<LicenseVariant>, AppError>;

    /// Get active variants with pagination
    async fn get_active_paginated(&self, params: &PaginationParams) -> Result<PaginatedResponse<LicenseVariant>, AppError>;

    /// Count all active variants
    async fn count_active(&self) -> Result<i64, AppError>;

    /// Get variant by ID
    async fn get_by_id(&self, id: i32) -> Result<Option<LicenseVariant>, AppError>;

    /// Get variant by share percentage
    async fn get_by_share(&self, user_share: i32, operator_share: i32) -> Result<Option<LicenseVariant>, AppError>;

    /// Create or update variant
    async fn upsert(&self, variant: &LicenseVariant) -> Result<LicenseVariant, AppError>;

    /// Increment claimed count for a variant
    async fn increment_claimed(&self, variant_id: i32) -> Result<(), AppError>;

    /// Update variant status
    async fn update_status(&self, variant_id: i32, status: VariantStatus) -> Result<(), AppError>;
}

/// Concrete implementation of VariantRepository
pub struct VariantRepositoryImpl {
    pub db_pool: ConnectionPool,
}

impl VariantRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl VariantRepository for VariantRepositoryImpl {
    async fn get_active(&self) -> Result<Vec<LicenseVariant>, AppError> {
        let variants = sqlx::query_as::<_, LicenseVariant>(r#"
            SELECT * FROM license_variants
            WHERE status = 'active'
            ORDER BY display_order ASC, user_share_percentage DESC
        "#)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(variants)
    }

    async fn get_active_paginated(&self, params: &PaginationParams) -> Result<PaginatedResponse<LicenseVariant>, AppError> {
        // Get total count first
        let total = self.count_active().await?;

        // Get paginated data
        let variants = sqlx::query_as::<_, LicenseVariant>(r#"
            SELECT * FROM license_variants
            WHERE status = 'active'
            ORDER BY display_order ASC, user_share_percentage DESC
            LIMIT $1 OFFSET $2
        "#)
        .bind(params.limit())
        .bind(params.offset())
        .fetch_all(&self.db_pool)
        .await?;

        Ok(PaginatedResponse::new(variants, params, total))
    }

    async fn count_active(&self) -> Result<i64, AppError> {
        let result: (i64,) = sqlx::query_as(r#"
            SELECT COUNT(*) FROM license_variants WHERE status = 'active'
        "#)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result.0)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<LicenseVariant>, AppError> {
        let variant = sqlx::query_as::<_, LicenseVariant>(
            "SELECT * FROM license_variants WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(variant)
    }

    async fn get_by_share(&self, user_share: i32, operator_share: i32) -> Result<Option<LicenseVariant>, AppError> {
        let variant = sqlx::query_as::<_, LicenseVariant>(r#"
            SELECT * FROM license_variants
            WHERE user_share_percentage = $1 AND operator_share_percentage = $2
        "#)
        .bind(user_share)
        .bind(operator_share)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(variant)
    }

    async fn upsert(&self, variant: &LicenseVariant) -> Result<LicenseVariant, AppError> {
        let result = sqlx::query_as::<_, LicenseVariant>(r#"
            INSERT INTO license_variants (
                user_share_percentage, operator_share_percentage, lease_duration_months,
                min_uptime_percentage, total_quantity, claimed_count,
                min_monthly_earnings, max_monthly_earnings, display_name,
                display_order, is_featured, status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (user_share_percentage, operator_share_percentage, lease_duration_months)
            DO UPDATE SET
                total_quantity = license_variants.total_quantity + EXCLUDED.total_quantity,
                min_uptime_percentage = COALESCE(EXCLUDED.min_uptime_percentage, license_variants.min_uptime_percentage),
                updated_at = NOW()
            RETURNING *
        "#)
        .bind(variant.user_share_percentage)
        .bind(variant.operator_share_percentage)
        .bind(variant.lease_duration_months)
        .bind(variant.min_uptime_percentage)
        .bind(variant.total_quantity)
        .bind(variant.claimed_count)
        .bind(variant.min_monthly_earnings)
        .bind(variant.max_monthly_earnings)
        .bind(&variant.display_name)
        .bind(variant.display_order)
        .bind(variant.is_featured)
        .bind(&variant.status)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result)
    }

    async fn increment_claimed(&self, variant_id: i32) -> Result<(), AppError> {
        sqlx::query(r#"
            UPDATE license_variants
            SET claimed_count = claimed_count + 1, updated_at = NOW()
            WHERE id = $1
        "#)
        .bind(variant_id)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    async fn update_status(&self, variant_id: i32, status: VariantStatus) -> Result<(), AppError> {
        sqlx::query(r#"
            UPDATE license_variants
            SET status = $1, updated_at = NOW()
            WHERE id = $2
        "#)
        .bind(status.as_str())
        .bind(variant_id)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}
