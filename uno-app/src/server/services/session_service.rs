//! Session service for identity and session management
//!
//! Provides business logic for:
//! - User identity management (find/create from JWT 'sub' claim)
//! - Session creation and revocation (logout)
//! - Session blacklist management
//! - Activity tracking and cleanup
//!
//! This integrates with the existing JWT-based SessionVerifier in uno-api.

use chrono::{DateTime, Duration, Utc};
use serde_json::json;
use std::net::IpAddr;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::server::repositories::{
    ActorType, AuditCategory, AuditEvent, AuditEventType, AuditOutcome,
    DynImmutableAuditRepository, DynSessionRepository, Session, SessionMetadata, UserIdentity,
    hash_token,
};
use crate::types::AppError;

/// Session service for managing user identities and sessions
#[derive(Clone)]
pub struct SessionService {
    session_repo: DynSessionRepository,
    audit_repo: DynImmutableAuditRepository,
}

impl SessionService {
    /// Create a new session service
    pub fn new(
        session_repo: DynSessionRepository,
        audit_repo: DynImmutableAuditRepository,
    ) -> Self {
        Self {
            session_repo,
            audit_repo,
        }
    }

    // ============================================
    // IDENTITY METHODS
    // ============================================

    /// Find or create a user identity from JWT claims
    ///
    /// This should be called when a user logs in successfully.
    /// If the identity doesn't exist, a new one is created.
    pub async fn find_or_create_identity(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<UserIdentity, AppError> {
        let identity = self
            .session_repo
            .find_or_create_user_identity(provider, provider_user_id)
            .await?;

        // Log successful login
        self.log_auth_event(
            "login",
            &identity,
            AuditOutcome::Success,
            json!({
                "provider": provider,
                "is_new_identity": identity.created_at == identity.updated_at,
            }),
            None,
            None,
        )
        .await;

        info!(
            user_id = %identity.id,
            provider = %provider,
            "User identity found or created"
        );

        Ok(identity)
    }

    /// Get a user identity by internal ID
    pub async fn get_identity(&self, id: Uuid) -> Result<Option<UserIdentity>, AppError> {
        self.session_repo.get_user_by_id(id).await
    }

    /// Check if a user identity is active and not suspended
    pub fn is_identity_usable(identity: &UserIdentity) -> Result<(), AppError> {
        if !identity.is_active {
            return Err(AppError::Forbidden("Account is deactivated".to_string()));
        }
        if identity.is_suspended {
            let reason = identity
                .suspension_reason
                .as_deref()
                .unwrap_or("Account suspended");
            return Err(AppError::Forbidden(reason.to_string()));
        }
        Ok(())
    }

    /// Suspend a user identity
    pub async fn suspend_identity(
        &self,
        id: Uuid,
        reason: &str,
        actor: &str,
    ) -> Result<UserIdentity, AppError> {
        let mut identity = self
            .session_repo
            .get_user_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("User identity not found".to_string()))?;

        identity.is_suspended = true;
        identity.suspension_reason = Some(reason.to_string());
        identity.suspended_at = Some(Utc::now());
        identity.suspended_by = Some(actor.to_string());

        let updated = self.session_repo.update_user_identity(&identity).await?;

        // Revoke all sessions for this user
        let sessions_revoked = self
            .session_repo
            .revoke_all_user_sessions(id, "account_suspended", Some(actor))
            .await?;

        // Log suspension
        self.log_auth_event(
            "suspend",
            &updated,
            AuditOutcome::Success,
            json!({
                "reason": reason,
                "sessions_revoked": sessions_revoked,
            }),
            None,
            Some(actor),
        )
        .await;

        info!(
            user_id = %id,
            actor = %actor,
            sessions_revoked = sessions_revoked,
            "User identity suspended and sessions revoked"
        );

        Ok(updated)
    }

    // ============================================
    // SESSION METHODS
    // ============================================

    /// Create a session record for an authenticated JWT
    ///
    /// Called after JWT validation to track the session for revocation support.
    pub async fn create_session(
        &self,
        user_id: Uuid,
        token: &str,
        issued_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
        user_agent: Option<String>,
        ip_address: Option<IpAddr>,
        device_fingerprint: Option<String>,
    ) -> Result<Session, AppError> {
        let token_hash = hash_token(token);
        let metadata = SessionMetadata {
            user_agent,
            ip_address,
            device_fingerprint,
        };

        let session = self
            .session_repo
            .create_session(user_id, &token_hash, issued_at, expires_at, metadata)
            .await?;

        info!(
            session_id = %session.id,
            user_id = %user_id,
            "Session created"
        );

        Ok(session)
    }

