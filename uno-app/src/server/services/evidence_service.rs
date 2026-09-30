//! Evidence service for tracking external data with provenance
//!
//! Provides:
//! - Evidence collection from multiple providers
//! - Source reference and observation time tracking
//! - Freshness validation
//! - Manual verification workflow for unsupported cases

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::repositories::{
    DynImmutableAuditRepository, AuditEventBuilder, AuditCategory, AuditEventType, AuditOutcome, ActorType,
};
use crate::types::AppError;

// ============================================
// EVIDENCE TYPES
// ============================================

/// Type of evidence being tracked
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceType {
    /// License activation on chain
    Activation,
    /// User activity/engagement
    Activity,
    /// Reward distribution
    Reward,
    /// Funding/payment verification
    Funding,
    /// Custom evidence type
    Custom,
}

impl EvidenceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Activation => "activation",
            Self::Activity => "activity",
            Self::Reward => "reward",
            Self::Funding => "funding",
            Self::Custom => "custom",
        }
    }
}

/// Status of evidence verification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    /// Evidence observed but not yet verified
    Pending,
    /// Evidence verified and confirmed
    Confirmed,
    /// Evidence verification failed
    Failed,
    /// Manual review required
    ManualReview,
    /// Evidence is stale (needs refresh)
    Stale,
    /// Evidence provider unavailable
    Unavailable,
}

impl EvidenceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Confirmed => "confirmed",
            Self::Failed => "failed",
            Self::ManualReview => "manual_review",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Evidence record with full provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRecord {
    pub id: Uuid,
    pub evidence_type: EvidenceType,
    pub status: EvidenceStatus,

    // Source tracking
    pub provider_id: String,
    pub source_reference: String,
    pub source_chain_id: Option<String>,
    pub source_contract: Option<String>,
    pub source_tx_hash: Option<String>,
    pub source_block: Option<i64>,

    // Subject
    pub subject_type: String,
    pub subject_id: String,

    // Observation tracking
    pub observed_at: DateTime<Utc>,
    pub observation_source: String,
    pub observation_method: String,

    // Freshness
    pub source_timestamp: Option<DateTime<Utc>>,
    pub freshness_ttl_secs: i64,
    pub last_verified_at: Option<DateTime<Utc>>,
    pub next_verification_at: Option<DateTime<Utc>>,

    // Data
    pub evidence_data: JsonValue,
    pub verification_data: Option<JsonValue>,

    // Audit
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub verification_actor: Option<String>,
    pub notes: Option<String>,
}

impl EvidenceRecord {
    /// Check if evidence is still fresh
    pub fn is_fresh(&self) -> bool {
        if self.freshness_ttl_secs <= 0 {
            return true; // No expiry
        }

        let expires_at = self.observed_at + Duration::seconds(self.freshness_ttl_secs);
        Utc::now() < expires_at
    }

    /// Check if evidence needs verification
    pub fn needs_verification(&self) -> bool {
        match self.status {
            EvidenceStatus::Pending | EvidenceStatus::Stale => true,
            EvidenceStatus::ManualReview => false, // Awaiting manual action
            EvidenceStatus::Confirmed if !self.is_fresh() => true,
            _ => false,
        }
    }
}

/// Input for recording new evidence
#[derive(Debug, Clone)]
pub struct RecordEvidenceInput {
    pub evidence_type: EvidenceType,
    pub provider_id: String,
    pub source_reference: String,
    pub source_chain_id: Option<String>,
    pub source_contract: Option<String>,
    pub source_tx_hash: Option<String>,
    pub source_block: Option<i64>,
    pub subject_type: String,
    pub subject_id: String,
    pub observation_source: String,
    pub observation_method: String,
    pub source_timestamp: Option<DateTime<Utc>>,
    pub freshness_ttl_secs: i64,
    pub evidence_data: JsonValue,
    pub created_by: Option<String>,
}

/// Input for manual verification
#[derive(Debug, Clone)]
pub struct ManualVerificationInput {
    pub evidence_id: Uuid,
    pub verified_by: String,
    pub status: EvidenceStatus,
    pub verification_data: Option<JsonValue>,
    pub notes: Option<String>,
}

// ============================================
// PROVIDER TRAIT
// ============================================

/// Trait for external evidence providers
#[async_trait]
pub trait EvidenceProvider: Send + Sync {
    /// Provider identifier
    fn provider_id(&self) -> &str;

    /// Evidence types this provider supports
    fn supported_types(&self) -> &[EvidenceType];

    /// Check if provider is available
    async fn is_available(&self) -> bool;

    /// Fetch evidence for a subject
    async fn fetch_evidence(
        &self,
        subject_type: &str,
        subject_id: &str,
        evidence_type: EvidenceType,
    ) -> Result<Option<RecordEvidenceInput>, AppError>;

