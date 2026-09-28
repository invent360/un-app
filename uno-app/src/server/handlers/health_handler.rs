//! Health check endpoint

use actix_web::{HttpResponse, web};
use serde::Serialize;
use crate::server::app::ServiceFactory;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub database: String,
}

/// Health check endpoint
pub async fn health_check(
    factory: Option<web::Data<ServiceFactory>>,
) -> HttpResponse {
    let db_status = match factory {
        Some(_) => "connected",
        None => "disconnected",
    };

    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status.to_string(),
    };

    HttpResponse::Ok().json(response)
}
