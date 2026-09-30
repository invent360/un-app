//! Consent repository for privacy and consent management
//!
//! Implements CRUD operations for:
//! - Consent versions (versioned consent documents)
//! - User consents (immutable consent records)
//! - Data access requests (GDPR subject requests)
//! - Data retention policies

use async_trait::async_trait;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::net::IpAddr;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Versioned consent document
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConsentVersion {
    pub id: i32,
    pub consent_type: String,
    pub version: String,
    pub title: String,
    pub content_hash: String,
    pub content_url: Option<String>,
    pub summary: Option<String>,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
    pub requires_explicit_consent: bool,
    pub is_mandatory: bool,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
}

/// User consent record (immutable)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserConsent {
    pub id: i64,
    pub user_id: Uuid,
    pub consent_version_id: i32,
    pub consented: bool,
    pub consent_method: String,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub session_id: Option<Uuid>,
    pub consented_at: DateTime<Utc>,
    pub withdrawn_at: Option<DateTime<Utc>>,
    pub withdrawal_reason: Option<String>,
}

/// Data access request (GDPR)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DataAccessRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub request_type: String,
    pub request_reason: Option<String>,
    pub status: String,
    pub assigned_to: Option<String>,
    pub processing_notes: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<String>,
    pub completion_notes: Option<String>,
    pub export_url: Option<String>,
    pub export_expires_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub requested_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub regulatory_deadline: Option<DateTime<Utc>>,
}

/// Data retention policy
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DataRetentionPolicy {
    pub id: i32,
    pub data_category: String,
    pub description: Option<String>,
    pub retention_days: i32,
    pub anonymize_after_days: Option<i32>,
    pub requires_consent: bool,
    pub legal_hold_exempt: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<String>,
}

/// Input for recording user consent
#[derive(Debug, Clone)]
pub struct RecordConsentInput {
    pub user_id: Uuid,
    pub consent_version_id: i32,
    pub consented: bool,
    pub consent_method: String,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub session_id: Option<Uuid>,
}

/// Input for creating a data access request
#[derive(Debug, Clone)]
pub struct CreateDataRequestInput {
    pub user_id: Uuid,
    pub request_type: String,
    pub request_reason: Option<String>,
}

/// User consent status summary
#[derive(Debug, Clone, Serialize)]
pub struct UserConsentStatus {
    pub user_id: Uuid,
    pub consents: Vec<ConsentSummary>,
    pub has_required_consents: bool,
    pub pending_consent_types: Vec<String>,
}

/// Summary of a single consent
#[derive(Debug, Clone, Serialize)]
pub struct ConsentSummary {
    pub consent_type: String,
    pub version: String,
    pub consented: bool,
    pub consented_at: DateTime<Utc>,
    pub is_mandatory: bool,
    pub is_current_version: bool,
}

// ============================================
// TRAIT DEFINITION
// ============================================

/// Dynamic type alias for ConsentRepository trait object
pub type DynConsentRepository = Arc<dyn ConsentRepository + Send + Sync>;

/// Consent repository trait defining database operations
#[async_trait]
pub trait ConsentRepository: Send + Sync {
    // --- Consent Version Operations ---

    /// Get active consent version for a type
    async fn get_active_consent_version(
        &self,
        consent_type: &str,
    ) -> Result<Option<ConsentVersion>, AppError>;

    /// Get all active consent versions
    async fn get_all_active_consent_versions(&self) -> Result<Vec<ConsentVersion>, AppError>;

    /// Get consent version by ID
    async fn get_consent_version_by_id(&self, id: i32) -> Result<Option<ConsentVersion>, AppError>;

    // --- User Consent Operations ---

    /// Record a user's consent decision (immutable)
    async fn record_consent(&self, input: RecordConsentInput) -> Result<UserConsent, AppError>;

    /// Get user's current consent for a specific type
    async fn get_user_consent(
        &self,
        user_id: Uuid,
        consent_type: &str,
    ) -> Result<Option<UserConsent>, AppError>;

    /// Get all consents for a user
    async fn get_user_consents(&self, user_id: Uuid) -> Result<Vec<UserConsent>, AppError>;

