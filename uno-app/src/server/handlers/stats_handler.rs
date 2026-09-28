//! Stats-related API handlers

use actix_web::{HttpResponse, web};
use crate::types::{StatsQueryParams, PeriodFilter};
use crate::server::app::ServiceFactory;

/// GET /api/v1/stats/visitors
/// Returns visitor statistics with optional period filter
pub async fn get_visitor_stats(
    factory: Option<web::Data<ServiceFactory>>,
    query: web::Query<StatsQueryParams>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Database not available",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    let period = query.period_filter();

    match factory.stats_service.get_visitor_stats(period).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/stats/countries
/// Returns top countries by visitor count
pub async fn get_country_stats(
    factory: Option<web::Data<ServiceFactory>>,
    query: web::Query<StatsQueryParams>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Database not available",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    let period = query.period_filter();
    let limit = query.get_country_limit();

    match factory.stats_service.get_top_countries(period, limit).await {
        Ok(countries) => HttpResponse::Ok().json(countries),
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/stats/dashboard
/// Returns dashboard visitor statistics
pub async fn get_dashboard_stats(
    factory: Option<web::Data<ServiceFactory>>,
    query: web::Query<StatsQueryParams>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Database not available",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    let period = query.period_filter();

    match factory.stats_service.get_dashboard_stats(period).await {
        Ok(dashboard) => HttpResponse::Ok().json(dashboard),
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}
