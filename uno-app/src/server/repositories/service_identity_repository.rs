//! Service identity repository for machine-to-machine authentication
//!
//! Manages persistent service identities with:
//! - HMAC key storage (hashed, never plaintext)
//! - Key rotation support with grace period
//! - Role-based access control
//! - Request logging and rate limit tracking

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use sqlx::FromRow;
use std::net::IpAddr;
use std::sync::Arc;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Service type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceType {
    Internal,   // Internal services (uno-admin, sync workers)
    External,   // External integrations
    Worker,     // Background workers
}

impl ServiceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::External => "external",
            Self::Worker => "worker",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "internal" => Some(Self::Internal),
            "external" => Some(Self::External),
            "worker" => Some(Self::Worker),
            _ => None,
        }
    }
}

/// Rate limit tier for services
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitTier {
    Low,        // 10 req/min
    Standard,   // 100 req/min
    High,       // 1000 req/min
    Unlimited,  // No limit
}

impl RateLimitTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Standard => "standard",
            Self::High => "high",
            Self::Unlimited => "unlimited",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "low" => Some(Self::Low),
            "standard" => Some(Self::Standard),
            "high" => Some(Self::High),
            "unlimited" => Some(Self::Unlimited),
            _ => None,
        }
    }

    pub fn requests_per_minute(&self) -> Option<u32> {
        match self {
            Self::Low => Some(10),
            Self::Standard => Some(100),
            Self::High => Some(1000),
            Self::Unlimited => None,
        }
    }
}

/// Service identity entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ServiceIdentity {
    pub id: i64,
    pub service_id: String,
    pub service_name: String,
    pub service_type: String,

    // Keys (hashed)
    pub active_key_hash: String,
    pub active_key_hint: String,
    pub active_key_created_at: DateTime<Utc>,
    pub previous_key_hash: Option<String>,
    pub previous_key_hint: Option<String>,
    pub previous_key_created_at: Option<DateTime<Utc>>,
    pub previous_key_expires_at: Option<DateTime<Utc>>,

    // Authorization
    pub roles: Vec<String>,
    pub allowed_scopes: Vec<String>,
    pub rate_limit_tier: String,

    // Status
    pub is_active: bool,
    pub suspended_at: Option<DateTime<Utc>>,
    pub suspended_reason: Option<String>,

    // Audit
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub total_requests: i64,

    // Metadata
    pub description: Option<String>,
}

impl ServiceIdentity {
    /// Check if a secret key matches the active or previous key
    pub fn verify_key(&self, secret_key: &[u8]) -> bool {
        let key_hash = hash_key(secret_key);

        // Check active key
        if self.active_key_hash == key_hash {
            return true;
        }

        // Check previous key if within grace period
        if let (Some(prev_hash), Some(expires_at)) = (&self.previous_key_hash, self.previous_key_expires_at) {
            if Utc::now() < expires_at && prev_hash == &key_hash {
                return true;
            }
        }

        false
    }

    /// Check if the service has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// Check if the service has a specific scope
    pub fn has_scope(&self, scope: &str) -> bool {
        self.allowed_scopes.iter().any(|s| s == scope || s == "*")
    }

    /// Get the rate limit tier enum
    pub fn rate_limit(&self) -> RateLimitTier {
        RateLimitTier::from_str(&self.rate_limit_tier).unwrap_or(RateLimitTier::Standard)
    }
}

/// Input for creating a service identity
#[derive(Debug, Clone)]
pub struct CreateServiceIdentityInput {
    pub service_id: String,
    pub service_name: String,
    pub service_type: ServiceType,
    pub secret_key: Vec<u8>,
    pub roles: Vec<String>,
    pub allowed_scopes: Vec<String>,
    pub rate_limit_tier: RateLimitTier,
    pub description: Option<String>,
    pub created_by: Option<String>,
}

