//! Health probe handlers for Kubernetes-style readiness/liveness checks
//!
//! Provides endpoints for:
//! - GET /health       - Full health check (all components)
//! - GET /health/live  - Liveness probe (is process alive?)
//! - GET /health/ready - Readiness probe (can handle traffic?)

#[cfg(feature = "ssr")]
use actix_web::{web, HttpResponse};

#[cfg(feature = "ssr")]
use crate::logic::health::{HealthService, HealthStatus};

/// Liveness probe - checks if process is alive
///
/// Returns 200 OK if the process is running.
/// This should be fast and not depend on external services.
#[cfg(feature = "ssr")]
pub async fn liveness() -> HttpResponse {
    // get_db() already returns Option<Arc<Session>>
    let pool = crate::db::get_db();
    let keyspace = std::env::var("SCYLLA_KEYSPACE").ok();

    let service = HealthService::new(pool, keyspace);
    let health = service.liveness().await;

    HttpResponse::Ok().json(health)
}

/// Readiness probe - checks if service can handle traffic
///
/// Returns 200 OK if ready, 503 Service Unavailable if not.
/// Includes database connectivity check.
#[cfg(feature = "ssr")]
pub async fn readiness() -> HttpResponse {
    // get_db() already returns Option<Arc<Session>>
    let pool = crate::db::get_db();
    let keyspace = std::env::var("SCYLLA_KEYSPACE").ok();

    let service = HealthService::new(pool, keyspace);
    let health = service.readiness().await;

    if health.is_ready() {
        HttpResponse::Ok().json(health)
    } else {
        HttpResponse::ServiceUnavailable().json(health)
    }
}

/// Full health check - comprehensive status of all components
///
/// Returns 200 OK if healthy, 503 if unhealthy, 200 with degraded status if partially healthy.
#[cfg(feature = "ssr")]
pub async fn full_health() -> HttpResponse {
    // get_db() already returns Option<Arc<Session>>
    let pool = crate::db::get_db();
    let keyspace = std::env::var("SCYLLA_KEYSPACE").ok();

    let service = HealthService::new(pool, keyspace);
    let health = service.full().await;

    match health.status {
        HealthStatus::Healthy => HttpResponse::Ok().json(health),
        HealthStatus::Degraded => HttpResponse::Ok().json(health),
        HealthStatus::Unhealthy => HttpResponse::ServiceUnavailable().json(health),
    }
}

/// Configure health check routes
#[cfg(feature = "ssr")]
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(full_health))
            .route("/live", web::get().to(liveness))
            .route("/ready", web::get().to(readiness)),
    );
}
