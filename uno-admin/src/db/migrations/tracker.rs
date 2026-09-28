//! Tracks applied migrations in schema_migrations table

use crate::db::error::{DbError, DbResult};
use chrono::Utc;
use scylla::frame::value::CqlTimestamp;
use scylla::Session;
use std::sync::Arc;
use tracing::{debug, info};

pub struct VersionTracker {
    session: Arc<Session>,
    keyspace: String,
}

impl VersionTracker {
    pub fn new(session: Arc<Session>, keyspace: &str) -> Self {
        Self {
            session,
            keyspace: keyspace.to_string(),
        }
    }

    /// Ensure schema_migrations table exists
    pub async fn init(&self) -> DbResult<()> {
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {}.schema_migrations (
                version int PRIMARY KEY,
                name text,
                applied_at timestamp,
                checksum text
            )",
            self.keyspace
        );

        self.session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| DbError::Migration(e.to_string()))?;

        debug!("schema_migrations table ready");
        Ok(())
    }

    /// Get all applied migration versions
    pub async fn get_applied_versions(&self) -> DbResult<Vec<u32>> {
        let query = format!(
            "SELECT version FROM {}.schema_migrations",
            self.keyspace
        );

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| DbError::Migration(e.to_string()))?;

        let mut versions = Vec::new();

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(versions),
        };

        let rows = match rows_result.rows::<(i32,)>() {
            Ok(r) => r,
            Err(_) => return Ok(versions),
        };

        for row_result in rows {
            if let Ok((version,)) = row_result {
                versions.push(version as u32);
            }
        }

        versions.sort();
        Ok(versions)
    }

    /// Record a migration as applied
    pub async fn record_migration(&self, version: u32, name: &str, checksum: &str) -> DbResult<()> {
        let query = format!(
            "INSERT INTO {}.schema_migrations (version, name, applied_at, checksum)
             VALUES (?, ?, ?, ?)",
            self.keyspace
        );

        let now = CqlTimestamp(Utc::now().timestamp_millis());

        self.session
            .query_unpaged(query, (version as i32, name, now, checksum))
            .await
            .map_err(|e| DbError::Migration(e.to_string()))?;

        info!("Recorded migration {} - {}", version, name);
        Ok(())
    }

    /// Remove a migration record (for rollback)
    pub async fn remove_migration(&self, version: u32) -> DbResult<()> {
        let query = format!(
            "DELETE FROM {}.schema_migrations WHERE version = ?",
            self.keyspace
        );

        self.session
            .query_unpaged(query, (version as i32,))
            .await
            .map_err(|e| DbError::Migration(e.to_string()))?;

        info!("Removed migration record {}", version);
        Ok(())
    }
}
