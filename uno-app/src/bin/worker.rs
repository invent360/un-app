//! Standalone worker binary for background job processing
//!
//! Runs separately from the main web application to process:
//! - Job queue tasks
//! - Outbox event publishing
//! - Nonce cleanup
//! - Event retention cleanup
//!
//! Usage:
//!   worker --type all          # Run all workers
//!   worker --type jobs         # Run job processor only
//!   worker --type publisher    # Run outbox publisher only
//!   worker --type cleanup      # Run cleanup tasks only

use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing_subscriber::EnvFilter;

/// Worker CLI arguments
#[derive(Parser, Debug)]
#[command(name = "worker", about = "Background worker for UNO App")]
struct Args {
    /// Worker type: all, jobs, publisher, cleanup
    #[arg(long, short = 't', default_value = "all")]
    r#type: WorkerType,

    /// Poll interval in seconds
    #[arg(long, default_value = "5")]
    poll_interval: u64,

    /// Maximum jobs to process before recycling (0 = unlimited)
    #[arg(long, default_value = "0")]
    max_jobs: u64,

    /// Outbox batch size
    #[arg(long, default_value = "50")]
    batch_size: i32,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkerType {
    All,
    Jobs,
    Publisher,
    Cleanup,
}

impl std::str::FromStr for WorkerType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "all" => Ok(Self::All),
            "jobs" | "job_processor" => Ok(Self::Jobs),
            "publisher" | "outbox_publisher" => Ok(Self::Publisher),
            "cleanup" => Ok(Self::Cleanup),
            _ => Err(format!(
                "Unknown worker type: {}. Valid types: all, jobs, publisher, cleanup",
                s
            )),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if present
    let _ = dotenvy::dotenv();

    // Parse CLI arguments
    let args = Args::parse();

    // Initialize tracing
    let filter = EnvFilter::try_new(&args.log_level)
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .json()
        .init();

    tracing::info!(
        worker_type = ?args.r#type,
        poll_interval = args.poll_interval,
        batch_size = args.batch_size,
        "Starting worker"
    );

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL environment variable not set")?;

    // Create connection pool
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    tracing::info!("Connected to database");

    // Setup shutdown signal handling
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    ctrlc::set_handler(move || {
        tracing::info!("Shutdown signal received");
        shutdown_clone.store(true, Ordering::SeqCst);
    })?;

    // Create repositories
    let outbox_repo = Arc::new(
        uno_app::server::repositories::OutboxRepositoryImpl::new(pool.clone())
    );
    let audit_repo = Arc::new(
        uno_app::server::repositories::ImmutableAuditRepositoryImpl::new(pool.clone())
    );
    let nonce_repo = Arc::new(
        uno_app::server::repositories::NonceRepositoryImpl::new(pool.clone())
    );

    // Run workers based on type
    match args.r#type {
        WorkerType::All => {
            run_all_workers(
                pool,
                outbox_repo,
                audit_repo,
                nonce_repo,
                args,
                shutdown,
            ).await?;
        }
        WorkerType::Jobs => {
            run_job_processor(
                pool,
                outbox_repo,
                audit_repo,
                args,
                shutdown,
            ).await?;
        }
        WorkerType::Publisher => {
            run_outbox_publisher(
                outbox_repo,
                args,
                shutdown,
            ).await?;
        }
        WorkerType::Cleanup => {
            run_cleanup_tasks(
                outbox_repo,
                nonce_repo,
                args,
                shutdown,
            ).await?;
        }
    }

    tracing::info!("Worker shutdown complete");
    Ok(())
}

