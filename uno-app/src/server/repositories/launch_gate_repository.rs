//! Launch gate repository for database operations
//!
//! Provides access to launch_gates and launch_gate_history tables
//! for server-enforced feature toggles with compliance tracking.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

/// Dynamic type alias for LaunchGateRepository trait object
pub type DynLaunchGateRepository = Arc<dyn LaunchGateRepository + Send + Sync>;

// ============================================
// TYPES
// ============================================

/// Known gate names for type-safe access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateName {
    Issuance,
    Marketplace,
    Funding,
    ReferralPayments,
    Campaigns,
    Uploads,
}

impl GateName {
    /// Get the database string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            GateName::Issuance => "issuance",
            GateName::Marketplace => "marketplace",
            GateName::Funding => "funding",
            GateName::ReferralPayments => "referral_payments",
            GateName::Campaigns => "campaigns",
            GateName::Uploads => "uploads",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "issuance" => Some(GateName::Issuance),
            "marketplace" => Some(GateName::Marketplace),
            "funding" => Some(GateName::Funding),
            "referral_payments" => Some(GateName::ReferralPayments),
            "campaigns" => Some(GateName::Campaigns),
            "uploads" => Some(GateName::Uploads),
            _ => None,
        }
    }

    /// Get all gate names
    pub fn all() -> &'static [GateName] {
        &[
            GateName::Issuance,
            GateName::Marketplace,
            GateName::Funding,
            GateName::ReferralPayments,
            GateName::Campaigns,
            GateName::Uploads,
        ]
    }
}

impl std::fmt::Display for GateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Launch gate record from database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct LaunchGate {
    pub id: i32,
    pub gate_name: String,
    pub gate_description: Option<String>,
    pub is_enabled: bool,
    pub enabled_at: Option<DateTime<Utc>>,
    pub enabled_by: Option<String>,
    pub enabled_reason: Option<String>,
    pub disabled_at: Option<DateTime<Utc>>,
    pub disabled_by: Option<String>,
    pub disabled_reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub evidence_url: Option<String>,
    pub evidence_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl LaunchGate {
    /// Check if gate is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            expires_at < Utc::now()
        } else {
            false
        }
    }

    /// Get effective enabled state (considering expiry)
    pub fn is_effectively_enabled(&self) -> bool {
        self.is_enabled && !self.is_expired()
    }
}

/// Launch gate history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct LaunchGateHistory {
    pub id: i64,
    pub gate_id: i32,
    pub previous_state: Option<bool>,
    pub new_state: bool,
    pub changed_by: String,
    pub change_reason: String,
    pub evidence_url: Option<String>,
    pub changed_at: DateTime<Utc>,
}

// ============================================
// REPOSITORY TRAIT
// ============================================

/// Launch gate repository trait defining database operations
#[async_trait]
pub trait LaunchGateRepository: Send + Sync {
    /// Get a gate by name
    async fn get_gate(&self, name: &str) -> Result<Option<LaunchGate>, AppError>;

    /// Get all gates
    async fn get_all_gates(&self) -> Result<Vec<LaunchGate>, AppError>;

    /// Check if a gate is enabled (returns false if not found)
    async fn is_gate_enabled(&self, name: &str) -> Result<bool, AppError>;

