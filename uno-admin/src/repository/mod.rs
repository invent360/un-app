//! Repository layer for database operations
//!
//! Provides ScyllaDB implementations for dashboard entities.

pub mod traits;
pub mod scylla;

pub use traits::{AgentRepositoryTrait, LicenseRepositoryTrait, LicenseAnalyticsRepositoryTrait, NodeRepositoryTrait, RewardRepositoryTrait, SyncJobRepositoryTrait};
pub use scylla::{AgentRepository, LicenseRepository, LicenseAnalyticsRepository, NodeRepository, RewardRepository, SyncJobRepository};
