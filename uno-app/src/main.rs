//! UNO Web Application Entry Point

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    use tracing::{info, warn};
    use uno_app::api::{
        register_faq_server_fns, register_guides_server_fns, register_home_server_fns,
        register_tasks_server_fns,
    };
    use uno_app::app::App;
    use uno_app::server;
    use uno_app::server::app::ServiceFactory;
    use uno_app::server::db::ConnectionManager;
    use uno_app::server::middleware::{
        init_logging, CsrfProtection, RequestLogger, SecurityHeaders, VisitorTracker,
    };

    // Load environment variables
    dotenvy::dotenv().ok();
    let session_verifier =
        uno_api::auth::session::SessionVerifier::from_env().map_err(std::io::Error::other)?;
    let _machine_auth =
        uno_app::server::middleware::AdminAuth::from_env().map_err(std::io::Error::other)?;
    let csrf_secret = std::env::var("CSRF_SECRET_KEY")
        .ok()
        .filter(|secret| secret.len() >= 32 && !secret.starts_with("dev-"))
        .ok_or_else(|| {
            std::io::Error::other("CSRF_SECRET_KEY must be configured with at least 32 characters")
        })?;
    // R5-11: Use unified production mode resolver
    let is_production = uno_app::server::config::is_production_mode();
    let ui_only = !is_production && std::env::var("UNO_UI_ONLY").is_ok_and(|value| value == "true");

    // Register server functions explicitly
    register_faq_server_fns();
    register_guides_server_fns();
    register_tasks_server_fns();
    register_home_server_fns();

    // Initialize logging
    init_logging();

    info!("Starting UNO Web Application...");

    // R3-11: Verify media volume at startup (optional - warn if failed)
    if let Err(e) = ServiceFactory::verify_volume() {
        if is_production {
            return Err(std::io::Error::other(format!("Media volume verification failed: {}", e)));
        } else {
            warn!("Media volume verification failed (non-production): {}", e);
        }
    }

    // Try to initialize database pool (optional for UI development)
    let service_factory = match ConnectionManager::from_env(true).await {
        Ok(pool) => {
            info!("Database connected successfully");
            let factory = ServiceFactory::new(pool);

            // Start background content scheduler
            let scheduler_factory = factory.clone();
            actix_rt::spawn(async move {
                uno_app::server::scheduler::start_content_scheduler(scheduler_factory).await;
            });

            // R5-09: Start outbox publisher background task
            if let Some(webhook_url) = std::env::var("WEBHOOK_URL").ok() {
                let outbox_repo = factory.outbox_repository.clone();
                let webhook_secret = std::env::var("WEBHOOK_SECRET").ok();
                let config = uno_app::server::services::OutboxPublisherConfig {
                    webhook_url: Some(webhook_url),
                    webhook_secret,
                    ..Default::default()
                };
                let shutdown = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

                // Outbox publisher - polls and publishes events
                let outbox_repo_clone = outbox_repo.clone();
                let shutdown_clone = shutdown.clone();
                actix_rt::spawn(async move {
                    uno_app::server::services::run_outbox_publisher(
                        outbox_repo_clone,
                        config,
                        std::time::Duration::from_secs(5),
                        shutdown_clone,
                    ).await;
                });

                // Stale event recovery - resets stuck events
                actix_rt::spawn(async move {
                    uno_app::server::services::run_outbox_stale_recovery(
                        outbox_repo,
                        std::time::Duration::from_secs(60),
                        shutdown,
                    ).await;
                });

                info!("Outbox publisher enabled");
            } else {
                warn!("WEBHOOK_URL not configured - outbox publishing disabled");
            }

            Some(factory)
        }
        Err(e) => {
            if !ui_only {
                return Err(std::io::Error::other(e));
            }
            warn!(error = %e, "Database not available - running in UI-only mode");
            None
        }
    };

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    info!(address = %addr, "Server starting");

    // Determine if we're in development or production
    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        // Build the base app with optional ServiceFactory
        let mut app = actix_web::App::new()
            .app_data(web::Data::new(session_verifier.clone()));

        // Add ServiceFactory if available
        if let Some(ref factory) = service_factory {
            app = app
                .app_data(web::Data::new(factory.clone()))
                // Register individual repositories and services for handler extraction
                // R5-01: Use Data::new() to match handler Data<DynXxx> expectations
                // Data::from(Arc<T>) creates Data<T>, but handlers expect Data<Arc<T>>
                // Data::new(Arc<T>) creates Data<Arc<T>> which matches handlers
                // Phase 7-8 repositories
                .app_data(web::Data::new(factory.support_repository.clone()))
                .app_data(web::Data::new(factory.cohort_repository.clone()))
                .app_data(web::Data::new(factory.exit_repository.clone()))
                .app_data(web::Data::new(factory.market_repository.clone()))
                .app_data(web::Data::new(factory.operator_metrics_repository.clone()))
                .app_data(web::Data::new(factory.forecast_repository.clone()))
                .app_data(web::Data::new(factory.webhook_repository.clone()))
                .app_data(web::Data::new(factory.communication_repository.clone()))
                .app_data(web::Data::new(factory.media_asset_repository.clone()))
                // R5-11: Media backup repository
                .app_data(web::Data::new(factory.media_backup_repository.clone()))
                // Phase 7-8 services
                .app_data(web::Data::new(factory.forecast_service.clone()))
                .app_data(web::Data::new(factory.webhook_service.clone()))
                .app_data(web::Data::new(factory.communication_service.clone()))
                .app_data(web::Data::new(factory.media_asset_service.clone()))
                // R5-11: Media backup service
                .app_data(web::Data::new(factory.media_backup_service.clone()))
                // B1 FIX: Register ConnectionPool for handlers that need direct DB access
                .app_data(web::Data::new(factory.pool.clone()));
        }

        // Increase JSON payload limit to 50MB for content with embedded images
        app = app.app_data(web::JsonConfig::default().limit(50 * 1024 * 1024));

        // Configure routes and middleware
        let logger = if is_production {
            RequestLogger::production()
        } else {
            RequestLogger::development()
        };

        app
            // API routes
            .configure(server::handlers::configure_api_routes)
            // Leptos server functions (for preview, etc.)
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // Static files
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", &site_root))
            .service(favicon)
            // Leptos routes
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en" dir="ltr">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                // Locale detection script - runs before hydration for SSR agreement
                                <script>
                                    r#"
                                    (function() {
                                        // RTL locales
                                        var rtlLocales = ['ar'];

                                        // Detect locale from cookie, then browser preference
                                        // R5-14: Cookie name must match server (uno-locale with hyphen)
                                        var locale = (document.cookie.match(/uno-locale=([^;]+)/) || [])[1] ||
                                                     (navigator.language || navigator.userLanguage || 'en').split('-')[0];

                                        // Store for hydration agreement
                                        window.__UNO_LOCALE__ = locale;

                                        // Set dir attribute for RTL support
                                        if (rtlLocales.indexOf(locale) !== -1) {
                                            document.documentElement.setAttribute('dir', 'rtl');
                                        }

                                        // Set lang attribute
                                        document.documentElement.setAttribute('lang', locale);
                                    })();
                                    "#
                                </script>
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
            // Add security headers (X-Frame-Options, CSP, etc.)
            .wrap(SecurityHeaders)
            // Add CSRF protection (skip API routes which use token auth)
            .wrap(CsrfProtection::new(
                csrf_secret.clone()
            ).skip_api(true))
            // Add visitor tracking middleware
            .wrap(VisitorTracker)
            // Add request logging middleware last (wraps all routes)
            .wrap(logger)
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
    // Serve PNG favicon (ico not available)
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.png"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // No client-side main function
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    use leptos::mount::mount_to_body;
    use uno_app::app::App;
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
