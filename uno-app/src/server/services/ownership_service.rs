//! Ownership validation service
//!
//! Provides:
//! - License ownership tracking
//! - Gate validation for operations
//! - Identity verification checks
//! - Ownership transfer management

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::net::IpAddr;
use std::sync::Arc;

use crate::server::db::ConnectionPool;
use crate::server::repositories::{LaunchGateRepository, GateName};
use crate::types::AppError;

// ============================================
// TYPES
// ============================================

/// Owner type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerType {
    User,
    Service,
    Agent,
    System,
}

impl OwnerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Service => "service",
            Self::Agent => "agent",
            Self::System => "system",
        }
    }
}

/// Acquisition type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionType {
    Issuance,
    Transfer,
    Claim,
    Assignment,
}

impl AcquisitionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Issuance => "issuance",
            Self::Transfer => "transfer",
            Self::Claim => "claim",
            Self::Assignment => "assignment",
        }
    }
}

/// Verification method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationMethod {
    Email,
    Sms,
    Kyc,
    None,
}

impl VerificationMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Sms => "sms",
            Self::Kyc => "kyc",
            Self::None => "none",
        }
    }
}

/// License ownership record
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LicenseOwnership {
    pub id: i64,
    pub license_id: String,
    pub owner_id: String,
    pub owner_type: String,
    pub owned_from: DateTime<Utc>,
    pub owned_until: Option<DateTime<Utc>>,
    pub acquisition_type: String,
    pub acquisition_source: Option<String>,
    pub verified_identity: bool,
    pub verification_method: Option<String>,
    pub verification_id: Option<String>,
    pub gates_checked: Option<JsonValue>,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}

/// Input for establishing ownership
#[derive(Debug, Clone)]
pub struct EstablishOwnershipInput {
    pub license_id: String,
    pub owner_id: String,
    pub owner_type: OwnerType,
    pub acquisition_type: AcquisitionType,
    pub acquisition_source: Option<String>,
    pub verified: bool,
    pub verification_method: Option<VerificationMethod>,
    pub verification_id: Option<String>,
    pub metadata: Option<JsonValue>,
}

/// Gate validation result
#[derive(Debug, Clone, Serialize)]
pub struct GateValidationResult {
    pub all_passed: bool,
    pub gates_required: Vec<String>,
    pub gates_passed: Vec<String>,
    pub gates_failed: Vec<String>,
    pub failure_reason: Option<String>,
}

impl GateValidationResult {
    pub fn success(gates: Vec<String>) -> Self {
        Self {
            all_passed: true,
            gates_required: gates.clone(),
            gates_passed: gates,
            gates_failed: vec![],
            failure_reason: None,
        }
    }

    pub fn failure(required: Vec<String>, passed: Vec<String>, failed: Vec<String>) -> Self {
        let reason = format!("Gates failed: {}", failed.join(", "));
        Self {
            all_passed: false,
            gates_required: required,
            gates_passed: passed,
            gates_failed: failed,
            failure_reason: Some(reason),
        }
    }
}

/// Verification context for operations
#[derive(Debug, Clone, Default)]
pub struct VerificationContext {
    pub user_id: Option<String>,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub kyc_completed: bool,
    pub two_fa_enabled: bool,
    pub account_created_at: Option<DateTime<Utc>>,
    pub country_code: Option<String>,
    pub ip_address: Option<IpAddr>,
}

/// Verification check result
#[derive(Debug, Clone, Serialize)]
pub struct VerificationCheckResult {
    pub passed: bool,
    pub requirement_name: String,
    pub checks_passed: Vec<String>,
    pub checks_failed: Vec<String>,
}

// ============================================
// SERVICE TRAIT
// ============================================

pub type DynOwnershipService = Arc<dyn OwnershipService + Send + Sync>;

#[async_trait]
pub trait OwnershipService: Send + Sync {
    // Ownership operations
    async fn establish_ownership(&self, input: EstablishOwnershipInput) -> Result<LicenseOwnership, AppError>;
    async fn get_current_owner(&self, license_id: &str) -> Result<Option<LicenseOwnership>, AppError>;
    async fn get_ownership_history(&self, license_id: &str) -> Result<Vec<LicenseOwnership>, AppError>;
    async fn verify_owner(&self, license_id: &str, expected_owner: &str) -> Result<bool, AppError>;

    // Gate validation
    async fn validate_gates(
        &self,
        gates: &[GateName],
        operation_type: &str,
        license_id: Option<&str>,
        user_id: Option<&str>,
    ) -> Result<GateValidationResult, AppError>;

    /// Check if issuance is allowed (all required gates open)
    async fn can_issue(&self) -> Result<bool, AppError>;

    // Verification checks
    async fn check_verification(
        &self,
        context: &VerificationContext,
        requirement_name: &str,
    ) -> Result<VerificationCheckResult, AppError>;

    /// Get verification requirement for issuance
    async fn get_issuance_requirement(&self, license_id: &str) -> Result<Option<String>, AppError>;
}

