//! Session repository for user identity and session management
//!
//! Implements CRUD operations for:
//! - User identities (linked to JWT 'sub' claim)
//! - Active sessions (create, revoke, check validity)
//! - Session blacklist (for fast JWT revocation checks)
//! - Cleanup functions for expired sessions/blacklist

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::FromRow;
use std::net::IpAddr;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// User identity linked to external provider via JWT 'sub' claim
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserIdentity {
    pub id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub is_active: bool,
    pub is_suspended: bool,
    pub suspension_reason: Option<String>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub suspended_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

/// Active session with revocation support
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token_hash: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub user_agent: Option<String>,
    pub ip_address: Option<IpAddr>,
    pub device_fingerprint: Option<String>,
    pub mfa_verified: bool,
    pub mfa_verified_at: Option<DateTime<Utc>>,
    pub is_revoked: bool,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_by: Option<String>,
    pub revocation_reason: Option<String>,
}

/// Session metadata for creating new sessions
#[derive(Debug, Clone, Default)]
pub struct SessionMetadata {
    pub user_agent: Option<String>,
    pub ip_address: Option<IpAddr>,
    pub device_fingerprint: Option<String>,
}

/// Blacklist entry for revoked tokens
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BlacklistEntry {
    pub token_hash: String,
    pub blacklisted_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub reason: String,
    pub blacklisted_by: Option<String>,
}

/// Result of cleanup operations
#[derive(Debug, Clone, Default)]
pub struct CleanupResult {
    pub sessions_cleaned: i64,
    pub blacklist_cleaned: i64,
}

// ============================================
// TRAIT DEFINITION
// ============================================

/// Dynamic type alias for SessionRepository trait object
pub type DynSessionRepository = Arc<dyn SessionRepository + Send + Sync>;

/// Session repository trait defining database operations
#[async_trait]
pub trait SessionRepository: Send + Sync {
    // --- User Identity Operations ---

    /// Find existing user identity or create a new one
    async fn find_or_create_user_identity(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<UserIdentity, AppError>;

    /// Get user identity by internal ID
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<UserIdentity>, AppError>;

    /// Update user identity fields
    async fn update_user_identity(&self, identity: &UserIdentity) -> Result<UserIdentity, AppError>;

    // --- Session Operations ---

    /// Create a new active session
    async fn create_session(
        &self,
        user_id: Uuid,
        token_hash: &str,
        issued_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
        metadata: SessionMetadata,
    ) -> Result<Session, AppError>;

    /// Get session by token hash
    async fn get_session(&self, token_hash: &str) -> Result<Option<Session>, AppError>;

    /// Revoke a specific session
    async fn revoke_session(
        &self,
        token_hash: &str,
        reason: &str,
        revoked_by: Option<&str>,
    ) -> Result<bool, AppError>;

    /// Revoke all sessions for a user
    async fn revoke_all_user_sessions(
        &self,
        user_id: Uuid,
        reason: &str,
        revoked_by: Option<&str>,
    ) -> Result<i64, AppError>;

    /// Update last activity timestamp for a session
    async fn update_session_activity(&self, token_hash: &str) -> Result<(), AppError>;

    // --- Blacklist Operations ---

    /// Check if a token is blacklisted
    async fn is_token_blacklisted(&self, token_hash: &str) -> Result<bool, AppError>;

    /// Add a token to the blacklist
    async fn add_to_blacklist(
        &self,
        token_hash: &str,
        expires_at: DateTime<Utc>,
        reason: &str,
        blacklisted_by: Option<&str>,
    ) -> Result<(), AppError>;

    // --- Cleanup Operations ---

    /// Clean up expired sessions and blacklist entries
    async fn cleanup_expired(&self) -> Result<CleanupResult, AppError>;
}

// ============================================
// HELPER FUNCTIONS
// ============================================

/// Hash a session token using SHA256
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

// ============================================
// IMPLEMENTATION
// ============================================

/// PostgreSQL implementation of SessionRepository
pub struct SessionRepositoryImpl {
    db_pool: ConnectionPool,
}

impl SessionRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    // --- User Identity Operations ---

