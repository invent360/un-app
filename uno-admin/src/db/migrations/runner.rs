//! Orchestrates migration execution

use super::loader::MigrationLoader;
use super::tracker::VersionTracker;
use super::types::{Migration, MigrationDirection, MigrationResult};
use crate::db::error::{DbError, DbResult};
use scylla::Session;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

pub struct MigrationRunner {
    session: Arc<Session>,
    keyspace: String,
    loader: MigrationLoader,
    tracker: VersionTracker,
}

impl MigrationRunner {
    pub fn new(session: Arc<Session>, keyspace: &str, migrations_dir: &str) -> Self {
        Self {
            session: session.clone(),
            keyspace: keyspace.to_string(),
            loader: MigrationLoader::new(migrations_dir),
            tracker: VersionTracker::new(session, keyspace),
        }
    }

    /// Ensure we're using the correct keyspace before running migrations
    async fn ensure_keyspace(&self) -> DbResult<()> {
        debug!("Ensuring keyspace {} is active", self.keyspace);
        self.session
            .use_keyspace(&self.keyspace, false)
            .await
            .map_err(|e| DbError::Migration(format!("Failed to use keyspace: {}", e)))?;
        Ok(())
    }

    /// Run all pending migrations
    pub async fn run_pending(&self) -> DbResult<Vec<MigrationResult>> {
        info!("Starting migration run...");

        // Ensure we're using the correct keyspace
        self.ensure_keyspace().await?;

        // Ensure tracking table exists
        self.tracker.init().await?;

        // Load all migrations from files
        let all_migrations = self.loader.load_all()?;
        info!("Found {} migration files", all_migrations.len());

        // Get already applied versions
        let applied = self.tracker.get_applied_versions().await?;
        debug!("Already applied: {:?}", applied);

        // Filter to pending migrations
        let pending: Vec<&Migration> = all_migrations
            .iter()
            .filter(|m| !applied.contains(&m.version))
            .collect();

        if pending.is_empty() {
            info!("No pending migrations");
            return Ok(vec![]);
        }

        info!("Running {} pending migrations", pending.len());

        let mut results = Vec::new();
        for migration in pending {
            let result = self.run_migration(migration, MigrationDirection::Up).await;
            let success = result.is_ok();

            results.push(MigrationResult {
                version: migration.version,
                name: migration.name.clone(),
                direction: MigrationDirection::Up,
                success,
                error: result.err().map(|e| e.to_string()),
            });

            if !success {
                error!("Migration {} failed, stopping", migration.version);
                break;
            }
        }

        Ok(results)
    }

    /// Run a single migration
    async fn run_migration(
        &self,
        migration: &Migration,
        direction: MigrationDirection,
    ) -> DbResult<()> {
        let sql = match direction {
            MigrationDirection::Up => &migration.up_sql,
            MigrationDirection::Down => migration
                .down_sql
                .as_ref()
                .ok_or_else(|| DbError::Migration("No down migration available".to_string()))?,
        };

        info!(
            "Running migration {} - {} ({:?})",
            migration.version, migration.name, direction
        );

        // Execute each statement (split by semicolon)
        for statement in sql.split(';') {
            // Remove comment lines and trim
            let statement: String = statement
                .lines()
                .filter(|line| !line.trim().starts_with("--"))
                .collect::<Vec<_>>()
                .join("\n");
            let statement = statement.trim();

            if statement.is_empty() {
                continue;
            }

            debug!(
                "Executing: {}...",
                &statement[..statement.len().min(50)]
            );

            self.session
                .query_unpaged(statement, &[])
                .await
                .map_err(|e| {
                    error!("Statement failed: {}", e);
                    DbError::Migration(format!("Migration {} failed: {}", migration.version, e))
                })?;
        }

        // Record or remove from tracking table
        match direction {
            MigrationDirection::Up => {
                let checksum = self.compute_checksum(&migration.up_sql);
                self.tracker
                    .record_migration(migration.version, &migration.name, &checksum)
                    .await?;
            }
            MigrationDirection::Down => {
                self.tracker.remove_migration(migration.version).await?;
            }
        }

        info!("Migration {} completed successfully", migration.version);
        Ok(())
    }

