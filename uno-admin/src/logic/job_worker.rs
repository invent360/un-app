//! Background worker that processes the job queue
//!
//! This module provides a background worker task that:
//! - Receives job commands from the queue
//! - Executes them asynchronously
//! - Reports results via WebSocket broadcasts

use super::job_queue::{init_job_queue, JobCommand};
use crate::db::get_db;
use crate::logic::{LicenseSyncService, RewardsSyncService};
use tracing::{error, info};

/// Start the background job worker
///
/// This spawns a tokio task that:
/// 1. Initializes the job queue (creates the mpsc channel)
/// 2. Loops forever, receiving and processing job commands
/// 3. Each job is executed asynchronously with proper error handling
///
/// # Panics
/// Panics if called more than once (job queue can only be initialized once).
pub fn start_job_worker() {
    tokio::spawn(async move {
        let mut rx = init_job_queue();
        info!("[JobWorker] Background job worker started");

        while let Some(cmd) = rx.recv().await {
            info!("[JobWorker] Received command: {:?}", cmd);

            if let Some(pool) = get_db() {
                match cmd {
                    JobCommand::RewardsSync { job_id, target_date } => {
                        info!(
                            "[JobWorker] Starting rewards sync for job {} (date: {})",
                            job_id, target_date
                        );
                        let service = RewardsSyncService::new(pool.clone());
                        match service.execute_job(&job_id, &target_date).await {
                            Ok(_) => {
                                info!("[JobWorker] Rewards sync completed for job {}", job_id)
                            }
                            Err(e) => {
                                error!("[JobWorker] Rewards sync failed for job {}: {}", job_id, e)
                            }
                        }
                    }

                    JobCommand::LicenseSync { job_id } => {
                        info!("[JobWorker] Starting license sync for job {}", job_id);
                        let service = LicenseSyncService::new(pool.clone());
                        match service.execute_job(&job_id).await {
                            Ok(_) => {
                                info!("[JobWorker] License sync completed for job {}", job_id)
                            }
                            Err(e) => {
                                error!("[JobWorker] License sync failed for job {}: {}", job_id, e)
                            }
                        }
                    }

                    JobCommand::RerunJob {
                        job_id,
                        job_type,
                        target_date,
                    } => {
                        info!(
                            "[JobWorker] Re-running job {} (type: {}, date: {})",
                            job_id, job_type, target_date
                        );

                        match job_type.as_str() {
                            "rewards_sync" => {
                                let service = RewardsSyncService::new(pool.clone());
                                // Reset job then execute
                                if let Err(e) = service.reset_and_execute(&job_id, &target_date).await {
                                    error!("[JobWorker] Rerun failed for job {}: {}", job_id, e);
                                } else {
                                    info!("[JobWorker] Rerun completed for job {}", job_id);
                                }
                            }
                            "licenses_sync" => {
                                let service = LicenseSyncService::new(pool.clone());
                                // Reset job then execute
                                if let Err(e) = service.reset_and_execute(&job_id).await {
                                    error!("[JobWorker] Rerun failed for job {}: {}", job_id, e);
                                } else {
                                    info!("[JobWorker] Rerun completed for job {}", job_id);
                                }
                            }
                            _ => {
                                error!("[JobWorker] Unknown job type for rerun: {}", job_type);
                            }
                        }
                    }
                }
            } else {
                error!("[JobWorker] Database not available, skipping command: {:?}", cmd);
            }
        }

        error!("[JobWorker] Job queue receiver dropped - worker exiting!");
    });
}
