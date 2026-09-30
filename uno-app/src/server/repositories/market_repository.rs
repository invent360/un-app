//! Market repository for quota and status management
//!
//! Implements CRUD operations for:
//! - Market status (open/closed, readiness)
//! - Market quotas (daily/weekly/total limits)
//! - Campaign sources and attribution
//! - Quota consumption tracking

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Market status - controls whether a market is open for claims
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MarketStatus {
    pub country_code: String,
    pub is_open: bool,
    pub pause_reason: Option<String>,
    pub paused_at: Option<DateTime<Utc>>,
    pub paused_by: Option<String>,
    pub resumed_at: Option<DateTime<Utc>>,
    pub resumed_by: Option<String>,
    pub requires_reviewed_content: bool,
    pub requires_local_support: bool,
    pub requires_local_language: bool,
    pub content_review_status: Option<String>,
    pub content_reviewed_at: Option<DateTime<Utc>>,
    pub content_reviewed_by: Option<String>,
    pub support_readiness_status: Option<String>,
    pub support_ready_at: Option<DateTime<Utc>>,
    pub support_ready_by: Option<String>,
    pub language_review_status: Option<String>,
    pub language_reviewed_at: Option<DateTime<Utc>>,
    pub language_reviewed_by: Option<String>,
    pub primary_locale: Option<String>,
    pub supported_locales: Vec<String>,
    pub timezone: Option<String>,
    pub currency_code: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Market quota configuration
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MarketQuota {
    pub id: Uuid,
    pub country_code: String,
    pub quota_type: String,
    pub max_value: i32,
    pub current_value: i32,
    pub warning_threshold: Option<i32>,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
    pub enabled: bool,
    pub exhausted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Quota consumption log entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuotaConsumption {
    pub id: Uuid,
    pub quota_id: Uuid,
    pub user_id: String,
    pub license_id: Option<String>,
    pub amount: i32,
    pub action: String,
    pub consumed_at: DateTime<Utc>,
}

/// Campaign source for tracking
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CampaignSource {
    pub id: Uuid,
    pub campaign_code: String,
    pub campaign_name: String,
    pub description: Option<String>,
    pub source_type: String,
    pub country_code: Option<String>,
    pub agent_id: Option<Uuid>,
    pub partner_name: Option<String>,
    pub qr_code_url: Option<String>,
    pub short_url: Option<String>,
    pub full_url: Option<String>,
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_campaign: Option<String>,
    pub utm_content: Option<String>,
    pub is_active: bool,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub click_count: i32,
    pub registration_count: i32,
    pub claim_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

/// Campaign attribution record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CampaignAttribution {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub user_id: Option<String>,
    pub license_id: Option<String>,
    pub attribution_type: String,
    pub referrer_url: Option<String>,
    pub landing_url: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub country_code: Option<String>,
    pub attributed_at: DateTime<Utc>,
}

// ============================================
// INPUT STRUCTS
// ============================================

/// Input for creating/updating market status
#[derive(Debug, Clone)]
pub struct UpsertMarketStatusInput {
    pub country_code: String,
    pub is_open: bool,
    pub requires_reviewed_content: bool,
    pub requires_local_support: bool,
    pub requires_local_language: bool,
    pub primary_locale: Option<String>,
    pub timezone: Option<String>,
    pub currency_code: Option<String>,
}

/// Input for creating a quota
#[derive(Debug, Clone)]
pub struct CreateQuotaInput {
    pub country_code: String,
    pub quota_type: String,
    pub max_value: i32,
    pub warning_threshold: Option<i32>,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
}

/// Input for creating a campaign
#[derive(Debug, Clone)]
pub struct CreateCampaignInput {
    pub campaign_code: String,
    pub campaign_name: String,
    pub description: Option<String>,
    pub source_type: String,
    pub country_code: Option<String>,
    pub agent_id: Option<Uuid>,
    pub partner_name: Option<String>,
    pub qr_code_url: Option<String>,
    pub short_url: Option<String>,
    pub full_url: Option<String>,
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_campaign: Option<String>,
    pub utm_content: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
}

/// Input for recording attribution
#[derive(Debug, Clone)]
pub struct RecordAttributionInput {
    pub campaign_id: Uuid,
    pub user_id: Option<String>,
    pub license_id: Option<String>,
    pub attribution_type: String,
    pub referrer_url: Option<String>,
    pub landing_url: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub country_code: Option<String>,
}

/// Market readiness check result
#[derive(Debug, Clone, Serialize)]
pub struct MarketReadiness {
    pub country_code: String,
    pub is_ready: bool,
    pub is_open: bool,
    pub content_ready: bool,
    pub support_ready: bool,
    pub language_ready: bool,
    pub blocking_reasons: Vec<String>,
}

/// Quota status check result
#[derive(Debug, Clone, Serialize)]
pub struct QuotaStatus {
    pub quota_id: Uuid,
    pub country_code: String,
    pub quota_type: String,
    pub max_value: i32,
    pub current_value: i32,
    pub remaining: i32,
    pub is_exhausted: bool,
    pub at_warning_threshold: bool,
}

// ============================================
// TRAIT DEFINITION
// ============================================

/// Dynamic type alias for MarketRepository trait object
pub type DynMarketRepository = Arc<dyn MarketRepository + Send + Sync>;

/// Market repository trait defining database operations
#[async_trait]
pub trait MarketRepository: Send + Sync {
    // --- Market Status Operations ---

    /// Get market status for a country
    async fn get_market_status(&self, country_code: &str) -> Result<Option<MarketStatus>, AppError>;

    /// Get all market statuses
    async fn get_all_market_statuses(&self) -> Result<Vec<MarketStatus>, AppError>;

    /// Get open markets
    async fn get_open_markets(&self) -> Result<Vec<MarketStatus>, AppError>;

    /// Create or update market status
    async fn upsert_market_status(&self, input: UpsertMarketStatusInput) -> Result<MarketStatus, AppError>;

    /// Pause a market
    async fn pause_market(
        &self,
        country_code: &str,
        reason: &str,
        paused_by: &str,
    ) -> Result<MarketStatus, AppError>;

    /// Resume a market
    async fn resume_market(
        &self,
        country_code: &str,
        resumed_by: &str,
    ) -> Result<MarketStatus, AppError>;

    /// Update content review status
    async fn update_content_review(
        &self,
        country_code: &str,
        status: &str,
        reviewed_by: &str,
    ) -> Result<MarketStatus, AppError>;

    /// Update support readiness status
    async fn update_support_readiness(
        &self,
        country_code: &str,
        status: &str,
        updated_by: &str,
    ) -> Result<MarketStatus, AppError>;

    /// Update language review status
    async fn update_language_review(
        &self,
        country_code: &str,
        status: &str,
        reviewed_by: &str,
    ) -> Result<MarketStatus, AppError>;

    /// Check market readiness
    async fn check_market_readiness(&self, country_code: &str) -> Result<MarketReadiness, AppError>;

    // --- Quota Operations ---

    /// Create a quota
    async fn create_quota(&self, input: CreateQuotaInput) -> Result<MarketQuota, AppError>;

    /// Get quota by ID
    async fn get_quota(&self, id: Uuid) -> Result<Option<MarketQuota>, AppError>;

    /// Get quotas for a country
    async fn get_country_quotas(&self, country_code: &str) -> Result<Vec<MarketQuota>, AppError>;

    /// Get active quota by type
    async fn get_active_quota(
        &self,
        country_code: &str,
        quota_type: &str,
    ) -> Result<Option<MarketQuota>, AppError>;

    /// Check quota status
    async fn check_quota(&self, country_code: &str, quota_type: &str) -> Result<QuotaStatus, AppError>;

    /// Consume quota (atomic increment with check)
    async fn consume_quota(
        &self,
        country_code: &str,
        quota_type: &str,
        user_id: &str,
        license_id: Option<&str>,
        action: &str,
    ) -> Result<bool, AppError>;

    /// Reset quota value
    async fn reset_quota(&self, quota_id: Uuid) -> Result<MarketQuota, AppError>;

    /// Update quota max value
    async fn update_quota_max(&self, quota_id: Uuid, max_value: i32) -> Result<MarketQuota, AppError>;

    /// Enable/disable quota
    async fn set_quota_enabled(&self, quota_id: Uuid, enabled: bool) -> Result<MarketQuota, AppError>;

    /// Reset all daily quotas (for scheduled job)
    async fn reset_daily_quotas(&self, period_date: NaiveDate) -> Result<i32, AppError>;

    // --- Campaign Operations ---

    /// Create a campaign
    async fn create_campaign(&self, input: CreateCampaignInput) -> Result<CampaignSource, AppError>;

    /// Get campaign by ID
    async fn get_campaign(&self, id: Uuid) -> Result<Option<CampaignSource>, AppError>;

    /// Get campaign by code
    async fn get_campaign_by_code(&self, code: &str) -> Result<Option<CampaignSource>, AppError>;

    /// Get campaigns for country
    async fn get_country_campaigns(&self, country_code: &str) -> Result<Vec<CampaignSource>, AppError>;

    /// Get campaigns for agent
    async fn get_agent_campaigns(&self, agent_id: Uuid) -> Result<Vec<CampaignSource>, AppError>;

    /// Get active campaigns
    async fn get_active_campaigns(&self) -> Result<Vec<CampaignSource>, AppError>;

    /// Update campaign status
    async fn set_campaign_active(&self, id: Uuid, is_active: bool) -> Result<CampaignSource, AppError>;

    /// Increment campaign counter
    async fn increment_campaign_counter(
        &self,
        id: Uuid,
        counter: &str,
    ) -> Result<CampaignSource, AppError>;

    // --- Attribution Operations ---

    /// Record attribution
    async fn record_attribution(&self, input: RecordAttributionInput) -> Result<CampaignAttribution, AppError>;

    /// Get attributions for campaign
    async fn get_campaign_attributions(
        &self,
        campaign_id: Uuid,
        attribution_type: Option<&str>,
    ) -> Result<Vec<CampaignAttribution>, AppError>;

    /// Get attributions for user
    async fn get_user_attributions(&self, user_id: &str) -> Result<Vec<CampaignAttribution>, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct MarketRepositoryImpl {
    pool: ConnectionPool,
}

impl MarketRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MarketRepository for MarketRepositoryImpl {
    async fn get_market_status(&self, country_code: &str) -> Result<Option<MarketStatus>, AppError> {
        let status = sqlx::query_as::<_, MarketStatus>(
            r#"
            SELECT country_code, is_open, pause_reason, paused_at, paused_by, resumed_at, resumed_by,
                   requires_reviewed_content, requires_local_support, requires_local_language,
                   content_review_status, content_reviewed_at, content_reviewed_by,
                   support_readiness_status, support_ready_at, support_ready_by,
                   language_review_status, language_reviewed_at, language_reviewed_by,
                   primary_locale, supported_locales, timezone, currency_code, notes,
                   created_at, updated_at
            FROM market_status
            WHERE country_code = $1
            "#,
        )
        .bind(country_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(status)
    }

    async fn get_all_market_statuses(&self) -> Result<Vec<MarketStatus>, AppError> {
        let statuses = sqlx::query_as::<_, MarketStatus>(
            r#"
            SELECT country_code, is_open, pause_reason, paused_at, paused_by, resumed_at, resumed_by,
                   requires_reviewed_content, requires_local_support, requires_local_language,
                   content_review_status, content_reviewed_at, content_reviewed_by,
                   support_readiness_status, support_ready_at, support_ready_by,
                   language_review_status, language_reviewed_at, language_reviewed_by,
                   primary_locale, supported_locales, timezone, currency_code, notes,
                   created_at, updated_at
            FROM market_status
            ORDER BY country_code
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(statuses)
    }

    async fn get_open_markets(&self) -> Result<Vec<MarketStatus>, AppError> {
        let statuses = sqlx::query_as::<_, MarketStatus>(
            r#"
            SELECT country_code, is_open, pause_reason, paused_at, paused_by, resumed_at, resumed_by,
                   requires_reviewed_content, requires_local_support, requires_local_language,
                   content_review_status, content_reviewed_at, content_reviewed_by,
                   support_readiness_status, support_ready_at, support_ready_by,
                   language_review_status, language_reviewed_at, language_reviewed_by,
                   primary_locale, supported_locales, timezone, currency_code, notes,
                   created_at, updated_at
            FROM market_status
            WHERE is_open = TRUE
            ORDER BY country_code
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(statuses)
    }

    async fn upsert_market_status(&self, input: UpsertMarketStatusInput) -> Result<MarketStatus, AppError> {
        let status = sqlx::query_as::<_, MarketStatus>(
            r#"
            INSERT INTO market_status (
                country_code, is_open, requires_reviewed_content, requires_local_support,
                requires_local_language, primary_locale, timezone, currency_code
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (country_code)
            DO UPDATE SET
                is_open = EXCLUDED.is_open,
                requires_reviewed_content = EXCLUDED.requires_reviewed_content,
                requires_local_support = EXCLUDED.requires_local_support,
                requires_local_language = EXCLUDED.requires_local_language,
                primary_locale = COALESCE(EXCLUDED.primary_locale, market_status.primary_locale),
                timezone = COALESCE(EXCLUDED.timezone, market_status.timezone),
                currency_code = COALESCE(EXCLUDED.currency_code, market_status.currency_code),
                updated_at = NOW()
            RETURNING country_code, is_open, pause_reason, paused_at, paused_by, resumed_at, resumed_by,
                      requires_reviewed_content, requires_local_support, requires_local_language,
                      content_review_status, content_reviewed_at, content_reviewed_by,
                      support_readiness_status, support_ready_at, support_ready_by,
                      language_review_status, language_reviewed_at, language_reviewed_by,
                      primary_locale, supported_locales, timezone, currency_code, notes,
                      created_at, updated_at
            "#,
        )
        .bind(&input.country_code)
        .bind(input.is_open)
        .bind(input.requires_reviewed_content)
        .bind(input.requires_local_support)
        .bind(input.requires_local_language)
        .bind(&input.primary_locale)
        .bind(&input.timezone)
        .bind(&input.currency_code)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(status)
    }

    async fn pause_market(
        &self,
        country_code: &str,
        reason: &str,
        paused_by: &str,
    ) -> Result<MarketStatus, AppError> {
        let status = sqlx::query_as::<_, MarketStatus>(
            r#"
            UPDATE market_status
            SET is_open = FALSE,
                pause_reason = $2,
                paused_at = NOW(),
                paused_by = $3,
                updated_at = NOW()
            WHERE country_code = $1
            RETURNING country_code, is_open, pause_reason, paused_at, paused_by, resumed_at, resumed_by,
                      requires_reviewed_content, requires_local_support, requires_local_language,
                      content_review_status, content_reviewed_at, content_reviewed_by,
                      support_readiness_status, support_ready_at, support_ready_by,
                      language_review_status, language_reviewed_at, language_reviewed_by,
                      primary_locale, supported_locales, timezone, currency_code, notes,
                      created_at, updated_at
            "#,
        )
        .bind(country_code)
        .bind(reason)
        .bind(paused_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(status)
    }

    async fn resume_market(
        &self,
        country_code: &str,
        resumed_by: &str,
    ) -> Result<MarketStatus, AppError> {
        let status = sqlx::query_as::<_, MarketStatus>(
            r#"
            UPDATE market_status
            SET is_open = TRUE,
                pause_reason = NULL,
                resumed_at = NOW(),
                resumed_by = $2,
                updated_at = NOW()
            WHERE country_code = $1
            RETURNING country_code, is_open, pause_reason, paused_at, paused_by, resumed_at, resumed_by,
                      requires_reviewed_content, requires_local_support, requires_local_language,
                      content_review_status, content_reviewed_at, content_reviewed_by,
                      support_readiness_status, support_ready_at, support_ready_by,
                      language_review_status, language_reviewed_at, language_reviewed_by,
                      primary_locale, supported_locales, timezone, currency_code, notes,
                      created_at, updated_at
            "#,
        )
        .bind(country_code)
        .bind(resumed_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(status)
    }

    async fn update_content_review(
        &self,
        country_code: &str,
        status: &str,
        reviewed_by: &str,
    ) -> Result<MarketStatus, AppError> {
        let result = sqlx::query_as::<_, MarketStatus>(
            r#"
            UPDATE market_status
            SET content_review_status = $2,
                content_reviewed_at = NOW(),
                content_reviewed_by = $3,
                updated_at = NOW()
            WHERE country_code = $1
            RETURNING country_code, is_open, pause_reason, paused_at, paused_by, resumed_at, resumed_by,
                      requires_reviewed_content, requires_local_support, requires_local_language,
                      content_review_status, content_reviewed_at, content_reviewed_by,
                      support_readiness_status, support_ready_at, support_ready_by,
                      language_review_status, language_reviewed_at, language_reviewed_by,
                      primary_locale, supported_locales, timezone, currency_code, notes,
                      created_at, updated_at
            "#,
        )
        .bind(country_code)
        .bind(status)
        .bind(reviewed_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn update_support_readiness(
        &self,
        country_code: &str,
        status: &str,
        updated_by: &str,
    ) -> Result<MarketStatus, AppError> {
        let result = sqlx::query_as::<_, MarketStatus>(
            r#"
            UPDATE market_status
            SET support_readiness_status = $2,
                support_ready_at = NOW(),
                support_ready_by = $3,
                updated_at = NOW()
            WHERE country_code = $1
            RETURNING country_code, is_open, pause_reason, paused_at, paused_by, resumed_at, resumed_by,
                      requires_reviewed_content, requires_local_support, requires_local_language,
                      content_review_status, content_reviewed_at, content_reviewed_by,
                      support_readiness_status, support_ready_at, support_ready_by,
                      language_review_status, language_reviewed_at, language_reviewed_by,
                      primary_locale, supported_locales, timezone, currency_code, notes,
                      created_at, updated_at
            "#,
        )
        .bind(country_code)
        .bind(status)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn update_language_review(
        &self,
        country_code: &str,
        status: &str,
        reviewed_by: &str,
    ) -> Result<MarketStatus, AppError> {
        let result = sqlx::query_as::<_, MarketStatus>(
            r#"
            UPDATE market_status
            SET language_review_status = $2,
                language_reviewed_at = NOW(),
                language_reviewed_by = $3,
                updated_at = NOW()
            WHERE country_code = $1
            RETURNING country_code, is_open, pause_reason, paused_at, paused_by, resumed_at, resumed_by,
                      requires_reviewed_content, requires_local_support, requires_local_language,
                      content_review_status, content_reviewed_at, content_reviewed_by,
                      support_readiness_status, support_ready_at, support_ready_by,
                      language_review_status, language_reviewed_at, language_reviewed_by,
                      primary_locale, supported_locales, timezone, currency_code, notes,
                      created_at, updated_at
            "#,
        )
        .bind(country_code)
        .bind(status)
        .bind(reviewed_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn check_market_readiness(&self, country_code: &str) -> Result<MarketReadiness, AppError> {
        // Use the database function
        let row = sqlx::query_as::<_, (bool, bool, bool, bool, bool, Vec<String>)>(
            "SELECT * FROM check_market_readiness($1)",
        )
        .bind(country_code)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(MarketReadiness {
            country_code: country_code.to_string(),
            is_ready: row.0,
            is_open: row.1,
            content_ready: row.2,
            support_ready: row.3,
            language_ready: row.4,
            blocking_reasons: row.5,
        })
    }

    async fn create_quota(&self, input: CreateQuotaInput) -> Result<MarketQuota, AppError> {
        let quota = sqlx::query_as::<_, MarketQuota>(
            r#"
            INSERT INTO market_quotas (
                country_code, quota_type, max_value, warning_threshold, period_start, period_end
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, country_code, quota_type, max_value, current_value, warning_threshold,
                      period_start, period_end, enabled, exhausted_at, created_at, updated_at
            "#,
        )
        .bind(&input.country_code)
        .bind(&input.quota_type)
        .bind(input.max_value)
        .bind(input.warning_threshold)
        .bind(input.period_start)
        .bind(input.period_end)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(quota)
    }

    async fn get_quota(&self, id: Uuid) -> Result<Option<MarketQuota>, AppError> {
        let quota = sqlx::query_as::<_, MarketQuota>(
            r#"
            SELECT id, country_code, quota_type, max_value, current_value, warning_threshold,
                   period_start, period_end, enabled, exhausted_at, created_at, updated_at
            FROM market_quotas
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(quota)
    }

    async fn get_country_quotas(&self, country_code: &str) -> Result<Vec<MarketQuota>, AppError> {
        let quotas = sqlx::query_as::<_, MarketQuota>(
            r#"
            SELECT id, country_code, quota_type, max_value, current_value, warning_threshold,
                   period_start, period_end, enabled, exhausted_at, created_at, updated_at
            FROM market_quotas
            WHERE country_code = $1
            ORDER BY quota_type
            "#,
        )
        .bind(country_code)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(quotas)
    }

    async fn get_active_quota(
        &self,
        country_code: &str,
        quota_type: &str,
    ) -> Result<Option<MarketQuota>, AppError> {
        let quota = sqlx::query_as::<_, MarketQuota>(
            r#"
            SELECT id, country_code, quota_type, max_value, current_value, warning_threshold,
                   period_start, period_end, enabled, exhausted_at, created_at, updated_at
            FROM market_quotas
            WHERE country_code = $1
              AND quota_type = $2
              AND enabled = TRUE
              AND (period_start IS NULL OR period_start <= CURRENT_DATE)
              AND (period_end IS NULL OR period_end >= CURRENT_DATE)
            "#,
        )
        .bind(country_code)
        .bind(quota_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(quota)
    }

    async fn check_quota(&self, country_code: &str, quota_type: &str) -> Result<QuotaStatus, AppError> {
        let quota = self.get_active_quota(country_code, quota_type).await?
            .ok_or_else(|| AppError::NotFound(format!(
                "No active quota found for {} / {}",
                country_code, quota_type
            )))?;

        let remaining = quota.max_value - quota.current_value;
        let at_warning = quota.warning_threshold
            .map(|threshold| quota.current_value >= threshold)
            .unwrap_or(false);

        Ok(QuotaStatus {
            quota_id: quota.id,
            country_code: quota.country_code,
            quota_type: quota.quota_type,
            max_value: quota.max_value,
            current_value: quota.current_value,
            remaining: remaining.max(0),
            is_exhausted: remaining <= 0,
            at_warning_threshold: at_warning,
        })
    }

    async fn consume_quota(
        &self,
        country_code: &str,
        quota_type: &str,
        user_id: &str,
        license_id: Option<&str>,
        action: &str,
    ) -> Result<bool, AppError> {
        // Use the database function for atomic operation
        let (consumed,): (bool,) = sqlx::query_as(
            "SELECT consume_market_quota($1, $2, $3, $4, $5)",
        )
        .bind(country_code)
        .bind(quota_type)
        .bind(user_id)
        .bind(license_id)
        .bind(action)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(consumed)
    }

    async fn reset_quota(&self, quota_id: Uuid) -> Result<MarketQuota, AppError> {
        let quota = sqlx::query_as::<_, MarketQuota>(
            r#"
            UPDATE market_quotas
            SET current_value = 0,
                exhausted_at = NULL,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, country_code, quota_type, max_value, current_value, warning_threshold,
                      period_start, period_end, enabled, exhausted_at, created_at, updated_at
            "#,
        )
        .bind(quota_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(quota)
    }

    async fn update_quota_max(&self, quota_id: Uuid, max_value: i32) -> Result<MarketQuota, AppError> {
        let quota = sqlx::query_as::<_, MarketQuota>(
            r#"
            UPDATE market_quotas
            SET max_value = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, country_code, quota_type, max_value, current_value, warning_threshold,
                      period_start, period_end, enabled, exhausted_at, created_at, updated_at
            "#,
        )
        .bind(quota_id)
        .bind(max_value)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(quota)
    }

    async fn set_quota_enabled(&self, quota_id: Uuid, enabled: bool) -> Result<MarketQuota, AppError> {
        let quota = sqlx::query_as::<_, MarketQuota>(
            r#"
            UPDATE market_quotas
            SET enabled = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, country_code, quota_type, max_value, current_value, warning_threshold,
                      period_start, period_end, enabled, exhausted_at, created_at, updated_at
            "#,
        )
        .bind(quota_id)
        .bind(enabled)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(quota)
    }

    async fn reset_daily_quotas(&self, period_date: NaiveDate) -> Result<i32, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE market_quotas
            SET current_value = 0,
                exhausted_at = NULL,
                period_start = $1,
                period_end = $1,
                updated_at = NOW()
            WHERE quota_type = 'daily_claims'
              AND enabled = TRUE
            "#,
        )
        .bind(period_date)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() as i32)
    }

    async fn create_campaign(&self, input: CreateCampaignInput) -> Result<CampaignSource, AppError> {
        let campaign = sqlx::query_as::<_, CampaignSource>(
            r#"
            INSERT INTO campaign_sources (
                campaign_code, campaign_name, description, source_type, country_code,
                agent_id, partner_name, qr_code_url, short_url, full_url,
                utm_source, utm_medium, utm_campaign, utm_content,
                starts_at, ends_at, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING id, campaign_code, campaign_name, description, source_type, country_code,
                      agent_id, partner_name, qr_code_url, short_url, full_url,
                      utm_source, utm_medium, utm_campaign, utm_content, is_active,
                      starts_at, ends_at, click_count, registration_count, claim_count,
                      created_at, updated_at, created_by
            "#,
        )
        .bind(&input.campaign_code)
        .bind(&input.campaign_name)
        .bind(&input.description)
        .bind(&input.source_type)
        .bind(&input.country_code)
        .bind(input.agent_id)
        .bind(&input.partner_name)
        .bind(&input.qr_code_url)
        .bind(&input.short_url)
        .bind(&input.full_url)
        .bind(&input.utm_source)
        .bind(&input.utm_medium)
        .bind(&input.utm_campaign)
        .bind(&input.utm_content)
        .bind(input.starts_at)
        .bind(input.ends_at)
        .bind(&input.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(campaign)
    }

    async fn get_campaign(&self, id: Uuid) -> Result<Option<CampaignSource>, AppError> {
        let campaign = sqlx::query_as::<_, CampaignSource>(
            r#"
            SELECT id, campaign_code, campaign_name, description, source_type, country_code,
                   agent_id, partner_name, qr_code_url, short_url, full_url,
                   utm_source, utm_medium, utm_campaign, utm_content, is_active,
                   starts_at, ends_at, click_count, registration_count, claim_count,
                   created_at, updated_at, created_by
            FROM campaign_sources
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(campaign)
    }

    async fn get_campaign_by_code(&self, code: &str) -> Result<Option<CampaignSource>, AppError> {
        let campaign = sqlx::query_as::<_, CampaignSource>(
            r#"
            SELECT id, campaign_code, campaign_name, description, source_type, country_code,
                   agent_id, partner_name, qr_code_url, short_url, full_url,
                   utm_source, utm_medium, utm_campaign, utm_content, is_active,
                   starts_at, ends_at, click_count, registration_count, claim_count,
                   created_at, updated_at, created_by
            FROM campaign_sources
            WHERE campaign_code = $1
            "#,
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(campaign)
    }

    async fn get_country_campaigns(&self, country_code: &str) -> Result<Vec<CampaignSource>, AppError> {
        let campaigns = sqlx::query_as::<_, CampaignSource>(
            r#"
            SELECT id, campaign_code, campaign_name, description, source_type, country_code,
                   agent_id, partner_name, qr_code_url, short_url, full_url,
                   utm_source, utm_medium, utm_campaign, utm_content, is_active,
                   starts_at, ends_at, click_count, registration_count, claim_count,
                   created_at, updated_at, created_by
            FROM campaign_sources
            WHERE country_code = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(country_code)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(campaigns)
    }

    async fn get_agent_campaigns(&self, agent_id: Uuid) -> Result<Vec<CampaignSource>, AppError> {
        let campaigns = sqlx::query_as::<_, CampaignSource>(
            r#"
            SELECT id, campaign_code, campaign_name, description, source_type, country_code,
                   agent_id, partner_name, qr_code_url, short_url, full_url,
                   utm_source, utm_medium, utm_campaign, utm_content, is_active,
                   starts_at, ends_at, click_count, registration_count, claim_count,
                   created_at, updated_at, created_by
            FROM campaign_sources
            WHERE agent_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(agent_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(campaigns)
    }

    async fn get_active_campaigns(&self) -> Result<Vec<CampaignSource>, AppError> {
        let campaigns = sqlx::query_as::<_, CampaignSource>(
            r#"
            SELECT id, campaign_code, campaign_name, description, source_type, country_code,
                   agent_id, partner_name, qr_code_url, short_url, full_url,
                   utm_source, utm_medium, utm_campaign, utm_content, is_active,
                   starts_at, ends_at, click_count, registration_count, claim_count,
                   created_at, updated_at, created_by
            FROM campaign_sources
            WHERE is_active = TRUE
              AND (starts_at IS NULL OR starts_at <= NOW())
              AND (ends_at IS NULL OR ends_at >= NOW())
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(campaigns)
    }

    async fn set_campaign_active(&self, id: Uuid, is_active: bool) -> Result<CampaignSource, AppError> {
        let campaign = sqlx::query_as::<_, CampaignSource>(
            r#"
            UPDATE campaign_sources
            SET is_active = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, campaign_code, campaign_name, description, source_type, country_code,
                      agent_id, partner_name, qr_code_url, short_url, full_url,
                      utm_source, utm_medium, utm_campaign, utm_content, is_active,
                      starts_at, ends_at, click_count, registration_count, claim_count,
                      created_at, updated_at, created_by
            "#,
        )
        .bind(id)
        .bind(is_active)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(campaign)
    }

    async fn increment_campaign_counter(
        &self,
        id: Uuid,
        counter: &str,
    ) -> Result<CampaignSource, AppError> {
        let field = match counter {
            "click" => "click_count",
            "registration" => "registration_count",
            "claim" => "claim_count",
            _ => return Err(AppError::BadRequest(format!("Invalid counter: {}", counter))),
        };

        let query = format!(
            r#"
            UPDATE campaign_sources
            SET {} = {} + 1, updated_at = NOW()
            WHERE id = $1
            RETURNING id, campaign_code, campaign_name, description, source_type, country_code,
                      agent_id, partner_name, qr_code_url, short_url, full_url,
                      utm_source, utm_medium, utm_campaign, utm_content, is_active,
                      starts_at, ends_at, click_count, registration_count, claim_count,
                      created_at, updated_at, created_by
            "#,
            field, field
        );

        let campaign = sqlx::query_as::<_, CampaignSource>(&query)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(campaign)
    }

    async fn record_attribution(&self, input: RecordAttributionInput) -> Result<CampaignAttribution, AppError> {
        let attribution = sqlx::query_as::<_, CampaignAttribution>(
            r#"
            INSERT INTO campaign_attributions (
                campaign_id, user_id, license_id, attribution_type,
                referrer_url, landing_url, user_agent, ip_address, country_code
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8::inet, $9)
            RETURNING id, campaign_id, user_id, license_id, attribution_type,
                      referrer_url, landing_url, user_agent, ip_address::text, country_code, attributed_at
            "#,
        )
        .bind(input.campaign_id)
        .bind(&input.user_id)
        .bind(&input.license_id)
        .bind(&input.attribution_type)
        .bind(&input.referrer_url)
        .bind(&input.landing_url)
        .bind(&input.user_agent)
        .bind(&input.ip_address)
        .bind(&input.country_code)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(attribution)
    }

    async fn get_campaign_attributions(
        &self,
        campaign_id: Uuid,
        attribution_type: Option<&str>,
    ) -> Result<Vec<CampaignAttribution>, AppError> {
        let attributions = if let Some(attr_type) = attribution_type {
            sqlx::query_as::<_, CampaignAttribution>(
                r#"
                SELECT id, campaign_id, user_id, license_id, attribution_type,
                       referrer_url, landing_url, user_agent, ip_address::text, country_code, attributed_at
                FROM campaign_attributions
                WHERE campaign_id = $1 AND attribution_type = $2
                ORDER BY attributed_at DESC
                "#,
            )
            .bind(campaign_id)
            .bind(attr_type)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, CampaignAttribution>(
                r#"
                SELECT id, campaign_id, user_id, license_id, attribution_type,
                       referrer_url, landing_url, user_agent, ip_address::text, country_code, attributed_at
                FROM campaign_attributions
                WHERE campaign_id = $1
                ORDER BY attributed_at DESC
                "#,
            )
            .bind(campaign_id)
            .fetch_all(&self.pool)
            .await
        }
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(attributions)
    }

    async fn get_user_attributions(&self, user_id: &str) -> Result<Vec<CampaignAttribution>, AppError> {
        let attributions = sqlx::query_as::<_, CampaignAttribution>(
            r#"
            SELECT id, campaign_id, user_id, license_id, attribution_type,
                   referrer_url, landing_url, user_agent, ip_address::text, country_code, attributed_at
            FROM campaign_attributions
            WHERE user_id = $1
            ORDER BY attributed_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(attributions)
    }
}
