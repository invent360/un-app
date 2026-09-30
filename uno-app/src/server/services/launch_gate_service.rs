//! Launch gate service for feature toggle management
//!
//! Provides business logic for managing launch gates with audit logging.
//! Gates are server-enforced pause controls for features like issuance,
//! marketplace, funding, etc.

use chrono::{DateTime, Utc};
use serde_json::json;
use tracing::{info, warn};

use crate::server::repositories::{
    ActorType, AuditCategory, AuditEvent, AuditEventType, AuditOutcome,
    DynImmutableAuditRepository, DynLaunchGateRepository, GateName, LaunchGate, LaunchGateHistory,
};
use crate::types::AppError;

/// Resource type for launch gates in audit log
const RESOURCE_LAUNCH_GATE: &str = "launch_gate";

/// Launch gate service for managing feature toggles
#[derive(Clone)]
pub struct LaunchGateService {
    gate_repo: DynLaunchGateRepository,
    audit_repo: DynImmutableAuditRepository,
}

impl LaunchGateService {
    /// Create a new launch gate service
    pub fn new(gate_repo: DynLaunchGateRepository, audit_repo: DynImmutableAuditRepository) -> Self {
        Self {
            gate_repo,
            audit_repo,
        }
    }

    // ============================================
    // QUERY METHODS
    // ============================================

    /// Check if a gate is enabled
    ///
    /// Returns true if the gate exists, is enabled, and not expired.
    /// Returns false if the gate doesn't exist or is disabled/expired.
    pub async fn is_enabled(&self, name: &str) -> bool {
        match self.gate_repo.is_gate_enabled(name).await {
            Ok(enabled) => enabled,
            Err(e) => {
                warn!(gate = %name, error = %e, "Failed to check gate status, defaulting to disabled");
                false
            }
        }
    }

    /// Check if a gate is enabled by enum
    pub async fn is_enabled_by_name(&self, gate: GateName) -> bool {
        self.is_enabled(gate.as_str()).await
    }

    /// Require a gate to be enabled, returning an error if not
    ///
    /// Use this in handlers to guard features behind launch gates.
    ///
    /// # Example
    /// ```ignore
    /// launch_gate_service.require_gate("issuance").await?;
    /// // ... proceed with issuance logic
    /// ```
    pub async fn require_gate(&self, name: &str) -> Result<(), AppError> {
        if !self.is_enabled(name).await {
            return Err(AppError::Forbidden(format!(
                "Feature '{}' is currently disabled",
                name
            )));
        }
        Ok(())
    }

    /// Require a gate to be enabled by enum
    pub async fn require_gate_by_name(&self, gate: GateName) -> Result<(), AppError> {
        self.require_gate(gate.as_str()).await
    }

    /// Get all gates
    pub async fn get_all_gates(&self) -> Result<Vec<LaunchGate>, AppError> {
        self.gate_repo.get_all_gates().await
    }

    /// Get a specific gate
    pub async fn get_gate(&self, name: &str) -> Result<Option<LaunchGate>, AppError> {
        self.gate_repo.get_gate(name).await
    }

    /// Get gate history
    pub async fn get_gate_history(
        &self,
        name: &str,
        limit: i32,
    ) -> Result<Vec<LaunchGateHistory>, AppError> {
        self.gate_repo.get_gate_history(name, limit).await
    }

    // ============================================
    // MUTATION METHODS
    // ============================================

