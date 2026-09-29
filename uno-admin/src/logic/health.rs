//! Health and readiness probes
//!
//! Implements Kubernetes-style health probes:
//! - Liveness: Is the process alive?
//! - Readiness: Can the process handle traffic?

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{error, info, warn};

#[cfg(feature = "ssr")]
use scylla::Session;

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Overall status: "healthy", "degraded", or "unhealthy"
    pub status: HealthStatus,
    /// Individual component checks
    pub checks: Vec<ComponentCheck>,
    /// Timestamp of the check
    pub timestamp: String,
    /// Total duration in milliseconds
    pub duration_ms: u64,
}

impl HealthCheck {
    /// Create a new health check result
    pub fn new(checks: Vec<ComponentCheck>, duration: Duration) -> Self {
        let status = Self::aggregate_status(&checks);
        Self {
            status,
            checks,
            timestamp: Utc::now().to_rfc3339(),
            duration_ms: duration.as_millis() as u64,
        }
    }

    /// Aggregate component statuses into overall status
    fn aggregate_status(checks: &[ComponentCheck]) -> HealthStatus {
        if checks.iter().any(|c| c.critical && matches!(c.status, HealthStatus::Unhealthy)) {
            HealthStatus::Unhealthy
        } else if checks.iter().any(|c| matches!(c.status, HealthStatus::Degraded)) {
            HealthStatus::Degraded
        } else if checks.iter().any(|c| matches!(c.status, HealthStatus::Unhealthy)) {
            // Non-critical unhealthy = degraded
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        }
    }

    /// Check if all components are healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.status, HealthStatus::Healthy)
    }

    /// Check if service is ready to handle traffic
    pub fn is_ready(&self) -> bool {
        !matches!(self.status, HealthStatus::Unhealthy)
    }
}

/// Health status levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some non-critical issues
    Degraded,
    /// Critical failure
    Unhealthy,
}

impl HealthStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unhealthy => "unhealthy",
        }
    }
}

/// Individual component health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentCheck {
    /// Component name
    pub name: String,
    /// Component status
    pub status: HealthStatus,
    /// Optional message
    pub message: Option<String>,
    /// Check duration in milliseconds
    pub duration_ms: u64,
    /// Whether this component is critical for readiness
    pub critical: bool,
}

impl ComponentCheck {
    pub fn healthy(name: impl Into<String>, duration: Duration) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Healthy,
            message: None,
            duration_ms: duration.as_millis() as u64,
            critical: true,
        }
    }

    pub fn unhealthy(name: impl Into<String>, message: impl Into<String>, duration: Duration) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Unhealthy,
            message: Some(message.into()),
            duration_ms: duration.as_millis() as u64,
            critical: true,
        }
    }

    pub fn degraded(name: impl Into<String>, message: impl Into<String>, duration: Duration) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Degraded,
            message: Some(message.into()),
            duration_ms: duration.as_millis() as u64,
            critical: false,
        }
    }

    /// Mark this check as non-critical
    pub fn non_critical(mut self) -> Self {
        self.critical = false;
        self
    }
}

/// Health probe configuration
#[derive(Debug, Clone)]
pub struct HealthConfig {
    /// Timeout for database queries
    pub db_timeout: Duration,
    /// Timeout for external service checks
    pub external_timeout: Duration,
    /// How often to update cached health status
    pub cache_duration: Duration,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            db_timeout: Duration::from_secs(5),
            external_timeout: Duration::from_secs(10),
            cache_duration: Duration::from_secs(10),
        }
    }
}

/// Health probe service
#[cfg(feature = "ssr")]
pub struct HealthService {
    session: Option<Arc<Session>>,
    keyspace: Option<String>,
    config: HealthConfig,
}

