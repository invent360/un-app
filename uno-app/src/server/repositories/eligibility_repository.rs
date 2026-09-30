//! License eligibility repository
//!
//! Provides:
//! - Eligibility ruleset management
//! - Eligibility checking with logging
//! - Country/device/task type validation

use async_trait::async_trait;
use chrono::{DateTime, Utc, Datelike, Timelike};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::net::IpAddr;
use std::sync::Arc;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Device type for eligibility checks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    Phone,
    Tablet,
    Desktop,
    Tv,
    Wearable,
}

impl DeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Phone => "phone",
            Self::Tablet => "tablet",
            Self::Desktop => "desktop",
            Self::Tv => "tv",
            Self::Wearable => "wearable",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "phone" | "mobile" => Some(Self::Phone),
            "tablet" => Some(Self::Tablet),
            "desktop" | "computer" | "pc" => Some(Self::Desktop),
            "tv" | "television" => Some(Self::Tv),
            "wearable" | "watch" => Some(Self::Wearable),
            _ => None,
        }
    }
}

/// Eligibility check type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckType {
    Claim,
    Reservation,
    Validation,
}

impl CheckType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Claim => "claim",
            Self::Reservation => "reservation",
            Self::Validation => "validation",
        }
    }
}

/// Eligibility failure codes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EligibilityFailure {
    CountryBlocked,
    CountryNotAllowed,
    DeviceBlocked,
    DeviceNotAllowed,
    TaskTypeBlocked,
    TaskTypeNotAllowed,
    AppVersionTooOld,
    OsVersionTooOld,
    VerificationRequired,
    OutsideValidPeriod,
    OutsideAllowedHours,
    OutsideAllowedDays,
    RulesetNotFound,
    RulesetInactive,
}

impl EligibilityFailure {
    pub fn code(&self) -> &'static str {
        match self {
            Self::CountryBlocked => "country_blocked",
            Self::CountryNotAllowed => "country_not_allowed",
            Self::DeviceBlocked => "device_blocked",
            Self::DeviceNotAllowed => "device_not_allowed",
            Self::TaskTypeBlocked => "task_type_blocked",
            Self::TaskTypeNotAllowed => "task_type_not_allowed",
            Self::AppVersionTooOld => "app_version_too_old",
            Self::OsVersionTooOld => "os_version_too_old",
            Self::VerificationRequired => "verification_required",
            Self::OutsideValidPeriod => "outside_valid_period",
            Self::OutsideAllowedHours => "outside_allowed_hours",
            Self::OutsideAllowedDays => "outside_allowed_days",
            Self::RulesetNotFound => "ruleset_not_found",
            Self::RulesetInactive => "ruleset_inactive",
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::CountryBlocked => "This license is not available in your country",
            Self::CountryNotAllowed => "This license is only available in specific countries",
            Self::DeviceBlocked => "This license cannot be claimed on your device type",
            Self::DeviceNotAllowed => "This license is only available on specific device types",
            Self::TaskTypeBlocked => "This task type is blocked for this license",
            Self::TaskTypeNotAllowed => "This license is only valid for specific task types",
            Self::AppVersionTooOld => "Please update your app to claim this license",
            Self::OsVersionTooOld => "Your operating system version is not supported",
            Self::VerificationRequired => "Account verification is required to claim this license",
            Self::OutsideValidPeriod => "This license is not currently available",
            Self::OutsideAllowedHours => "This license can only be claimed during specific hours",
            Self::OutsideAllowedDays => "This license can only be claimed on specific days",
            Self::RulesetNotFound => "Eligibility ruleset not found",
            Self::RulesetInactive => "Eligibility ruleset is inactive",
        }
    }
}

/// Eligibility ruleset entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EligibilityRuleset {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub allowed_countries: Option<Vec<String>>,
    pub blocked_countries: Option<Vec<String>>,
    pub allowed_task_types: Option<Vec<String>>,
    pub blocked_task_types: Option<Vec<String>>,
    pub allowed_device_types: Option<Vec<String>>,
    pub blocked_device_types: Option<Vec<String>>,
    pub min_app_version: Option<String>,
    pub min_os_version: Option<String>,
    pub requires_verification: bool,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub allowed_hours_start: Option<i32>,
    pub allowed_hours_end: Option<i32>,
    pub allowed_days_of_week: Option<Vec<i32>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

