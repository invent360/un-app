//! Service Identity Service for machine-to-machine authentication
//!
//! Provides:
//! - Service authentication with HMAC verification
//! - Key rotation with zero-downtime grace periods
//! - Rate limiting by service tier
//! - Audit logging for security events

use std::net::IpAddr;
use std::sync::Arc;

use crate::server::repositories::{
    DynServiceIdentityRepository, ServiceIdentity, ServiceType, RateLimitTier,
    CreateServiceIdentityInput, RotateKeyInput, ServiceIdentityAudit,
    DynImmutableAuditRepository, AuditEventBuilder, AuditCategory, AuditEventType, AuditOutcome, ActorType,
};
use crate::types::AppError;

/// Result of service authentication
#[derive(Debug, Clone)]
pub struct AuthenticatedService {
    pub service_id: String,
    pub service_name: String,
    pub service_type: ServiceType,
    pub roles: Vec<String>,
    pub allowed_scopes: Vec<String>,
    pub rate_limit_tier: RateLimitTier,
}

impl From<ServiceIdentity> for AuthenticatedService {
    fn from(identity: ServiceIdentity) -> Self {
        let rate_limit_tier = identity.rate_limit();
        Self {
            service_id: identity.service_id,
            service_name: identity.service_name,
            service_type: ServiceType::from_str(&identity.service_type).unwrap_or(ServiceType::External),
            roles: identity.roles,
            allowed_scopes: identity.allowed_scopes,
            rate_limit_tier,
        }
    }
}

/// Service Identity Service
#[derive(Clone)]
pub struct ServiceIdentityService {
    repo: DynServiceIdentityRepository,
    audit_repo: DynImmutableAuditRepository,
}

impl ServiceIdentityService {
    pub fn new(
        repo: DynServiceIdentityRepository,
        audit_repo: DynImmutableAuditRepository,
    ) -> Self {
        Self { repo, audit_repo }
    }

