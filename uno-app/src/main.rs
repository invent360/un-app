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
    let is_production = std::env::var("RUST_ENV")
        .is_ok_and(|value| value.eq_ignore_ascii_case("production"))
        || std::env::var("LEPTOS_ENV").is_ok_and(|value| value.eq_ignore_ascii_case("PROD"));
    let ui_only = !is_production && std::env::var("UNO_UI_ONLY").is_ok_and(|value| value == "true");

    // Register server functions explicitly
    register_faq_server_fns();
    register_guides_server_fns();
    register_tasks_server_fns();
    register_home_server_fns();

    // Initialize logging
    init_logging();

    info!("Starting UNO Web Application...");

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
            app = app.app_data(web::Data::new(factory.clone()));
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
