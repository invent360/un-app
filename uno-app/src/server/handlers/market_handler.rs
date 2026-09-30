//! Market management handlers for Phase 7
//!
//! Endpoints for:
//! - Market status and readiness
//! - Quota management
//! - Campaign tracking

use actix_web::{web, HttpResponse};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::repositories::{
    DynMarketRepository, UpsertMarketStatusInput, CreateQuotaInput,
    CreateCampaignInput, RecordAttributionInput,
};
use crate::types::AppError;

// ============================================
// REQUEST/RESPONSE TYPES
// ============================================

#[derive(Debug, Deserialize)]
pub struct UpsertMarketRequest {
    pub country_code: String,
    pub is_open: bool,
    pub requires_reviewed_content: Option<bool>,
    pub requires_local_support: Option<bool>,
    pub requires_local_language: Option<bool>,
    pub primary_locale: Option<String>,
    pub timezone: Option<String>,
    pub currency_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PauseMarketRequest {
    pub reason: String,
    pub paused_by: String,
}

#[derive(Debug, Deserialize)]
pub struct ResumeMarketRequest {
    pub resumed_by: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReviewRequest {
    pub status: String,
    pub reviewed_by: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuotaRequest {
    pub country_code: String,
    pub quota_type: String,
    pub max_value: i32,
    pub warning_threshold: Option<i32>,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct ConsumeQuotaRequest {
    pub country_code: String,
    pub quota_type: String,
    pub user_id: String,
    pub license_id: Option<String>,
    pub action: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuotaMaxRequest {
    pub max_value: i32,
}

#[derive(Debug, Deserialize)]
pub struct ResetDailyQuotasRequest {
    pub period_date: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub campaign_code: String,
    pub campaign_name: String,
    pub description: Option<String>,
    pub source_type: String,
    pub country_code: Option<String>,
    pub agent_id: Option<Uuid>,
    pub partner_name: Option<String>,
    pub qr_code_url: Option<String>,
    pub short_url: Option<String>,
    pub full_url: Option<String>,
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_campaign: Option<String>,
    pub utm_content: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecordAttributionRequest {
    pub campaign_code: String,
    pub user_id: Option<String>,
    pub license_id: Option<String>,
    pub attribution_type: String,
    pub referrer_url: Option<String>,
    pub landing_url: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IncrementCounterRequest {
    pub counter: String, // "click", "registration", "claim"
}

#[derive(Debug, Deserialize)]
pub struct AttributionQuery {
    pub attribution_type: Option<String>,
}

// ============================================
// PUBLIC ENDPOINTS
// ============================================

/// GET /api/v1/markets/{country_code}
/// Get market status (public - for eligibility check)
pub async fn get_market_status(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let country_code = path.into_inner();
    let status = repo.get_market_status(&country_code).await?
        .ok_or_else(|| AppError::NotFound(format!("Market {} not found", country_code)))?;
    Ok(HttpResponse::Ok().json(status))
}

/// GET /api/v1/markets/{country_code}/readiness
/// Check market readiness (public)
pub async fn check_market_readiness(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let country_code = path.into_inner();
    let readiness = repo.check_market_readiness(&country_code).await?;
    Ok(HttpResponse::Ok().json(readiness))
}

/// GET /api/v1/markets/open
/// Get all open markets (public)
pub async fn get_open_markets(
    repo: web::Data<DynMarketRepository>,
) -> Result<HttpResponse, AppError> {
    let markets = repo.get_open_markets().await?;
    Ok(HttpResponse::Ok().json(markets))
}

/// POST /api/v1/markets/attribution
/// Record campaign attribution (public - called on click/registration)
pub async fn record_attribution(
    repo: web::Data<DynMarketRepository>,
    body: web::Json<RecordAttributionRequest>,
) -> Result<HttpResponse, AppError> {
    // Get campaign by code
    let campaign = repo.get_campaign_by_code(&body.campaign_code).await?
        .ok_or_else(|| AppError::NotFound(format!("Campaign {} not found", body.campaign_code)))?;

    let input = RecordAttributionInput {
        campaign_id: campaign.id,
        user_id: body.user_id.clone(),
        license_id: body.license_id.clone(),
        attribution_type: body.attribution_type.clone(),
        referrer_url: body.referrer_url.clone(),
        landing_url: body.landing_url.clone(),
        user_agent: body.user_agent.clone(),
        ip_address: body.ip_address.clone(),
        country_code: body.country_code.clone(),
    };

    let attribution = repo.record_attribution(input).await?;

    // Increment campaign counter
    let counter = match body.attribution_type.as_str() {
        "click" => "click",
        "registration" => "registration",
        "claim" => "claim",
        _ => "click",
    };
    let _ = repo.increment_campaign_counter(campaign.id, counter).await;

    Ok(HttpResponse::Created().json(attribution))
}

// ============================================
// ADMIN ENDPOINTS - Market Status
// ============================================

/// GET /api/v1/admin/markets
/// Get all market statuses
pub async fn get_all_markets(
    repo: web::Data<DynMarketRepository>,
) -> Result<HttpResponse, AppError> {
    let markets = repo.get_all_market_statuses().await?;
    Ok(HttpResponse::Ok().json(markets))
}

/// POST /api/v1/admin/markets
/// Create or update market status
pub async fn upsert_market(
    repo: web::Data<DynMarketRepository>,
    body: web::Json<UpsertMarketRequest>,
) -> Result<HttpResponse, AppError> {
    let input = UpsertMarketStatusInput {
        country_code: body.country_code.clone(),
        is_open: body.is_open,
        requires_reviewed_content: body.requires_reviewed_content.unwrap_or(false),
        requires_local_support: body.requires_local_support.unwrap_or(false),
        requires_local_language: body.requires_local_language.unwrap_or(false),
        primary_locale: body.primary_locale.clone(),
        timezone: body.timezone.clone(),
        currency_code: body.currency_code.clone(),
    };

    let market = repo.upsert_market_status(input).await?;
    Ok(HttpResponse::Ok().json(market))
}

/// POST /api/v1/admin/markets/{country_code}/pause
/// Pause a market
pub async fn pause_market(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<String>,
    body: web::Json<PauseMarketRequest>,
) -> Result<HttpResponse, AppError> {
    let country_code = path.into_inner();
    let market = repo.pause_market(&country_code, &body.reason, &body.paused_by).await?;
    Ok(HttpResponse::Ok().json(market))
}

/// POST /api/v1/admin/markets/{country_code}/resume
/// Resume a market
pub async fn resume_market(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<String>,
    body: web::Json<ResumeMarketRequest>,
) -> Result<HttpResponse, AppError> {
    let country_code = path.into_inner();
    let market = repo.resume_market(&country_code, &body.resumed_by).await?;
    Ok(HttpResponse::Ok().json(market))
}

/// POST /api/v1/admin/markets/{country_code}/content-review
/// Update content review status
pub async fn update_content_review(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<String>,
    body: web::Json<UpdateReviewRequest>,
) -> Result<HttpResponse, AppError> {
    let country_code = path.into_inner();
    let market = repo.update_content_review(&country_code, &body.status, &body.reviewed_by).await?;
    Ok(HttpResponse::Ok().json(market))
}

/// POST /api/v1/admin/markets/{country_code}/support-readiness
/// Update support readiness status
pub async fn update_support_readiness(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<String>,
    body: web::Json<UpdateReviewRequest>,
) -> Result<HttpResponse, AppError> {
    let country_code = path.into_inner();
    let market = repo.update_support_readiness(&country_code, &body.status, &body.reviewed_by).await?;
    Ok(HttpResponse::Ok().json(market))
}

/// POST /api/v1/admin/markets/{country_code}/language-review
/// Update language review status
pub async fn update_language_review(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<String>,
    body: web::Json<UpdateReviewRequest>,
) -> Result<HttpResponse, AppError> {
    let country_code = path.into_inner();
    let market = repo.update_language_review(&country_code, &body.status, &body.reviewed_by).await?;
    Ok(HttpResponse::Ok().json(market))
}

// ============================================
// ADMIN ENDPOINTS - Quotas
// ============================================

/// POST /api/v1/admin/quotas
/// Create a quota
pub async fn create_quota(
    repo: web::Data<DynMarketRepository>,
    body: web::Json<CreateQuotaRequest>,
) -> Result<HttpResponse, AppError> {
    let input = CreateQuotaInput {
        country_code: body.country_code.clone(),
        quota_type: body.quota_type.clone(),
        max_value: body.max_value,
        warning_threshold: body.warning_threshold,
        period_start: body.period_start,
        period_end: body.period_end,
    };

    let quota = repo.create_quota(input).await?;
    Ok(HttpResponse::Created().json(quota))
}

/// GET /api/v1/admin/quotas/{id}
/// Get quota by ID
pub async fn get_quota(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let quota = repo.get_quota(id).await?
        .ok_or_else(|| AppError::NotFound(format!("Quota {} not found", id)))?;
    Ok(HttpResponse::Ok().json(quota))
}

/// GET /api/v1/admin/quotas/country/{country_code}
/// Get quotas for a country
pub async fn get_country_quotas(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let country_code = path.into_inner();
    let quotas = repo.get_country_quotas(&country_code).await?;
    Ok(HttpResponse::Ok().json(quotas))
}

/// GET /api/v1/admin/quotas/check/{country_code}/{quota_type}
/// Check quota status
pub async fn check_quota(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let (country_code, quota_type) = path.into_inner();
    let status = repo.check_quota(&country_code, &quota_type).await?;
    Ok(HttpResponse::Ok().json(status))
}

/// POST /api/v1/admin/quotas/consume
/// Consume quota (atomic)
pub async fn consume_quota(
    repo: web::Data<DynMarketRepository>,
    body: web::Json<ConsumeQuotaRequest>,
) -> Result<HttpResponse, AppError> {
    let consumed = repo.consume_quota(
        &body.country_code,
        &body.quota_type,
        &body.user_id,
        body.license_id.as_deref(),
        &body.action,
    ).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "consumed": consumed
    })))
}

/// POST /api/v1/admin/quotas/{id}/reset
/// Reset quota value
pub async fn reset_quota(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let quota = repo.reset_quota(id).await?;
    Ok(HttpResponse::Ok().json(quota))
}

/// PUT /api/v1/admin/quotas/{id}/max
/// Update quota max value
pub async fn update_quota_max(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateQuotaMaxRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let quota = repo.update_quota_max(id, body.max_value).await?;
    Ok(HttpResponse::Ok().json(quota))
}

/// POST /api/v1/admin/quotas/{id}/enable
/// Enable quota
pub async fn enable_quota(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let quota = repo.set_quota_enabled(id, true).await?;
    Ok(HttpResponse::Ok().json(quota))
}

/// POST /api/v1/admin/quotas/{id}/disable
/// Disable quota
pub async fn disable_quota(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let quota = repo.set_quota_enabled(id, false).await?;
    Ok(HttpResponse::Ok().json(quota))
}

/// POST /api/v1/admin/quotas/reset-daily
/// Reset all daily quotas (for scheduled job)
pub async fn reset_daily_quotas(
    repo: web::Data<DynMarketRepository>,
    body: web::Json<ResetDailyQuotasRequest>,
) -> Result<HttpResponse, AppError> {
    let count = repo.reset_daily_quotas(body.period_date).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "reset_count": count
    })))
}

