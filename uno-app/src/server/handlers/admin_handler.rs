//! Admin API handlers with HMAC authentication
//!
//! These handlers use uno-api's signed request authentication
//! for secure communication with admin clients.

use actix_web::{HttpResponse, web};
use chrono::{DateTime, Utc};

use uno_api::auth::{verify_request_with_replay_protection_async, SignedRequest};
use uno_api::models::{PaginationParams, CsvImportRequest, RevokeRequest, PublishLicensesRequest};
use uno_api::models::marketplace::{
    GetClaimedLicensesRequest, ClaimedLicenseDto, ClaimedLicensesResponse,
    ReferralDto, ReferralsResponse, SyncReferralsRequest, SyncResult,
    GetVisitorStatsRequest, CountryVisitorStats, VisitorStatsResponse,
};

use crate::server::app::ServiceFactory;
use crate::server::repositories::ReferralInput as RepoReferralInput;

/// Maximum request age in seconds (5 minutes)
const MAX_REQUEST_AGE_SECS: i64 = 300;

/// POST /api/v1/admin/licenses
/// Publish a batch of licenses
pub async fn publish_licenses(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<PublishLicensesRequest>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    // Process the request
    match factory.license_admin_service.publish_licenses(body.payload.clone()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            tracing::error!("Failed to publish licenses: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/licenses/import
/// Import licenses from CSV
pub async fn import_csv(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<CsvImportRequest>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    // Process the import
    match factory.license_admin_service.import_csv(body.payload.clone()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            tracing::error!("Failed to import CSV: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "IMPORT_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/licenses/search
/// Search licenses with filters
pub async fn search_licenses(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<SearchRequest>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let SearchRequest { filters, pagination } = body.payload.clone();

    match factory.license_admin_service.search_licenses(filters, pagination).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            tracing::error!("Failed to search licenses: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// DELETE /api/v1/admin/licenses
/// Revoke licenses
pub async fn revoke_licenses(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<RevokeRequest>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    match factory.license_admin_service.revoke_licenses(body.payload.clone()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            tracing::error!("Failed to revoke licenses: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/summary
/// Get license summary statistics
pub async fn get_summary(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<serde_json::Value>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    match factory.license_admin_service.get_summary().await {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(e) => {
            tracing::error!("Failed to get license summary: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/admin/health
/// Health check endpoint (no auth required)
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}

/// POST /api/v1/admin/licenses/claimed
/// Get claimed licenses for uno-admin sync
pub async fn get_claimed_licenses(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<GetClaimedLicensesRequest>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    // Parse since timestamp if provided
    let since: Option<DateTime<Utc>> = body.payload.since.as_ref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let limit = body.payload.limit.unwrap_or(100);

    match factory.claim_repository.get_claimed_since(since, limit).await {
        Ok(claimed) => {
            // R3-06: Cursor-based pagination with compound key (timestamp:id)
            let has_more = claimed.len() as i32 >= limit;
            let next_cursor = if has_more {
                claimed.last().and_then(|c| {
                    c.claimed_at.map(|dt| {
                        format!("{}:{}", dt.timestamp_millis(), c.license_id)
                    })
                })
            } else {
                None
            };

            let licenses: Vec<ClaimedLicenseDto> = claimed
                .into_iter()
                .map(|c| ClaimedLicenseDto {
                    license_id: c.license_id,
                    claimed_at: c.claimed_at
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default(),
                    referral_code: c.referral_code,
                    claim_token: c.claim_token,
                })
                .collect();

            let total = licenses.len() as i64;

            HttpResponse::Ok().json(ClaimedLicensesResponse {
                licenses,
                total,
                next_cursor,
                has_more,
            })
        }
        Err(e) => {
            tracing::error!("Failed to get claimed licenses: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/referrals
/// Get all referrals for sync
pub async fn get_referrals(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<serde_json::Value>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    match factory.referral_repository.list_all(1000).await {
        Ok(referrals) => {
            let dtos: Vec<ReferralDto> = referrals
                .into_iter()
                .map(|r| ReferralDto {
                    id: r.id,
                    username: r.username,
                    email: r.email,
                    country_code: r.country_code,
                    referral_code: r.referral_code,
                    status: r.status.to_string(),
                    created_at: r.created_at.to_rfc3339(),
                })
                .collect();

            let total = dtos.len() as i64;

            HttpResponse::Ok().json(ReferralsResponse {
                referrals: dtos,
                total,
            })
        }
        Err(e) => {
            tracing::error!("Failed to get referrals: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/admin/referrals/sync
/// Receive referrals from uno-admin
pub async fn sync_referrals(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<SyncReferralsRequest>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    let mut created = 0;
    let mut updated = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    for input in &body.payload.referrals {
        // Check if referral exists
        let exists = factory.referral_repository
            .get_by_code(&input.referral_code)
            .await
            .unwrap_or(None)
            .is_some();

        let repo_input = RepoReferralInput {
            username: input.username.clone(),
            email: input.email.clone(),
            country_code: input.country_code.clone(),
            referral_code: input.referral_code.clone(),
            status: input.status.clone(),
        };

        match factory.referral_repository.upsert(repo_input).await {
            Ok(_) => {
                if exists {
                    updated += 1;
                } else {
                    created += 1;
                }
            }
            Err(e) => {
                failed += 1;
                errors.push(format!("Failed to sync {}: {}", input.referral_code, e));
            }
        }
    }

    HttpResponse::Ok().json(SyncResult {
        created,
        updated,
        failed,
        errors,
    })
}

/// Search request combining filters and pagination
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchRequest {
    pub filters: uno_api::traits::LicenseFilters,
    pub pagination: PaginationParams,
}

/// POST /api/v1/admin/stats/visitors
/// Get visitor stats by country for uno-admin sync
pub async fn get_visitor_stats(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<SignedRequest<GetVisitorStatsRequest>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "code": "SERVICE_UNAVAILABLE"
            }));
        }
    };

    // Verify HMAC signature with replay protection
    if let Err(e) = verify_request_with_replay_protection_async(
        &body,
        &factory.client_registry,
        factory.nonce_repository.as_ref(),
        MAX_REQUEST_AGE_SECS,
    ).await {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string(),
            "code": "UNAUTHORIZED"
        }));
    }

    // Parse period from request (default to monthly)
    let period_str = body.payload.period.as_deref().unwrap_or("monthly");
    let (period_days, period_label) = match period_str {
        "daily" => (1, "Today"),
        "weekly" => (7, "This Week"),
        "monthly" => (30, "This Month"),
        "all" => (3650, "All Time"),
        _ => (30, "This Month"),
    };

    let limit = body.payload.limit.unwrap_or(20);

    // Get stats from repository
    let total_visitors = match factory.stats_repository.get_total_visits(period_days).await {
        Ok(count) => count,
        Err(e) => {
            tracing::error!("Failed to get total visitors: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }));
        }
    };

    let unique_visitors = match factory.stats_repository.get_unique_visitor_count(period_days).await {
        Ok(count) => count,
        Err(e) => {
            tracing::error!("Failed to get unique visitors: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }));
        }
    };

    let country_stats = match factory.stats_repository.get_visitors_by_country(period_days, limit).await {
        Ok(stats) => stats,
        Err(e) => {
            tracing::error!("Failed to get visitors by country: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string(),
                "code": "INTERNAL_ERROR"
            }));
        }
    };

    // Convert to response DTOs
    let stats: Vec<CountryVisitorStats> = country_stats
        .into_iter()
        .map(|s| CountryVisitorStats {
            country_code: s.country_code,
            visitor_count: s.visitor_count,
            unique_visitors: s.visitor_count, // Same as visitor_count (COUNT DISTINCT)
        })
        .collect();

    HttpResponse::Ok().json(VisitorStatsResponse {
        stats,
        total_visitors,
        unique_visitors,
        period: period_label.to_string(),
    })
}

/// Configure admin API routes
pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin")
            .route("/licenses", web::post().to(publish_licenses))
            .route("/licenses/import", web::post().to(import_csv))
            .route("/licenses/search", web::post().to(search_licenses))
            .route("/licenses/claimed", web::post().to(get_claimed_licenses))
            .route("/licenses", web::delete().to(revoke_licenses))
            .route("/summary", web::post().to(get_summary))
            .route("/referrals", web::post().to(get_referrals))
            .route("/referrals/sync", web::post().to(sync_referrals))
            .route("/stats/visitors", web::post().to(get_visitor_stats))
            .route("/health", web::get().to(health))
    );
}
