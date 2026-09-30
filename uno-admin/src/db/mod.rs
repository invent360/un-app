//! Database module for unity-dashboard.
//!
//! Provides both ScyllaDB (legacy) and PostgreSQL connectivity for dashboard entities.
//!
//! ## Database Selection
//!
//! By default, the application uses ScyllaDB. To use PostgreSQL instead:
//!
//! 1. Enable the `postgres-db` feature in Cargo.toml
//! 2. Set the `DATABASE_URL` environment variable
//! 3. PostgreSQL migrations are in `./migrations/` directory
//!
//! With `postgres-db` feature enabled:
//! - `init_db()` initializes PostgreSQL instead of ScyllaDB
//! - `get_db()` returns `Arc<PgPool>` instead of `Arc<Session>`
//! - Services automatically use PostgreSQL repositories

#[cfg(feature = "ssr")]
pub mod connection;
#[cfg(feature = "ssr")]
pub mod error;
#[cfg(feature = "ssr")]
pub mod migrations;
#[cfg(feature = "ssr")]
pub mod seeder;

#[cfg(feature = "ssr")]
pub use connection::{DatabaseConfig, DatabaseConnection, DatabaseStats};
#[cfg(feature = "ssr")]
pub use connection::{PostgresConfig, PostgresConnection, PgPool};
#[cfg(feature = "ssr")]
pub use error::{DbError, DbResult};
#[cfg(feature = "ssr")]
pub use migrations::{MigrationRunner, MigrationResult};
#[cfg(feature = "ssr")]
pub use seeder::DatabaseSeeder;

#[cfg(feature = "ssr")]
use std::sync::OnceLock;

#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use scylla::Session;

// =============================================================================
// ScyllaDB Global State (Legacy)
// =============================================================================

/// Global ScyllaDB session
#[cfg(feature = "ssr")]
static DB_SESSION: OnceLock<Arc<Session>> = OnceLock::new();

/// Global keyspace name for ScyllaDB
#[cfg(feature = "ssr")]
static DB_KEYSPACE: OnceLock<String> = OnceLock::new();

// =============================================================================
// PostgreSQL Global State
// =============================================================================

/// Global PostgreSQL pool
#[cfg(feature = "ssr")]
static PG_POOL: OnceLock<Arc<PgPool>> = OnceLock::new();

// =============================================================================
// Type Aliases - Selected by Feature Flag
// =============================================================================

/// Database pool type alias.
///
/// When `postgres-db` feature is enabled, this is `Arc<PgPool>`.
/// Otherwise, this is `Arc<Session>` (ScyllaDB).
#[cfg(all(feature = "ssr", feature = "postgres-db"))]
pub type DbPool = Arc<PgPool>;

#[cfg(all(feature = "ssr", not(feature = "postgres-db")))]
pub type DbPool = Arc<Session>;

// =============================================================================
// Initialization Functions
// =============================================================================

/// Initialize the database connection.
///
/// When `postgres-db` feature is enabled, initializes PostgreSQL.
/// Otherwise, initializes ScyllaDB (legacy behavior).
#[cfg(all(feature = "ssr", feature = "postgres-db"))]
pub async fn init_db() -> DbResult<()> {
    init_pg_db().await
}

#[cfg(all(feature = "ssr", not(feature = "postgres-db")))]
pub async fn init_db() -> DbResult<()> {
    init_scylla_db().await
}

/// Initialize ScyllaDB connection (legacy).
#[cfg(feature = "ssr")]
pub async fn init_scylla_db() -> DbResult<()> {
    use tracing::info;

    info!("Initializing ScyllaDB connection...");
    let db = DatabaseConnection::new().await?;

    DB_SESSION.set(db.get_session()).map_err(|_| {
        DbError::Config("ScyllaDB already initialized".to_string())
    })?;

    DB_KEYSPACE.set(db.keyspace.clone()).map_err(|_| {
        DbError::Config("Keyspace already set".to_string())
    })?;

    info!("ScyllaDB connection initialized successfully");
    Ok(())
}

/// Initialize PostgreSQL connection.
#[cfg(feature = "ssr")]
pub async fn init_pg_db() -> DbResult<()> {
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

/// Get the database pool.
///
/// When `postgres-db` feature is enabled, returns PostgreSQL pool.
/// Otherwise, returns ScyllaDB session (legacy behavior).
#[cfg(all(feature = "ssr", feature = "postgres-db"))]
pub fn get_db() -> Option<DbPool> {
    get_pg_db()
}

#[cfg(all(feature = "ssr", not(feature = "postgres-db")))]
pub fn get_db() -> Option<DbPool> {
    get_scylla_db()
}

/// Get the ScyllaDB session (legacy).
#[cfg(feature = "ssr")]
pub fn get_scylla_db() -> Option<Arc<Session>> {
    DB_SESSION.get().cloned()
}

/// Get the PostgreSQL pool.
#[cfg(feature = "ssr")]
pub fn get_pg_db() -> Option<Arc<PgPool>> {
    PG_POOL.get().cloned()
}

/// Get the ScyllaDB keyspace name (legacy).
#[cfg(feature = "ssr")]
pub fn get_keyspace() -> Option<String> {
    DB_KEYSPACE.get().cloned()
}
