//! Repository trait definitions

mod agent_repository_trait;
mod license_repository_trait;
mod license_analytics_repository_trait;
mod node_repository_trait;
mod reward_repository_trait;
mod sync_job_repository_trait;

pub use agent_repository_trait::AgentRepositoryTrait;
pub use license_repository_trait::{LicenseRepositoryTrait, MarketplaceData, SplitData};
pub use license_analytics_repository_trait::LicenseAnalyticsRepositoryTrait;
pub use node_repository_trait::NodeRepositoryTrait;
pub use reward_repository_trait::RewardRepositoryTrait;
pub use sync_job_repository_trait::SyncJobRepositoryTrait;