    async fn find_or_create_user_identity(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<UserIdentity, AppError> {
        // Try to find existing identity first
        let existing = sqlx::query_as::<_, UserIdentity>(
            r#"
            SELECT id, provider, provider_user_id, display_name, email, email_verified,
                   phone, phone_verified, is_active, is_suspended, suspension_reason,
                   suspended_at, suspended_by, created_at, updated_at, last_login_at
            FROM user_identities
            WHERE provider = $1 AND provider_user_id = $2
            "#,
        )
        .bind(provider)
        .bind(provider_user_id)
        .fetch_optional(&self.db_pool)
        .await?;

        if let Some(identity) = existing {
            // Update last_login_at
            let updated = sqlx::query_as::<_, UserIdentity>(
                r#"
                UPDATE user_identities
                SET last_login_at = NOW(), updated_at = NOW()
                WHERE id = $1
                RETURNING id, provider, provider_user_id, display_name, email, email_verified,
                          phone, phone_verified, is_active, is_suspended, suspension_reason,
                          suspended_at, suspended_by, created_at, updated_at, last_login_at
                "#,
            )
            .bind(identity.id)
            .fetch_one(&self.db_pool)
            .await?;

            return Ok(updated);
        }

        // Create new identity
        let identity = sqlx::query_as::<_, UserIdentity>(
            r#"
            INSERT INTO user_identities (provider, provider_user_id, last_login_at)
            VALUES ($1, $2, NOW())
            RETURNING id, provider, provider_user_id, display_name, email, email_verified,
                      phone, phone_verified, is_active, is_suspended, suspension_reason,
                      suspended_at, suspended_by, created_at, updated_at, last_login_at
            "#,
        )
        .bind(provider)
        .bind(provider_user_id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(identity)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<UserIdentity>, AppError> {
        let identity = sqlx::query_as::<_, UserIdentity>(
            r#"
            SELECT id, provider, provider_user_id, display_name, email, email_verified,
                   phone, phone_verified, is_active, is_suspended, suspension_reason,
                   suspended_at, suspended_by, created_at, updated_at, last_login_at
            FROM user_identities
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(identity)
    }

    async fn update_user_identity(&self, identity: &UserIdentity) -> Result<UserIdentity, AppError> {
        let updated = sqlx::query_as::<_, UserIdentity>(
            r#"
            UPDATE user_identities
            SET display_name = $2,
                email = $3,
                email_verified = $4,
                phone = $5,
                phone_verified = $6,
                is_active = $7,
                is_suspended = $8,
                suspension_reason = $9,
                suspended_at = $10,
                suspended_by = $11,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, provider, provider_user_id, display_name, email, email_verified,
                      phone, phone_verified, is_active, is_suspended, suspension_reason,
                      suspended_at, suspended_by, created_at, updated_at, last_login_at
            "#,
        )
        .bind(identity.id)
        .bind(&identity.display_name)
        .bind(&identity.email)
        .bind(identity.email_verified)
        .bind(&identity.phone)
        .bind(identity.phone_verified)
        .bind(identity.is_active)
        .bind(identity.is_suspended)
        .bind(&identity.suspension_reason)
        .bind(identity.suspended_at)
        .bind(&identity.suspended_by)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(updated)
    }

    // --- Session Operations ---

    async fn create_session(
        &self,
        user_id: Uuid,
        token_hash: &str,
        issued_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
        metadata: SessionMetadata,
    ) -> Result<Session, AppError> {
        let session = sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO active_sessions
                (user_id, session_token_hash, issued_at, expires_at, user_agent, ip_address, device_fingerprint)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, session_token_hash, issued_at, expires_at, last_activity_at,
                      user_agent, ip_address, device_fingerprint, mfa_verified, mfa_verified_at,
                      is_revoked, revoked_at, revoked_by, revocation_reason
            "#,
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(issued_at)
        .bind(expires_at)
        .bind(&metadata.user_agent)
        .bind(metadata.ip_address)
        .bind(&metadata.device_fingerprint)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(session)
    }

    async fn get_session(&self, token_hash: &str) -> Result<Option<Session>, AppError> {
        let session = sqlx::query_as::<_, Session>(
            r#"
            SELECT id, user_id, session_token_hash, issued_at, expires_at, last_activity_at,
                   user_agent, ip_address, device_fingerprint, mfa_verified, mfa_verified_at,
                   is_revoked, revoked_at, revoked_by, revocation_reason
            FROM active_sessions
            WHERE session_token_hash = $1
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(session)
    }

    async fn revoke_session(
        &self,
        token_hash: &str,
        reason: &str,
        revoked_by: Option<&str>,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE active_sessions
            SET is_revoked = true,
                revoked_at = NOW(),
                revoked_by = $2,
                revocation_reason = $3
            WHERE session_token_hash = $1 AND is_revoked = false
            "#,
        )
        .bind(token_hash)
        .bind(revoked_by)
        .bind(reason)
        .execute(&self.db_pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn revoke_all_user_sessions(
        &self,
        user_id: Uuid,
        reason: &str,
        revoked_by: Option<&str>,
    ) -> Result<i64, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE active_sessions
            SET is_revoked = true,
                revoked_at = NOW(),
                revoked_by = $2,
                revocation_reason = $3
            WHERE user_id = $1 AND is_revoked = false
            "#,
        )
        .bind(user_id)
        .bind(revoked_by)
        .bind(reason)
        .execute(&self.db_pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }

    async fn update_session_activity(&self, token_hash: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE active_sessions
            SET last_activity_at = NOW()
            WHERE session_token_hash = $1 AND is_revoked = false
            "#,
        )
        .bind(token_hash)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    // --- Blacklist Operations ---

    async fn is_token_blacklisted(&self, token_hash: &str) -> Result<bool, AppError> {
        let result: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM session_blacklist
                WHERE token_hash = $1 AND expires_at > NOW()
            )
            "#,
        )
        .bind(token_hash)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(result.0)
    }

    async fn add_to_blacklist(
        &self,
        token_hash: &str,
        expires_at: DateTime<Utc>,
        reason: &str,
        blacklisted_by: Option<&str>,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO session_blacklist (token_hash, expires_at, reason, blacklisted_by)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (token_hash) DO UPDATE
            SET expires_at = EXCLUDED.expires_at,
                reason = EXCLUDED.reason,
                blacklisted_by = EXCLUDED.blacklisted_by,
                blacklisted_at = NOW()
            "#,
        )
        .bind(token_hash)
        .bind(expires_at)
        .bind(reason)
        .bind(blacklisted_by)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    // --- Cleanup Operations ---

    async fn cleanup_expired(&self) -> Result<CleanupResult, AppError> {
        // Mark expired sessions as revoked
        let sessions_result = sqlx::query(
            r#"
            UPDATE active_sessions
            SET is_revoked = true,
                revoked_at = NOW(),
                revocation_reason = 'expired'
            WHERE expires_at < NOW() AND is_revoked = false
            "#,
        )
        .execute(&self.db_pool)
        .await?;

        // Delete expired blacklist entries
        let blacklist_result = sqlx::query(
            r#"
            DELETE FROM session_blacklist WHERE expires_at < NOW()
            "#,
        )
        .execute(&self.db_pool)
        .await?;

        Ok(CleanupResult {
            sessions_cleaned: sessions_result.rows_affected() as i64,
            blacklist_cleaned: blacklist_result.rows_affected() as i64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_token() {
        let token = "test-jwt-token-12345";
        let hash = hash_token(token);

        // SHA256 produces 64 hex characters
        assert_eq!(hash.len(), 64);

        // Same input should produce same hash
        assert_eq!(hash, hash_token(token));

        // Different input should produce different hash
        assert_ne!(hash, hash_token("different-token"));
    }

    #[test]
    fn test_session_metadata_default() {
        let metadata = SessionMetadata::default();
        assert!(metadata.user_agent.is_none());
        assert!(metadata.ip_address.is_none());
        assert!(metadata.device_fingerprint.is_none());
    }
}
