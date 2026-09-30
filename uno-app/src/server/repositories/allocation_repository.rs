//! Allocation repository for finance ledger operations
//!
//! Implements persistence for revenue allocations with:
//! - Append-only allocation entries with reconciliation verification
//! - Pool balance summaries with break-even tracking
//! - State transitions: accrued -> payable -> paid

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

/// Dynamic type alias for AllocationRepository trait object
pub type DynAllocationRepository = Arc<dyn AllocationRepository + Send + Sync>;

/// Allocation state for payment lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "allocation_state", rename_all = "snake_case")]
pub enum AllocationState {
    /// Allocation recorded but not yet payable
    Accrued,
    /// Allocation approved for payment
    Payable,
    /// Allocation has been paid/settled
    Paid,
}

impl Default for AllocationState {
    fn default() -> Self {
        Self::Accrued
    }
}

/// Allocation ledger entry
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AllocationEntry {
    pub id: Uuid,
    pub license_id: String,
    pub agreement_version: i32,
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
    pub allocated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    // State columns (added in migration 00034)
    #[sqlx(default)]
    pub state: Option<String>,
    #[sqlx(default)]
    pub state_changed_at: Option<DateTime<Utc>>,
    #[sqlx(default)]
    pub state_changed_by: Option<String>,
    #[sqlx(default)]
    pub settlement_ref: Option<String>,
}

/// Input for creating a new allocation entry
#[derive(Debug, Clone)]
pub struct CreateAllocationInput {
    pub license_id: String,
    pub agreement_version: i32,
    pub pool_micros: i64,
    pub pool_currency: String,
    pub ulo_micros: i64,
    pub uno_micros: i64,
    pub referral_micros: i64,
    pub ulo_bps: u32,
    pub uno_bps: u32,
    pub referral_bps: u32,
    pub remainder_micros: i64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub source: String,
    pub external_ref: Option<String>,
}

/// Pool balance summary for a license
#[derive(Debug, Clone)]
pub struct PoolBalanceSummary {
    pub license_id: String,
    pub total_pool_micros: i64,
    pub total_ulo_micros: i64,
    pub total_uno_allocated_micros: i64,
    pub total_referral_micros: i64,
    pub total_credit_expenditure_micros: i64,
    pub net_uno_contribution_micros: i64,
    pub below_break_even_threshold: bool,
    // By state
    pub accrued_micros: i64,
    pub payable_micros: i64,
    pub paid_micros: i64,
}

/// Payable balances by party
#[derive(Debug, Clone)]
pub struct PayableBalances {
    pub total_ulo_payable_micros: i64,
    pub total_uno_payable_micros: i64,
    pub total_referral_payable_micros: i64,
    pub license_count: i64,
}

/// Allocation repository trait
#[async_trait]
pub trait AllocationRepository: Send + Sync {
    /// Create a new allocation entry
    async fn create(&self, input: CreateAllocationInput) -> Result<Uuid, AppError>;

    /// Get allocation by ID
    async fn get(&self, id: Uuid) -> Result<Option<AllocationEntry>, AppError>;

