//! Adapter bridging uno-app's license repository to uno-api's LicenseRepository trait.

use async_trait::async_trait;
use uuid::Uuid;

use uno_api::error::DbError;
use uno_api::models::{License, LicenseStatus, Paginated, PaginationParams};
use uno_api::traits::{LicenseFilters, LicenseRepository};

use crate::server::db::ConnectionPool;

/// Adapter implementing uno-api's LicenseRepository trait.
pub struct LicenseRepositoryAdapter {
    pool: ConnectionPool,
}

impl LicenseRepositoryAdapter {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LicenseRepository for LicenseRepositoryAdapter {
    async fn insert(&self, license: &License) -> Result<(), DbError> {
        sqlx::query(r#"
            INSERT INTO admin_licenses (
                id, license_key, variant_id, user_share, operator_share,
                duration_months, status, claimed_by, claimed_at, expires_at,
                created_at, updated_at, batch_id, notes
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        "#)
        .bind(license.id)
        .bind(&license.license_key)
        .bind(license.variant_id)
        .bind(license.user_share)
        .bind(license.operator_share)
        .bind(license.duration_months)
        .bind(status_to_str(&license.status))
        .bind(license.claimed_by)
        .bind(license.claimed_at)
        .bind(license.expires_at)
        .bind(license.created_at)
        .bind(license.updated_at)
        .bind(&license.batch_id)
        .bind(&license.notes)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn insert_batch(&self, licenses: &[License]) -> Result<usize, DbError> {
        // Use a transaction for batch insert
        let mut tx = self.pool.begin().await
            .map_err(|e| DbError::TransactionError(e.to_string()))?;

        let mut count = 0;
        for license in licenses {
            let result = sqlx::query(r#"
                INSERT INTO admin_licenses (
                    id, license_key, variant_id, user_share, operator_share,
                    duration_months, status, claimed_by, claimed_at, expires_at,
                    created_at, updated_at, batch_id, notes
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#)
            .bind(license.id)
            .bind(&license.license_key)
            .bind(license.variant_id)
            .bind(license.user_share)
            .bind(license.operator_share)
            .bind(license.duration_months)
            .bind(status_to_str(&license.status))
            .bind(license.claimed_by)
            .bind(license.claimed_at)
            .bind(license.expires_at)
            .bind(license.created_at)
            .bind(license.updated_at)
            .bind(&license.batch_id)
            .bind(&license.notes)
            .execute(&mut *tx)
            .await;

            if result.is_ok() {
                count += 1;
            }
        }

        tx.commit().await
            .map_err(|e| DbError::TransactionError(e.to_string()))?;

        Ok(count)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<License>, DbError> {
        let row = sqlx::query_as::<_, LicenseRow>(
            "SELECT * FROM admin_licenses WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn get_by_key(&self, key: &str) -> Result<Option<License>, DbError> {
        let row = sqlx::query_as::<_, LicenseRow>(
            "SELECT * FROM admin_licenses WHERE license_key = $1"
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn get_first_unclaimed(&self, variant_id: i32) -> Result<Option<License>, DbError> {
        let row = sqlx::query_as::<_, LicenseRow>(r#"
            SELECT * FROM admin_licenses
            WHERE variant_id = $1 AND status = 'active' AND claimed_by IS NULL
            ORDER BY created_at ASC
            LIMIT 1
            FOR UPDATE SKIP LOCKED
        "#)
        .bind(variant_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn get_by_variant(
        &self,
        variant_id: i32,
        pagination: &PaginationParams,
    ) -> Result<Paginated<License>, DbError> {
        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM admin_licenses WHERE variant_id = $1"
        )
        .bind(variant_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        let rows = sqlx::query_as::<_, LicenseRow>(r#"
            SELECT * FROM admin_licenses
            WHERE variant_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        "#)
        .bind(variant_id)
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(Paginated::new(
            rows.into_iter().map(Into::into).collect(),
            total.0,
            pagination,
        ))
    }

    async fn get_by_status(
        &self,
        status: LicenseStatus,
        pagination: &PaginationParams,
    ) -> Result<Paginated<License>, DbError> {
        let status_str = status_to_str(&status);

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM admin_licenses WHERE status = $1"
        )
        .bind(status_str)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        let rows = sqlx::query_as::<_, LicenseRow>(r#"
            SELECT * FROM admin_licenses
            WHERE status = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        "#)
        .bind(status_str)
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(Paginated::new(
            rows.into_iter().map(Into::into).collect(),
            total.0,
            pagination,
        ))
    }

    async fn get_by_batch(&self, batch_id: &str) -> Result<Vec<License>, DbError> {
        let rows = sqlx::query_as::<_, LicenseRow>(
            "SELECT * FROM admin_licenses WHERE batch_id = $1 ORDER BY created_at ASC"
        )
        .bind(batch_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn count_by_variant(&self, variant_id: i32) -> Result<i64, DbError> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM admin_licenses WHERE variant_id = $1"
        )
        .bind(variant_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(result.0)
    }

    async fn count_unclaimed_by_variant(&self, variant_id: i32) -> Result<i64, DbError> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM admin_licenses WHERE variant_id = $1 AND status = 'active' AND claimed_by IS NULL"
        )
        .bind(variant_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(result.0)
    }

    async fn update(&self, license: &License) -> Result<(), DbError> {
        sqlx::query(r#"
            UPDATE admin_licenses SET
                license_key = $2, variant_id = $3, user_share = $4, operator_share = $5,
                duration_months = $6, status = $7, claimed_by = $8, claimed_at = $9,
                expires_at = $10, updated_at = NOW(), batch_id = $11, notes = $12
            WHERE id = $1
        "#)
        .bind(license.id)
        .bind(&license.license_key)
        .bind(license.variant_id)
        .bind(license.user_share)
        .bind(license.operator_share)
        .bind(license.duration_months)
        .bind(status_to_str(&license.status))
        .bind(license.claimed_by)
        .bind(license.claimed_at)
        .bind(license.expires_at)
        .bind(&license.batch_id)
        .bind(&license.notes)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn update_status(&self, id: Uuid, status: LicenseStatus) -> Result<(), DbError> {
        let result = sqlx::query(
            "UPDATE admin_licenses SET status = $1, updated_at = NOW() WHERE id = $2"
        )
        .bind(status_to_str(&status))
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!("License {} not found", id)));
        }

        Ok(())
    }

    async fn mark_claimed(
        &self,
        id: Uuid,
        claimed_by: Uuid,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), DbError> {
        sqlx::query(r#"
            UPDATE admin_licenses SET
                status = 'claimed', claimed_by = $2, claimed_at = NOW(), expires_at = $3, updated_at = NOW()
            WHERE id = $1
        "#)
        .bind(id)
        .bind(claimed_by)
        .bind(expires_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DbError> {
        let result = sqlx::query("DELETE FROM admin_licenses WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!("License {} not found", id)));
        }

        Ok(())
    }

    async fn delete_batch(&self, ids: &[Uuid]) -> Result<usize, DbError> {
        let result = sqlx::query("DELETE FROM admin_licenses WHERE id = ANY($1)")
            .bind(ids)
            .execute(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(result.rows_affected() as usize)
    }

    async fn key_exists(&self, key: &str) -> Result<bool, DbError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM admin_licenses WHERE license_key = $1)"
        )
        .bind(key)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(result.0)
    }

    async fn search(
        &self,
        filters: &LicenseFilters,
        pagination: &PaginationParams,
    ) -> Result<Paginated<License>, DbError> {
        // Build dynamic query
        let mut conditions = vec!["1=1".to_string()];
        let mut params: Vec<String> = Vec::new();

        if let Some(variant_id) = filters.variant_id {
            conditions.push(format!("variant_id = ${}", params.len() + 1));
            params.push(variant_id.to_string());
        }

        if let Some(status) = &filters.status {
            conditions.push(format!("status = ${}", params.len() + 1));
            params.push(status_to_str(status).to_string());
        }

        if let Some(batch_id) = &filters.batch_id {
            conditions.push(format!("batch_id = ${}", params.len() + 1));
            params.push(batch_id.clone());
        }

        let where_clause = conditions.join(" AND ");

        // For simplicity, use a basic query without dynamic binding
        // In production, you'd use a query builder
        let count_query = format!(
            "SELECT COUNT(*) FROM admin_licenses WHERE {}",
            where_clause
        );
        let select_query = format!(
            "SELECT * FROM admin_licenses WHERE {} ORDER BY created_at DESC LIMIT {} OFFSET {}",
            where_clause,
            pagination.limit(),
            pagination.offset()
        );

        // Execute simpler query for now
        let total: (i64,) = sqlx::query_as(&count_query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        let rows = sqlx::query_as::<_, LicenseRow>(&select_query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(Paginated::new(
            rows.into_iter().map(Into::into).collect(),
            total.0,
            pagination,
        ))
    }
}

// Helper to convert status enum to string
fn status_to_str(status: &LicenseStatus) -> &'static str {
    match status {
        LicenseStatus::Active => "active",
        LicenseStatus::Claimed => "claimed",
        LicenseStatus::Revoked => "revoked",
        LicenseStatus::Expired => "expired",
    }
}

fn str_to_status(s: &str) -> LicenseStatus {
    match s {
        "active" => LicenseStatus::Active,
        "claimed" => LicenseStatus::Claimed,
        "revoked" => LicenseStatus::Revoked,
        "expired" => LicenseStatus::Expired,
        _ => LicenseStatus::Active,
    }
}

/// Database row for license
#[derive(sqlx::FromRow)]
struct LicenseRow {
    id: Uuid,
    license_key: String,
    variant_id: i32,
    user_share: i32,
    operator_share: i32,
    duration_months: i32,
    status: String,
    claimed_by: Option<Uuid>,
    claimed_at: Option<chrono::DateTime<chrono::Utc>>,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    batch_id: Option<String>,
    notes: Option<String>,
}

impl From<LicenseRow> for License {
    fn from(row: LicenseRow) -> Self {
        License {
            id: row.id,
            license_key: row.license_key,
            variant_id: row.variant_id,
            user_share: row.user_share,
            operator_share: row.operator_share,
            duration_months: row.duration_months,
            status: str_to_status(&row.status),
            claimed_by: row.claimed_by,
            claimed_at: row.claimed_at,
            expires_at: row.expires_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
            batch_id: row.batch_id,
            notes: row.notes,
        }
    }
}
