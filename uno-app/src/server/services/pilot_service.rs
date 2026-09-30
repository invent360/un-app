//! Pilot service for controlled rollout management
//!
//! Phase 9: Release Validation - manages pilot participant lifecycle,
//! cohort tracking, and milestone validation.

use crate::server::metrics;
use crate::server::repositories::{
    DynImmutableAuditRepository, DynLaunchGateRepository, DynPilotRepository,
    AuditEventBuilder, AuditCategory, AuditEventType, AuditOutcome, ActorType,
    PilotCohort, PilotParticipant, PilotParticipantState, CohortReport,
    AddParticipantInput, TransitionStateInput, RecordPilotActivityInput,
};
use crate::types::AppError;
use std::sync::Arc;

/// Dynamic type alias for dependency injection
pub type DynPilotService = Arc<dyn PilotService + Send + Sync>;

/// Pilot service trait
#[async_trait::async_trait]
pub trait PilotService: Send + Sync {
    /// Check if participant enrollment is allowed
    async fn can_enroll(&self, market_code: &str) -> Result<bool, AppError>;

    /// Add a new participant to a pilot cohort
    async fn add_participant(&self, input: AddParticipantInput, actor: &str) -> Result<PilotParticipant, AppError>;

    /// Transition a participant to a new state
    async fn transition_state(&self, input: TransitionStateInput) -> Result<PilotParticipant, AppError>;

    /// Record participant activity
    async fn record_activity(&self, input: RecordPilotActivityInput) -> Result<(), AppError>;

    /// Process D7 milestone check
    async fn process_d7_check(&self, participant_id: uuid::Uuid, is_active: bool, actor: &str) -> Result<PilotParticipant, AppError>;

    /// Process D30 milestone check
    async fn process_d30_check(&self, participant_id: uuid::Uuid, is_active: bool, actor: &str) -> Result<PilotParticipant, AppError>;

    /// Get participants pending D7 check
    async fn get_pending_d7(&self) -> Result<Vec<PilotParticipant>, AppError>;

    /// Get participants pending D30 check
    async fn get_pending_d30(&self) -> Result<Vec<PilotParticipant>, AppError>;

    /// Get cohort report
    async fn get_cohort_report(&self, cohort_id: &str) -> Result<CohortReport, AppError>;

    /// Get all active cohorts
    async fn get_active_cohorts(&self, market_code: Option<&str>) -> Result<Vec<PilotCohort>, AppError>;
}

/// Pilot service implementation
pub struct PilotServiceImpl {
    pilot_repo: DynPilotRepository,
    launch_gate_repo: DynLaunchGateRepository,
    audit_repo: DynImmutableAuditRepository,
}

impl PilotServiceImpl {
    pub fn new(
        pilot_repo: DynPilotRepository,
        launch_gate_repo: DynLaunchGateRepository,
        audit_repo: DynImmutableAuditRepository,
    ) -> Self {
        Self {
            pilot_repo,
            launch_gate_repo,
            audit_repo,
        }
    }

    async fn log_audit(&self, event: AuditEventBuilder) {
        if let Err(e) = self.audit_repo.log_event(event.build()).await {
            tracing::warn!("Failed to log pilot audit event: {}", e);
        }
    }
}

