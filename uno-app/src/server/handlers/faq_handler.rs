//! FAQ-related API handlers

use actix_web::{HttpResponse, web};
use crate::types::FaqSearchParams;
use crate::server::app::ServiceFactory;

/// GET /api/v1/faq
/// Returns all FAQ items with optional filtering
pub async fn get_faqs(
    factory: Option<web::Data<ServiceFactory>>,
    query: web::Query<FaqSearchParams>,
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

    match factory.faq_service.get_faqs(query.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            HttpResponse::InternalServerError().json(e.error_response())
        }
    }
}

/// Search query parameters
#[derive(Debug, serde::Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default)]
    pub locale: Option<String>,
}

/// GET /api/v1/faq/search?q={query}
/// Search FAQ items by query string
pub async fn search_faqs(
    factory: Option<web::Data<ServiceFactory>>,
    query: web::Query<SearchQuery>,
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

    match factory.faq_service.search_faqs(&query.q, query.locale.as_deref()).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            HttpResponse::InternalServerError().json(e.error_response())
        }
    }
}

/// GET /api/v1/faq/categories
/// Returns all FAQ categories with counts
pub async fn get_categories(
    factory: Option<web::Data<ServiceFactory>>,
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

    match factory.faq_service.get_categories().await {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => {
            HttpResponse::InternalServerError().json(e.error_response())
        }
    }
}

/// Locale query parameter
#[derive(Debug, serde::Deserialize)]
pub struct LocaleQuery {
    #[serde(default)]
    pub locale: Option<String>,
}

/// GET /api/v1/faq/featured
/// Returns featured FAQ items
pub async fn get_featured(
    factory: Option<web::Data<ServiceFactory>>,
    query: web::Query<LocaleQuery>,
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

    match factory.faq_service.get_featured(query.locale.as_deref()).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            HttpResponse::InternalServerError().json(e.error_response())
        }
    }
}

/// GET /api/v1/faq/{id}
/// Returns a single FAQ item by ID
pub async fn get_faq_by_id(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<i32>,
    query: web::Query<LocaleQuery>,
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

    let id = path.into_inner();

    match factory.faq_service.get_faq_by_id(id, query.locale.as_deref()).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "FAQ item not found",
            "code": "NOT_FOUND"
        })),
        Err(e) => {
            HttpResponse::InternalServerError().json(e.error_response())
        }
    }
}
