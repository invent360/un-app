//! Prometheus metrics module for monitoring and alerting
//!
//! Phase 9: Release Validation monitoring infrastructure.
//! Exposes key operational metrics in Prometheus format.

use once_cell::sync::Lazy;
use prometheus::{
    Counter, CounterVec, Gauge, GaugeVec, Histogram, HistogramVec, Opts, Registry,
};

/// Global Prometheus registry
pub static REGISTRY: Lazy<Registry> = Lazy::new(|| {
    let registry = Registry::new();
    register_metrics(&registry);
    registry
});

// =============================================================================
// Claim and Allocation Metrics
// =============================================================================

/// Counter for allocation conflicts (concurrent claim race conditions)
pub static CLAIMS_CONFLICTS: Lazy<Counter> = Lazy::new(|| {
    Counter::with_opts(Opts::new(
        "uno_claims_conflicts_total",
        "Total number of allocation conflicts from concurrent claims",
    ))
    .expect("claims_conflicts metric")
});

/// Counter for successful claims
pub static CLAIMS_SUCCESS: Lazy<Counter> = Lazy::new(|| {
    Counter::with_opts(Opts::new(
        "uno_claims_success_total",
        "Total number of successful license claims",
    ))
    .expect("claims_success metric")
});

/// Counter for failed claims by reason
pub static CLAIMS_FAILED: Lazy<CounterVec> = Lazy::new(|| {
    CounterVec::new(
        Opts::new(
            "uno_claims_failed_total",
            "Total number of failed license claims by reason",
        ),
        &["reason"],
    )
    .expect("claims_failed metric")
});

// =============================================================================
// Finance Metrics
// =============================================================================

/// Gauge for count of allocations with unknown funding source
pub static FUNDING_UNKNOWN: Lazy<Gauge> = Lazy::new(|| {
    Gauge::with_opts(Opts::new(
        "uno_funding_unknown_count",
        "Number of allocations with unknown funding source",
    ))
    .expect("funding_unknown metric")
});

/// Gauge for total pool value across all licenses
pub static POOL_VALUE_TOTAL: Lazy<Gauge> = Lazy::new(|| {
    Gauge::with_opts(Opts::new(
        "uno_pool_value_total",
        "Total value in all license pools (USD cents)",
    ))
    .expect("pool_value_total metric")
});

// =============================================================================
// Sync Metrics
// =============================================================================

/// Gauge for sync staleness in seconds (per sync_id)
pub static SYNC_STALENESS_SECONDS: Lazy<GaugeVec> = Lazy::new(|| {
    GaugeVec::new(
        Opts::new(
            "uno_sync_staleness_seconds",
            "Seconds since last successful sync by sync_id",
        ),
        &["sync_id"],
    )
    .expect("sync_staleness metric")
});

/// Counter for sync errors by type
pub static SYNC_ERRORS: Lazy<CounterVec> = Lazy::new(|| {
    CounterVec::new(
        Opts::new(
            "uno_sync_errors_total",
            "Total sync errors by type",
        ),
        &["sync_id", "error_type"],
    )
    .expect("sync_errors metric")
});

// =============================================================================
// Job Queue Metrics
// =============================================================================

/// Gauge for job queue depth by status
pub static QUEUE_DEPTH: Lazy<GaugeVec> = Lazy::new(|| {
    GaugeVec::new(
        Opts::new(
            "uno_queue_depth",
            "Number of jobs in queue by status",
        ),
        &["status"],
    )
    .expect("queue_depth metric")
});

/// Gauge for age of oldest pending job in seconds
pub static QUEUE_AGE_SECONDS: Lazy<Gauge> = Lazy::new(|| {
    Gauge::with_opts(Opts::new(
        "uno_queue_oldest_age_seconds",
        "Age of oldest pending job in seconds",
    ))
    .expect("queue_age metric")
});

/// Counter for dead letter accumulation
pub static QUEUE_DEAD_LETTER: Lazy<Counter> = Lazy::new(|| {
    Counter::with_opts(Opts::new(
        "uno_queue_dead_letter_total",
        "Total number of jobs moved to dead letter queue",
    ))
    .expect("dead_letter metric")
});

// =============================================================================
// Webhook Metrics
// =============================================================================

/// Counter for webhook delivery errors by endpoint
pub static WEBHOOK_ERRORS: Lazy<CounterVec> = Lazy::new(|| {
    CounterVec::new(
        Opts::new(
            "uno_webhook_errors_total",
            "Total webhook delivery errors by endpoint",
        ),
        &["endpoint"],
    )
    .expect("webhook_errors metric")
});

/// Counter for successful webhook deliveries
pub static WEBHOOK_SUCCESS: Lazy<Counter> = Lazy::new(|| {
    Counter::with_opts(Opts::new(
        "uno_webhook_success_total",
        "Total successful webhook deliveries",
    ))
    .expect("webhook_success metric")
});

/// Histogram for webhook delivery latency
pub static WEBHOOK_LATENCY: Lazy<Histogram> = Lazy::new(|| {
    Histogram::with_opts(
        prometheus::HistogramOpts::new(
            "uno_webhook_latency_seconds",
            "Webhook delivery latency in seconds",
        )
        .buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 30.0]),
    )
    .expect("webhook_latency metric")
});

// =============================================================================
// System Metrics
// =============================================================================

/// Gauge for disk usage percentage
pub static DISK_USAGE_PERCENT: Lazy<Gauge> = Lazy::new(|| {
    Gauge::with_opts(Opts::new(
        "uno_disk_usage_percent",
        "Disk usage percentage for file storage",
    ))
    .expect("disk_usage metric")
});

