//! ScyllaDB implementation of RewardRepositoryTrait

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use scylla::frame::value::CqlTimestamp;
use scylla::Session;
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::entity::{
    ListRewardsByDateParams, ListRewardsByLicenseAndDateParams, ListRewardsByLicenseParams,
    NewReward, PaginatedResult, PaginationParams, RewardEntity,
};
use crate::repository::traits::RewardRepositoryTrait;

type RewardRow = (
    String,         // id
    String,         // user_id
    String,         // type
    Option<String>, // description
    String,         // node_id
    String,         // license_id
    String,         // license_lease_id
    Option<String>, // task_key
    Option<String>, // task_metadata
    CqlTimestamp,   // completed_at
    CqlTimestamp,   // created_at
    i64,            // amount_micros
);

/// ScyllaDB implementation of the Reward repository
pub struct RewardRepository {
    session: Arc<Session>,
}

impl RewardRepository {
    /// Create a new RewardRepository with the given session
    pub fn new(session: DbPool) -> Self {
        Self { session }
    }

    fn timestamp_to_string(ts: CqlTimestamp) -> String {
        DateTime::from_timestamp_millis(ts.0)
            .map(|d| d.to_rfc3339())
            .unwrap_or_else(|| Utc::now().to_rfc3339())
    }

    fn string_to_timestamp(s: &str) -> CqlTimestamp {
        DateTime::parse_from_rfc3339(s)
            .map(|d| CqlTimestamp(d.timestamp_millis()))
            .unwrap_or_else(|_| CqlTimestamp(Utc::now().timestamp_millis()))
    }

    fn parse_single_reward(result: scylla::QueryResult) -> Result<Option<RewardEntity>, String> {
        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        let rows = match rows_result.rows::<RewardRow>() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        for row_result in rows {
            if let Ok((
                id,
                user_id,
                reward_type,
                description,
                node_id,
                license_id,
                license_lease_id,
                task_key,
                task_metadata,
                completed_at,
                created_at,
                amount_micros,
            )) = row_result
            {
                return Ok(Some(RewardEntity {
                    id,
                    user_id,
                    reward_type,
                    description,
                    node_id,
                    license_id,
                    license_lease_id,
                    task_key,
                    task_metadata,
                    completed_at: Self::timestamp_to_string(completed_at),
                    created_at: Self::timestamp_to_string(created_at),
                    amount_micros,
                }));
            }
        }

        Ok(None)
    }

    fn parse_rewards(result: scylla::QueryResult) -> Result<Vec<RewardEntity>, String> {
        let mut rewards = Vec::new();

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(rewards),
        };

        let rows = match rows_result.rows::<RewardRow>() {
            Ok(r) => r,
            Err(_) => return Ok(rewards),
        };

        for row_result in rows {
            if let Ok((
                id,
                user_id,
                reward_type,
                description,
                node_id,
                license_id,
                license_lease_id,
                task_key,
                task_metadata,
                completed_at,
                created_at,
                amount_micros,
            )) = row_result
            {
                rewards.push(RewardEntity {
                    id,
                    user_id,
                    reward_type,
                    description,
                    node_id,
                    license_id,
                    license_lease_id,
                    task_key,
                    task_metadata,
                    completed_at: Self::timestamp_to_string(completed_at),
                    created_at: Self::timestamp_to_string(created_at),
                    amount_micros,
                });
            }
        }

        Ok(rewards)
    }

    fn parse_count(result: scylla::QueryResult) -> Result<i64, String> {
        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        let rows = match rows_result.rows::<(i64,)>() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        for row_result in rows {
            if let Ok((count,)) = row_result {
                return Ok(count);
            }
        }

        Ok(0)
    }

    fn parse_sum(result: scylla::QueryResult) -> Result<i64, String> {
        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        // SUM returns bigint which may be null if no rows
        let rows = match rows_result.rows::<(Option<i64>,)>() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        for row_result in rows {
            if let Ok((sum,)) = row_result {
                return Ok(sum.unwrap_or(0));
            }
        }

        Ok(0)
    }

    /// Apply pagination to a list of rewards (in-memory pagination for ScyllaDB)
    fn paginate(mut rewards: Vec<RewardEntity>, params: &PaginationParams) -> Vec<RewardEntity> {
        // Sort by created_at descending (most recent first)
        rewards.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let offset = params.offset() as usize;
        let limit = params.limit as usize;

        if offset >= rewards.len() {
            return Vec::new();
        }

        rewards.into_iter().skip(offset).take(limit).collect()
    }
}

#[async_trait]
impl RewardRepositoryTrait for RewardRepository {
    async fn save_reward(&self, new_reward: NewReward) -> Result<RewardEntity, String> {
        let now = Utc::now();
        let now_ts = CqlTimestamp(now.timestamp_millis());
        // Use provided ID or generate new UUID
        let id = new_reward.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());

        let completed_at_ts = Self::string_to_timestamp(&new_reward.completed_at);