    /// Authenticate a service using service_id and secret_key
    ///
    /// Returns the authenticated service identity or an error.
    pub async fn authenticate(
        &self,
        service_id: &str,
        secret_key: &[u8],
        ip_address: Option<IpAddr>,
    ) -> Result<AuthenticatedService, AppError> {
        // Look up the service identity
        let identity = self.repo.get_by_service_id(service_id).await?
            .ok_or_else(|| {
                tracing::warn!(service_id = %service_id, "Unknown service identity");
                AppError::Unauthorized("Unknown service identity".to_string())
            })?;

        // Check if suspended
        if identity.suspended_at.is_some() {
            tracing::warn!(service_id = %service_id, "Service identity is suspended");
            return Err(AppError::Unauthorized("Service identity is suspended".to_string()));
        }

        // Verify the secret key
        if !identity.verify_key(secret_key) {
            // Log failed authentication attempt
            let _ = self.audit_repo.log_event(
                AuditEventBuilder::new(
                    AuditEventType::AuthFailure,
                    AuditCategory::Security,
                )
                .actor(ActorType::Service, service_id.to_string())
                .action("authenticate")
                .outcome(AuditOutcome::Failure)
                .ip_address(ip_address.map(|ip| ip.to_string()))
                .event_data(serde_json::json!({
                    "reason": "invalid_key"
                }))
                .build()
            ).await;

            tracing::warn!(service_id = %service_id, "Invalid secret key");
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // Update usage statistics
        let _ = self.repo.record_usage(service_id).await;

        tracing::debug!(service_id = %service_id, "Service authenticated successfully");

        Ok(identity.into())
    }

    /// Check if a service has exceeded its rate limit
    pub async fn check_rate_limit(
        &self,
        service_id: &str,
        tier: RateLimitTier,
    ) -> Result<bool, AppError> {
        let requests_per_minute = match tier.requests_per_minute() {
            Some(limit) => limit,
            None => return Ok(true), // Unlimited
        };

        let count = self.repo.get_request_count(service_id, 60).await?;

        Ok(count < requests_per_minute as i64)
    }

    /// Log a request for rate limiting and monitoring
    pub async fn log_request(
        &self,
        service_id: &str,
        endpoint: &str,
        method: &str,
        status_code: i32,
        response_time_ms: i32,
        ip_address: Option<IpAddr>,
    ) -> Result<(), AppError> {
        self.repo.log_request(
            service_id,
            endpoint,
            method,
            status_code,
            response_time_ms,
            ip_address,
        ).await
    }

    /// Create a new service identity
    pub async fn create_identity(
        &self,
        input: CreateServiceIdentityInput,
    ) -> Result<ServiceIdentity, AppError> {
        let identity = self.repo.create(input.clone()).await?;

        // Audit log
        let _ = self.audit_repo.log_event(
            AuditEventBuilder::new(
                AuditEventType::Custom("service_identity.created".to_string()),
                AuditCategory::System,
            )
            .actor(ActorType::Admin, input.created_by.unwrap_or_default())
            .resource("service_identity", &identity.service_id)
            .action("create")
            .outcome(AuditOutcome::Success)
            .event_data(serde_json::json!({
                "service_type": input.service_type.as_str(),
                "roles": input.roles,
            }))
            .build()
        ).await;

        tracing::info!(
            service_id = %identity.service_id,
            service_type = %identity.service_type,
            "Service identity created"
        );

        Ok(identity)
    }

    /// Rotate the secret key for a service
    pub async fn rotate_key(
        &self,
        service_id: &str,
        new_secret_key: &[u8],
        grace_period_hours: i64,
        rotated_by: &str,
    ) -> Result<ServiceIdentity, AppError> {
        let identity = self.repo.rotate_key(RotateKeyInput {
            service_id: service_id.to_string(),
            new_secret_key: new_secret_key.to_vec(),
            grace_period_hours,
            rotated_by: rotated_by.to_string(),
        }).await?;

        // Audit log
        let _ = self.audit_repo.log_event(
            AuditEventBuilder::new(
                AuditEventType::Custom("service_identity.key_rotated".to_string()),
                AuditCategory::Security,
            )
            .actor(ActorType::Admin, rotated_by.to_string())
            .resource("service_identity", service_id)
            .action("rotate_key")
            .outcome(AuditOutcome::Success)
            .event_data(serde_json::json!({
                "grace_period_hours": grace_period_hours,
                "new_key_hint": identity.active_key_hint,
            }))
            .build()
        ).await;

        Ok(identity)
    }

    /// Suspend a service identity
    pub async fn suspend_service(
        &self,
        service_id: &str,
        reason: &str,
        suspended_by: &str,
    ) -> Result<(), AppError> {
        self.repo.suspend(service_id, reason, suspended_by).await?;

        // Audit log
        let _ = self.audit_repo.log_event(
            AuditEventBuilder::new(
                AuditEventType::Custom("service_identity.suspended".to_string()),
                AuditCategory::Security,
            )
            .actor(ActorType::Admin, suspended_by.to_string())
            .resource("service_identity", service_id)
            .action("suspend")
            .outcome(AuditOutcome::Success)
            .event_data(serde_json::json!({ "reason": reason }))
            .build()
        ).await;

        Ok(())
    }

    /// Activate a suspended service identity
    pub async fn activate_service(
        &self,
        service_id: &str,
        activated_by: &str,
    ) -> Result<(), AppError> {
        self.repo.activate(service_id, activated_by).await?;

        // Audit log
        let _ = self.audit_repo.log_event(
            AuditEventBuilder::new(
                AuditEventType::Custom("service_identity.activated".to_string()),
                AuditCategory::Security,
            )
            .actor(ActorType::Admin, activated_by.to_string())
            .resource("service_identity", service_id)
            .action("activate")
            .outcome(AuditOutcome::Success)
            .build()
        ).await;

        Ok(())
    }

    /// List all active service identities
    pub async fn list_identities(&self) -> Result<Vec<ServiceIdentity>, AppError> {
        self.repo.list_active().await
    }

    /// Get audit history for a service
    pub async fn get_audit_history(
        &self,
        service_id: &str,
        limit: i32,
    ) -> Result<Vec<ServiceIdentityAudit>, AppError> {
        self.repo.get_audit_history(service_id, limit).await
    }

    /// Cleanup old request logs
    pub async fn cleanup_request_logs(&self, older_than_hours: i32) -> Result<i64, AppError> {
        self.repo.cleanup_request_logs(older_than_hours).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authenticated_service_from_identity() {
        // Basic conversion test - would need mock for full test
    }
}