    /// Get user's consent status summary
    async fn get_user_consent_status(&self, user_id: Uuid) -> Result<UserConsentStatus, AppError>;

    /// Withdraw consent (records withdrawal, does not delete)
    async fn withdraw_consent(
        &self,
        user_id: Uuid,
        consent_type: &str,
        reason: Option<&str>,
    ) -> Result<bool, AppError>;

    // --- Data Access Request Operations ---

    /// Create a data access request
    async fn create_data_request(
        &self,
        input: CreateDataRequestInput,
    ) -> Result<DataAccessRequest, AppError>;

    /// Get data access request by ID
    async fn get_data_request(&self, id: Uuid) -> Result<Option<DataAccessRequest>, AppError>;

    /// Get all data requests for a user
    async fn get_user_data_requests(&self, user_id: Uuid) -> Result<Vec<DataAccessRequest>, AppError>;

    /// Update data request status
    async fn update_data_request_status(
        &self,
        id: Uuid,
        status: &str,
        notes: Option<&str>,
        updated_by: &str,
    ) -> Result<DataAccessRequest, AppError>;

    /// Complete a data request
    async fn complete_data_request(
        &self,
        id: Uuid,
        completed_by: &str,
        notes: Option<&str>,
        export_url: Option<&str>,
    ) -> Result<DataAccessRequest, AppError>;

    // --- Data Retention Operations ---

    /// Get all retention policies
    async fn get_retention_policies(&self) -> Result<Vec<DataRetentionPolicy>, AppError>;

