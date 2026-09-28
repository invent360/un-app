//! Stats repository for visitor and analytics data

use async_trait::async_trait;
use sqlx::PgPool;
use crate::types::{AppError, CountryStats, CountryStatsRow};

/// Stats repository trait
#[async_trait]
pub trait StatsRepository: Send + Sync {
    /// Get count of unique visitors (distinct IPs)
    async fn get_unique_visitor_count(&self, period_days: i32) -> Result<i64, AppError>;
    /// Get total visits (sum of all visit counts)
    async fn get_total_visits(&self, period_days: i32) -> Result<i64, AppError>;
    async fn get_visitors_by_country(&self, period_days: i32, limit: i32) -> Result<Vec<CountryStats>, AppError>;
    async fn record_visitor(&self, ip: &str, country_code: Option<&str>, user_agent: Option<&str>) -> Result<(), AppError>;
}

/// Stats repository implementation
pub struct StatsRepositoryImpl {
    db_pool: PgPool,
}

impl StatsRepositoryImpl {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl StatsRepository for StatsRepositoryImpl {
    async fn get_unique_visitor_count(&self, period_days: i32) -> Result<i64, AppError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(DISTINCT ip_address)
            FROM visitors
            WHERE visited_at > NOW() - INTERVAL '1 day' * $1
            "#
        )
        .bind(period_days)
        .fetch_one(&self.db_pool)
        .await
        .unwrap_or(0);

        Ok(result)
    }

    async fn get_total_visits(&self, period_days: i32) -> Result<i64, AppError> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COALESCE(SUM(visit_count), 0)
            FROM visitors
            WHERE visited_at > NOW() - INTERVAL '1 day' * $1
            "#
        )
        .bind(period_days)
        .fetch_one(&self.db_pool)
        .await
        .unwrap_or(0);

        Ok(result)
    }

    async fn get_visitors_by_country(&self, period_days: i32, limit: i32) -> Result<Vec<CountryStats>, AppError> {
        let rows = sqlx::query_as::<_, CountryStatsRow>(
            r#"
            SELECT
                COALESCE(country_code, 'Unknown') as country_code,
                COUNT(DISTINCT ip_address) as visitor_count
            FROM visitors
            WHERE visited_at > NOW() - INTERVAL '1 day' * $1
            GROUP BY country_code
            ORDER BY visitor_count DESC
            LIMIT $2
            "#
        )
        .bind(period_days)
        .bind(limit)
        .fetch_all(&self.db_pool)
        .await
        .unwrap_or_default();

        // Convert rows to stats with computed flag
        let stats = rows.into_iter().map(|r| r.into_stats()).collect();
        Ok(stats)
    }

    async fn record_visitor(&self, ip: &str, country_code: Option<&str>, user_agent: Option<&str>) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO visitors (ip_address, country_code, user_agent, visited_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (ip_address) 
            DO UPDATE SET visited_at = NOW(), visit_count = visitors.visit_count + 1
            "#
        )
        .bind(ip)
        .bind(country_code)
        .bind(user_agent)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}

pub type DynStatsRepository = std::sync::Arc<dyn StatsRepository>;
