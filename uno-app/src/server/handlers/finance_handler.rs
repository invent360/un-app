//! Finance HTTP handlers
//!
//! Provides REST endpoints for allocation management, settlements, and finance operations.

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::app::ServiceFactory;
use crate::server::services::{
    RecordAllocationInput, PrepareSettlementInput, ExecuteSettlementInput,
};
use crate::server::repositories::{AllocationState, PoolBalanceSummary, PayableBalances};

// ============================================
// REQUEST/RESPONSE TYPES
// ============================================

#[derive(Debug, Deserialize)]
pub struct CreateAllocationRequest {
    pub license_id: String,
    pub pool_micros: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(default = "default_version")]
    pub agreement_version: i32,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub source: String,
    pub external_ref: Option<String>,
}

fn default_currency() -> String {
    "USD".to_string()
}

fn default_version() -> i32 {
    1
}

#[derive(Debug, Deserialize)]
pub struct ListAllocationsQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct MarkPayableRequest {
    pub ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct PrepareSettlementRequest {
    pub party_type: String,  // ulo, uno, referral
    pub allocation_ids: Vec<Uuid>,
    #[serde(default)]
    pub fee_micros: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteSettlementRequest {
    pub provider: String,
    pub provider_ref: String,
}

#[derive(Debug, Serialize)]
pub struct AllocationResponse {
    pub id: String,
    pub license_id: String,
    pub pool_micros: i64,
    pub pool_currency: String,
    pub ulo_micros: i64,
    pub uno_micros: i64,
    pub referral_micros: i64,
    pub ulo_bps: i32,
    pub uno_bps: i32,
    pub referral_bps: i32,
    pub remainder_micros: i64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub source: String,
    pub external_ref: Option<String>,
    pub state: Option<String>,
    pub allocated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PoolBalanceResponse {
    pub license_id: String,
    pub total_pool_micros: i64,
    pub total_pool_dollars: f64,
    pub total_ulo_micros: i64,
    pub total_uno_allocated_micros: i64,
    pub total_referral_micros: i64,
    pub total_credit_expenditure_micros: i64,
    pub net_uno_contribution_micros: i64,
    pub net_uno_contribution_dollars: f64,
    pub below_break_even_threshold: bool,
    pub accrued_micros: i64,
    pub payable_micros: i64,
    pub paid_micros: i64,
}

#[derive(Debug, Serialize)]
pub struct PayableBalancesResponse {
    pub total_ulo_payable_micros: i64,
    pub total_ulo_payable_dollars: f64,
    pub total_uno_payable_micros: i64,
    pub total_uno_payable_dollars: f64,
    pub total_referral_payable_micros: i64,
    pub total_referral_payable_dollars: f64,
    pub license_count: i64,
}

#[derive(Debug, Serialize)]
pub struct SettlementResponse {
    pub id: String,
    pub settlement_ref: String,
    pub party_type: String,
    pub total_micros: i64,
    pub total_dollars: f64,
    pub fee_micros: i64,
    pub net_micros: i64,
    pub net_dollars: f64,
    pub allocation_count: i32,
    pub state: String,
    pub prepared_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub executed_at: Option<DateTime<Utc>>,
}

// ============================================
// HELPERS
// ============================================

fn service_unavailable() -> HttpResponse {
    HttpResponse::ServiceUnavailable().json(serde_json::json!({
        "error": "Service factory not available"
    }))
}

fn error_response<E: std::fmt::Display>(e: E) -> HttpResponse {
    HttpResponse::InternalServerError().json(serde_json::json!({
        "error": e.to_string()
    }))
}

fn get_admin_id(req: &HttpRequest) -> String {
    req.headers()
        .get("X-Admin-Id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("admin")
        .to_string()
}

fn micros_to_dollars(micros: i64) -> f64 {
    micros as f64 / 1_000_000.0
}

// ============================================
// ALLOCATION HANDLERS
// ============================================

/// POST /api/v1/admin/finance/allocations
/// Create a new allocation entry
pub async fn create_allocation(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<CreateAllocationRequest>,
    _req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let settlement_service = match &factory.settlement_service {
        Some(s) => s,
        None => return HttpResponse::NotImplemented().json(serde_json::json!({
            "error": "Settlement service not configured"
        })),
    };

    let input = RecordAllocationInput {
        license_id: body.license_id.clone(),
        pool_micros: body.pool_micros,
        currency: body.currency.clone(),
        agreement_version: body.agreement_version,
        period_start: body.period_start,
        period_end: body.period_end,
        source: body.source.clone(),
        external_ref: body.external_ref.clone(),
    };

    match settlement_service.record_allocation(input).await {
        Ok(entry) => HttpResponse::Created().json(AllocationResponse {
            id: entry.id.to_string(),
            license_id: entry.license_id,
            pool_micros: entry.pool_micros,
            pool_currency: entry.pool_currency,
            ulo_micros: entry.ulo_micros,
            uno_micros: entry.uno_micros,
            referral_micros: entry.referral_micros,
            ulo_bps: entry.ulo_bps,
            uno_bps: entry.uno_bps,
            referral_bps: entry.referral_bps,
            remainder_micros: entry.remainder_micros,
            period_start: entry.period_start,
            period_end: entry.period_end,
            source: entry.source,
            external_ref: entry.external_ref,
            state: entry.state,
            allocated_at: entry.allocated_at,
        }),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/finance/allocations/{id}
/// Get allocation by ID
pub async fn get_allocation(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let settlement_service = match &factory.settlement_service {
        Some(s) => s,
        None => return HttpResponse::NotImplemented().json(serde_json::json!({
            "error": "Settlement service not configured"
        })),
    };

    let id = path.into_inner();

    match settlement_service.get_allocation(id).await {
        Ok(Some(entry)) => HttpResponse::Ok().json(AllocationResponse {
            id: entry.id.to_string(),
            license_id: entry.license_id,
            pool_micros: entry.pool_micros,
            pool_currency: entry.pool_currency,
            ulo_micros: entry.ulo_micros,
            uno_micros: entry.uno_micros,
            referral_micros: entry.referral_micros,
            ulo_bps: entry.ulo_bps,
            uno_bps: entry.uno_bps,
            referral_bps: entry.referral_bps,
            remainder_micros: entry.remainder_micros,
            period_start: entry.period_start,
            period_end: entry.period_end,
            source: entry.source,
            external_ref: entry.external_ref,
            state: entry.state,
            allocated_at: entry.allocated_at,
        }),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Allocation not found"
        })),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/finance/allocations
/// List allocations (with optional filters)
pub async fn list_allocations(
    factory: Option<web::Data<ServiceFactory>>,
    query: web::Query<ListAllocationsQuery>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let allocation_repo = match &factory.allocation_repository {
        Some(r) => r,
        None => return HttpResponse::NotImplemented().json(serde_json::json!({
            "error": "Allocation repository not configured"
        })),
    };

    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    // List all allocations (by state = accrued for default)
    match allocation_repo.list_by_state(AllocationState::Accrued, limit, offset).await {
        Ok(entries) => {
            let items: Vec<AllocationResponse> = entries.into_iter().map(|entry| {
                AllocationResponse {
                    id: entry.id.to_string(),
                    license_id: entry.license_id,
                    pool_micros: entry.pool_micros,
                    pool_currency: entry.pool_currency,
                    ulo_micros: entry.ulo_micros,
                    uno_micros: entry.uno_micros,
                    referral_micros: entry.referral_micros,
                    ulo_bps: entry.ulo_bps,
                    uno_bps: entry.uno_bps,
                    referral_bps: entry.referral_bps,
                    remainder_micros: entry.remainder_micros,
                    period_start: entry.period_start,
                    period_end: entry.period_end,
                    source: entry.source,
                    external_ref: entry.external_ref,
                    state: entry.state,
                    allocated_at: entry.allocated_at,
                }
            }).collect();

            HttpResponse::Ok().json(serde_json::json!({
                "items": items,
                "count": items.len()
            }))
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/finance/licenses/{id}/balance
/// Get pool balance summary for a license
pub async fn get_pool_balance(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let settlement_service = match &factory.settlement_service {
        Some(s) => s,
        None => return HttpResponse::NotImplemented().json(serde_json::json!({
            "error": "Settlement service not configured"
        })),
    };

    let license_id = path.into_inner();

    match settlement_service.get_pool_balance(&license_id).await {
        Ok(summary) => HttpResponse::Ok().json(PoolBalanceResponse {
            license_id: summary.license_id,
            total_pool_micros: summary.total_pool_micros,
            total_pool_dollars: micros_to_dollars(summary.total_pool_micros),
            total_ulo_micros: summary.total_ulo_micros,
            total_uno_allocated_micros: summary.total_uno_allocated_micros,
            total_referral_micros: summary.total_referral_micros,
            total_credit_expenditure_micros: summary.total_credit_expenditure_micros,
            net_uno_contribution_micros: summary.net_uno_contribution_micros,
            net_uno_contribution_dollars: micros_to_dollars(summary.net_uno_contribution_micros),
            below_break_even_threshold: summary.below_break_even_threshold,
            accrued_micros: summary.accrued_micros,
            payable_micros: summary.payable_micros,
            paid_micros: summary.paid_micros,
        }),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/finance/payable
/// Get aggregate payable balances
pub async fn get_payable_balances(
    factory: Option<web::Data<ServiceFactory>>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let settlement_service = match &factory.settlement_service {
        Some(s) => s,
        None => return HttpResponse::NotImplemented().json(serde_json::json!({
            "error": "Settlement service not configured"
        })),
    };

    match settlement_service.get_payable_balances().await {
        Ok(balances) => HttpResponse::Ok().json(PayableBalancesResponse {
            total_ulo_payable_micros: balances.total_ulo_payable_micros,
            total_ulo_payable_dollars: micros_to_dollars(balances.total_ulo_payable_micros),
            total_uno_payable_micros: balances.total_uno_payable_micros,
            total_uno_payable_dollars: micros_to_dollars(balances.total_uno_payable_micros),
            total_referral_payable_micros: balances.total_referral_payable_micros,
            total_referral_payable_dollars: micros_to_dollars(balances.total_referral_payable_micros),
            license_count: balances.license_count,
        }),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/admin/finance/allocations/mark-payable
/// Mark allocations as payable
pub async fn mark_payable(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<MarkPayableRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let settlement_service = match &factory.settlement_service {
        Some(s) => s,
        None => return HttpResponse::NotImplemented().json(serde_json::json!({
            "error": "Settlement service not configured"
        })),
    };

    let actor_id = get_admin_id(&req);

    match settlement_service.mark_payable(&body.ids, &actor_id).await {
        Ok(count) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "updated_count": count
        })),
        Err(e) => error_response(e),
    }
}

// ============================================
// SETTLEMENT HANDLERS
// ============================================

/// POST /api/v1/admin/finance/settlements
/// Prepare a settlement
pub async fn prepare_settlement(
    factory: Option<web::Data<ServiceFactory>>,
    body: web::Json<PrepareSettlementRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let settlement_service = match &factory.settlement_service {
        Some(s) => s,
        None => return HttpResponse::NotImplemented().json(serde_json::json!({
            "error": "Settlement service not configured"
        })),
    };

    let preparer_id = get_admin_id(&req);

    let input = PrepareSettlementInput {
        party_type: body.party_type.clone(),
        allocation_ids: body.allocation_ids.clone(),
        fee_micros: body.fee_micros,
        currency: body.currency.clone(),
        period_start: body.period_start,
        period_end: body.period_end,
        preparer_id,
    };

    match settlement_service.prepare_settlement(input).await {
        Ok(prep) => HttpResponse::Created().json(serde_json::json!({
            "settlement_ref": prep.settlement_ref,
            "total_micros": prep.total_micros,
            "total_dollars": micros_to_dollars(prep.total_micros),
            "fee_micros": prep.fee_micros,
            "net_micros": prep.net_micros,
            "net_dollars": micros_to_dollars(prep.net_micros),
            "allocation_count": prep.allocation_count
        })),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/finance/settlements
/// List settlements
pub async fn list_settlements(
    factory: Option<web::Data<ServiceFactory>>,
    query: web::Query<ListAllocationsQuery>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let settlement_service = match &factory.settlement_service {
        Some(s) => s,
        None => return HttpResponse::NotImplemented().json(serde_json::json!({
            "error": "Settlement service not configured"
        })),
    };

    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    match settlement_service.list_settlements(limit, offset).await {
        Ok(settlements) => {
            let items: Vec<SettlementResponse> = settlements.into_iter().map(|s| {
                SettlementResponse {
                    id: s.id.to_string(),
                    settlement_ref: s.settlement_ref,
                    party_type: s.party_type,
                    total_micros: s.total_micros,
                    total_dollars: micros_to_dollars(s.total_micros),
                    fee_micros: s.fee_micros,
                    net_micros: s.net_micros,
                    net_dollars: micros_to_dollars(s.net_micros),
                    allocation_count: s.allocation_count,
                    state: s.state,
                    prepared_at: s.prepared_at,
                    approved_at: s.approved_at,
                    executed_at: s.executed_at,
                }
            }).collect();

            HttpResponse::Ok().json(serde_json::json!({
                "items": items,
                "count": items.len()
            }))
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/finance/settlements/{ref}
/// Get settlement by reference
pub async fn get_settlement(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let settlement_service = match &factory.settlement_service {
        Some(s) => s,
        None => return HttpResponse::NotImplemented().json(serde_json::json!({
            "error": "Settlement service not configured"
        })),
    };

    let settlement_ref = path.into_inner();

    match settlement_service.get_settlement(&settlement_ref).await {
        Ok(Some(s)) => HttpResponse::Ok().json(SettlementResponse {
            id: s.id.to_string(),
            settlement_ref: s.settlement_ref,
            party_type: s.party_type,
            total_micros: s.total_micros,
            total_dollars: micros_to_dollars(s.total_micros),
            fee_micros: s.fee_micros,
            net_micros: s.net_micros,
            net_dollars: micros_to_dollars(s.net_micros),
            allocation_count: s.allocation_count,
            state: s.state,
            prepared_at: s.prepared_at,
            approved_at: s.approved_at,
            executed_at: s.executed_at,
        }),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Settlement not found"
        })),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/admin/finance/settlements/{ref}/approve
/// Approve a settlement
pub async fn approve_settlement(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let settlement_service = match &factory.settlement_service {
        Some(s) => s,
        None => return HttpResponse::NotImplemented().json(serde_json::json!({
            "error": "Settlement service not configured"
        })),
    };

    let settlement_ref = path.into_inner();
    let approver_id = get_admin_id(&req);

    match settlement_service.approve_settlement(&settlement_ref, &approver_id).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Settlement approved"
        })),
        Ok(false) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Settlement could not be approved (invalid state)"
        })),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/admin/finance/settlements/{ref}/execute
/// Execute an approved settlement
pub async fn execute_settlement(
    factory: Option<web::Data<ServiceFactory>>,
    path: web::Path<String>,
    body: web::Json<ExecuteSettlementRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let factory = match factory {
        Some(f) => f,
        None => return service_unavailable(),
    };

    let settlement_service = match &factory.settlement_service {
        Some(s) => s,
        None => return HttpResponse::NotImplemented().json(serde_json::json!({
            "error": "Settlement service not configured"
        })),
    };

    let settlement_ref = path.into_inner();
    let executor_id = get_admin_id(&req);

    let input = ExecuteSettlementInput {
        settlement_ref: settlement_ref.clone(),
        executor_id,
        provider: body.provider.clone(),
        provider_ref: body.provider_ref.clone(),
    };

    match settlement_service.execute_settlement(input).await {
        Ok(settlement) => HttpResponse::Ok().json(SettlementResponse {
            id: settlement.id.to_string(),
            settlement_ref: settlement.settlement_ref,
            party_type: settlement.party_type,
            total_micros: settlement.total_micros,
            total_dollars: micros_to_dollars(settlement.total_micros),
            fee_micros: settlement.fee_micros,
            net_micros: settlement.net_micros,
            net_dollars: micros_to_dollars(settlement.net_micros),
            allocation_count: settlement.allocation_count,
            state: settlement.state,
            prepared_at: settlement.prepared_at,
            approved_at: settlement.approved_at,
            executed_at: settlement.executed_at,
        }),
        Err(e) => error_response(e),
    }
}
