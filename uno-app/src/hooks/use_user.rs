//! User session context hook for R5-13
//!
//! Provides reactive user session state management for the frontend.
//! Fetches user profile from /api/v1/user/me and provides hooks for
//! authenticated components.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================
// USER DATA TYPES (mirroring server response)
// ============================================

/// User profile from /api/v1/user/me
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserProfile {
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
    /// User's country code
    pub country_code: Option<String>,
}

/// License status for frontend display
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// User loading state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserLoadState {
    #[default]
    Loading,
    Loaded,
    NotAuthenticated,
    Error,
}

// ============================================
// USER CONTEXT
// ============================================

/// User session context for sharing user state across components
#[derive(Clone)]
pub struct UserContext {
    /// Current user profile (None if not authenticated)
    pub user: RwSignal<Option<UserProfile>>,
    /// Loading state
    pub state: RwSignal<UserLoadState>,
    /// Error message if any
    pub error: RwSignal<Option<String>>,
    /// Trigger to refresh user data
    pub refresh_trigger: RwSignal<u32>,
}

impl UserContext {
    /// Check if user is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.user.get().is_some()
    }

    /// Check if user is loading
    pub fn is_loading(&self) -> bool {
        self.state.get() == UserLoadState::Loading
    }

    /// Get user ID if authenticated
    pub fn user_id(&self) -> Option<String> {
        self.user.get().map(|u| u.id)
    }

    /// Get user role if authenticated
    pub fn role(&self) -> Option<String> {
        self.user.get().map(|u| u.role)
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.user.get().map(|u| u.role == role).unwrap_or(false)
    }

    /// Check if user is an admin
    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }

    /// Check if user is an operator
    pub fn is_operator(&self) -> bool {
        self.has_role("operator") || self.is_admin()
    }

    /// Check if user is an agent
    pub fn is_agent(&self) -> bool {
        self.has_role("agent")
    }

    /// Get license ID if user has one
    pub fn license_id(&self) -> Option<String> {
        self.user.get().and_then(|u| u.license_id)
    }

    /// Check if user has an active license
    pub fn has_license(&self) -> bool {
        self.user.get()
            .and_then(|u| u.license_status)
            .map(|s| s.is_active)
            .unwrap_or(false)
    }

    /// Get cohort status if available
    pub fn cohort_status(&self) -> Option<CohortStatus> {
        self.user.get().and_then(|u| u.cohort_status)
    }

    /// Trigger a refresh of user data
    pub fn refresh(&self) {
        self.refresh_trigger.update(|n| *n += 1);
    }

    /// Clear user data (logout)
    pub fn clear(&self) {
        self.user.set(None);
        self.state.set(UserLoadState::NotAuthenticated);
        self.error.set(None);
    }
}

/// Provide user context to the app
pub fn provide_user_context() {
    let user = RwSignal::new(None::<UserProfile>);
    let state = RwSignal::new(UserLoadState::Loading);
    let error = RwSignal::new(None::<String>);
    let refresh_trigger = RwSignal::new(0u32);

    provide_context(UserContext {
        user,
        state,
        error,
        refresh_trigger,
    });

    // Fetch user data on context creation
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::task::spawn_local;

        spawn_local(async move {
            fetch_user_data(user, state, error).await;
        });

        // Re-fetch when refresh_trigger changes
        Effect::new(move |prev: Option<u32>| {
            let current = refresh_trigger.get();
            if let Some(prev_val) = prev {
                if current > prev_val {
                    spawn_local(async move {
                        fetch_user_data(user, state, error).await;
                    });
                }
            }
            current
        });
    }
}

/// Fetch user data from API
#[cfg(any(feature = "csr", feature = "hydrate"))]
async fn fetch_user_data(
    user: RwSignal<Option<UserProfile>>,
    state: RwSignal<UserLoadState>,
    error: RwSignal<Option<String>>,
) {
    use gloo_net::http::Request;

    state.set(UserLoadState::Loading);
    error.set(None);

    match Request::get("/api/v1/user/me")
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 401 {
                // Not authenticated - this is expected for unauthenticated users
                user.set(None);
                state.set(UserLoadState::NotAuthenticated);
            } else if response.ok() {
                match response.json::<UserProfile>().await {
                    Ok(profile) => {
                        user.set(Some(profile));
                        state.set(UserLoadState::Loaded);
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse user profile: {}", e);
                        error.set(Some("Failed to parse user data".to_string()));
                        state.set(UserLoadState::Error);
                    }
                }
            } else {
                let status = response.status();
                tracing::error!("User API returned status {}", status);
                error.set(Some(format!("API error: {}", status)));
                state.set(UserLoadState::Error);
            }
        }
        Err(e) => {
            tracing::error!("Failed to fetch user data: {}", e);
            error.set(Some("Network error".to_string()));
            state.set(UserLoadState::Error);
        }
    }
}

/// Use the user context
pub fn use_user() -> UserContext {
    expect_context::<UserContext>()
}

/// Check if user is authenticated (reactive)
pub fn is_authenticated() -> impl Fn() -> bool + Clone {
    let ctx = use_user();
    move || ctx.is_authenticated()
}

/// Get user profile (reactive)
pub fn current_user() -> impl Fn() -> Option<UserProfile> + Clone {
    let ctx = use_user();
    move || ctx.user.get()
}

/// Get user role (reactive)
pub fn user_role() -> impl Fn() -> Option<String> + Clone {
    let ctx = use_user();
    move || ctx.role()
}