    /// Verify existing evidence is still valid
    async fn verify_evidence(
        &self,
        record: &EvidenceRecord,
    ) -> Result<EvidenceStatus, AppError>;
}

pub type DynEvidenceProvider = Arc<dyn EvidenceProvider>;

// ============================================
// EVIDENCE SERVICE
// ============================================

/// Evidence service for managing evidence records
#[derive(Clone)]
pub struct EvidenceService {
    providers: Vec<DynEvidenceProvider>,
    audit_repo: DynImmutableAuditRepository,
    // In a real implementation, this would use a database repository
    // For now, we define the interface
}

impl EvidenceService {
    pub fn new(
        providers: Vec<DynEvidenceProvider>,
        audit_repo: DynImmutableAuditRepository,
    ) -> Self {
        Self {
            providers,
            audit_repo,
        }
    }

    /// Register a new evidence provider
    pub fn register_provider(&mut self, provider: DynEvidenceProvider) {
        self.providers.push(provider);
    }

    /// Get provider by ID
    pub fn get_provider(&self, provider_id: &str) -> Option<&DynEvidenceProvider> {
        self.providers.iter().find(|p| p.provider_id() == provider_id)
    }

    /// List all registered providers
    pub fn list_providers(&self) -> Vec<&str> {
        self.providers.iter().map(|p| p.provider_id()).collect()
    }

    /// Fetch and record evidence from a provider
    pub async fn fetch_and_record(
        &self,
        provider_id: &str,
        subject_type: &str,
        subject_id: &str,
        evidence_type: EvidenceType,
    ) -> Result<Option<EvidenceRecord>, AppError> {
        let provider = self.get_provider(provider_id)
            .ok_or_else(|| AppError::NotFound(format!("Provider not found: {}", provider_id)))?;

        // Check provider availability
        if !provider.is_available().await {
            tracing::warn!(
                provider_id = %provider_id,
                "Evidence provider unavailable"
            );
            return Err(AppError::ServiceUnavailable("Provider unavailable".to_string()));
        }

        // Fetch evidence
        let input = provider.fetch_evidence(subject_type, subject_id, evidence_type).await?;

        match input {
            Some(input) => {
                let record = self.create_evidence_record(input);

                // Audit log
                let _ = self.audit_repo.log_event(
                    AuditEventBuilder::new(
                        AuditEventType::Custom("evidence.recorded".to_string()),
                        AuditCategory::System,
                    )
                    .resource("evidence", &record.id.to_string())
                    .action("record")
                    .outcome(AuditOutcome::Success)
                    .event_data(serde_json::json!({
                        "evidence_type": evidence_type.as_str(),
                        "provider_id": provider_id,
                        "subject_type": subject_type,
                        "subject_id": subject_id,
                    }))
                    .build()
                ).await;

                tracing::info!(
                    evidence_id = %record.id,
                    evidence_type = %evidence_type.as_str(),
                    provider_id = %provider_id,
                    "Evidence recorded"
                );

                Ok(Some(record))
            }
            None => {
                tracing::debug!(
                    provider_id = %provider_id,
                    subject_type = %subject_type,
                    subject_id = %subject_id,
                    "No evidence found"
                );
                Ok(None)
            }
        }
    }

    /// Verify an evidence record
    pub async fn verify(
        &self,
        record: &EvidenceRecord,
    ) -> Result<EvidenceStatus, AppError> {
        let provider = self.get_provider(&record.provider_id)
            .ok_or_else(|| {
                AppError::ValidationError(format!(
                    "Provider not found for verification: {}",
                    record.provider_id
                ))
            })?;

        // Check provider availability
        if !provider.is_available().await {
            return Ok(EvidenceStatus::Unavailable);
        }

        // Verify with provider
        let status = provider.verify_evidence(record).await?;

        // Audit log
        let _ = self.audit_repo.log_event(
            AuditEventBuilder::new(
                AuditEventType::Custom("evidence.verified".to_string()),
                AuditCategory::System,
            )
            .resource("evidence", &record.id.to_string())
            .action("verify")
            .outcome(if status == EvidenceStatus::Confirmed {
                AuditOutcome::Success
            } else {
                AuditOutcome::Failure
            })
            .event_data(serde_json::json!({
                "status": status.as_str(),
                "provider_id": record.provider_id,
            }))
            .build()
        ).await;

        tracing::info!(
            evidence_id = %record.id,
            status = %status.as_str(),
            "Evidence verified"
        );

        Ok(status)
    }

