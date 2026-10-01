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

/// R5-07: Quarantine status for manual allocation review
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "allocation_quarantine_status", rename_all = "snake_case")]
pub enum QuarantineStatus {
    /// Normal allocation (automated or validated)
    None,
    /// Requires manual review
    PendingReview,
    /// Flagged as problematic
    Quarantined,
    /// Issue resolved
    Resolved,
    /// Rejected as invalid
    Rejected,
}

impl Default for QuarantineStatus {
    fn default() -> Self {
        Self::None
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
    /// R3-07: Reserve allocation for no-referral cases
    #[sqlx(default)]
    pub reserve_micros: i64,
    pub ulo_bps: i32,
    pub uno_bps: i32,
    pub referral_bps: i32,
    pub remainder_micros: i64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub source: String,
    pub external_ref: Option<String>,
    /// R3-08: Provider identifier for deduplication
    #[sqlx(default)]
    pub provider_id: Option<String>,
    /// R3-08: Unique event ID from provider
    #[sqlx(default)]
    pub reward_event_id: Option<String>,
    /// R4-04: Referral agent receiving referral share
    #[sqlx(default)]
    pub referral_agent_id: Option<Uuid>,
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
    // R5-07: Quarantine tracking
    #[sqlx(default)]
    pub quarantine_status: Option<String>,
    #[sqlx(default)]
    pub quarantine_reason: Option<String>,
    #[sqlx(default)]
    pub quarantine_reviewed_by: Option<String>,
    #[sqlx(default)]
    pub quarantine_reviewed_at: Option<DateTime<Utc>>,
    // R5-07: Reward source tracking
    #[sqlx(default)]
    pub reward_source: Option<String>,
    #[sqlx(default)]
    pub reward_batch_id: Option<Uuid>,
    #[sqlx(default)]
    pub is_batch_member: bool,
    // R5-07: Agreement snapshot
    #[sqlx(default)]
    pub agreement_snapshot: Option<serde_json::Value>,
    // R5-07: Recipient tracking
    #[sqlx(default)]
    pub ulo_recipient_id: Option<Uuid>,
    #[sqlx(default)]
    pub uno_recipient_id: Option<Uuid>,
    #[sqlx(default)]
    pub reserve_recipient_id: Option<Uuid>,
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
    /// R3-07: Reserve allocation for no-referral cases
    pub reserve_micros: i64,
    pub ulo_bps: u32,
    pub uno_bps: u32,
    pub referral_bps: u32,
    pub remainder_micros: i64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub source: String,
    pub external_ref: Option<String>,
    /// R3-08: Provider identifier for deduplication (e.g., "unetwork", "marketplace")
    pub provider_id: Option<String>,
    /// R3-08: Unique event ID from provider for duplicate prevention
    pub reward_event_id: Option<String>,
    /// R3-07: Agent receiving referral share (if any)
    pub referral_agent_id: Option<Uuid>,
    // R5-07: Reward source tracking
    /// Source type: automatic, manual, compensating, batch
    pub reward_source: Option<String>,
    /// Groups allocations from single event covering multiple licenses
    pub reward_batch_id: Option<Uuid>,
    /// True if allocation is part of a batch reward event
    pub is_batch_member: bool,
    // R5-07: Agreement snapshot
    /// Snapshot of agreement terms at allocation time for audit
    pub agreement_snapshot: Option<serde_json::Value>,
    // R5-07: Recipient tracking
    /// User/participant receiving ULO share
    pub ulo_recipient_id: Option<Uuid>,
    /// UNO operator entity (usually system)
    pub uno_recipient_id: Option<Uuid>,
    /// Reserve pool recipient (for no-referral allocations)
    pub reserve_recipient_id: Option<Uuid>,
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

    /// R3-08: Check for duplicate allocation by provider and event ID
    /// This is the authoritative deduplication check for reward events
    async fn exists_by_provider_event(
        &self,
        provider_id: &str,
        reward_event_id: &str,
    ) -> Result<bool, AppError>;

    /// R5-07: List allocations pending quarantine review
    async fn list_quarantined(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<AllocationEntry>, AppError>;

    /// R5-07: Mark allocation for quarantine review
    async fn quarantine(
        &self,
        id: Uuid,
        reason: &str,
        reviewer_id: Option<&str>,
    ) -> Result<(), AppError>;

    /// R5-07: Resolve quarantined allocation
    async fn resolve_quarantine(
        &self,
        id: Uuid,
        resolution: &str,
        reviewer_id: &str,
        notes: Option<&str>,
    ) -> Result<(), AppError>;

    /// R5-07: Get alert policy by code
    async fn get_alert_policy(&self, policy_code: &str) -> Result<Option<AlertPolicyRow>, AppError>;

    /// R5-07: List all active alert policies
    async fn list_alert_policies(&self) -> Result<Vec<AlertPolicyRow>, AppError>;
}

/// R5-07: Alert policy row from database
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AlertPolicyRow {
    pub id: i32,
    pub policy_code: String,
    pub policy_name: String,
    pub description: Option<String>,
    pub parameters: serde_json::Value,
    pub threshold_micros: Option<i64>,
    pub threshold_percent: Option<f64>,
    pub period_type: Option<String>,
    pub period_value: Option<i32>,
    pub basis: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
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
                ulo_micros, uno_micros, referral_micros, reserve_micros,
                ulo_bps, uno_bps, referral_bps,
                remainder_micros,
                period_start, period_end,
                source, external_ref,
                provider_id, reward_event_id, referral_agent_id,
                allocated_at, created_at,
                reward_source, reward_batch_id, is_batch_member,
                agreement_snapshot,
                ulo_recipient_id, uno_recipient_id, reserve_recipient_id
            ) VALUES (
                $1, $2, $3,
                $4, $5,
                $6, $7, $8, $9,
                $10, $11, $12,
                $13,
                $14, $15,
                $16, $17,
                $18, $19, $20,
                $21, $22,
                $23, $24, $25,
                $26,
                $27, $28, $29
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
        .bind(input.reserve_micros)
        .bind(input.ulo_bps as i32)
        .bind(input.uno_bps as i32)
        .bind(input.referral_bps as i32)
        .bind(input.remainder_micros)
        .bind(input.period_start)
        .bind(input.period_end)
        .bind(&input.source)
        .bind(&input.external_ref)
        .bind(&input.provider_id)
        .bind(&input.reward_event_id)
        .bind(input.referral_agent_id)
        .bind(now)
        .bind(now)
        // R5-07 columns
        .bind(&input.reward_source)
        .bind(input.reward_batch_id)
        .bind(input.is_batch_member)
        .bind(&input.agreement_snapshot)
        .bind(input.ulo_recipient_id)
        .bind(input.uno_recipient_id)
        .bind(input.reserve_recipient_id)
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
                COALESCE(reserve_micros, 0) as reserve_micros,
                ulo_bps, uno_bps, referral_bps,
                remainder_micros,
                period_start, period_end,
                source, external_ref,
                provider_id, reward_event_id, referral_agent_id,
                allocated_at, created_at,
                state::text as state, state_changed_at, state_changed_by, settlement_ref,
                quarantine_status::text as quarantine_status, quarantine_reason,
                quarantine_reviewed_by, quarantine_reviewed_at,
                reward_source, reward_batch_id, COALESCE(is_batch_member, false) as is_batch_member,
                agreement_snapshot,
                ulo_recipient_id, uno_recipient_id, reserve_recipient_id
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
                COALESCE(reserve_micros, 0) as reserve_micros,
                ulo_bps, uno_bps, referral_bps,
                remainder_micros,
                period_start, period_end,
                source, external_ref,
                provider_id, reward_event_id, referral_agent_id,
                allocated_at, created_at,
                state::text as state, state_changed_at, state_changed_by, settlement_ref,
                quarantine_status::text as quarantine_status, quarantine_reason,
                quarantine_reviewed_by, quarantine_reviewed_at,
                reward_source, reward_batch_id, COALESCE(is_batch_member, false) as is_batch_member,
                agreement_snapshot,
                ulo_recipient_id, uno_recipient_id, reserve_recipient_id
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
                COALESCE(reserve_micros, 0) as reserve_micros,
                ulo_bps, uno_bps, referral_bps,
                remainder_micros,
                period_start, period_end,
                source, external_ref,
                provider_id, reward_event_id, referral_agent_id,
                allocated_at, created_at,
                state::text as state, state_changed_at, state_changed_by, settlement_ref,
                quarantine_status::text as quarantine_status, quarantine_reason,
                quarantine_reviewed_by, quarantine_reviewed_at,
                reward_source, reward_batch_id, COALESCE(is_batch_member, false) as is_batch_member,
                agreement_snapshot,
                ulo_recipient_id, uno_recipient_id, reserve_recipient_id
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

    /// R3-08: Check for duplicate allocation by provider and event ID
    /// This provides provider-scoped deduplication, preventing the same
    /// reward event from being processed twice regardless of license
    async fn exists_by_provider_event(
        &self,
        provider_id: &str,
        reward_event_id: &str,
    ) -> Result<bool, AppError> {
        let row = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM allocation_ledger
                WHERE provider_id = $1 AND reward_event_id = $2
            ) as exists
            "#,
        )
        .bind(provider_id)
        .bind(reward_event_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.get("exists"))
    }

