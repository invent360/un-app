//! PostgreSQL connection pool using SQLx with ConnectionManager pattern

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;

/// Database pool type alias
pub type ConnectionPool = Pool<Postgres>;

/// ConnectionManager handles database pool creation and migrations
pub struct ConnectionManager;

impl ConnectionManager {
    /// Create a new database connection pool
    pub async fn new_pool(
        connection_url: &str,
        run_migrations: bool,
    ) -> Result<ConnectionPool, sqlx::Error> {
        let max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(600))
            .connect(connection_url)
            .await?;

        println!("Database pool created with max {} connections", max_connections);

        if run_migrations {
            println!("Running database migrations...");
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .map_err(|e| sqlx::Error::Configuration(e.to_string().into()))?;
            println!("Migrations completed successfully");
        }

        Ok(pool)
    }

    /// Create pool from environment variable DATABASE_URL
    pub async fn from_env(run_migrations: bool) -> Result<ConnectionPool, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://user:password@localhost:5438/amba".to_string());

        Self::new_pool(&database_url, run_migrations).await
    }
}