/// Gauge for free inodes
pub static DISK_INODES_FREE: Lazy<Gauge> = Lazy::new(|| {
    Gauge::with_opts(Opts::new(
        "uno_disk_inodes_free",
        "Free inodes for file storage",
    ))
    .expect("disk_inodes metric")
});

/// Gauge for database connection pool size
pub static DB_POOL_SIZE: Lazy<GaugeVec> = Lazy::new(|| {
    GaugeVec::new(
        Opts::new(
            "uno_db_pool_connections",
            "Database connection pool status",
        ),
        &["state"],
    )
    .expect("db_pool metric")
});

// =============================================================================
// HTTP Metrics
// =============================================================================

/// Histogram for HTTP request duration by method and path
pub static HTTP_REQUEST_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    HistogramVec::new(
        prometheus::HistogramOpts::new(
            "uno_http_request_duration_seconds",
            "HTTP request duration in seconds",
        )
        .buckets(vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]),
        &["method", "path", "status"],
    )
    .expect("http_duration metric")
});

/// Counter for HTTP requests by method, path, and status
pub static HTTP_REQUESTS_TOTAL: Lazy<CounterVec> = Lazy::new(|| {
    CounterVec::new(
        Opts::new(
            "uno_http_requests_total",
            "Total HTTP requests by method, path, and status",
        ),
        &["method", "path", "status"],
    )
    .expect("http_requests metric")
});

// =============================================================================
// Pilot Metrics
// =============================================================================

/// Gauge for pilot participants by market and state
pub static PILOT_PARTICIPANTS: Lazy<GaugeVec> = Lazy::new(|| {
    GaugeVec::new(
        Opts::new(
            "uno_pilot_participants",
            "Pilot participants by market and state",
        ),
        &["market", "state"],
    )
    .expect("pilot_participants metric")
});

/// Counter for pilot milestone completions
pub static PILOT_MILESTONES: Lazy<CounterVec> = Lazy::new(|| {
    CounterVec::new(
        Opts::new(
            "uno_pilot_milestones_total",
            "Total pilot milestone completions by type",
        ),
        &["milestone"],
    )
    .expect("pilot_milestones metric")
});

// =============================================================================
// Registration
// =============================================================================

/// Register all metrics with the given registry
fn register_metrics(registry: &Registry) {
    // Claims
    registry.register(Box::new(CLAIMS_CONFLICTS.clone())).ok();
    registry.register(Box::new(CLAIMS_SUCCESS.clone())).ok();
    registry.register(Box::new(CLAIMS_FAILED.clone())).ok();

    // Finance
    registry.register(Box::new(FUNDING_UNKNOWN.clone())).ok();
    registry.register(Box::new(POOL_VALUE_TOTAL.clone())).ok();

    // Sync
    registry.register(Box::new(SYNC_STALENESS_SECONDS.clone())).ok();
    registry.register(Box::new(SYNC_ERRORS.clone())).ok();

    // Queue
    registry.register(Box::new(QUEUE_DEPTH.clone())).ok();
    registry.register(Box::new(QUEUE_AGE_SECONDS.clone())).ok();
    registry.register(Box::new(QUEUE_DEAD_LETTER.clone())).ok();

    // Webhooks
    registry.register(Box::new(WEBHOOK_ERRORS.clone())).ok();
    registry.register(Box::new(WEBHOOK_SUCCESS.clone())).ok();
    registry.register(Box::new(WEBHOOK_LATENCY.clone())).ok();

    // System
    registry.register(Box::new(DISK_USAGE_PERCENT.clone())).ok();
    registry.register(Box::new(DISK_INODES_FREE.clone())).ok();
    registry.register(Box::new(DB_POOL_SIZE.clone())).ok();

    // HTTP
    registry.register(Box::new(HTTP_REQUEST_DURATION.clone())).ok();
    registry.register(Box::new(HTTP_REQUESTS_TOTAL.clone())).ok();

    // Pilot
    registry.register(Box::new(PILOT_PARTICIPANTS.clone())).ok();
    registry.register(Box::new(PILOT_MILESTONES.clone())).ok();
}

/// Encode all metrics to Prometheus text format
pub fn encode_metrics() -> String {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_registration() {
        // Trigger lazy initialization
        let _ = &*REGISTRY;

        // Verify metrics can be incremented
        CLAIMS_SUCCESS.inc();
        assert!(CLAIMS_SUCCESS.get() >= 1.0);

        CLAIMS_CONFLICTS.inc();
        assert!(CLAIMS_CONFLICTS.get() >= 1.0);
    }

    #[test]
    fn test_encode_metrics() {
        // Initialize and encode
        let _ = &*REGISTRY;
        let output = encode_metrics();

        // Verify Prometheus format
        assert!(output.contains("# HELP"));
        assert!(output.contains("# TYPE"));
    }

    #[test]
    fn test_labeled_metrics() {
        CLAIMS_FAILED.with_label_values(&["unavailable"]).inc();
        QUEUE_DEPTH.with_label_values(&["pending"]).set(10.0);
        SYNC_STALENESS_SECONDS.with_label_values(&["licenses"]).set(30.0);

        let output = encode_metrics();
        assert!(output.contains("uno_claims_failed_total"));
        assert!(output.contains("uno_queue_depth"));
        assert!(output.contains("uno_sync_staleness_seconds"));
    }
}
