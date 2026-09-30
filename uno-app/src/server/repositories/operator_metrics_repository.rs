//! Operator metrics repository for dashboard snapshots and exception tracking
//!
//! Implements CRUD operations for:
//! - Daily metrics snapshots (inventory, cohort, financial, support, sync)
//! - Exception tracking with severity and resolution
//! - Margin tracking by country/task

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Daily aggregated metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OperatorMetrics {
    pub id: Uuid,
    pub metric_date: NaiveDate,
    pub country_code: Option<String>,

    // Inventory metrics
    pub total_licenses: i32,
    pub published_licenses: i32,
    pub reserved_licenses: i32,
    pub issued_licenses: i32,
    pub expired_licenses: i32,
    pub quarantined_licenses: i32,

    // Cohort metrics
    pub new_cohorts: i32,
    pub d1_completions: i32,
    pub d3_completions: i32,
    pub d7_completions: i32,
    pub d30_completions: i32,
    pub d7_completion_rate: Option<f64>,
    pub d30_completion_rate: Option<f64>,
    pub churned_count: i32,

    // Financial metrics (micros)
    pub total_rewards_micros: i64,
    pub total_allocated_micros: i64,
    pub participant_allocated_micros: i64,
    pub referral_allocated_micros: i64,
    pub uno_allocated_micros: i64,
    pub total_payable_micros: i64,
    pub total_paid_micros: i64,

    // Support metrics
    pub open_tickets: i32,
    pub new_tickets: i32,
    pub resolved_tickets: i32,
    pub escalated_tickets: i32,
    pub avg_resolution_hours: Option<f64>,
    pub avg_first_response_hours: Option<f64>,

    // Sync/Integration metrics
    pub last_sync_at: Option<DateTime<Utc>>,
    pub sync_lag_seconds: Option<i32>,
    pub pending_outbox_events: i32,
    pub pending_inbox_events: i32,
    pub failed_jobs: i32,
    pub dead_letter_jobs: i32,

    // Agent metrics
    pub active_agents: i32,
    pub pending_agent_approvals: i32,

    pub created_at: DateTime<Utc>,
}

/// Exception severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "exception_severity", rename_all = "lowercase")]
pub enum ExceptionSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for ExceptionSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExceptionSeverity::Info => write!(f, "info"),
            ExceptionSeverity::Warning => write!(f, "warning"),
            ExceptionSeverity::Error => write!(f, "error"),
            ExceptionSeverity::Critical => write!(f, "critical"),
        }
    }
}

/// Operator exception for tracking issues
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OperatorException {
    pub id: Uuid,
    pub exception_type: String,
    pub severity: ExceptionSeverity,
    pub country_code: Option<String>,

    // Entity reference
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,

    // Exception details
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub stack_trace: Option<String>,

    // Source information
    pub source_service: Option<String>,
    pub correlation_id: Option<String>,

    // Resolution
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<String>,
    pub resolution_notes: Option<String>,
    pub auto_resolved: bool,

    // Occurrence tracking
    pub occurrence_count: i32,
    pub first_occurred_at: DateTime<Utc>,
    pub last_occurred_at: DateTime<Utc>,

    pub created_at: DateTime<Utc>,
}

/// Margin tracking record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OperatorMargin {
    pub id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub country_code: String,
    pub task_code: Option<String>,

    // Revenue
    pub gross_revenue_micros: i64,

    // Shares
    pub participant_share_micros: i64,
    pub referral_share_micros: i64,
    pub uno_share_micros: i64,

    // Costs
    pub acquisition_cost_micros: i64,
    pub support_cost_micros: i64,
    pub hosting_cost_micros: i64,
    pub messaging_cost_micros: i64,
    pub other_cost_micros: i64,

    // Derived (computed columns)
    pub total_cost_micros: i64,
    pub net_margin_micros: i64,
    pub margin_percentage: Option<f64>,

    // Counts
    pub active_licenses: i32,
    pub productive_licenses: i32,

    pub created_at: DateTime<Utc>,
}

// ============================================
// INPUT STRUCTS
// ============================================

