//! Participant dashboard handler for Phase 7
//!
//! Aggregates data for the participant home dashboard including:
//! - License status
//! - Cohort progress (D1/D7/D30)
//! - Earnings summary
//! - Tasks summary
//! - Recent notifications

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::server::repositories::{
    DynCohortRepository, DynSupportRepository, DynExitRepository,
};
use crate::types::AppError;

// ============================================
// RESPONSE TYPES
// ============================================

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub user_id: String,
    pub license_id: String,
    pub cohort: Option<CohortSummary>,
    pub support: SupportSummary,
    pub exit_status: Option<ExitSummary>,
}

#[derive(Debug, Serialize)]
pub struct CohortSummary {
    pub cohort_date: String,
    pub d1_completed: bool,
    pub d3_completed: bool,
    pub d7_completed: bool,
    pub d7_active_days: i32,
    pub d7_target_days: i32,
    pub d30_completed: bool,
    pub d30_active_days: i32,
    pub days_since_activation: i64,
}

#[derive(Debug, Serialize)]
pub struct SupportSummary {
    pub open_tickets: i64,
    pub pending_response: i64,
    pub recent_ticket_number: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExitSummary {
    pub has_active_exit: bool,
    pub status: Option<String>,
    pub payout_status: Option<String>,
    pub net_payout_micros: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct DashboardQuery {
    pub user_id: String,
    pub license_id: String,
}

// ============================================
// HANDLERS
// ============================================

/// GET /api/v1/dashboard
/// Get participant dashboard data
pub async fn get_dashboard(
    cohort_repo: web::Data<DynCohortRepository>,
    support_repo: web::Data<DynSupportRepository>,
    exit_repo: web::Data<DynExitRepository>,
    query: web::Query<DashboardQuery>,
) -> Result<HttpResponse, AppError> {
    let user_id = &query.user_id;
    let license_id = &query.license_id;

    // Get cohort data
    let cohort_summary = match cohort_repo.get_cohort_by_user_license(user_id, license_id).await? {
        Some(cohort) => {
            let today = chrono::Utc::now().date_naive();
            let days_since = (today - cohort.cohort_date).num_days();

            Some(CohortSummary {
                cohort_date: cohort.cohort_date.to_string(),
                d1_completed: cohort.d1_completed,
                d3_completed: cohort.d3_completed,
                d7_completed: cohort.d7_completed,
                d7_active_days: cohort.d7_active_days,
                d7_target_days: 4,
                d30_completed: cohort.d30_completed,
                d30_active_days: cohort.d30_active_days,
                days_since_activation: days_since,
            })
        }
        None => None,
    };

    // Get support ticket summary
    let filters = crate::server::repositories::TicketFilters {
        user_id: Some(user_id.clone()),
        status: Some(crate::server::repositories::TicketStatus::Open),
        ..Default::default()
    };
    let open_count = support_repo.count_tickets(&filters).await.unwrap_or(0);

    let in_progress_filters = crate::server::repositories::TicketFilters {
        user_id: Some(user_id.clone()),
        status: Some(crate::server::repositories::TicketStatus::WaitingUser),
        ..Default::default()
    };
    let pending_count = support_repo.count_tickets(&in_progress_filters).await.unwrap_or(0);

    // Get most recent ticket
    let all_filters = crate::server::repositories::TicketFilters {
        user_id: Some(user_id.clone()),
        ..Default::default()
    };
    let recent_tickets = support_repo.search_tickets(&all_filters, 1, 0).await.unwrap_or_default();
    let recent_ticket_number = recent_tickets.first().map(|t| t.ticket_number.clone());

    let support_summary = SupportSummary {
        open_tickets: open_count,
        pending_response: pending_count,
        recent_ticket_number,
    };

    // Get exit status
    let exit_summary = match exit_repo.get_exit_by_license(license_id).await? {
        Some(exit) => Some(ExitSummary {
            has_active_exit: true,
            status: Some(exit.status.as_str().to_string()),
            payout_status: Some(exit.payout_status.as_str().to_string()),
            net_payout_micros: Some(exit.net_payout_micros),
        }),
        None => Some(ExitSummary {
            has_active_exit: false,
            status: None,
            payout_status: None,
            net_payout_micros: None,
        }),
    };

    Ok(HttpResponse::Ok().json(DashboardResponse {
        user_id: user_id.clone(),
        license_id: license_id.clone(),
        cohort: cohort_summary,
        support: support_summary,
        exit_status: exit_summary,
    }))
}

/// GET /api/v1/dashboard/progress/{user_id}/{license_id}
/// Get cohort progress details
pub async fn get_progress(
    cohort_repo: web::Data<DynCohortRepository>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, license_id) = path.into_inner();

    let cohort = cohort_repo.get_cohort_by_user_license(&user_id, &license_id).await?
        .ok_or_else(|| AppError::NotFound("Cohort not found".to_string()))?;

    let d7_progress = cohort_repo.calculate_d7_progress(cohort.id).await?;
    let d30_progress = cohort_repo.calculate_d30_progress(cohort.id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "cohort": cohort,
        "d7_progress": d7_progress,
        "d30_progress": d30_progress,
    })))
}

/// GET /api/v1/dashboard/activities/{user_id}/{license_id}
/// Get recent activities
pub async fn get_activities(
    cohort_repo: web::Data<DynCohortRepository>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, license_id) = path.into_inner();

    let cohort = cohort_repo.get_cohort_by_user_license(&user_id, &license_id).await?
        .ok_or_else(|| AppError::NotFound("Cohort not found".to_string()))?;

    let activities = cohort_repo.get_cohort_activities(cohort.id).await?;

    Ok(HttpResponse::Ok().json(activities))
}

// ============================================
// ROUTE CONFIGURATION
// ============================================

/// Configure dashboard routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dashboard")
            .route("", web::get().to(get_dashboard))
            .route("/progress/{user_id}/{license_id}", web::get().to(get_progress))
            .route("/activities/{user_id}/{license_id}", web::get().to(get_activities)),
    );
}
