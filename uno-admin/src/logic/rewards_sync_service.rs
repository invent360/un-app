//! Rewards synchronization service
//!
//! Fetches rewards from Unity API and stores them in the local database.
//!
//! Supports both ScyllaDB (legacy) and PostgreSQL backends via feature flags.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::api::config::{UnityApiConfig, DEFAULT_API_KEY, DEFAULT_BASE_URL};
use crate::api::types::RewardsAllocationRequest;
use crate::db::DbPool;
use crate::models::entity::{LicenseRewardContext, NewReward, NewSyncJob, SyncJobContext, SyncJobEntity};
use crate::repository::traits::{LicenseRepositoryTrait, RewardRepositoryTrait, SyncJobRepositoryTrait};

// Import the appropriate repositories based on feature flag
#[cfg(feature = "postgres-db")]
use crate::repository::postgres::{
    PgLicenseRepository as LicenseRepository,
    PgRewardRepository as RewardRepository,
    PgSyncJobRepository as SyncJobRepository,
};
#[cfg(not(feature = "postgres-db"))]
use crate::repository::scylla::{LicenseRepository, RewardRepository, SyncJobRepository};

#[cfg(feature = "ssr")]
use crate::ws::{broadcast_job_update, broadcast_job_deleted};

/// Reward allocation from the rewards_get_allocations RPC endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardAllocationRpc {
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
    pub completed_at: String,
    pub created_at: String,
    pub amount_micros: i64,
}

/// Service for synchronizing rewards from UNO API
pub struct RewardsSyncService {
    job_repo: SyncJobRepository,
    reward_repo: RewardRepository,
    license_repo: LicenseRepository,
}

impl RewardsSyncService {
    /// Create a new RewardsSyncService
    pub fn new(pool: DbPool) -> Self {
        Self {
            job_repo: SyncJobRepository::new(pool.clone()),
            reward_repo: RewardRepository::new(pool.clone()),
            license_repo: LicenseRepository::new(pool),
        }
    }

    /// Main sync function - synchronizes rewards for a specific date
    ///
    /// This function always calls the API to check for new rewards.
    /// The upsert to the database is idempotent - duplicate rewards won't be created.
    /// Each call creates a new job row for visibility in the UI.
    pub async fn sync_rewards_for_date(&self, target_date: &str) -> Result<SyncJobEntity, String> {
        info!("Starting rewards sync for date: {}", target_date);

        // 1. Check if any job is currently running for this date (prevent concurrent execution)
        let existing_jobs = self.job_repo.list_jobs_by_status("running").await?;
        for job in existing_jobs {
            if job.target_date == target_date && job.job_type == "rewards_sync" {
                return Err(format!("Job already running for date: {}", target_date));
            }
        }

        // 2. Create a new job for this run
        let job = self.create_or_reset_job(target_date).await?;
        let job_id = job.id.clone();

        // 3. Mark as running and broadcast
        self.job_repo.mark_running(&job_id).await?;

        // Broadcast running status via WebSocket
        #[cfg(feature = "ssr")]
        if let Ok(Some(running_job)) = self.job_repo.get_job_by_id(&job_id).await {
            broadcast_job_update(&running_job);
        }

        // 4. Fetch and store rewards with timing
        let start_time = Instant::now();
        match self.execute_sync(&job_id, target_date).await {
            Ok((fetched, inserted, context_json, logs)) => {
                let duration_ms = start_time.elapsed().as_millis() as i64;
                info!(
                    "Sync completed for {}: {} fetched, {} inserted, took {}ms",
                    target_date, fetched, inserted, duration_ms
                );
                self.job_repo.mark_completed_with_context(
                    &job_id,
                    fetched as i32,
                    inserted as i32,
                    Some(&context_json),
                    duration_ms,
                    Some(&logs),
                ).await?;

                // Broadcast completed status via WebSocket
                #[cfg(feature = "ssr")]
                if let Ok(Some(completed_job)) = self.job_repo.get_job_by_id(&job_id).await {
                    broadcast_job_update(&completed_job);
                }
            }
            Err(e) => {
                error!("Sync failed for {}: {}", target_date, e);
                self.handle_failure(&job, &e).await?;

                // Broadcast failed status via WebSocket
                #[cfg(feature = "ssr")]
                if let Ok(Some(failed_job)) = self.job_repo.get_job_by_id(&job_id).await {
                    broadcast_job_update(&failed_job);
                }
            }
        }

        // Return updated job
        self.job_repo
            .get_job_by_id(&job_id)
            .await?
            .ok_or_else(|| "Job not found after execution".to_string())
    }

