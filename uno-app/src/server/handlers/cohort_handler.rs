//! Cohort tracking handlers for Phase 7
//!
//! Endpoints for:
//! - D1/D3/D7/D30 progress tracking
//! - Cohort analytics
//! - Notification management

use actix_web::{web, HttpResponse};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::repositories::{
    DynCohortRepository, CreateCohortInput, RecordActivityInput,
    ScheduleNotificationInput,
};
use crate::types::AppError;

// ============================================
// REQUEST/RESPONSE TYPES
// ============================================

#[derive(Debug, Deserialize)]
pub struct CreateCohortRequest {
    pub user_id: String,
    pub license_id: String,
    pub cohort_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct RecordActivityRequest {
    pub activity_date: Option<NaiveDate>,
    pub sessions_count: Option<i32>,
    pub tasks_completed: Option<i32>,
    pub earnings_micros: Option<i64>,
    pub data_collected_bytes: Option<i64>,
    pub device_type: Option<String>,
    pub app_version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleNotificationRequest {
    pub notification_type: String,
    pub scheduled_at: chrono::DateTime<Utc>,
    pub channel: String,
}

#[derive(Debug, Deserialize)]
pub struct DateRangeQuery {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub cohort_date: NaiveDate,
    pub country_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CohortProgressResponse {
    pub cohort: serde_json::Value,
    pub d7_progress: serde_json::Value,
    pub d30_progress: serde_json::Value,
}

// ============================================
// PUBLIC ENDPOINTS (User-facing)
// ============================================

/// POST /api/v1/cohort
/// Create cohort entry for a participant (called on license claim)
pub async fn create_cohort(
    repo: web::Data<DynCohortRepository>,
    body: web::Json<CreateCohortRequest>,
) -> Result<HttpResponse, AppError> {
    let cohort_date = body.cohort_date.unwrap_or_else(|| Utc::now().date_naive());

    let input = CreateCohortInput {
        user_id: body.user_id.clone(),
        license_id: body.license_id.clone(),
        cohort_date,
    };

    let cohort = repo.create_cohort(input).await?;
    Ok(HttpResponse::Created().json(cohort))
}

/// GET /api/v1/cohort/{id}
/// Get cohort by ID
pub async fn get_cohort(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let cohort = repo.get_cohort(id).await?
        .ok_or_else(|| AppError::NotFound(format!("Cohort {} not found", id)))?;
    Ok(HttpResponse::Ok().json(cohort))
}

/// GET /api/v1/cohort/user/{user_id}/license/{license_id}
/// Get cohort by user and license
pub async fn get_cohort_by_user_license(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, license_id) = path.into_inner();
    let cohort = repo.get_cohort_by_user_license(&user_id, &license_id).await?
        .ok_or_else(|| AppError::NotFound("Cohort not found".to_string()))?;
    Ok(HttpResponse::Ok().json(cohort))
}

/// GET /api/v1/cohort/user/{user_id}
/// Get all cohorts for a user
pub async fn get_user_cohorts(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let cohorts = repo.get_user_cohorts(&user_id).await?;
    Ok(HttpResponse::Ok().json(cohorts))
}

/// GET /api/v1/cohort/{id}/progress
/// Get cohort progress (D7/D30)
pub async fn get_cohort_progress(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    let cohort = repo.get_cohort(id).await?
        .ok_or_else(|| AppError::NotFound(format!("Cohort {} not found", id)))?;

    let d7_progress = repo.calculate_d7_progress(id).await?;
    let d30_progress = repo.calculate_d30_progress(id).await?;

    Ok(HttpResponse::Ok().json(CohortProgressResponse {
        cohort: serde_json::to_value(&cohort).unwrap_or_default(),
        d7_progress: serde_json::to_value(&d7_progress).unwrap_or_default(),
        d30_progress: serde_json::to_value(&d30_progress).unwrap_or_default(),
    }))
}

/// POST /api/v1/cohort/{id}/activity
/// Record daily activity
pub async fn record_activity(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<Uuid>,
    body: web::Json<RecordActivityRequest>,
) -> Result<HttpResponse, AppError> {
    let cohort_id = path.into_inner();

    let input = RecordActivityInput {
        cohort_id,
        activity_date: body.activity_date.unwrap_or_else(|| Utc::now().date_naive()),
        sessions_count: body.sessions_count.unwrap_or(1),
        tasks_completed: body.tasks_completed.unwrap_or(0),
        earnings_micros: body.earnings_micros.unwrap_or(0),
        data_collected_bytes: body.data_collected_bytes.unwrap_or(0),
        device_type: body.device_type.clone(),
        app_version: body.app_version.clone(),
    };

    let activity = repo.record_activity(input).await?;
    Ok(HttpResponse::Created().json(activity))
}

/// GET /api/v1/cohort/{id}/activities
/// Get all activities for a cohort
pub async fn get_cohort_activities(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let cohort_id = path.into_inner();
    let activities = repo.get_cohort_activities(cohort_id).await?;
    Ok(HttpResponse::Ok().json(activities))
}

/// POST /api/v1/cohort/{id}/d1
/// Mark D1 completed
pub async fn complete_d1(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let cohort_id = path.into_inner();
    let cohort = repo.complete_d1(cohort_id).await?;
    Ok(HttpResponse::Ok().json(cohort))
}

/// POST /api/v1/cohort/{id}/d3
/// Mark D3 completed
#[derive(Debug, Deserialize)]
pub struct CompleteD3Request {
    pub activity_count: i32,
}

pub async fn complete_d3(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<Uuid>,
    body: web::Json<CompleteD3Request>,
) -> Result<HttpResponse, AppError> {
    let cohort_id = path.into_inner();
    let cohort = repo.complete_d3(cohort_id, body.activity_count).await?;
    Ok(HttpResponse::Ok().json(cohort))
}

// ============================================
// ADMIN ENDPOINTS
// ============================================

/// GET /api/v1/admin/cohorts/date-range
/// Get cohorts by date range
pub async fn get_cohorts_by_date_range(
    repo: web::Data<DynCohortRepository>,
    query: web::Query<DateRangeQuery>,
) -> Result<HttpResponse, AppError> {
    let cohorts = repo.get_cohorts_by_date_range(
        query.start_date,
        query.end_date,
    ).await?;
    Ok(HttpResponse::Ok().json(cohorts))
}

/// GET /api/v1/admin/cohorts/stats
/// Get cohort statistics for a date
pub async fn get_cohort_stats(
    repo: web::Data<DynCohortRepository>,
    query: web::Query<AnalyticsQuery>,
) -> Result<HttpResponse, AppError> {
    let stats = repo.get_cohort_stats(query.cohort_date).await?;
    Ok(HttpResponse::Ok().json(stats))
}

/// POST /api/v1/admin/cohorts/analytics
/// Calculate and store cohort analytics
pub async fn calculate_analytics(
    repo: web::Data<DynCohortRepository>,
    body: web::Json<AnalyticsQuery>,
) -> Result<HttpResponse, AppError> {
    let analytics = repo.calculate_analytics(
        body.cohort_date,
        body.country_code.as_deref(),
    ).await?;
    Ok(HttpResponse::Ok().json(analytics))
}

/// GET /api/v1/admin/cohorts/analytics
/// Get stored analytics
pub async fn get_analytics(
    repo: web::Data<DynCohortRepository>,
    query: web::Query<AnalyticsQuery>,
) -> Result<HttpResponse, AppError> {
    let analytics = repo.get_analytics(
        query.cohort_date,
        query.country_code.as_deref(),
    ).await?;

    match analytics {
        Some(a) => Ok(HttpResponse::Ok().json(a)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Analytics not found for this date",
            "code": "NOT_FOUND"
        }))),
    }
}

