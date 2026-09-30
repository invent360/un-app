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
mod finance_handler;
mod support_handler;
mod cohort_handler;
mod market_handler;
mod exit_handler;
mod dashboard_handler;
mod operator_handler;
mod forecast_handler;
mod webhook_handler;
mod communication_handler;
mod metrics_handler;
mod pilot_handler;
mod media_backup_handler;
mod onboarding_handler;

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
            .route("/licenses/process-expired", web::post().to(lifecycle_handler::process_expired))
            // Finance routes (Phase 5)
            .route("/finance/allocations", web::get().to(finance_handler::list_allocations))
            .route("/finance/allocations", web::post().to(finance_handler::create_allocation))
            .route("/finance/allocations/{id}", web::get().to(finance_handler::get_allocation))
            .route("/finance/allocations/mark-payable", web::post().to(finance_handler::mark_payable))
            .route("/finance/licenses/{id}/balance", web::get().to(finance_handler::get_pool_balance))
            .route("/finance/payable", web::get().to(finance_handler::get_payable_balances))
            .route("/finance/settlements", web::get().to(finance_handler::list_settlements))
            .route("/finance/settlements", web::post().to(finance_handler::prepare_settlement))
            .route("/finance/settlements/{ref}", web::get().to(finance_handler::get_settlement))
            .route("/finance/settlements/{ref}/approve", web::post().to(finance_handler::approve_settlement))
            .route("/finance/settlements/{ref}/execute", web::post().to(finance_handler::execute_settlement))
            // Support admin routes (Phase 7)
            .route("/support/tickets", web::post().to(support_handler::search_tickets))
            .route("/support/tickets/{id}", web::get().to(support_handler::get_ticket_by_number))
            .route("/support/tickets/{id}/assign", web::post().to(support_handler::assign_ticket))
            .route("/support/tickets/{id}/escalate", web::post().to(support_handler::escalate_ticket))
            .route("/support/tickets/{id}/resolve", web::post().to(support_handler::resolve_ticket))
            .route("/support/queue", web::get().to(support_handler::get_agent_queue))
            .route("/support/queue/unassigned", web::get().to(support_handler::get_unassigned_tickets))
            // Cohort admin routes (Phase 7)
            .route("/cohorts/date-range", web::get().to(cohort_handler::get_cohorts_by_date_range))
            .route("/cohorts/stats", web::get().to(cohort_handler::get_cohort_stats))
            .route("/cohorts/analytics", web::post().to(cohort_handler::calculate_analytics))
            .route("/cohorts/analytics", web::get().to(cohort_handler::get_analytics))
            .route("/cohorts/pending/d1", web::get().to(cohort_handler::get_cohorts_pending_d1))
            .route("/cohorts/pending/d7", web::get().to(cohort_handler::get_cohorts_pending_d7))
            .route("/cohorts/pending/d30", web::get().to(cohort_handler::get_cohorts_pending_d30))
            .route("/cohorts/{id}/notifications", web::post().to(cohort_handler::schedule_notification))
            .route("/cohorts/{id}/notifications", web::get().to(cohort_handler::get_cohort_notifications))
            .route("/notifications/pending", web::get().to(cohort_handler::get_pending_notifications))
            .route("/notifications/{id}/sent", web::post().to(cohort_handler::mark_notification_sent))
            .route("/notifications/{id}/skip", web::post().to(cohort_handler::skip_notification))
            // Market admin routes (Phase 7)
            .route("/markets", web::get().to(market_handler::get_all_markets))
            .route("/markets/{code}", web::get().to(market_handler::get_market_status))
            .route("/markets/{code}", web::post().to(market_handler::upsert_market))
            .route("/markets/{code}/pause", web::post().to(market_handler::pause_market))
            .route("/markets/{code}/resume", web::post().to(market_handler::resume_market))
            .route("/markets/{code}/content-review", web::post().to(market_handler::update_content_review))
            .route("/markets/{code}/support-readiness", web::post().to(market_handler::update_support_readiness))
            .route("/quotas/{code}", web::get().to(market_handler::get_country_quotas))
            .route("/quotas", web::post().to(market_handler::create_quota))
            .route("/quotas/reset", web::post().to(market_handler::reset_daily_quotas))
            .route("/campaigns", web::get().to(market_handler::get_active_campaigns))
            .route("/campaigns", web::post().to(market_handler::create_campaign))
            .route("/campaigns/{id}", web::get().to(market_handler::get_campaign))
            .route("/campaigns/{id}/deactivate", web::post().to(market_handler::deactivate_campaign))
            .route("/attributions/{user_id}", web::get().to(market_handler::get_user_attributions))
            // Exit admin routes (Phase 7)
            .route("/exits", web::get().to(exit_handler::get_exits_by_status))
            .route("/exits/{id}", web::get().to(exit_handler::get_exit))
            .route("/exits/{id}/approve", web::post().to(exit_handler::approve_exit))
            .route("/exits/{id}/reject", web::post().to(exit_handler::reject_exit))
            .route("/exits/{id}/payout/start", web::post().to(exit_handler::initiate_payout))
            .route("/exits/{id}/payout/complete", web::post().to(exit_handler::complete_payout))
            .route("/exits/{id}/payout/fail", web::post().to(exit_handler::fail_payout))
            .route("/exits/{id}/audit", web::get().to(exit_handler::get_audit_log))
            .route("/waitlist/{code}", web::get().to(exit_handler::get_country_waitlist))
            .route("/waitlist/notify", web::post().to(exit_handler::get_waitlist_to_notify))
            // Operator dashboard routes (Phase 8)
            .route("/operator/dashboard", web::get().to(operator_handler::get_dashboard))
            .route("/operator/inventory", web::get().to(operator_handler::get_inventory))
            .route("/operator/cohorts", web::get().to(operator_handler::get_cohorts))
            .route("/operator/finance", web::get().to(operator_handler::get_finance))
            .route("/operator/sync", web::get().to(operator_handler::get_sync))
            .route("/operator/exceptions", web::get().to(operator_handler::get_exceptions))
            .route("/operator/exceptions", web::post().to(operator_handler::record_exception))
            .route("/operator/exceptions/{id}/resolve", web::post().to(operator_handler::resolve_exception))
            .route("/operator/metrics", web::get().to(operator_handler::get_metrics))
            .route("/operator/metrics/range", web::get().to(operator_handler::get_metrics_range))
            .route("/operator/metrics/aggregate", web::post().to(operator_handler::aggregate_metrics))
            .route("/operator/metrics/export", web::get().to(operator_handler::export_metrics))
            // Forecast routes (Phase 8)
            .route("/forecasts", web::get().to(forecast_handler::list_scenarios))
            .route("/forecasts", web::post().to(forecast_handler::create_scenario))
            .route("/forecasts/import", web::post().to(forecast_handler::import_scenario))
            .route("/forecasts/golden", web::get().to(forecast_handler::list_golden_fixtures))
            // NOTE: get_golden_fixture not yet implemented
            .route("/forecasts/{id}", web::get().to(forecast_handler::get_scenario))
            .route("/forecasts/{id}", web::put().to(forecast_handler::update_scenario))
            .route("/forecasts/{id}", web::delete().to(forecast_handler::archive_scenario))
            .route("/forecasts/{id}/approve", web::post().to(forecast_handler::approve_scenario))
            .route("/forecasts/{id}/tasks", web::post().to(forecast_handler::add_task))
            .route("/forecasts/{scenario_id}/tasks/{task_id}", web::put().to(forecast_handler::update_task))
            .route("/forecasts/{scenario_id}/tasks/{task_id}", web::delete().to(forecast_handler::remove_task))
            .route("/forecasts/{id}/run", web::post().to(forecast_handler::run_forecast))
            .route("/forecasts/{id}/results", web::get().to(forecast_handler::get_results))
            .route("/forecasts/{id}/export", web::get().to(forecast_handler::export_scenario))
            .route("/forecasts/{id}/validate", web::post().to(forecast_handler::validate_scenario))
            .route("/forecasts/{id}/golden", web::post().to(forecast_handler::create_golden_fixture))
            // Webhook admin routes (Phase 8)
            .route("/webhooks/endpoints", web::get().to(webhook_handler::list_endpoints))
            .route("/webhooks/endpoints", web::post().to(webhook_handler::create_endpoint))
            .route("/webhooks/endpoints/{id}", web::get().to(webhook_handler::get_endpoint))
            .route("/webhooks/endpoints/{id}", web::put().to(webhook_handler::update_endpoint))
            .route("/webhooks/endpoints/{id}", web::delete().to(webhook_handler::delete_endpoint))
            .route("/webhooks/endpoints/{id}/verify", web::post().to(webhook_handler::verify_endpoint))
            .route("/webhooks/endpoints/{id}/toggle", web::post().to(webhook_handler::toggle_endpoint))
            // NOTE: reset_circuit_breaker not yet implemented
            .route("/webhooks/endpoints/{id}/deliveries", web::get().to(webhook_handler::get_deliveries))
            .route("/webhooks/process", web::post().to(webhook_handler::process_deliveries))
            .route("/webhooks/retry", web::post().to(webhook_handler::retry_deliveries))
            // NOTE: list_inbound_webhooks not yet implemented
            .route("/webhooks/inbound/process", web::post().to(webhook_handler::process_inbound))
            .route("/webhooks/sources", web::get().to(webhook_handler::list_sources))
            .route("/webhooks/sources", web::post().to(webhook_handler::create_source))
            // Communication admin routes (Phase 8)
            .route("/communication/templates", web::get().to(communication_handler::list_templates))
            .route("/communication/templates", web::post().to(communication_handler::create_template))
            .route("/communication/templates/{id}", web::put().to(communication_handler::update_template))
            .route("/communication/templates/{id}/approve", web::post().to(communication_handler::approve_template))
            .route("/communication/suppressions", web::get().to(communication_handler::list_suppressions))
            .route("/communication/suppressions", web::post().to(communication_handler::add_suppression))
            .route("/communication/suppressions", web::delete().to(communication_handler::remove_suppression))
            .route("/communication/suppressions/check", web::get().to(communication_handler::check_suppression))
            .route("/communication/send", web::post().to(communication_handler::send_immediate))
            .route("/communication/scheduled", web::get().to(communication_handler::list_scheduled))
            .route("/communication/scheduled", web::post().to(communication_handler::schedule_message))
            .route("/communication/scheduled/{id}", web::delete().to(communication_handler::cancel_scheduled))
            .route("/communication/process", web::post().to(communication_handler::process_scheduled))
            .route("/communication/can-receive", web::get().to(communication_handler::can_receive))
            // Phase 9: Pilot management
            .route("/pilot/cohorts", web::get().to(pilot_handler::list_cohorts))
            .route("/pilot/cohorts/{id}", web::get().to(pilot_handler::get_cohort))
            .route("/pilot/cohorts/{id}/participants", web::get().to(pilot_handler::list_participants))
            .route("/pilot/participants", web::post().to(pilot_handler::add_participant))
            .route("/pilot/participants/{id}", web::get().to(pilot_handler::get_participant))
            .route("/pilot/participants/{id}/transition", web::post().to(pilot_handler::transition_state))
            .route("/pilot/participants/{id}/activity", web::post().to(pilot_handler::record_activity))
            .route("/pilot/participants/{id}/history", web::get().to(pilot_handler::get_history))
            .route("/pilot/participants/{id}/d7", web::post().to(pilot_handler::process_d7))
            .route("/pilot/participants/{id}/d30", web::post().to(pilot_handler::process_d30))
            .route("/pilot/pending/d7", web::get().to(pilot_handler::get_pending_d7))
            .route("/pilot/pending/d30", web::get().to(pilot_handler::get_pending_d30))
            // R3-13: Onboarding journey admin routes
            .route("/onboarding/funnel", web::get().to(onboarding_handler::get_funnel_metrics))
            .route("/onboarding/funnel/record", web::post().to(onboarding_handler::record_funnel_stage))
            .route("/onboarding/{user_id}/progress", web::get().to(onboarding_handler::get_progress))
            .route("/onboarding/{user_id}/transition", web::post().to(onboarding_handler::transition_state)),
    );

    // Public preview endpoint (no HMAC required)
    cfg.route(
        "/api/v1/preview/{token}",
        web::get().to(review_admin_handler::get_preview_content),
    );

    // Cloud file storage routes (GCS/S3)
    file_handler::configure_routes(cfg);

    // R4-06: Backup/restore routes
    media_backup_handler::configure_routes(cfg);

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

    // Support ticket endpoints (Phase 7)
    cfg.service(
        web::scope("/api/v1/support")
            .wrap(RateLimiter::new(RateLimitConfig::default()))
            .route("/tickets", web::post().to(support_handler::create_ticket))
            .route("/tickets/{id}", web::get().to(support_handler::get_ticket))
            .route("/tickets/{id}/messages", web::get().to(support_handler::get_messages))
            .route("/tickets/{id}/messages", web::post().to(support_handler::add_message))
            .route("/my-tickets", web::get().to(support_handler::get_user_tickets)),
    );

    // Cohort tracking endpoints (Phase 7)
    cfg.service(
        web::scope("/api/v1/cohort")
            .wrap(RateLimiter::new(RateLimitConfig::default()))
            .route("", web::post().to(cohort_handler::create_cohort))
            .route("/{id}", web::get().to(cohort_handler::get_cohort))
            .route("/user/{user_id}", web::get().to(cohort_handler::get_user_cohorts))
            .route("/user/{user_id}/license/{license_id}", web::get().to(cohort_handler::get_cohort_by_user_license))
            .route("/{id}/progress", web::get().to(cohort_handler::get_cohort_progress))
            .route("/{id}/activity", web::post().to(cohort_handler::record_activity))
            .route("/{id}/activities", web::get().to(cohort_handler::get_cohort_activities))
            .route("/{id}/d1", web::post().to(cohort_handler::complete_d1))
            .route("/{id}/d3", web::post().to(cohort_handler::complete_d3)),
    );

    // Market status endpoints (Phase 7)
    cfg.service(
        web::scope("/api/v1/market")
            .wrap(RateLimiter::new(RateLimitConfig::default()))
            .route("/{code}/status", web::get().to(market_handler::get_market_status))
            .route("/{code}/readiness", web::get().to(market_handler::check_market_readiness))
            .route("/{code}/quota", web::get().to(market_handler::check_quota))
            .route("/attribution", web::post().to(market_handler::record_attribution)),
    );

    // Exit endpoints (Phase 7)
    cfg.service(
        web::scope("/api/v1/exit")
            .wrap(RateLimiter::new(RateLimitConfig::strict()))
            .route("", web::post().to(exit_handler::initiate_exit))
            .route("/license/{id}", web::get().to(exit_handler::get_exit_by_license))
            .route("/balance", web::get().to(exit_handler::calculate_balance))
            .route("/payout", web::post().to(exit_handler::request_payout))
            .route("/feedback", web::post().to(exit_handler::submit_feedback))
            .route("/waitlist", web::post().to(exit_handler::add_to_waitlist)),
    );

    // Participant dashboard endpoints (Phase 7)
    cfg.service(
        web::scope("/api/v1/dashboard")
            .wrap(RateLimiter::new(RateLimitConfig::default()))
            .route("", web::get().to(dashboard_handler::get_dashboard))
            .route("/progress/{user_id}/{license_id}", web::get().to(dashboard_handler::get_progress))
            .route("/activities/{user_id}/{license_id}", web::get().to(dashboard_handler::get_activities)),
    );

    // Communication preferences endpoints (Phase 8)
    cfg.service(
        web::scope("/api/v1/communication")
            .wrap(RateLimiter::new(RateLimitConfig::default()))
            .route("/preferences", web::get().to(communication_handler::get_preferences))
            .route("/preferences", web::put().to(communication_handler::update_preferences))
            .route("/opt-out", web::post().to(communication_handler::opt_out))
            .route("/devices", web::post().to(communication_handler::register_device)),
    );

    // Pilot enrollment check (Phase 9) - public
    cfg.service(
        web::scope("/api/v1/pilot")
            .wrap(RateLimiter::new(RateLimitConfig::default()))
            .route("/{market_code}/check", web::get().to(pilot_handler::check_enrollment)),
    );

    // R3-13: Onboarding journey endpoints (user-facing)
    cfg.service(
        web::scope("/api/v1/onboarding")
            .wrap(RateLimiter::new(RateLimitConfig::default()))
            .route("/progress", web::get().to(onboarding_handler::get_or_create_progress))
            .route("/resume", web::post().to(onboarding_handler::resume_onboarding))
            .route("/eligibility", web::post().to(onboarding_handler::complete_eligibility))
            .route("/economics", web::post().to(onboarding_handler::accept_economics))
            .route("/setup", web::post().to(onboarding_handler::complete_setup_step))
            .route("/reserve", web::post().to(onboarding_handler::reserve_license))
            .route("/activate", web::post().to(onboarding_handler::activate_license)),
    );

    // Inbound webhook endpoint (Phase 8) - public with signature validation
    cfg.service(
        web::scope("/api/v1/webhooks")
            .wrap(RateLimiter::new(RateLimitConfig::default()))
            .route("/inbound/{source}", web::post().to(webhook_handler::receive_webhook)),
    );

    // Prometheus metrics endpoint (no rate limiting for scraping)
    cfg.route("/metrics", web::get().to(metrics_handler::get_metrics));

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