/// Input for rotating a service key
#[derive(Debug, Clone)]
pub struct RotateKeyInput {
    pub service_id: String,
    pub new_secret_key: Vec<u8>,
    pub grace_period_hours: i64,
    pub rotated_by: String,
}

/// Service identity audit entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ServiceIdentityAudit {
    pub id: i64,
    pub service_id: String,
    pub action: String,
    pub actor: Option<String>,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Hash a secret key using SHA256
pub fn hash_key(key: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key);
    hex::encode(hasher.finalize())
}

/// Get hint (first 8 chars of hash) for key identification
pub fn key_hint(key_hash: &str) -> String {
    key_hash.chars().take(8).collect()
}

// ============================================
// TRAIT DEFINITION
// ============================================

pub type DynServiceIdentityRepository = Arc<dyn ServiceIdentityRepository + Send + Sync>;

#[async_trait]
pub trait ServiceIdentityRepository: Send + Sync {
    /// Create a new service identity
    async fn create(&self, input: CreateServiceIdentityInput) -> Result<ServiceIdentity, AppError>;

    /// Get a service identity by service_id
    async fn get_by_service_id(&self, service_id: &str) -> Result<Option<ServiceIdentity>, AppError>;

    /// List all active service identities
    async fn list_active(&self) -> Result<Vec<ServiceIdentity>, AppError>;

    /// Rotate the secret key for a service
    async fn rotate_key(&self, input: RotateKeyInput) -> Result<ServiceIdentity, AppError>;

    /// Suspend a service identity
    async fn suspend(&self, service_id: &str, reason: &str, suspended_by: &str) -> Result<(), AppError>;

    /// Activate a suspended service identity
    async fn activate(&self, service_id: &str, activated_by: &str) -> Result<(), AppError>;

    /// Update last_used_at and increment total_requests
    async fn record_usage(&self, service_id: &str) -> Result<(), AppError>;

    /// Log an audit event for a service identity
    async fn log_audit(
        &self,
        service_id: &str,
        action: &str,
        actor: Option<&str>,
        details: Option<serde_json::Value>,
        ip_address: Option<IpAddr>,
    ) -> Result<(), AppError>;

    /// Get audit history for a service
    async fn get_audit_history(&self, service_id: &str, limit: i32) -> Result<Vec<ServiceIdentityAudit>, AppError>;

    /// Log a request for rate limiting
    async fn log_request(
        &self,
        service_id: &str,
        endpoint: &str,
        method: &str,
        status_code: i32,
        response_time_ms: i32,
        ip_address: Option<IpAddr>,
    ) -> Result<(), AppError>;

    /// Get request count in the last N seconds
    async fn get_request_count(&self, service_id: &str, window_secs: i64) -> Result<i64, AppError>;

    /// Cleanup old request logs
    async fn cleanup_request_logs(&self, older_than_hours: i32) -> Result<i64, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct ServiceIdentityRepositoryImpl {
    pool: ConnectionPool,
}

impl ServiceIdentityRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServiceIdentityRepository for ServiceIdentityRepositoryImpl {
    async fn create(&self, input: CreateServiceIdentityInput) -> Result<ServiceIdentity, AppError> {
        let key_hash = hash_key(&input.secret_key);
        let hint = key_hint(&key_hash);

        let result = sqlx::query_as::<_, ServiceIdentity>(
            r#"
            INSERT INTO service_identities (
                service_id, service_name, service_type,
                active_key_hash, active_key_hint, active_key_created_at,
                roles, allowed_scopes, rate_limit_tier,
                description, created_by, is_active
            )
            VALUES ($1, $2, $3, $4, $5, NOW(), $6, $7, $8, $9, $10, TRUE)
            RETURNING id, service_id, service_name, service_type,
                      active_key_hash, active_key_hint, active_key_created_at,
                      previous_key_hash, previous_key_hint, previous_key_created_at, previous_key_expires_at,
                      roles, allowed_scopes, rate_limit_tier,
                      is_active, suspended_at, suspended_reason,
                      created_at, updated_at, created_by, last_used_at, total_requests,
                      description
            "#,
        )
        .bind(&input.service_id)
        .bind(&input.service_name)
        .bind(input.service_type.as_str())
        .bind(&key_hash)
        .bind(&hint)
        .bind(&input.roles)
        .bind(&input.allowed_scopes)
        .bind(input.rate_limit_tier.as_str())
        .bind(input.description.as_deref())
        .bind(input.created_by.as_deref())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                AppError::Conflict(format!("Service identity '{}' already exists", input.service_id))
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        // Log audit event
        let _ = self.log_audit(
            &input.service_id,
            "created",
            input.created_by.as_deref(),
            Some(serde_json::json!({
                "service_type": input.service_type.as_str(),
                "roles": input.roles,
            })),
            None,
        ).await;

        Ok(result)
    }

