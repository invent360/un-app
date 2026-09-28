//! Adapter bridging uno-app's variant repository to uno-api's VariantRepository trait.

use async_trait::async_trait;

use uno_api::error::DbError;
use uno_api::models::{Variant, VariantInput, VariantStatus, VariantSummary};
use uno_api::traits::VariantRepository;

use crate::server::db::ConnectionPool;

/// Adapter implementing uno-api's VariantRepository trait.
pub struct VariantRepositoryAdapter {
    pool: ConnectionPool,
}

impl VariantRepositoryAdapter {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VariantRepository for VariantRepositoryAdapter {
    async fn upsert(&self, input: &VariantInput) -> Result<Variant, DbError> {
        let row = sqlx::query_as::<_, VariantRow>(r#"
            INSERT INTO admin_variants (
                name, user_share, operator_share, duration_months, description, status
            ) VALUES ($1, $2, $3, $4, $5, 'active')
            ON CONFLICT (user_share, operator_share, duration_months)
            DO UPDATE SET
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                updated_at = NOW()
            RETURNING *
        "#)
        .bind(&input.name)
        .bind(input.user_share)
        .bind(input.operator_share)
        .bind(input.duration_months)
        .bind(&input.description)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(row.into())
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Variant>, DbError> {
        let row = sqlx::query_as::<_, VariantRow>(
            "SELECT * FROM admin_variants WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn get_by_shares(
        &self,
        user_share: i32,
        operator_share: i32,
        duration_months: i32,
    ) -> Result<Option<Variant>, DbError> {
        let row = sqlx::query_as::<_, VariantRow>(r#"
            SELECT * FROM admin_variants
            WHERE user_share = $1 AND operator_share = $2 AND duration_months = $3
        "#)
        .bind(user_share)
        .bind(operator_share)
        .bind(duration_months)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn list_all(&self) -> Result<Vec<Variant>, DbError> {
        let rows = sqlx::query_as::<_, VariantRow>(
            "SELECT * FROM admin_variants ORDER BY user_share DESC, duration_months ASC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_by_status(&self, status: VariantStatus) -> Result<Vec<Variant>, DbError> {
        let rows = sqlx::query_as::<_, VariantRow>(
            "SELECT * FROM admin_variants WHERE status = $1 ORDER BY user_share DESC"
        )
        .bind(status_to_str(&status))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn increment_total(&self, id: i32, count: i64) -> Result<(), DbError> {
        sqlx::query(r#"
            UPDATE admin_variants
            SET total_licenses = total_licenses + $2, updated_at = NOW()
            WHERE id = $1
        "#)
        .bind(id)
        .bind(count)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn increment_claimed(&self, id: i32) -> Result<(), DbError> {
        sqlx::query(r#"
            UPDATE admin_variants
            SET claimed_count = claimed_count + 1, updated_at = NOW()
            WHERE id = $1
        "#)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn decrement_total(&self, id: i32, count: i64) -> Result<(), DbError> {
        sqlx::query(r#"
            UPDATE admin_variants
            SET total_licenses = GREATEST(0, total_licenses - $2), updated_at = NOW()
            WHERE id = $1
        "#)
        .bind(id)
        .bind(count)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn update_status(&self, id: i32, status: VariantStatus) -> Result<(), DbError> {
        sqlx::query(
            "UPDATE admin_variants SET status = $1, updated_at = NOW() WHERE id = $2"
        )
        .bind(status_to_str(&status))
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn update(&self, variant: &Variant) -> Result<(), DbError> {
        sqlx::query(r#"
            UPDATE admin_variants SET
                name = $2, user_share = $3, operator_share = $4, duration_months = $5,
                description = $6, status = $7, total_licenses = $8, claimed_count = $9,
                updated_at = NOW()
            WHERE id = $1
        "#)
        .bind(variant.id)
        .bind(&variant.name)
        .bind(variant.user_share)
        .bind(variant.operator_share)
        .bind(variant.duration_months)
        .bind(&variant.description)
        .bind(status_to_str(&variant.status))
        .bind(variant.total_licenses)
        .bind(variant.claimed_count)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), DbError> {
        // Check if any licenses exist for this variant
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM admin_licenses WHERE variant_id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        if count.0 > 0 {
            return Err(DbError::Other(
                "Cannot delete variant with existing licenses".to_string(),
            ));
        }

        sqlx::query("DELETE FROM admin_variants WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn get_summary(&self) -> Result<VariantSummary, DbError> {
        let row = sqlx::query_as::<_, SummaryRow>(r#"
            SELECT
                COUNT(*)::int as total_variants,
                COUNT(*) FILTER (WHERE status = 'active')::int as active_variants,
                COALESCE(SUM(total_licenses), 0)::bigint as total_licenses,
                COALESCE(SUM(claimed_count), 0)::bigint as total_claimed
            FROM admin_variants
        "#)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(VariantSummary {
            total_variants: row.total_variants as usize,
            active_variants: row.active_variants as usize,
            total_licenses: row.total_licenses,
            total_claimed: row.total_claimed,
            total_remaining: row.total_licenses - row.total_claimed,
        })
    }
}

fn status_to_str(status: &VariantStatus) -> &'static str {
    match status {
        VariantStatus::Active => "active",
        VariantStatus::Paused => "paused",
        VariantStatus::Archived => "archived",
    }
}

fn str_to_status(s: &str) -> VariantStatus {
    match s {
        "active" => VariantStatus::Active,
        "paused" => VariantStatus::Paused,
        "archived" => VariantStatus::Archived,
        _ => VariantStatus::Active,
    }
}

#[derive(sqlx::FromRow)]
struct VariantRow {
    id: i32,
    name: String,
    user_share: i32,
    operator_share: i32,
    duration_months: i32,
    description: Option<String>,
    status: String,
    total_licenses: i64,
    claimed_count: i64,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<VariantRow> for Variant {
    fn from(row: VariantRow) -> Self {
        Variant {
            id: row.id,
            name: row.name,
            user_share: row.user_share,
            operator_share: row.operator_share,
            duration_months: row.duration_months,
            description: row.description,
            status: str_to_status(&row.status),
            total_licenses: row.total_licenses,
            claimed_count: row.claimed_count,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct SummaryRow {
    total_variants: i32,
    active_variants: i32,
    total_licenses: i64,
    total_claimed: i64,
}
