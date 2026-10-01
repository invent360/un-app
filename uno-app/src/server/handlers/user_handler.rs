//! User profile handler for R5-13
//!
//! Provides `/api/v1/user/me` endpoint for frontend user context.
//! Returns authenticated user's profile, license binding, and cohort status.

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::server::extractors::auth::get_authenticated_user;
use crate::server::repositories::{DynCohortRepository, DynExitRepository, ExitStatus};

// ============================================
// RESPONSE TYPES
// ============================================

/// Response for /api/v1/user/me endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeResponse {
    /// User ID from JWT
    pub id: String,
    /// User role (participant, agent, operator, admin)
    pub role: String,
    /// Bound license ID (if any)
    pub license_id: Option<String>,
    /// License status
    pub license_status: Option<LicenseStatus>,
    /// Cohort progress status
    pub cohort_status: Option<CohortStatus>,
    /// Whether user has an active exit request
    pub has_active_exit: bool,
    /// User's country code (from license or profile)
    pub country_code: Option<String>,
}

/// License status for frontend display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseStatus {
    /// License state (reserved, active, suspended, expired, etc.)
    pub state: String,
    /// License variant/split type
    pub variant: Option<String>,
    /// Expiry date (ISO format)
    pub expires_at: Option<String>,
    /// Whether license is usable for tasks
    pub is_active: bool,
}

/// Cohort progress status for D1/D7/D30 display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortStatus {
    /// Cohort start date
    pub cohort_date: String,
    /// Days since cohort started
    pub days_elapsed: i32,
    /// D1 milestone completed
    pub d1_completed: bool,
    /// D3 milestone completed
    pub d3_completed: bool,
    /// D7 milestone completed
    pub d7_completed: bool,
    /// Days active towards D7 (target: 4)
    pub d7_active_days: i32,
    /// D30 milestone completed
    pub d30_completed: bool,
    /// Days active towards D30
    pub d30_active_days: i32,
    /// Current milestone name for display
    pub current_milestone: String,
    /// Next required action
    pub next_action: Option<String>,
}

// ============================================
// HANDLERS
// ============================================

/// GET /api/v1/user/me
/// Get current authenticated user's profile
/// F3: Added pool for direct license status queries
pub async fn get_me(
    req: HttpRequest,
    pool: web::Data<crate::server::db::ConnectionPool>,
    cohort_repo: web::Data<DynCohortRepository>,
    exit_repo: web::Data<DynExitRepository>,
) -> HttpResponse {
    // Get authenticated user from JWT
    let user = match get_authenticated_user(&req) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required"
            }));
        }
    };
    let user_id = &user.id;
    let role = user.role.clone();

    // Find user's cohort (which contains license binding)
    // Use the most recent cohort as the active one
    let user_cohorts = match cohort_repo.get_user_cohorts(user_id).await {
        Ok(cohorts) => cohorts,
        Err(e) => {
            tracing::error!(user_id = %user_id, error = %e, "Failed to fetch user cohorts");
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch user data"
            }));
        }
    };
    let active_cohort = user_cohorts.into_iter().max_by_key(|c| c.cohort_date);

    let (license_id, cohort_status) = match active_cohort {
        Some(cohort) => {
            let today = chrono::Utc::now().date_naive();
            let days_elapsed = (today - cohort.cohort_date).num_days() as i32;

            // Determine current milestone and next action
            let (current_milestone, next_action) = if !cohort.d1_completed {
                ("onboarding".to_string(), Some("Complete app setup".to_string()))
            } else if !cohort.d7_completed && days_elapsed <= 7 {
                let remaining = 4 - cohort.d7_active_days;
                (
                    "d7_progress".to_string(),
                    Some(format!("Complete {} more active days", remaining.max(0))),
                )
            } else if !cohort.d30_completed && days_elapsed <= 30 {
                ("d30_progress".to_string(), Some("Stay active for D30".to_string()))
            } else if cohort.d30_completed {
                ("graduated".to_string(), None)
            } else {
                ("active".to_string(), None)
            };

            let status = CohortStatus {
                cohort_date: cohort.cohort_date.to_string(),
                days_elapsed,
                d1_completed: cohort.d1_completed,
                d3_completed: cohort.d3_completed,
                d7_completed: cohort.d7_completed,
                d7_active_days: cohort.d7_active_days,
                d30_completed: cohort.d30_completed,
                d30_active_days: cohort.d30_active_days,
                current_milestone,
                next_action,
            };

            (Some(cohort.license_id.clone()), Some(status))
        }
        None => (None, None),
    };

    // Check for active exit request
    let has_active_exit = if let Some(ref lid) = license_id {
        match exit_repo.get_exit_by_license(lid).await {
            Ok(Some(exit)) => {
                exit.status != ExitStatus::Completed && exit.status != ExitStatus::Rejected
            }
            Ok(None) => false,
            Err(e) => {
                tracing::warn!(license_id = %lid, error = %e, "Failed to check exit status");
                false
            }
        }
    } else {
        false
    };

    // F3: Query actual license state from database instead of inferring from cohort
    let license_status = if let Some(ref lid) = license_id {
        // Fetch real license state
        #[derive(sqlx::FromRow)]
        struct LicenseRow {
            claimed: bool,
            is_quarantined: Option<bool>,
            valid_to: chrono::DateTime<chrono::Utc>,
            split_type: Option<String>,
        }

        match sqlx::query_as::<_, LicenseRow>(r#"
            SELECT claimed, is_quarantined, valid_to, split_type::text
            FROM licenses WHERE id = $1
        "#)
        .bind(lid)
        .fetch_optional(pool.as_ref())
        .await
        {
            Ok(Some(lic)) => {
                let is_quarantined = lic.is_quarantined.unwrap_or(false);
                let is_expired = lic.valid_to <= chrono::Utc::now();

                let state = if has_active_exit {
                    "exiting".to_string()
                } else if is_quarantined {
                    "quarantined".to_string()
                } else if is_expired {
                    "expired".to_string()
                } else if !lic.claimed {
                    "unclaimed".to_string()
                } else {
                    "active".to_string()
                };

                let is_active = lic.claimed && !is_quarantined && !is_expired && !has_active_exit;

                Some(LicenseStatus {
                    state,
                    variant: lic.split_type,
                    expires_at: Some(lic.valid_to.to_rfc3339()),
                    is_active,
                })
            }
            Ok(None) => {
                tracing::warn!(license_id = %lid, "License not found in database");
                None
            }
            Err(e) => {
                tracing::error!(license_id = %lid, error = %e, "Failed to fetch license status");
                // Return degraded response rather than failing completely
                Some(LicenseStatus {
                    state: if has_active_exit { "exiting".to_string() } else { "unknown".to_string() },
                    variant: None,
                    expires_at: None,
                    is_active: !has_active_exit,
                })
            }
        }
    } else {
        None
    };

    let response = MeResponse {
        id: user_id.clone(),
        role,
        license_id,
        license_status,
        cohort_status,
        has_active_exit,
        country_code: None, // Could be added later from market attribution
    };

    HttpResponse::Ok().json(response)
}

/// GET /api/v1/user/status
/// Quick status check (lighter than full /api/me)
pub async fn get_status(req: HttpRequest) -> HttpResponse {
    let user = match get_authenticated_user(&req) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "authenticated": false,
                "error": "Authentication required"
            }));
        }
    };

    #[derive(Serialize)]
    struct StatusResponse {
        authenticated: bool,
        user_id: String,
        role: String,
    }

    HttpResponse::Ok().json(StatusResponse {
        authenticated: true,
        user_id: user.id,
        role: user.role,
    })
}
