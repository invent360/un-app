//! Consent service for privacy workflow management
//!
//! Business logic for:
//! - Recording and verifying user consents
//! - Processing data access requests (GDPR)
//! - Enforcing consent requirements

use std::net::IpAddr;
use uuid::Uuid;

use crate::server::repositories::{
    ConsentRepository, DynConsentRepository, RecordConsentInput, CreateDataRequestInput,
    UserConsentStatus, ConsentVersion, UserConsent, DataAccessRequest, DataRetentionPolicy,
    DynImmutableAuditRepository, AuditEventBuilder, AuditCategory, AuditEventType, AuditOutcome, ActorType,
};
use crate::types::AppError;

/// Consent service for managing user privacy workflows
#[derive(Clone)]
pub struct ConsentService {
    consent_repo: DynConsentRepository,
    audit_repo: DynImmutableAuditRepository,
}

impl ConsentService {
    pub fn new(
        consent_repo: DynConsentRepository,
        audit_repo: DynImmutableAuditRepository,
    ) -> Self {
        Self {
            consent_repo,
            audit_repo,
        }
    }

    /// Get all active consent versions that users need to agree to
    pub async fn get_required_consents(&self) -> Result<Vec<ConsentVersion>, AppError> {
        self.consent_repo.get_all_active_consent_versions().await
    }

    /// Get active consent version for a specific type
    pub async fn get_consent_version(
        &self,
        consent_type: &str,
    ) -> Result<Option<ConsentVersion>, AppError> {
        self.consent_repo.get_active_consent_version(consent_type).await
    }

    /// Record a user's consent decision
    pub async fn record_consent(
        &self,
        user_id: Uuid,
        consent_type: &str,
        consented: bool,
        consent_method: &str,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
        session_id: Option<Uuid>,
    ) -> Result<UserConsent, AppError> {
        // Get the active consent version
        let version = self
            .consent_repo
            .get_active_consent_version(consent_type)
            .await?
            .ok_or_else(|| AppError::NotFound(format!(
                "No active consent version for type: {}",
                consent_type
            )))?;

        // Check if declining a mandatory consent
        if version.is_mandatory && !consented {
            return Err(AppError::ValidationError(
                "Cannot decline mandatory consent".to_string(),
            ));
        }

        // Record the consent
        let consent = self.consent_repo.record_consent(RecordConsentInput {
            user_id,
            consent_version_id: version.id,
            consented,
            consent_method: consent_method.to_string(),
            ip_address,
            user_agent,
            session_id,
        }).await?;

        // Audit the consent
        let action = if consented { "granted" } else { "declined" };
        let mut builder = AuditEventBuilder::new(
            AuditEventType::ConsentRecorded,
            AuditCategory::Privacy,
        )
        .actor(ActorType::User, user_id.to_string())
        .resource("consent", consent_type)
        .action(action)
        .outcome(AuditOutcome::Success)
        .event_data(serde_json::json!({
            "version": version.version,
            "method": consent_method,
        }));

        if let Some(ip) = ip_address {
            builder = builder.actor_ip(ip);
        }

        let _ = self.audit_repo.log_event(builder.build()).await;

        Ok(consent)
    }

    /// Check if a user has given all required consents
    pub async fn check_required_consents(&self, user_id: Uuid) -> Result<UserConsentStatus, AppError> {
        self.consent_repo.get_user_consent_status(user_id).await
    }

    /// Verify user has required consents before allowing access
    pub async fn verify_consents(&self, user_id: Uuid) -> Result<(), AppError> {
        let status = self.check_required_consents(user_id).await?;

        if !status.has_required_consents {
            return Err(AppError::ConsentRequired {
                missing: status.pending_consent_types,
            });
        }

        Ok(())
    }

    /// Withdraw consent for a specific type
    pub async fn withdraw_consent(
        &self,
        user_id: Uuid,
        consent_type: &str,
        reason: Option<&str>,
        ip_address: Option<IpAddr>,
    ) -> Result<bool, AppError> {
        // Check if consent type allows withdrawal
        let version = self
            .consent_repo
            .get_active_consent_version(consent_type)
            .await?;

        if let Some(v) = &version {
            if v.is_mandatory {
                return Err(AppError::ValidationError(
                    "Cannot withdraw mandatory consent".to_string(),
                ));
            }
        }

        let withdrawn = self.consent_repo.withdraw_consent(user_id, consent_type, reason).await?;

        if withdrawn {
            // Audit the withdrawal
            let mut builder = AuditEventBuilder::new(
                AuditEventType::ConsentWithdrawn,
                AuditCategory::Privacy,
            )
            .actor(ActorType::User, user_id.to_string())
            .resource("consent", consent_type)
            .action("withdraw")
            .outcome(AuditOutcome::Success);

            if let Some(r) = reason {
                builder = builder.event_data(serde_json::json!({ "reason": r }));
            }

            if let Some(ip) = ip_address {
                builder = builder.actor_ip(ip);
            }

            let _ = self.audit_repo.log_event(builder.build()).await;
        }

        Ok(withdrawn)
    }

