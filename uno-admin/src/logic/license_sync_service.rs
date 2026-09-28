//! License synchronization service
//!
//! Fetches licenses from Unetwork API and syncs to local ScyllaDB.
//! Runs every 10 minutes, tracks progress for resume on failure.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{debug, error, info, warn};

use crate::db::DbPool;
use crate::models::entity::{NewLicenseFromApi, NewSyncJob, SyncJobEntity};
use crate::repository::scylla::{LicenseRepository, SyncJobRepository};
use crate::repository::traits::{LicenseRepositoryTrait, SyncJobRepositoryTrait};

#[cfg(feature = "ssr")]
use crate::ws::broadcast_job_update;

#[cfg(feature = "ssr")]
use ember_multichain::siwe::unetwork;

/// Context for license sync job (stored as JSON in job_context)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseSyncContext {
    pub total_fetched: usize,
    pub total_updated: usize,
    pub total_inserted: usize,
    pub last_processed_page: i32,
    pub total_pages: i32,
    pub unique_licenses: usize,
}

impl Default for LicenseSyncContext {
    fn default() -> Self {
        Self {
            total_fetched: 0,
            total_updated: 0,
            total_inserted: 0,
            last_processed_page: 0,
            total_pages: 0,
            unique_licenses: 0,
        }
    }
}

/// Request payload for paginated licenses API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LicenseApiRequest {
    pub role: String,
    pub page: i32,
    pub page_size: i32,
    pub skip: i32,
    pub take: i32,
}

impl LicenseApiRequest {
    fn new(page: i32, page_size: i32) -> Self {
        Self {
            role: "uno".to_string(),
            page,
            page_size,
            skip: (page - 1) * page_size,
            take: page_size,
        }
    }
}

/// License item from API response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LicenseApiItem {
    pub id: String,
    pub node_id: Option<String>,
    pub owner_wallet_address: Option<String>,
    pub alias: Option<String>,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub activation_start_at: Option<String>,
    pub activation_end_at: Option<String>,
    pub activation_by: Option<String>,
    pub activation_postponed_ms: Option<i64>,
    pub lease_user_id: Option<String>,
    pub lease_share_percentage: Option<f64>,
    pub lease_min_uptime_percentage: Option<f64>,
    pub lease_from: Option<String>,
    pub lease_to: Option<String>,
    pub validation_last_success_at: Option<String>,
    pub uptime: Option<f64>,
    #[serde(default)]
    pub settings: Option<serde_json::Value>,
    pub total_count: i64,
    pub is_online: Option<bool>,
}

impl From<LicenseApiItem> for NewLicenseFromApi {
    fn from(item: LicenseApiItem) -> Self {
        Self {
            license_id: item.id,
            node_id: item.node_id.unwrap_or_default(),
            owner_wallet_address: item.owner_wallet_address,
            alias: item.alias,
            device_id: item.device_id,
            device_name: item.device_name,
            activation_start_at: item.activation_start_at,
            activation_end_at: item.activation_end_at,
            activation_by: item.activation_by,
            activation_postponed_ms: item.activation_postponed_ms,
            lease_user_id: item.lease_user_id,
            lease_share_percentage: item.lease_share_percentage.unwrap_or(0.0),
            lease_min_uptime_percentage: item.lease_min_uptime_percentage.unwrap_or(0.0),
            lease_from: item.lease_from,
            lease_to: item.lease_to,
            validation_last_success_at: item.validation_last_success_at,
            uptime: item.uptime.unwrap_or(0.0),
            settings: item.settings.map(|v| v.to_string()),
            is_online: item.is_online.unwrap_or(false),
        }
    }
}

/// Service for synchronizing licenses from Unetwork API
pub struct LicenseSyncService {
    job_repo: SyncJobRepository,
    license_repo: LicenseRepository,
}

impl LicenseSyncService {
    /// Create a new LicenseSyncService
    pub fn new(pool: DbPool) -> Self {
        Self {
            job_repo: SyncJobRepository::new(pool.clone()),
            license_repo: LicenseRepository::new(pool),
        }
    }

    /// Check if a license sync job is currently running
    pub async fn is_running(&self) -> bool {
        match self.job_repo.list_jobs_by_status("running").await {
            Ok(jobs) => jobs.iter().any(|j| j.job_type == "licenses_sync"),
            Err(_) => false,
        }
    }

