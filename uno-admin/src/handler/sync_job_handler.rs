//! Handlers for sync job operations

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::entity::SyncJobEntity;

/// Server function to list all sync jobs
#[server(ListSyncJobs, "/api")]
pub async fn list_sync_jobs() -> Result<Vec<SyncJobEntity>, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::RewardsSyncService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = RewardsSyncService::new(pool);
    service
        .list_jobs()
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Server function to get a single sync job by ID
#[server(GetSyncJob, "/api")]
pub async fn get_sync_job(id: String) -> Result<Option<SyncJobEntity>, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::RewardsSyncService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = RewardsSyncService::new(pool);
    service
        .get_job(&id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Server function to run a new sync job for a specific date
///
/// This creates a pending job and enqueues it for background execution.
/// The HTTP response returns immediately with the pending job.
/// Job status updates are broadcast via WebSocket.
#[server(RunSyncJob, "/api")]
pub async fn run_sync_job(target_date: String) -> Result<SyncJobEntity, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::{enqueue, JobCommand, RewardsSyncService};

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = RewardsSyncService::new(pool);

    // Create a pending job
    let job = service
        .create_pending_job(&target_date)
        .await
        .map_err(|e| ServerFnError::new(e))?;

    // Enqueue for background execution
    enqueue(JobCommand::RewardsSync {
        job_id: job.id.clone(),
        target_date: target_date.clone(),
    })
    .map_err(|e| ServerFnError::new(e))?;

    // Return the pending job immediately
    Ok(job)
}

/// Server function to re-run an existing job (idempotent)
///
/// This resets the job and enqueues it for background execution.
/// The HTTP response returns immediately with the reset job.
#[server(RerunSyncJob, "/api")]
pub async fn rerun_sync_job(job_id: String) -> Result<SyncJobEntity, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::{enqueue, JobCommand, RewardsSyncService};
    use crate::repository::scylla::SyncJobRepository;
    use crate::repository::traits::SyncJobRepositoryTrait;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    // Get the existing job to determine its type and target date
    let job_repo = SyncJobRepository::new(pool.clone());
    let job = job_repo
        .get_job_by_id(&job_id)
        .await
        .map_err(|e| ServerFnError::new(e))?
        .ok_or_else(|| ServerFnError::new("Job not found"))?;

    // Reset the job to pending status
    job_repo
        .reset_job(&job_id)
        .await
        .map_err(|e| ServerFnError::new(e))?;

    // Broadcast reset status
    #[cfg(feature = "ssr")]
    if let Ok(Some(reset_job)) = job_repo.get_job_by_id(&job_id).await {
        crate::ws::broadcast_job_update(&reset_job);
    }

    // Enqueue for background execution based on job type
    enqueue(JobCommand::RerunJob {
        job_id: job.id.clone(),
        job_type: job.job_type.clone(),
        target_date: job.target_date.clone(),
    })
    .map_err(|e| ServerFnError::new(e))?;

    // Return the reset job immediately
    job_repo
        .get_job_by_id(&job_id)
        .await
        .map_err(|e| ServerFnError::new(e))?
        .ok_or_else(|| ServerFnError::new("Job not found after reset"))
}

/// Server function to delete a sync job
#[server(DeleteSyncJob, "/api")]
pub async fn delete_sync_job(id: String) -> Result<bool, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::RewardsSyncService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = RewardsSyncService::new(pool);
    service
        .delete_job(&id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Server function to run a license sync job
///
/// This creates a pending job and enqueues it for background execution.
/// The HTTP response returns immediately with the pending job.
#[server(RunLicenseSyncJob, "/api")]
pub async fn run_license_sync_job() -> Result<SyncJobEntity, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::{enqueue, JobCommand, LicenseSyncService};

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = LicenseSyncService::new(pool);

    // Create a pending job
    let job = service
        .create_pending_job()
        .await
        .map_err(|e| ServerFnError::new(e))?;

    // Enqueue for background execution
    enqueue(JobCommand::LicenseSync {
        job_id: job.id.clone(),
    })
    .map_err(|e| ServerFnError::new(e))?;

    // Return the pending job immediately
    Ok(job)
}

