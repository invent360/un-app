//! Credit order repository for UNO-funded credit lifecycle
//!
//! Implements persistence for credit orders with:
//! - Idempotent order creation
//! - State transitions with approval workflow
//! - Settlement record management

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

/// Dynamic type alias for CreditOrderRepository trait object
pub type DynCreditOrderRepository = Arc<dyn CreditOrderRepository + Send + Sync>;

/// Credit order state
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "credit_order_state", rename_all = "snake_case")]
pub enum CreditOrderState {
    Pending,
    Approved,
    Confirmed,
    Failed,
    Unknown,
}

impl Default for CreditOrderState {
    fn default() -> Self {
        Self::Pending
    }
}

impl std::fmt::Display for CreditOrderState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Approved => write!(f, "approved"),
            Self::Confirmed => write!(f, "confirmed"),
            Self::Failed => write!(f, "failed"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Payer type for credit orders
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayerType {
    /// UNO platform-funded credit
    Uno,
    /// License owner user-funded credit
    LicenseOwner,
}

impl PayerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Uno => "uno",
            Self::LicenseOwner => "license_owner",
        }
    }
}

/// Credit order record
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CreditOrder {
    pub id: Uuid,
    pub license_id: String,
    pub amount_micros: i64,
    pub currency: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub payer_type: String,
    #[sqlx(default)]
    pub state: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approval_notes: Option<String>,
    pub provider: Option<String>,
    pub provider_order_id: Option<String>,
    pub idempotency_key: String,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub failure_reason: Option<String>,
    pub retry_count: i32,
    pub last_retry_at: Option<DateTime<Utc>>,
    pub reconciled_at: Option<DateTime<Utc>>,
    pub reconciled_by: Option<String>,
    pub reconciliation_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a credit order
#[derive(Debug, Clone)]
pub struct CreateCreditOrderInput {
    pub license_id: String,
    pub amount_micros: i64,
    pub currency: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub payer_type: PayerType,
    pub idempotency_key: String,
}

/// Input for approving a credit order
#[derive(Debug, Clone)]
pub struct ApproveCreditOrderInput {
    pub order_id: Uuid,
    pub approver_id: String,
    pub notes: Option<String>,
}

/// Input for confirming a credit order
#[derive(Debug, Clone)]
pub struct ConfirmCreditOrderInput {
    pub order_id: Uuid,
    pub provider: String,
    pub provider_order_id: String,
}

/// Settlement record
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Settlement {
    pub id: Uuid,
    pub settlement_ref: String,
    pub party_type: String,
    pub total_micros: i64,
    pub fee_micros: i64,
    pub net_micros: i64,
    pub currency: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub allocation_ids: Vec<Uuid>,
    pub allocation_count: i32,
    pub prepared_by: String,
    pub prepared_at: DateTime<Utc>,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub executed_at: Option<DateTime<Utc>>,
    pub executed_by: Option<String>,
    pub provider: Option<String>,
    pub provider_ref: Option<String>,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a settlement
#[derive(Debug, Clone)]
pub struct CreateSettlementInput {
    pub settlement_ref: String,
    pub party_type: String,
    pub total_micros: i64,
    pub fee_micros: i64,
    pub currency: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub allocation_ids: Vec<Uuid>,
    pub prepared_by: String,
}

/// Credit order repository trait
#[async_trait]
pub trait CreditOrderRepository: Send + Sync {
    // Credit order operations
    async fn create_order(&self, input: CreateCreditOrderInput) -> Result<Uuid, AppError>;
    async fn get_order(&self, id: Uuid) -> Result<Option<CreditOrder>, AppError>;
    async fn get_order_by_idempotency_key(&self, key: &str) -> Result<Option<CreditOrder>, AppError>;
    async fn list_orders_by_license(&self, license_id: &str, limit: i32, offset: i32) -> Result<Vec<CreditOrder>, AppError>;
    async fn list_orders_by_state(&self, state: CreditOrderState, limit: i32, offset: i32) -> Result<Vec<CreditOrder>, AppError>;
    async fn approve_order(&self, input: ApproveCreditOrderInput) -> Result<bool, AppError>;
    async fn confirm_order(&self, input: ConfirmCreditOrderInput) -> Result<bool, AppError>;
    async fn fail_order(&self, order_id: Uuid, reason: &str) -> Result<bool, AppError>;
    async fn mark_unknown(&self, order_id: Uuid) -> Result<bool, AppError>;
    async fn reconcile_order(&self, order_id: Uuid, actor_id: &str, notes: &str) -> Result<bool, AppError>;

    // Settlement operations
    async fn create_settlement(&self, input: CreateSettlementInput) -> Result<Uuid, AppError>;
    async fn get_settlement(&self, id: Uuid) -> Result<Option<Settlement>, AppError>;
    async fn get_settlement_by_ref(&self, settlement_ref: &str) -> Result<Option<Settlement>, AppError>;
    async fn list_settlements(&self, limit: i32, offset: i32) -> Result<Vec<Settlement>, AppError>;
    async fn approve_settlement(&self, id: Uuid, approver_id: &str) -> Result<bool, AppError>;
    async fn execute_settlement(&self, id: Uuid, executor_id: &str, provider: &str, provider_ref: &str) -> Result<bool, AppError>;
}

/// PostgreSQL implementation of CreditOrderRepository
pub struct CreditOrderRepositoryImpl {
    pool: ConnectionPool,
}

impl CreditOrderRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CreditOrderRepository for CreditOrderRepositoryImpl {
    async fn create_order(&self, input: CreateCreditOrderInput) -> Result<Uuid, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO credit_orders (
                id, license_id, amount_micros, currency,
                period_start, period_end, payer_type,
                idempotency_key, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (idempotency_key) DO NOTHING
            "#,
        )
        .bind(id)
        .bind(&input.license_id)
        .bind(input.amount_micros)
        .bind(&input.currency)
        .bind(input.period_start)
        .bind(input.period_end)
        .bind(input.payer_type.as_str())
        .bind(&input.idempotency_key)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // Check if this was a duplicate
        if let Some(existing) = self.get_order_by_idempotency_key(&input.idempotency_key).await? {
            return Ok(existing.id);
        }

        Ok(id)
    }

    async fn get_order(&self, id: Uuid) -> Result<Option<CreditOrder>, AppError> {
        sqlx::query_as::<_, CreditOrder>(
            r#"
            SELECT
                id, license_id, amount_micros, currency,
                period_start, period_end, payer_type,
                state::text as state, approved_by, approved_at, approval_notes,
                provider, provider_order_id, idempotency_key,
                confirmed_at, failed_at, failure_reason,
                retry_count, last_retry_at,
                reconciled_at, reconciled_by, reconciliation_notes,
                created_at, updated_at
            FROM credit_orders
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn get_order_by_idempotency_key(&self, key: &str) -> Result<Option<CreditOrder>, AppError> {
        sqlx::query_as::<_, CreditOrder>(
            r#"
            SELECT
                id, license_id, amount_micros, currency,
                period_start, period_end, payer_type,
                state::text as state, approved_by, approved_at, approval_notes,
                provider, provider_order_id, idempotency_key,
                confirmed_at, failed_at, failure_reason,
                retry_count, last_retry_at,
                reconciled_at, reconciled_by, reconciliation_notes,
                created_at, updated_at
            FROM credit_orders
            WHERE idempotency_key = $1
            "#,
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn list_orders_by_license(
        &self,
        license_id: &str,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<CreditOrder>, AppError> {
        sqlx::query_as::<_, CreditOrder>(
            r#"
            SELECT
                id, license_id, amount_micros, currency,
                period_start, period_end, payer_type,
                state::text as state, approved_by, approved_at, approval_notes,
                provider, provider_order_id, idempotency_key,
                confirmed_at, failed_at, failure_reason,
                retry_count, last_retry_at,
                reconciled_at, reconciled_by, reconciliation_notes,
                created_at, updated_at
            FROM credit_orders
            WHERE license_id = $1
            ORDER BY created_at DESC
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

    async fn list_orders_by_state(
        &self,
        state: CreditOrderState,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<CreditOrder>, AppError> {
        sqlx::query_as::<_, CreditOrder>(
            r#"
            SELECT
                id, license_id, amount_micros, currency,
                period_start, period_end, payer_type,
                state::text as state, approved_by, approved_at, approval_notes,
                provider, provider_order_id, idempotency_key,
                confirmed_at, failed_at, failure_reason,
                retry_count, last_retry_at,
                reconciled_at, reconciled_by, reconciliation_notes,
                created_at, updated_at
            FROM credit_orders
            WHERE state = $1::credit_order_state
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(state.to_string())
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn approve_order(&self, input: ApproveCreditOrderInput) -> Result<bool, AppError> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE credit_orders
            SET
                state = 'approved'::credit_order_state,
                approved_by = $2,
                approved_at = $3,
                approval_notes = $4,
                updated_at = $3
            WHERE id = $1 AND state = 'pending'
            "#,
        )
        .bind(input.order_id)
        .bind(&input.approver_id)
        .bind(now)
        .bind(&input.notes)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn confirm_order(&self, input: ConfirmCreditOrderInput) -> Result<bool, AppError> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE credit_orders
            SET
                state = 'confirmed'::credit_order_state,
                provider = $2,
                provider_order_id = $3,
                confirmed_at = $4,
                updated_at = $4
            WHERE id = $1 AND state = 'approved'
            "#,
        )
        .bind(input.order_id)
        .bind(&input.provider)
        .bind(&input.provider_order_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn fail_order(&self, order_id: Uuid, reason: &str) -> Result<bool, AppError> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE credit_orders
            SET
                state = 'failed'::credit_order_state,
                failed_at = $2,
                failure_reason = $3,
                retry_count = retry_count + 1,
                last_retry_at = $2,
                updated_at = $2
            WHERE id = $1 AND state IN ('approved', 'unknown')
            "#,
        )
        .bind(order_id)
        .bind(now)
        .bind(reason)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn mark_unknown(&self, order_id: Uuid) -> Result<bool, AppError> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE credit_orders
            SET
                state = 'unknown'::credit_order_state,
                retry_count = retry_count + 1,
                last_retry_at = $2,
                updated_at = $2
            WHERE id = $1 AND state = 'approved'
            "#,
        )
        .bind(order_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn reconcile_order(&self, order_id: Uuid, actor_id: &str, notes: &str) -> Result<bool, AppError> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE credit_orders
            SET
                state = 'confirmed'::credit_order_state,
                reconciled_at = $2,
                reconciled_by = $3,
                reconciliation_notes = $4,
                updated_at = $2
            WHERE id = $1 AND state = 'unknown'
            "#,
        )
        .bind(order_id)
        .bind(now)
        .bind(actor_id)
        .bind(notes)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn create_settlement(&self, input: CreateSettlementInput) -> Result<Uuid, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let net_micros = input.total_micros - input.fee_micros;
        let allocation_count = input.allocation_ids.len() as i32;

        sqlx::query(
            r#"
            INSERT INTO settlements (
                id, settlement_ref, party_type,
                total_micros, fee_micros, net_micros, currency,
                period_start, period_end,
                allocation_ids, allocation_count,
                prepared_by, prepared_at,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3,
                $4, $5, $6, $7,
                $8, $9,
                $10, $11,
                $12, $13,
                $14, $15
            )
            "#,
        )
        .bind(id)
        .bind(&input.settlement_ref)
        .bind(&input.party_type)
        .bind(input.total_micros)
        .bind(input.fee_micros)
        .bind(net_micros)
        .bind(&input.currency)
        .bind(input.period_start)
        .bind(input.period_end)
        .bind(&input.allocation_ids)
        .bind(allocation_count)
        .bind(&input.prepared_by)
        .bind(now)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(id)
    }

    async fn get_settlement(&self, id: Uuid) -> Result<Option<Settlement>, AppError> {
        sqlx::query_as::<_, Settlement>(
            r#"
            SELECT
                id, settlement_ref, party_type,
                total_micros, fee_micros, net_micros, currency,
                period_start, period_end,
                allocation_ids, allocation_count,
                prepared_by, prepared_at,
                approved_by, approved_at,
                executed_at, executed_by,
                provider, provider_ref,
                state, created_at, updated_at
            FROM settlements
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn get_settlement_by_ref(&self, settlement_ref: &str) -> Result<Option<Settlement>, AppError> {
        sqlx::query_as::<_, Settlement>(
            r#"
            SELECT
                id, settlement_ref, party_type,
                total_micros, fee_micros, net_micros, currency,
                period_start, period_end,
                allocation_ids, allocation_count,
                prepared_by, prepared_at,
                approved_by, approved_at,
                executed_at, executed_by,
                provider, provider_ref,
                state, created_at, updated_at
            FROM settlements
            WHERE settlement_ref = $1
            "#,
        )
        .bind(settlement_ref)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn list_settlements(&self, limit: i32, offset: i32) -> Result<Vec<Settlement>, AppError> {
        sqlx::query_as::<_, Settlement>(
            r#"
            SELECT
                id, settlement_ref, party_type,
                total_micros, fee_micros, net_micros, currency,
                period_start, period_end,
                allocation_ids, allocation_count,
                prepared_by, prepared_at,
                approved_by, approved_at,
                executed_at, executed_by,
                provider, provider_ref,
                state, created_at, updated_at
            FROM settlements
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

    async fn approve_settlement(&self, id: Uuid, approver_id: &str) -> Result<bool, AppError> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE settlements
            SET
                state = 'approved',
                approved_by = $2,
                approved_at = $3,
                updated_at = $3
            WHERE id = $1 AND state = 'prepared'
            "#,
        )
        .bind(id)
        .bind(approver_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn execute_settlement(
        &self,
        id: Uuid,
        executor_id: &str,
        provider: &str,
        provider_ref: &str,
    ) -> Result<bool, AppError> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE settlements
            SET
                state = 'executed',
                executed_by = $2,
                executed_at = $3,
                provider = $4,
                provider_ref = $5,
                updated_at = $3
            WHERE id = $1 AND state = 'approved'
            "#,
        )
        .bind(id)
        .bind(executor_id)
        .bind(now)
        .bind(provider)
        .bind(provider_ref)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}
