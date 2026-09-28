//! Analytics handler for fetching referrals, visitor stats, and incentive data

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// ========================================
// Daily Incentive Totals
// ========================================

/// Daily incentive data point for charting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyIncentive {
    pub date: String,
    pub amount: f64,
    pub license_count: i32,
}

/// Fetch daily incentive totals for the last 30 days
#[server(GetDailyIncentiveTotals, "/api")]
pub async fn get_daily_incentive_totals() -> Result<Vec<DailyIncentive>, ServerFnError> {
    use chrono::{Duration, Utc};
    use crate::db::get_db;
    use crate::repository::scylla::RewardRepository;
    use crate::repository::traits::RewardRepositoryTrait;

    let pool = get_db()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?
        .clone();

    let reward_repo = RewardRepository::new(pool);

    // Calculate date range: last 30 days
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let start_str = start_date.format("%Y-%m-%dT00:00:00+00:00").to_string();
    let end_str = end_date.format("%Y-%m-%dT23:59:59+00:00").to_string();

    // Get daily totals from repository
    let daily_totals = reward_repo
        .get_daily_totals(&start_str, &end_str)
        .await
        .map_err(|e| ServerFnError::new(e))?;

    // Build maps of existing data (amount and license count)
    let mut totals_map: std::collections::HashMap<String, (i64, i32)> = daily_totals
        .into_iter()
        .map(|(date, amount, license_count)| (date, (amount, license_count)))
        .collect();

    // Generate all 30 days and fill in missing dates with 0
    let mut result: Vec<DailyIncentive> = Vec::with_capacity(30);
    for i in 0..30 {
        let date = (end_date - Duration::days(29 - i)).format("%Y-%m-%d").to_string();
        let (amount_micros, license_count) = totals_map.remove(&date).unwrap_or((0, 0));
        // Convert micros to display units (divide by 1,000,000)
        let amount = amount_micros as f64 / 1_000_000.0;
        result.push(DailyIncentive { date, amount, license_count });
    }

    Ok(result)
}

// ========================================
// Referrals from uno-app
// ========================================

/// Referral data returned from uno-app
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnoAppReferral {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub country_code: String,
    pub referral_code: String,
    pub status: String,
    pub created_at: String,
}

/// Response from fetching referrals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralsListResponse {
    pub referrals: Vec<UnoAppReferral>,
    pub total: i64,
}

/// Fetch all referrals from uno-app
#[server(GetUnoAppReferrals, "/api")]
pub async fn get_uno_app_referrals() -> Result<ReferralsListResponse, ServerFnError> {
    use crate::api::UnoLicenseClient;

    let client = UnoLicenseClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    let response = client.get_referrals().await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch referrals: {}", e)))?;

    let referrals: Vec<UnoAppReferral> = response.referrals.into_iter().map(|r| {
        UnoAppReferral {
            id: r.id,
            username: r.username,
            email: r.email,
            country_code: r.country_code,
            referral_code: r.referral_code,
            status: r.status,
            created_at: r.created_at,
        }
    }).collect();

    Ok(ReferralsListResponse {
        total: referrals.len() as i64,
        referrals,
    })
}

// ========================================
// Visitor Stats from uno-app
// ========================================

/// Country visitor statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryVisitorStat {
    pub country_code: String,
    pub visitor_count: i64,
    pub unique_visitors: i64,
}

/// Response from fetching visitor stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitorStatsListResponse {
    pub stats: Vec<CountryVisitorStat>,
    pub total_visitors: i64,
    pub unique_visitors: i64,
    pub period: String,
}

/// Fetch visitor stats from uno-app
#[server(GetUnoAppVisitorStats, "/api")]
pub async fn get_uno_app_visitor_stats(
    period: Option<String>,
    limit: Option<i32>,
) -> Result<VisitorStatsListResponse, ServerFnError> {
    use crate::api::UnoLicenseClient;

    let client = UnoLicenseClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    let response = client.get_visitor_stats(period.as_deref(), limit).await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch visitor stats: {}", e)))?;

    let stats: Vec<CountryVisitorStat> = response.stats.into_iter().map(|s| {
        CountryVisitorStat {
            country_code: s.country_code,
            visitor_count: s.visitor_count,
            unique_visitors: s.unique_visitors,
        }
    }).collect();

    Ok(VisitorStatsListResponse {
        stats,
        total_visitors: response.total_visitors,
        unique_visitors: response.unique_visitors,
        period: response.period,
    })
}

#[cfg(feature = "ssr")]
pub fn register_analytics_server_fns() {
    use server_fn::ServerFn;

    println!("Registering analytics server functions:");
    println!("  GetUnoAppReferrals: {}", GetUnoAppReferrals::url());
    println!("  GetUnoAppVisitorStats: {}", GetUnoAppVisitorStats::url());
    println!("  GetDailyIncentiveTotals: {}", GetDailyIncentiveTotals::url());
}
