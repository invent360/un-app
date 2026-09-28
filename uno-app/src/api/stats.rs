//! Statistics-related server functions

use leptos::prelude::*;
use crate::types::{NetworkStats, VisitorStats, TierStats, CountryStats, DashboardStats};

/// Get network overview statistics
#[server(GetNetworkStats, "/api")]
pub async fn get_network_stats() -> Result<NetworkStats, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // Get license summary and convert to NetworkStats
    let summary = factory.license_service.get_summary()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Calculate average user share from split types
    let avg_share = if summary.by_split_type.is_empty() {
        55.0 // Default average
    } else {
        let total_share: i32 = summary.by_split_type.iter()
            .map(|s| s.split_type.user_share() * s.total as i32)
            .sum();
        let total_count: i64 = summary.by_split_type.iter()
            .map(|s| s.total)
            .sum();
        if total_count > 0 {
            total_share as f64 / total_count as f64
        } else {
            55.0
        }
    };

    Ok(NetworkStats::new(
        summary.total,
        summary.claimed,
        summary.by_split_type.len() as i64,
        avg_share,
    ))
}

/// Get visitor statistics for a period
#[server(GetVisitorStats, "/api")]
pub async fn get_visitor_stats(period: Option<String>) -> Result<VisitorStats, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::types::PeriodFilter;

    let factory: Data<ServiceFactory> = extract().await?;

    let period_filter = period
        .map(|p| PeriodFilter::from_str(&p))
        .unwrap_or_default();

    factory.stats_service.get_visitor_stats(period_filter)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get statistics by tier/variant
#[server(GetTierStats, "/api")]
pub async fn get_tier_stats() -> Result<Vec<TierStats>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // Get license summary and convert to TierStats
    let summary = factory.license_service.get_summary()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let tier_stats: Vec<TierStats> = summary.by_split_type
        .into_iter()
        .map(|s| TierStats {
            tier_name: s.split_type.display().to_string(),
            user_share: s.split_type.user_share(),
            operator_share: s.split_type.operator_share(),
            license_count: s.total,
            claimed_count: s.claimed,
            min_earnings: None,
            max_earnings: None,
        })
        .collect();

    Ok(tier_stats)
}

/// Get top countries by visitor count
#[server(GetTopCountries, "/api")]
pub async fn get_top_countries(
    period: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<CountryStats>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::types::PeriodFilter;

    let factory: Data<ServiceFactory> = extract().await?;

    let period_filter = period
        .map(|p| PeriodFilter::from_str(&p))
        .unwrap_or_default();
    let limit = limit.unwrap_or(10);

    factory.stats_service.get_top_countries(period_filter, limit)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get complete dashboard statistics
#[server(GetDashboardStats, "/api")]
pub async fn get_dashboard_stats(period: Option<String>) -> Result<DashboardStats, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::types::PeriodFilter;

    let factory: Data<ServiceFactory> = extract().await?;

    let period_filter = period
        .map(|p| PeriodFilter::from_str(&p))
        .unwrap_or_default();

    factory.stats_service.get_dashboard_stats(period_filter)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