    /// Create a data access request (GDPR: export, delete, rectify, restrict)
    pub async fn create_data_request(
        &self,
        user_id: Uuid,
        request_type: &str,
        reason: Option<&str>,
        ip_address: Option<IpAddr>,
    ) -> Result<DataAccessRequest, AppError> {
        // Validate request type
        let valid_types = ["export", "delete", "rectify", "restrict"];
        if !valid_types.contains(&request_type) {
            return Err(AppError::ValidationError(format!(
                "Invalid request type: {}. Must be one of: {:?}",
                request_type, valid_types
            )));
        }

        // Check for existing pending request of same type
        let existing = self.consent_repo.get_user_data_requests(user_id).await?;
        let has_pending = existing.iter().any(|r| {
            r.request_type == request_type && (r.status == "pending" || r.status == "processing")
        });

        if has_pending {
            return Err(AppError::ValidationError(format!(
                "You already have a pending {} request",
                request_type
            )));
        }

        let request = self.consent_repo.create_data_request(CreateDataRequestInput {
            user_id,
            request_type: request_type.to_string(),
            request_reason: reason.map(|s| s.to_string()),
        }).await?;

        // Audit the request
        let mut builder = AuditEventBuilder::new(
            AuditEventType::DataRequestCreated,
            AuditCategory::Privacy,
        )
        .actor(ActorType::User, user_id.to_string())
        .resource("data_request", request.id.to_string())
        .action("create")
        .outcome(AuditOutcome::Success)
        .event_data(serde_json::json!({ "request_type": request_type }));

        if let Some(ip) = ip_address {
            builder = builder.actor_ip(ip);
        }

        let _ = self.audit_repo.log_event(builder.build()).await;

        tracing::info!(
            user_id = %user_id,
            request_id = %request.id,
            request_type = %request_type,
            "Data access request created"
        );

        Ok(request)
    }

    /// Get user's data access requests
    pub async fn get_user_data_requests(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<DataAccessRequest>, AppError> {
        self.consent_repo.get_user_data_requests(user_id).await
    }

    /// Get data request by ID (admin use)
    pub async fn get_data_request(&self, id: Uuid) -> Result<Option<DataAccessRequest>, AppError> {
        self.consent_repo.get_data_request(id).await
    }

    /// Process a data request (admin use)
    pub async fn process_data_request(
        &self,
        id: Uuid,
        status: &str,
        notes: Option<&str>,
        admin_id: &str,
    ) -> Result<DataAccessRequest, AppError> {
        let request = self.consent_repo.update_data_request_status(id, status, notes, admin_id).await?;

        // Audit the status change
        let builder = AuditEventBuilder::new(
            AuditEventType::DataRequestUpdated,
            AuditCategory::Privacy,
        )
        .actor(ActorType::Admin, admin_id.to_string())
        .resource("data_request", id.to_string())
        .action("update_status")
        .outcome(AuditOutcome::Success)
        .event_data(serde_json::json!({ "new_status": status }));

        let _ = self.audit_repo.log_event(builder.build()).await;

        Ok(request)
    }

    /// Complete a data request with optional export URL
    pub async fn complete_data_request(
        &self,
        id: Uuid,
        admin_id: &str,
        notes: Option<&str>,
        export_url: Option<&str>,
    ) -> Result<DataAccessRequest, AppError> {
        let request = self.consent_repo.complete_data_request(id, admin_id, notes, export_url).await?;

        // Audit completion
        let builder = AuditEventBuilder::new(
            AuditEventType::DataRequestCompleted,
            AuditCategory::Privacy,
        )
        .actor(ActorType::Admin, admin_id.to_string())
        .resource("data_request", id.to_string())
        .action("complete")
        .outcome(AuditOutcome::Success)
        .event_data(serde_json::json!({ "request_type": request.request_type }));

        let _ = self.audit_repo.log_event(builder.build()).await;

        tracing::info!(
            request_id = %id,
            request_type = %request.request_type,
            completed_by = %admin_id,
            "Data access request completed"
        );

        Ok(request)
    }

    /// Get all data retention policies
    pub async fn get_retention_policies(&self) -> Result<Vec<DataRetentionPolicy>, AppError> {
        self.consent_repo.get_retention_policies().await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_request_types() {
        let valid_types = ["export", "delete", "rectify", "restrict"];
        assert!(valid_types.contains(&"export"));
        assert!(valid_types.contains(&"delete"));
        assert!(!valid_types.contains(&"invalid"));
    }
}
