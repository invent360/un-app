//! PostgreSQL implementation of RewardRepositoryTrait

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

use crate::models::entity::{
    ListRewardsByDateParams, ListRewardsByLicenseAndDateParams, ListRewardsByLicenseParams,
    NewReward, PaginatedResult, PaginationParams, RewardEntity,
};
use crate::repository::traits::RewardRepositoryTrait;

/// Internal row structure for sqlx mapping
#[derive(Debug, FromRow)]
struct RewardRow {
    id: String,
    user_id: String,
    reward_type: String,
    description: Option<String>,
    node_id: String,
    license_id: String,
    license_lease_id: Option<String>,  // Nullable in schema
    task_key: Option<String>,
    task_metadata: Option<String>,
    completed_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    amount_micros: i64,
}

impl From<RewardRow> for RewardEntity {
    fn from(row: RewardRow) -> Self {
        RewardEntity {
            id: row.id,
            user_id: row.user_id,
            reward_type: row.reward_type,
            description: row.description,
            node_id: row.node_id,
            license_id: row.license_id,
            license_lease_id: row.license_lease_id.unwrap_or_default(),  // Convert NULL to empty string
            task_key: row.task_key,
            task_metadata: row.task_metadata,
            completed_at: row.completed_at.to_rfc3339(),
            created_at: row.created_at.to_rfc3339(),
            amount_micros: row.amount_micros,
        }
    }
}

/// PostgreSQL implementation of the Reward repository
pub struct PgRewardRepository {
    pool: Arc<PgPool>,
}

impl PgRewardRepository {
    /// Create a new PgRewardRepository with the given connection pool
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Parse a date string to DateTime<Utc>
    fn parse_datetime(s: &str) -> Result<DateTime<Utc>, String> {
        // Try RFC3339 format first
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return Ok(dt.with_timezone(&Utc));
        }

        // Try date-only format (YYYY-MM-DD) and set to start of day
        if let Ok(date) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            let datetime = date.and_hms_opt(0, 0, 0).unwrap();
            return Ok(DateTime::from_naive_utc_and_offset(datetime, Utc));
        }

        Err(format!("Failed to parse datetime: {}", s))
    }

    /// Parse a date string to DateTime<Utc>, setting to end of day for range queries
    fn parse_datetime_end_of_day(s: &str) -> Result<DateTime<Utc>, String> {
        // Try RFC3339 format first
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return Ok(dt.with_timezone(&Utc));
        }

        // Try date-only format (YYYY-MM-DD) and set to end of day
        if let Ok(date) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            let datetime = date.and_hms_opt(23, 59, 59).unwrap();
            return Ok(DateTime::from_naive_utc_and_offset(datetime, Utc));
        }

        Err(format!("Failed to parse datetime: {}", s))
    }
}

#[async_trait]
impl RewardRepositoryTrait for PgRewardRepository {
    async fn save_reward(&self, new_reward: NewReward) -> Result<RewardEntity, String> {
        let now = Utc::now();
        let id = new_reward.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
        let completed_at = Self::parse_datetime(&new_reward.completed_at)?;

        let row = sqlx::query_as::<_, RewardRow>(
            r#"
            INSERT INTO rewards (
                id, user_id, reward_type, description, node_id, license_id,
                license_lease_id, task_key, task_metadata, completed_at,
                created_at, amount_micros
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id, user_id, reward_type, description, node_id, license_id,
                      license_lease_id, task_key, task_metadata, completed_at,
                      created_at, amount_micros
            "#,
        )
        .bind(&id)
        .bind(&new_reward.user_id)
        .bind(&new_reward.reward_type)
        .bind(&new_reward.description)
        .bind(&new_reward.node_id)
        .bind(&new_reward.license_id)
        .bind(&new_reward.license_lease_id)
        .bind(&new_reward.task_key)
        .bind(&new_reward.task_metadata)
        .bind(completed_at)
        .bind(now)
        .bind(new_reward.amount_micros)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| {
            error!("Failed to save reward: {}", e);
            e.to_string()
        })?;

        debug!("Saved reward: {}", id);
        Ok(row.into())
    }

    async fn save_rewards(&self, rewards: Vec<NewReward>) -> Result<usize, String> {
        let mut count = 0;

        for reward in rewards {
            match self.save_reward(reward).await {
                Ok(_) => count += 1,
                Err(e) => {
                    error!("Failed to save reward in bulk: {}", e);
                }
            }
        }

        Ok(count)
    }

