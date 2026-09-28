//! Standalone seeder script for importing rewards from chunk files
//!
//! Usage:
//!   cargo run --bin seed-rewards --features ssr
//!
//! Environment variables:
//!   SCYLLA_DB_HOST - ScyllaDB host (default: 127.0.0.1)
//!   SCYLLA_DB_PORT - ScyllaDB port (default: 9042)
//!   SCYLLA_DB_KEYSPACE - Database keyspace (default: unity_dashboard)
//!   REWARDS_CHUNKS_DIR - Directory containing chunk files (default: ./data/rewards)

use chrono::{DateTime, Utc};
use scylla::frame::value::CqlTimestamp;
use scylla::{Session, SessionBuilder};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::Arc;

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
    pub task_metadata: Option<serde_json::Value>,
    pub completed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub amount_micros: i64,
}

/// Chunk file format
#[derive(Debug, Deserialize)]
pub struct RewardChunk {
    pub chunk_index: u32,
    pub record_count: usize,
    pub date_range: ChunkDateRange,
    pub records: Vec<RewardJson>,
}

#[derive(Debug, Deserialize)]
pub struct ChunkDateRange {
    pub newest: String,
    pub oldest: String,
}

struct RewardsSeeder {
    session: Arc<Session>,
    keyspace: String,
}

impl RewardsSeeder {
    async fn new(host: &str, port: u16, keyspace: &str) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Connecting to ScyllaDB at {}:{}...", host, port);

        let session = SessionBuilder::new()
            .known_node(format!("{}:{}", host, port))
            .build()
            .await?;

        // Use keyspace
        session
            .query_unpaged(format!("USE {}", keyspace), &[])
            .await?;

        println!("Connected to keyspace: {}", keyspace);

        Ok(Self {
            session: Arc::new(session),
            keyspace: keyspace.to_string(),
        })
    }

    async fn get_existing_ids(&self) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
        let query = format!("SELECT id FROM {}.rewards", self.keyspace);
        let result = self.session.query_unpaged(query, &[]).await?;

        let mut ids = HashSet::new();

        if let Ok(rows_result) = result.into_rows_result() {
            if let Ok(rows) = rows_result.rows::<(String,)>() {
                for row_result in rows {
                    if let Ok((id,)) = row_result {
                        ids.insert(id);
                    }
                }
            }
        }

        Ok(ids)
    }

    async fn insert_reward(&self, reward: &RewardJson) -> Result<(), Box<dyn std::error::Error>> {
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
        let task_metadata_str: Option<String> = reward.task_metadata.as_ref().map(|v| v.to_string());

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
            .await?;

        Ok(())
    }

    async fn seed_from_chunks(&self, chunks_dir: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let dir_path = Path::new(chunks_dir);
        if !dir_path.exists() || !dir_path.is_dir() {
            return Err(format!("Chunks directory not found: {}", chunks_dir).into());
        }

        println!("Loading rewards from: {}", chunks_dir);

        // Find chunk files
        let mut chunk_files: Vec<_> = fs::read_dir(dir_path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let name = entry.file_name().to_string_lossy().to_string();
                name.starts_with("rewards_chunk_") && name.ends_with(".json")
            })
            .collect();

        chunk_files.sort_by_key(|e| e.file_name());

        if chunk_files.is_empty() {
            return Err("No chunk files found".into());
        }

        println!("Found {} chunk files", chunk_files.len());

        // Get existing IDs
        let existing_ids = self.get_existing_ids().await?;
        println!("Found {} existing rewards in database", existing_ids.len());

        let mut total_inserted = 0;
        let mut total_skipped = 0;
        let mut total_errors = 0;

        for entry in chunk_files {
            let file_path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            let content = fs::read_to_string(&file_path)?;
            let chunk: RewardChunk = serde_json::from_str(&content)?;

            let mut chunk_inserted = 0;
            let mut chunk_skipped = 0;

            for reward in &chunk.records {
                if existing_ids.contains(&reward.id) {
                    chunk_skipped += 1;
                    continue;
                }

                match self.insert_reward(reward).await {
                    Ok(_) => chunk_inserted += 1,
                    Err(e) => {
                        eprintln!("Error inserting {}: {}", reward.id, e);
                        total_errors += 1;
                    }
                }
            }

            total_inserted += chunk_inserted;
            total_skipped += chunk_skipped;

            println!(
                "Chunk {} ({}): +{} inserted, {} skipped | Total: {}",
                chunk.chunk_index, file_name, chunk_inserted, chunk_skipped, total_inserted
            );
        }

        println!("\n=== COMPLETE ===");
        println!("Inserted: {}", total_inserted);
        println!("Skipped (duplicates): {}", total_skipped);
        println!("Errors: {}", total_errors);

        Ok(total_inserted)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let host = std::env::var("SCYLLA_DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("SCYLLA_DB_PORT")
        .unwrap_or_else(|_| "9042".to_string())
        .parse()
        .unwrap_or(9042);
    let keyspace = std::env::var("SCYLLA_DB_KEYSPACE").unwrap_or_else(|_| "unity_dashboard".to_string());
    let chunks_dir = std::env::var("REWARDS_CHUNKS_DIR").unwrap_or_else(|_| "./data/rewards".to_string());

    println!("=== Rewards Seeder ===");
    println!("Host: {}:{}", host, port);
    println!("Keyspace: {}", keyspace);
    println!("Chunks dir: {}", chunks_dir);
    println!();

    let seeder = RewardsSeeder::new(&host, port, &keyspace).await?;
    seeder.seed_from_chunks(&chunks_dir).await?;

    Ok(())
}
