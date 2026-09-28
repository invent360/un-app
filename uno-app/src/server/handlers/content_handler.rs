//! Public content API handlers

use actix_web::{HttpResponse, web};
use crate::server::app::ServiceFactory;

/// Query parameters for content list
#[derive(Debug, serde::Deserialize)]
pub struct ContentQuery {
    #[serde(default)]
    pub locale: Option<String>,
}

/// Search query parameters
#[derive(Debug, serde::Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default)]
    pub locale: Option<String>,
}

/// GET /api/v1/contents/{type}
/// Returns all published content of a type
pub async fn get_contents_by_type(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    query: web::Query<ContentQuery>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "error": "Service unavailable",
            "code": "SERVICE_UNAVAILABLE"
        })),
    };

    let content_type = path.into_inner();
    let locale = query.locale.as_deref().unwrap_or("en");

    match factory.content_service.get_contents_by_type(&content_type, locale).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            tracing::error!("Failed to get contents by type: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/contents/{type}/featured
/// Returns featured content of a type
pub async fn get_featured_contents(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    query: web::Query<ContentQuery>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "error": "Service unavailable",
            "code": "SERVICE_UNAVAILABLE"
        })),
    };

    let content_type = path.into_inner();
    let locale = query.locale.as_deref().unwrap_or("en");

    match factory.content_service.get_featured_contents(&content_type, locale).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            tracing::error!("Failed to get featured contents: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/contents/{type}/{slug}
/// Returns single published content by slug
pub async fn get_content_by_slug(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<(String, String)>,
    query: web::Query<ContentQuery>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "error": "Service unavailable",
            "code": "SERVICE_UNAVAILABLE"
        })),
    };

    let (content_type, slug) = path.into_inner();
    let locale = query.locale.as_deref().unwrap_or("en");

    match factory.content_service.get_content_by_slug(&content_type, &slug, locale).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Content not found",
            "code": "NOT_FOUND"
        })),
        Err(e) => {
            tracing::error!("Failed to get content by slug: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/contents/{type}/search
/// Search content by query
pub async fn search_contents(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    query: web::Query<SearchQuery>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "error": "Service unavailable",
            "code": "SERVICE_UNAVAILABLE"
        })),
    };

    let content_type = path.into_inner();
    let locale = query.locale.as_deref().unwrap_or("en");

    match factory.content_service.search_contents(&content_type, &query.q, locale).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            tracing::error!("Failed to search contents: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}