/// Input for creating a ruleset
#[derive(Debug, Clone)]
pub struct CreateRulesetInput {
    pub name: String,
    pub description: Option<String>,
    pub allowed_countries: Option<Vec<String>>,
    pub blocked_countries: Option<Vec<String>>,
    pub allowed_task_types: Option<Vec<String>>,
    pub blocked_task_types: Option<Vec<String>>,
    pub allowed_device_types: Option<Vec<String>>,
    pub blocked_device_types: Option<Vec<String>>,
    pub min_app_version: Option<String>,
    pub min_os_version: Option<String>,
    pub requires_verification: bool,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
}

/// Context for eligibility check
#[derive(Debug, Clone, Serialize)]
pub struct EligibilityContext {
    pub country_code: Option<String>,
    pub device_type: Option<DeviceType>,
    pub task_type: Option<String>,
    pub app_version: Option<String>,
    pub os_version: Option<String>,
    pub is_verified: bool,
    pub ip_address: Option<IpAddr>,
}

impl Default for EligibilityContext {
    fn default() -> Self {
        Self {
            country_code: None,
            device_type: None,
            task_type: None,
            app_version: None,
            os_version: None,
            is_verified: false,
            ip_address: None,
        }
    }
}

/// Result of eligibility check
#[derive(Debug, Clone, Serialize)]
pub struct EligibilityResult {
    pub is_eligible: bool,
    pub failure: Option<EligibilityFailure>,
    pub ruleset_id: Option<i32>,
    pub rules_evaluated: Vec<String>,
}

impl EligibilityResult {
    pub fn eligible() -> Self {
        Self {
            is_eligible: true,
            failure: None,
            ruleset_id: None,
            rules_evaluated: vec![],
        }
    }

    pub fn ineligible(failure: EligibilityFailure) -> Self {
        Self {
            is_eligible: false,
            failure: Some(failure),
            ruleset_id: None,
            rules_evaluated: vec![],
        }
    }

    pub fn with_ruleset(mut self, ruleset_id: i32) -> Self {
        self.ruleset_id = Some(ruleset_id);
        self
    }

    pub fn with_rules(mut self, rules: Vec<String>) -> Self {
        self.rules_evaluated = rules;
        self
    }
}

/// License eligibility info (combined from license and ruleset)
#[derive(Debug, Clone, FromRow)]
pub struct LicenseEligibility {
    pub license_id: String,
    pub eligibility_ruleset_id: Option<i32>,
    pub allowed_countries: Option<Vec<String>>,
    pub blocked_countries: Option<Vec<String>>,
    pub allowed_task_types: Option<Vec<String>>,
    pub allowed_device_types: Option<Vec<String>>,
    pub requires_verification: Option<bool>,
}

// ============================================
// TRAIT DEFINITION
// ============================================

pub type DynEligibilityRepository = Arc<dyn EligibilityRepository + Send + Sync>;

#[async_trait]
pub trait EligibilityRepository: Send + Sync {
    // Ruleset operations
    async fn create_ruleset(&self, input: CreateRulesetInput) -> Result<EligibilityRuleset, AppError>;
    async fn get_ruleset(&self, id: i32) -> Result<Option<EligibilityRuleset>, AppError>;
    async fn get_ruleset_by_name(&self, name: &str) -> Result<Option<EligibilityRuleset>, AppError>;
    async fn list_rulesets(&self, include_inactive: bool) -> Result<Vec<EligibilityRuleset>, AppError>;
    async fn update_ruleset(&self, id: i32, input: CreateRulesetInput) -> Result<(), AppError>;
    async fn deactivate_ruleset(&self, id: i32) -> Result<(), AppError>;

    // License eligibility
    async fn get_license_eligibility(&self, license_id: &str) -> Result<Option<LicenseEligibility>, AppError>;
    async fn set_license_ruleset(&self, license_id: &str, ruleset_id: i32) -> Result<(), AppError>;
    async fn set_license_countries(&self, license_id: &str, allowed: Option<Vec<String>>, blocked: Option<Vec<String>>) -> Result<(), AppError>;

    // Eligibility checking
    async fn check_eligibility(
        &self,
        license_id: &str,
        context: &EligibilityContext,
        check_type: CheckType,
        user_id: Option<&str>,
    ) -> Result<EligibilityResult, AppError>;

