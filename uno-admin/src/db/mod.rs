//! Database module for uno-admin.
//!
//! Provides PostgreSQL connectivity for dashboard entities.

#[cfg(feature = "ssr")]
pub mod connection;
#[cfg(feature = "ssr")]
pub mod error;

#[cfg(feature = "ssr")]
pub use connection::{PostgresConfig, PostgresConnection, PgPool};
#[cfg(feature = "ssr")]
pub use error::{DbError, DbResult};

#[cfg(feature = "ssr")]
use std::sync::OnceLock;

#[cfg(feature = "ssr")]
use std::sync::Arc;

// =============================================================================
// PostgreSQL Global State
// =============================================================================

/// Global PostgreSQL pool
#[cfg(feature = "ssr")]
static PG_POOL: OnceLock<Arc<PgPool>> = OnceLock::new();

/// Database pool type alias (PostgreSQL).
#[cfg(feature = "ssr")]
pub type DbPool = Arc<PgPool>;

// =============================================================================
// Initialization Functions
// =============================================================================

/// Initialize the database connection (PostgreSQL).
#[cfg(feature = "ssr")]
pub async fn init_db() -> DbResult<()> {
    use tracing::info;

    info!("Initializing PostgreSQL connection...");
    let db = PostgresConnection::new().await?;

    PG_POOL.set(db.get_pool()).map_err(|_| {
        DbError::Config("PostgreSQL already initialized".to_string())
    })?;

    info!("PostgreSQL connection initialized successfully");
    Ok(())
}

// =============================================================================
// Accessor Functions
// =============================================================================

/// Get the database pool (PostgreSQL).
#[cfg(feature = "ssr")]
pub fn get_db() -> Option<DbPool> {
    PG_POOL.get().cloned()
}
