//! R3-13: Journey service for onboarding progress tracking
//!
//! Manages user onboarding state transitions and progress persistence.

use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::server::repositories::{DynImmutableAuditRepository, AuditEvent, AuditCategory, AuditEventType, ActorType, AuditOutcome};
use crate::types::{AppError, OnboardingState, OnboardingProgress, EligibilityProgress, SetupStep};

// ============================================
// TYPES
// ============================================

/// Journey entry for funnel tracking
#[derive(Debug, Clone)]
pub struct JourneyEntry {
    pub id: Uuid,
    pub session_id: Option<Uuid>,
    pub visitor_id: Option<String>,
    pub license_id: Option<Uuid>,
    pub stage_id: String,
    pub measured_at: DateTime<Utc>,
    pub metadata: Option<JsonValue>,
}

/// Funnel metrics for a stage
#[derive(Debug, Clone, Default)]
pub struct StageMetrics {
    pub stage_id: String,
    pub total_entries: i64,
    pub unique_users: i64,
    pub conversions: i64,
    pub conversion_rate: f64,
}

/// Dynamic type alias for dependency injection
pub type DynJourneyService = Arc<dyn JourneyService + Send + Sync>;

// ============================================
// TRAIT
// ============================================

#[async_trait::async_trait]
pub trait JourneyService: Send + Sync {
    /// Get or create onboarding progress for a user
    async fn get_or_create_progress(&self, user_id: Uuid) -> Result<OnboardingProgress, AppError>;

    /// Get existing progress (if any)
    async fn get_progress(&self, user_id: Uuid) -> Result<Option<OnboardingProgress>, AppError>;

    /// Resume onboarding from paused state
    async fn resume_onboarding(&self, user_id: Uuid) -> Result<OnboardingProgress, AppError>;

    /// Transition to new state
    async fn transition_state(
        &self,
        user_id: Uuid,
        new_state: OnboardingState,
        actor_id: Option<&str>,
    ) -> Result<OnboardingProgress, AppError>;

    /// Record funnel stage for analytics
    async fn record_funnel_stage(
        &self,
        session_id: Option<Uuid>,
        visitor_id: Option<&str>,
        stage_id: &str,
        license_id: Option<Uuid>,
        metadata: Option<JsonValue>,
    ) -> Result<JourneyEntry, AppError>;