    /// List allocations for a license
    async fn list_by_license(
        &self,
        license_id: &str,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<AllocationEntry>, AppError>;

    /// List allocations by state
    async fn list_by_state(
        &self,
        state: AllocationState,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<AllocationEntry>, AppError>;

    /// Get pool balance summary for a license
    async fn get_pool_balance(&self, license_id: &str) -> Result<PoolBalanceSummary, AppError>;

    /// Get aggregate payable balances
    async fn get_payable_balances(&self) -> Result<PayableBalances, AppError>;

    /// Mark allocations as payable
    async fn mark_payable(&self, ids: &[Uuid], actor_id: &str) -> Result<i64, AppError>;

    /// Mark allocations as paid with settlement reference
    async fn mark_paid(
        &self,
        ids: &[Uuid],
        actor_id: &str,
        settlement_ref: &str,
    ) -> Result<i64, AppError>;

    /// Check for duplicate allocation (by external_ref + license_id)
    async fn exists_by_external_ref(
        &self,
        license_id: &str,
        external_ref: &str,
    ) -> Result<bool, AppError>;
}

/// PostgreSQL implementation of AllocationRepository
pub struct AllocationRepositoryImpl {
    pool: ConnectionPool,
}

impl AllocationRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AllocationRepository for AllocationRepositoryImpl {
    async fn create(&self, input: CreateAllocationInput) -> Result<Uuid, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO allocation_ledger (
                id, license_id, agreement_version,
                pool_micros, pool_currency,
                ulo_micros, uno_micros, referral_micros,
                ulo_bps, uno_bps, referral_bps,
                remainder_micros,
                period_start, period_end,
                source, external_ref,
                allocated_at, created_at
            ) VALUES (
                $1, $2, $3,
                $4, $5,
                $6, $7, $8,
                $9, $10, $11,
                $12,
                $13, $14,
                $15, $16,
                $17, $18
            )
            "#,
        )
        .bind(id)
        .bind(&input.license_id)
        .bind(input.agreement_version)
        .bind(input.pool_micros)
        .bind(&input.pool_currency)
        .bind(input.ulo_micros)
        .bind(input.uno_micros)
        .bind(input.referral_micros)
        .bind(input.ulo_bps as i32)
        .bind(input.uno_bps as i32)
        .bind(input.referral_bps as i32)
        .bind(input.remainder_micros)
        .bind(input.period_start)
        .bind(input.period_end)
        .bind(&input.source)
        .bind(&input.external_ref)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(id)
    }

