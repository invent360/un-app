//! ScyllaDB implementation of LicenseAnalyticsRepositoryTrait

use async_trait::async_trait;
use chrono::Utc;
use scylla::frame::value::CqlTimestamp;
use scylla::Session;
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::db::DbPool;
use crate::models::entity::{LicenseAnalyticsEntity, NewLicenseAnalytics};
use crate::repository::traits::LicenseAnalyticsRepositoryTrait;

type AnalyticsRow = (String, String, f64, f64, CqlTimestamp, CqlTimestamp);

/// ScyllaDB implementation of the License Analytics repository
pub struct LicenseAnalyticsRepository {
    session: Arc<Session>,
}

impl LicenseAnalyticsRepository {
    /// Create a new LicenseAnalyticsRepository with the given session
    pub fn new(session: DbPool) -> Self {
        Self { session }
    }

    fn timestamp_to_datetime(ts: CqlTimestamp) -> String {
        chrono::DateTime::from_timestamp_millis(ts.0)
            .map(|d| d.to_rfc3339())
            .unwrap_or_else(|| Utc::now().to_rfc3339())
    }

    fn parse_single_analytics(result: scylla::QueryResult) -> Result<Option<LicenseAnalyticsEntity>, String> {
        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        let rows = match rows_result.rows::<AnalyticsRow>() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        for row_result in rows {
            if let Ok((license_id, date, uptime, required_uptime, created_at, updated_at)) = row_result {
                return Ok(Some(LicenseAnalyticsEntity {
                    license_id,
                    date,
                    uptime,
                    required_uptime,
                    created_at: Self::timestamp_to_datetime(created_at),
                    updated_at: Self::timestamp_to_datetime(updated_at),
                }));
            }
        }

        Ok(None)
    }

    fn parse_analytics_list(result: scylla::QueryResult) -> Result<Vec<LicenseAnalyticsEntity>, String> {
        let mut analytics = Vec::new();

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(analytics),
        };

        let rows = match rows_result.rows::<AnalyticsRow>() {
            Ok(r) => r,
            Err(_) => return Ok(analytics),
        };

        for row_result in rows {
            if let Ok((license_id, date, uptime, required_uptime, created_at, updated_at)) = row_result {
                analytics.push(LicenseAnalyticsEntity {
                    license_id,
                    date,
                    uptime,
                    required_uptime,
                    created_at: Self::timestamp_to_datetime(created_at),
                    updated_at: Self::timestamp_to_datetime(updated_at),
                });
            }
        }

        Ok(analytics)
    }
}

#[async_trait]
impl LicenseAnalyticsRepositoryTrait for LicenseAnalyticsRepository {
    async fn upsert_analytics(&self, analytics: NewLicenseAnalytics) -> Result<LicenseAnalyticsEntity, String> {
        let now = Utc::now();
        let now_ts = CqlTimestamp(now.timestamp_millis());

        let query = r#"
            INSERT INTO license_analytics (license_id, date, uptime, required_uptime, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
        "#;

        self.session
            .query_unpaged(
                query,
                (
                    &analytics.license_id,
                    &analytics.date,
                    analytics.uptime,
                    analytics.required_uptime,
                    now_ts,
                    now_ts,
                ),
            )
            .await
            .map_err(|e| {
                error!("Failed to upsert analytics: {}", e);
                e.to_string()
            })?;

        debug!("Upserted analytics for license {} on {}", analytics.license_id, analytics.date);

        Ok(LicenseAnalyticsEntity {
            license_id: analytics.license_id,
            date: analytics.date,
            uptime: analytics.uptime,
            required_uptime: analytics.required_uptime,
            created_at: now.to_rfc3339(),
            updated_at: now.to_rfc3339(),
        })
    }

    async fn upsert_analytics_batch(&self, analytics: Vec<NewLicenseAnalytics>) -> Result<usize, String> {
        let mut count = 0;

        for item in analytics {
            match self.upsert_analytics(item).await {
                Ok(_) => count += 1,
                Err(e) => {
                    error!("Failed to upsert analytics in batch: {}", e);
                }
            }
        }

        info!("Upserted {} analytics records", count);
        Ok(count)
    }

    async fn get_analytics_by_license(
        &self,
        license_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<LicenseAnalyticsEntity>, String> {
        let query = "SELECT license_id, date, uptime, required_uptime, created_at, updated_at FROM license_analytics WHERE license_id = ? AND date >= ? AND date <= ?";

        let result = self
            .session
            .query_unpaged(query, (license_id, start_date, end_date))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_analytics_list(result)
    }

    async fn get_latest_analytics(&self, license_id: &str) -> Result<Option<LicenseAnalyticsEntity>, String> {
        // Since clustering is DESC by date, first result is the latest
        let query = "SELECT license_id, date, uptime, required_uptime, created_at, updated_at FROM license_analytics WHERE license_id = ? LIMIT 1";

        let result = self
            .session
            .query_unpaged(query, (license_id,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_single_analytics(result)
    }

    async fn get_analytics_by_date(&self, date: &str) -> Result<Vec<LicenseAnalyticsEntity>, String> {
        let query = "SELECT license_id, date, uptime, required_uptime, created_at, updated_at FROM license_analytics WHERE date = ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (date,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_analytics_list(result)
    }

    async fn delete_analytics_by_license(&self, license_id: &str) -> Result<usize, String> {
        // First get all dates for this license, then delete each
        let query = "SELECT date FROM license_analytics WHERE license_id = ?";

        let result = self
            .session
            .query_unpaged(query, (license_id,))
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        let rows = match rows_result.rows::<(String,)>() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        let mut count = 0;
        let delete_query = "DELETE FROM license_analytics WHERE license_id = ? AND date = ?";

        for row_result in rows {
            if let Ok((date,)) = row_result {
                match self.session.query_unpaged(delete_query, (license_id, &date)).await {
                    Ok(_) => count += 1,
                    Err(e) => error!("Failed to delete analytics for date {}: {}", date, e),
                }
            }
        }

        Ok(count)
    }
}
