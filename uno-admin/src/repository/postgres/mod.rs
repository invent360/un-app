//! PostgreSQL repository implementations
//!
//! Provides PostgreSQL-backed implementations for dashboard entities,
//! replacing the Scylla implementations for better ACID guarantees.

mod agent_repository;
mod license_analytics_repository;
mod license_repository;
mod node_repository;
mod reward_repository;
mod sync_job_repository;

pub use agent_repository::PgAgentRepository;
pub use license_analytics_repository::PgLicenseAnalyticsRepository;
pub use license_repository::LicenseRepository as PgLicenseRepository;
pub use node_repository::PgNodeRepository;
pub use reward_repository::PgRewardRepository;
pub use sync_job_repository::PgSyncJobRepository;