    async fn get_reward_by_id(&self, id: &str) -> Result<Option<RewardEntity>, String> {
        let row = sqlx::query_as::<_, RewardRow>(
            r#"
            SELECT id, user_id, reward_type, description, node_id, license_id,
                   license_lease_id, task_key, task_metadata, completed_at,
                   created_at, amount_micros
            FROM rewards
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(Into::into))
    }

    async fn list_rewards(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatedResult<RewardEntity>, String> {
        let total = self.count_rewards().await?;

        let offset = params.offset() as i64;
        let limit = params.limit as i64;

        let rows = sqlx::query_as::<_, RewardRow>(
            r#"
            SELECT id, user_id, reward_type, description, node_id, license_id,
                   license_lease_id, task_key, task_metadata, completed_at,
                   created_at, amount_micros
            FROM rewards
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        let items: Vec<RewardEntity> = rows.into_iter().map(Into::into).collect();

        Ok(PaginatedResult::new(items, total, params.page, params.limit))
    }

    async fn list_rewards_by_license_id(
        &self,
        params: ListRewardsByLicenseParams,
    ) -> Result<PaginatedResult<RewardEntity>, String> {
        let total = self.count_rewards_by_license_id(&params.license_id).await?;

        let offset = params.pagination.offset() as i64;
        let limit = params.pagination.limit as i64;

        let rows = sqlx::query_as::<_, RewardRow>(
            r#"
            SELECT id, user_id, reward_type, description, node_id, license_id,
                   license_lease_id, task_key, task_metadata, completed_at,
                   created_at, amount_micros
            FROM rewards
            WHERE license_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&params.license_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        let items: Vec<RewardEntity> = rows.into_iter().map(Into::into).collect();

        Ok(PaginatedResult::new(
            items,
            total,
            params.pagination.page,
            params.pagination.limit,
        ))
    }

    async fn list_rewards_by_date(
        &self,
        params: ListRewardsByDateParams,
    ) -> Result<PaginatedResult<RewardEntity>, String> {
        let start_dt = Self::parse_datetime(&params.start_date)?;
        let end_dt = Self::parse_datetime_end_of_day(&params.end_date)?;

        // Get count for this date range
        let count_row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM rewards
            WHERE created_at >= $1 AND created_at <= $2
            "#,
        )
        .bind(start_dt)
        .bind(end_dt)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        let total = count_row.0;
        let offset = params.pagination.offset() as i64;
        let limit = params.pagination.limit as i64;

        let rows = sqlx::query_as::<_, RewardRow>(
            r#"
            SELECT id, user_id, reward_type, description, node_id, license_id,
                   license_lease_id, task_key, task_metadata, completed_at,
                   created_at, amount_micros
            FROM rewards
            WHERE created_at >= $1 AND created_at <= $2
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(start_dt)
        .bind(end_dt)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        let items: Vec<RewardEntity> = rows.into_iter().map(Into::into).collect();

        Ok(PaginatedResult::new(
            items,
            total,
            params.pagination.page,
            params.pagination.limit,
        ))
    }

    async fn list_rewards_by_license_id_and_date(
        &self,
        params: ListRewardsByLicenseAndDateParams,
    ) -> Result<PaginatedResult<RewardEntity>, String> {
        let start_dt = Self::parse_datetime(&params.start_date)?;
        let end_dt = Self::parse_datetime_end_of_day(&params.end_date)?;

        // Get count for this license + date range
        let count_row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM rewards
            WHERE license_id = $1 AND created_at >= $2 AND created_at <= $3
            "#,
        )
        .bind(&params.license_id)
        .bind(start_dt)
        .bind(end_dt)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        let total = count_row.0;
        let offset = params.pagination.offset() as i64;
        let limit = params.pagination.limit as i64;

        let rows = sqlx::query_as::<_, RewardRow>(
            r#"
            SELECT id, user_id, reward_type, description, node_id, license_id,
                   license_lease_id, task_key, task_metadata, completed_at,
                   created_at, amount_micros
            FROM rewards
            WHERE license_id = $1 AND created_at >= $2 AND created_at <= $3
            ORDER BY created_at DESC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(&params.license_id)
        .bind(start_dt)
        .bind(end_dt)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        let items: Vec<RewardEntity> = rows.into_iter().map(Into::into).collect();

        Ok(PaginatedResult::new(
            items,
            total,
            params.pagination.page,
            params.pagination.limit,
        ))
    }

    async fn count_rewards(&self) -> Result<i64, String> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM rewards")
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| e.to_string())?;

        Ok(row.0)
    }

    async fn count_rewards_by_license_id(&self, license_id: &str) -> Result<i64, String> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM rewards WHERE license_id = $1")
            .bind(license_id)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| e.to_string())?;