    /// Main sync function - synchronizes all licenses from Unetwork API
    pub async fn sync_all_licenses(&self) -> Result<SyncJobEntity, String> {
        info!("Starting license sync");

        // 1. Check if any license sync job is running
        let existing_jobs = self.job_repo.list_jobs_by_status("running").await?;
        for job in existing_jobs {
            if job.job_type == "licenses_sync" {
                return Err("License sync job already running".to_string());
            }
        }

        // 2. Create new job
        let job = self.job_repo.create_job(NewSyncJob::licenses_sync()).await?;
        let job_id = job.id.clone();

        // 3. Mark as running
        self.job_repo.mark_running(&job_id).await?;

        // Broadcast running status
        #[cfg(feature = "ssr")]
        if let Ok(Some(running_job)) = self.job_repo.get_job_by_id(&job_id).await {
            broadcast_job_update(&running_job);
        }

        // 4. Execute sync with timing
        let start_time = Instant::now();
        match self.execute_sync(&job_id).await {
            Ok((fetched, inserted, context_json, logs)) => {
                let duration_ms = start_time.elapsed().as_millis() as i64;
                info!(
                    "License sync completed: {} fetched, {} upserted, took {}ms",
                    fetched, inserted, duration_ms
                );
                self.job_repo
                    .mark_completed_with_context(
                        &job_id,
                        fetched as i32,
                        inserted as i32,
                        Some(&context_json),
                        duration_ms,
                        Some(&logs),
                    )
                    .await?;

                // Broadcast completed status
                #[cfg(feature = "ssr")]
                if let Ok(Some(completed_job)) = self.job_repo.get_job_by_id(&job_id).await {
                    broadcast_job_update(&completed_job);
                }
            }
            Err(e) => {
                error!("License sync failed: {}", e);
                self.job_repo.mark_failed(&job_id, &e).await?;

                // Broadcast failed status
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

    /// Helper to format a log line with timestamp
    fn log_line(msg: &str) -> String {
        let ts = Utc::now().format("%H:%M:%S%.3f");
        format!("[{}] {}", ts, msg)
    }

    /// Execute the sync operation
    async fn execute_sync(&self, job_id: &str) -> Result<(usize, usize, String, String), String> {
        let mut logs = Vec::new();
        logs.push(Self::log_line("Starting license sync from Unetwork API"));

        // Get JWT token
        let jwt_token = std::env::var("API_TOKEN")
            .or_else(|_| std::env::var("UNITY_JWT_TOKEN"))
            .map_err(|_| "API_TOKEN environment variable not set")?;

        if jwt_token.is_empty() {
            logs.push(Self::log_line("ERROR: JWT token is empty"));
            return Err("JWT token is empty".to_string());
        }
        logs.push(Self::log_line("JWT token loaded successfully"));

        let client = crate::api::http_client::get_client();
        let url = format!("{}/functions/v1/licenses_get_licenses", unetwork::API_URL);

        let mut all_licenses: Vec<NewLicenseFromApi> = Vec::new();
        let mut page = 1;
        let page_size = 100;
        let mut total_pages = 1;

        logs.push(Self::log_line(&format!("Fetching licenses from: {}", url)));

        // Paginate through all licenses
        loop {
            logs.push(Self::log_line(&format!("Fetching page {} (size {})", page, page_size)));

            let request = LicenseApiRequest::new(page, page_size);

            let response = client
                .post(&url)
                .header("apikey", unetwork::API_KEY)
                .header("Authorization", format!("Bearer {}", jwt_token))
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await
                .map_err(|e| {
                    let msg = format!("API request failed on page {}: {}", page, e);
                    error!("{}", msg);
                    msg
                })?;

            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                let msg = format!("API error {} on page {}: {}", status.as_u16(), page, error_text);
                logs.push(Self::log_line(&format!("ERROR: {}", msg)));
                return Err(msg);
            }

            let items: Vec<LicenseApiItem> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response on page {}: {}", page, e))?;

            if items.is_empty() {
                logs.push(Self::log_line("No more licenses to fetch"));
                break;
            }

            // Get total count from first item
            let total_count = items.first().map(|i| i.total_count).unwrap_or(0);
            total_pages = ((total_count as f64) / (page_size as f64)).ceil() as i32;
            if total_pages == 0 {
                total_pages = 1;
            }

            let batch_count = items.len();
            logs.push(Self::log_line(&format!(
                "Page {}/{}: fetched {} licenses (total: {})",
                page, total_pages, batch_count, total_count
            )));

            // Convert and collect
            for item in items {
                all_licenses.push(item.into());
            }

            // Update context in job (for resume capability)
            let context = LicenseSyncContext {
                total_fetched: all_licenses.len(),
                total_updated: 0,
                total_inserted: 0,
                last_processed_page: page,
                total_pages,
                unique_licenses: all_licenses.len(),
            };
            let _ = self
                .job_repo
                .update_job_context(job_id, &serde_json::to_string(&context).unwrap_or_default())
                .await;

            // Check if we've fetched all
            if all_licenses.len() >= total_count as usize || page >= total_pages {
                break;
            }

            page += 1;
        }

        let total_fetched = all_licenses.len();
        logs.push(Self::log_line(&format!(
            "Fetched {} total licenses from API",
            total_fetched
        )));

        if total_fetched == 0 {
            logs.push(Self::log_line("WARN: No licenses found in API"));
            let context = LicenseSyncContext {
                total_fetched: 0,
                total_updated: 0,
                total_inserted: 0,
                last_processed_page: 0,
                total_pages: 0,
                unique_licenses: 0,
            };
            return Ok((0, 0, serde_json::to_string(&context).unwrap_or_default(), logs.join("\n")));
        }

        // Upsert licenses to database
        logs.push(Self::log_line("Upserting licenses to local database..."));
        let mut upserted_count = 0;
        let mut errors = 0;

        for license in &all_licenses {
            match self.license_repo.upsert_license_from_api(license.clone()).await {
                Ok(_) => upserted_count += 1,
                Err(e) => {
                    warn!("Failed to upsert license {}: {}", license.license_id, e);
                    errors += 1;
                }
            }
        }

        logs.push(Self::log_line(&format!(
            "Upserted {} licenses ({} errors)",
            upserted_count, errors
        )));

        // Build final context
        let context = LicenseSyncContext {
            total_fetched,
            total_updated: upserted_count, // We can't distinguish update vs insert easily
            total_inserted: 0,
            last_processed_page: total_pages,
            total_pages,
            unique_licenses: total_fetched,
        };

        let context_json = serde_json::to_string(&context)
            .map_err(|e| format!("Failed to serialize context: {}", e))?;

        logs.push(Self::log_line("License sync completed successfully"));

        Ok((total_fetched, upserted_count, context_json, logs.join("\n")))
    }

    /// Resume a failed job from where it left off
    pub async fn resume_failed_job(&self, job_id: &str) -> Result<SyncJobEntity, String> {
        let job = self
            .job_repo
            .get_job_by_id(job_id)
            .await?
            .ok_or("Job not found")?;

        if job.status != "failed" {
            return Err("Job is not in failed state".to_string());
        }

        // Parse previous context to get resume point
        let prev_context: Option<LicenseSyncContext> = job
            .job_context
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok());

        if let Some(ctx) = prev_context {
            info!(
                "Resuming job {} from page {} (previously fetched {})",
                job_id, ctx.last_processed_page + 1, ctx.total_fetched
            );
        }

        // Reset job and run fresh (simpler than true resume for now)
        self.job_repo.reset_job(job_id).await?;
        self.sync_all_licenses().await
    }

    /// Rerun a specific job
    pub async fn rerun_job(&self, job_id: &str) -> Result<SyncJobEntity, String> {
        let job = self
            .job_repo
            .get_job_by_id(job_id)
            .await?
            .ok_or("Job not found")?;

        if job.job_type != "licenses_sync" {
            return Err("Not a license sync job".to_string());
        }

        // Reset and run fresh
        self.job_repo.reset_job(job_id).await?;
        self.sync_all_licenses().await
    }

    // =========================================================================
    // Background Job Execution Methods (for job_worker)
    // =========================================================================

    /// Create a pending job without executing it
    ///
    /// Used by HTTP handlers to create a job that will be executed by
    /// the background worker. Returns the job in "pending" status.
    pub async fn create_pending_job(&self) -> Result<SyncJobEntity, String> {
        info!("Creating pending license sync job");

        // Check if any license sync job is running
        let existing_jobs = self.job_repo.list_jobs_by_status("running").await?;
        for job in existing_jobs {
            if job.job_type == "licenses_sync" {
                return Err("License sync job already running".to_string());
            }
        }

        // Create a new pending job
        let job = self.job_repo.create_job(NewSyncJob::licenses_sync()).await?;

        // Broadcast pending job
        #[cfg(feature = "ssr")]
        broadcast_job_update(&job);

        Ok(job)
    }

    /// Execute an existing job (called by background worker)
    ///
    /// Takes a job ID, marks the job as running, executes the sync,
    /// and marks as completed or failed.
    pub async fn execute_job(&self, job_id: &str) -> Result<SyncJobEntity, String> {
        info!("Executing license sync job {}", job_id);

        // Mark as running
        self.job_repo.mark_running(job_id).await?;

        // Broadcast running status
        #[cfg(feature = "ssr")]
        if let Ok(Some(running_job)) = self.job_repo.get_job_by_id(job_id).await {
            broadcast_job_update(&running_job);
        }

        // Execute with timing
        let start_time = Instant::now();
        match self.execute_sync(job_id).await {
            Ok((fetched, inserted, context_json, logs)) => {
                let duration_ms = start_time.elapsed().as_millis() as i64;
                info!(
                    "License sync completed for job {}: {} fetched, {} upserted, took {}ms",
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
                error!("License sync failed for job {}: {}", job_id, e);
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
    pub async fn reset_and_execute(&self, job_id: &str) -> Result<SyncJobEntity, String> {
        info!("Resetting and executing license sync job {}", job_id);

        // Reset job to pending
        self.job_repo.reset_job(job_id).await?;

        // Broadcast reset
        #[cfg(feature = "ssr")]
        if let Ok(Some(reset_job)) = self.job_repo.get_job_by_id(job_id).await {
            broadcast_job_update(&reset_job);
        }

        // Execute the job
        self.execute_job(job_id).await
    }
}
