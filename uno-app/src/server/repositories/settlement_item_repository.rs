//! Settlement item repository for R4-04 per-recipient/per-party settlement
//!
//! Provides persistence for immutable settlement items with:
//! - Unique constraint preventing duplicate settlements of same liability
//! - Per-item state tracking (submitted, confirmed, failed, reconciled)
//! - Atomic creation within settlement preparation transaction

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

/// Dynamic type alias for SettlementItemRepository trait object
pub type DynSettlementItemRepository = Arc<dyn SettlementItemRepository + Send + Sync>;

/// Settlement item state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementItemState {
    /// Included in settlement, awaiting confirmation
    Submitted,
    /// Provider confirmed payment
    Confirmed,
    /// Provider rejected payment
    Failed,
    /// Resolved after timeout
    Reconciled,
}

impl Default for SettlementItemState {
    fn default() -> Self {
        Self::Submitted
    }
}

impl std::fmt::Display for SettlementItemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Submitted => write!(f, "submitted"),
            Self::Confirmed => write!(f, "confirmed"),
            Self::Failed => write!(f, "failed"),
            Self::Reconciled => write!(f, "reconciled"),
        }
    }
}

impl std::str::FromStr for SettlementItemState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "submitted" => Ok(Self::Submitted),
            "confirmed" => Ok(Self::Confirmed),
            "failed" => Ok(Self::Failed),
            "reconciled" => Ok(Self::Reconciled),
            _ => Err(format!("Invalid settlement item state: {}", s)),
        }
    }
}

/// Settlement item entry
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SettlementItem {
    pub id: Uuid,
    pub settlement_id: Uuid,
    pub allocation_id: Uuid,
    pub party_type: String,
    pub recipient_id: Option<String>,
    pub recipient_type: Option<String>,
    pub amount_micros: i64,
    pub currency: String,
    pub item_state: String,
    pub created_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
}

/// Input for creating a settlement item
#[derive(Debug, Clone)]
pub struct CreateSettlementItemInput {
    pub settlement_id: Uuid,
    pub allocation_id: Uuid,
    pub party_type: String,
    pub recipient_id: Option<String>,
    pub recipient_type: Option<String>,
    pub amount_micros: i64,
    pub currency: String,
}

/// Summary of unsettled liabilities for an allocation
#[derive(Debug, Clone)]
pub struct UnsettledLiability {
    pub allocation_id: Uuid,
    pub license_id: String,
    pub referral_agent_id: Option<Uuid>,
    pub currency: String,
    pub ulo_unsettled_micros: i64,
    pub ulo_settlement_state: String,
    pub uno_unsettled_micros: i64,
    pub uno_settlement_state: String,
    pub referral_unsettled_micros: i64,
    pub referral_settlement_state: String,
    pub reserve_unsettled_micros: i64,
    pub reserve_settlement_state: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub allocated_at: DateTime<Utc>,
}

/// Settlement item repository trait
#[async_trait]
pub trait SettlementItemRepository: Send + Sync {
    /// Create a settlement item (called within transaction)
    async fn create(&self, input: CreateSettlementItemInput) -> Result<Uuid, AppError>;

    /// Create multiple settlement items in batch
    async fn create_batch(&self, inputs: Vec<CreateSettlementItemInput>) -> Result<Vec<Uuid>, AppError>;

    /// Get settlement item by ID
    async fn get(&self, id: Uuid) -> Result<Option<SettlementItem>, AppError>;

    /// List items for a settlement
    async fn list_by_settlement(&self, settlement_id: Uuid) -> Result<Vec<SettlementItem>, AppError>;

    /// List items for an allocation
    async fn list_by_allocation(&self, allocation_id: Uuid) -> Result<Vec<SettlementItem>, AppError>;

    /// Check if allocation+party is already in a pending settlement
    async fn is_pending(
        &self,
        allocation_id: Uuid,
        party_type: &str,
    ) -> Result<bool, AppError>;

    /// Mark items as confirmed
    async fn mark_confirmed(&self, ids: &[Uuid]) -> Result<i64, AppError>;

    /// Mark items as failed
    async fn mark_failed(&self, ids: &[Uuid]) -> Result<i64, AppError>;

    /// Mark items as reconciled
    async fn mark_reconciled(&self, ids: &[Uuid]) -> Result<i64, AppError>;