    /// Enable a gate
    ///
    /// # Arguments
    /// * `name` - Gate name (e.g., "issuance", "marketplace")
    /// * `actor` - ID of the user/system enabling the gate
    /// * `reason` - Explanation for why the gate is being enabled
    /// * `evidence_url` - Optional URL to compliance evidence
    /// * `expires_at` - Optional expiry time (gate auto-disables after this)
    pub async fn enable_gate(
        &self,
        name: &str,
        actor: &str,
        reason: &str,
        evidence_url: Option<&str>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<LaunchGate, AppError> {
        // Validate gate name exists
        let current_gate = self.gate_repo.get_gate(name).await?;
        if current_gate.is_none() {
            return Err(AppError::NotFound(format!("Gate '{}' not found", name)));
        }
        let current_gate = current_gate.unwrap();

        // Perform the update
        let gate = self
            .gate_repo
            .set_gate_state(name, true, actor, reason, evidence_url, expires_at)
            .await?;

        // Log to audit
        self.log_gate_change(
            &gate,
            "enable",
            actor,
            json!({
                "previous_state": {
                    "is_enabled": current_gate.is_enabled,
                    "disabled_reason": current_gate.disabled_reason,
                },
                "new_state": {
                    "is_enabled": true,
                    "reason": reason,
                    "evidence_url": evidence_url,
                    "expires_at": expires_at,
                }
            }),
        )
        .await;

        info!(
            gate = %name,
            actor = %actor,
            reason = %reason,
            expires_at = ?expires_at,
            "Launch gate enabled"
        );

        Ok(gate)
    }

    /// Disable a gate
    ///
    /// # Arguments
    /// * `name` - Gate name (e.g., "issuance", "marketplace")
    /// * `actor` - ID of the user/system disabling the gate
    /// * `reason` - Explanation for why the gate is being disabled
    pub async fn disable_gate(
        &self,
        name: &str,
        actor: &str,
        reason: &str,
    ) -> Result<LaunchGate, AppError> {
        // Validate gate name exists
        let current_gate = self.gate_repo.get_gate(name).await?;
        if current_gate.is_none() {
            return Err(AppError::NotFound(format!("Gate '{}' not found", name)));
        }
        let current_gate = current_gate.unwrap();

        // Perform the update
        let gate = self
            .gate_repo
            .set_gate_state(name, false, actor, reason, None, None)
            .await?;

        // Log to audit
        self.log_gate_change(
            &gate,
            "disable",
            actor,
            json!({
                "previous_state": {
                    "is_enabled": current_gate.is_enabled,
                    "enabled_reason": current_gate.enabled_reason,
                    "evidence_url": current_gate.evidence_url,
                },
                "new_state": {
                    "is_enabled": false,
                    "reason": reason,
                }
            }),
        )
        .await;

        info!(
            gate = %name,
            actor = %actor,
            reason = %reason,
            "Launch gate disabled"
        );

        Ok(gate)
    }

    /// Check and disable expired gates
    ///
    /// This should be called periodically (e.g., from a background job)
    /// to auto-disable gates that have passed their expiry time.
    ///
    /// Returns the number of gates that were auto-disabled.
    pub async fn check_gate_expiry(&self) -> Result<i64, AppError> {
        let count = self.gate_repo.check_gate_expiry().await?;

        if count > 0 {
            // Log auto-disable events to audit
            let event = AuditEvent::builder(AuditEventType::FeatureToggled, AuditCategory::System)
                .actor(ActorType::System, "system")
                .action("auto_disable_expired")
                .outcome(AuditOutcome::Success)
                .event_data(json!({
                    "gates_disabled": count,
                    "reason": "Scheduled expiry",
                }))
                .build();

            self.audit_repo.log_event(event).await.ok(); // Don't fail if audit logging fails

            info!(
                count = count,
                "Auto-disabled expired launch gates"
            );
        }

        Ok(count)
    }

    // ============================================
    // HELPER METHODS
    // ============================================

    /// Log a gate change to the audit log
    async fn log_gate_change(
        &self,
        gate: &LaunchGate,
        action: &str,
        actor: &str,
        event_data: serde_json::Value,
    ) {
        let event = AuditEvent::builder(AuditEventType::FeatureToggled, AuditCategory::System)
            .actor(ActorType::Admin, actor)
            .resource(RESOURCE_LAUNCH_GATE, gate.id.to_string())
            .action(action)
            .outcome(AuditOutcome::Success)
            .event_data(json!({
                "gate_name": gate.gate_name,
                "change": event_data,
            }))
            .build();

        let result = self.audit_repo.log_event(event).await;

        if let Err(e) = result {
            warn!(
                gate = %gate.gate_name,
                action = %action,
                error = %e,
                "Failed to log gate change to audit log"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_name_strings() {
        assert_eq!(GateName::Issuance.as_str(), "issuance");
        assert_eq!(GateName::Marketplace.as_str(), "marketplace");
        assert_eq!(GateName::Funding.as_str(), "funding");
        assert_eq!(GateName::ReferralPayments.as_str(), "referral_payments");
        assert_eq!(GateName::Campaigns.as_str(), "campaigns");
        assert_eq!(GateName::Uploads.as_str(), "uploads");
    }
}