    /// Set gate state (enable or disable)
    async fn set_gate_state(
        &self,
        name: &str,
        enabled: bool,
        actor: &str,
        reason: &str,
        evidence_url: Option<&str>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<LaunchGate, AppError>;

    /// Get gate history
    async fn get_gate_history(
        &self,
        name: &str,
        limit: i32,
    ) -> Result<Vec<LaunchGateHistory>, AppError>;

    /// Check and disable expired gates, returns count of gates disabled
    async fn check_gate_expiry(&self) -> Result<i64, AppError>;
}

// ============================================
// IMPLEMENTATION
// ============================================

/// Concrete implementation of LaunchGateRepository
pub struct LaunchGateRepositoryImpl {
    db_pool: ConnectionPool,
}

impl LaunchGateRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl LaunchGateRepository for LaunchGateRepositoryImpl {
    async fn get_gate(&self, name: &str) -> Result<Option<LaunchGate>, AppError> {
        let gate = sqlx::query_as::<_, LaunchGate>(
            r#"
            SELECT id, gate_name, gate_description, is_enabled,
                   enabled_at, enabled_by, enabled_reason,
                   disabled_at, disabled_by, disabled_reason,
                   expires_at, evidence_url, evidence_notes,
                   created_at, updated_at
            FROM launch_gates
            WHERE gate_name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(gate)
    }

    async fn get_all_gates(&self) -> Result<Vec<LaunchGate>, AppError> {
        let gates = sqlx::query_as::<_, LaunchGate>(
            r#"
            SELECT id, gate_name, gate_description, is_enabled,
                   enabled_at, enabled_by, enabled_reason,
                   disabled_at, disabled_by, disabled_reason,
                   expires_at, evidence_url, evidence_notes,
                   created_at, updated_at
            FROM launch_gates
            ORDER BY gate_name ASC
            "#,
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(gates)
    }

    async fn is_gate_enabled(&self, name: &str) -> Result<bool, AppError> {
        let result: Option<(bool, Option<DateTime<Utc>>)> = sqlx::query_as(
            r#"
            SELECT is_enabled, expires_at
            FROM launch_gates
            WHERE gate_name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(&self.db_pool)
        .await?;

        match result {
            Some((is_enabled, expires_at)) => {
                // Check if expired
                if let Some(exp) = expires_at {
                    if exp < Utc::now() {
                        return Ok(false);
                    }
                }
                Ok(is_enabled)
            }
            None => Ok(false),
        }
    }

    async fn set_gate_state(
        &self,
        name: &str,
        enabled: bool,
        actor: &str,
        reason: &str,
        evidence_url: Option<&str>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<LaunchGate, AppError> {
        // Start transaction
        let mut tx = self.db_pool.begin().await?;

        // Get current state for history
        let current: Option<(i32, bool)> = sqlx::query_as(
            r#"
            SELECT id, is_enabled
            FROM launch_gates
            WHERE gate_name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(&mut *tx)
        .await?;

        let (gate_id, previous_state) = match current {
            Some((id, state)) => (id, Some(state)),
            None => {
                return Err(AppError::NotFound(format!("Gate '{}' not found", name)));
            }
        };

        // Update gate state
        let gate = if enabled {
            sqlx::query_as::<_, LaunchGate>(
                r#"
                UPDATE launch_gates
                SET is_enabled = true,
                    enabled_at = NOW(),
                    enabled_by = $2,
                    enabled_reason = $3,
                    evidence_url = $4,
                    expires_at = $5,
                    updated_at = NOW()
                WHERE gate_name = $1
                RETURNING id, gate_name, gate_description, is_enabled,
                          enabled_at, enabled_by, enabled_reason,
                          disabled_at, disabled_by, disabled_reason,
                          expires_at, evidence_url, evidence_notes,
                          created_at, updated_at
                "#,
            )
            .bind(name)
            .bind(actor)
            .bind(reason)
            .bind(evidence_url)
            .bind(expires_at)
            .fetch_one(&mut *tx)
            .await?
        } else {
            sqlx::query_as::<_, LaunchGate>(
                r#"
                UPDATE launch_gates
                SET is_enabled = false,
                    disabled_at = NOW(),
                    disabled_by = $2,
                    disabled_reason = $3,
                    expires_at = NULL,
                    updated_at = NOW()
                WHERE gate_name = $1
                RETURNING id, gate_name, gate_description, is_enabled,
                          enabled_at, enabled_by, enabled_reason,
                          disabled_at, disabled_by, disabled_reason,
                          expires_at, evidence_url, evidence_notes,
                          created_at, updated_at
                "#,
            )
            .bind(name)
            .bind(actor)
            .bind(reason)
            .fetch_one(&mut *tx)
            .await?
        };

        // Insert history record
        sqlx::query(
            r#"
            INSERT INTO launch_gate_history
                (gate_id, previous_state, new_state, changed_by, change_reason, evidence_url)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(gate_id)
        .bind(previous_state)
        .bind(enabled)
        .bind(actor)
        .bind(reason)
        .bind(evidence_url)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(gate)
    }

    async fn get_gate_history(
        &self,
        name: &str,
        limit: i32,
    ) -> Result<Vec<LaunchGateHistory>, AppError> {
        let history = sqlx::query_as::<_, LaunchGateHistory>(
            r#"
            SELECT h.id, h.gate_id, h.previous_state, h.new_state,
                   h.changed_by, h.change_reason, h.evidence_url, h.changed_at
            FROM launch_gate_history h
            INNER JOIN launch_gates g ON g.id = h.gate_id
            WHERE g.gate_name = $1
            ORDER BY h.changed_at DESC
            LIMIT $2
            "#,
        )
        .bind(name)
        .bind(limit)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(history)
    }

    async fn check_gate_expiry(&self) -> Result<i64, AppError> {
        // Start transaction
        let mut tx = self.db_pool.begin().await?;

        // Find expired gates that are still enabled
        let expired_gates: Vec<(i32, String)> = sqlx::query_as(
            r#"
            SELECT id, gate_name
            FROM launch_gates
            WHERE is_enabled = true
              AND expires_at IS NOT NULL
              AND expires_at < NOW()
            "#,
        )
        .fetch_all(&mut *tx)
        .await?;

        let count = expired_gates.len() as i64;

        // Disable each expired gate and add history
        for (gate_id, gate_name) in &expired_gates {
            // Update gate
            sqlx::query(
                r#"
                UPDATE launch_gates
                SET is_enabled = false,
                    disabled_at = NOW(),
                    disabled_by = 'system',
                    disabled_reason = 'Auto-disabled: gate expired',
                    updated_at = NOW()
                WHERE id = $1
                "#,
            )
            .bind(gate_id)
            .execute(&mut *tx)
            .await?;

            // Insert history
            sqlx::query(
                r#"
                INSERT INTO launch_gate_history
                    (gate_id, previous_state, new_state, changed_by, change_reason)
                VALUES ($1, true, false, 'system', 'Auto-disabled: gate expired at scheduled time')
                "#,
            )
            .bind(gate_id)
            .execute(&mut *tx)
            .await?;

            tracing::info!(
                gate_name = %gate_name,
                "Auto-disabled expired launch gate"
            );
        }

        tx.commit().await?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_name_roundtrip() {
        for gate in GateName::all() {
            let s = gate.as_str();
            let parsed = GateName::from_str(s);
            assert_eq!(parsed, Some(*gate));
        }
    }

    #[test]
    fn test_gate_name_display() {
        assert_eq!(GateName::Issuance.to_string(), "issuance");
        assert_eq!(GateName::ReferralPayments.to_string(), "referral_payments");
    }
}