    /// Create a new job for each sync run
    /// Each scheduler tick creates a new job row for visibility
    async fn create_or_reset_job(&self, target_date: &str) -> Result<SyncJobEntity, String> {
        // Always create a new job for each run - this allows seeing job history
        let new_job = NewSyncJob::rewards_sync(target_date.to_string());
        self.job_repo.create_job(new_job).await
    }

    /// Helper to format a log line with timestamp
    fn log_line(msg: &str) -> String {
        let ts = Utc::now().format("%H:%M:%S%.3f");
        format!("[{}] {}", ts, msg)
    }

    /// Execute the actual sync operation
    /// Fetches all rewards from the rewards_get_allocations endpoint and filters by date
    /// Returns (fetched_count, inserted_count, context_json, logs)
    async fn execute_sync(&self, _job_id: &str, target_date: &str) -> Result<(usize, usize, String, String), String> {
        let mut logs = Vec::new();

        logs.push(Self::log_line(&format!("Starting rewards sync for date: {}", target_date)));

        // Get API config from environment
        let config = UnityApiConfig::from_env();
        if !config.has_token() {
            logs.push(Self::log_line("ERROR: API_TOKEN environment variable not set"));
            return Err("API_TOKEN environment variable not set".to_string());
        }
        logs.push(Self::log_line("API configuration loaded successfully"));

        info!("Fetching rewards from API for date: {}", target_date);

        // Fetch all allocations from the RPC endpoint (paginated batch-by-batch)
        let allocations = self.fetch_allocations_from_api(&config, &mut logs).await?;

        // Filter by target date (using completedAt field)
        logs.push(Self::log_line(&format!("Filtering allocations for target date: {}", target_date)));
        let filtered_allocations: Vec<RewardAllocationRpc> = allocations
            .into_iter()
            .filter(|r| {
                // Extract date from completedAt (format: "2026-06-07T11:06:07.749+00:00")
                r.completed_at
                    .split('T')
                    .next()
                    .map(|d| d == target_date)
                    .unwrap_or(false)
            })
            .collect();

        let fetched_count = filtered_allocations.len();
        info!("Found {} rewards for date {}", fetched_count, target_date);
        logs.push(Self::log_line(&format!("Found {} rewards matching target date", fetched_count)));

        if fetched_count == 0 {
            // No rewards found for this date
            logs.push(Self::log_line("WARN: No rewards found for target date - rewards may not be available yet"));
            return Err(format!("No rewards found for target date {} - rewards may not be available yet", target_date));
        }

        // Build context with per-license breakdown
        logs.push(Self::log_line("Building reward context with per-license breakdown..."));
        let license_rewards: Vec<LicenseRewardContext> = filtered_allocations
            .iter()
            .map(|r| LicenseRewardContext {
                license_id: r.license_id.clone(),
                amount_micros: r.amount_micros,
            })
            .collect();

        let total_amount_micros: i64 = license_rewards.iter().map(|r| r.amount_micros).sum();
        let unique_licenses: usize = license_rewards
            .iter()
            .map(|r| &r.license_id)
            .collect::<HashSet<_>>()
            .len();

        logs.push(Self::log_line(&format!(
            "Context: {} unique licenses, total {} micros ({:.4} UNO)",
            unique_licenses,
            total_amount_micros,
            total_amount_micros as f64 / 1_000_000.0
        )));

        let context = SyncJobContext {
            license_rewards,
            total_amount_micros,
            unique_licenses,
        };

        let context_json = serde_json::to_string(&context)
            .map_err(|e| format!("Failed to serialize context: {}", e))?;

        // Convert to NewReward and upsert
        logs.push(Self::log_line("Converting rewards to database format..."));
        let filtered_rewards: Vec<NewReward> = filtered_allocations
            .into_iter()
            .map(|r| self.convert_rpc_reward_to_new_reward(r))
            .collect();

        // Upsert rewards (idempotent)
        logs.push(Self::log_line("Upserting rewards to database..."));
        let inserted_count = self.reward_repo.upsert_rewards(filtered_rewards).await?;
        logs.push(Self::log_line(&format!("Upserted {} rewards to database", inserted_count)));

        logs.push(Self::log_line("Sync completed successfully"));

        let logs_string = logs.join("\n");
        Ok((fetched_count, inserted_count, context_json, logs_string))
    }

