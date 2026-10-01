//! Dashboard API client for R5-13
//!
//! Fetches dashboard data from /api/v1/dashboard for authenticated users.
//! Used by the participant home dashboard page.

use serde::{Deserialize, Serialize};

// ============================================
// RESPONSE TYPES (mirrors server handler)
// ============================================

/// Dashboard response from /api/v1/dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub user_id: String,
    pub license_id: String,
    pub cohort: Option<CohortSummary>,
    pub support: SupportSummary,
    pub exit_status: Option<ExitSummary>,
}

/// Cohort progress summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortSummary {
    pub cohort_date: String,
    pub d1_completed: bool,
    pub d3_completed: bool,
    pub d7_completed: bool,
    pub d7_active_days: i32,
    pub d7_target_days: i32,
    pub d30_completed: bool,
    pub d30_active_days: i32,
    pub days_since_activation: i64,
}

/// Support ticket summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportSummary {
    pub open_tickets: i64,
    pub pending_response: i64,
    pub recent_ticket_number: Option<String>,
}

/// Exit/leave status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitSummary {
    pub has_active_exit: bool,
    pub status: Option<String>,
    pub payout_status: Option<String>,
    pub net_payout_micros: Option<i64>,
}

/// Activity item from /api/v1/dashboard/activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityItem {
    pub activity_date: String,
    pub activity_type: String,
    pub description: String,
    pub metadata: Option<serde_json::Value>,
}

// ============================================
// CLIENT-SIDE API FUNCTIONS
// ============================================

/// Fetch dashboard data for authenticated user
#[cfg(any(feature = "csr", feature = "hydrate"))]
pub async fn fetch_dashboard(user_id: &str, license_id: &str) -> Result<DashboardData, String> {
    use gloo_net::http::Request;

    let url = format!(
        "/api/v1/dashboard?user_id={}&license_id={}",
        urlencoding::encode(user_id),
        urlencoding::encode(license_id)
    );

    let response = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status() == 401 {
        return Err("Authentication required".to_string());
    }

    if !response.ok() {
        return Err(format!("API error: {}", response.status()));
    }

    response
        .json::<DashboardData>()
        .await
        .map_err(|e| format!("Parse error: {}", e))
}

/// Fetch recent activities for dashboard
#[cfg(any(feature = "csr", feature = "hydrate"))]
pub async fn fetch_activities(user_id: &str, license_id: &str) -> Result<Vec<ActivityItem>, String> {
    use gloo_net::http::Request;

    let url = format!(
        "/api/v1/dashboard/activities/{}/{}",
        urlencoding::encode(user_id),
        urlencoding::encode(license_id)
    );

    let response = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() {
        return Err(format!("API error: {}", response.status()));
    }

    response
        .json::<Vec<ActivityItem>>()
        .await
        .map_err(|e| format!("Parse error: {}", e))
}
