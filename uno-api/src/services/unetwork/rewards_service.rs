//! Unetwork rewards service implementation.
//!
//! Provides a service wrapper around `UnetworkRewardsTrait` implementations
//! with additional convenience methods for managing license rewards.

use std::sync::Arc;

use async_trait::async_trait;

use crate::error::ApiError;
use crate::models::request::{GetRewardsBalanceRequest, GetRewardsRequest};
use crate::models::response::{RewardAllocation, RewardBalance};

use super::UnetworkRewardsTrait;

/// Unetwork rewards service.
///
/// This service wraps a `UnetworkRewardsTrait` implementation (typically
/// the HTTP client) and provides additional convenience methods.
///
/// # Example
///
/// ```ignore
/// use uno_api::client::UnetworkClient;
/// use uno_api::config::UnetworkConfig;
/// use uno_api::services::unetwork::UnetworkRewardsService;
/// use std::sync::Arc;
///
/// let config = UnetworkConfig::new("your-jwt-token");
/// let client = Arc::new(UnetworkClient::new(config));
/// let service = UnetworkRewardsService::new(client);
///
/// // Get rewards for a license
/// let rewards = service.get_rewards("0x123...", 0, 20).await?;
///
/// // Get balance
/// let balance = service.get_balance("0x123...").await?;
/// ```
pub struct UnetworkRewardsService<P: UnetworkRewardsTrait> {
    provider: Arc<P>,
}

impl<P: UnetworkRewardsTrait> UnetworkRewardsService<P> {
    /// Create a new service with the given provider.
    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }

    /// Get the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }

    /// Get all reward allocations for a license with pagination.
    pub async fn get_rewards(
        &self,
        license_id: &str,
        skip: u32,
        take: u32,
    ) -> Result<Vec<RewardAllocation>, ApiError> {
        self.provider
            .get_rewards(GetRewardsRequest::new(license_id).with_pagination(skip, take))
            .await
    }

    /// Get all rewards for a license (fetches all pages).
    ///
    /// Warning: This may make multiple API calls for licenses with many rewards.
    pub async fn get_all_rewards(
        &self,
        license_id: &str,
    ) -> Result<Vec<RewardAllocation>, ApiError> {
        let mut all_rewards = Vec::new();
        let page_size = 100;
        let mut skip = 0;

        loop {
            let rewards = self.get_rewards(license_id, skip, page_size).await?;
            let count = rewards.len();

            if rewards.is_empty() {
                break;
            }

            // Check if we've reached the end
            let total = rewards.first().and_then(|r| r.total_count).unwrap_or(0) as usize;
            all_rewards.extend(rewards);

            if all_rewards.len() >= total || count < page_size as usize {
                break;
            }

            skip += page_size;
        }

        Ok(all_rewards)
    }

    /// Get reward balance for a license.
    pub async fn get_balance(&self, license_id: &str) -> Result<RewardBalance, ApiError> {
        self.provider
            .get_balance(GetRewardsBalanceRequest::new(license_id))
            .await
    }

    /// Get pending (unpaid) rewards for a license.
    pub async fn get_pending_rewards(
        &self,
        license_id: &str,
    ) -> Result<Vec<RewardAllocation>, ApiError> {
        let rewards = self.get_all_rewards(license_id).await?;
        Ok(rewards.into_iter().filter(|r| r.is_pending()).collect())
    }

    /// Get paid rewards for a license.
    pub async fn get_paid_rewards(
        &self,
        license_id: &str,
    ) -> Result<Vec<RewardAllocation>, ApiError> {
        let rewards = self.get_all_rewards(license_id).await?;
        Ok(rewards.into_iter().filter(|r| r.is_paid()).collect())
    }

    /// Calculate total rewards earned for a license.
    pub async fn get_total_earned(&self, license_id: &str) -> Result<f64, ApiError> {
        let balance = self.get_balance(license_id).await?;
        Ok(balance.total_earned)
    }

    /// Calculate total pending rewards for a license.
    pub async fn get_total_pending(&self, license_id: &str) -> Result<f64, ApiError> {
        let balance = self.get_balance(license_id).await?;
        Ok(balance.pending_balance)
    }
}