    /// Rollback the last N migrations
    pub async fn rollback(&self, count: usize) -> DbResult<Vec<MigrationResult>> {
        info!("Rolling back {} migrations...", count);

        // Ensure we're using the correct keyspace
        if let Err(e) = self.ensure_keyspace().await {
            warn!("Could not ensure keyspace (may not exist yet): {}", e);
        }

        // Initialize tracker first to ensure schema_migrations exists
        if let Err(e) = self.tracker.init().await {
            warn!("Could not initialize tracker: {}", e);
            // If we can't even init the tracker, there's nothing to rollback
            return Ok(vec![]);
        }

        let applied = match self.tracker.get_applied_versions().await {
            Ok(versions) => versions,
            Err(e) => {
                warn!("Could not get applied versions: {}", e);
                return Ok(vec![]);
            }
        };

        if applied.is_empty() {
            info!("No migrations to rollback");
            return Ok(vec![]);
        }

        let all_migrations = self.loader.load_all()?;

        info!("Applied migrations to rollback: {:?}", applied);

        let mut results = Vec::new();

        // Rollback in reverse order
        for version in applied.iter().rev().take(count) {
            info!("Processing rollback for version {}", version);
            let migration = all_migrations.iter().find(|m| m.version == *version);

            if let Some(migration) = migration {
                // Try to run the down migration, but don't fail if it doesn't work
                let down_result = self
                    .run_migration_without_tracking(migration, MigrationDirection::Down)
                    .await;

                if let Err(ref e) = down_result {
                    info!("Down migration {} failed (may be ok): {}", version, e);
                }

                results.push(MigrationResult {
                    version: migration.version,
                    name: migration.name.clone(),
                    direction: MigrationDirection::Down,
                    success: down_result.is_ok(),
                    error: down_result.err().map(|e| e.to_string()),
                });
            } else {
                info!("No migration file found for version {}, will remove record", version);
            }

            // Always remove the migration record, even if down migration failed or file not found
            // This ensures we can re-run migrations from scratch
            match self.tracker.remove_migration(*version).await {
                Ok(_) => info!("Removed migration record {}", version),
                Err(e) => warn!("Failed to remove migration record {}: {}", version, e),
            }
        }

        Ok(results)
    }

    /// Run migration SQL without updating tracker (used by rollback)
    async fn run_migration_without_tracking(
        &self,
        migration: &Migration,
        direction: MigrationDirection,
    ) -> DbResult<()> {
        let sql = match direction {
            MigrationDirection::Up => &migration.up_sql,
            MigrationDirection::Down => migration
                .down_sql
                .as_ref()
                .ok_or_else(|| DbError::Migration("No down migration available".to_string()))?,
        };

        info!(
            "Running migration {} - {} ({:?})",
            migration.version, migration.name, direction
        );

        for statement in sql.split(';') {
            let statement = statement.trim();
            if statement.is_empty() || statement.starts_with("--") {
                continue;
            }

            debug!(
                "Executing: {}...",
                &statement[..statement.len().min(50)]
            );

            self.session
                .query_unpaged(statement, &[])
                .await
                .map_err(|e| {
                    error!("Statement failed: {}", e);
                    DbError::Migration(format!("Migration {} failed: {}", migration.version, e))
                })?;
        }

        info!("Migration {} completed successfully", migration.version);
        Ok(())
    }

    /// Get migration status
    pub async fn status(&self) -> DbResult<Vec<(u32, String, bool)>> {
        let all_migrations = self.loader.load_all()?;
        let applied = self.tracker.get_applied_versions().await?;

        Ok(all_migrations
            .iter()
            .map(|m| (m.version, m.name.clone(), applied.contains(&m.version)))
            .collect())
    }

    fn compute_checksum(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