/// Input for recording metrics
#[derive(Debug, Clone)]
pub struct RecordMetricsInput {
    pub metric_date: NaiveDate,
    pub country_code: Option<String>,

    // Inventory
    pub total_licenses: i32,
    pub published_licenses: i32,
    pub reserved_licenses: i32,
    pub issued_licenses: i32,
    pub expired_licenses: i32,
    pub quarantined_licenses: i32,

    // Cohort
    pub new_cohorts: i32,
    pub d1_completions: i32,
    pub d3_completions: i32,
    pub d7_completions: i32,
    pub d30_completions: i32,
    pub churned_count: i32,

    // Financial
    pub total_rewards_micros: i64,
    pub total_allocated_micros: i64,
    pub total_payable_micros: i64,
    pub total_paid_micros: i64,

    // Support
    pub open_tickets: i32,
    pub new_tickets: i32,
    pub resolved_tickets: i32,
    pub escalated_tickets: i32,
}

/// Input for recording an exception
#[derive(Debug, Clone)]
pub struct RecordExceptionInput {
    pub exception_type: String,
    pub severity: ExceptionSeverity,
    pub message: String,
    pub country_code: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub details: Option<serde_json::Value>,
    pub source_service: Option<String>,
    pub correlation_id: Option<String>,
    pub stack_trace: Option<String>,
}

/// Input for recording margin
#[derive(Debug, Clone)]
pub struct RecordMarginInput {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub country_code: String,
    pub task_code: Option<String>,
    pub gross_revenue_micros: i64,
    pub participant_share_micros: i64,
    pub referral_share_micros: i64,
    pub uno_share_micros: i64,
    pub acquisition_cost_micros: i64,
    pub support_cost_micros: i64,
    pub hosting_cost_micros: i64,
    pub messaging_cost_micros: i64,
    pub other_cost_micros: i64,
    pub active_licenses: i32,
    pub productive_licenses: i32,
}

// ============================================
// SUMMARY STRUCTS
// ============================================

/// Inventory summary
#[derive(Debug, Clone, Serialize)]
pub struct InventorySummary {
    pub total: i64,
    pub published: i64,
    pub reserved: i64,
    pub issued: i64,
    pub expired: i64,
    pub quarantined: i64,
}

/// Cohort summary
#[derive(Debug, Clone, Serialize)]
pub struct CohortSummary {
    pub total_participants: i64,
    pub d1_completed: i64,
    pub d3_completed: i64,
    pub d7_completed: i64,
    pub d30_completed: i64,
    pub d7_rate: f64,
    pub d30_rate: f64,
}

/// Financial summary
#[derive(Debug, Clone, Serialize)]
pub struct FinancialSummary {
    pub total_rewards_micros: i64,
    pub total_allocated_micros: i64,
    pub total_payable_micros: i64,
    pub total_paid_micros: i64,
    pub participant_share_micros: i64,
    pub referral_share_micros: i64,
    pub uno_share_micros: i64,
}

/// Support summary
#[derive(Debug, Clone, Serialize)]
pub struct SupportSummary {
    pub open_tickets: i64,
    pub new_today: i64,
    pub resolved_today: i64,
    pub escalated: i64,
    pub avg_resolution_hours: Option<f64>,
}

/// Sync status
#[derive(Debug, Clone, Serialize)]
pub struct SyncStatus {
    pub last_sync_at: Option<DateTime<Utc>>,
    pub sync_lag_seconds: Option<i32>,
    pub pending_outbox: i64,
    pub pending_inbox: i64,
    pub failed_jobs: i64,
    pub dead_letter: i64,
}

// ============================================
// TRAIT DEFINITION
// ============================================

/// Dynamic type alias for OperatorMetricsRepository trait object
pub type DynOperatorMetricsRepository = Arc<dyn OperatorMetricsRepository + Send + Sync>;

/// Operator metrics repository trait
#[async_trait]
pub trait OperatorMetricsRepository: Send + Sync {
    // --- Metrics Snapshots ---

    /// Record daily metrics
    async fn record_metrics(&self, input: RecordMetricsInput) -> Result<OperatorMetrics, AppError>;