    /// Fetch all allocations from the rewards_get_allocations RPC endpoint
    /// Uses batch pagination to avoid overloading the API
    async fn fetch_allocations_from_api(
        &self,
        config: &UnityApiConfig,
        logs: &mut Vec<String>,
    ) -> Result<Vec<RewardAllocationRpc>, String> {
        let client = crate::api::http_client::get_client();
        let url = format!("{}/rest/v1/rpc/rewards_get_allocations", config.base_url);

        const BATCH_SIZE: u32 = 100;
        const BATCH_DELAY_MS: u64 = 250;

        let mut all_allocations: Vec<RewardAllocationRpc> = Vec::new();
        let mut request = RewardsAllocationRequest::first_page(BATCH_SIZE);
        let mut batch_num = 0u32;

        debug!("Starting paginated fetch from {}", url);
        logs.push(Self::log_line(&format!(
            "Starting paginated fetch (batch_size={})",
            BATCH_SIZE
        )));

        loop {
            batch_num += 1;
            logs.push(Self::log_line(&format!(
                "Fetching batch {} (skip={}, take={})",
                batch_num, request.skip, request.take
            )));

            let response = client
                .post(&url)
                .header("apikey", &config.api_key)
                .header("Authorization", format!("Bearer {}", config.jwt_token))
                .header("Content-Type", "application/json")
                .header("Content-Profile", "public")
                .json(&request)
                .send()
                .await
                .map_err(|e| format!("Failed to call rewards API on batch {}: {}", batch_num, e))?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                let err = format!("API returned error {} on batch {}: {}", status, batch_num, body);
                logs.push(Self::log_line(&format!("ERROR: {}", err)));
                return Err(err);
            }

            let batch: Vec<RewardAllocationRpc> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse rewards response on batch {}: {}", batch_num, e))?;

            let batch_count = batch.len();
            logs.push(Self::log_line(&format!(
                "Batch {}: received {} records (total so far: {})",
                batch_num,
                batch_count,
                all_allocations.len() + batch_count
            )));

            // Termination check: if we got fewer than batch_size, this is the last page
            let is_last_batch = (batch_count as u32) < BATCH_SIZE;

            // Append results
            all_allocations.extend(batch);

            if is_last_batch {
                logs.push(Self::log_line(&format!(
                    "Pagination complete: {} total records in {} batches",
                    all_allocations.len(),
                    batch_num
                )));
                break;
            }

            // Delay before next batch to avoid overwhelming API
            sleep(Duration::from_millis(BATCH_DELAY_MS)).await;

            // Advance to next page
            request = request.next_page();
        }

        info!("Fetched {} total allocations in {} batches", all_allocations.len(), batch_num);