    /// Get unsettled liabilities for a party type
    async fn get_unsettled_liabilities(
        &self,
        party_type: &str,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<UnsettledLiability>, AppError>;

    /// Update allocation per-party settlement state
    async fn update_allocation_settlement_state(
        &self,
        allocation_id: Uuid,
        party_type: &str,
        state: &str,
    ) -> Result<(), AppError>;
}

/// PostgreSQL implementation of SettlementItemRepository
pub struct SettlementItemRepositoryImpl {
    pool: ConnectionPool,
}

impl SettlementItemRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettlementItemRepository for SettlementItemRepositoryImpl {
    async fn create(&self, input: CreateSettlementItemInput) -> Result<Uuid, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO settlement_items (
                id, settlement_id, allocation_id,
                party_type, recipient_id, recipient_type,
                amount_micros, currency,
                item_state, created_at
            ) VALUES (
                $1, $2, $3,
                $4, $5, $6,
                $7, $8,
                'submitted', $9
            )
            "#,
        )
        .bind(id)
        .bind(input.settlement_id)
        .bind(input.allocation_id)
        .bind(&input.party_type)
        .bind(&input.recipient_id)
        .bind(&input.recipient_type)
        .bind(input.amount_micros)
        .bind(&input.currency)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            // Check for unique constraint violation
            if e.to_string().contains("duplicate key") || e.to_string().contains("unique constraint") {
                AppError::Conflict(format!(
                    "Allocation {} party {} already in a pending settlement",
                    input.allocation_id, input.party_type
                ))
            } else {
                AppError::Database(e.to_string())
            }
        })?;

        Ok(id)
    }

    async fn create_batch(&self, inputs: Vec<CreateSettlementItemInput>) -> Result<Vec<Uuid>, AppError> {
        if inputs.is_empty() {
            return Ok(vec![]);
        }

        let mut ids = Vec::with_capacity(inputs.len());
        let now = Utc::now();

        // Use a transaction for batch insert
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::Database(e.to_string()))?;

        for input in inputs {
            let id = Uuid::new_v4();

            sqlx::query(
                r#"
                INSERT INTO settlement_items (
                    id, settlement_id, allocation_id,
                    party_type, recipient_id, recipient_type,
                    amount_micros, currency,
                    item_state, created_at
                ) VALUES (
                    $1, $2, $3,
                    $4, $5, $6,
                    $7, $8,
                    'submitted', $9
                )
                "#,
            )
            .bind(id)
            .bind(input.settlement_id)
            .bind(input.allocation_id)
            .bind(&input.party_type)
            .bind(&input.recipient_id)
            .bind(&input.recipient_type)
            .bind(input.amount_micros)
            .bind(&input.currency)
            .bind(now)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                if e.to_string().contains("duplicate key") || e.to_string().contains("unique constraint") {
                    AppError::Conflict(format!(
                        "Allocation {} party {} already in a pending settlement",
                        input.allocation_id, input.party_type
                    ))
                } else {
                    AppError::Database(e.to_string())
                }
            })?;

            ids.push(id);
        }

        tx.commit().await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(ids)
    }

    async fn get(&self, id: Uuid) -> Result<Option<SettlementItem>, AppError> {
        sqlx::query_as::<_, SettlementItem>(
            r#"
            SELECT
                id, settlement_id, allocation_id,
                party_type, recipient_id, recipient_type,
                amount_micros, currency,
                item_state, created_at, confirmed_at
            FROM settlement_items
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn list_by_settlement(&self, settlement_id: Uuid) -> Result<Vec<SettlementItem>, AppError> {
        sqlx::query_as::<_, SettlementItem>(
            r#"
            SELECT
                id, settlement_id, allocation_id,
                party_type, recipient_id, recipient_type,
                amount_micros, currency,
                item_state, created_at, confirmed_at
            FROM settlement_items
            WHERE settlement_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(settlement_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn list_by_allocation(&self, allocation_id: Uuid) -> Result<Vec<SettlementItem>, AppError> {
        sqlx::query_as::<_, SettlementItem>(
            r#"
            SELECT
                id, settlement_id, allocation_id,
                party_type, recipient_id, recipient_type,
                amount_micros, currency,
                item_state, created_at, confirmed_at
            FROM settlement_items
            WHERE allocation_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(allocation_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn is_pending(
        &self,
        allocation_id: Uuid,
        party_type: &str,
    ) -> Result<bool, AppError> {
        let row = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM settlement_items
                WHERE allocation_id = $1
                  AND party_type = $2
                  AND item_state IN ('submitted', 'confirmed')
            ) as exists
            "#,
        )
        .bind(allocation_id)
        .bind(party_type)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.get("exists"))
    }

    async fn mark_confirmed(&self, ids: &[Uuid]) -> Result<i64, AppError> {
        if ids.is_empty() {
            return Ok(0);
        }

        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE settlement_items
            SET item_state = 'confirmed', confirmed_at = $2
            WHERE id = ANY($1) AND item_state = 'submitted'
            "#,
        )
        .bind(ids)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    async fn mark_failed(&self, ids: &[Uuid]) -> Result<i64, AppError> {
        if ids.is_empty() {
            return Ok(0);
        }

        let result = sqlx::query(
            r#"
            UPDATE settlement_items
            SET item_state = 'failed'
            WHERE id = ANY($1) AND item_state = 'submitted'
            "#,
        )
        .bind(ids)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    async fn mark_reconciled(&self, ids: &[Uuid]) -> Result<i64, AppError> {
        if ids.is_empty() {
            return Ok(0);
        }

        let result = sqlx::query(
            r#"
            UPDATE settlement_items
            SET item_state = 'reconciled'
            WHERE id = ANY($1) AND item_state IN ('submitted', 'failed')
            "#,
        )
        .bind(ids)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    async fn get_unsettled_liabilities(
        &self,
        party_type: &str,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<UnsettledLiability>, AppError> {
        // Use the unsettled_liabilities view with party filter
        let rows = sqlx::query(
            r#"
            SELECT
                allocation_id,
                license_id,
                referral_agent_id,
                currency,
                ulo_unsettled_micros,
                ulo_settlement_state,
                uno_unsettled_micros,
                uno_settlement_state,
                referral_unsettled_micros,
                referral_settlement_state,
                reserve_unsettled_micros,
                reserve_settlement_state,
                period_start,
                period_end,
                allocated_at
            FROM unsettled_liabilities
            WHERE
                CASE $1
                    WHEN 'ulo' THEN ulo_unsettled_micros > 0
                    WHEN 'uno' THEN uno_unsettled_micros > 0
                    WHEN 'referral' THEN referral_unsettled_micros > 0
                    WHEN 'reserve' THEN reserve_unsettled_micros > 0
                    ELSE false
                END
            ORDER BY allocated_at ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(party_type)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let mut liabilities = Vec::with_capacity(rows.len());
        for row in rows {
            liabilities.push(UnsettledLiability {
                allocation_id: row.get("allocation_id"),
                license_id: row.get("license_id"),
                referral_agent_id: row.get("referral_agent_id"),
                currency: row.get("currency"),
                ulo_unsettled_micros: row.get("ulo_unsettled_micros"),
                ulo_settlement_state: row.get("ulo_settlement_state"),
                uno_unsettled_micros: row.get("uno_unsettled_micros"),
                uno_settlement_state: row.get("uno_settlement_state"),
                referral_unsettled_micros: row.get("referral_unsettled_micros"),
                referral_settlement_state: row.get("referral_settlement_state"),
                reserve_unsettled_micros: row.get("reserve_unsettled_micros"),
                reserve_settlement_state: row.get("reserve_settlement_state"),
                period_start: row.get("period_start"),
                period_end: row.get("period_end"),
                allocated_at: row.get("allocated_at"),
            });
        }

        Ok(liabilities)
    }

    async fn update_allocation_settlement_state(
        &self,
        allocation_id: Uuid,
        party_type: &str,
        state: &str,
    ) -> Result<(), AppError> {
        let column = match party_type {
            "ulo" => "ulo_settlement_state",
            "uno" => "uno_settlement_state",
            "referral" => "referral_settlement_state",
            "reserve" => "reserve_settlement_state",
            _ => return Err(AppError::ValidationError(format!("Invalid party type: {}", party_type))),
        };

        // Use dynamic SQL for column name
        let query = format!(
            "UPDATE allocation_ledger SET {} = $1 WHERE id = $2",
            column
        );

        sqlx::query(&query)
            .bind(state)
            .bind(allocation_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settlement_item_state_display() {
        assert_eq!(SettlementItemState::Submitted.to_string(), "submitted");
        assert_eq!(SettlementItemState::Confirmed.to_string(), "confirmed");
        assert_eq!(SettlementItemState::Failed.to_string(), "failed");
        assert_eq!(SettlementItemState::Reconciled.to_string(), "reconciled");
    }

    #[test]
    fn test_settlement_item_state_from_str() {
        assert_eq!(
            "submitted".parse::<SettlementItemState>().unwrap(),
            SettlementItemState::Submitted
        );
        assert_eq!(
            "confirmed".parse::<SettlementItemState>().unwrap(),
            SettlementItemState::Confirmed
        );
        assert!("invalid".parse::<SettlementItemState>().is_err());
    }
}
