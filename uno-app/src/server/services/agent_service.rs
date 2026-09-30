//! Agent workflow service
//!
//! Provides:
//! - Agent approval/rejection workflow
//! - Suspension management
//! - Status tracking and audit
//! - Application management

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::net::IpAddr;
use std::sync::Arc;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// TYPES
// ============================================

/// Agent status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    PendingApproval,
    Approved,
    Rejected,
    Suspended,
    Terminated,
}

impl AgentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PendingApproval => "pending_approval",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
            Self::Suspended => "suspended",
            Self::Terminated => "terminated",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending_approval" => Some(Self::PendingApproval),
            "approved" => Some(Self::Approved),
            "rejected" => Some(Self::Rejected),
            "suspended" => Some(Self::Suspended),
            "terminated" => Some(Self::Terminated),
            _ => None,
        }
    }
}

/// Agent entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub email: String,
    pub country: Option<String>,
    pub commission_percent: rust_decimal::Decimal,
    pub referral_code: Option<String>,
    pub is_active: bool,
    pub status: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub rejected_by: Option<String>,
    pub rejection_reason: Option<String>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub suspended_by: Option<String>,
    pub suspension_reason: Option<String>,
    pub suspension_ends_at: Option<DateTime<Utc>>,
    pub license_count: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Agent {
    pub fn get_status(&self) -> AgentStatus {
        self.status.as_ref()
            .and_then(|s| AgentStatus::from_str(s))
            .unwrap_or(AgentStatus::PendingApproval)
    }
}

/// Agent status history entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgentStatusHistory {
    pub id: i64,
    pub agent_id: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub changed_by: String,
    pub change_reason: Option<String>,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}

/// Input for creating agent
#[derive(Debug, Clone)]
pub struct CreateAgentInput {
    pub name: String,
    pub email: String,
    pub country: Option<String>,
    pub commission_percent: Option<rust_decimal::Decimal>,
    pub referral_code: Option<String>,
}

/// Input for approving agent
#[derive(Debug, Clone)]
pub struct ApproveAgentInput {
    pub agent_id: String,
    pub approver_id: String,
    pub notes: Option<String>,
}

/// Input for rejecting agent
#[derive(Debug, Clone)]
pub struct RejectAgentInput {
    pub agent_id: String,
    pub rejector_id: String,
    pub reason: String,
}

/// Input for suspending agent
#[derive(Debug, Clone)]
pub struct SuspendAgentInput {
    pub agent_id: String,
    pub suspender_id: String,
    pub reason: String,
    pub duration_days: Option<i32>,  // None = indefinite
}

/// Input for lifting suspension
#[derive(Debug, Clone)]
pub struct LiftSuspensionInput {
    pub agent_id: String,
    pub lifter_id: String,
    pub reason: Option<String>,
}

/// Summary of agents by status
#[derive(Debug, Clone, Serialize)]
pub struct AgentSummary {
    pub total: i64,
    pub pending_approval: i64,
    pub approved: i64,
    pub rejected: i64,
    pub suspended: i64,
    pub terminated: i64,
}

// ============================================
// SERVICE TRAIT
// ============================================

pub type DynAgentService = Arc<dyn AgentService + Send + Sync>;

#[async_trait]
pub trait AgentService: Send + Sync {
    // CRUD operations
    async fn create_agent(&self, input: CreateAgentInput) -> Result<Agent, AppError>;
    async fn get_agent(&self, id: &str) -> Result<Option<Agent>, AppError>;
    async fn get_agent_by_email(&self, email: &str) -> Result<Option<Agent>, AppError>;
    async fn list_agents(&self, status: Option<AgentStatus>, limit: i32, offset: i32) -> Result<Vec<Agent>, AppError>;
    async fn get_summary(&self) -> Result<AgentSummary, AppError>;

    // Workflow operations
    async fn approve(&self, input: ApproveAgentInput) -> Result<Agent, AppError>;
    async fn reject(&self, input: RejectAgentInput) -> Result<Agent, AppError>;
    async fn suspend(&self, input: SuspendAgentInput) -> Result<Agent, AppError>;
    async fn lift_suspension(&self, input: LiftSuspensionInput) -> Result<Agent, AppError>;
    async fn terminate(&self, agent_id: &str, terminator_id: &str, reason: &str) -> Result<(), AppError>;

    // Status history
    async fn get_status_history(&self, agent_id: &str) -> Result<Vec<AgentStatusHistory>, AppError>;

    // Auto-lift expired suspensions
    async fn auto_lift_expired_suspensions(&self) -> Result<i32, AppError>;

