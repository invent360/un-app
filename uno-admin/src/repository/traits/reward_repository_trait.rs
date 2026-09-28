//! Reward repository trait definition

use async_trait::async_trait;
use crate::models::entity::{
    ListRewardsByDateParams,
    ListRewardsByLicenseAndDateParams,
    ListRewardsByLicenseParams,
    NewReward,
    PaginatedResult,
    PaginationParams,
    RewardEntity,
};

/// Repository trait for Reward operations
#[async_trait]
pub trait RewardRepositoryTrait: Send + Sync {
    /// Save a new reward to the database
    async fn save_reward(&self, new_reward: NewReward) -> Result<RewardEntity, String>;

    /// Bulk save rewards to the database
    async fn save_rewards(&self, rewards: Vec<NewReward>) -> Result<usize, String>;

    /// Get a reward by its primary key (UUID)
    async fn get_reward_by_id(&self, id: &str) -> Result<Option<RewardEntity>, String>;

    /// List all rewards with pagination, ordered by created_at (most recent first)
    async fn list_rewards(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatedResult<RewardEntity>, String>;

    /// List rewards by license_id with pagination, ordered by created_at (most recent first)
    async fn list_rewards_by_license_id(
        &self,
        params: ListRewardsByLicenseParams,
    ) -> Result<PaginatedResult<RewardEntity>, String>;

    /// List rewards by date range with pagination, ordered by created_at (most recent first)
    async fn list_rewards_by_date(
        &self,
        params: ListRewardsByDateParams,
    ) -> Result<PaginatedResult<RewardEntity>, String>;

    /// List rewards by license_id and date range with pagination, ordered by created_at (most recent first)
    async fn list_rewards_by_license_id_and_date(
        &self,
        params: ListRewardsByLicenseAndDateParams,
    ) -> Result<PaginatedResult<RewardEntity>, String>;

    /// Count all rewards
    async fn count_rewards(&self) -> Result<i64, String>;

    /// Count rewards for a specific license
    async fn count_rewards_by_license_id(&self, license_id: &str) -> Result<i64, String>;

    /// Get total earnings (sum of amount_micros) for a license
    async fn get_total_earnings_by_license_id(&self, license_id: &str) -> Result<i64, String>;

    /// Delete a reward by ID
    async fn delete_reward(&self, id: &str) -> Result<bool, String>;

    /// Get the most recent reward by created_at timestamp
    async fn get_most_recent_reward(&self) -> Result<Option<RewardEntity>, String>;

    /// Check if reward with given ID already exists
    async fn reward_exists(&self, id: &str) -> Result<bool, String>;

    /// Bulk upsert rewards (insert if not exists, returns count of newly inserted)
    async fn upsert_rewards(&self, rewards: Vec<NewReward>) -> Result<usize, String>;

    /// Get daily totals for rewards in a date range
    /// Returns a vector of (date_string, total_amount_micros, unique_license_count) tuples sorted by date ascending
    async fn get_daily_totals(&self, start_date: &str, end_date: &str) -> Result<Vec<(String, i64, i32)>, String>;
}