        Ok(all_allocations)
    }

    /// Convert RPC reward allocation to internal NewReward format
    fn convert_rpc_reward_to_new_reward(&self, rpc_reward: RewardAllocationRpc) -> NewReward {
        NewReward {
            id: Some(rpc_reward.id),
            user_id: rpc_reward.user_id,
            reward_type: rpc_reward.reward_type,
            description: rpc_reward.description,
            node_id: rpc_reward.node_id,
            license_id: rpc_reward.license_id,
            license_lease_id: rpc_reward.license_lease_id,
            task_key: rpc_reward.task_key,
            task_metadata: rpc_reward.task_metadata.map(|v| v.to_string()),
            completed_at: rpc_reward.completed_at,
            amount_micros: rpc_reward.amount_micros,
        }
    }

    /// Handle sync failure - mark as failed (no automatic retry)
    async fn handle_failure(&self, job: &SyncJobEntity, error: &str) -> Result<(), String> {
        error!("Job {} failed: {}", job.id, error);
        self.job_repo.mark_failed(&job.id, error).await?;
        Ok(())
    }

    /// Process all pending retry jobs
    pub async fn process_pending_retries(&self) -> Result<usize, String> {
        let pending_jobs = self.job_repo.list_pending_retries().await?;

        if pending_jobs.is_empty() {
            return Ok(0);
        }

        info!("Processing {} pending retry jobs", pending_jobs.len());

        let mut processed = 0;
        for job in pending_jobs {
            match self.sync_rewards_for_date(&job.target_date).await {
                Ok(_) => processed += 1,
                Err(e) => warn!("Retry failed for job {}: {}", job.id, e),
            }
        }

        Ok(processed)
    }

    /// Re-run a job by ID (resets and re-executes)
    pub async fn rerun_job(&self, job_id: &str) -> Result<SyncJobEntity, String> {
        let job = self
            .job_repo
            .get_job_by_id(job_id)
            .await?
            .ok_or_else(|| format!("Job not found: {}", job_id))?;

        // Reset the job
        self.job_repo.reset_job(job_id).await?;

        // Re-run sync
        self.sync_rewards_for_date(&job.target_date).await
    }

    /// Delete a job by ID
    pub async fn delete_job(&self, job_id: &str) -> Result<bool, String> {
        let result = self.job_repo.delete_job(job_id).await?;

        // Broadcast deletion via WebSocket
        #[cfg(feature = "ssr")]
        if result {
            broadcast_job_deleted(job_id);
        }

        Ok(result)
    }

    /// List all sync jobs
    pub async fn list_jobs(&self) -> Result<Vec<SyncJobEntity>, String> {
        self.job_repo.list_jobs().await
    }

    /// Get a job by ID
    pub async fn get_job(&self, job_id: &str) -> Result<Option<SyncJobEntity>, String> {
        self.job_repo.get_job_by_id(job_id).await
    }

    // =========================================================================
    // Background Job Execution Methods (for job_worker)
    // =========================================================================

    /// Create a pending job without executing it
    ///
    /// Used by HTTP handlers to create a job that will be executed by
    /// the background worker. Returns the job in "pending" status.
    pub async fn create_pending_job(&self, target_date: &str) -> Result<SyncJobEntity, String> {
        info!("Creating pending rewards sync job for date: {}", target_date);

        // Check if any job is currently running for this date
        let existing_jobs = self.job_repo.list_jobs_by_status("running").await?;
        for job in existing_jobs {
            if job.target_date == target_date && job.job_type == "rewards_sync" {
                return Err(format!("Job already running for date: {}", target_date));
            }
        }

        // Create a new pending job
        let new_job = NewSyncJob::rewards_sync(target_date.to_string());
        let job = self.job_repo.create_job(new_job).await?;

        // Broadcast pending job via WebSocket
        #[cfg(feature = "ssr")]
        broadcast_job_update(&job);

        Ok(job)
    }

    /// Execute an existing job (called by background worker)
    ///
    /// Takes a job ID and target date, marks the job as running,
    /// executes the sync, and marks as completed or failed.
    pub async fn execute_job(&self, job_id: &str, target_date: &str) -> Result<SyncJobEntity, String> {
        info!("Executing rewards sync job {} for date: {}", job_id, target_date);

        // Mark as running
        self.job_repo.mark_running(job_id).await?;

        // Broadcast running status
        #[cfg(feature = "ssr")]
        if let Ok(Some(running_job)) = self.job_repo.get_job_by_id(job_id).await {
            broadcast_job_update(&running_job);
        }

        // Execute with timing
        let start_time = Instant::now();
        match self.execute_sync(job_id, target_date).await {
            Ok((fetched, inserted, context_json, logs)) => {
                let duration_ms = start_time.elapsed().as_millis() as i64;
                info!(
                    "Sync completed for job {}: {} fetched, {} inserted, took {}ms",
                    job_id, fetched, inserted, duration_ms
                );
                self.job_repo
                    .mark_completed_with_context(
                        job_id,
                        fetched as i32,
                        inserted as i32,
                        Some(&context_json),
                        duration_ms,
                        Some(&logs),
                    )
                    .await?;

                // Broadcast completed status
                #[cfg(feature = "ssr")]
                if let Ok(Some(completed_job)) = self.job_repo.get_job_by_id(job_id).await {
                    broadcast_job_update(&completed_job);
                }
            }
            Err(e) => {
                error!("Sync failed for job {}: {}", job_id, e);
                self.job_repo.mark_failed(job_id, &e).await?;

                // Broadcast failed status
                #[cfg(feature = "ssr")]
                if let Ok(Some(failed_job)) = self.job_repo.get_job_by_id(job_id).await {
                    broadcast_job_update(&failed_job);
                }
            }
        }

        // Return updated job
        self.job_repo
            .get_job_by_id(job_id)
            .await?
            .ok_or_else(|| "Job not found after execution".to_string())
    }

    /// Reset a job and execute it (for re-runs)
    ///
    /// Resets the job to pending status, then executes it.
    pub async fn reset_and_execute(&self, job_id: &str, target_date: &str) -> Result<SyncJobEntity, String> {
        info!("Resetting and executing job {} for date: {}", job_id, target_date);

        // Reset job to pending
        self.job_repo.reset_job(job_id).await?;

        // Broadcast reset
        #[cfg(feature = "ssr")]
        if let Ok(Some(reset_job)) = self.job_repo.get_job_by_id(job_id).await {
            broadcast_job_update(&reset_job);
        }

        // Execute the job
        self.execute_job(job_id, target_date).await
    }
}
