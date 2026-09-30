//! Evidence repository for gate evidence tracking
//!
//! Phase 9: Release Validation - tracks evidence requirements
//! for launch gate enablement.

use crate::server::db::ConnectionPool;
use crate::types::AppError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

/// Dynamic type alias for dependency injection
pub type DynEvidenceRepository = Arc<dyn EvidenceRepository + Send + Sync>;

/// Evidence requirement for a launch gate
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GateEvidenceRequirement {
    pub id: Uuid,
    pub gate_name: String,
    pub requirement_type: String,
    pub requirement_description: String,
    pub is_mandatory: bool,
    pub evidence_url: Option<String>,
    pub evidence_hash: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Gate enablement log entry
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GateEnablementLog {
    pub id: Uuid,
    pub gate_name: String,
    pub action: String,
    pub actor: String,
    pub reason: Option<String>,
    pub evidence_snapshot: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// Emergency pause record
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EmergencyPause {
    pub id: Uuid,
    pub initiated_by: String,
    pub reason: String,
    pub gates_paused: Vec<String>,
    pub paused_at: DateTime<Utc>,
    pub resumed_at: Option<DateTime<Utc>>,
    pub resumed_by: Option<String>,
    pub resume_reason: Option<String>,
}

/// Input for recording evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordEvidenceInput {
    pub gate_name: String,
    pub requirement_type: String,
    pub evidence_url: String,
    pub evidence_hash: Option<String>,
    pub verified_by: String,
}

/// Input for creating an emergency pause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmergencyPauseInput {
    pub initiated_by: String,
    pub reason: String,
    pub gates_to_pause: Vec<String>,
}

/// Summary of evidence status for a gate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateEvidenceSummary {
    pub gate_name: String,
    pub total_requirements: i64,
    pub mandatory_requirements: i64,
    pub verified_requirements: i64,
    pub verified_mandatory: i64,
    pub can_enable: bool,
}

/// Evidence repository trait
#[async_trait]
pub trait EvidenceRepository {
    /// Get all evidence requirements for a gate
    async fn get_requirements(&self, gate_name: &str) -> Result<Vec<GateEvidenceRequirement>, AppError>;

    /// Get a specific evidence requirement
    async fn get_requirement(&self, gate_name: &str, requirement_type: &str) -> Result<Option<GateEvidenceRequirement>, AppError>;

    /// Record evidence for a requirement
    async fn record_evidence(&self, input: RecordEvidenceInput) -> Result<GateEvidenceRequirement, AppError>;

    /// Get evidence summary for a gate
    async fn get_evidence_summary(&self, gate_name: &str) -> Result<GateEvidenceSummary, AppError>;

    /// Check if a gate can be enabled (all mandatory evidence verified)
    async fn can_enable_gate(&self, gate_name: &str) -> Result<bool, AppError>;

    /// Log a gate enablement action
    async fn log_enablement(&self, gate_name: &str, action: &str, actor: &str, reason: Option<&str>) -> Result<GateEnablementLog, AppError>;

    /// Get enablement history for a gate
    async fn get_enablement_history(&self, gate_name: &str) -> Result<Vec<GateEnablementLog>, AppError>;

    /// Create an emergency pause
    async fn create_emergency_pause(&self, input: CreateEmergencyPauseInput) -> Result<EmergencyPause, AppError>;

    /// Resume from emergency pause
    async fn resume_emergency_pause(&self, pause_id: Uuid, resumed_by: &str, reason: &str) -> Result<EmergencyPause, AppError>;

    /// Get active emergency pauses
    async fn get_active_pauses(&self) -> Result<Vec<EmergencyPause>, AppError>;
}

/// PostgreSQL implementation of EvidenceRepository
pub struct EvidenceRepositoryImpl {
    pool: ConnectionPool,
}

impl EvidenceRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EvidenceRepository for EvidenceRepositoryImpl {
    async fn get_requirements(&self, gate_name: &str) -> Result<Vec<GateEvidenceRequirement>, AppError> {
        let requirements = sqlx::query_as::<_, GateEvidenceRequirement>(
            r#"
            SELECT id, gate_name, requirement_type, requirement_description,
                   is_mandatory, evidence_url, evidence_hash, verified_at,
                   verified_by, created_at, updated_at
            FROM gate_evidence_requirements
            WHERE gate_name = $1
            ORDER BY is_mandatory DESC, requirement_type
            "#,
        )
        .bind(gate_name)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(requirements)
    }