// ============================================
// ADMIN ENDPOINTS - Campaigns
// ============================================

/// POST /api/v1/admin/campaigns
/// Create a campaign
pub async fn create_campaign(
    repo: web::Data<DynMarketRepository>,
    body: web::Json<CreateCampaignRequest>,
) -> Result<HttpResponse, AppError> {
    let input = CreateCampaignInput {
        campaign_code: body.campaign_code.clone(),
        campaign_name: body.campaign_name.clone(),
        description: body.description.clone(),
        source_type: body.source_type.clone(),
        country_code: body.country_code.clone(),
        agent_id: body.agent_id,
        partner_name: body.partner_name.clone(),
        qr_code_url: body.qr_code_url.clone(),
        short_url: body.short_url.clone(),
        full_url: body.full_url.clone(),
        utm_source: body.utm_source.clone(),
        utm_medium: body.utm_medium.clone(),
        utm_campaign: body.utm_campaign.clone(),
        utm_content: body.utm_content.clone(),
        starts_at: body.starts_at,
        ends_at: body.ends_at,
        created_by: body.created_by.clone(),
    };

    let campaign = repo.create_campaign(input).await?;
    Ok(HttpResponse::Created().json(campaign))
}

/// GET /api/v1/admin/campaigns/{id}
/// Get campaign by ID
pub async fn get_campaign(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let campaign = repo.get_campaign(id).await?
        .ok_or_else(|| AppError::NotFound(format!("Campaign {} not found", id)))?;
    Ok(HttpResponse::Ok().json(campaign))
}