        Ok(row.0)
    }

    async fn get_total_earnings_by_license_id(&self, license_id: &str) -> Result<i64, String> {
        let row: (Option<i64>,) = sqlx::query_as(
            "SELECT COALESCE(SUM(amount_micros), 0) FROM rewards WHERE license_id = $1",
        )
        .bind(license_id)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.0.unwrap_or(0))
    }

    async fn delete_reward(&self, id: &str) -> Result<bool, String> {
        let result = sqlx::query("DELETE FROM rewards WHERE id = $1")
            .bind(id)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_most_recent_reward(&self) -> Result<Option<RewardEntity>, String> {
        let row = sqlx::query_as::<_, RewardRow>(
            r#"
            SELECT id, user_id, reward_type, description, node_id, license_id,
                   license_lease_id, task_key, task_metadata, completed_at,
                   created_at, amount_micros
            FROM rewards
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(Into::into))
    }

    async fn reward_exists(&self, id: &str) -> Result<bool, String> {
        let row: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM rewards WHERE id = $1)")
            .bind(id)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| e.to_string())?;

        Ok(row.0)
    }

    async fn upsert_rewards(&self, rewards: Vec<NewReward>) -> Result<usize, String> {
        let mut inserted_count = 0;

        for reward in rewards {
            let id = reward.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
            let completed_at = Self::parse_datetime(&reward.completed_at)?;
            let now = Utc::now();

            // Use ON CONFLICT DO NOTHING to skip existing records
            let result = sqlx::query(
                r#"
                INSERT INTO rewards (
                    id, user_id, reward_type, description, node_id, license_id,
                    license_lease_id, task_key, task_metadata, completed_at,
                    created_at, amount_micros
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                ON CONFLICT (id) DO NOTHING
                "#,
            )
            .bind(&id)
            .bind(&reward.user_id)
            .bind(&reward.reward_type)
            .bind(&reward.description)
            .bind(&reward.node_id)
            .bind(&reward.license_id)
            .bind(&reward.license_lease_id)
            .bind(&reward.task_key)
            .bind(&reward.task_metadata)
            .bind(completed_at)
            .bind(now)
            .bind(reward.amount_micros)
            .execute(self.pool.as_ref())
            .await;

            match result {
                Ok(r) => {
                    if r.rows_affected() > 0 {
                        inserted_count += 1;
                        debug!("Inserted reward: {}", id);
                    } else {
                        debug!("Reward {} already exists, skipped", id);
                    }
                }
                Err(e) => {
                    error!("Failed to upsert reward {}: {}", id, e);
                }
            }
        }

        Ok(inserted_count)
    }

    async fn get_daily_totals(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<(String, i64, i32)>, String> {
        let start_dt = Self::parse_datetime(start_date)?;
        let end_dt = Self::parse_datetime_end_of_day(end_date)?;

        // Use PostgreSQL date functions to aggregate by date
        // Only count licenses with positive earnings
        let rows: Vec<(String, i64, i64)> = sqlx::query_as(
            r#"
            SELECT
                DATE(completed_at)::text as date,
                COALESCE(SUM(amount_micros), 0) as total_amount,
                COUNT(DISTINCT CASE WHEN amount_micros > 0 THEN license_id END) as unique_licenses
            FROM rewards
            WHERE completed_at >= $1 AND completed_at <= $2
            GROUP BY DATE(completed_at)
            ORDER BY DATE(completed_at) ASC
            "#,
        )
        .bind(start_dt)
        .bind(end_dt)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

        // Convert to expected return type
        let result: Vec<(String, i64, i32)> = rows
            .into_iter()
            .map(|(date, amount, licenses)| (date, amount, licenses as i32))
            .collect();

        Ok(result)
    }
}
