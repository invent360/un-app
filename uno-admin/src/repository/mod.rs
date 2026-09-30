//! Repository layer for database operations
//!
//! PostgreSQL implementations for dashboard entities.

pub mod traits;
pub mod postgres;

// Re-export traits
pub use traits::{
    AgentRepositoryTrait,
    LicenseRepositoryTrait,
    LicenseAnalyticsRepositoryTrait,
    NodeRepositoryTrait,
    RewardRepositoryTrait,
    SyncJobRepositoryTrait,
    MarketplaceData,
    SplitData,
};

// Re-export PostgreSQL implementations
pub use postgres::{
    PgAgentRepository,
    PgLicenseRepository,
    PgLicenseAnalyticsRepository,
    PgNodeRepository,
    PgRewardRepository,
    PgSyncJobRepository,
};

// Default aliases (PostgreSQL)
pub use postgres::PgAgentRepository as AgentRepository;
pub use postgres::PgLicenseRepository as LicenseRepository;
pub use postgres::PgLicenseAnalyticsRepository as LicenseAnalyticsRepository;
pub use postgres::PgNodeRepository as NodeRepository;
pub use postgres::PgRewardRepository as RewardRepository;
pub use postgres::PgSyncJobRepository as SyncJobRepository;
