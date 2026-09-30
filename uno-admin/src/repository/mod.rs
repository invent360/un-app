//! Repository layer for database operations
//!
//! This module provides both ScyllaDB and PostgreSQL implementations for dashboard entities.
//!
//! ## Migration Status
//!
//! The PostgreSQL implementations are complete and ready to use. To switch from Scylla to PostgreSQL:
//!
//! 1. Set up PostgreSQL database and run migrations (see uno-app/migrations/00019_admin_tables.up.sql)
//! 2. Add `DATABASE_URL` environment variable pointing to PostgreSQL
//! 3. Enable the `postgres-db` feature flag in Cargo.toml
//!
//! ## Available implementations:
//!
//! - `Pg*Repository` - PostgreSQL implementations (recommended for new deployments)
//! - `Scylla*Repository` - Legacy ScyllaDB implementations (only compiled when postgres-db is NOT enabled)

pub mod traits;

// Conditionally compile Scylla module only when postgres-db is NOT enabled
#[cfg(not(feature = "postgres-db"))]
pub mod scylla;

// PostgreSQL module is always compiled (it's the target database)
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

// Re-export Scylla implementations with Scylla prefix (only when not using postgres-db)
#[cfg(not(feature = "postgres-db"))]
pub use scylla::{
    AgentRepository as ScyllaAgentRepository,
    LicenseRepository as ScyllaLicenseRepository,
    LicenseAnalyticsRepository as ScyllaLicenseAnalyticsRepository,
    NodeRepository as ScyllaNodeRepository,
    RewardRepository as ScyllaRewardRepository,
    SyncJobRepository as ScyllaSyncJobRepository,
};

// Re-export PostgreSQL implementations with Pg prefix
pub use postgres::{
    PgAgentRepository,
    PgLicenseRepository,
    PgLicenseAnalyticsRepository,
    PgNodeRepository,
    PgRewardRepository,
    PgSyncJobRepository,
};

// Default aliases based on feature flag
#[cfg(not(feature = "postgres-db"))]
mod defaults {
    pub use super::scylla::AgentRepository;
    pub use super::scylla::LicenseRepository;
    pub use super::scylla::LicenseAnalyticsRepository;
    pub use super::scylla::NodeRepository;
    pub use super::scylla::RewardRepository;
    pub use super::scylla::SyncJobRepository;
}

#[cfg(feature = "postgres-db")]
mod defaults {
    pub use super::postgres::PgAgentRepository as AgentRepository;
    pub use super::postgres::PgLicenseRepository as LicenseRepository;
    pub use super::postgres::PgLicenseAnalyticsRepository as LicenseAnalyticsRepository;
    pub use super::postgres::PgNodeRepository as NodeRepository;
    pub use super::postgres::PgRewardRepository as RewardRepository;
    pub use super::postgres::PgSyncJobRepository as SyncJobRepository;
}

pub use defaults::*;