async fn run_all_workers(
    pool: sqlx::PgPool,
    outbox_repo: Arc<uno_app::server::repositories::OutboxRepositoryImpl>,
    audit_repo: Arc<uno_app::server::repositories::ImmutableAuditRepositoryImpl>,
    nonce_repo: Arc<uno_app::server::repositories::NonceRepositoryImpl>,
    args: Args,
    shutdown: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let shutdown_jobs = shutdown.clone();
    let shutdown_publisher = shutdown.clone();
    let shutdown_cleanup = shutdown.clone();

    let pool_jobs = pool.clone();
    let outbox_jobs = outbox_repo.clone();
    let audit_jobs = audit_repo.clone();
    let poll_interval_jobs = args.poll_interval;
    let max_jobs = args.max_jobs;

    let outbox_publisher = outbox_repo.clone();
    let batch_size = args.batch_size;
    let poll_interval_publisher = args.poll_interval;

    // Spawn all worker tasks
    let jobs_handle = tokio::spawn(async move {
        run_job_processor_loop(
            pool_jobs,
            outbox_jobs,
            audit_jobs,
            poll_interval_jobs,
            max_jobs,
            shutdown_jobs,
        ).await
    });

    let publisher_handle = tokio::spawn(async move {
        run_publisher_loop(
            outbox_publisher,
            batch_size,
            poll_interval_publisher,
            shutdown_publisher,
        ).await
    });

    let cleanup_handle = tokio::spawn(async move {
        run_cleanup_loop(
            outbox_repo,
            nonce_repo,
            shutdown_cleanup,
        ).await
    });

    // Wait for all tasks to complete
    let _ = tokio::join!(jobs_handle, publisher_handle, cleanup_handle);

    Ok(())
}

async fn run_job_processor(
    pool: sqlx::PgPool,
    outbox_repo: Arc<uno_app::server::repositories::OutboxRepositoryImpl>,
    audit_repo: Arc<uno_app::server::repositories::ImmutableAuditRepositoryImpl>,
    args: Args,
    shutdown: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    run_job_processor_loop(
        pool,
        outbox_repo,
        audit_repo,
        args.poll_interval,
        args.max_jobs,
        shutdown,
    ).await
}