/// GET /api/v1/admin/cohorts/pending/d1
/// Get cohorts pending D1 check
pub async fn get_cohorts_pending_d1(
    repo: web::Data<DynCohortRepository>,
    query: web::Query<LimitQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(100);
    let cohorts = repo.get_cohorts_pending_d1(limit).await?;
    Ok(HttpResponse::Ok().json(cohorts))
}

/// GET /api/v1/admin/cohorts/pending/d7
/// Get cohorts pending D7 check
pub async fn get_cohorts_pending_d7(
    repo: web::Data<DynCohortRepository>,
    query: web::Query<LimitQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(100);
    let cohorts = repo.get_cohorts_pending_d7(limit).await?;
    Ok(HttpResponse::Ok().json(cohorts))
}

/// GET /api/v1/admin/cohorts/pending/d30
/// Get cohorts pending D30 check
pub async fn get_cohorts_pending_d30(
    repo: web::Data<DynCohortRepository>,
    query: web::Query<LimitQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(100);
    let cohorts = repo.get_cohorts_pending_d30(limit).await?;
    Ok(HttpResponse::Ok().json(cohorts))
}

#[derive(Debug, Deserialize)]
pub struct LimitQuery {
    pub limit: Option<i32>,
}

// ============================================
// NOTIFICATION ENDPOINTS
// ============================================