#[async_trait::async_trait]
impl PilotService for PilotServiceImpl {
    async fn can_enroll(&self, market_code: &str) -> Result<bool, AppError> {
        // Check if pilot enrollment is enabled
        if let Ok(Some(gate)) = self.launch_gate_repo.get_gate("pilot_enrollment").await {
            if !gate.is_enabled {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }

        // Check market-specific gate
        let market_gate = format!("pilot_market_{}", market_code.to_lowercase());
        if let Ok(Some(gate)) = self.launch_gate_repo.get_gate(&market_gate).await {
            if !gate.is_enabled {
                return Ok(false);
            }
        }

        // Check if any cohort has capacity
        let cohorts = self.pilot_repo.get_active_cohorts(market_code).await?;
        for cohort in cohorts {
            if self.pilot_repo.cohort_has_capacity(&cohort.id).await? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn add_participant(&self, input: AddParticipantInput, actor: &str) -> Result<PilotParticipant, AppError> {
        // Verify enrollment is allowed
        if !self.can_enroll(&input.market_code).await? {
            return Err(AppError::ValidationError("Pilot enrollment not available for this market".to_string()));
        }

        // Verify cohort has capacity
        if !self.pilot_repo.cohort_has_capacity(&input.cohort_id).await? {
            return Err(AppError::ValidationError("Cohort is at capacity".to_string()));
        }

        // Check if user already in cohort
        if let Some(_) = self.pilot_repo.get_participant_by_user(&input.external_user_id, &input.cohort_id).await? {
            return Err(AppError::ValidationError("User already enrolled in this cohort".to_string()));
        }

        // Add participant
        let participant = self.pilot_repo.add_participant(input.clone()).await?;

        // Update metrics
        metrics::PILOT_PARTICIPANTS
            .with_label_values(&[&input.market_code, "invited"])
            .inc();

        // Log audit event
        self.log_audit(
            AuditEventBuilder::new(AuditEventType::Create, AuditCategory::System)
                .actor(ActorType::System, actor)
                .resource("pilot_participant", participant.id.to_string())
                .action("add_participant")
                .outcome(AuditOutcome::Success)
                .event_data(serde_json::json!({
                    "cohort_id": &input.cohort_id,
                    "market_code": &input.market_code,
                }))
        ).await;

        Ok(participant)
    }

    async fn transition_state(&self, input: TransitionStateInput) -> Result<PilotParticipant, AppError> {
        let participant = self.pilot_repo.transition_state(input.clone()).await?;

        // Update metrics
        let old_state = if let Some(p) = self.pilot_repo.get_participant(input.participant_id).await? {
            p.state.to_string()
        } else {
            "unknown".to_string()
        };

        metrics::PILOT_PARTICIPANTS
            .with_label_values(&[&participant.market_code, &old_state])
            .dec();
        metrics::PILOT_PARTICIPANTS
            .with_label_values(&[&participant.market_code, &participant.state.to_string()])
            .inc();

        // Track milestone completions
        match input.new_state {
            PilotParticipantState::Registered => {
                metrics::PILOT_MILESTONES.with_label_values(&["registered"]).inc();
            }
            PilotParticipantState::LicenseClaimed => {
                metrics::PILOT_MILESTONES.with_label_values(&["license_claimed"]).inc();
            }
            PilotParticipantState::D7Active => {
                metrics::PILOT_MILESTONES.with_label_values(&["d7_active"]).inc();
            }
            PilotParticipantState::D30Active => {
                metrics::PILOT_MILESTONES.with_label_values(&["d30_active"]).inc();
            }
            PilotParticipantState::Graduated => {
                metrics::PILOT_MILESTONES.with_label_values(&["graduated"]).inc();
            }
            _ => {}
        }

        // Log audit event
        self.log_audit(
            AuditEventBuilder::new(AuditEventType::Update, AuditCategory::System)
                .actor(ActorType::System, input.actor.as_deref().unwrap_or("system"))
                .resource("pilot_participant", input.participant_id.to_string())
                .action("transition_state")
                .outcome(AuditOutcome::Success)
                .event_data(serde_json::json!({
                    "from_state": &old_state,
                    "to_state": &participant.state.to_string(),
                }))
        ).await;

        Ok(participant)
    }

    async fn record_activity(&self, input: RecordPilotActivityInput) -> Result<(), AppError> {
        self.pilot_repo.record_activity(input).await?;
        Ok(())
    }

    async fn process_d7_check(&self, participant_id: uuid::Uuid, is_active: bool, actor: &str) -> Result<PilotParticipant, AppError> {
        let new_state = if is_active {
            PilotParticipantState::D7Active
        } else {
            PilotParticipantState::D7Inactive
        };

        let input = TransitionStateInput {
            participant_id,
            new_state,
            triggered_by: "system".to_string(),
            actor: Some(actor.to_string()),
            reason: Some(format!("D7 check: {}", if is_active { "active" } else { "inactive" })),
            metadata: None,
        };

        self.transition_state(input).await
    }

    async fn process_d30_check(&self, participant_id: uuid::Uuid, is_active: bool, actor: &str) -> Result<PilotParticipant, AppError> {
        let new_state = if is_active {
            PilotParticipantState::D30Active
        } else {
            PilotParticipantState::D30Inactive
        };

        let input = TransitionStateInput {
            participant_id,
            new_state,
            triggered_by: "system".to_string(),
            actor: Some(actor.to_string()),
            reason: Some(format!("D30 check: {}", if is_active { "active" } else { "inactive" })),
            metadata: None,
        };

        self.transition_state(input).await
    }

    async fn get_pending_d7(&self) -> Result<Vec<PilotParticipant>, AppError> {
        self.pilot_repo.get_pending_d7_check().await
    }

    async fn get_pending_d30(&self) -> Result<Vec<PilotParticipant>, AppError> {
        self.pilot_repo.get_pending_d30_check().await
    }

    async fn get_cohort_report(&self, cohort_id: &str) -> Result<CohortReport, AppError> {
        self.pilot_repo.get_cohort_report(cohort_id).await
    }

    async fn get_active_cohorts(&self, market_code: Option<&str>) -> Result<Vec<PilotCohort>, AppError> {
        if let Some(market) = market_code {
            self.pilot_repo.get_active_cohorts(market).await
        } else {
            let all_cohorts = self.pilot_repo.get_cohorts().await?;
            Ok(all_cohorts.into_iter().filter(|c| c.is_active).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pilot_participant_state_display() {
        assert_eq!(PilotParticipantState::Invited.to_string(), "invited");
        assert_eq!(PilotParticipantState::D7Active.to_string(), "d7_active");
    }
}