    /// Get metrics for a specific date
    async fn get_metrics(
        &self,
        date: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<Option<OperatorMetrics>, AppError>;

    /// Get metrics for a date range
    async fn get_metrics_range(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<Vec<OperatorMetrics>, AppError>;

    /// Aggregate metrics using database function
    async fn aggregate_daily_metrics(
        &self,
        date: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<OperatorMetrics, AppError>;

    // --- Real-time Summaries ---

    /// Calculate current inventory summary
    async fn calculate_inventory_summary(
        &self,
        country_code: Option<&str>,
    ) -> Result<InventorySummary, AppError>;

    /// Calculate cohort summary for date range
    async fn calculate_cohort_summary(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<CohortSummary, AppError>;

    /// Calculate financial summary for date range
    async fn calculate_financial_summary(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<FinancialSummary, AppError>;

    /// Calculate current support summary
    async fn calculate_support_summary(&self) -> Result<SupportSummary, AppError>;

    /// Calculate current sync status
    async fn calculate_sync_status(&self) -> Result<SyncStatus, AppError>;

    // --- Exceptions ---

    /// Record an exception (with deduplication)
    async fn record_exception(&self, input: RecordExceptionInput) -> Result<OperatorException, AppError>;

    /// Get unresolved exceptions
    async fn get_unresolved_exceptions(&self, limit: i32) -> Result<Vec<OperatorException>, AppError>;

    /// Get exceptions by severity
    async fn get_exceptions_by_severity(
        &self,
        severity: ExceptionSeverity,
        limit: i32,
    ) -> Result<Vec<OperatorException>, AppError>;

    /// Get exception by ID
    async fn get_exception(&self, id: Uuid) -> Result<Option<OperatorException>, AppError>;

    /// Resolve exception
    async fn resolve_exception(
        &self,
        id: Uuid,
        resolved_by: &str,
        notes: &str,
    ) -> Result<OperatorException, AppError>;

    /// Auto-resolve exception
    async fn auto_resolve_exception(&self, id: Uuid) -> Result<OperatorException, AppError>;

    // --- Margins ---

    /// Record margin data
    async fn record_margin(&self, input: RecordMarginInput) -> Result<OperatorMargin, AppError>;

    /// Get margins for period
    async fn get_margins(
        &self,
        period_start: NaiveDate,
        period_end: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<Vec<OperatorMargin>, AppError>;

    /// Get margin by country and task
    async fn get_margin_by_country_task(
        &self,
        period_start: NaiveDate,
        period_end: NaiveDate,
        country_code: &str,
        task_code: Option<&str>,
    ) -> Result<Option<OperatorMargin>, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct OperatorMetricsRepositoryImpl {
    pool: ConnectionPool,
}

impl OperatorMetricsRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OperatorMetricsRepository for OperatorMetricsRepositoryImpl {
    async fn record_metrics(&self, input: RecordMetricsInput) -> Result<OperatorMetrics, AppError> {
        let metrics = sqlx::query_as::<_, OperatorMetrics>(
            r#"
            INSERT INTO operator_metrics (
                metric_date, country_code,
                total_licenses, published_licenses, reserved_licenses, issued_licenses,
                expired_licenses, quarantined_licenses,
                new_cohorts, d1_completions, d3_completions, d7_completions, d30_completions, churned_count,
                total_rewards_micros, total_allocated_micros, total_payable_micros, total_paid_micros,
                open_tickets, new_tickets, resolved_tickets, escalated_tickets
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
            ON CONFLICT (metric_date, country_code) DO UPDATE SET
                total_licenses = EXCLUDED.total_licenses,
                published_licenses = EXCLUDED.published_licenses,
                reserved_licenses = EXCLUDED.reserved_licenses,
                issued_licenses = EXCLUDED.issued_licenses,
                expired_licenses = EXCLUDED.expired_licenses,
                quarantined_licenses = EXCLUDED.quarantined_licenses,
                new_cohorts = EXCLUDED.new_cohorts,
                d1_completions = EXCLUDED.d1_completions,
                d3_completions = EXCLUDED.d3_completions,
                d7_completions = EXCLUDED.d7_completions,
                d30_completions = EXCLUDED.d30_completions,
                churned_count = EXCLUDED.churned_count,
                total_rewards_micros = EXCLUDED.total_rewards_micros,
                total_allocated_micros = EXCLUDED.total_allocated_micros,
                total_payable_micros = EXCLUDED.total_payable_micros,
                total_paid_micros = EXCLUDED.total_paid_micros,
                open_tickets = EXCLUDED.open_tickets,
                new_tickets = EXCLUDED.new_tickets,
                resolved_tickets = EXCLUDED.resolved_tickets,
                escalated_tickets = EXCLUDED.escalated_tickets
            RETURNING *
            "#,
        )
        .bind(input.metric_date)
        .bind(&input.country_code)
        .bind(input.total_licenses)
        .bind(input.published_licenses)
        .bind(input.reserved_licenses)
        .bind(input.issued_licenses)
        .bind(input.expired_licenses)
        .bind(input.quarantined_licenses)
        .bind(input.new_cohorts)
        .bind(input.d1_completions)
        .bind(input.d3_completions)
        .bind(input.d7_completions)
        .bind(input.d30_completions)
        .bind(input.churned_count)
        .bind(input.total_rewards_micros)
        .bind(input.total_allocated_micros)
        .bind(input.total_payable_micros)
        .bind(input.total_paid_micros)
        .bind(input.open_tickets)
        .bind(input.new_tickets)
        .bind(input.resolved_tickets)
        .bind(input.escalated_tickets)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(metrics)
    }

    async fn get_metrics(
        &self,
        date: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<Option<OperatorMetrics>, AppError> {
        let metrics = if let Some(cc) = country_code {
            sqlx::query_as::<_, OperatorMetrics>(
                "SELECT * FROM operator_metrics WHERE metric_date = $1 AND country_code = $2",
            )
            .bind(date)
            .bind(cc)
            .fetch_optional(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, OperatorMetrics>(
                "SELECT * FROM operator_metrics WHERE metric_date = $1 AND country_code IS NULL",
            )
            .bind(date)
            .fetch_optional(&self.pool)
            .await
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(metrics)
    }

    async fn get_metrics_range(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<Vec<OperatorMetrics>, AppError> {
        let metrics = if let Some(cc) = country_code {
            sqlx::query_as::<_, OperatorMetrics>(
                r#"
                SELECT * FROM operator_metrics
                WHERE metric_date >= $1 AND metric_date <= $2 AND country_code = $3
                ORDER BY metric_date ASC
                "#,
            )
            .bind(start)
            .bind(end)
            .bind(cc)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, OperatorMetrics>(
                r#"
                SELECT * FROM operator_metrics
                WHERE metric_date >= $1 AND metric_date <= $2 AND country_code IS NULL
                ORDER BY metric_date ASC
                "#,
            )
            .bind(start)
            .bind(end)
            .fetch_all(&self.pool)
            .await
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(metrics)
    }

    async fn aggregate_daily_metrics(
        &self,
        date: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<OperatorMetrics, AppError> {
        // Call the stored function
        let (metric_id,): (Uuid,) = sqlx::query_as("SELECT aggregate_daily_metrics($1, $2)")
            .bind(date)
            .bind(country_code)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Fetch the created metrics
        self.get_metrics(date, country_code)
            .await?
            .ok_or_else(|| AppError::InternalServerError(format!("Failed to aggregate metrics for {}", date)))
    }

    async fn calculate_inventory_summary(
        &self,
        country_code: Option<&str>,
    ) -> Result<InventorySummary, AppError> {
        let summary = if let Some(cc) = country_code {
            sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64)>(
                r#"
                SELECT
                    COUNT(*) as total,
                    COUNT(*) FILTER (WHERE status = 'published') as published,
                    COUNT(*) FILTER (WHERE status = 'reserved') as reserved,
                    COUNT(*) FILTER (WHERE status = 'issued') as issued,
                    COUNT(*) FILTER (WHERE status = 'expired') as expired,
                    COUNT(*) FILTER (WHERE status = 'quarantined') as quarantined
                FROM licenses
                WHERE country_code = $1
                "#,
            )
            .bind(cc)
            .fetch_one(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64)>(
                r#"
                SELECT
                    COUNT(*) as total,
                    COUNT(*) FILTER (WHERE status = 'published') as published,
                    COUNT(*) FILTER (WHERE status = 'reserved') as reserved,
                    COUNT(*) FILTER (WHERE status = 'issued') as issued,
                    COUNT(*) FILTER (WHERE status = 'expired') as expired,
                    COUNT(*) FILTER (WHERE status = 'quarantined') as quarantined
                FROM licenses
                "#,
            )
            .fetch_one(&self.pool)
            .await
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(InventorySummary {
            total: summary.0,
            published: summary.1,
            reserved: summary.2,
            issued: summary.3,
            expired: summary.4,
            quarantined: summary.5,
        })
    }

    async fn calculate_cohort_summary(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<CohortSummary, AppError> {
        let summary = sqlx::query_as::<_, (i64, i64, i64, i64, i64)>(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE d1_completed) as d1,
                COUNT(*) FILTER (WHERE d3_completed) as d3,
                COUNT(*) FILTER (WHERE d7_completed) as d7,
                COUNT(*) FILTER (WHERE d30_completed) as d30
            FROM participant_cohorts
            WHERE cohort_date >= $1 AND cohort_date <= $2
            "#,
        )
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let total = summary.0.max(1) as f64;
        Ok(CohortSummary {
            total_participants: summary.0,
            d1_completed: summary.1,
            d3_completed: summary.2,
            d7_completed: summary.3,
            d30_completed: summary.4,
            d7_rate: (summary.3 as f64 / total) * 100.0,
            d30_rate: (summary.4 as f64 / total) * 100.0,
        })
    }

    async fn calculate_financial_summary(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<FinancialSummary, AppError> {
        let summary = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64, i64)>(
            r#"
            SELECT
                COALESCE(SUM(amount_micros), 0) as total_rewards,
                COALESCE(SUM(amount_micros) FILTER (WHERE state IN ('allocated', 'payable', 'paid')), 0) as allocated,
                COALESCE(SUM(amount_micros) FILTER (WHERE state = 'payable'), 0) as payable,
                COALESCE(SUM(amount_micros) FILTER (WHERE state = 'paid'), 0) as paid,
                COALESCE(SUM(amount_micros) FILTER (WHERE share_type = 'participant'), 0) as participant,
                COALESCE(SUM(amount_micros) FILTER (WHERE share_type = 'referral'), 0) as referral,
                COALESCE(SUM(amount_micros) FILTER (WHERE share_type = 'uno'), 0) as uno
            FROM allocation_ledger
            WHERE created_at::DATE >= $1 AND created_at::DATE <= $2
            "#,
        )
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(FinancialSummary {
            total_rewards_micros: summary.0,
            total_allocated_micros: summary.1,
            total_payable_micros: summary.2,
            total_paid_micros: summary.3,
            participant_share_micros: summary.4,
            referral_share_micros: summary.5,
            uno_share_micros: summary.6,
        })
    }

    async fn calculate_support_summary(&self) -> Result<SupportSummary, AppError> {
        let summary = sqlx::query_as::<_, (i64, i64, i64, i64, Option<f64>)>(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE status IN ('open', 'in_progress', 'waiting_user')) as open_tickets,
                COUNT(*) FILTER (WHERE created_at::DATE = CURRENT_DATE) as new_today,
                COUNT(*) FILTER (WHERE resolved_at::DATE = CURRENT_DATE) as resolved_today,
                COUNT(*) FILTER (WHERE escalated_at IS NOT NULL AND status != 'resolved') as escalated,
                AVG(EXTRACT(EPOCH FROM (resolved_at - created_at)) / 3600)
                    FILTER (WHERE resolved_at IS NOT NULL) as avg_resolution
            FROM support_tickets
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(SupportSummary {
            open_tickets: summary.0,
            new_today: summary.1,
            resolved_today: summary.2,
            escalated: summary.3,
            avg_resolution_hours: summary.4,
        })
    }

    async fn calculate_sync_status(&self) -> Result<SyncStatus, AppError> {
        let summary = sqlx::query_as::<_, (Option<DateTime<Utc>>, i64, i64, i64, i64)>(
            r#"
            SELECT
                (SELECT MAX(processed_at) FROM outbox_events WHERE status = 'processed') as last_sync,
                (SELECT COUNT(*) FROM outbox_events WHERE status = 'pending') as pending_outbox,
                (SELECT COUNT(*) FROM inbox_events WHERE status = 'pending') as pending_inbox,
                (SELECT COUNT(*) FROM job_queue WHERE status = 'failed') as failed_jobs,
                (SELECT COUNT(*) FROM job_queue WHERE status = 'dead_letter') as dead_letter
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let sync_lag = summary.0.map(|last| {
            (Utc::now() - last).num_seconds() as i32
        });

        Ok(SyncStatus {
            last_sync_at: summary.0,
            sync_lag_seconds: sync_lag,
            pending_outbox: summary.1,
            pending_inbox: summary.2,
            failed_jobs: summary.3,
            dead_letter: summary.4,
        })
    }

    async fn record_exception(&self, input: RecordExceptionInput) -> Result<OperatorException, AppError> {
        // Use the stored function for deduplication
        let (exception_id,): (Uuid,) = sqlx::query_as(
            "SELECT record_exception($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(&input.exception_type)
        .bind(input.severity)
        .bind(&input.message)
        .bind(&input.country_code)
        .bind(&input.entity_type)
        .bind(&input.entity_id)
        .bind(&input.details)
        .bind(&input.source_service)
        .bind(&input.correlation_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.get_exception(exception_id)
            .await?
            .ok_or_else(|| AppError::InternalServerError("Failed to record exception".to_string()))
    }

    async fn get_unresolved_exceptions(&self, limit: i32) -> Result<Vec<OperatorException>, AppError> {
        let exceptions = sqlx::query_as::<_, OperatorException>(
            r#"
            SELECT * FROM operator_exceptions
            WHERE resolved_at IS NULL
            ORDER BY
                CASE severity
                    WHEN 'critical' THEN 1
                    WHEN 'error' THEN 2
                    WHEN 'warning' THEN 3
                    ELSE 4
                END,
                last_occurred_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(exceptions)
    }

    async fn get_exceptions_by_severity(
        &self,
        severity: ExceptionSeverity,
        limit: i32,
    ) -> Result<Vec<OperatorException>, AppError> {
        let exceptions = sqlx::query_as::<_, OperatorException>(
            r#"
            SELECT * FROM operator_exceptions
            WHERE severity = $1 AND resolved_at IS NULL
            ORDER BY last_occurred_at DESC
            LIMIT $2
            "#,
        )
        .bind(severity)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(exceptions)
    }

    async fn get_exception(&self, id: Uuid) -> Result<Option<OperatorException>, AppError> {
        let exception = sqlx::query_as::<_, OperatorException>(
            "SELECT * FROM operator_exceptions WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(exception)
    }

    async fn resolve_exception(
        &self,
        id: Uuid,
        resolved_by: &str,
        notes: &str,
    ) -> Result<OperatorException, AppError> {
        let exception = sqlx::query_as::<_, OperatorException>(
            r#"
            UPDATE operator_exceptions
            SET resolved_at = NOW(), resolved_by = $2, resolution_notes = $3
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(resolved_by)
        .bind(notes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(exception)
    }

    async fn auto_resolve_exception(&self, id: Uuid) -> Result<OperatorException, AppError> {
        let exception = sqlx::query_as::<_, OperatorException>(
            r#"
            UPDATE operator_exceptions
            SET resolved_at = NOW(), auto_resolved = TRUE, resolution_notes = 'Auto-resolved'
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(exception)
    }

    async fn record_margin(&self, input: RecordMarginInput) -> Result<OperatorMargin, AppError> {
        let margin_percentage = if input.gross_revenue_micros > 0 {
            let net = input.uno_share_micros - input.acquisition_cost_micros
                - input.support_cost_micros - input.hosting_cost_micros
                - input.messaging_cost_micros - input.other_cost_micros;
            Some((net as f64 / input.gross_revenue_micros as f64) * 100.0)
        } else {
            None
        };

        let margin = sqlx::query_as::<_, OperatorMargin>(
            r#"
            INSERT INTO operator_margins (
                period_start, period_end, country_code, task_code,
                gross_revenue_micros, participant_share_micros, referral_share_micros, uno_share_micros,
                acquisition_cost_micros, support_cost_micros, hosting_cost_micros,
                messaging_cost_micros, other_cost_micros,
                margin_percentage, active_licenses, productive_licenses
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (period_start, period_end, country_code, task_code) DO UPDATE SET
                gross_revenue_micros = EXCLUDED.gross_revenue_micros,
                participant_share_micros = EXCLUDED.participant_share_micros,
                referral_share_micros = EXCLUDED.referral_share_micros,
                uno_share_micros = EXCLUDED.uno_share_micros,
                acquisition_cost_micros = EXCLUDED.acquisition_cost_micros,
                support_cost_micros = EXCLUDED.support_cost_micros,
                hosting_cost_micros = EXCLUDED.hosting_cost_micros,
                messaging_cost_micros = EXCLUDED.messaging_cost_micros,
                other_cost_micros = EXCLUDED.other_cost_micros,
                margin_percentage = EXCLUDED.margin_percentage,
                active_licenses = EXCLUDED.active_licenses,
                productive_licenses = EXCLUDED.productive_licenses
            RETURNING *
            "#,
        )
        .bind(input.period_start)
        .bind(input.period_end)
        .bind(&input.country_code)
        .bind(&input.task_code)
        .bind(input.gross_revenue_micros)
        .bind(input.participant_share_micros)
        .bind(input.referral_share_micros)
        .bind(input.uno_share_micros)
        .bind(input.acquisition_cost_micros)
        .bind(input.support_cost_micros)
        .bind(input.hosting_cost_micros)
        .bind(input.messaging_cost_micros)
        .bind(input.other_cost_micros)
        .bind(margin_percentage)
        .bind(input.active_licenses)
        .bind(input.productive_licenses)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(margin)
    }

    async fn get_margins(
        &self,
        period_start: NaiveDate,
        period_end: NaiveDate,
        country_code: Option<&str>,
    ) -> Result<Vec<OperatorMargin>, AppError> {
        let margins = if let Some(cc) = country_code {
            sqlx::query_as::<_, OperatorMargin>(
                r#"
                SELECT * FROM operator_margins
                WHERE period_start >= $1 AND period_end <= $2 AND country_code = $3
                ORDER BY period_start, country_code, task_code
                "#,
            )
            .bind(period_start)
            .bind(period_end)
            .bind(cc)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, OperatorMargin>(
                r#"
                SELECT * FROM operator_margins
                WHERE period_start >= $1 AND period_end <= $2
                ORDER BY period_start, country_code, task_code
                "#,
            )
            .bind(period_start)
            .bind(period_end)
            .fetch_all(&self.pool)
            .await
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(margins)
    }

    async fn get_margin_by_country_task(
        &self,
        period_start: NaiveDate,
        period_end: NaiveDate,
        country_code: &str,
        task_code: Option<&str>,
    ) -> Result<Option<OperatorMargin>, AppError> {
        let margin = if let Some(tc) = task_code {
            sqlx::query_as::<_, OperatorMargin>(
                r#"
                SELECT * FROM operator_margins
                WHERE period_start = $1 AND period_end = $2
                  AND country_code = $3 AND task_code = $4
                "#,
            )
            .bind(period_start)
            .bind(period_end)
            .bind(country_code)
            .bind(tc)
            .fetch_optional(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, OperatorMargin>(
                r#"
                SELECT * FROM operator_margins
                WHERE period_start = $1 AND period_end = $2
                  AND country_code = $3 AND task_code IS NULL
                "#,
            )
            .bind(period_start)
            .bind(period_end)
            .bind(country_code)
            .fetch_optional(&self.pool)
            .await
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(margin)
    }
}