/// POST /api/v1/admin/cohorts/{id}/notifications
/// Schedule a notification
pub async fn schedule_notification(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<Uuid>,
    body: web::Json<ScheduleNotificationRequest>,
) -> Result<HttpResponse, AppError> {
    let cohort_id = path.into_inner();

    let input = ScheduleNotificationInput {
        cohort_id,
        notification_type: body.notification_type.clone(),
        scheduled_at: body.scheduled_at,
        channel: body.channel.clone(),
    };

    let notification = repo.schedule_notification(input).await?;
    Ok(HttpResponse::Created().json(notification))
}

/// GET /api/v1/admin/cohorts/{id}/notifications
/// Get notifications for a cohort
pub async fn get_cohort_notifications(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let cohort_id = path.into_inner();
    let notifications = repo.get_cohort_notifications(cohort_id).await?;
    Ok(HttpResponse::Ok().json(notifications))
}

/// GET /api/v1/admin/notifications/pending
/// Get pending notifications (for worker)
pub async fn get_pending_notifications(
    repo: web::Data<DynCohortRepository>,
    query: web::Query<LimitQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(50);
    let notifications = repo.get_pending_notifications(limit).await?;
    Ok(HttpResponse::Ok().json(notifications))
}

/// POST /api/v1/admin/notifications/{id}/sent
/// Mark notification as sent
#[derive(Debug, Deserialize)]
pub struct MarkSentRequest {
    pub external_id: Option<String>,
}

pub async fn mark_notification_sent(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<Uuid>,
    body: web::Json<MarkSentRequest>,
) -> Result<HttpResponse, AppError> {
    let notification_id = path.into_inner();
    let notification = repo.mark_notification_sent(
        notification_id,
        body.external_id.as_deref(),
    ).await?;
    Ok(HttpResponse::Ok().json(notification))
}

/// POST /api/v1/admin/notifications/{id}/skip
/// Skip notification with reason
#[derive(Debug, Deserialize)]
pub struct SkipNotificationRequest {
    pub reason: String,
}

pub async fn skip_notification(
    repo: web::Data<DynCohortRepository>,
    path: web::Path<Uuid>,
    body: web::Json<SkipNotificationRequest>,
) -> Result<HttpResponse, AppError> {
    let notification_id = path.into_inner();
    let notification = repo.skip_notification(notification_id, &body.reason).await?;
    Ok(HttpResponse::Ok().json(notification))
}

// ============================================
// ROUTE CONFIGURATION
// ============================================

/// Configure public cohort routes
pub fn configure_public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cohort")
            .route("", web::post().to(create_cohort))
            .route("/{id}", web::get().to(get_cohort))
            .route("/user/{user_id}", web::get().to(get_user_cohorts))
            .route("/user/{user_id}/license/{license_id}", web::get().to(get_cohort_by_user_license))
            .route("/{id}/progress", web::get().to(get_cohort_progress))
            .route("/{id}/activity", web::post().to(record_activity))
            .route("/{id}/activities", web::get().to(get_cohort_activities))
            .route("/{id}/d1", web::post().to(complete_d1))
            .route("/{id}/d3", web::post().to(complete_d3)),
    );
}

/// Configure admin cohort routes
pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cohorts")
            .route("/date-range", web::get().to(get_cohorts_by_date_range))
            .route("/stats", web::get().to(get_cohort_stats))
            .route("/analytics", web::post().to(calculate_analytics))
            .route("/analytics", web::get().to(get_analytics))
            .route("/pending/d1", web::get().to(get_cohorts_pending_d1))
            .route("/pending/d7", web::get().to(get_cohorts_pending_d7))
            .route("/pending/d30", web::get().to(get_cohorts_pending_d30))
            .route("/{id}/notifications", web::post().to(schedule_notification))
            .route("/{id}/notifications", web::get().to(get_cohort_notifications)),
    );

    cfg.service(
        web::scope("/notifications")
            .route("/pending", web::get().to(get_pending_notifications))
            .route("/{id}/sent", web::post().to(mark_notification_sent))
            .route("/{id}/skip", web::post().to(skip_notification)),
    );
}
