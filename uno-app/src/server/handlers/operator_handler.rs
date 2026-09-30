//! Operator dashboard handlers (Phase 8)
//!
//! Provides admin cockpit views for monitoring operations, metrics,
//! and exception tracking.

use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::repositories::{
    DynOperatorMetricsRepository, ExceptionSeverity, RecordExceptionInput,
};
use crate::types::AppError;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct DateRangeQuery {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub country_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MetricsQuery {
    pub date: NaiveDate,
    pub country_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExceptionsQuery {
    pub limit: Option<i32>,
    pub severity: Option<String>,
    pub exception_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResolveExceptionRequest {
    pub resolved_by: String,
    pub resolution_notes: String,
}

#[derive(Debug, Deserialize)]
pub struct RecordExceptionRequest {
    pub exception_type: String,
    pub severity: String,
    pub country_code: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub inventory: InventorySummaryResponse,
    pub cohorts: CohortSummaryResponse,
    pub finance: FinancialSummaryResponse,
    pub support: SupportSummaryResponse,
    pub sync: SyncStatusResponse,
    pub recent_exceptions: Vec<ExceptionResponse>,
}

#[derive(Debug, Serialize)]
pub struct InventorySummaryResponse {
    pub total: i64,
    pub published: i64,
    pub reserved: i64,
    pub issued: i64,
    pub expired: i64,
    pub quarantined: i64,
}

#[derive(Debug, Serialize)]
pub struct CohortSummaryResponse {
    pub total_participants: i64,
    pub d1_completed: i64,
    pub d3_completed: i64,
    pub d7_completed: i64,
    pub d30_completed: i64,
    pub d7_rate: f64,
    pub d30_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct FinancialSummaryResponse {
    pub total_rewards_micros: i64,
    pub total_allocated_micros: i64,
    pub total_payable_micros: i64,
    pub total_paid_micros: i64,
    pub participant_share_micros: i64,
    pub referral_share_micros: i64,
    pub uno_share_micros: i64,
}

#[derive(Debug, Serialize)]
pub struct SupportSummaryResponse {
    pub open_tickets: i64,
    pub new_today: i64,
    pub resolved_today: i64,
    pub escalated: i64,
    pub avg_resolution_hours: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct SyncStatusResponse {
    pub last_sync_at: Option<chrono::DateTime<chrono::Utc>>,
    pub sync_lag_seconds: Option<i32>,
    pub pending_outbox: i64,
    pub pending_inbox: i64,
    pub failed_jobs: i64,
    pub dead_letter: i64,
}

#[derive(Debug, Serialize)]
pub struct ExceptionResponse {
    pub id: Uuid,
    pub exception_type: String,
    pub severity: String,
    pub country_code: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub resolved_by: Option<String>,
    pub resolution_notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub id: Uuid,
    pub metric_date: NaiveDate,
    pub country_code: Option<String>,
    pub total_licenses: i32,
    pub published_licenses: i32,
    pub reserved_licenses: i32,
    pub issued_licenses: i32,
    pub expired_licenses: i32,
    pub d1_completions: i32,
    pub d7_completions: i32,
    pub d30_completions: i32,
    pub d7_completion_rate: Option<f64>,
    pub d30_completion_rate: Option<f64>,
    pub total_rewards_micros: i64,
    pub total_allocated_micros: i64,
    pub total_payable_micros: i64,
    pub total_paid_micros: i64,
    pub open_tickets: i32,
    pub resolved_tickets: i32,
    pub avg_resolution_hours: Option<f64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// Handlers
// ============================================================================

/// GET /api/v1/admin/operator/dashboard
/// Main operator dashboard with all summary metrics
pub async fn get_dashboard(
    repo: web::Data<DynOperatorMetricsRepository>,
) -> Result<impl Responder, AppError> {
    // Calculate all summaries in parallel
    let inventory = repo.calculate_inventory_summary(None).await?;
    let today = chrono::Utc::now().date_naive();
    let thirty_days_ago = today - chrono::Duration::days(30);
    let cohorts = repo.calculate_cohort_summary(thirty_days_ago, today).await?;
    let finance = repo.calculate_financial_summary(thirty_days_ago, today).await?;
    let support = repo.calculate_support_summary().await?;
    let sync = repo.calculate_sync_status().await?;
    let exceptions = repo.get_unresolved_exceptions(10).await?;

    let response = DashboardResponse {
        inventory: InventorySummaryResponse {
            total: inventory.total,
            published: inventory.published,
            reserved: inventory.reserved,
            issued: inventory.issued,
            expired: inventory.expired,
            quarantined: inventory.quarantined,
        },
        cohorts: CohortSummaryResponse {
            total_participants: cohorts.total_participants,
            d1_completed: cohorts.d1_completed,
            d3_completed: cohorts.d3_completed,
            d7_completed: cohorts.d7_completed,
            d30_completed: cohorts.d30_completed,
            d7_rate: cohorts.d7_rate,
            d30_rate: cohorts.d30_rate,
        },
        finance: FinancialSummaryResponse {
            total_rewards_micros: finance.total_rewards_micros,
            total_allocated_micros: finance.total_allocated_micros,
            total_payable_micros: finance.total_payable_micros,
            total_paid_micros: finance.total_paid_micros,
            participant_share_micros: finance.participant_share_micros,
            referral_share_micros: finance.referral_share_micros,
            uno_share_micros: finance.uno_share_micros,
        },
        support: SupportSummaryResponse {
            open_tickets: support.open_tickets,
            new_today: support.new_today,
            resolved_today: support.resolved_today,
            escalated: support.escalated,
            avg_resolution_hours: support.avg_resolution_hours,
        },
        sync: SyncStatusResponse {
            last_sync_at: sync.last_sync_at,
            sync_lag_seconds: sync.sync_lag_seconds,
            pending_outbox: sync.pending_outbox,
            pending_inbox: sync.pending_inbox,
            failed_jobs: sync.failed_jobs,
            dead_letter: sync.dead_letter,
        },
        recent_exceptions: exceptions
            .into_iter()
            .map(|e| ExceptionResponse {
                id: e.id,
                exception_type: e.exception_type,
                severity: e.severity.to_string(),
                country_code: e.country_code,
                entity_type: e.entity_type,
                entity_id: e.entity_id,
                message: e.message,
                details: e.details,
                resolved_at: e.resolved_at,
                resolved_by: e.resolved_by,
                resolution_notes: e.resolution_notes,
                created_at: e.created_at,
            })
            .collect(),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/v1/admin/operator/inventory
/// Inventory metrics for a specific country or global
pub async fn get_inventory(
    repo: web::Data<DynOperatorMetricsRepository>,
    query: web::Query<Option<String>>,
) -> Result<impl Responder, AppError> {
    let country_code = query.into_inner();
    let inventory = repo.calculate_inventory_summary(country_code.as_deref()).await?;

    Ok(HttpResponse::Ok().json(InventorySummaryResponse {
        total: inventory.total,
        published: inventory.published,
        reserved: inventory.reserved,
        issued: inventory.issued,
        expired: inventory.expired,
        quarantined: inventory.quarantined,
    }))
}

/// GET /api/v1/admin/operator/cohorts
/// Cohort metrics for a date range
pub async fn get_cohorts(
    repo: web::Data<DynOperatorMetricsRepository>,
    query: web::Query<DateRangeQuery>,
) -> Result<impl Responder, AppError> {
    let cohorts = repo
        .calculate_cohort_summary(query.start_date, query.end_date)
        .await?;

    Ok(HttpResponse::Ok().json(CohortSummaryResponse {
        total_participants: cohorts.total_participants,
        d1_completed: cohorts.d1_completed,
        d3_completed: cohorts.d3_completed,
        d7_completed: cohorts.d7_completed,
        d30_completed: cohorts.d30_completed,
        d7_rate: cohorts.d7_rate,
        d30_rate: cohorts.d30_rate,
    }))
}

/// GET /api/v1/admin/operator/finance
/// Financial summary for a date range
pub async fn get_finance(
    repo: web::Data<DynOperatorMetricsRepository>,
    query: web::Query<DateRangeQuery>,
) -> Result<impl Responder, AppError> {
    let finance = repo
        .calculate_financial_summary(query.start_date, query.end_date)
        .await?;

    Ok(HttpResponse::Ok().json(FinancialSummaryResponse {
        total_rewards_micros: finance.total_rewards_micros,
        total_allocated_micros: finance.total_allocated_micros,
        total_payable_micros: finance.total_payable_micros,
        total_paid_micros: finance.total_paid_micros,
        participant_share_micros: finance.participant_share_micros,
        referral_share_micros: finance.referral_share_micros,
        uno_share_micros: finance.uno_share_micros,
    }))
}

/// GET /api/v1/admin/operator/sync
/// Sync status and event queue metrics
pub async fn get_sync(
    repo: web::Data<DynOperatorMetricsRepository>,
) -> Result<impl Responder, AppError> {
    let sync = repo.calculate_sync_status().await?;

    Ok(HttpResponse::Ok().json(SyncStatusResponse {
        last_sync_at: sync.last_sync_at,
        sync_lag_seconds: sync.sync_lag_seconds,
        pending_outbox: sync.pending_outbox,
        pending_inbox: sync.pending_inbox,
        failed_jobs: sync.failed_jobs,
        dead_letter: sync.dead_letter,
    }))
}

/// GET /api/v1/admin/operator/exceptions
/// Unresolved exceptions with optional filtering
pub async fn get_exceptions(
    repo: web::Data<DynOperatorMetricsRepository>,
    query: web::Query<ExceptionsQuery>,
) -> Result<impl Responder, AppError> {
    let limit = query.limit.unwrap_or(50);
    let exceptions = repo.get_unresolved_exceptions(limit).await?;

    let response: Vec<ExceptionResponse> = exceptions
        .into_iter()
        .filter(|e| {
            // Filter by severity if specified
            if let Some(ref sev) = query.severity {
                let sev_lower = sev.to_lowercase();
                let e_sev = e.severity.to_string().to_lowercase();
                if e_sev != sev_lower {
                    return false;
                }
            }
            // Filter by exception_type if specified
            if let Some(ref et) = query.exception_type {
                if &e.exception_type != et {
                    return false;
                }
            }
            true
        })
        .map(|e| ExceptionResponse {
            id: e.id,
            exception_type: e.exception_type,
            severity: e.severity.to_string(),
            country_code: e.country_code,
            entity_type: e.entity_type,
            entity_id: e.entity_id,
            message: e.message,
            details: e.details,
            resolved_at: e.resolved_at,
            resolved_by: e.resolved_by,
            resolution_notes: e.resolution_notes,
            created_at: e.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/admin/operator/exceptions
/// Record a new exception
pub async fn record_exception(
    repo: web::Data<DynOperatorMetricsRepository>,
    body: web::Json<RecordExceptionRequest>,
) -> Result<impl Responder, AppError> {
    let severity = match body.severity.to_lowercase().as_str() {
        "info" => ExceptionSeverity::Info,
        "warning" => ExceptionSeverity::Warning,
        "error" => ExceptionSeverity::Error,
        "critical" => ExceptionSeverity::Critical,
        _ => ExceptionSeverity::Info,
    };

    let exception = repo
        .record_exception(RecordExceptionInput {
            exception_type: body.exception_type.clone(),
            severity,
            message: body.message.clone(),
            country_code: body.country_code.clone(),
            entity_type: body.entity_type.clone(),
            entity_id: body.entity_id.clone(),
            details: body.details.clone(),
            source_service: None,
            correlation_id: None,
            stack_trace: None,
        })
        .await?;

    Ok(HttpResponse::Created().json(ExceptionResponse {
        id: exception.id,
        exception_type: exception.exception_type,
        severity: exception.severity.to_string(),
        country_code: exception.country_code,
        entity_type: exception.entity_type,
        entity_id: exception.entity_id,
        message: exception.message,
        details: exception.details,
        resolved_at: exception.resolved_at,
        resolved_by: exception.resolved_by,
        resolution_notes: exception.resolution_notes,
        created_at: exception.created_at,
    }))
}

/// POST /api/v1/admin/operator/exceptions/{id}/resolve
/// Resolve an exception
pub async fn resolve_exception(
    repo: web::Data<DynOperatorMetricsRepository>,
    path: web::Path<Uuid>,
    body: web::Json<ResolveExceptionRequest>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let exception = repo
        .resolve_exception(id, &body.resolved_by, &body.resolution_notes)
        .await?;

    Ok(HttpResponse::Ok().json(ExceptionResponse {
        id: exception.id,
        exception_type: exception.exception_type,
        severity: exception.severity.to_string(),
        country_code: exception.country_code,
        entity_type: exception.entity_type,
        entity_id: exception.entity_id,
        message: exception.message,
        details: exception.details,
        resolved_at: exception.resolved_at,
        resolved_by: exception.resolved_by,
        resolution_notes: exception.resolution_notes,
        created_at: exception.created_at,
    }))
}

/// GET /api/v1/admin/operator/metrics
/// Get metrics for a specific date
pub async fn get_metrics(
    repo: web::Data<DynOperatorMetricsRepository>,
    query: web::Query<MetricsQuery>,
) -> Result<impl Responder, AppError> {
    let metrics = repo
        .get_metrics(query.date, query.country_code.as_deref())
        .await?;

    match metrics {
        Some(m) => Ok(HttpResponse::Ok().json(MetricsResponse {
            id: m.id,
            metric_date: m.metric_date,
            country_code: m.country_code,
            total_licenses: m.total_licenses,
            published_licenses: m.published_licenses,
            reserved_licenses: m.reserved_licenses,
            issued_licenses: m.issued_licenses,
            expired_licenses: m.expired_licenses,
            d1_completions: m.d1_completions,
            d7_completions: m.d7_completions,
            d30_completions: m.d30_completions,
            d7_completion_rate: m.d7_completion_rate,
            d30_completion_rate: m.d30_completion_rate,
            total_rewards_micros: m.total_rewards_micros,
            total_allocated_micros: m.total_allocated_micros,
            total_payable_micros: m.total_payable_micros,
            total_paid_micros: m.total_paid_micros,
            open_tickets: m.open_tickets,
            resolved_tickets: m.resolved_tickets,
            avg_resolution_hours: m.avg_resolution_hours,
            created_at: m.created_at,
        })),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

/// GET /api/v1/admin/operator/metrics/range
/// Get metrics for a date range
pub async fn get_metrics_range(
    repo: web::Data<DynOperatorMetricsRepository>,
    query: web::Query<DateRangeQuery>,
) -> Result<impl Responder, AppError> {
    let metrics = repo
        .get_metrics_range(query.start_date, query.end_date, query.country_code.as_deref())
        .await?;

    let response: Vec<MetricsResponse> = metrics
        .into_iter()
        .map(|m| MetricsResponse {
            id: m.id,
            metric_date: m.metric_date,
            country_code: m.country_code,
            total_licenses: m.total_licenses,
            published_licenses: m.published_licenses,
            reserved_licenses: m.reserved_licenses,
            issued_licenses: m.issued_licenses,
            expired_licenses: m.expired_licenses,
            d1_completions: m.d1_completions,
            d7_completions: m.d7_completions,
            d30_completions: m.d30_completions,
            d7_completion_rate: m.d7_completion_rate,
            d30_completion_rate: m.d30_completion_rate,
            total_rewards_micros: m.total_rewards_micros,
            total_allocated_micros: m.total_allocated_micros,
            total_payable_micros: m.total_payable_micros,
            total_paid_micros: m.total_paid_micros,
            open_tickets: m.open_tickets,
            resolved_tickets: m.resolved_tickets,
            avg_resolution_hours: m.avg_resolution_hours,
            created_at: m.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/v1/admin/operator/metrics/aggregate
/// Trigger daily metrics aggregation
pub async fn aggregate_metrics(
    repo: web::Data<DynOperatorMetricsRepository>,
    query: web::Query<MetricsQuery>,
) -> Result<impl Responder, AppError> {
    let metrics = repo
        .aggregate_daily_metrics(query.date, query.country_code.as_deref())
        .await?;

    Ok(HttpResponse::Ok().json(MetricsResponse {
        id: metrics.id,
        metric_date: metrics.metric_date,
        country_code: metrics.country_code,
        total_licenses: metrics.total_licenses,
        published_licenses: metrics.published_licenses,
        reserved_licenses: metrics.reserved_licenses,
        issued_licenses: metrics.issued_licenses,
        expired_licenses: metrics.expired_licenses,
        d1_completions: metrics.d1_completions,
        d7_completions: metrics.d7_completions,
        d30_completions: metrics.d30_completions,
        d7_completion_rate: metrics.d7_completion_rate,
        d30_completion_rate: metrics.d30_completion_rate,
        total_rewards_micros: metrics.total_rewards_micros,
        total_allocated_micros: metrics.total_allocated_micros,
        total_payable_micros: metrics.total_payable_micros,
        total_paid_micros: metrics.total_paid_micros,
        open_tickets: metrics.open_tickets,
        resolved_tickets: metrics.resolved_tickets,
        avg_resolution_hours: metrics.avg_resolution_hours,
        created_at: metrics.created_at,
    }))
}

/// GET /api/v1/admin/operator/metrics/export
/// Export metrics as CSV
pub async fn export_metrics(
    repo: web::Data<DynOperatorMetricsRepository>,
    query: web::Query<DateRangeQuery>,
) -> Result<impl Responder, AppError> {
    let metrics = repo
        .get_metrics_range(query.start_date, query.end_date, query.country_code.as_deref())
        .await?;

    // Build CSV
    let mut csv = String::from(
        "date,country,total_licenses,published,reserved,issued,expired,\
         d1_completions,d7_completions,d30_completions,d7_rate,d30_rate,\
         total_rewards,total_allocated,total_payable,total_paid,\
         open_tickets,resolved_tickets,avg_resolution_hours\n",
    );

    for m in metrics {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{:.4},{:.4},{},{},{},{},{},{},{:.2}\n",
            m.metric_date,
            m.country_code.as_deref().unwrap_or("global"),
            m.total_licenses,
            m.published_licenses,
            m.reserved_licenses,
            m.issued_licenses,
            m.expired_licenses,
            m.d1_completions,
            m.d7_completions,
            m.d30_completions,
            m.d7_completion_rate.unwrap_or(0.0),
            m.d30_completion_rate.unwrap_or(0.0),
            m.total_rewards_micros,
            m.total_allocated_micros,
            m.total_payable_micros,
            m.total_paid_micros,
            m.open_tickets,
            m.resolved_tickets,
            m.avg_resolution_hours.unwrap_or(0.0),
        ));
    }

    Ok(HttpResponse::Ok()
        .content_type("text/csv")
        .insert_header(("Content-Disposition", "attachment; filename=operator_metrics.csv"))
        .body(csv))
}
