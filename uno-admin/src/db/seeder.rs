//! Database seeder for loading initial data from JSON files

use crate::db::error::{DbError, DbResult};
use chrono::{DateTime, Utc};
use scylla::frame::value::CqlTimestamp;
use scylla::Session;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// Reward record from JSON
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardJson {
    pub id: String,
    pub user_id: String,
    #[serde(rename = "type")]
    pub reward_type: String,
    pub description: Option<String>,
    pub node_id: String,
    pub license_id: String,
    pub license_lease_id: String,
    pub task_key: Option<String>,
    /// taskMetadata can be a string or an object, so we store it as JSON Value
    pub task_metadata: Option<serde_json::Value>,
    pub completed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub amount_micros: i64,
}

/// Chunk file format for batch reward imports
#[derive(Debug, Deserialize)]
pub struct RewardChunk {
    pub chunk_index: u32,
    pub record_count: usize,
    pub date_range: ChunkDateRange,
    pub records: Vec<RewardJson>,
}

/// Date range metadata for a chunk
#[derive(Debug, Deserialize)]
pub struct ChunkDateRange {
    pub newest: String,
    pub oldest: String,
}

pub struct DatabaseSeeder {
    session: Arc<Session>,
    keyspace: String,
}

impl DatabaseSeeder {
    pub fn new(session: Arc<Session>, keyspace: &str) -> Self {
        Self {
            session,
            keyspace: keyspace.to_string(),
        }
    }

    /// Seed rewards from JSON file
    pub async fn seed_rewards(&self, json_path: &str) -> DbResult<usize> {
        let path = Path::new(json_path);
        if !path.exists() {
            warn!("Rewards JSON file not found: {}", json_path);
            return Ok(0);
        }

        info!("Loading rewards from: {}", json_path);

        // Read and parse JSON
        let content = std::fs::read_to_string(path)
            .map_err(|e| DbError::Config(format!("Failed to read rewards file: {}", e)))?;

        let rewards: Vec<RewardJson> = serde_json::from_str(&content)
            .map_err(|e| DbError::Config(format!("Failed to parse rewards JSON: {}", e)))?;

        info!("Found {} rewards to seed", rewards.len());

        // Check if rewards table already has data
        let count = self.get_rewards_count().await?;
        if count > 0 {
            info!("Rewards table already has {} records, skipping seed", count);
            return Ok(0);
        }

        // Insert each reward
        let mut inserted = 0;
        for reward in &rewards {
            match self.insert_reward(reward).await {
                Ok(_) => {
                    inserted += 1;
                    if inserted % 100 == 0 {
                        debug!("Inserted {} rewards...", inserted);
                    }
                }
                Err(e) => {
                    error!("Failed to insert reward {}: {}", reward.id, e);
                }
            }
        }

        info!("Seeded {} rewards successfully", inserted);
        Ok(inserted)
    }