    /// Check if a token is valid (not blacklisted and not revoked)
    ///
    /// This should be called during authentication to verify the session is still valid.
    pub async fn is_session_valid(&self, token: &str) -> Result<bool, AppError> {
        let token_hash = hash_token(token);

        // Check blacklist first (fast path)
        if self.session_repo.is_token_blacklisted(&token_hash).await? {
            return Ok(false);
        }

        // Check session record if exists
        if let Some(session) = self.session_repo.get_session(&token_hash).await? {
            if session.is_revoked {
                return Ok(false);
            }
            if session.expires_at < Utc::now() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Update session activity timestamp
    ///
    /// Call this periodically (e.g., every few minutes) to track active sessions.
    pub async fn touch_session(&self, token: &str) -> Result<(), AppError> {
        let token_hash = hash_token(token);
        self.session_repo.update_session_activity(&token_hash).await
    }

    /// Logout - revoke a specific session
    ///
    /// This adds the token to the blacklist and marks the session as revoked.
    pub async fn logout(
        &self,
        token: &str,
        user_id: Uuid,
        ip_address: Option<IpAddr>,
    ) -> Result<(), AppError> {
        let token_hash = hash_token(token);

        // Revoke the session
        let revoked = self
            .session_repo
            .revoke_session(&token_hash, "logout", None)
            .await?;

        // Also add to blacklist for immediate rejection
        // Set expiry to 1 hour from now (max JWT lifetime)
        let blacklist_expires = Utc::now() + Duration::hours(1);
        self.session_repo
            .add_to_blacklist(&token_hash, blacklist_expires, "logout", None)
            .await?;

        if revoked {
            info!(user_id = %user_id, "User logged out successfully");
        }

        // Log logout event
        if let Some(identity) = self.session_repo.get_user_by_id(user_id).await? {
            self.log_auth_event(
                "logout",
                &identity,
                AuditOutcome::Success,
                json!({
                    "session_revoked": revoked,
                }),
                ip_address,
                None,
            )
            .await;
        }

        Ok(())
    }

    /// Revoke all sessions for a user (e.g., "logout everywhere")
    pub async fn revoke_all_sessions(
        &self,
        user_id: Uuid,
        reason: &str,
        actor: Option<&str>,
    ) -> Result<i64, AppError> {
        let count = self
            .session_repo
            .revoke_all_user_sessions(user_id, reason, actor)
            .await?;

        if let Some(identity) = self.session_repo.get_user_by_id(user_id).await? {
            self.log_auth_event(
                "revoke_all_sessions",
                &identity,
                AuditOutcome::Success,
                json!({
                    "sessions_revoked": count,
                    "reason": reason,
                }),
                None,
                actor,
            )
            .await;
        }

        info!(
            user_id = %user_id,
            sessions_revoked = count,
            reason = %reason,
            "All user sessions revoked"
        );

        Ok(count)
    }

    /// Admin: Revoke a specific session by token hash
    pub async fn admin_revoke_session(
        &self,
        token_hash: &str,
        reason: &str,
        admin_actor: &str,
    ) -> Result<bool, AppError> {
        // Get session details first for logging
        let session = self.session_repo.get_session(token_hash).await?;

        let revoked = self
            .session_repo
            .revoke_session(token_hash, reason, Some(admin_actor))
            .await?;

        // Add to blacklist
        let blacklist_expires = Utc::now() + Duration::hours(1);
        self.session_repo
            .add_to_blacklist(token_hash, blacklist_expires, reason, Some(admin_actor))
            .await?;

        if let Some(session) = session {
            if let Some(identity) = self.session_repo.get_user_by_id(session.user_id).await? {
                self.log_auth_event(
                    "admin_revoke_session",
                    &identity,
                    AuditOutcome::Success,
                    json!({
                        "session_id": session.id,
                        "reason": reason,
                        "admin_actor": admin_actor,
                    }),
                    None,
                    Some(admin_actor),
                )
                .await;
            }
        }

        info!(
            admin = %admin_actor,
            reason = %reason,
            revoked = revoked,
            "Admin revoked session"
        );

        Ok(revoked)
    }

    // ============================================
    // BLACKLIST METHODS
    // ============================================

    /// Check if a token is blacklisted
    pub async fn is_token_blacklisted(&self, token: &str) -> Result<bool, AppError> {
        let token_hash = hash_token(token);
        self.session_repo.is_token_blacklisted(&token_hash).await
    }

    /// Add a token to the blacklist (for security revocation)
    pub async fn blacklist_token(
        &self,
        token: &str,
        reason: &str,
        blacklisted_by: Option<&str>,
    ) -> Result<(), AppError> {
        let token_hash = hash_token(token);
        let expires_at = Utc::now() + Duration::hours(1);

        self.session_repo
            .add_to_blacklist(&token_hash, expires_at, reason, blacklisted_by)
            .await?;

        info!(reason = %reason, "Token added to blacklist");
        Ok(())
    }

    // ============================================
    // CLEANUP METHODS
    // ============================================

    /// Clean up expired sessions and blacklist entries
    ///
    /// Should be called periodically from a background job.
    pub async fn cleanup_expired(&self) -> Result<(i64, i64), AppError> {
        let result = self.session_repo.cleanup_expired().await?;

        if result.sessions_cleaned > 0 || result.blacklist_cleaned > 0 {
            info!(
                sessions_cleaned = result.sessions_cleaned,
                blacklist_cleaned = result.blacklist_cleaned,
                "Cleaned up expired sessions and blacklist entries"
            );
        }

        Ok((result.sessions_cleaned, result.blacklist_cleaned))
    }

    // ============================================
    // HELPER METHODS
    // ============================================

    /// Log an authentication event to the audit log
    async fn log_auth_event(
        &self,
        action: &str,
        identity: &UserIdentity,
        outcome: AuditOutcome,
        event_data: serde_json::Value,
        ip_address: Option<IpAddr>,
        actor_override: Option<&str>,
    ) {
        // Map action to appropriate event type
        let event_type = match action {
            "login" => AuditEventType::Login,
            "logout" => AuditEventType::Logout,
            "suspend" | "admin_revoke_session" | "revoke_all_sessions" => AuditEventType::SessionRevoked,
            _ => AuditEventType::SessionCreated,
        };

        let mut builder = AuditEvent::builder(event_type, AuditCategory::Auth)
            .actor(
                ActorType::User,
                actor_override.unwrap_or(&identity.id.to_string()),
            )
            .resource("user_identity", identity.id.to_string())
            .action(action)
            .outcome(outcome)
            .event_data(json!({
                "provider": identity.provider,
                "details": event_data,
            }));

        // Only add IP if present
        if let Some(ip) = ip_address {
            builder = builder.actor_ip(ip);
        }

        let event = builder.build();

        if let Err(e) = self.audit_repo.log_event(event).await {
            warn!(
                action = %action,
                user_id = %identity.id,
                error = %e,
                "Failed to log auth event to audit log"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_identity_usable_active() {
        let identity = UserIdentity {
            id: Uuid::new_v4(),
            provider: "test".to_string(),
            provider_user_id: "user123".to_string(),
            display_name: None,
            email: None,
            email_verified: false,
            phone: None,
            phone_verified: false,
            is_active: true,
            is_suspended: false,
            suspension_reason: None,
            suspended_at: None,
            suspended_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login_at: None,
        };

        assert!(SessionService::is_identity_usable(&identity).is_ok());
    }

    #[test]
    fn test_is_identity_usable_inactive() {
        let identity = UserIdentity {
            id: Uuid::new_v4(),
            provider: "test".to_string(),
            provider_user_id: "user123".to_string(),
            display_name: None,
            email: None,
            email_verified: false,
            phone: None,
            phone_verified: false,
            is_active: false,
            is_suspended: false,
            suspension_reason: None,
            suspended_at: None,
            suspended_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login_at: None,
        };

        let result = SessionService::is_identity_usable(&identity);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_identity_usable_suspended() {
        let identity = UserIdentity {
            id: Uuid::new_v4(),
            provider: "test".to_string(),
            provider_user_id: "user123".to_string(),
            display_name: None,
            email: None,
            email_verified: false,
            phone: None,
            phone_verified: false,
            is_active: true,
            is_suspended: true,
            suspension_reason: Some("Violation of terms".to_string()),
            suspended_at: Some(Utc::now()),
            suspended_by: Some("admin".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login_at: None,
        };

        let result = SessionService::is_identity_usable(&identity);
        assert!(result.is_err());
    }
}