    /// R5-07: List allocations pending quarantine review
    async fn list_quarantined(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<AllocationEntry>, AppError> {
        sqlx::query_as::<_, AllocationEntry>(
            r#"
            SELECT
                id, license_id, agreement_version,
                pool_micros, pool_currency,
                ulo_micros, uno_micros, referral_micros,
                COALESCE(reserve_micros, 0) as reserve_micros,
                ulo_bps, uno_bps, referral_bps,
                remainder_micros,
                period_start, period_end,
                source, external_ref,
                provider_id, reward_event_id, referral_agent_id,
                allocated_at, created_at,
                state::text as state, state_changed_at, state_changed_by, settlement_ref,
                quarantine_status::text as quarantine_status, quarantine_reason,
                quarantine_reviewed_by, quarantine_reviewed_at,
                reward_source, reward_batch_id, COALESCE(is_batch_member, false) as is_batch_member,
                agreement_snapshot,
                ulo_recipient_id, uno_recipient_id, reserve_recipient_id
            FROM allocation_ledger
            WHERE quarantine_status IN ('pending_review', 'quarantined')
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    /// R5-07: Mark allocation for quarantine review
    async fn quarantine(
        &self,
        id: Uuid,
        reason: &str,
        reviewer_id: Option<&str>,
    ) -> Result<(), AppError> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE allocation_ledger
            SET quarantine_status = 'pending_review'::allocation_quarantine_status,
                quarantine_reason = $2,
                quarantine_reviewed_by = $3,
                updated_at = $4
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(reason)
        .bind(reviewer_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    /// R5-07: Resolve quarantined allocation
    async fn resolve_quarantine(
        &self,
        id: Uuid,
        resolution: &str,
        reviewer_id: &str,
        notes: Option<&str>,
    ) -> Result<(), AppError> {
        // Validate resolution value
        if resolution != "resolved" && resolution != "rejected" {
            return Err(AppError::ValidationError(
                "Resolution must be 'resolved' or 'rejected'".to_string(),
            ));
        }

        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE allocation_ledger
            SET quarantine_status = $2::allocation_quarantine_status,
                quarantine_reviewed_by = $3,
                quarantine_reviewed_at = $4,
                quarantine_resolution_notes = $5,
                updated_at = $4
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(resolution)
        .bind(reviewer_id)
        .bind(now)
        .bind(notes)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    /// R5-07: Get alert policy by code
    async fn get_alert_policy(&self, policy_code: &str) -> Result<Option<AlertPolicyRow>, AppError> {
        sqlx::query_as::<_, AlertPolicyRow>(
            r#"
            SELECT
                id, policy_code, policy_name, description,
                parameters, threshold_micros, threshold_percent,
                period_type, period_value, basis,
                is_active, created_at, updated_at, version
            FROM alert_policies
            WHERE policy_code = $1 AND is_active = true
            "#,
        )
        .bind(policy_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    /// R5-07: List all active alert policies
    async fn list_alert_policies(&self) -> Result<Vec<AlertPolicyRow>, AppError> {
        sqlx::query_as::<_, AlertPolicyRow>(
            r#"
            SELECT
                id, policy_code, policy_name, description,
                parameters, threshold_micros, threshold_percent,
                period_type, period_value, basis,
                is_active, created_at, updated_at, version
            FROM alert_policies
            WHERE is_active = true
            ORDER BY policy_code
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }
}
