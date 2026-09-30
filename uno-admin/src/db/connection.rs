//! Database connection management for uno-admin.
//!
//! PostgreSQL is the production database.

use crate::db::error::{DbError, DbResult};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

/// PostgreSQL connection pool type alias
pub type PgPool = Pool<Postgres>;

/// PostgreSQL configuration.
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub acquire_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    /// Whether to run migrations on startup (default: true).
    pub run_migrations: bool,
}

impl PostgresConfig {
    /// Create configuration from environment variables.
    pub fn from_env() -> DbResult<Self> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| DbError::Config("DATABASE_URL environment variable is required".to_string()))?;

        let max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);

        let acquire_timeout_secs: u64 = env::var("DATABASE_ACQUIRE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        let idle_timeout_secs: u64 = env::var("DATABASE_IDLE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(600);

        let run_migrations = env::var("RUN_MIGRATIONS")
            .unwrap_or_else(|_| "true".to_string())
            != "false";

        Ok(Self {
            database_url,
            max_connections,
            acquire_timeout_secs,
            idle_timeout_secs,
            run_migrations,
        })
    }
}

/// PostgreSQL connection wrapper.
pub struct PostgresConnection {
    pub pool: Arc<PgPool>,
    pub config: PostgresConfig,
}

impl PostgresConnection {
    /// Create new PostgreSQL connection from environment variables.
    pub async fn new() -> DbResult<Self> {
        let config = PostgresConfig::from_env()?;
        Self::with_config(config).await
    }

    /// Create PostgreSQL connection with specific configuration.
    pub async fn with_config(config: PostgresConfig) -> DbResult<Self> {
        info!("Initializing PostgreSQL connection");
        debug!("PostgreSQL Configuration:");
        debug!("  Max Connections: {}", config.max_connections);
        debug!("  Acquire Timeout: {}s", config.acquire_timeout_secs);
        debug!("  Idle Timeout: {}s", config.idle_timeout_secs);
        debug!("  Run Migrations: {}", config.run_migrations);

        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs))
            .idle_timeout(Duration::from_secs(config.idle_timeout_secs))
            .connect(&config.database_url)
            .await
            .map_err(|e| {
                error!("Failed to create PostgreSQL pool: {}", e);
                DbError::Connection(e.to_string())
            })?;

        info!(
            "PostgreSQL pool created with max {} connections",
            config.max_connections
        );

        let connection = Self {
            pool: Arc::new(pool),
            config,
        };

        // Run migrations if enabled
        if connection.config.run_migrations {
            connection.migrate().await?;
        } else {
            info!("Skipping PostgreSQL migrations (RUN_MIGRATIONS=false)");
        }

        info!("PostgreSQL connection established successfully");
        Ok(connection)
    }

    /// Get reference to the PostgreSQL pool.
    pub fn get_pool(&self) -> Arc<PgPool> {
        self.pool.clone()
    }

    /// Run database migrations.
    ///
    /// Note: Uses uno-app's migrations directory since uno-admin shares the same
    /// PostgreSQL database. The migrations include admin tables (00019_admin_tables.up.sql).
    async fn migrate(&self) -> DbResult<()> {
        info!("Running PostgreSQL migrations...");

        // Get migrations directory from environment or use uno-app's default
        let migrations_dir = env::var("PG_MIGRATIONS_DIR")
            .unwrap_or_else(|_| "../uno-app/migrations".to_string());

        sqlx::migrate::Migrator::new(std::path::Path::new(&migrations_dir))
            .await
            .map_err(|e| {
                error!("Failed to load migrations from {}: {}", migrations_dir, e);
                DbError::Migration(e.to_string())
            })?
            .run(self.pool.as_ref())
            .await
            .map_err(|e| {
                error!("PostgreSQL migration failed: {}", e);
                DbError::Migration(e.to_string())
            })?;

        info!("PostgreSQL migrations completed successfully");
        Ok(())
    }

    /// Test database connectivity.
    pub async fn test_connection(&self) -> DbResult<()> {
        match sqlx::query("SELECT 1")
            .fetch_one(self.pool.as_ref())
            .await
        {
            Ok(_) => {
                info!("PostgreSQL connection test passed");
                Ok(())
            }
            Err(e) => {
                error!("PostgreSQL connection test failed: {}", e);
                Err(DbError::Query(e.to_string()))
            }
        }
    }
}
