//! ScyllaDB connection management for unity-dashboard.

use crate::db::error::{DbError, DbResult};
use scylla::Session;
use scylla::SessionBuilder;
use std::sync::Arc;
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