    /// Get retention policy by category
    async fn get_retention_policy(
        &self,
        category: &str,
    ) -> Result<Option<DataRetentionPolicy>, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct ConsentRepositoryImpl {
    pool: ConnectionPool,
}

impl ConsentRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ConsentRepository for ConsentRepositoryImpl {
    async fn get_active_consent_version(
        &self,
        consent_type: &str,
    ) -> Result<Option<ConsentVersion>, AppError> {
        let version = sqlx::query_as::<_, ConsentVersion>(
            r#"
            SELECT id, consent_type, version, title, content_hash, content_url, summary,
                   effective_from, effective_to, requires_explicit_consent, is_mandatory,
                   created_at, created_by
            FROM consent_versions
            WHERE consent_type = $1
              AND effective_from <= NOW()
              AND (effective_to IS NULL OR effective_to > NOW())
            ORDER BY effective_from DESC
            LIMIT 1
            "#,
        )
        .bind(consent_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(version)
    }

    async fn get_all_active_consent_versions(&self) -> Result<Vec<ConsentVersion>, AppError> {
        let versions = sqlx::query_as::<_, ConsentVersion>(
            r#"
            SELECT DISTINCT ON (consent_type)
                   id, consent_type, version, title, content_hash, content_url, summary,
                   effective_from, effective_to, requires_explicit_consent, is_mandatory,
                   created_at, created_by
            FROM consent_versions
            WHERE effective_from <= NOW()
              AND (effective_to IS NULL OR effective_to > NOW())
            ORDER BY consent_type, effective_from DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(versions)
    }

    async fn get_consent_version_by_id(&self, id: i32) -> Result<Option<ConsentVersion>, AppError> {
        let version = sqlx::query_as::<_, ConsentVersion>(
            r#"
            SELECT id, consent_type, version, title, content_hash, content_url, summary,
                   effective_from, effective_to, requires_explicit_consent, is_mandatory,
                   created_at, created_by
            FROM consent_versions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(version)
    }

    async fn record_consent(&self, input: RecordConsentInput) -> Result<UserConsent, AppError> {
        let consent = sqlx::query_as::<_, UserConsent>(
            r#"
            INSERT INTO user_consents (
                user_id, consent_version_id, consented, consent_method,
                ip_address, user_agent, session_id, consented_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            RETURNING id, user_id, consent_version_id, consented, consent_method,
                      ip_address, user_agent, session_id, consented_at,
                      withdrawn_at, withdrawal_reason
            "#,
        )
        .bind(input.user_id)
        .bind(input.consent_version_id)
        .bind(input.consented)
        .bind(&input.consent_method)
        .bind(input.ip_address)
        .bind(input.user_agent.as_deref())
        .bind(input.session_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(consent)
    }

    async fn get_user_consent(
        &self,
        user_id: Uuid,
        consent_type: &str,
    ) -> Result<Option<UserConsent>, AppError> {
        let consent = sqlx::query_as::<_, UserConsent>(
            r#"
            SELECT uc.id, uc.user_id, uc.consent_version_id, uc.consented, uc.consent_method,
                   uc.ip_address, uc.user_agent, uc.session_id, uc.consented_at,
                   uc.withdrawn_at, uc.withdrawal_reason
            FROM user_consents uc
            JOIN consent_versions cv ON cv.id = uc.consent_version_id
            WHERE uc.user_id = $1
              AND cv.consent_type = $2
              AND uc.withdrawn_at IS NULL
            ORDER BY uc.consented_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .bind(consent_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(consent)
    }

    async fn get_user_consents(&self, user_id: Uuid) -> Result<Vec<UserConsent>, AppError> {
        let consents = sqlx::query_as::<_, UserConsent>(
            r#"
            SELECT id, user_id, consent_version_id, consented, consent_method,
                   ip_address, user_agent, session_id, consented_at,
                   withdrawn_at, withdrawal_reason
            FROM user_consents
            WHERE user_id = $1
            ORDER BY consented_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(consents)
    }

    async fn get_user_consent_status(&self, user_id: Uuid) -> Result<UserConsentStatus, AppError> {
        // Get all active consent versions
        let active_versions = self.get_all_active_consent_versions().await?;

        // Get user's current consents
        let user_consents = self.get_user_consents(user_id).await?;

        let mut consents = Vec::new();
        let mut pending_consent_types = Vec::new();
        let mut has_required_consents = true;

        for version in &active_versions {
            // Find matching user consent for this version
            let matching_consent = user_consents.iter().find(|c| {
                c.consent_version_id == version.id && c.withdrawn_at.is_none()
            });

            if let Some(consent) = matching_consent {
                consents.push(ConsentSummary {
                    consent_type: version.consent_type.clone(),
                    version: version.version.clone(),
                    consented: consent.consented,
                    consented_at: consent.consented_at,
                    is_mandatory: version.is_mandatory,
                    is_current_version: true,
                });

                if version.is_mandatory && !consent.consented {
                    has_required_consents = false;
                    pending_consent_types.push(version.consent_type.clone());
                }
            } else if version.is_mandatory {
                has_required_consents = false;
                pending_consent_types.push(version.consent_type.clone());
            }
        }

        Ok(UserConsentStatus {
            user_id,
            consents,
            has_required_consents,
            pending_consent_types,
        })
    }

    async fn withdraw_consent(
        &self,
        user_id: Uuid,
        consent_type: &str,
        reason: Option<&str>,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE user_consents uc
            SET withdrawn_at = NOW(),
                withdrawal_reason = $3
            FROM consent_versions cv
            WHERE uc.consent_version_id = cv.id
              AND uc.user_id = $1
              AND cv.consent_type = $2
              AND uc.withdrawn_at IS NULL
            "#,
        )
        .bind(user_id)
        .bind(consent_type)
        .bind(reason)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn create_data_request(
        &self,
        input: CreateDataRequestInput,
    ) -> Result<DataAccessRequest, AppError> {
        // GDPR requires response within 30 days
        let deadline = Utc::now() + Duration::days(30);

        let request = sqlx::query_as::<_, DataAccessRequest>(
            r#"
            INSERT INTO data_access_requests (
                user_id, request_type, request_reason, status, regulatory_deadline
            )
            VALUES ($1, $2, $3, 'pending', $4)
            RETURNING id, user_id, request_type, request_reason, status,
                      assigned_to, processing_notes, completed_at, completed_by,
                      completion_notes, export_url, export_expires_at, rejection_reason,
                      requested_at, updated_at, regulatory_deadline
            "#,
        )
        .bind(input.user_id)
        .bind(&input.request_type)
        .bind(input.request_reason.as_deref())
        .bind(deadline)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(request)
    }

    async fn get_data_request(&self, id: Uuid) -> Result<Option<DataAccessRequest>, AppError> {
        let request = sqlx::query_as::<_, DataAccessRequest>(
            r#"
            SELECT id, user_id, request_type, request_reason, status,
                   assigned_to, processing_notes, completed_at, completed_by,
                   completion_notes, export_url, export_expires_at, rejection_reason,
                   requested_at, updated_at, regulatory_deadline
            FROM data_access_requests
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(request)
    }

    async fn get_user_data_requests(&self, user_id: Uuid) -> Result<Vec<DataAccessRequest>, AppError> {
        let requests = sqlx::query_as::<_, DataAccessRequest>(
            r#"
            SELECT id, user_id, request_type, request_reason, status,
                   assigned_to, processing_notes, completed_at, completed_by,
                   completion_notes, export_url, export_expires_at, rejection_reason,
                   requested_at, updated_at, regulatory_deadline
            FROM data_access_requests
            WHERE user_id = $1
            ORDER BY requested_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(requests)
    }

    async fn update_data_request_status(
        &self,
        id: Uuid,
        status: &str,
        notes: Option<&str>,
        updated_by: &str,
    ) -> Result<DataAccessRequest, AppError> {
        let request = sqlx::query_as::<_, DataAccessRequest>(
            r#"
            UPDATE data_access_requests
            SET status = $2,
                processing_notes = COALESCE($3, processing_notes),
                assigned_to = COALESCE(assigned_to, $4),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, request_type, request_reason, status,
                      assigned_to, processing_notes, completed_at, completed_by,
                      completion_notes, export_url, export_expires_at, rejection_reason,
                      requested_at, updated_at, regulatory_deadline
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(notes)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(request)
    }

    async fn complete_data_request(
        &self,
        id: Uuid,
        completed_by: &str,
        notes: Option<&str>,
        export_url: Option<&str>,
    ) -> Result<DataAccessRequest, AppError> {
        // Export links expire in 7 days
        let export_expires = export_url.map(|_| Utc::now() + Duration::days(7));

        let request = sqlx::query_as::<_, DataAccessRequest>(
            r#"
            UPDATE data_access_requests
            SET status = 'completed',
                completed_at = NOW(),
                completed_by = $2,
                completion_notes = $3,
                export_url = $4,
                export_expires_at = $5,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, request_type, request_reason, status,
                      assigned_to, processing_notes, completed_at, completed_by,
                      completion_notes, export_url, export_expires_at, rejection_reason,
                      requested_at, updated_at, regulatory_deadline
            "#,
        )
        .bind(id)
        .bind(completed_by)
        .bind(notes)
        .bind(export_url)
        .bind(export_expires)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(request)
    }

    async fn get_retention_policies(&self) -> Result<Vec<DataRetentionPolicy>, AppError> {
        let policies = sqlx::query_as::<_, DataRetentionPolicy>(
            r#"
            SELECT id, data_category, description, retention_days, anonymize_after_days,
                   requires_consent, legal_hold_exempt, created_at, updated_at, updated_by
            FROM data_retention_policies
            ORDER BY data_category
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(policies)
    }

    async fn get_retention_policy(
        &self,
        category: &str,
    ) -> Result<Option<DataRetentionPolicy>, AppError> {
        let policy = sqlx::query_as::<_, DataRetentionPolicy>(
            r#"
            SELECT id, data_category, description, retention_days, anonymize_after_days,
                   requires_consent, legal_hold_exempt, created_at, updated_at, updated_by
            FROM data_retention_policies
            WHERE data_category = $1
            "#,
        )
        .bind(category)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(policy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consent_summary_serialization() {
        let summary = ConsentSummary {
            consent_type: "terms_of_service".to_string(),
            version: "1.0.0".to_string(),
            consented: true,
            consented_at: Utc::now(),
            is_mandatory: true,
            is_current_version: true,
        };

        let json = serde_json::to_string(&summary).unwrap();
        assert!(json.contains("terms_of_service"));
    }

    #[test]
    fn test_user_consent_status_serialization() {
        let status = UserConsentStatus {
            user_id: Uuid::new_v4(),
            consents: vec![],
            has_required_consents: true,
            pending_consent_types: vec![],
        };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("has_required_consents"));
    }
}
