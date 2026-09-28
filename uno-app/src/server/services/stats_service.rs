//! Stats service for statistics business logic

use crate::types::{
    VisitorStats, CountryStats, TierStats,
    DashboardStats, PeriodFilter, AppError
};
use crate::server::repositories::DynStatsRepository;

/// Stats service for statistics operations
#[derive(Clone)]
pub struct StatsServiceImpl {
    stats_repo: DynStatsRepository,
}

impl StatsServiceImpl {
    pub fn new(stats_repo: DynStatsRepository) -> Self {
        Self { stats_repo }
    }

    /// Get visitor statistics for a period
    pub async fn get_visitor_stats(&self, period: PeriodFilter) -> Result<VisitorStats, AppError> {
        let days = period.days();

        let total_visitors = self.stats_repo.get_total_visits(days).await?;
        let unique_visitors = self.stats_repo.get_unique_visitor_count(days).await?;
        let countries = self.stats_repo.get_visitors_by_country(days, 10).await?;

        Ok(VisitorStats {
            period: period.display_name().to_string(),
            total_visitors,
            unique_visitors,
            countries,
        })
    }

    /// Get top countries by visitor count
    pub async fn get_top_countries(&self, period: PeriodFilter, limit: i32) -> Result<Vec<CountryStats>, AppError> {
        let days = period.days();
        self.stats_repo.get_visitors_by_country(days, limit).await
    }

    /// Get dashboard visitor statistics
    pub async fn get_dashboard_stats(&self, period: PeriodFilter) -> Result<DashboardStats, AppError> {
        let visitors = self.get_visitor_stats(period).await?;
        let top_countries = self.get_top_countries(period, 10).await?;

        Ok(DashboardStats {
            visitors,
            top_countries,
        })
    }
}