// Forward trait implementation to provider
#[async_trait]
impl<P: UnetworkRewardsTrait> UnetworkRewardsTrait for UnetworkRewardsService<P> {
    async fn get_rewards(
        &self,
        request: GetRewardsRequest,
    ) -> Result<Vec<RewardAllocation>, ApiError> {
        self.provider.get_rewards(request).await
    }

    async fn get_balance(&self, request: GetRewardsBalanceRequest) -> Result<RewardBalance, ApiError> {
        self.provider.get_balance(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    // Mock provider for testing
    struct MockProvider {
        rewards: Vec<RewardAllocation>,
        balance: RewardBalance,
    }

    impl MockProvider {
        fn new() -> Self {
            Self {
                rewards: vec![],
                balance: RewardBalance {
                    license_id: "0x123".to_string(),
                    total_earned: 100.0,
                    total_paid: 40.0,
                    pending_balance: 60.0,
                    currency: Some("USDT".to_string()),
                    updated_at: None,
                },
            }
        }

        fn with_rewards(mut self, rewards: Vec<RewardAllocation>) -> Self {
            self.rewards = rewards;
            self
        }

        fn with_balance(mut self, balance: RewardBalance) -> Self {
            self.balance = balance;
            self
        }
    }

    #[async_trait]
    impl UnetworkRewardsTrait for MockProvider {
        async fn get_rewards(
            &self,
            _request: GetRewardsRequest,
        ) -> Result<Vec<RewardAllocation>, ApiError> {
            Ok(self.rewards.clone())
        }

        async fn get_balance(
            &self,
            _request: GetRewardsBalanceRequest,
        ) -> Result<RewardBalance, ApiError> {
            Ok(self.balance.clone())
        }
    }

    fn create_test_reward(status: Option<&str>) -> RewardAllocation {
        RewardAllocation {
            id: Uuid::new_v4(),
            license_id: "0x123".to_string(),
            amount: 10.0,
            currency: Some("USDT".to_string()),
            period: Some("2024-01".to_string()),
            created_at: None,
            status: status.map(|s| s.to_string()),
            tx_hash: None,
            total_count: Some(2),
        }
    }

    #[tokio::test]
    async fn test_service_creation() {
        let provider = Arc::new(MockProvider::new());
        let service = UnetworkRewardsService::new(provider);

        let balance = service.get_balance("0x123").await.unwrap();
        assert_eq!(balance.total_earned, 100.0);
    }

    #[tokio::test]
    async fn test_get_rewards() {
        let rewards = vec![
            create_test_reward(Some("pending")),
            create_test_reward(Some("paid")),
        ];

        let provider = Arc::new(MockProvider::new().with_rewards(rewards));
        let service = UnetworkRewardsService::new(provider);

        let result = service.get_rewards("0x123", 0, 20).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_get_pending_rewards() {
        let rewards = vec![
            create_test_reward(Some("pending")),
            create_test_reward(Some("paid")),
        ];

        let provider = Arc::new(MockProvider::new().with_rewards(rewards));
        let service = UnetworkRewardsService::new(provider);

        let pending = service.get_pending_rewards("0x123").await.unwrap();
        assert_eq!(pending.len(), 1);
        assert!(pending[0].is_pending());
    }

    #[tokio::test]
    async fn test_get_paid_rewards() {
        let rewards = vec![
            create_test_reward(Some("pending")),
            create_test_reward(Some("paid")),
        ];

        let provider = Arc::new(MockProvider::new().with_rewards(rewards));
        let service = UnetworkRewardsService::new(provider);

        let paid = service.get_paid_rewards("0x123").await.unwrap();
        assert_eq!(paid.len(), 1);
        assert!(paid[0].is_paid());
    }

    #[tokio::test]
    async fn test_get_total_earned() {
        let provider = Arc::new(MockProvider::new());
        let service = UnetworkRewardsService::new(provider);

        let total = service.get_total_earned("0x123").await.unwrap();
        assert!((total - 100.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_get_total_pending() {
        let provider = Arc::new(MockProvider::new());
        let service = UnetworkRewardsService::new(provider);

        let total = service.get_total_pending("0x123").await.unwrap();
        assert!((total - 60.0).abs() < 0.01);
    }
}
