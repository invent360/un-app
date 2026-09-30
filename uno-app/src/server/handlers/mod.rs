//! HTTP request handlers

mod admin_handler;
mod agent_handler;
mod audit_handler;
mod content_admin_handler;
mod content_handler;
mod content_item_handler;
mod faq_handler;
mod file_handler;
mod health_handler;
mod licenses_handler;
mod lifecycle_handler;
mod rbac_handler;
mod review_admin_handler;
mod schema_handler;
mod session_handler;
mod stats_handler;
mod consent_handler;

use crate::server::middleware::{AdminAuth, RateLimitConfig, RateLimiter};
use actix_web::web;

pub use admin_handler::configure_admin_routes;
pub use audit_handler::*;
pub use content_admin_handler::*;
pub use content_handler::*;
pub use content_item_handler::*;
pub use faq_handler::*;
pub use file_handler::configure_routes as configure_file_routes;
pub use health_handler::*;
pub use licenses_handler::*;
pub use rbac_handler::*;
pub use review_admin_handler::*;
pub use schema_handler::*;
pub use session_handler::*;
pub use stats_handler::*;

/// Configure all API routes
pub fn configure_api_routes(cfg: &mut web::ServiceConfig) {
    // Configure admin API routes with nested content routes
    // Protected by AdminAuth middleware - requires valid API key in Authorization header
    // Rate limited at 1000 req/min to prevent runaway scripts
    cfg.service(
        web::scope("/api/v1/admin")
            .wrap(RateLimiter::new(RateLimitConfig::relaxed()))
            .wrap(AdminAuth::from_env().expect("ADMIN_API_KEY validated at startup"))
            // License admin routes
            .route("/licenses", web::post().to(admin_handler::publish_licenses))
            .route(
                "/licenses/import",
                web::post().to(admin_handler::import_csv),
            )
            .route(
                "/licenses/search",
                web::post().to(admin_handler::search_licenses),
            )
            .route(
                "/licenses/claimed",
                web::post().to(admin_handler::get_claimed_licenses),
            )
            .route(
                "/licenses",
                web::delete().to(admin_handler::revoke_licenses),
            )
            .route("/summary", web::post().to(admin_handler::get_summary))
            .route("/referrals", web::post().to(admin_handler::get_referrals))
            .route(
                "/referrals/sync",
                web::post().to(admin_handler::sync_referrals),
            )
            .route(
                "/stats/visitors",
                web::post().to(admin_handler::get_visitor_stats),
            )
            .route("/health", web::get().to(admin_handler::health))
            // Content admin routes
            .route(
                "/contents",
                web::post().to(content_admin_handler::create_content),
            )
            .route(
                "/contents/list",
                web::post().to(content_admin_handler::list_contents),
            )
            .route(
                "/contents/{id}",
                web::put().to(content_admin_handler::update_content),
            )
            .route(
                "/contents/{id}/get",
                web::post().to(content_admin_handler::get_content),
            )
            .route(
                "/contents/{id}/delete",
                web::post().to(content_admin_handler::delete_content),
            )
            .route(
                "/contents/{id}/publish",
                web::post().to(content_admin_handler::publish_content),
            )
            .route(
                "/contents/{id}/archive",
                web::post().to(content_admin_handler::archive_content),
            )
            .route(
                "/contents/{id}/revert",
                web::post().to(content_admin_handler::revert_content),
            )
            .route(
                "/contents/{id}/schedule",
                web::put().to(content_admin_handler::update_content_schedule),
            )
            .route(
                "/contents/{id}/audit",
                web::post().to(audit_handler::get_content_audit_history),
            )
            // Audit log routes
            .route(
                "/audit-logs",
                web::post().to(audit_handler::list_audit_logs),
            )
            // RBAC routes
            .route("/roles", web::post().to(rbac_handler::list_roles))
            .route(
                "/users/me/permissions",
                web::post().to(rbac_handler::get_my_permissions),
            )
            .route(
                "/users/{user_id}/permissions",
                web::post().to(rbac_handler::get_user_permissions),
            )
            .route(
                "/users/roles/assign",
                web::post().to(rbac_handler::assign_role),
            )
            .route(
                "/users/roles/remove",
                web::post().to(rbac_handler::remove_role),
            )
            // Review workflow routes
            .route(
                "/reviews/submit",
                web::post().to(review_admin_handler::submit_for_review),
            )
            .route(
                "/reviews/pending",
                web::post().to(review_admin_handler::get_pending_reviews),
            )
            .route(
                "/reviews/my-submissions",
                web::post().to(review_admin_handler::get_my_submissions),
            )
            .route(
                "/reviews/{id}/approve",
                web::post().to(review_admin_handler::approve_review),
            )
            .route(
                "/reviews/{id}/request-changes",
                web::post().to(review_admin_handler::request_changes),
            )
            .route(
                "/reviews/{id}/reject",
                web::post().to(review_admin_handler::reject_review),
            )
            // Preview tokens
            .route(
                "/preview-tokens",
                web::post().to(review_admin_handler::create_preview_token),
            )
            // Publish routes
            .route(
                "/publish/direct",
                web::post().to(review_admin_handler::publish_direct),
            )
            .route(
                "/publish/batch",
                web::post().to(review_admin_handler::publish_batch),
            )
            // Version history routes
            .route(
                "/versions/{content_id}/history",
                web::post().to(review_admin_handler::get_version_history),
            )
            .route(
                "/versions/{content_id}/compare",
                web::post().to(review_admin_handler::compare_versions),
            )
            .route(
                "/versions/{content_id}/revert",
                web::post().to(review_admin_handler::revert_to_version),
            )
            // Schema admin routes (schema-driven CMS)
            .route("/schemas", web::post().to(schema_handler::create_schema))
            .route(
                "/schemas/{id}",
                web::put().to(schema_handler::update_schema),
            )
            .route(
                "/schemas/{id}/delete",
                web::post().to(schema_handler::delete_schema),
            )
            // Content item admin routes (schema-driven CMS)
            .route("/items", web::post().to(content_item_handler::create_item))
            .route(
                "/items/list",
                web::post().to(content_item_handler::list_items),
            )
            .route(
                "/items/bulk/status",
                web::post().to(content_item_handler::bulk_update_status),
            )
            .route(
                "/items/reorder",
                web::post().to(content_item_handler::reorder_items),
            )
            .route(
                "/items/{id}",
                web::put().to(content_item_handler::update_item),
            )
            .route(
                "/items/{id}/get",
                web::post().to(content_item_handler::get_item),
            )
            .route(
                "/items/{id}/delete",
                web::post().to(content_item_handler::delete_item),
            )
            .route(
                "/items/{id}/publish",
                web::post().to(content_item_handler::publish_item),
            )
            .route(
                "/items/{id}/archive",
                web::post().to(content_item_handler::archive_item),
            )
            .route(
                "/items/{id}/versions",
                web::post().to(content_item_handler::get_item_versions),
            )
            .route(
                "/items/{id}/revert",
                web::post().to(content_item_handler::revert_item),
            )
            .route(
                "/items/{id}/schedule",
                web::put().to(content_item_handler::update_item_schedule),
            )
            .route(
                "/items/{id}/translations",
                web::put().to(content_item_handler::update_item_translation),
            )
            // Agent workflow routes (Phase 4)
            .route("/agents", web::get().to(agent_handler::list_agents))
            .route("/agents/pending", web::get().to(agent_handler::get_pending_approvals))
            .route("/agents/summary", web::get().to(agent_handler::get_summary))
            .route("/agents/{id}", web::get().to(agent_handler::get_agent))
            .route("/agents/{id}/approve", web::post().to(agent_handler::approve_agent))
            .route("/agents/{id}/reject", web::post().to(agent_handler::reject_agent))
            .route("/agents/{id}/suspend", web::post().to(agent_handler::suspend_agent))
            .route("/agents/{id}/lift-suspension", web::post().to(agent_handler::lift_suspension))
            .route("/agents/{id}/terminate", web::post().to(agent_handler::terminate_agent))
            .route("/agents/{id}/history", web::get().to(agent_handler::get_status_history))
            // License lifecycle routes (Phase 4)
            .route("/licenses/{id}/cancel", web::post().to(lifecycle_handler::cancel_license))
            .route("/licenses/{id}/release", web::post().to(lifecycle_handler::release_license))
            .route("/licenses/{id}/reactivate", web::post().to(lifecycle_handler::reactivate_license))
            .route("/licenses/{id}/exposure", web::get().to(lifecycle_handler::get_exposure))
            .route("/licenses/{id}/lifecycle", web::get().to(lifecycle_handler::get_lifecycle_history))
            .route("/licenses/pending-expiry", web::get().to(lifecycle_handler::get_pending_expiry))
            .route("/licenses/process-expired", web::post().to(lifecycle_handler::process_expired)),
    );

    // Public preview endpoint (no HMAC required)
    cfg.route(
        "/api/v1/preview/{token}",
        web::get().to(review_admin_handler::get_preview_content),
    );

    // Cloud file storage routes (GCS/S3)
    file_handler::configure_routes(cfg);

    // Sensitive auth endpoints with strict rate limits (10 req/min)
    cfg.service(
        web::scope("/api/v1/auth")
            .wrap(RateLimiter::new(RateLimitConfig::strict()))
            .route("/logout", web::post().to(session_handler::logout))
            .route("/validate", web::post().to(session_handler::validate_session))
            .route("/touch", web::post().to(session_handler::touch_session)),
    );

    // License claim endpoint with strict rate limits (10 req/min per IP)
    cfg.service(
        web::scope("/api/v1/licenses")
            .wrap(RateLimiter::new(RateLimitConfig::strict()))
            .route("/variants", web::get().to(get_variants))
            .route("/claim", web::post().to(claim_license)),
    );

    // Consent endpoints with standard rate limits
    cfg.service(
        web::scope("/api/v1/consent")
            .wrap(RateLimiter::new(RateLimitConfig::default()))
            .route("/required", web::get().to(consent_handler::get_required_consents))
            .route("/status", web::get().to(consent_handler::get_consent_status))
            .route("/record", web::post().to(consent_handler::record_consent))
            .route("/withdraw", web::post().to(consent_handler::withdraw_consent)),
    );

    // Privacy/GDPR endpoints with strict rate limits
    cfg.service(
        web::scope("/api/v1/privacy")
            .wrap(RateLimiter::new(RateLimitConfig::strict()))
            .route("/request", web::post().to(consent_handler::create_data_request))
            .route("/requests", web::get().to(consent_handler::get_data_requests))
            .route("/retention-policies", web::get().to(consent_handler::get_retention_policies)),
    );

    // Then configure public API routes with standard rate limiting (100 req/min)
    cfg.service(
        web::scope("/api/v1")
            .wrap(RateLimiter::new(RateLimitConfig::default()))
            // Health check endpoints
            .route("/health", web::get().to(health_check))         // Legacy
            .route("/ready", web::get().to(readiness))             // Kubernetes readiness
            .route("/live", web::get().to(liveness))               // Kubernetes liveness
            .route("/readiness", web::get().to(readiness))         // Explicit readiness
            .route("/liveness", web::get().to(liveness))           // Explicit liveness
            .route("/integration-health", web::get().to(integration_health)) // Dashboard
            // FAQ
            .route("/faq", web::get().to(faq_handler::get_faqs))
            .route("/faq/search", web::get().to(faq_handler::search_faqs))
            .route(
                "/faq/categories",
                web::get().to(faq_handler::get_categories),
            )
            .route("/faq/featured", web::get().to(faq_handler::get_featured))
            .route("/faq/{id}", web::get().to(faq_handler::get_faq_by_id))
            // Stats
            .route(
                "/stats/visitors",
                web::get().to(stats_handler::get_visitor_stats),
            )
            .route(
                "/stats/countries",
                web::get().to(stats_handler::get_country_stats),
            )
            .route(
                "/stats/dashboard",
                web::get().to(stats_handler::get_dashboard_stats),
            )
            // CMS Content (public, legacy - page_contents table)
            .route(
                "/contents/{type}",
                web::get().to(content_handler::get_contents_by_type),
            )
            .route(
                "/contents/{type}/featured",
                web::get().to(content_handler::get_featured_contents),
            )
            .route(
                "/contents/{type}/search",
                web::get().to(content_handler::search_contents),
            )
            .route(
                "/contents/{type}/{slug}",
                web::get().to(content_handler::get_content_by_slug),
            )
            // Schema-driven CMS (public)
            .route("/schemas", web::get().to(schema_handler::get_schemas))
            .route("/schemas/{id}", web::get().to(schema_handler::get_schema))
            // Content items (public, schema-driven)
            .route(
                "/items/{schema_id}",
                web::get().to(content_item_handler::get_items_by_schema),
            )
            .route(
                "/items/{schema_id}/featured",
                web::get().to(content_item_handler::get_featured_items),
            )
            .route(
                "/items/{schema_id}/{slug}",
                web::get().to(content_item_handler::get_item_by_slug),
            ),
    );
}