/// Server function to process pending retry jobs
#[server(ProcessPendingRetries, "/api")]
pub async fn process_pending_retries() -> Result<usize, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::RewardsSyncService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = RewardsSyncService::new(pool);
    service
        .process_pending_retries()
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// DTO for running a sync job via form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSyncJobForm {
    pub target_date: String,
}

/// State for the sync jobs page
#[derive(Debug, Clone, Default)]
pub struct SyncJobsPageState {
    pub jobs: Vec<SyncJobEntity>,
    pub loading: bool,
    pub error: Option<String>,
    pub running_job: Option<String>,
}

/// Context for managing sync jobs state
#[derive(Clone, Copy)]
pub struct SyncJobsContext {
    pub state: RwSignal<SyncJobsPageState>,
}

impl SyncJobsContext {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(SyncJobsPageState::default()),
        }
    }

    pub fn set_loading(&self, loading: bool) {
        self.state.update(|s| s.loading = loading);
    }

    pub fn set_error(&self, error: Option<String>) {
        self.state.update(|s| s.error = error);
    }

    pub fn set_jobs(&self, jobs: Vec<SyncJobEntity>) {
        self.state.update(|s| {
            s.jobs = jobs;
            s.loading = false;
            s.error = None;
        });
    }

    pub fn set_running_job(&self, job_id: Option<String>) {
        self.state.update(|s| s.running_job = job_id);
    }

    pub fn add_job(&self, job: SyncJobEntity) {
        self.state.update(|s| {
            // Remove existing job with same ID if present
            s.jobs.retain(|j| j.id != job.id);
            // Add at beginning
            s.jobs.insert(0, job);
        });
    }

    pub fn update_job(&self, job: SyncJobEntity) {
        self.state.update(|s| {
            if let Some(existing) = s.jobs.iter_mut().find(|j| j.id == job.id) {
                *existing = job;
            }
        });
    }

    pub fn remove_job(&self, id: &str) {
        self.state.update(|s| {
            s.jobs.retain(|j| j.id != id);
        });
    }
}

impl Default for SyncJobsContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook to access sync jobs context
pub fn use_sync_jobs() -> SyncJobsContext {
    expect_context::<SyncJobsContext>()
}

/// Provider component for SyncJobsContext
#[component]
pub fn SyncJobsContextProvider(children: Children) -> impl IntoView {
    let context = SyncJobsContext::new();
    provide_context(context);
    children()
}

/// Server function to get the badge count (sum of records_inserted from completed jobs since a timestamp)
#[server(GetJobsBadgeCount, "/api")]
pub async fn get_jobs_badge_count(since_timestamp: String) -> Result<u32, ServerFnError> {
    use crate::db::get_db;
    use crate::logic::RewardsSyncService;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let service = RewardsSyncService::new(pool);
    let jobs = service
        .list_jobs()
        .await
        .map_err(|e| ServerFnError::new(e))?;

    // Filter jobs completed after the given timestamp and sum records_inserted
    let count: i32 = jobs
        .iter()
        .filter(|job| {
            job.status == "completed" &&
            job.completed_at.as_ref().map(|t| t.as_str() > since_timestamp.as_str()).unwrap_or(false)
        })
        .map(|job| job.records_inserted)
        .sum();

    Ok(count as u32)
}

/// Register sync job server functions
#[cfg(feature = "ssr")]
pub fn register_sync_job_server_fns() {
    use server_fn::ServerFn;

    // Reference the URL to ensure the server functions are linked
    println!("Registering sync job server functions:");
    println!("  ListSyncJobs: {}", ListSyncJobs::url());
    println!("  GetSyncJob: {}", GetSyncJob::url());
    println!("  RunSyncJob: {}", RunSyncJob::url());
    println!("  RerunSyncJob: {}", RerunSyncJob::url());
    println!("  DeleteSyncJob: {}", DeleteSyncJob::url());
    println!("  ProcessPendingRetries: {}", ProcessPendingRetries::url());
    println!("  GetJobsBadgeCount: {}", GetJobsBadgeCount::url());
    println!("  RunLicenseSyncJob: {}", RunLicenseSyncJob::url());
}