#[cfg(feature = "ssr")]
impl HealthService {
    /// Create a new health service
    pub fn new(session: Option<Arc<Session>>, keyspace: Option<String>) -> Self {
        Self {
            session,
            keyspace,
            config: HealthConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(session: Option<Arc<Session>>, keyspace: Option<String>, config: HealthConfig) -> Self {
        Self { session, keyspace, config }
    }

    /// Liveness probe - just checks if process is alive
    ///
    /// Should be fast and not depend on external services.
    pub async fn liveness(&self) -> HealthCheck {
        let start = Instant::now();
        let checks = vec![ComponentCheck::healthy("process", start.elapsed())];
        HealthCheck::new(checks, start.elapsed())
    }

    /// Readiness probe - checks if service can handle traffic
    ///
    /// Includes database connectivity check with timeout.
    pub async fn readiness(&self) -> HealthCheck {
        let start = Instant::now();
        let mut checks = Vec::new();

        // Database check
        let db_check = self.check_database().await;
        checks.push(db_check);

        // Memory check (basic)
        let mem_check = self.check_memory();
        checks.push(mem_check);

        HealthCheck::new(checks, start.elapsed())
    }

    /// Full health check including optional external services
    pub async fn full(&self) -> HealthCheck {
        let start = Instant::now();
        let mut checks = Vec::new();

        // Core checks
        checks.push(self.check_database().await);
        checks.push(self.check_memory());

        HealthCheck::new(checks, start.elapsed())
    }

    /// Check database connectivity with bounded timeout
    async fn check_database(&self) -> ComponentCheck {
        let start = Instant::now();

        let Some(session) = &self.session else {
            return ComponentCheck::unhealthy(
                "database",
                "Database session not configured",
                start.elapsed(),
            );
        };

        let Some(keyspace) = &self.keyspace else {
            return ComponentCheck::unhealthy(
                "database",
                "Database keyspace not configured",
                start.elapsed(),
            );
        };

        // Use tokio timeout to bound the query
        let result = tokio::time::timeout(
            self.config.db_timeout,
            session.query_unpaged(
                format!("SELECT keyspace_name FROM system_schema.keyspaces WHERE keyspace_name = '{}'", keyspace),
                &[],
            ),
        )
        .await;

        match result {
            Ok(Ok(_)) => ComponentCheck::healthy("database", start.elapsed()),
            Ok(Err(e)) => ComponentCheck::unhealthy(
                "database",
                format!("Query failed: {}", e),
                start.elapsed(),
            ),
            Err(_) => ComponentCheck::unhealthy(
                "database",
                format!("Query timed out after {:?}", self.config.db_timeout),
                start.elapsed(),
            ),
        }
    }

    /// Basic memory check
    fn check_memory(&self) -> ComponentCheck {
        let start = Instant::now();

        // Simple check - if we can allocate, we're probably fine
        let _test_alloc: Vec<u8> = Vec::with_capacity(1024);

        ComponentCheck::healthy("memory", start.elapsed()).non_critical()
    }
}

/// Startup checks - run once at application start
#[cfg(feature = "ssr")]
pub struct StartupChecks {
    session: Option<Arc<Session>>,
    keyspace: Option<String>,
}

#[cfg(feature = "ssr")]
impl StartupChecks {
    pub fn new(session: Option<Arc<Session>>, keyspace: Option<String>) -> Self {
        Self { session, keyspace }
    }

    /// Run all startup checks
    ///
    /// Returns Err if any critical check fails.
    pub async fn run(&self) -> Result<(), String> {
        info!("Running startup checks...");

        // Check required configuration
        self.check_required_config()?;

        // Check database connectivity
        self.check_database_connection().await?;

        info!("Startup checks passed");
        Ok(())
    }

    /// Check required environment variables
    fn check_required_config(&self) -> Result<(), String> {
        let required = [
            "SCYLLA_URI",
            "SCYLLA_KEYSPACE",
        ];

        let mut missing = Vec::new();
        for var in required {
            if std::env::var(var).is_err() {
                missing.push(var);
            }
        }

        if !missing.is_empty() {
            // Warn but don't fail - config might come from other sources
            warn!("Environment variables not set: {:?}", missing);
        }

        Ok(())
    }

    /// Check database is reachable
    async fn check_database_connection(&self) -> Result<(), String> {
        let Some(session) = &self.session else {
            return Err("Database session not configured".to_string());
        };

        let Some(keyspace) = &self.keyspace else {
            return Err("Database keyspace not configured".to_string());
        };

        // Try a simple query with timeout
        let result = tokio::time::timeout(
            Duration::from_secs(10),
            session.query_unpaged(
                format!("SELECT keyspace_name FROM system_schema.keyspaces WHERE keyspace_name = '{}'", keyspace),
                &[],
            ),
        )
        .await;

        match result {
            Ok(Ok(_)) => {
                info!("Database connection verified");
                Ok(())
            }
            Ok(Err(e)) => Err(format!("Database query failed: {}", e)),
            Err(_) => Err("Database connection timed out".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_aggregation() {
        // All healthy
        let checks = vec![
            ComponentCheck::healthy("a", Duration::from_millis(1)),
            ComponentCheck::healthy("b", Duration::from_millis(1)),
        ];
        let health = HealthCheck::new(checks, Duration::from_millis(2));
        assert!(matches!(health.status, HealthStatus::Healthy));

        // One degraded
        let checks = vec![
            ComponentCheck::healthy("a", Duration::from_millis(1)),
            ComponentCheck::degraded("b", "issue", Duration::from_millis(1)),
        ];
        let health = HealthCheck::new(checks, Duration::from_millis(2));
        assert!(matches!(health.status, HealthStatus::Degraded));

        // One critical unhealthy
        let checks = vec![
            ComponentCheck::healthy("a", Duration::from_millis(1)),
            ComponentCheck::unhealthy("b", "error", Duration::from_millis(1)),
        ];
        let health = HealthCheck::new(checks, Duration::from_millis(2));
        assert!(matches!(health.status, HealthStatus::Unhealthy));
    }

    #[test]
    fn test_non_critical_unhealthy() {
        // Non-critical unhealthy = degraded overall
        let checks = vec![
            ComponentCheck::healthy("a", Duration::from_millis(1)),
            ComponentCheck::unhealthy("b", "error", Duration::from_millis(1)).non_critical(),
        ];
        let health = HealthCheck::new(checks, Duration::from_millis(2));
        assert!(matches!(health.status, HealthStatus::Degraded));
    }

    #[test]
    fn test_ready_status() {
        let healthy = HealthCheck::new(
            vec![ComponentCheck::healthy("a", Duration::ZERO)],
            Duration::ZERO,
        );
        assert!(healthy.is_ready());
        assert!(healthy.is_healthy());

        let degraded = HealthCheck::new(
            vec![ComponentCheck::degraded("a", "warn", Duration::ZERO)],
            Duration::ZERO,
        );
        assert!(degraded.is_ready()); // Degraded is still ready
        assert!(!degraded.is_healthy());

        let unhealthy = HealthCheck::new(
            vec![ComponentCheck::unhealthy("a", "error", Duration::ZERO)],
            Duration::ZERO,
        );
        assert!(!unhealthy.is_ready());
        assert!(!unhealthy.is_healthy());
    }
}
