pub mod license_logic;
pub mod user_logic;
pub mod dashboard_overview;

#[cfg(feature = "ssr")]
pub mod agent_logic;
#[cfg(feature = "ssr")]
pub mod node_logic;
#[cfg(feature = "ssr")]
pub mod rewards_sync_service;
#[cfg(feature = "ssr")]
pub mod marketplace_service;
#[cfg(feature = "ssr")]
pub mod referral_sync_service;
#[cfg(feature = "ssr")]
pub mod license_sync_service;
#[cfg(feature = "ssr")]
pub mod job_queue;
#[cfg(feature = "ssr")]
pub mod job_worker;

// Phase 8: Sync & Reconciliation modules
#[cfg(feature = "ssr")]
pub mod claim_cursor;
#[cfg(feature = "ssr")]
pub mod durable_job_queue;
#[cfg(feature = "ssr")]
pub mod health;

pub use license_logic::*;
pub use user_logic::*;
pub use dashboard_overview::*;

#[cfg(feature = "ssr")]
pub use agent_logic::*;
#[cfg(feature = "ssr")]
pub use node_logic::*;
#[cfg(feature = "ssr")]
pub use rewards_sync_service::*;
#[cfg(feature = "ssr")]
pub use marketplace_service::*;
#[cfg(feature = "ssr")]
pub use referral_sync_service::*;
#[cfg(feature = "ssr")]
pub use license_sync_service::*;
#[cfg(feature = "ssr")]
pub use job_queue::{enqueue, is_initialized, JobCommand};
#[cfg(feature = "ssr")]
pub use job_worker::start_job_worker;

// Phase 8 exports
#[cfg(feature = "ssr")]
pub use claim_cursor::{ClaimCursor, SyncCheckpoint, ClaimRecord};
#[cfg(feature = "ssr")]
pub use durable_job_queue::{DurableJob, JobStatus, JobPriority, WorkerLease, DurableQueueConfig};
#[cfg(feature = "ssr")]
pub use health::{HealthService, HealthCheck, HealthStatus, StartupChecks};