    async fn get(&self, id: Uuid) -> Result<Option<AllocationEntry>, AppError> {
        sqlx::query_as::<_, AllocationEntry>(
            r#"
            SELECT
                id, license_id, agreement_version,
                pool_micros, pool_currency,
                ulo_micros, uno_micros, referral_micros,
                ulo_bps, uno_bps, referral_bps,
                remainder_micros,
                period_start, period_end,
                source, external_ref,
                allocated_at, created_at,
                state::text as state, state_changed_at, state_changed_by, settlement_ref
            FROM allocation_ledger
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn list_by_license(
        &self,
        license_id: &str,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<AllocationEntry>, AppError> {
        sqlx::query_as::<_, AllocationEntry>(
            r#"
            SELECT
                id, license_id, agreement_version,
                pool_micros, pool_currency,
                ulo_micros, uno_micros, referral_micros,
                ulo_bps, uno_bps, referral_bps,
                remainder_micros,
                period_start, period_end,
                source, external_ref,
                allocated_at, created_at,
                state::text as state, state_changed_at, state_changed_by, settlement_ref
            FROM allocation_ledger
            WHERE license_id = $1
            ORDER BY allocated_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(license_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn list_by_state(
        &self,
        state: AllocationState,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<AllocationEntry>, AppError> {
        let state_str = match state {
            AllocationState::Accrued => "accrued",
            AllocationState::Payable => "payable",
            AllocationState::Paid => "paid",
        };

        sqlx::query_as::<_, AllocationEntry>(
            r#"
            SELECT
                id, license_id, agreement_version,
                pool_micros, pool_currency,
                ulo_micros, uno_micros, referral_micros,
                ulo_bps, uno_bps, referral_bps,
                remainder_micros,
                period_start, period_end,
                source, external_ref,
                allocated_at, created_at,
                state::text as state, state_changed_at, state_changed_by, settlement_ref
            FROM allocation_ledger
            WHERE state = $1::allocation_state
            ORDER BY allocated_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(state_str)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn get_pool_balance(&self, license_id: &str) -> Result<PoolBalanceSummary, AppError> {
        // Use the pool_balance_summary view for base data
        let row = sqlx::query(
            r#"
            SELECT
                license_id,
                total_pool_micros,
                total_ulo_micros,
                total_uno_allocated_micros,
                total_referral_micros,
                total_credit_expenditure_micros,
                net_uno_contribution_micros,
                below_break_even_threshold,
                COALESCE((
                    SELECT SUM(pool_micros)
                    FROM allocation_ledger
                    WHERE license_id = $1 AND (state = 'accrued' OR state IS NULL)
                ), 0) as accrued_micros,
                COALESCE((
                    SELECT SUM(pool_micros)
                    FROM allocation_ledger
                    WHERE license_id = $1 AND state = 'payable'
                ), 0) as payable_micros,
                COALESCE((
                    SELECT SUM(pool_micros)
                    FROM allocation_ledger
                    WHERE license_id = $1 AND state = 'paid'
                ), 0) as paid_micros
            FROM pool_balance_summary
            WHERE license_id = $1
            "#,
        )
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        match row {
            Some(r) => Ok(PoolBalanceSummary {
                license_id: r.get("license_id"),
                total_pool_micros: r.get("total_pool_micros"),
                total_ulo_micros: r.get("total_ulo_micros"),
                total_uno_allocated_micros: r.get("total_uno_allocated_micros"),
                total_referral_micros: r.get("total_referral_micros"),
                total_credit_expenditure_micros: r.get("total_credit_expenditure_micros"),
                net_uno_contribution_micros: r.get("net_uno_contribution_micros"),
                below_break_even_threshold: r.get("below_break_even_threshold"),
                accrued_micros: r.get("accrued_micros"),
                payable_micros: r.get("payable_micros"),
                paid_micros: r.get("paid_micros"),
            }),
            None => Ok(PoolBalanceSummary {
                license_id: license_id.to_string(),
                total_pool_micros: 0,
                total_ulo_micros: 0,
                total_uno_allocated_micros: 0,
                total_referral_micros: 0,
                total_credit_expenditure_micros: 0,
                net_uno_contribution_micros: 0,
                below_break_even_threshold: true,
                accrued_micros: 0,
                payable_micros: 0,
                paid_micros: 0,
            }),
        }
    }

    async fn get_payable_balances(&self) -> Result<PayableBalances, AppError> {
        let row = sqlx::query(
            r#"
            SELECT
                COALESCE(SUM(ulo_micros), 0) as total_ulo_payable_micros,
                COALESCE(SUM(uno_micros), 0) as total_uno_payable_micros,
                COALESCE(SUM(referral_micros), 0) as total_referral_payable_micros,
                COUNT(DISTINCT license_id) as license_count
            FROM allocation_ledger
            WHERE state = 'payable'
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(PayableBalances {
            total_ulo_payable_micros: row.get("total_ulo_payable_micros"),
            total_uno_payable_micros: row.get("total_uno_payable_micros"),
            total_referral_payable_micros: row.get("total_referral_payable_micros"),
            license_count: row.get("license_count"),
        })
    }

    async fn mark_payable(&self, ids: &[Uuid], actor_id: &str) -> Result<i64, AppError> {
        if ids.is_empty() {
            return Ok(0);
        }

        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE allocation_ledger
            SET
                state = 'payable'::allocation_state,
                state_changed_at = $2,
                state_changed_by = $3
            WHERE id = ANY($1) AND (state = 'accrued' OR state IS NULL)
            "#,
        )
        .bind(ids)
        .bind(now)
        .bind(actor_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    async fn mark_paid(
        &self,
        ids: &[Uuid],
        actor_id: &str,
        settlement_ref: &str,
    ) -> Result<i64, AppError> {
        if ids.is_empty() {
            return Ok(0);
        }

        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE allocation_ledger
            SET
                state = 'paid'::allocation_state,
                state_changed_at = $2,
                state_changed_by = $3,
                settlement_ref = $4
            WHERE id = ANY($1) AND state = 'payable'
            "#,
        )
        .bind(ids)
        .bind(now)
        .bind(actor_id)
        .bind(settlement_ref)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    async fn exists_by_external_ref(
        &self,
        license_id: &str,
        external_ref: &str,
    ) -> Result<bool, AppError> {
        let row = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM allocation_ledger
                WHERE license_id = $1 AND external_ref = $2
            ) as exists
            "#,
        )
        .bind(license_id)
        .bind(external_ref)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.get("exists"))
    }
}