    // Pending approvals
    async fn get_pending_approvals(&self, limit: i32) -> Result<Vec<Agent>, AppError>;
}

// ============================================
// SERVICE IMPLEMENTATION
// ============================================

pub struct AgentServiceImpl {
    pool: ConnectionPool,
}

impl AgentServiceImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AgentService for AgentServiceImpl {
    async fn create_agent(&self, input: CreateAgentInput) -> Result<Agent, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let commission = input.commission_percent
            .unwrap_or_else(|| rust_decimal::Decimal::new(300, 2)); // 3.00%

        let agent = sqlx::query_as::<_, Agent>(
            r#"
            INSERT INTO agents (id, name, email, country, commission_percent, referral_code, status)
            VALUES ($1, $2, $3, $4, $5, $6, 'pending_approval')
            RETURNING id, name, email, country, commission_percent, referral_code, is_active, status,
                      approved_at, approved_by, rejected_at, rejected_by, rejection_reason,
                      suspended_at, suspended_by, suspension_reason, suspension_ends_at,
                      license_count, created_at, updated_at
            "#,
        )
        .bind(&id)
        .bind(&input.name)
        .bind(&input.email)
        .bind(&input.country)
        .bind(commission)
        .bind(&input.referral_code)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Log initial status
        sqlx::query(
            r#"
            INSERT INTO agent_status_history (agent_id, to_status, changed_by, change_reason)
            VALUES ($1, 'pending_approval', 'system', 'Agent application submitted')
            "#,
        )
        .bind(&id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(agent_id = %id, email = %input.email, "Agent created (pending approval)");

        Ok(agent)
    }

    async fn get_agent(&self, id: &str) -> Result<Option<Agent>, AppError> {
        let result = sqlx::query_as::<_, Agent>(
            r#"
            SELECT id, name, email, country, commission_percent, referral_code, is_active, status,
                   approved_at, approved_by, rejected_at, rejected_by, rejection_reason,
                   suspended_at, suspended_by, suspension_reason, suspension_ends_at,
                   license_count, created_at, updated_at
            FROM agents WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn get_agent_by_email(&self, email: &str) -> Result<Option<Agent>, AppError> {
        let result = sqlx::query_as::<_, Agent>(
            r#"
            SELECT id, name, email, country, commission_percent, referral_code, is_active, status,
                   approved_at, approved_by, rejected_at, rejected_by, rejection_reason,
                   suspended_at, suspended_by, suspension_reason, suspension_ends_at,
                   license_count, created_at, updated_at
            FROM agents WHERE email = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn list_agents(&self, status: Option<AgentStatus>, limit: i32, offset: i32) -> Result<Vec<Agent>, AppError> {
        let query = match status {
            Some(s) => {
                sqlx::query_as::<_, Agent>(
                    r#"
                    SELECT id, name, email, country, commission_percent, referral_code, is_active, status,
                           approved_at, approved_by, rejected_at, rejected_by, rejection_reason,
                           suspended_at, suspended_by, suspension_reason, suspension_ends_at,
                           license_count, created_at, updated_at
                    FROM agents
                    WHERE status = $1 AND deleted_at IS NULL
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                )
                .bind(s.as_str())
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
            }
            None => {
                sqlx::query_as::<_, Agent>(
                    r#"
                    SELECT id, name, email, country, commission_percent, referral_code, is_active, status,
                           approved_at, approved_by, rejected_at, rejected_by, rejection_reason,
                           suspended_at, suspended_by, suspension_reason, suspension_ends_at,
                           license_count, created_at, updated_at
                    FROM agents
                    WHERE deleted_at IS NULL
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                    "#,
                )
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
            }
        };

        query.map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn get_summary(&self) -> Result<AgentSummary, AppError> {
        let row: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE status = 'pending_approval') as pending,
                COUNT(*) FILTER (WHERE status = 'approved') as approved,
                COUNT(*) FILTER (WHERE status = 'rejected') as rejected,
                COUNT(*) FILTER (WHERE status = 'suspended') as suspended,
                COUNT(*) FILTER (WHERE status = 'terminated') as terminated
            FROM agents WHERE deleted_at IS NULL
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(AgentSummary {
            total: row.0,
            pending_approval: row.1,
            approved: row.2,
            rejected: row.3,
            suspended: row.4,
            terminated: row.5,
        })
    }

    async fn approve(&self, input: ApproveAgentInput) -> Result<Agent, AppError> {
        // Use database function for atomic update
        let result: (bool,) = sqlx::query_as(
            "SELECT approve_agent($1, $2, $3)"
        )
        .bind(&input.agent_id)
        .bind(&input.approver_id)
        .bind(&input.notes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !result.0 {
            return Err(AppError::ValidationError("Failed to approve agent".into()));
        }

        tracing::info!(
            agent_id = %input.agent_id,
            approver = %input.approver_id,
            "Agent approved"
        );

        self.get_agent(&input.agent_id).await?
            .ok_or_else(|| AppError::NotFound("Agent not found".into()))
    }

    async fn reject(&self, input: RejectAgentInput) -> Result<Agent, AppError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT reject_agent($1, $2, $3)"
        )
        .bind(&input.agent_id)
        .bind(&input.rejector_id)
        .bind(&input.reason)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !result.0 {
            return Err(AppError::ValidationError("Failed to reject agent".into()));
        }

        tracing::info!(
            agent_id = %input.agent_id,
            rejector = %input.rejector_id,
            reason = %input.reason,
            "Agent rejected"
        );

        self.get_agent(&input.agent_id).await?
            .ok_or_else(|| AppError::NotFound("Agent not found".into()))
    }

    async fn suspend(&self, input: SuspendAgentInput) -> Result<Agent, AppError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT suspend_agent($1, $2, $3, $4)"
        )
        .bind(&input.agent_id)
        .bind(&input.suspender_id)
        .bind(&input.reason)
        .bind(input.duration_days)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !result.0 {
            return Err(AppError::ValidationError("Failed to suspend agent".into()));
        }

        tracing::warn!(
            agent_id = %input.agent_id,
            suspender = %input.suspender_id,
            reason = %input.reason,
            duration_days = ?input.duration_days,
            "Agent suspended"
        );

        self.get_agent(&input.agent_id).await?
            .ok_or_else(|| AppError::NotFound("Agent not found".into()))
    }

    async fn lift_suspension(&self, input: LiftSuspensionInput) -> Result<Agent, AppError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT lift_agent_suspension($1, $2, $3)"
        )
        .bind(&input.agent_id)
        .bind(&input.lifter_id)
        .bind(input.reason.as_deref().unwrap_or("Suspension lifted"))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !result.0 {
            return Err(AppError::ValidationError("Failed to lift suspension".into()));
        }