/// GET /api/v1/admin/campaigns/code/{code}
/// Get campaign by code
pub async fn get_campaign_by_code(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let code = path.into_inner();
    let campaign = repo.get_campaign_by_code(&code).await?
        .ok_or_else(|| AppError::NotFound(format!("Campaign {} not found", code)))?;
    Ok(HttpResponse::Ok().json(campaign))
}

/// GET /api/v1/admin/campaigns/country/{country_code}
/// Get campaigns for country
pub async fn get_country_campaigns(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let country_code = path.into_inner();
    let campaigns = repo.get_country_campaigns(&country_code).await?;
    Ok(HttpResponse::Ok().json(campaigns))
}

/// GET /api/v1/admin/campaigns/agent/{agent_id}
/// Get campaigns for agent
pub async fn get_agent_campaigns(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let agent_id = path.into_inner();
    let campaigns = repo.get_agent_campaigns(agent_id).await?;
    Ok(HttpResponse::Ok().json(campaigns))
}

/// GET /api/v1/admin/campaigns/active
/// Get active campaigns
pub async fn get_active_campaigns(
    repo: web::Data<DynMarketRepository>,
) -> Result<HttpResponse, AppError> {
    let campaigns = repo.get_active_campaigns().await?;
    Ok(HttpResponse::Ok().json(campaigns))
}

