//! Database module for unity-dashboard.
//!
//! Provides ScyllaDB connectivity and file-based migrations for dashboard entities.

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

/// Global database session
#[cfg(feature = "ssr")]
static DB_SESSION: OnceLock<Arc<Session>> = OnceLock::new();

/// Global keyspace name
#[cfg(feature = "ssr")]
static DB_KEYSPACE: OnceLock<String> = OnceLock::new();

/// Database pool type alias for compatibility
#[cfg(feature = "ssr")]
pub type DbPool = Arc<Session>;

/// Initialize the database connection
#[cfg(feature = "ssr")]
pub async fn init_db() -> DbResult<()> {
    use tracing::info;

    info!("Initializing ScyllaDB connection...");
    let db = DatabaseConnection::new().await?;

    DB_SESSION.set(db.get_session()).map_err(|_| {
        DbError::Config("Database already initialized".to_string())
    })?;

    DB_KEYSPACE.set(db.keyspace.clone()).map_err(|_| {
        DbError::Config("Keyspace already set".to_string())
    })?;

    info!("ScyllaDB connection initialized successfully");
    Ok(())
}

/// Get the database session
#[cfg(feature = "ssr")]
pub fn get_db() -> Option<DbPool> {
    DB_SESSION.get().cloned()
}

/// Get the keyspace name
#[cfg(feature = "ssr")]
pub fn get_keyspace() -> Option<String> {
    DB_KEYSPACE.get().cloned()
}