    async fn get_by_service_id(&self, service_id: &str) -> Result<Option<ServiceIdentity>, AppError> {
        let result = sqlx::query_as::<_, ServiceIdentity>(
            r#"
            SELECT id, service_id, service_name, service_type,
                   active_key_hash, active_key_hint, active_key_created_at,
                   previous_key_hash, previous_key_hint, previous_key_created_at, previous_key_expires_at,
                   roles, allowed_scopes, rate_limit_tier,
                   is_active, suspended_at, suspended_reason,
                   created_at, updated_at, created_by, last_used_at, total_requests,
                   description
            FROM service_identities
            WHERE service_id = $1 AND is_active = TRUE
            "#,
        )
        .bind(service_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn list_active(&self) -> Result<Vec<ServiceIdentity>, AppError> {
        let results = sqlx::query_as::<_, ServiceIdentity>(
            r#"
            SELECT id, service_id, service_name, service_type,
                   active_key_hash, active_key_hint, active_key_created_at,
                   previous_key_hash, previous_key_hint, previous_key_created_at, previous_key_expires_at,
                   roles, allowed_scopes, rate_limit_tier,
                   is_active, suspended_at, suspended_reason,
                   created_at, updated_at, created_by, last_used_at, total_requests,
                   description
            FROM service_identities
            WHERE is_active = TRUE
            ORDER BY service_name
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn rotate_key(&self, input: RotateKeyInput) -> Result<ServiceIdentity, AppError> {
        let new_key_hash = hash_key(&input.new_secret_key);
        let new_hint = key_hint(&new_key_hash);

        let result = sqlx::query_as::<_, ServiceIdentity>(
            r#"
            UPDATE service_identities
            SET previous_key_hash = active_key_hash,
                previous_key_hint = active_key_hint,
                previous_key_created_at = active_key_created_at,
                previous_key_expires_at = NOW() + ($2 || ' hours')::INTERVAL,
                active_key_hash = $3,
                active_key_hint = $4,
                active_key_created_at = NOW()
            WHERE service_id = $1 AND is_active = TRUE
            RETURNING id, service_id, service_name, service_type,
                      active_key_hash, active_key_hint, active_key_created_at,
                      previous_key_hash, previous_key_hint, previous_key_created_at, previous_key_expires_at,
                      roles, allowed_scopes, rate_limit_tier,
                      is_active, suspended_at, suspended_reason,
                      created_at, updated_at, created_by, last_used_at, total_requests,
                      description
            "#,
        )
        .bind(&input.service_id)
        .bind(input.grace_period_hours.to_string())
        .bind(&new_key_hash)
        .bind(&new_hint)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Log audit event
        let _ = self.log_audit(
            &input.service_id,
            "key_rotated",
            Some(&input.rotated_by),
            Some(serde_json::json!({
                "new_key_hint": new_hint,
                "grace_period_hours": input.grace_period_hours,
            })),
            None,
        ).await;

        tracing::info!(
            service_id = %input.service_id,
            new_key_hint = %new_hint,
            grace_period_hours = input.grace_period_hours,
            "Service key rotated"
        );

        Ok(result)
    }

    async fn suspend(&self, service_id: &str, reason: &str, suspended_by: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE service_identities
            SET suspended_at = NOW(),
                suspended_reason = $2
            WHERE service_id = $1 AND is_active = TRUE
            "#,
        )
        .bind(service_id)
        .bind(reason)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let _ = self.log_audit(
            service_id,
            "suspended",
            Some(suspended_by),
            Some(serde_json::json!({ "reason": reason })),
            None,
        ).await;

        tracing::warn!(service_id = %service_id, reason = %reason, "Service identity suspended");

        Ok(())
    }

    async fn activate(&self, service_id: &str, activated_by: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE service_identities
            SET suspended_at = NULL,
                suspended_reason = NULL
            WHERE service_id = $1 AND is_active = TRUE
            "#,
        )
        .bind(service_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let _ = self.log_audit(
            service_id,
            "activated",
            Some(activated_by),
            None,
            None,
        ).await;

        tracing::info!(service_id = %service_id, "Service identity activated");

        Ok(())
    }

    async fn record_usage(&self, service_id: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE service_identities
            SET last_used_at = NOW(),
                total_requests = total_requests + 1
            WHERE service_id = $1
            "#,
        )
        .bind(service_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn log_audit(
        &self,
        service_id: &str,
        action: &str,
        actor: Option<&str>,
        details: Option<serde_json::Value>,
        ip_address: Option<IpAddr>,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO service_identity_audit (service_id, action, actor, details, ip_address)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(service_id)
        .bind(action)
        .bind(actor)
        .bind(details)
        .bind(ip_address.map(|ip| ip.to_string()))
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_audit_history(&self, service_id: &str, limit: i32) -> Result<Vec<ServiceIdentityAudit>, AppError> {
        let results = sqlx::query_as::<_, ServiceIdentityAudit>(
            r#"
            SELECT id, service_id, action, actor, details,
                   ip_address::TEXT as ip_address, created_at
            FROM service_identity_audit
            WHERE service_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(service_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn log_request(
        &self,
        service_id: &str,
        endpoint: &str,
        method: &str,
        status_code: i32,
        response_time_ms: i32,
        ip_address: Option<IpAddr>,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO service_request_log (service_id, endpoint, method, status_code, response_time_ms, ip_address)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(service_id)
        .bind(endpoint)
        .bind(method)
        .bind(status_code)
        .bind(response_time_ms)
        .bind(ip_address.map(|ip| ip.to_string()))
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_request_count(&self, service_id: &str, window_secs: i64) -> Result<i64, AppError> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM service_request_log
            WHERE service_id = $1
              AND created_at > NOW() - ($2 || ' seconds')::INTERVAL
            "#,
        )
        .bind(service_id)
        .bind(window_secs.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.0)
    }

    async fn cleanup_request_logs(&self, older_than_hours: i32) -> Result<i64, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM service_request_log
            WHERE created_at < NOW() - ($1 || ' hours')::INTERVAL
            "#,
        )
        .bind(older_than_hours.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_key() {
        let key = b"my-secret-key";
        let hash = hash_key(key);
        assert_eq!(hash.len(), 64); // SHA256 = 32 bytes = 64 hex chars
    }

    #[test]
    fn test_key_hint() {
        let hash = "abcdef1234567890abcdef1234567890";
        let hint = key_hint(hash);
        assert_eq!(hint, "abcdef12");
    }

    #[test]
    fn test_rate_limit_tier() {
        assert_eq!(RateLimitTier::Low.requests_per_minute(), Some(10));
        assert_eq!(RateLimitTier::Standard.requests_per_minute(), Some(100));
        assert_eq!(RateLimitTier::High.requests_per_minute(), Some(1000));
        assert_eq!(RateLimitTier::Unlimited.requests_per_minute(), None);
    }

    #[test]
    fn test_service_type() {
        assert_eq!(ServiceType::Internal.as_str(), "internal");
        assert_eq!(ServiceType::from_str("external"), Some(ServiceType::External));
    }
}