// ============================================
// SERVICE IMPLEMENTATION
// ============================================

pub struct OwnershipServiceImpl {
    pool: ConnectionPool,
    gate_repo: Arc<dyn LaunchGateRepository + Send + Sync>,
}

impl OwnershipServiceImpl {
    pub fn new(
        pool: ConnectionPool,
        gate_repo: Arc<dyn LaunchGateRepository + Send + Sync>,
    ) -> Self {
        Self { pool, gate_repo }
    }
}

#[async_trait]
impl OwnershipService for OwnershipServiceImpl {
    async fn establish_ownership(&self, input: EstablishOwnershipInput) -> Result<LicenseOwnership, AppError> {
        // First validate gates for the acquisition type
        let gates_to_check = match input.acquisition_type {
            AcquisitionType::Issuance => vec![GateName::Issuance],
            AcquisitionType::Claim => vec![GateName::Issuance, GateName::Marketplace],
            AcquisitionType::Transfer => vec![GateName::Marketplace],
            AcquisitionType::Assignment => vec![], // Admin action, no gates
        };

        let gate_result = if !gates_to_check.is_empty() {
            self.validate_gates(&gates_to_check, input.acquisition_type.as_str(), Some(&input.license_id), Some(&input.owner_id)).await?
        } else {
            GateValidationResult::success(vec![])
        };

        if !gate_result.all_passed {
            return Err(AppError::ValidationError(
                gate_result.failure_reason.unwrap_or_else(|| "Gate validation failed".to_string())
            ));
        }

        let gates_json = serde_json::to_value(&gate_result).ok();

        // Start transaction
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Close any existing ownership
        sqlx::query(
            "UPDATE license_ownership SET owned_until = NOW() WHERE license_id = $1 AND owned_until IS NULL"
        )
        .bind(&input.license_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Create new ownership record
        let ownership = sqlx::query_as::<_, LicenseOwnership>(
            r#"
            INSERT INTO license_ownership (
                license_id, owner_id, owner_type,
                acquisition_type, acquisition_source,
                verified_identity, verification_method, verification_id,
                gates_checked, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(&input.license_id)
        .bind(&input.owner_id)
        .bind(input.owner_type.as_str())
        .bind(input.acquisition_type.as_str())
        .bind(&input.acquisition_source)
        .bind(input.verified)
        .bind(input.verification_method.map(|v| v.as_str()))
        .bind(&input.verification_id)
        .bind(gates_json)
        .bind(&input.metadata)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Update license with owner info
        sqlx::query(
            r#"
            UPDATE licenses
            SET issued_to = $2,
                issued_at = COALESCE(issued_at, NOW()),
                owner_verified = $3,
                owner_verification_at = CASE WHEN $3 THEN NOW() ELSE NULL END
            WHERE id = $1
            "#,
        )
        .bind(&input.license_id)
        .bind(&input.owner_id)
        .bind(input.verified)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            license_id = %input.license_id,
            owner_id = %input.owner_id,
            acquisition_type = %input.acquisition_type.as_str(),
            "Ownership established"
        );

        Ok(ownership)
    }

    async fn get_current_owner(&self, license_id: &str) -> Result<Option<LicenseOwnership>, AppError> {
        let result = sqlx::query_as::<_, LicenseOwnership>(
            "SELECT * FROM license_ownership WHERE license_id = $1 AND owned_until IS NULL",
        )
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn get_ownership_history(&self, license_id: &str) -> Result<Vec<LicenseOwnership>, AppError> {
        let results = sqlx::query_as::<_, LicenseOwnership>(
            "SELECT * FROM license_ownership WHERE license_id = $1 ORDER BY owned_from DESC",
        )
        .bind(license_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn verify_owner(&self, license_id: &str, expected_owner: &str) -> Result<bool, AppError> {
        let current = self.get_current_owner(license_id).await?;

        Ok(current.map(|o| o.owner_id == expected_owner).unwrap_or(false))
    }

    async fn validate_gates(
        &self,
        gates: &[GateName],
        operation_type: &str,
        license_id: Option<&str>,
        user_id: Option<&str>,
    ) -> Result<GateValidationResult, AppError> {
        let mut passed = Vec::new();
        let mut failed = Vec::new();

        for gate in gates {
            let is_enabled = self.gate_repo.is_gate_enabled(gate.as_str()).await?;
            if is_enabled {
                passed.push(gate.to_string());
            } else {
                failed.push(gate.to_string());
            }
        }

        let required: Vec<String> = gates.iter().map(|g| g.to_string()).collect();

        // Log the validation
        let all_passed = failed.is_empty();
        sqlx::query(
            r#"
            INSERT INTO gate_validation_log (
                operation_type, license_id, user_id,
                gates_required, gates_passed, gates_failed,
                all_passed, failure_reason
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(operation_type)
        .bind(license_id)
        .bind(user_id)
        .bind(&required)
        .bind(&passed)
        .bind(if failed.is_empty() { None } else { Some(&failed) })
        .bind(all_passed)
        .bind(if failed.is_empty() { None } else {
            Some(format!("Gates failed: {}", failed.join(", ")))
        })
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if all_passed {
            Ok(GateValidationResult::success(required))
        } else {
            Ok(GateValidationResult::failure(required, passed, failed))
        }
    }

    async fn can_issue(&self) -> Result<bool, AppError> {
        self.gate_repo.is_gate_enabled(GateName::Issuance.as_str()).await
    }

    async fn check_verification(
        &self,
        context: &VerificationContext,
        requirement_name: &str,
    ) -> Result<VerificationCheckResult, AppError> {
        // Get the requirement
        let requirement: Option<VerificationRequirementRow> = sqlx::query_as(
            r#"
            SELECT name, require_email_verified, require_phone_verified,
                   require_kyc_completed, require_2fa_enabled,
                   min_account_age_days, allowed_countries, blocked_countries
            FROM verification_requirements
            WHERE name = $1 AND is_active = TRUE
            "#,
        )
        .bind(requirement_name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let req = match requirement {
            Some(r) => r,
            None => {
                // No requirement = pass
                return Ok(VerificationCheckResult {
                    passed: true,
                    requirement_name: requirement_name.to_string(),
                    checks_passed: vec!["no_requirement".to_string()],
                    checks_failed: vec![],
                });
            }
        };

        let mut passed = Vec::new();
        let mut failed = Vec::new();

        // Email verification
        if req.require_email_verified {
            if context.email_verified {
                passed.push("email_verified".to_string());
            } else {
                failed.push("email_verified".to_string());
            }
        }

        // Phone verification
        if req.require_phone_verified {
            if context.phone_verified {
                passed.push("phone_verified".to_string());
            } else {
                failed.push("phone_verified".to_string());
            }
        }

        // KYC completion
        if req.require_kyc_completed {
            if context.kyc_completed {
                passed.push("kyc_completed".to_string());
            } else {
                failed.push("kyc_completed".to_string());
            }
        }

        // 2FA enabled
        if req.require_2fa_enabled {
            if context.two_fa_enabled {
                passed.push("2fa_enabled".to_string());
            } else {
                failed.push("2fa_enabled".to_string());
            }
        }

        // Account age
        if let Some(min_days) = req.min_account_age_days {
            if min_days > 0 {
                if let Some(created_at) = context.account_created_at {
                    let age_days = (Utc::now() - created_at).num_days();
                    if age_days >= min_days as i64 {
                        passed.push(format!("account_age_{}d", min_days));
                    } else {
                        failed.push(format!("account_age_{}d", min_days));
                    }
                } else {
                    failed.push(format!("account_age_{}d", min_days));
                }
            }
        }

        // Country restrictions
        if let Some(ref country) = context.country_code {
            let country_upper = country.to_uppercase();

            // Check blocked first
            if let Some(ref blocked) = req.blocked_countries {
                if blocked.iter().any(|c| c.to_uppercase() == country_upper) {
                    failed.push("country_not_blocked".to_string());
                } else {
                    passed.push("country_not_blocked".to_string());
                }
            }

            // Then check allowed
            if let Some(ref allowed) = req.allowed_countries {
                if !allowed.is_empty() {
                    if allowed.iter().any(|c| c.to_uppercase() == country_upper) {
                        passed.push("country_allowed".to_string());
                    } else {
                        failed.push("country_allowed".to_string());
                    }
                }
            }
        }

        Ok(VerificationCheckResult {
            passed: failed.is_empty(),
            requirement_name: requirement_name.to_string(),
            checks_passed: passed,
            checks_failed: failed,
        })
    }

    async fn get_issuance_requirement(&self, license_id: &str) -> Result<Option<String>, AppError> {
        let result: Option<(Option<String>,)> = sqlx::query_as(
            r#"
            SELECT vr.name
            FROM licenses l
            LEFT JOIN verification_requirements vr ON l.verification_requirement_id = vr.id
            WHERE l.id = $1
            "#,
        )
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.and_then(|(name,)| name))
    }
}

// ============================================
// HELPER TYPES
// ============================================

#[derive(Debug, sqlx::FromRow)]
struct VerificationRequirementRow {
    name: String,
    require_email_verified: bool,
    require_phone_verified: bool,
    require_kyc_completed: bool,
    require_2fa_enabled: bool,
    min_account_age_days: Option<i32>,
    allowed_countries: Option<Vec<String>>,
    blocked_countries: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owner_type() {
        assert_eq!(OwnerType::User.as_str(), "user");
        assert_eq!(OwnerType::Service.as_str(), "service");
    }

    #[test]
    fn test_acquisition_type() {
        assert_eq!(AcquisitionType::Issuance.as_str(), "issuance");
        assert_eq!(AcquisitionType::Transfer.as_str(), "transfer");
    }

    #[test]
    fn test_gate_validation_result() {
        let success = GateValidationResult::success(vec!["issuance".to_string()]);
        assert!(success.all_passed);
        assert!(success.gates_failed.is_empty());
    }
}