    /// Manual verification workflow
    pub async fn manual_verify(
        &self,
        input: ManualVerificationInput,
    ) -> Result<EvidenceRecord, AppError> {
        // In a real implementation, this would:
        // 1. Load the evidence record from database
        // 2. Update status and verification data
        // 3. Save back to database
        // 4. Audit log the manual action

        // For now, we just log the intent
        let _ = self.audit_repo.log_event(
            AuditEventBuilder::new(
                AuditEventType::Custom("evidence.manual_verified".to_string()),
                AuditCategory::Compliance,
            )
            .actor(ActorType::Admin, input.verified_by.clone())
            .resource("evidence", &input.evidence_id.to_string())
            .action("manual_verify")
            .outcome(if input.status == EvidenceStatus::Confirmed {
                AuditOutcome::Success
            } else {
                AuditOutcome::Failure
            })
            .event_data(serde_json::json!({
                "status": input.status.as_str(),
                "notes": input.notes,
            }))
            .build()
        ).await;

        tracing::info!(
            evidence_id = %input.evidence_id,
            verified_by = %input.verified_by,
            status = %input.status.as_str(),
            "Manual verification recorded"
        );

        // Return a placeholder - real implementation would return updated record
        Err(AppError::NotImplemented("Database integration pending".to_string()))
    }

    fn create_evidence_record(&self, input: RecordEvidenceInput) -> EvidenceRecord {
        let now = Utc::now();
        let next_verification = if input.freshness_ttl_secs > 0 {
            Some(now + Duration::seconds(input.freshness_ttl_secs))
        } else {
            None
        };

        EvidenceRecord {
            id: Uuid::new_v4(),
            evidence_type: input.evidence_type,
            status: EvidenceStatus::Pending,
            provider_id: input.provider_id,
            source_reference: input.source_reference,
            source_chain_id: input.source_chain_id,
            source_contract: input.source_contract,
            source_tx_hash: input.source_tx_hash,
            source_block: input.source_block,
            subject_type: input.subject_type,
            subject_id: input.subject_id,
            observed_at: now,
            observation_source: input.observation_source,
            observation_method: input.observation_method,
            source_timestamp: input.source_timestamp,
            freshness_ttl_secs: input.freshness_ttl_secs,
            last_verified_at: None,
            next_verification_at: next_verification,
            evidence_data: input.evidence_data,
            verification_data: None,
            created_at: now,
            updated_at: now,
            created_by: input.created_by,
            verification_actor: None,
            notes: None,
        }
    }
}

// ============================================
// NULL PROVIDER (for testing)
// ============================================

/// Null provider that always returns no evidence (for testing)
pub struct NullEvidenceProvider {
    id: String,
}

impl NullEvidenceProvider {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

#[async_trait]
impl EvidenceProvider for NullEvidenceProvider {
    fn provider_id(&self) -> &str {
        &self.id
    }

    fn supported_types(&self) -> &[EvidenceType] {
        &[]
    }

    async fn is_available(&self) -> bool {
        true
    }

    async fn fetch_evidence(
        &self,
        _subject_type: &str,
        _subject_id: &str,
        _evidence_type: EvidenceType,
    ) -> Result<Option<RecordEvidenceInput>, AppError> {
        Ok(None)
    }

    async fn verify_evidence(
        &self,
        _record: &EvidenceRecord,
    ) -> Result<EvidenceStatus, AppError> {
        Ok(EvidenceStatus::Unavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_type() {
        assert_eq!(EvidenceType::Activation.as_str(), "activation");
        assert_eq!(EvidenceType::Reward.as_str(), "reward");
    }

    #[test]
    fn test_evidence_status() {
        assert_eq!(EvidenceStatus::Confirmed.as_str(), "confirmed");
        assert_eq!(EvidenceStatus::ManualReview.as_str(), "manual_review");
    }

    #[test]
    fn test_evidence_freshness() {
        let now = Utc::now();
        let record = EvidenceRecord {
            id: Uuid::new_v4(),
            evidence_type: EvidenceType::Activation,
            status: EvidenceStatus::Confirmed,
            provider_id: "test".to_string(),
            source_reference: "ref".to_string(),
            source_chain_id: None,
            source_contract: None,
            source_tx_hash: None,
            source_block: None,
            subject_type: "license".to_string(),
            subject_id: "123".to_string(),
            observed_at: now,
            observation_source: "test".to_string(),
            observation_method: "api".to_string(),
            source_timestamp: Some(now),
            freshness_ttl_secs: 3600, // 1 hour
            last_verified_at: Some(now),
            next_verification_at: Some(now + Duration::hours(1)),
            evidence_data: serde_json::json!({}),
            verification_data: None,
            created_at: now,
            updated_at: now,
            created_by: None,
            verification_actor: None,
            notes: None,
        };

        assert!(record.is_fresh());
        assert!(!record.needs_verification());
    }
}