        let query = "INSERT INTO rewards (id, user_id, type, description, node_id, license_id, license_lease_id, task_key, task_metadata, completed_at, created_at, amount_micros) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        self.session
            .query_unpaged(
                query,
                (
                    &id,
                    &new_reward.user_id,
                    &new_reward.reward_type,
                    &new_reward.description,
                    &new_reward.node_id,
                    &new_reward.license_id,
                    &new_reward.license_lease_id,
                    &new_reward.task_key,
                    &new_reward.task_metadata,
                    completed_at_ts,
                    now_ts,
                    new_reward.amount_micros,
                ),
            )
            .await
            .map_err(|e| {
                error!("Failed to save reward: {}", e);
                e.to_string()
            })?;

        debug!("Saved reward: {}", id);

        Ok(RewardEntity {
            id,
            user_id: new_reward.user_id,
            reward_type: new_reward.reward_type,
            description: new_reward.description,
            node_id: new_reward.node_id,
            license_id: new_reward.license_id,
            license_lease_id: new_reward.license_lease_id,
            task_key: new_reward.task_key,
            task_metadata: new_reward.task_metadata,
            completed_at: new_reward.completed_at,
            created_at: now.to_rfc3339(),
            amount_micros: new_reward.amount_micros,
        })
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
        let query = "SELECT id, user_id, type, description, node_id, license_id, license_lease_id, task_key, task_metadata, completed_at, created_at, amount_micros FROM rewards WHERE id = ?";