    /// Get funnel metrics for date range
    async fn get_funnel_metrics(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<StageMetrics>, AppError>;

    /// Mark eligibility check as complete
    async fn complete_eligibility(
        &self,
        user_id: Uuid,
        country_code: &str,
        device_verified: bool,
        referral_code: Option<&str>,
    ) -> Result<OnboardingProgress, AppError>;

    /// Accept economics/terms
    async fn accept_economics(
        &self,
        user_id: Uuid,
        consent_version: Uuid,
    ) -> Result<OnboardingProgress, AppError>;

    /// Complete setup step
    async fn complete_setup_step(
        &self,
        user_id: Uuid,
        step: SetupStep,
    ) -> Result<OnboardingProgress, AppError>;

    /// Reserve license
    async fn reserve_license(
        &self,
        user_id: Uuid,
        license_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<OnboardingProgress, AppError>;

    /// Activate license (complete onboarding)
    async fn activate_license(
        &self,
        user_id: Uuid,
        license_id: Uuid,
    ) -> Result<OnboardingProgress, AppError>;
}

// ============================================
// IMPLEMENTATION
// ============================================

pub struct JourneyServiceImpl {
    pool: ConnectionPool,
    audit_repo: DynImmutableAuditRepository,
}

impl JourneyServiceImpl {
    pub fn new(pool: ConnectionPool, audit_repo: DynImmutableAuditRepository) -> Self {
        Self { pool, audit_repo }
    }

    async fn log_transition(
        &self,
        user_id: Uuid,
        from_state: &str,
        to_state: &str,
        actor_id: Option<&str>,
    ) -> Result<(), AppError> {
        let actor = actor_id.unwrap_or(&user_id.to_string()).to_string();

        let event = AuditEvent::builder(
            AuditEventType::Custom("onboarding.state_transition".to_string()),
            AuditCategory::User,
        )
        .actor(ActorType::User, &actor)
        .resource("onboarding_progress", &user_id.to_string())
        .action("state_transition")
        .outcome(AuditOutcome::Success)
        .event_data(serde_json::json!({
            "from_state": from_state,
            "to_state": to_state
        }))
        .build();

        self.audit_repo.log_event(event).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl JourneyService for JourneyServiceImpl {
    async fn get_or_create_progress(&self, user_id: Uuid) -> Result<OnboardingProgress, AppError> {
        // Try to get existing progress
        if let Some(progress) = self.get_progress(user_id).await? {
            return Ok(progress);
        }

        // Create new progress
        let progress = OnboardingProgress::new(user_id);
        let state_json = serde_json::to_string(&progress.state)
            .map_err(|e| AppError::Database(format!("Failed to serialize state: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO onboarding_progress (id, user_id, state_json, started_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (user_id) DO NOTHING
            "#,
        )
        .bind(progress.id)
        .bind(user_id)
        .bind(&state_json)
        .bind(progress.started_at)
        .bind(progress.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // Re-fetch in case of race condition
        self.get_progress(user_id).await?.ok_or_else(|| {
            AppError::Database("Failed to create onboarding progress".to_string())
        })
    }

    async fn get_progress(&self, user_id: Uuid) -> Result<Option<OnboardingProgress>, AppError> {
        let row = sqlx::query_as::<_, (Uuid, Uuid, Option<Uuid>, String, DateTime<Utc>, DateTime<Utc>, Option<DateTime<Utc>>)>(
            r#"
            SELECT id, user_id, session_id, state_json, started_at, updated_at, completed_at
            FROM onboarding_progress
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        match row {
            Some((id, user_id, session_id, state_json, started_at, updated_at, completed_at)) => {
                let state: OnboardingState = serde_json::from_str(&state_json)
                    .unwrap_or(OnboardingState::Landing);

                Ok(Some(OnboardingProgress {
                    id,
                    user_id,
                    session_id,
                    state,
                    started_at,
                    updated_at,
                    completed_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn resume_onboarding(&self, user_id: Uuid) -> Result<OnboardingProgress, AppError> {
        let progress = self.get_or_create_progress(user_id).await?;

        // If paused, restore previous state
        if let OnboardingState::Paused { previous_state, .. } = &progress.state {
            return self.transition_state(user_id, *previous_state.clone(), None).await;
        }

        // If can resume, just return current state
        if progress.state.can_resume() {
            return Ok(progress);
        }

        // If terminal, start fresh
        if progress.state.is_terminal() {
            return self.transition_state(user_id, OnboardingState::Landing, None).await;
        }

        Ok(progress)
    }

    async fn transition_state(
        &self,
        user_id: Uuid,
        new_state: OnboardingState,
        actor_id: Option<&str>,
    ) -> Result<OnboardingProgress, AppError> {
        let progress = self.get_or_create_progress(user_id).await?;
        let old_stage = progress.state.funnel_stage();
        let new_stage = new_state.funnel_stage();

        let state_json = serde_json::to_string(&new_state)
            .map_err(|e| AppError::Database(format!("Failed to serialize state: {}", e)))?;

        let completed_at = if new_state.is_terminal() && matches!(new_state, OnboardingState::Active { .. }) {
            Some(Utc::now())
        } else {
            None
        };

        sqlx::query(
            r#"
            UPDATE onboarding_progress
            SET state_json = $2, updated_at = NOW(), completed_at = COALESCE($3, completed_at)
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .bind(&state_json)
        .bind(completed_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // Log audit event
        self.log_transition(user_id, old_stage, new_stage, actor_id).await?;

        self.get_progress(user_id).await?.ok_or_else(|| {
            AppError::Database("Progress not found after update".to_string())
        })
    }

    async fn record_funnel_stage(
        &self,
        session_id: Option<Uuid>,
        visitor_id: Option<&str>,
        stage_id: &str,
        license_id: Option<Uuid>,
        metadata: Option<JsonValue>,
    ) -> Result<JourneyEntry, AppError> {
        let entry = JourneyEntry {
            id: Uuid::new_v4(),
            session_id,
            visitor_id: visitor_id.map(String::from),
            license_id,
            stage_id: stage_id.to_string(),
            measured_at: Utc::now(),
            metadata: metadata.clone(),
        };

        sqlx::query(
            r#"
            INSERT INTO user_journey_events (id, session_id, visitor_id, license_id, stage_id, measured_at, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(entry.id)
        .bind(session_id)
        .bind(visitor_id)
        .bind(license_id)
        .bind(stage_id)
        .bind(entry.measured_at)
        .bind(metadata)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(entry)
    }

    async fn get_funnel_metrics(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<StageMetrics>, AppError> {
        let rows = sqlx::query_as::<_, (String, i64, i64)>(
            r#"
            SELECT
                stage_id,
                COUNT(*) as total_entries,
                COUNT(DISTINCT COALESCE(visitor_id, session_id::text)) as unique_users
            FROM user_journey_events
            WHERE measured_at BETWEEN $1 AND $2
            GROUP BY stage_id
            ORDER BY MIN(measured_at)
            "#,
        )
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|(stage_id, total, unique)| {
            StageMetrics {
                stage_id,
                total_entries: total,
                unique_users: unique,
                conversions: 0, // Would need to calculate based on next stage
                conversion_rate: 0.0,
            }
        }).collect())
    }

    async fn complete_eligibility(
        &self,
        user_id: Uuid,
        country_code: &str,
        device_verified: bool,
        referral_code: Option<&str>,
    ) -> Result<OnboardingProgress, AppError> {
        // F3: Actually verify the country code against supported countries
        // Check if the country has any active pilot cohorts (meaning it's supported)
        // B5 FIX: Use correct column names from schema (market_code, is_active)
        let country_verified: bool = if country_code.is_empty() {
            false
        } else {
            let result: (bool,) = sqlx::query_as(r#"
                SELECT EXISTS(
                    SELECT 1 FROM pilot_cohorts
                    WHERE market_code = $1
                      AND is_active = true
                      AND NOW() BETWEEN start_date AND COALESCE(end_date, '2099-12-31')
                )
            "#)
            .bind(country_code.to_uppercase())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
            result.0
        };

        // F3: Actually validate the referral code against the database
        // Check that referral exists, is active, and agent is approved
        let referral_validated: Option<bool> = if let Some(code) = referral_code {
            let code = code.trim().to_uppercase();
            if code.is_empty() {
                None
            } else {
                let result: (bool,) = sqlx::query_as(r#"
                    SELECT EXISTS(
                        SELECT 1 FROM referrals r
                        JOIN agents a ON r.agent_id = a.id
                        WHERE r.referral_code = $1
                          AND r.status = 'active'
                          AND a.status = 'approved'
                    )
                "#)
                .bind(&code)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
                Some(result.0)
            }
        } else {
            None
        };

        let new_state = OnboardingState::Eligibility {
            progress: EligibilityProgress {
                country_verified,
                device_verified,
                referral_validated,
            },
            referral_code: referral_code.map(String::from),
        };

        self.transition_state(user_id, new_state, None).await
    }

    async fn accept_economics(
        &self,
        user_id: Uuid,
        consent_version: Uuid,
    ) -> Result<OnboardingProgress, AppError> {
        let new_state = OnboardingState::Economics {
            consent_version,
            accepted_at: Some(Utc::now()),
        };

        self.transition_state(user_id, new_state, None).await
    }

    async fn complete_setup_step(
        &self,
        user_id: Uuid,
        step: SetupStep,
    ) -> Result<OnboardingProgress, AppError> {
        // Move to next step or reservation
        let new_state = match step.next() {
            Some(next_step) => OnboardingState::Setup { step: next_step },
            None => {
                // Setup complete, ready for reservation
                // Note: actual reservation will be done by reservation service
                OnboardingState::Setup { step: SetupStep::Confirmation }
            }
        };

        self.transition_state(user_id, new_state, None).await
    }

    async fn reserve_license(
        &self,
        user_id: Uuid,
        license_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<OnboardingProgress, AppError> {
        let new_state = OnboardingState::Reservation {
            license_id,
            expires_at,
        };

        self.transition_state(user_id, new_state, None).await
    }

    async fn activate_license(
        &self,
        user_id: Uuid,
        license_id: Uuid,
    ) -> Result<OnboardingProgress, AppError> {
        // F3: Verify the license exists, is claimed by this user, and is not quarantined/expired
        let license_valid: Option<(bool, bool, bool)> = sqlx::query_as(r#"
            SELECT
                claimed,
                issued_to = $2 as is_owner,
                (is_quarantined = false OR is_quarantined IS NULL) AND valid_to > NOW() as is_valid
            FROM licenses
            WHERE id = $1
        "#)
        .bind(license_id)
        .bind(user_id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        match license_valid {
            None => {
                return Err(AppError::NotFound(format!("License {} not found", license_id)));
            }
            Some((false, _, _)) => {
                return Err(AppError::BadRequest("License has not been claimed yet".into()));
            }
            Some((_, false, _)) => {
                tracing::warn!(
                    license_id = %license_id,
                    user_id = %user_id,
                    "SECURITY: User attempted to activate license they don't own"
                );
                return Err(AppError::Unauthorized("This license belongs to a different user".into()));
            }
            Some((_, _, false)) => {
                return Err(AppError::BadRequest("License is quarantined or expired".into()));
            }
            Some((true, true, true)) => {
                // License is valid, claimed, owned by user, and not quarantined/expired
            }
        }

        let new_state = OnboardingState::Active {
            license_id,
            activated_at: Utc::now(),
        };

        self.transition_state(user_id, new_state, None).await
    }
}