    async fn get_requirement(&self, gate_name: &str, requirement_type: &str) -> Result<Option<GateEvidenceRequirement>, AppError> {
        let requirement = sqlx::query_as::<_, GateEvidenceRequirement>(
            r#"
            SELECT id, gate_name, requirement_type, requirement_description,
                   is_mandatory, evidence_url, evidence_hash, verified_at,
                   verified_by, created_at, updated_at
            FROM gate_evidence_requirements
            WHERE gate_name = $1 AND requirement_type = $2
            "#,
        )
        .bind(gate_name)
        .bind(requirement_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(requirement)
    }

    async fn record_evidence(&self, input: RecordEvidenceInput) -> Result<GateEvidenceRequirement, AppError> {
        let requirement = sqlx::query_as::<_, GateEvidenceRequirement>(
            r#"
            UPDATE gate_evidence_requirements
            SET evidence_url = $3,
                evidence_hash = $4,
                verified_at = NOW(),
                verified_by = $5,
                updated_at = NOW()
            WHERE gate_name = $1 AND requirement_type = $2
            RETURNING id, gate_name, requirement_type, requirement_description,
                      is_mandatory, evidence_url, evidence_hash, verified_at,
                      verified_by, created_at, updated_at
            "#,
        )
        .bind(&input.gate_name)
        .bind(&input.requirement_type)
        .bind(&input.evidence_url)
        .bind(&input.evidence_hash)
        .bind(&input.verified_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(requirement)
    }

    async fn get_evidence_summary(&self, gate_name: &str) -> Result<GateEvidenceSummary, AppError> {
        let (total, mandatory, verified, verified_mandatory): (i64, i64, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE is_mandatory) as mandatory,
                COUNT(*) FILTER (WHERE verified_at IS NOT NULL) as verified,
                COUNT(*) FILTER (WHERE is_mandatory AND verified_at IS NOT NULL) as verified_mandatory
            FROM gate_evidence_requirements
            WHERE gate_name = $1
            "#,
        )
        .bind(gate_name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(GateEvidenceSummary {
            gate_name: gate_name.to_string(),
            total_requirements: total,
            mandatory_requirements: mandatory,
            verified_requirements: verified,
            verified_mandatory,
            can_enable: mandatory == verified_mandatory,
        })
    }

    async fn can_enable_gate(&self, gate_name: &str) -> Result<bool, AppError> {
        let summary = self.get_evidence_summary(gate_name).await?;
        Ok(summary.can_enable)
    }

    async fn log_enablement(&self, gate_name: &str, action: &str, actor: &str, reason: Option<&str>) -> Result<GateEnablementLog, AppError> {
        // Get current evidence snapshot
        let requirements = self.get_requirements(gate_name).await?;
        let snapshot = serde_json::to_value(&requirements).ok();

        let log = sqlx::query_as::<_, GateEnablementLog>(
            r#"
            INSERT INTO gate_enablement_log (gate_name, action, actor, reason, evidence_snapshot)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, gate_name, action, actor, reason, evidence_snapshot, created_at
            "#,
        )
        .bind(gate_name)
        .bind(action)
        .bind(actor)
        .bind(reason)
        .bind(&snapshot)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(log)
    }

    async fn get_enablement_history(&self, gate_name: &str) -> Result<Vec<GateEnablementLog>, AppError> {
        let history = sqlx::query_as::<_, GateEnablementLog>(
            r#"
            SELECT id, gate_name, action, actor, reason, evidence_snapshot, created_at
            FROM gate_enablement_log
            WHERE gate_name = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(gate_name)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(history)
    }

    async fn create_emergency_pause(&self, input: CreateEmergencyPauseInput) -> Result<EmergencyPause, AppError> {
        let pause = sqlx::query_as::<_, EmergencyPause>(
            r#"
            INSERT INTO emergency_pauses (initiated_by, reason, gates_paused)
            VALUES ($1, $2, $3)
            RETURNING id, initiated_by, reason, gates_paused, paused_at, resumed_at, resumed_by, resume_reason
            "#,
        )
        .bind(&input.initiated_by)
        .bind(&input.reason)
        .bind(&input.gates_to_pause)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(pause)
    }

    async fn resume_emergency_pause(&self, pause_id: Uuid, resumed_by: &str, reason: &str) -> Result<EmergencyPause, AppError> {
        let pause = sqlx::query_as::<_, EmergencyPause>(
            r#"
            UPDATE emergency_pauses
            SET resumed_at = NOW(),
                resumed_by = $2,
                resume_reason = $3
            WHERE id = $1
            RETURNING id, initiated_by, reason, gates_paused, paused_at, resumed_at, resumed_by, resume_reason
            "#,
        )
        .bind(pause_id)
        .bind(resumed_by)
        .bind(reason)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(pause)
    }

    async fn get_active_pauses(&self) -> Result<Vec<EmergencyPause>, AppError> {
        let pauses = sqlx::query_as::<_, EmergencyPause>(
            r#"
            SELECT id, initiated_by, reason, gates_paused, paused_at, resumed_at, resumed_by, resume_reason
            FROM emergency_pauses
            WHERE resumed_at IS NULL
            ORDER BY paused_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(pauses)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_summary_can_enable() {
        let summary = GateEvidenceSummary {
            gate_name: "test_gate".to_string(),
            total_requirements: 5,
            mandatory_requirements: 3,
            verified_requirements: 5,
            verified_mandatory: 3,
            can_enable: true,
        };
        assert!(summary.can_enable);

        let incomplete = GateEvidenceSummary {
            gate_name: "test_gate".to_string(),
            total_requirements: 5,
            mandatory_requirements: 3,
            verified_requirements: 2,
            verified_mandatory: 2,
            can_enable: false,
        };
        assert!(!incomplete.can_enable);
    }
}
