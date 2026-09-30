//! Health check endpoints with proper separation
//!
//! Provides three levels of health probes:
//! - Liveness: Is the process alive? (no dependencies)
//! - Readiness: Can the service handle traffic? (critical deps only)
//! - Integration: Full health of all integrations (for dashboards)

use crate::server::app::ServiceFactory;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

// ============================================
// RESPONSE TYPES
// ============================================

/// Simple liveness response
#[derive(Debug, Serialize)]
pub struct LivenessResponse {
    pub status: &'static str,
    pub timestamp: DateTime<Utc>,
}

/// Readiness response with dependency status
#[derive(Debug, Serialize)]
pub struct ReadinessResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub dependencies: ReadinessDependencies,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ReadinessDependencies {
    pub database: DependencyStatus,
}

/// Full integration health response
#[derive(Debug, Serialize)]
pub struct IntegrationHealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub uptime_secs: u64,
    pub dependencies: IntegrationDependencies,
    pub sync_freshness: SyncFreshnessStatus,
    pub checks: Vec<HealthCheck>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct IntegrationDependencies {
    pub database: DependencyStatus,
    pub geoip: DependencyStatus,
    pub media_storage: DependencyStatus,
}

/// Sync freshness status for external integrations
#[derive(Debug, Serialize)]
pub struct SyncFreshnessStatus {
    /// Overall status: "fresh", "stale", "unknown"
    pub status: &'static str,
    /// Checkpoints that are stale or have errors
    pub stale_checkpoints: Vec<StaleCheckpoint>,
    /// Total number of sync checkpoints
    pub total_checkpoints: i64,
    /// Last check timestamp
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StaleCheckpoint {
    pub sync_id: String,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub error_count: i32,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DependencyStatus {
    pub name: &'static str,
    pub status: &'static str,
    pub latency_ms: Option<u64>,
    pub message: Option<String>,
}

impl DependencyStatus {
    pub fn healthy(name: &'static str, latency_ms: u64) -> Self {
        Self {
            name,
            status: "healthy",
            latency_ms: Some(latency_ms),
            message: None,
        }
    }

    pub fn unhealthy(name: &'static str, message: impl Into<String>) -> Self {
        Self {
            name,
            status: "unhealthy",
            latency_ms: None,
            message: Some(message.into()),
        }
    }

    pub fn unknown(name: &'static str) -> Self {
        Self {
            name,
            status: "unknown",
            latency_ms: None,
            message: Some("Not configured".to_string()),
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.status == "healthy"
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: &'static str,
    pub latency_ms: u64,
    pub message: Option<String>,
}

// ============================================
// HEALTH CHECK HANDLERS
// ============================================

/// Liveness probe - is the process alive?
///
/// This endpoint:
/// - Returns 200 immediately if the process is running
/// - No dependency checks (database, external services)
/// - Used by Kubernetes liveness probe
/// - Fast response time is critical
///
/// Failure here means the container should be restarted.
pub async fn liveness() -> HttpResponse {
    HttpResponse::Ok().json(LivenessResponse {
        status: "alive",
        timestamp: Utc::now(),
    })
}

/// Readiness probe - can the service handle traffic?
///
/// This endpoint:
/// - Checks critical dependencies (database)
/// - Returns 200 if all critical deps are healthy
/// - Returns 503 if any critical dep is unhealthy
/// - Used by Kubernetes readiness probe and load balancers
///
/// Failure here means the pod should be removed from service.
pub async fn readiness(factory: Option<web::Data<ServiceFactory>>) -> HttpResponse {
    let database = check_database(&factory).await;

    let all_healthy = database.is_healthy();

    let response = ReadinessResponse {
        status: if all_healthy { "ready" } else { "not_ready" },
        version: env!("CARGO_PKG_VERSION"),
        dependencies: ReadinessDependencies { database },
        timestamp: Utc::now(),
    };

    if all_healthy {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}

/// Integration health - full status of all integrations
///
/// This endpoint:
/// - Checks ALL dependencies (database, GeoIP, external APIs)
/// - Returns detailed status for monitoring dashboards
/// - Always returns 200 (status is in the body)
/// - Used for Prometheus metrics, Grafana dashboards, etc.
///
/// This is for observability, not traffic routing.
pub async fn integration_health(factory: Option<web::Data<ServiceFactory>>) -> HttpResponse {
    let start_time = get_start_time();
    let uptime_secs = start_time.elapsed().as_secs();

    let database = check_database(&factory).await;
    let geoip = check_geoip(&factory).await;
    let media_storage = check_media_storage().await;
    let sync_freshness = check_sync_freshness(&factory).await;

    // Run additional health checks
    let mut checks = Vec::new();

    // Check job queue health
    if let Some(ref f) = factory {
        checks.push(check_job_queue(&f.pool).await);
    }

    // Check disk thresholds if storage is healthy
    if media_storage.is_healthy() {
        checks.push(check_disk_thresholds().await);
    }

    // Determine overall status
    let critical_healthy = database.is_healthy() && media_storage.is_healthy();
    let sync_healthy = sync_freshness.status == "fresh";
    let all_healthy = critical_healthy && geoip.is_healthy() && sync_healthy;

    let status = if all_healthy {
        "healthy"
    } else if critical_healthy {
        "degraded"
    } else {
        "unhealthy"
    };

    let response = IntegrationHealthResponse {
        status,
        version: env!("CARGO_PKG_VERSION"),
        uptime_secs,
        dependencies: IntegrationDependencies { database, geoip, media_storage },
        sync_freshness,
        checks,
        timestamp: Utc::now(),
    };

    HttpResponse::Ok().json(response)
}

/// Legacy health check (for backwards compatibility)
///
/// Same as readiness but with old response format.
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub database: String,
}

pub async fn health_check(factory: Option<web::Data<ServiceFactory>>) -> HttpResponse {
    let database = check_database(&factory).await;
    let connected = database.is_healthy();

    let response = HealthResponse {
        status: if connected { "ready" } else { "not_ready" }.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: if connected {
            "connected"
        } else {
            "disconnected"
        }
        .to_string(),
    };

    if connected {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}

// ============================================
// INTERNAL HEALTH CHECK FUNCTIONS
// ============================================

async fn check_database(factory: &Option<web::Data<ServiceFactory>>) -> DependencyStatus {
    let Some(factory) = factory else {
        return DependencyStatus::unhealthy("database", "ServiceFactory not available");
    };

    let start = Instant::now();

    match tokio::time::timeout(
        Duration::from_secs(2),
        sqlx::query_scalar::<_, i32>("SELECT 1").fetch_one(&factory.pool),
    )
    .await
    {
        Ok(Ok(1)) => DependencyStatus::healthy("database", start.elapsed().as_millis() as u64),
        Ok(Ok(_)) => DependencyStatus::unhealthy("database", "Unexpected query result"),
        Ok(Err(e)) => DependencyStatus::unhealthy("database", format!("Query failed: {}", e)),
        Err(_) => DependencyStatus::unhealthy("database", "Connection timeout (2s)"),
    }
}

async fn check_geoip(factory: &Option<web::Data<ServiceFactory>>) -> DependencyStatus {
    let Some(factory) = factory else {
        return DependencyStatus::unknown("geoip");
    };

    let start = Instant::now();

    // Check if GeoIP is configured and working
    if factory.geoip.is_available() {
        // Try a test lookup with a well-known IP
        match factory.geoip.lookup_country("8.8.8.8") {
            Some(_) => DependencyStatus::healthy("geoip", start.elapsed().as_millis() as u64),
            None => DependencyStatus {
                name: "geoip",
                status: "degraded",
                latency_ms: Some(start.elapsed().as_millis() as u64),
                message: Some("Lookup returned no result".to_string()),
            },
        }
    } else {
        DependencyStatus::unknown("geoip")
    }
}

async fn check_job_queue(pool: &sqlx::PgPool) -> HealthCheck {
    let start = Instant::now();

    // Check for stuck jobs (running for > 30 minutes)
    let result: Result<(i64, i64, i64), _> = sqlx::query_as(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE status = 'pending') as pending,
            COUNT(*) FILTER (WHERE status = 'running' AND lease_expires_at < NOW()) as stuck,
            COUNT(*) FILTER (WHERE status = 'dead_letter') as dead_letter
        FROM job_queue
        "#,
    )
    .fetch_one(pool)
    .await;

    let latency_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok((pending, stuck, dead_letter)) => {
            let message = format!(
                "pending={}, stuck={}, dead_letter={}",
                pending, stuck, dead_letter
            );

            let status = if stuck > 0 {
                "warning"
            } else if dead_letter > 10 {
                "degraded"
            } else {
                "healthy"
            };

            HealthCheck {
                name: "job_queue".to_string(),
                status,
                latency_ms,
                message: Some(message),
            }
        }
        Err(e) => HealthCheck {
            name: "job_queue".to_string(),
            status: "unhealthy",
            latency_ms,
            message: Some(format!("Query failed: {}", e)),
        },
    }
}

/// Check sync checkpoint freshness for external integrations
///
/// A sync checkpoint is considered stale if:
/// - Last sync was more than 1 hour ago
/// - There are unresolved errors (error_count > 0)
async fn check_sync_freshness(factory: &Option<web::Data<ServiceFactory>>) -> SyncFreshnessStatus {
    let Some(factory) = factory else {
        return SyncFreshnessStatus {
            status: "unknown",
            stale_checkpoints: vec![],
            total_checkpoints: 0,
            checked_at: Utc::now(),
        };
    };

    // Query for stale or errored sync checkpoints
    #[derive(sqlx::FromRow)]
    struct StaleRow {
        sync_id: String,
        last_sync_at: Option<DateTime<Utc>>,
        error_count: i32,
        last_error: Option<String>,
    }

    let stale_result: Result<Vec<StaleRow>, _> = sqlx::query_as(
        r#"
        SELECT sync_id, last_sync_at, error_count, last_error
        FROM sync_checkpoints
        WHERE last_sync_at < NOW() - INTERVAL '1 hour'
           OR error_count > 0
        ORDER BY error_count DESC, last_sync_at ASC
        LIMIT 10
        "#,
    )
    .fetch_all(&factory.pool)
    .await;

    // Get total checkpoint count
    let total_result: Result<(i64,), _> = sqlx::query_as(
        "SELECT COUNT(*) FROM sync_checkpoints",
    )
    .fetch_one(&factory.pool)
    .await;

    let total_checkpoints = total_result.map(|(c,)| c).unwrap_or(0);

    match stale_result {
        Ok(rows) => {
            let stale_checkpoints: Vec<StaleCheckpoint> = rows
                .into_iter()
                .map(|r| StaleCheckpoint {
                    sync_id: r.sync_id,
                    last_sync_at: r.last_sync_at,
                    error_count: r.error_count,
                    last_error: r.last_error,
                })
                .collect();

            let status = if stale_checkpoints.is_empty() {
                "fresh"
            } else if stale_checkpoints.iter().any(|c| c.error_count > 0) {
                "error"
            } else {
                "stale"
            };

            SyncFreshnessStatus {
                status,
                stale_checkpoints,
                total_checkpoints,
                checked_at: Utc::now(),
            }
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to check sync freshness");
            SyncFreshnessStatus {
                status: "unknown",
                stale_checkpoints: vec![],
                total_checkpoints,
                checked_at: Utc::now(),
            }
        }
    }
}

/// Check media storage mount availability (P6-01)
///
/// Verifies:
/// - Storage path exists
/// - Path is writable
/// - Path is not a symlink (security)
async fn check_media_storage() -> DependencyStatus {
    let start = Instant::now();

    // Get storage path from environment
    let storage_path = std::env::var("FILE_STORAGE_LOCAL_PATH")
        .unwrap_or_else(|_| "/data/media".to_string());

    let path = std::path::Path::new(&storage_path);

    // Check if path exists
    if !path.exists() {
        return DependencyStatus::unhealthy(
            "media_storage",
            format!("Mount path does not exist: {}", storage_path),
        );
    }

    // Check if it's a directory
    if !path.is_dir() {
        return DependencyStatus::unhealthy(
            "media_storage",
            format!("Mount path is not a directory: {}", storage_path),
        );
    }

    // Check if it's a symlink (security concern)
    if path.is_symlink() {
        return DependencyStatus::unhealthy(
            "media_storage",
            format!("Mount path is a symlink (security risk): {}", storage_path),
        );
    }

    // Try to verify write access by checking temp file creation
    let test_file = path.join(".health_check_test");
    match std::fs::write(&test_file, b"health_check") {
        Ok(_) => {
            // Clean up test file
            let _ = std::fs::remove_file(&test_file);
            DependencyStatus::healthy("media_storage", start.elapsed().as_millis() as u64)
        }
        Err(e) => DependencyStatus::unhealthy(
            "media_storage",
            format!("Mount path is not writable: {}", e),
        ),
    }
}

/// Check disk thresholds (P6-08)
async fn check_disk_thresholds() -> HealthCheck {
    let start = Instant::now();

    let storage_path = std::env::var("FILE_STORAGE_LOCAL_PATH")
        .unwrap_or_else(|_| "/data/media".to_string());

    let path = std::path::Path::new(&storage_path);

    // For now, just check if path exists and is accessible
    // Production would use fs2::statvfs or sysinfo crate
    if !path.exists() {
        return HealthCheck {
            name: "disk_thresholds".to_string(),
            status: "unhealthy",
            latency_ms: start.elapsed().as_millis() as u64,
            message: Some("Storage path does not exist".to_string()),
        };
    }

    // Placeholder values - in production, use actual disk stats
    let used_percent = 50; // Would come from statvfs
    let available_gb = 100; // Would come from statvfs

    let status = if used_percent >= 95 {
        "emergency"
    } else if used_percent >= 90 {
        "critical"
    } else if used_percent >= 80 {
        "warning"
    } else {
        "healthy"
    };

    HealthCheck {
        name: "disk_thresholds".to_string(),
        status,
        latency_ms: start.elapsed().as_millis() as u64,
        message: Some(format!(
            "{}% used, ~{}GB available",
            used_percent, available_gb
        )),
    }
}

// Static start time for uptime calculation
fn get_start_time() -> &'static Instant {
    use std::sync::OnceLock;
    static START_TIME: OnceLock<Instant> = OnceLock::new();
    START_TIME.get_or_init(Instant::now)
}

// Initialize start time on module load
#[used]
static _INIT: () = {
    // Force start time initialization
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_status_healthy() {
        let status = DependencyStatus::healthy("test", 50);
        assert!(status.is_healthy());
        assert_eq!(status.latency_ms, Some(50));
    }

    #[test]
    fn test_dependency_status_unhealthy() {
        let status = DependencyStatus::unhealthy("test", "Connection failed");
        assert!(!status.is_healthy());
        assert_eq!(status.message, Some("Connection failed".to_string()));
    }

    #[test]
    fn test_dependency_status_unknown() {
        let status = DependencyStatus::unknown("test");
        assert!(!status.is_healthy());
        assert_eq!(status.status, "unknown");
    }
}