async fn run_job_processor_loop(
    pool: sqlx::PgPool,
    outbox_repo: Arc<uno_app::server::repositories::OutboxRepositoryImpl>,
    audit_repo: Arc<uno_app::server::repositories::ImmutableAuditRepositoryImpl>,
    poll_interval: u64,
    max_jobs: u64,
    shutdown: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    use uno_app::server::services::{WorkerRunner, WorkerConfig, JobResult};

    let config = WorkerConfig {
        worker_name: format!("worker-{}", uuid::Uuid::new_v4()),
        worker_type: "job_processor".to_string(),
        job_types: vec![], // Process all job types
        poll_interval: Duration::from_secs(poll_interval),
        heartbeat_interval: Duration::from_secs(60),
        lease_duration_secs: 300,
        max_jobs_before_recycle: if max_jobs > 0 { Some(max_jobs) } else { None },
        job_timeout: Duration::from_secs(240),
    };

    let mut runner = WorkerRunner::new(
        config,
        pool,
        outbox_repo,
        audit_repo,
    );

    // Set shutdown handle
    let runner_shutdown = runner.shutdown_handle();
    tokio::spawn(async move {
        while !shutdown.load(Ordering::Relaxed) {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        runner_shutdown.store(true, Ordering::SeqCst);
    });

    // Define job handler
    let handler = |job: &uno_app::server::services::Job| {
        let job_type = job.job_type.clone();
        let job_id = job.id;

        Box::pin(async move {
            tracing::info!(job_id = %job_id, job_type = %job_type, "Processing job");

            // Dispatch based on job type
            // In a real implementation, you'd have specific handlers per job type
            match job_type.as_str() {
                "sync_licenses" => {
                    // Handle license sync
                    tracing::info!("Syncing licenses...");
                    JobResult::success_with_data(serde_json::json!({"synced": true}))
                }
                "send_notification" => {
                    // Handle notification sending
                    tracing::info!("Sending notification...");
                    JobResult::success_with_data(serde_json::json!({"sent": true}))
                }
                "cleanup_expired" => {
                    // Handle cleanup
                    tracing::info!("Cleaning up expired items...");
                    JobResult::success_with_data(serde_json::json!({"cleaned": true}))
                }
                _ => {
                    tracing::warn!(job_type = %job_type, "Unknown job type");
                    JobResult::failure(&format!("Unknown job type: {}", job_type))
                }
            }
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = JobResult> + Send>>
    };

    let stats = runner.run(handler).await?;

    tracing::info!(
        worker_id = %stats.worker_id,
        jobs_processed = stats.jobs_processed,
        jobs_failed = stats.jobs_failed,
        "Job processor completed"
    );

    Ok(())
}

async fn run_outbox_publisher(
    outbox_repo: Arc<uno_app::server::repositories::OutboxRepositoryImpl>,
    args: Args,
    shutdown: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    run_publisher_loop(
        outbox_repo,
        args.batch_size,
        args.poll_interval,
        shutdown,
    ).await
}

async fn run_publisher_loop(
    outbox_repo: Arc<uno_app::server::repositories::OutboxRepositoryImpl>,
    batch_size: i32,
    poll_interval: u64,
    shutdown: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    use uno_app::server::services::{OutboxPublisher, OutboxPublisherConfig};

    let config = OutboxPublisherConfig {
        batch_size,
        max_retries: 5,
        request_timeout_secs: 30,
        webhook_url: std::env::var("WEBHOOK_URL").ok(),
        webhook_secret: std::env::var("WEBHOOK_SECRET").ok(),
    };

    let publisher = OutboxPublisher::from_config(outbox_repo, config);

    let mut timer = interval(Duration::from_secs(poll_interval));

    loop {
        timer.tick().await;

        if shutdown.load(Ordering::Relaxed) {
            break;
        }

        match publisher.publish_batch().await {
            Ok(stats) => {
                if stats.claimed > 0 {
                    tracing::info!(
                        claimed = stats.claimed,
                        published = stats.published,
                        failed = stats.failed,
                        "Published outbox batch"
                    );
                }
            }
            Err(e) => {
                tracing::error!(error = %e, "Failed to publish outbox batch");
            }
        }
    }

    Ok(())
}

async fn run_cleanup_tasks(
    outbox_repo: Arc<uno_app::server::repositories::OutboxRepositoryImpl>,
    nonce_repo: Arc<uno_app::server::repositories::NonceRepositoryImpl>,
    _args: Args,
    shutdown: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    run_cleanup_loop(outbox_repo, nonce_repo, shutdown).await
}

async fn run_cleanup_loop(
    outbox_repo: Arc<uno_app::server::repositories::OutboxRepositoryImpl>,
    nonce_repo: Arc<uno_app::server::repositories::NonceRepositoryImpl>,
    shutdown: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    use uno_app::server::repositories::NonceRepository;

    // Run cleanup every 5 minutes
    let mut timer = interval(Duration::from_secs(300));
    let retention_days = 7;
    let nonce_retention_secs = 600; // 10 minutes for nonces

    loop {
        timer.tick().await;

        if shutdown.load(Ordering::Relaxed) {
            break;
        }

        // Cleanup old outbox events
        match outbox_repo.cleanup_old_outbox_events(retention_days).await {
            Ok(count) if count > 0 => {
                tracing::info!(count = count, "Cleaned up old outbox events");
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error = %e, "Failed to cleanup outbox events");
            }
        }

        // Cleanup old inbox events
        match outbox_repo.cleanup_old_inbox_events(retention_days).await {
            Ok(count) if count > 0 => {
                tracing::info!(count = count, "Cleaned up old inbox events");
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error = %e, "Failed to cleanup inbox events");
            }
        }

        // Cleanup expired nonces
        match nonce_repo.cleanup_older_than_secs(nonce_retention_secs).await {
            Ok(count) if count > 0 => {
                tracing::info!(count = count, "Cleaned up expired nonces");
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error = %e, "Failed to cleanup nonces");
            }
        }
    }

    Ok(())
}
