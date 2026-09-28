//! License-related API handlers

use actix_web::{HttpResponse, web};
use crate::types::{ClaimRequest, LicenseVariant};
use crate::server::app::ServiceFactory;

/// GET /api/v1/licenses/variants
/// Returns available license split options
pub async fn get_variants(
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

    // Get available split options
    match factory.license_service.get_available_splits().await {
        Ok(splits) => {
            // Convert to LicenseVariant for UI compatibility
            let variants: Vec<LicenseVariant> = splits
                .into_iter()
                .map(|sa| LicenseVariant::from_split_with_counts(sa.split_type, sa.available, sa.claimed, sa.total))
                .collect();
            HttpResponse::Ok().json(variants)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string(),
            "code": "INTERNAL_ERROR"
        })),
    }
}

/// POST /api/v1/licenses/claim
pub async fn claim_license(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<ClaimRequest>,
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

    match factory.license_service.claim_license(body.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}
