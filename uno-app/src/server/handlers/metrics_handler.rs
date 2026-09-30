//! Prometheus metrics handler
//!
//! Exposes the /metrics endpoint for Prometheus scraping.
//! Phase 9: Release Validation monitoring infrastructure.

use actix_web::{HttpResponse, web};
use crate::server::metrics;

/// GET /metrics - Prometheus metrics endpoint
///
/// Returns all registered metrics in Prometheus text format.
/// This endpoint should be protected from public access in production.
pub async fn get_metrics() -> HttpResponse {
    let body = metrics::encode_metrics();

    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body(body)
}

/// Configure metrics routes
pub fn configure_metrics_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/metrics")
            .route(web::get().to(get_metrics))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_metrics_endpoint() {
        let app = test::init_service(
            App::new().configure(configure_metrics_routes)
        ).await;

        let req = test::TestRequest::get().uri("/metrics").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();

        // Verify Prometheus format
        assert!(body_str.contains("# HELP") || body_str.is_empty() || body_str.contains("uno_"));
    }

    #[actix_web::test]
    async fn test_metrics_content_type() {
        let app = test::init_service(
            App::new().configure(configure_metrics_routes)
        ).await;

        let req = test::TestRequest::get().uri("/metrics").to_request();
        let resp = test::call_service(&app, req).await;

        let content_type = resp
            .headers()
            .get("content-type")
            .map(|v| v.to_str().unwrap_or(""))
            .unwrap_or("");

        assert!(content_type.contains("text/plain"));
    }
}
