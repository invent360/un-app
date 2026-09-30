//! Database connection management for unity-dashboard.
//!
//! Supports both ScyllaDB (legacy) and PostgreSQL (recommended).

use crate::db::error::{DbError, DbResult};
use scylla::Session;
use scylla::SessionBuilder;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

/// Database configuration.
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: String,
    pub keyspace: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub replication_factor: String,
    pub should_destroy: bool,
    /// Whether to run migrations on startup (default: true).
    pub run_migrations: bool,
}

impl DatabaseConfig {
    /// Create configuration from environment variables.
    pub fn from_env() -> DbResult<Self> {
        Ok(Self {
            host: std::env::var("SCYLLA_DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("SCYLLA_DB_PORT").unwrap_or_else(|_| "9042".to_string()),
            keyspace: std::env::var("SCYLLA_DB_KEYSPACE")
                .unwrap_or_else(|_| "unity_dashboard".to_string())
                .replace('-', "_"),
            username: std::env::var("SCYLLA_DB_USERNAME").ok().filter(|s| !s.is_empty()),
            password: std::env::var("SCYLLA_DB_PASSWORD").ok().filter(|s| !s.is_empty()),
            replication_factor: std::env::var("SCYLLA_DB_REPLICATION_FACTOR")
                .unwrap_or_else(|_| "1".to_string()),
            should_destroy: std::env::var("DESTROY_TABLES")
                .unwrap_or_else(|_| "false".to_string())
                == "true",
            run_migrations: std::env::var("RUN_MIGRATIONS")
                .unwrap_or_else(|_| "true".to_string())
                != "false",
        })
    }

    /// Get connection string.
    pub fn connection_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Main database connection wrapper.
pub struct DatabaseConnection {
    pub session: Arc<Session>,
    pub keyspace: String,
    pub config: DatabaseConfig,
}

impl DatabaseConnection {
    /// Create new database connection with automatic migration.
    pub async fn new() -> DbResult<Self> {
        let config = DatabaseConfig::from_env()?;
        Self::with_config(config).await
    }

    /// Create database connection with specific configuration.
    pub async fn with_config(config: DatabaseConfig) -> DbResult<Self> {
        info!("Initializing ScyllaDB connection");
        debug!("Database Configuration:");
        debug!("  Host: {}", config.host);
        debug!("  Port: {}", config.port);
        debug!("  Keyspace: {}", config.keyspace);
        debug!(
            "  Username: {:?}",
            config.username.as_ref().map(|_| "***")
        );
        debug!("  Replication Factor: {}", config.replication_factor);

        // Build session
        let mut session_builder = SessionBuilder::new().known_node(&config.connection_string());

        // Add authentication if provided
        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            session_builder = session_builder.user(username, password);
            debug!("  Authentication: Enabled");
        } else {
            debug!("  Authentication: Disabled");
        }

        // Create session
        let session = session_builder.build().await.map_err(|e| {
            error!("Failed to create ScyllaDB session: {}", e);
            DbError::Connection(e.to_string())
        })?;

        let connection = Self {
            session: Arc::new(session),
            keyspace: config.keyspace.clone(),
            config,
        };

        // Run migrations
        connection.migrate().await?;

        info!("ScyllaDB connection established successfully");
        Ok(connection)
    }

    /// Get reference to the ScyllaDB session.
    pub fn get_session(&self) -> Arc<Session> {
        self.session.clone()
    }

    /// Run database migrations (file-based).
    pub async fn migrate(&self) -> DbResult<()> {
        use crate::db::migrations::MigrationRunner;

        // Skip migrations if disabled via RUN_MIGRATIONS=false
        if !self.config.run_migrations {
            info!("Skipping database migrations (RUN_MIGRATIONS=false)");
            return Ok(());
        }

        info!("Running file-based migrations");

        // Ensure keyspace exists first
        self.create_keyspace().await?;

        // Get migrations directory (relative to project root)
        let migrations_dir = std::env::var("MIGRATIONS_DIR")
            .unwrap_or_else(|_| "./migrations".to_string());

        let runner = MigrationRunner::new(
            self.session.clone(),
            &self.keyspace,
            &migrations_dir,
        );

        if self.config.should_destroy {
            info!("Rolling back all migrations (DESTROY_TABLES=true)");
            // First truncate schema_migrations to ensure clean slate
            let truncate_query = format!(
                "TRUNCATE {}.schema_migrations",
                self.keyspace
            );
            if let Err(e) = self.session.query_unpaged(truncate_query, &[]).await {
                debug!("Could not truncate schema_migrations (may not exist): {}", e);
            } else {
                info!("Truncated schema_migrations table");
            }
            // Then run rollback to drop tables
            let _ = runner.rollback(100).await; // Rollback all, ignore errors
        }

        let results = runner.run_pending().await?;

        for result in &results {
            if result.success {
                info!("✓ Migration {}: {}", result.version, result.name);
            } else {
                error!(
                    "✗ Migration {}: {} - {:?}",
                    result.version, result.name, result.error
                );
                return Err(DbError::Migration(
                    result.error.clone().unwrap_or_default(),
                ));
            }
        }

        info!("Migrations completed: {} applied", results.len());

        // Run database seeder after migrations
        self.seed().await?;

        Ok(())
    }

    /// Seed database with initial data from JSON files
    async fn seed(&self) -> DbResult<()> {
        use crate::db::seeder::DatabaseSeeder;

        // Skip seeding if disabled via SKIP_SEED=true
        if std::env::var("SKIP_SEED").unwrap_or_default() == "true" {
            info!("Skipping database seeding (SKIP_SEED=true)");
            return Ok(());
        }

        info!("Running database seeder");
        let seeder = DatabaseSeeder::new(self.session.clone(), &self.keyspace);
        seeder.seed_all().await?;

        Ok(())
    }

    /// Create keyspace if it doesn't exist.
    async fn create_keyspace(&self) -> DbResult<()> {
        let query = format!(
            "CREATE KEYSPACE IF NOT EXISTS {}
             WITH replication = {{'class': 'SimpleStrategy', 'replication_factor': {}}}
             AND durable_writes = true",
            self.keyspace, self.config.replication_factor
        );

        self.session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        self.session
            .use_keyspace(&self.keyspace, false)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        debug!("Keyspace {} ready", self.keyspace);
        Ok(())
    }

    /// Test database connectivity.
    pub async fn test_connection(&self) -> DbResult<()> {
        let query = format!(
            "SELECT keyspace_name FROM system_schema.keyspaces WHERE keyspace_name = '{}'",
            self.keyspace
        );

        match self.session.query_unpaged(query, &[]).await {
            Ok(_) => {
                info!("Database connection test passed");
                Ok(())
            }
            Err(e) => {
                error!("Database connection test failed: {}", e);
                Err(DbError::Query(e.to_string()))
            }
        }
    }

    /// Get database statistics.
    pub async fn get_stats(&self) -> DbResult<DatabaseStats> {
        Ok(DatabaseStats {
            keyspace: self.keyspace.clone(),
            table_count: 0,
            connection_info: format!("{}:{}", self.config.host, self.config.port),
        })
    }
}

/// Database statistics.
#[derive(Debug)]
pub struct DatabaseStats {
    pub keyspace: String,
    pub table_count: i64,
    pub connection_info: String,
}

// =============================================================================
// PostgreSQL Connection Management
// =============================================================================

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
