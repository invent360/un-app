//! PostgreSQL implementation of LicenseAnalyticsRepositoryTrait

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::models::entity::{LicenseAnalyticsEntity, NewLicenseAnalytics};
use crate::repository::traits::LicenseAnalyticsRepositoryTrait;

/// Row type for sqlx queries
#[derive(sqlx::FromRow)]
struct AnalyticsRow {
    license_id: String,
    date: String,
    uptime: f64,
    required_uptime: f64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<AnalyticsRow> for LicenseAnalyticsEntity {
    fn from(row: AnalyticsRow) -> Self {
        LicenseAnalyticsEntity {
            license_id: row.license_id,
            date: row.date,
            uptime: row.uptime,
            required_uptime: row.required_uptime,
            created_at: row.created_at.to_rfc3339(),
            updated_at: row.updated_at.to_rfc3339(),
        }
    }
}

/// PostgreSQL implementation of the License Analytics repository
pub struct PgLicenseAnalyticsRepository {
    pool: Arc<PgPool>,
}

impl PgLicenseAnalyticsRepository {
    /// Create a new PgLicenseAnalyticsRepository with the given connection pool
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LicenseAnalyticsRepositoryTrait for PgLicenseAnalyticsRepository {
    async fn upsert_analytics(&self, analytics: NewLicenseAnalytics) -> Result<LicenseAnalyticsEntity, String> {
        let now = Utc::now();

        let row = sqlx::query_as::<_, AnalyticsRow>(
            r#"
            INSERT INTO license_analytics (license_id, date, uptime, required_uptime, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $5)
            ON CONFLICT (license_id, date)
            DO UPDATE SET
                uptime = EXCLUDED.uptime,
                required_uptime = EXCLUDED.required_uptime,
                updated_at = EXCLUDED.updated_at
            RETURNING license_id, date, uptime, required_uptime, created_at, updated_at
            "#,
        )
        .bind(&analytics.license_id)
        .bind(&analytics.date)
        .bind(analytics.uptime)
        .bind(analytics.required_uptime)
        .bind(now)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to upsert analytics: {}", e);
            e.to_string()
        })?;

        debug!(
            "Upserted analytics for license {} on {}",
            analytics.license_id, analytics.date
        );

        Ok(row.into())
    }

    async fn upsert_analytics_batch(&self, analytics: Vec<NewLicenseAnalytics>) -> Result<usize, String> {
        if analytics.is_empty() {
            return Ok(0);
        }

        let now = Utc::now();
        let mut count = 0;

        // Use a transaction for batch operations
        let mut tx = self.pool.begin().await.map_err(|e| {
            error!("Failed to begin transaction: {}", e);
            e.to_string()
        })?;

        for item in analytics {
            let result = sqlx::query(
                r#"
                INSERT INTO license_analytics (license_id, date, uptime, required_uptime, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $5)
                ON CONFLICT (license_id, date)
                DO UPDATE SET
                    uptime = EXCLUDED.uptime,
                    required_uptime = EXCLUDED.required_uptime,
                    updated_at = EXCLUDED.updated_at
                "#,
            )
            .bind(&item.license_id)
            .bind(&item.date)
            .bind(item.uptime)
            .bind(item.required_uptime)
            .bind(now)
            .execute(&mut *tx)
            .await;

            match result {
                Ok(_) => count += 1,
                Err(e) => {
                    error!(
                        "Failed to upsert analytics for license {} on {}: {}",
                        item.license_id, item.date, e
                    );
                }
            }
        }

        tx.commit().await.map_err(|e| {
            error!("Failed to commit transaction: {}", e);
            e.to_string()
        })?;

        info!("Upserted {} analytics records", count);
        Ok(count)
    }

    async fn get_analytics_by_license(
        &self,
        license_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<LicenseAnalyticsEntity>, String> {
        let rows = sqlx::query_as::<_, AnalyticsRow>(
            r#"
            SELECT license_id, date, uptime, required_uptime, created_at, updated_at
            FROM license_analytics
            WHERE license_id = $1 AND date >= $2 AND date <= $3
            ORDER BY date DESC
            "#,
        )
        .bind(license_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to get analytics by license: {}", e);
            e.to_string()
        })?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn get_latest_analytics(&self, license_id: &str) -> Result<Option<LicenseAnalyticsEntity>, String> {
        let row = sqlx::query_as::<_, AnalyticsRow>(
            r#"
            SELECT license_id, date, uptime, required_uptime, created_at, updated_at
            FROM license_analytics
            WHERE license_id = $1
            ORDER BY date DESC
            LIMIT 1
            "#,
        )
        .bind(license_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to get latest analytics: {}", e);
            e.to_string()
        })?;

        Ok(row.map(Into::into))
    }

    async fn get_analytics_by_date(&self, date: &str) -> Result<Vec<LicenseAnalyticsEntity>, String> {
        let rows = sqlx::query_as::<_, AnalyticsRow>(
            r#"
            SELECT license_id, date, uptime, required_uptime, created_at, updated_at
            FROM license_analytics
            WHERE date = $1
            ORDER BY license_id
            "#,
        )
        .bind(date)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to get analytics by date: {}", e);
            e.to_string()
        })?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn delete_analytics_by_license(&self, license_id: &str) -> Result<usize, String> {
        let result = sqlx::query(
            r#"
            DELETE FROM license_analytics
            WHERE license_id = $1
            "#,
        )
        .bind(license_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to delete analytics by license: {}", e);
            e.to_string()
        })?;

        let count = result.rows_affected() as usize;
        info!("Deleted {} analytics records for license {}", count, license_id);
        Ok(count)
    }
}