        let result = self
            .session
            .query_unpaged(query, (id,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_single_reward(result)
    }

    async fn list_rewards(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatedResult<RewardEntity>, String> {
        // Get total count
        let total = self.count_rewards().await?;

        // Fetch all and paginate in memory (ScyllaDB limitation)
        let query = "SELECT id, user_id, type, description, node_id, license_id, license_lease_id, task_key, task_metadata, completed_at, created_at, amount_micros FROM rewards";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        let all_rewards = Self::parse_rewards(result)?;
        let paginated = Self::paginate(all_rewards, &params);

        Ok(PaginatedResult::new(
            paginated,
            total,
            params.page,
            params.limit,
        ))
    }

    async fn list_rewards_by_license_id(
        &self,
        params: ListRewardsByLicenseParams,
    ) -> Result<PaginatedResult<RewardEntity>, String> {
        // Get total count for this license
        let total = self.count_rewards_by_license_id(&params.license_id).await?;

        let query = "SELECT id, user_id, type, description, node_id, license_id, license_lease_id, task_key, task_metadata, completed_at, created_at, amount_micros FROM rewards WHERE license_id = ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (&params.license_id,))
            .await
            .map_err(|e| e.to_string())?;

        let all_rewards = Self::parse_rewards(result)?;
        let paginated = Self::paginate(all_rewards, &params.pagination);

        Ok(PaginatedResult::new(
            paginated,
            total,
            params.pagination.page,
            params.pagination.limit,
        ))
    }

    async fn list_rewards_by_date(
        &self,
        params: ListRewardsByDateParams,
    ) -> Result<PaginatedResult<RewardEntity>, String> {
        let start_ts = Self::string_to_timestamp(&params.start_date);
        let end_ts = Self::string_to_timestamp(&params.end_date);

        // Count query for date range
        let count_query =
            "SELECT COUNT(*) FROM rewards WHERE created_at >= ? AND created_at <= ? ALLOW FILTERING";
        let count_result = self
            .session
            .query_unpaged(count_query, (start_ts, end_ts))
            .await
            .map_err(|e| e.to_string())?;
        let total = Self::parse_count(count_result)?;

        let query = "SELECT id, user_id, type, description, node_id, license_id, license_lease_id, task_key, task_metadata, completed_at, created_at, amount_micros FROM rewards WHERE created_at >= ? AND created_at <= ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (start_ts, end_ts))
            .await
            .map_err(|e| e.to_string())?;

        let all_rewards = Self::parse_rewards(result)?;
        let paginated = Self::paginate(all_rewards, &params.pagination);

        Ok(PaginatedResult::new(
            paginated,
            total,
            params.pagination.page,
            params.pagination.limit,
        ))
    }

    async fn list_rewards_by_license_id_and_date(
        &self,
        params: ListRewardsByLicenseAndDateParams,
    ) -> Result<PaginatedResult<RewardEntity>, String> {
        let start_ts = Self::string_to_timestamp(&params.start_date);
        let end_ts = Self::string_to_timestamp(&params.end_date);

        // Count query for license + date range
        let count_query = "SELECT COUNT(*) FROM rewards WHERE license_id = ? AND created_at >= ? AND created_at <= ? ALLOW FILTERING";
        let count_result = self
            .session
            .query_unpaged(count_query, (&params.license_id, start_ts, end_ts))
            .await
            .map_err(|e| e.to_string())?;
        let total = Self::parse_count(count_result)?;

        let query = "SELECT id, user_id, type, description, node_id, license_id, license_lease_id, task_key, task_metadata, completed_at, created_at, amount_micros FROM rewards WHERE license_id = ? AND created_at >= ? AND created_at <= ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (&params.license_id, start_ts, end_ts))
            .await
            .map_err(|e| e.to_string())?;

        let all_rewards = Self::parse_rewards(result)?;
        let paginated = Self::paginate(all_rewards, &params.pagination);

        Ok(PaginatedResult::new(
            paginated,
            total,
            params.pagination.page,
            params.pagination.limit,
        ))
    }

    async fn count_rewards(&self) -> Result<i64, String> {
        let query = "SELECT COUNT(*) FROM rewards";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_count(result)
    }

    async fn count_rewards_by_license_id(&self, license_id: &str) -> Result<i64, String> {
        let query = "SELECT COUNT(*) FROM rewards WHERE license_id = ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (license_id,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_count(result)
    }

    async fn get_total_earnings_by_license_id(&self, license_id: &str) -> Result<i64, String> {
        let query = "SELECT SUM(amount_micros) FROM rewards WHERE license_id = ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (license_id,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_sum(result)
    }

    async fn delete_reward(&self, id: &str) -> Result<bool, String> {
        let query = "DELETE FROM rewards WHERE id = ?";

        self.session
            .query_unpaged(query, (id,))
            .await
            .map_err(|e| e.to_string())?;

        Ok(true)
    }

    async fn get_most_recent_reward(&self) -> Result<Option<RewardEntity>, String> {
        // Fetch all rewards and find the most recent by created_at
        // Note: ScyllaDB doesn't support ORDER BY on non-clustering columns
        let query = "SELECT id, user_id, type, description, node_id, license_id, license_lease_id, task_key, task_metadata, completed_at, created_at, amount_micros FROM rewards";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        let mut rewards = Self::parse_rewards(result)?;

        if rewards.is_empty() {
            return Ok(None);
        }

        // Sort by created_at descending and return first
        rewards.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(rewards.into_iter().next())
    }

    async fn reward_exists(&self, id: &str) -> Result<bool, String> {
        let query = "SELECT id FROM rewards WHERE id = ?";

        let result = self
            .session
            .query_unpaged(query, (id,))
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(false),
        };

        let rows = match rows_result.rows::<(String,)>() {
            Ok(r) => r,
            Err(_) => return Ok(false),
        };

        // Check if any row exists
        for row_result in rows {
            if row_result.is_ok() {
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn upsert_rewards(&self, rewards: Vec<NewReward>) -> Result<usize, String> {
        let mut inserted_count = 0;

        for reward in rewards {
            // Check if reward already exists (by ID if provided)
            if let Some(ref id) = reward.id {
                if self.reward_exists(id).await? {
                    debug!("Reward {} already exists, skipping", id);
                    continue;
                }
            }

            // Insert the reward
            match self.save_reward(reward).await {
                Ok(saved) => {
                    inserted_count += 1;
                    debug!("Inserted reward: {}", saved.id);
                }
                Err(e) => {
                    error!("Failed to upsert reward: {}", e);
                }
            }
        }

        Ok(inserted_count)
    }

    async fn get_daily_totals(&self, start_date: &str, end_date: &str) -> Result<Vec<(String, i64, i32)>, String> {
        use std::collections::{HashMap, HashSet};

        let start_ts = Self::string_to_timestamp(start_date);
        let end_ts = Self::string_to_timestamp(end_date);

        // Fetch all rewards in the date range
        let query = "SELECT id, user_id, type, description, node_id, license_id, license_lease_id, task_key, task_metadata, completed_at, created_at, amount_micros FROM rewards WHERE completed_at >= ? AND completed_at <= ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (start_ts, end_ts))
            .await
            .map_err(|e| e.to_string())?;

        let rewards = Self::parse_rewards(result)?;

        // Group by date: sum amounts and track unique licenses
        let mut daily_totals: HashMap<String, i64> = HashMap::new();
        let mut daily_licenses: HashMap<String, HashSet<String>> = HashMap::new();

        for reward in rewards {
            // Extract date from completed_at (format: "2026-06-07T11:06:07.749+00:00")
            let date = reward.completed_at
                .split('T')
                .next()
                .unwrap_or(&reward.completed_at)
                .to_string();

            *daily_totals.entry(date.clone()).or_insert(0) += reward.amount_micros;

            // Only count license if it has positive earnings
            if reward.amount_micros > 0 {
                daily_licenses.entry(date).or_default().insert(reward.license_id);
            }
        }

        // Convert to sorted vector with license counts
        let mut result: Vec<(String, i64, i32)> = daily_totals
            .into_iter()
            .map(|(date, amount)| {
                let license_count = daily_licenses.get(&date).map(|s| s.len()).unwrap_or(0) as i32;
                (date, amount, license_count)
            })
            .collect();
        result.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(result)
    }
}
