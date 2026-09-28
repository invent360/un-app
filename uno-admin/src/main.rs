#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos::config::get_configuration;
    use leptos_meta::MetaTags;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use uno_admin::app::*;
    use uno_admin::repository::traits::SyncJobRepositoryTrait;
    use tracing::{info, warn, error};

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize JSON logging
    init_logging();

    // Start background scheduler for rewards sync
    start_rewards_sync_scheduler();

    // Start background scheduler for marketplace sync
    start_marketplace_sync_scheduler();

    // Start background scheduler for license sync
    start_license_sync_scheduler();

    // Start job watchdog to cleanup stale running jobs
    start_job_watchdog();

    // Register schema client server functions
    uno_admin::api::register_schema_server_fns();

    // Register home page server functions
    uno_admin::api::register_home_server_fns();

    // Register FAQ page server functions
    uno_admin::api::register_faq_server_fns();

    // Register editor server functions (preview, version history, etc.)
    uno_admin::ui::pages::content::register_editor_server_fns();

    // Register license handler server functions
    uno_admin::handler::license_handler::register_license_server_fns();

    // Register sync job server functions
    uno_admin::handler::sync_job_handler::register_sync_job_server_fns();

    // Register analytics server functions
    uno_admin::handler::analytics_handler::register_analytics_server_fns();

    // Initialize database connection and run migrations
    info!("Initializing database connection...");
    uno_admin::db::init_db()
        .await
        .expect("Failed to initialize database");
    info!("Database initialized successfully");

    // Initialize WebSocket broadcaster for real-time job updates
    uno_admin::ws::init_broadcaster();

    // Start background job worker for async job execution
    uno_admin::logic::start_job_worker();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        tracing::info!("listening on http://{}", &addr);

        App::new()
            // WebSocket endpoint for real-time job updates
            .route("/ws/jobs", web::get().to(uno_admin::ws::ws_jobs_handler))
            // File upload endpoints (must be before generic /api handler)
            .configure(uno_admin::handler::file_handler::configure_routes)
            // Handle server functions
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve ember-fx styles from `styles` directory
            .service(Files::new("/styles", format!("{site_root}/styles")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", &site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use uno_admin::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}

/// Start background scheduler for marketplace synchronization
///
/// This scheduler:
/// - Runs every 5 minutes
/// - Polls uno-app for claimed licenses
/// - Syncs referrals bi-directionally
#[cfg(feature = "ssr")]
fn start_marketplace_sync_scheduler() {
    use std::time::Duration;

    tokio::spawn(async move {
        tracing::info!(scheduler = "marketplace", "Starting marketplace sync scheduler (5-minute interval)");

        // Wait for server initialization
        tokio::time::sleep(Duration::from_secs(60)).await;

        // Check every 5 minutes
        let mut interval = tokio::time::interval(Duration::from_secs(300));

        loop {
            interval.tick().await;

            tracing::info!(scheduler = "marketplace", "Starting sync...");

            if let Some(pool) = uno_admin::db::get_db() {
                // Poll for claimed licenses
                match uno_admin::logic::MarketplaceService::new(pool.clone()) {
                    Ok(marketplace_service) => {
                        match marketplace_service.poll_and_sync().await {
                            Ok(result) => {
                                tracing::info!(
                                    scheduler = "marketplace",
                                    fetched = result.fetched,
                                    updated = result.updated,
                                    "Claimed licenses synced"
                                );
                            }
                            Err(e) => {
                                tracing::error!(scheduler = "marketplace", error = %e, "Poll claimed failed");
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!(scheduler = "marketplace", error = %e, "Failed to create marketplace service");
                    }
                }

                // Sync referrals bi-directionally
                match uno_admin::logic::ReferralSyncService::new(pool.clone()) {
                    Ok(referral_service) => {
                        match referral_service.sync().await {
                            Ok(result) => {
                                tracing::info!(
                                    scheduler = "marketplace",
                                    created = result.created,
                                    updated = result.updated,
                                    failed = result.failed,
                                    "Referrals synced"
                                );
                            }
                            Err(e) => {
                                tracing::error!(scheduler = "marketplace", error = %e, "Referral sync failed");
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!(scheduler = "marketplace", error = %e, "Failed to create referral service");
                    }
                }
            } else {
                tracing::error!(scheduler = "marketplace", "Database not available");
            }
        }
    });
}

/// Start background scheduler for license synchronization
///
/// This scheduler:
/// - Runs every 10 minutes
/// - Fetches all licenses from Unetwork API
/// - Syncs status to local ScyllaDB
/// - Only runs if previous job completed
#[cfg(feature = "ssr")]
fn start_license_sync_scheduler() {
    use std::time::Duration;

    tokio::spawn(async move {
        tracing::info!(scheduler = "license_sync", "Starting license sync scheduler (10-minute interval)");

        // Wait for server initialization
        tokio::time::sleep(Duration::from_secs(45)).await;

        // Check every 10 minutes
        let mut interval = tokio::time::interval(Duration::from_secs(600));

        loop {
            interval.tick().await;

            tracing::info!(scheduler = "license_sync", "Checking if sync needed...");

            if let Some(pool) = uno_admin::db::get_db() {
                let service = uno_admin::logic::LicenseSyncService::new(pool.clone());

                // Check if any license sync job is currently running
                if service.is_running().await {
                    tracing::info!(scheduler = "license_sync", "Job already running, skipping");
                    continue;
                }

                match service.sync_all_licenses().await {
                    Ok(job) => {
                        tracing::info!(
                            scheduler = "license_sync",
                            job_id = %job.id,
                            fetched = job.records_fetched,
                            upserted = job.records_inserted,
                            "Job completed"
                        );
                    }
                    Err(e) => {
                        tracing::error!(scheduler = "license_sync", error = %e, "Job failed");
                    }
                }
            } else {
                tracing::error!(scheduler = "license_sync", "Database not available");
            }
        }
    });
}

/// Start background scheduler for rewards synchronization
///
/// This scheduler:
/// - Runs hourly to check if sync is needed
/// - At 02:00 AM UTC, triggers sync for previous day's rewards
/// - Processes any pending retry jobs
#[cfg(feature = "ssr")]
fn start_rewards_sync_scheduler() {
    use chrono::Utc;
    use std::time::Duration;

    tokio::spawn(async move {
        tracing::info!(scheduler = "rewards", "Starting rewards sync scheduler (1-minute interval)");

        // Wait a bit for server to fully initialize
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Check every 1 minute
        let mut interval = tokio::time::interval(Duration::from_secs(60));

        loop {
            interval.tick().await;

            let now = Utc::now();
            tracing::debug!(scheduler = "rewards", time = %now.format("%H:%M:%S"), "Tick");

            // Always trigger sync for previous day on each tick
            let yesterday = (now - chrono::Duration::days(1))
                .format("%Y-%m-%d")
                .to_string();

            tracing::info!(scheduler = "rewards", date = %yesterday, "Triggering rewards sync");

            if let Some(pool) = uno_admin::db::get_db() {
                let service = uno_admin::logic::RewardsSyncService::new(pool.clone());
                match service.sync_rewards_for_date(&yesterday).await {
                    Ok(job) => {
                        tracing::info!(
                            scheduler = "rewards",
                            job_id = %job.id,
                            status = %job.status,
                            "Sync job completed"
                        );
                    }
                    Err(e) => {
                        tracing::error!(scheduler = "rewards", error = %e, "Sync job failed");
                    }
                }
            } else {
                tracing::error!(scheduler = "rewards", "Database not available");
            }
        }
    });
}

/// Start background watchdog for stale/orphaned jobs
///
/// This watchdog:
/// - Runs every 60 seconds
/// - Finds jobs stuck in "running" status
/// - Marks as failed if running for > 5 minutes
/// - Broadcasts failure via WebSocket for UI updates
#[cfg(feature = "ssr")]
fn start_job_watchdog() {
    use chrono::Utc;
    use std::time::Duration;
    use uno_admin::repository::traits::SyncJobRepositoryTrait;

    tokio::spawn(async move {
        tracing::info!(scheduler = "watchdog", "Starting job watchdog (60-second interval)");

        // Wait for server initialization
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Check every 60 seconds
        let mut interval = tokio::time::interval(Duration::from_secs(60));

        // Maximum job runtime: 5 minutes (300 seconds)
        const MAX_RUNTIME_SECS: i64 = 300;

        loop {
            interval.tick().await;

            if let Some(pool) = uno_admin::db::get_db() {
                let job_repo =
                    uno_admin::repository::scylla::SyncJobRepository::new(pool.clone());

                match job_repo.list_jobs_by_status("running").await {
                    Ok(running_jobs) => {
                        if running_jobs.is_empty() {
                            continue;
                        }

                        let now = Utc::now();
                        tracing::info!(
                            scheduler = "watchdog",
                            count = running_jobs.len(),
                            "Checking running jobs for timeout"
                        );

                        for job in running_jobs {
                            if let Some(started_at_str) = &job.started_at {
                                if let Ok(started_at) =
                                    chrono::DateTime::parse_from_rfc3339(started_at_str)
                                {
                                    let running_secs =
                                        (now - started_at.with_timezone(&Utc)).num_seconds();

                                    if running_secs > MAX_RUNTIME_SECS {
                                        tracing::warn!(
                                            scheduler = "watchdog",
                                            job_id = %job.id,
                                            running_secs = running_secs,
                                            max_secs = MAX_RUNTIME_SECS,
                                            "Job timed out, marking as failed"
                                        );

                                        let error_msg = format!(
                                            "Job timed out: running for {} seconds (max: {})",
                                            running_secs, MAX_RUNTIME_SECS
                                        );

                                        if let Err(e) =
                                            job_repo.mark_failed(&job.id, &error_msg).await
                                        {
                                            tracing::error!(
                                                scheduler = "watchdog",
                                                job_id = %job.id,
                                                error = %e,
                                                "Failed to mark job as failed"
                                            );
                                        } else {
                                            tracing::info!(
                                                scheduler = "watchdog",
                                                job_id = %job.id,
                                                "Marked job as failed due to timeout"
                                            );

                                            // Broadcast the failure via WebSocket
                                            if let Ok(Some(failed_job)) =
                                                job_repo.get_job_by_id(&job.id).await
                                            {
                                                uno_admin::ws::broadcast_job_update(&failed_job);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!(scheduler = "watchdog", error = %e, "Failed to query running jobs");
                    }
                }
            }
        }
    });
}

/// Initialize JSON logging for structured output
#[cfg(feature = "ssr")]
fn init_logging() {
    use tracing_subscriber::{
        fmt::{self, format::FmtSpan},
        layer::SubscriberExt,
        util::SubscriberInitExt,
        EnvFilter,
    };

    // Get log level from environment or default to info
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("warn,uno_admin=info,actix_web=info,actix_server=info")
    });

    // Check LOG_FORMAT env var (default to json)
    let use_json = std::env::var("LOG_FORMAT")
        .map(|v| v.to_lowercase() != "pretty")
        .unwrap_or(true);

    if use_json {
        // JSON format for structured logging
        let fmt_layer = fmt::layer()
            .json()
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::CLOSE);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .init();
    } else {
        // Pretty format for development
        let fmt_layer = fmt::layer()
            .with_target(true)
            .with_file(true)
            .with_line_number(true);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .init();
    }
}