    // Direct ruleset checking (for batch validation)
    fn check_ruleset_eligibility(
        &self,
        ruleset: &EligibilityRuleset,
        context: &EligibilityContext,
    ) -> EligibilityResult;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct EligibilityRepositoryImpl {
    pool: ConnectionPool,
}

impl EligibilityRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }

    /// Compare semantic versions (returns true if actual >= required)
    fn version_meets_requirement(actual: &str, required: &str) -> bool {
        let parse_version = |v: &str| -> Vec<u32> {
            v.split('.')
                .filter_map(|p| p.parse().ok())
                .collect()
        };

        let actual_parts = parse_version(actual);
        let required_parts = parse_version(required);

        for i in 0..required_parts.len().max(actual_parts.len()) {
            let actual_part = actual_parts.get(i).copied().unwrap_or(0);
            let required_part = required_parts.get(i).copied().unwrap_or(0);

            if actual_part > required_part {
                return true;
            }
            if actual_part < required_part {
                return false;
            }
        }
        true
    }
}

#[async_trait]
impl EligibilityRepository for EligibilityRepositoryImpl {
    async fn create_ruleset(&self, input: CreateRulesetInput) -> Result<EligibilityRuleset, AppError> {
        let ruleset = sqlx::query_as::<_, EligibilityRuleset>(
            r#"
            INSERT INTO eligibility_rulesets (
                name, description,
                allowed_countries, blocked_countries,
                allowed_task_types, blocked_task_types,
                allowed_device_types, blocked_device_types,
                min_app_version, min_os_version,
                requires_verification,
                valid_from, valid_to,
                created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING *
            "#,
        )
        .bind(&input.name)
        .bind(&input.description)
        .bind(&input.allowed_countries)
        .bind(&input.blocked_countries)
        .bind(&input.allowed_task_types)
        .bind(&input.blocked_task_types)
        .bind(&input.allowed_device_types)
        .bind(&input.blocked_device_types)
        .bind(&input.min_app_version)
        .bind(&input.min_os_version)
        .bind(input.requires_verification)
        .bind(input.valid_from)
        .bind(input.valid_to)
        .bind(&input.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(ruleset_id = ruleset.id, name = %input.name, "Created eligibility ruleset");
        Ok(ruleset)
    }

    async fn get_ruleset(&self, id: i32) -> Result<Option<EligibilityRuleset>, AppError> {
        let result = sqlx::query_as::<_, EligibilityRuleset>(
            "SELECT * FROM eligibility_rulesets WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn get_ruleset_by_name(&self, name: &str) -> Result<Option<EligibilityRuleset>, AppError> {
        let result = sqlx::query_as::<_, EligibilityRuleset>(
            "SELECT * FROM eligibility_rulesets WHERE name = $1 AND is_active = TRUE",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn list_rulesets(&self, include_inactive: bool) -> Result<Vec<EligibilityRuleset>, AppError> {
        let query = if include_inactive {
            "SELECT * FROM eligibility_rulesets ORDER BY name"
        } else {
            "SELECT * FROM eligibility_rulesets WHERE is_active = TRUE ORDER BY name"
        };

        let results = sqlx::query_as::<_, EligibilityRuleset>(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn update_ruleset(&self, id: i32, input: CreateRulesetInput) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE eligibility_rulesets SET
                name = $2, description = $3,
                allowed_countries = $4, blocked_countries = $5,
                allowed_task_types = $6, blocked_task_types = $7,
                allowed_device_types = $8, blocked_device_types = $9,
                min_app_version = $10, min_os_version = $11,
                requires_verification = $12,
                valid_from = $13, valid_to = $14,
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(&input.name)
        .bind(&input.description)
        .bind(&input.allowed_countries)
        .bind(&input.blocked_countries)
        .bind(&input.allowed_task_types)
        .bind(&input.blocked_task_types)
        .bind(&input.allowed_device_types)
        .bind(&input.blocked_device_types)
        .bind(&input.min_app_version)
        .bind(&input.min_os_version)
        .bind(input.requires_verification)
        .bind(input.valid_from)
        .bind(input.valid_to)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn deactivate_ruleset(&self, id: i32) -> Result<(), AppError> {
        sqlx::query("UPDATE eligibility_rulesets SET is_active = FALSE, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_license_eligibility(&self, license_id: &str) -> Result<Option<LicenseEligibility>, AppError> {
        let result = sqlx::query_as::<_, LicenseEligibility>(
            r#"
            SELECT id as license_id, eligibility_ruleset_id, allowed_countries, blocked_countries,
                   allowed_task_types, allowed_device_types, requires_verification
            FROM licenses WHERE id = $1
            "#,
        )
        .bind(license_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn set_license_ruleset(&self, license_id: &str, ruleset_id: i32) -> Result<(), AppError> {
        sqlx::query("UPDATE licenses SET eligibility_ruleset_id = $2 WHERE id = $1")
            .bind(license_id)
            .bind(ruleset_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn set_license_countries(
        &self,
        license_id: &str,
        allowed: Option<Vec<String>>,
        blocked: Option<Vec<String>>,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE licenses SET allowed_countries = $2, blocked_countries = $3 WHERE id = $1",
        )
        .bind(license_id)
        .bind(allowed)
        .bind(blocked)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn check_eligibility(
        &self,
        license_id: &str,
        context: &EligibilityContext,
        check_type: CheckType,
        user_id: Option<&str>,
    ) -> Result<EligibilityResult, AppError> {
        // Get license eligibility settings
        let license_elig = self.get_license_eligibility(license_id).await?;

        let license_elig = match license_elig {
            Some(le) => le,
            None => return Ok(EligibilityResult::eligible()),  // No license = no restrictions
        };

        let mut rules_evaluated = Vec::new();
        let mut result = EligibilityResult::eligible();

        // If license has a ruleset, get and check it
        if let Some(ruleset_id) = license_elig.eligibility_ruleset_id {
            let ruleset = self.get_ruleset(ruleset_id).await?;
            match ruleset {
                Some(rs) if !rs.is_active => {
                    result = EligibilityResult::ineligible(EligibilityFailure::RulesetInactive)
                        .with_ruleset(ruleset_id);
                }
                Some(rs) => {
                    result = self.check_ruleset_eligibility(&rs, context)
                        .with_ruleset(ruleset_id);
                    rules_evaluated.push(format!("ruleset:{}", rs.name));
                }
                None => {
                    result = EligibilityResult::ineligible(EligibilityFailure::RulesetNotFound);
                }
            }
        }

        // Check license-level overrides (only if still eligible from ruleset)
        if result.is_eligible {
            // Country check (license-level)
            if let Some(ref country) = context.country_code {
                let country_upper = country.to_uppercase();

                if let Some(ref blocked) = license_elig.blocked_countries {
                    if blocked.iter().any(|c| c.to_uppercase() == country_upper) {
                        result = EligibilityResult::ineligible(EligibilityFailure::CountryBlocked);
                        rules_evaluated.push("license:blocked_countries".to_string());
                    }
                }

                if result.is_eligible {
                    if let Some(ref allowed) = license_elig.allowed_countries {
                        if !allowed.is_empty() && !allowed.iter().any(|c| c.to_uppercase() == country_upper) {
                            result = EligibilityResult::ineligible(EligibilityFailure::CountryNotAllowed);
                            rules_evaluated.push("license:allowed_countries".to_string());
                        }
                    }
                }
            }

            // Device check (license-level)
            if result.is_eligible {
                if let Some(ref device) = context.device_type {
                    if let Some(ref allowed) = license_elig.allowed_device_types {
                        if !allowed.is_empty() && !allowed.contains(&device.as_str().to_string()) {
                            result = EligibilityResult::ineligible(EligibilityFailure::DeviceNotAllowed);
                            rules_evaluated.push("license:allowed_device_types".to_string());
                        }
                    }
                }
            }

            // Task type check (license-level)
            if result.is_eligible {
                if let Some(ref task_type) = context.task_type {
                    if let Some(ref allowed) = license_elig.allowed_task_types {
                        if !allowed.is_empty() && !allowed.contains(task_type) {
                            result = EligibilityResult::ineligible(EligibilityFailure::TaskTypeNotAllowed);
                            rules_evaluated.push("license:allowed_task_types".to_string());
                        }
                    }
                }
            }

            // Verification check (license-level)
            if result.is_eligible {
                if license_elig.requires_verification.unwrap_or(false) && !context.is_verified {
                    result = EligibilityResult::ineligible(EligibilityFailure::VerificationRequired);
                    rules_evaluated.push("license:requires_verification".to_string());
                }
            }
        }

        result = result.with_rules(rules_evaluated.clone());

        // Log the check
        let _ = self.log_eligibility_check(
            license_id,
            user_id,
            check_type,
            context,
            &result,
        ).await;

        Ok(result)
    }

    fn check_ruleset_eligibility(
        &self,
        ruleset: &EligibilityRuleset,
        context: &EligibilityContext,
    ) -> EligibilityResult {
        let now = Utc::now();
        let mut rules_evaluated = Vec::new();

        // Time validity check
        if let Some(valid_from) = ruleset.valid_from {
            if now < valid_from {
                return EligibilityResult::ineligible(EligibilityFailure::OutsideValidPeriod)
                    .with_rules(vec!["valid_from".to_string()]);
            }
        }
        if let Some(valid_to) = ruleset.valid_to {
            if now > valid_to {
                return EligibilityResult::ineligible(EligibilityFailure::OutsideValidPeriod)
                    .with_rules(vec!["valid_to".to_string()]);
            }
        }
        rules_evaluated.push("time_validity".to_string());

        // Hours check
        if let (Some(start), Some(end)) = (ruleset.allowed_hours_start, ruleset.allowed_hours_end) {
            let current_hour = now.hour() as i32;
            let in_range = if start <= end {
                current_hour >= start && current_hour <= end
            } else {
                // Wrapping range (e.g., 22:00 to 06:00)
                current_hour >= start || current_hour <= end
            };
            if !in_range {
                return EligibilityResult::ineligible(EligibilityFailure::OutsideAllowedHours)
                    .with_rules(vec!["allowed_hours".to_string()]);
            }
            rules_evaluated.push("allowed_hours".to_string());
        }

        // Days of week check
        if let Some(ref allowed_days) = ruleset.allowed_days_of_week {
            if !allowed_days.is_empty() {
                let current_day = now.weekday().num_days_from_sunday() as i32;
                if !allowed_days.contains(&current_day) {
                    return EligibilityResult::ineligible(EligibilityFailure::OutsideAllowedDays)
                        .with_rules(vec!["allowed_days_of_week".to_string()]);
                }
                rules_evaluated.push("allowed_days_of_week".to_string());
            }
        }

        // Country check
        if let Some(ref country) = context.country_code {
            let country_upper = country.to_uppercase();

            if let Some(ref blocked) = ruleset.blocked_countries {
                if blocked.iter().any(|c| c.to_uppercase() == country_upper) {
                    return EligibilityResult::ineligible(EligibilityFailure::CountryBlocked)
                        .with_rules(vec!["blocked_countries".to_string()]);
                }
            }

            if let Some(ref allowed) = ruleset.allowed_countries {
                if !allowed.is_empty() && !allowed.iter().any(|c| c.to_uppercase() == country_upper) {
                    return EligibilityResult::ineligible(EligibilityFailure::CountryNotAllowed)
                        .with_rules(vec!["allowed_countries".to_string()]);
                }
            }
            rules_evaluated.push("country".to_string());
        }

        // Device check
        if let Some(ref device) = context.device_type {
            let device_str = device.as_str().to_string();

            if let Some(ref blocked) = ruleset.blocked_device_types {
                if blocked.contains(&device_str) {
                    return EligibilityResult::ineligible(EligibilityFailure::DeviceBlocked)
                        .with_rules(vec!["blocked_device_types".to_string()]);
                }
            }

            if let Some(ref allowed) = ruleset.allowed_device_types {
                if !allowed.is_empty() && !allowed.contains(&device_str) {
                    return EligibilityResult::ineligible(EligibilityFailure::DeviceNotAllowed)
                        .with_rules(vec!["allowed_device_types".to_string()]);
                }
            }
            rules_evaluated.push("device_type".to_string());
        }

        // Task type check
        if let Some(ref task_type) = context.task_type {
            if let Some(ref blocked) = ruleset.blocked_task_types {
                if blocked.contains(task_type) {
                    return EligibilityResult::ineligible(EligibilityFailure::TaskTypeBlocked)
                        .with_rules(vec!["blocked_task_types".to_string()]);
                }
            }

            if let Some(ref allowed) = ruleset.allowed_task_types {
                if !allowed.is_empty() && !allowed.contains(task_type) {
                    return EligibilityResult::ineligible(EligibilityFailure::TaskTypeNotAllowed)
                        .with_rules(vec!["allowed_task_types".to_string()]);
                }
            }
            rules_evaluated.push("task_type".to_string());
        }

        // App version check
        if let Some(ref min_version) = ruleset.min_app_version {
            if let Some(ref app_version) = context.app_version {
                if !Self::version_meets_requirement(app_version, min_version) {
                    return EligibilityResult::ineligible(EligibilityFailure::AppVersionTooOld)
                        .with_rules(vec!["min_app_version".to_string()]);
                }
                rules_evaluated.push("app_version".to_string());
            }
        }

        // OS version check
        if let Some(ref min_version) = ruleset.min_os_version {
            if let Some(ref os_version) = context.os_version {
                if !Self::version_meets_requirement(os_version, min_version) {
                    return EligibilityResult::ineligible(EligibilityFailure::OsVersionTooOld)
                        .with_rules(vec!["min_os_version".to_string()]);
                }
                rules_evaluated.push("os_version".to_string());
            }
        }

        // Verification check
        if ruleset.requires_verification && !context.is_verified {
            return EligibilityResult::ineligible(EligibilityFailure::VerificationRequired)
                .with_rules(vec!["requires_verification".to_string()]);
        }
        rules_evaluated.push("verification".to_string());

        EligibilityResult::eligible().with_rules(rules_evaluated)
    }
}

impl EligibilityRepositoryImpl {
    async fn log_eligibility_check(
        &self,
        license_id: &str,
        user_id: Option<&str>,
        check_type: CheckType,
        context: &EligibilityContext,
        result: &EligibilityResult,
    ) -> Result<(), AppError> {
        let rules_json: JsonValue = serde_json::to_value(&result.rules_evaluated)
            .unwrap_or(JsonValue::Null);

        sqlx::query(
            r#"
            INSERT INTO eligibility_check_log (
                license_id, user_id, check_type,
                country_code, device_type, task_type, app_version, os_version, ip_address,
                is_eligible, failure_reason, failure_code,
                ruleset_id, rules_evaluated
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
        )
        .bind(license_id)
        .bind(user_id)
        .bind(check_type.as_str())
        .bind(context.country_code.as_ref().map(|c| &c[..2.min(c.len())]))
        .bind(context.device_type.map(|d| d.as_str()))
        .bind(&context.task_type)
        .bind(&context.app_version)
        .bind(&context.os_version)
        .bind(context.ip_address.map(|ip| ip.to_string()))
        .bind(result.is_eligible)
        .bind(result.failure.as_ref().map(|f| f.message()))
        .bind(result.failure.as_ref().map(|f| f.code()))
        .bind(result.ruleset_id)
        .bind(rules_json)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !result.is_eligible {
            tracing::debug!(
                license_id = %license_id,
                failure_code = ?result.failure.as_ref().map(|f| f.code()),
                "Eligibility check failed"
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(EligibilityRepositoryImpl::version_meets_requirement("2.0.0", "2.0.0"));
        assert!(EligibilityRepositoryImpl::version_meets_requirement("2.1.0", "2.0.0"));
        assert!(EligibilityRepositoryImpl::version_meets_requirement("2.0.1", "2.0.0"));
        assert!(EligibilityRepositoryImpl::version_meets_requirement("3.0.0", "2.0.0"));
        assert!(!EligibilityRepositoryImpl::version_meets_requirement("1.9.9", "2.0.0"));
        assert!(!EligibilityRepositoryImpl::version_meets_requirement("2.0.0", "2.0.1"));
    }

    #[test]
    fn test_device_type_parsing() {
        assert_eq!(DeviceType::from_str("phone"), Some(DeviceType::Phone));
        assert_eq!(DeviceType::from_str("MOBILE"), Some(DeviceType::Phone));
        assert_eq!(DeviceType::from_str("tablet"), Some(DeviceType::Tablet));
        assert_eq!(DeviceType::from_str("desktop"), Some(DeviceType::Desktop));
        assert_eq!(DeviceType::from_str("unknown"), None);
    }

    #[test]
    fn test_eligibility_failure_codes() {
        assert_eq!(EligibilityFailure::CountryBlocked.code(), "country_blocked");
        assert_eq!(EligibilityFailure::DeviceNotAllowed.code(), "device_not_allowed");
    }
}