        tracing::info!(
            agent_id = %input.agent_id,
            lifter = %input.lifter_id,
            "Agent suspension lifted"
        );

        self.get_agent(&input.agent_id).await?
            .ok_or_else(|| AppError::NotFound("Agent not found".into()))
    }

    async fn terminate(&self, agent_id: &str, terminator_id: &str, reason: &str) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Get current status
        let current: Option<(String,)> = sqlx::query_as(
            "SELECT status FROM agents WHERE id = $1 FOR UPDATE"
        )
        .bind(agent_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let current_status = current
            .ok_or_else(|| AppError::NotFound("Agent not found".into()))?
            .0;

        // Update agent
        sqlx::query(
            r#"
            UPDATE agents SET
                status = 'terminated',
                is_active = FALSE,
                terminated_at = NOW(),
                terminated_by = $2,
                termination_reason = $3,
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(agent_id)
        .bind(terminator_id)
        .bind(reason)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Log history
        sqlx::query(
            r#"
            INSERT INTO agent_status_history (agent_id, from_status, to_status, changed_by, change_reason)
            VALUES ($1, $2::agent_status, 'terminated', $3, $4)
            "#,
        )
        .bind(agent_id)
        .bind(&current_status)
        .bind(terminator_id)
        .bind(reason)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::warn!(
            agent_id = %agent_id,
            terminator = %terminator_id,
            reason = %reason,
            "Agent terminated"
        );

        Ok(())
    }

    async fn get_status_history(&self, agent_id: &str) -> Result<Vec<AgentStatusHistory>, AppError> {
        let results = sqlx::query_as::<_, AgentStatusHistory>(
            r#"
            SELECT id, agent_id, from_status, to_status, changed_by, change_reason, metadata, created_at
            FROM agent_status_history
            WHERE agent_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(agent_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn auto_lift_expired_suspensions(&self) -> Result<i32, AppError> {
        let result: (i32,) = sqlx::query_as(
            "SELECT auto_lift_expired_suspensions()"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.0 > 0 {
            tracing::info!(count = result.0, "Auto-lifted expired agent suspensions");
        }

        Ok(result.0)
    }

    async fn get_pending_approvals(&self, limit: i32) -> Result<Vec<Agent>, AppError> {
        self.list_agents(Some(AgentStatus::PendingApproval), limit, 0).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_status() {
        assert_eq!(AgentStatus::Approved.as_str(), "approved");
        assert_eq!(AgentStatus::from_str("suspended"), Some(AgentStatus::Suspended));
    }
}