    /// Seed rewards from chunk files in a directory
    /// Chunk files are expected to be named rewards_chunk_XXXX.json
    pub async fn seed_rewards_from_chunks(&self, chunks_dir: &str) -> DbResult<usize> {
        let dir_path = Path::new(chunks_dir);
        if !dir_path.exists() || !dir_path.is_dir() {
            warn!("Rewards chunks directory not found: {}", chunks_dir);
            return Ok(0);
        }

        info!("Loading rewards from chunks directory: {}", chunks_dir);

        // Find all chunk files
        let mut chunk_files: Vec<_> = fs::read_dir(dir_path)
            .map_err(|e| DbError::Config(format!("Failed to read chunks directory: {}", e)))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with("rewards_chunk_")
                    && entry.file_name().to_string_lossy().ends_with(".json")
            })
            .collect();

        // Sort by filename to process in order
        chunk_files.sort_by_key(|e| e.file_name());

        if chunk_files.is_empty() {
            warn!("No chunk files found in: {}", chunks_dir);
            return Ok(0);
        }

        info!("Found {} chunk files to process", chunk_files.len());

        // Get existing reward IDs for duplicate detection
        let existing_ids = self.get_existing_reward_ids().await?;
        info!(
            "Found {} existing rewards in database",
            existing_ids.len()
        );

        let mut total_inserted = 0;
        let mut total_skipped = 0;

        for (chunk_num, entry) in chunk_files.iter().enumerate() {
            let file_path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            debug!("Processing chunk {}: {}", chunk_num, file_name);

            // Read and parse chunk file
            let content = match fs::read_to_string(&file_path) {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to read chunk file {}: {}", file_name, e);
                    continue;
                }
            };

            let chunk: RewardChunk = match serde_json::from_str(&content) {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to parse chunk file {}: {}", file_name, e);
                    continue;
                }
            };

            let mut chunk_inserted = 0;
            let mut chunk_skipped = 0;

            for reward in &chunk.records {
                // Skip if already exists
                if existing_ids.contains(&reward.id) {
                    chunk_skipped += 1;
                    continue;
                }

                match self.insert_reward(reward).await {
                    Ok(_) => {
                        chunk_inserted += 1;
                    }
                    Err(e) => {
                        error!("Failed to insert reward {}: {}", reward.id, e);
                    }
                }
            }

            total_inserted += chunk_inserted;
            total_skipped += chunk_skipped;

            info!(
                "Chunk {} ({}): inserted {}, skipped {} duplicates | Total: {} inserted",
                chunk.chunk_index,
                file_name,
                chunk_inserted,
                chunk_skipped,
                total_inserted
            );
        }

        info!(
            "Completed seeding from chunks: {} inserted, {} skipped (duplicates)",
            total_inserted, total_skipped
        );

        Ok(total_inserted)
    }

    /// Get all existing reward IDs for duplicate detection
    async fn get_existing_reward_ids(&self) -> DbResult<std::collections::HashSet<String>> {
        let query = format!("SELECT id FROM {}.rewards", self.keyspace);

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        let mut ids = std::collections::HashSet::new();

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(ids),
        };

        let rows = match rows_result.rows::<(String,)>() {
            Ok(r) => r,
            Err(_) => return Ok(ids),
        };

        for row_result in rows {
            if let Ok((id,)) = row_result {
                ids.insert(id);
            }
        }

        Ok(ids)
    }

    /// Get count of existing rewards
    async fn get_rewards_count(&self) -> DbResult<i64> {
        let query = format!("SELECT COUNT(*) FROM {}.rewards", self.keyspace);

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        let mut rows = match rows_result.rows::<(i64,)>() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        if let Some(Ok((count,))) = rows.next() {
            Ok(count)
        } else {
            Ok(0)
        }
    }

    /// Insert a single reward record
    async fn insert_reward(&self, reward: &RewardJson) -> DbResult<()> {
        let query = format!(
            "INSERT INTO {}.rewards (
                id, user_id, type, description, node_id, license_id,
                license_lease_id, task_key, task_metadata, completed_at,
                created_at, amount_micros
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            self.keyspace
        );

        let completed_at = CqlTimestamp(reward.completed_at.timestamp_millis());
        let created_at = CqlTimestamp(reward.created_at.timestamp_millis());

        // Convert task_metadata JSON value to string if present
        let task_metadata_str: Option<String> = reward
            .task_metadata
            .as_ref()
            .map(|v| v.to_string());

        self.session
            .query_unpaged(
                query,
                (
                    &reward.id,
                    &reward.user_id,
                    &reward.reward_type,
                    &reward.description,
                    &reward.node_id,
                    &reward.license_id,
                    &reward.license_lease_id,
                    &reward.task_key,
                    &task_metadata_str,
                    completed_at,
                    created_at,
                    reward.amount_micros,
                ),
            )
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Seed all data from configured paths
    pub async fn seed_all(&self) -> DbResult<()> {
        // Get rewards file path from env or use default
        let rewards_path = std::env::var("REWARDS_JSON_PATH")
            .unwrap_or_else(|_| "./data/incentives/rewards.json".to_string());

        self.seed_rewards(&rewards_path).await?;

        // Also seed from chunks directory if configured
        let chunks_dir = std::env::var("REWARDS_CHUNKS_DIR")
            .unwrap_or_else(|_| "./data/rewards".to_string());

        if Path::new(&chunks_dir).exists() {
            self.seed_rewards_from_chunks(&chunks_dir).await?;
        }

        Ok(())
    }
}