/// POST /api/v1/admin/campaigns/{id}/activate
/// Activate campaign
pub async fn activate_campaign(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let campaign = repo.set_campaign_active(id, true).await?;
    Ok(HttpResponse::Ok().json(campaign))
}

/// POST /api/v1/admin/campaigns/{id}/deactivate
/// Deactivate campaign
pub async fn deactivate_campaign(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let campaign = repo.set_campaign_active(id, false).await?;
    Ok(HttpResponse::Ok().json(campaign))
}

/// POST /api/v1/admin/campaigns/{id}/increment
/// Increment campaign counter
pub async fn increment_campaign_counter(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<Uuid>,
    body: web::Json<IncrementCounterRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let campaign = repo.increment_campaign_counter(id, &body.counter).await?;
    Ok(HttpResponse::Ok().json(campaign))
}

/// GET /api/v1/admin/campaigns/{id}/attributions
/// Get attributions for campaign
pub async fn get_campaign_attributions(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<Uuid>,
    query: web::Query<AttributionQuery>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let attributions = repo.get_campaign_attributions(
        id,
        query.attribution_type.as_deref(),
    ).await?;
    Ok(HttpResponse::Ok().json(attributions))
}

/// GET /api/v1/admin/attributions/user/{user_id}
/// Get attributions for user
pub async fn get_user_attributions(
    repo: web::Data<DynMarketRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let attributions = repo.get_user_attributions(&user_id).await?;
    Ok(HttpResponse::Ok().json(attributions))
}

// ============================================
// ROUTE CONFIGURATION
// ============================================

/// Configure public market routes
pub fn configure_public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/markets")
            .route("/open", web::get().to(get_open_markets))
            .route("/{country_code}", web::get().to(get_market_status))
            .route("/{country_code}/readiness", web::get().to(check_market_readiness))
            .route("/attribution", web::post().to(record_attribution)),
    );
}

/// Configure admin market routes
pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/markets")
            .route("", web::get().to(get_all_markets))
            .route("", web::post().to(upsert_market))
            .route("/{country_code}/pause", web::post().to(pause_market))
            .route("/{country_code}/resume", web::post().to(resume_market))
            .route("/{country_code}/content-review", web::post().to(update_content_review))
            .route("/{country_code}/support-readiness", web::post().to(update_support_readiness))
            .route("/{country_code}/language-review", web::post().to(update_language_review)),
    );

    cfg.service(
        web::scope("/quotas")
            .route("", web::post().to(create_quota))
            .route("/consume", web::post().to(consume_quota))
            .route("/reset-daily", web::post().to(reset_daily_quotas))
            .route("/country/{country_code}", web::get().to(get_country_quotas))
            .route("/check/{country_code}/{quota_type}", web::get().to(check_quota))
            .route("/{id}", web::get().to(get_quota))
            .route("/{id}/reset", web::post().to(reset_quota))
            .route("/{id}/max", web::put().to(update_quota_max))
            .route("/{id}/enable", web::post().to(enable_quota))
            .route("/{id}/disable", web::post().to(disable_quota)),
    );

    cfg.service(
        web::scope("/campaigns")
            .route("", web::post().to(create_campaign))
            .route("/active", web::get().to(get_active_campaigns))
            .route("/code/{code}", web::get().to(get_campaign_by_code))
            .route("/country/{country_code}", web::get().to(get_country_campaigns))
            .route("/agent/{agent_id}", web::get().to(get_agent_campaigns))
            .route("/{id}", web::get().to(get_campaign))
            .route("/{id}/activate", web::post().to(activate_campaign))
            .route("/{id}/deactivate", web::post().to(deactivate_campaign))
            .route("/{id}/increment", web::post().to(increment_campaign_counter))
            .route("/{id}/attributions", web::get().to(get_campaign_attributions)),
    );

    cfg.service(
        web::scope("/attributions")
            .route("/user/{user_id}", web::get().to(get_user_attributions)),
    );
}
