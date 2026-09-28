//! Observability component types.

use std::collections::HashMap;

/// Log level for log entries.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum LogLevel {
    /// Trace level (most verbose).
    Trace,
    /// Debug level.
    Debug,
    /// Info level (default).
    #[default]
    Info,
    /// Warning level.
    Warn,
    /// Error level (most severe).
    Error,
}

impl LogLevel {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }

    /// Returns the display label.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Trace => "TRC",
            Self::Debug => "DBG",
            Self::Info => "INF",
            Self::Warn => "WRN",
            Self::Error => "ERR",
        }
    }

    /// Returns the CSS color.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Trace => "var(--fx-color-text-tertiary, #8c8c8c)",
            Self::Debug => "var(--fx-color-text-secondary, #a6a6a6)",
            Self::Info => "var(--fx-color-info, #1890ff)",
            Self::Warn => "var(--fx-color-warning, #faad14)",
            Self::Error => "var(--fx-color-error, #ff4d4f)",
        }
    }
}

/// Log entry for the log viewer.
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// Unique entry ID.
    pub id: String,
    /// Unix timestamp in milliseconds.
    pub timestamp: i64,
    /// Log level.
    pub level: LogLevel,
    /// Log message.
    pub message: String,
    /// Source service/component.
    pub source: String,
    /// Associated trace ID.
    pub trace_id: Option<String>,
    /// Additional metadata.
    pub metadata: HashMap<String, String>,
}

impl LogEntry {
    /// Create a new log entry.
    pub fn new(
        timestamp: i64,
        level: LogLevel,
        message: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("{}-{}", timestamp, rand_id()),
            timestamp,
            level,
            message: message.into(),
            source: source.into(),
            trace_id: None,
            metadata: HashMap::new(),
        }
    }

    /// Set the trace ID.
    pub fn trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// Add metadata.
    pub fn meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Log filter configuration.
#[derive(Debug, Clone, Default)]
pub struct LogFilter {
    /// Filter by log levels.
    pub levels: Vec<LogLevel>,
    /// Search text.
    pub search: String,
    /// Time range (start, end) in milliseconds.
    pub time_range: Option<(i64, i64)>,
    /// Filter by sources.
    pub sources: Vec<String>,
}

impl LogFilter {
    /// Create a new empty filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by levels.
    pub fn levels(mut self, levels: Vec<LogLevel>) -> Self {
        self.levels = levels;
        self
    }

    /// Filter by search text.
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = search.into();
        self
    }

    /// Filter by time range.
    pub fn time_range(mut self, start: i64, end: i64) -> Self {
        self.time_range = Some((start, end));
        self
    }

    /// Check if a log entry matches this filter.
    pub fn matches(&self, entry: &LogEntry) -> bool {
        // Check level filter
        if !self.levels.is_empty() && !self.levels.contains(&entry.level) {
            return false;
        }

        // Check search filter
        if !self.search.is_empty() {
            let search_lower = self.search.to_lowercase();
            let matches_message = entry.message.to_lowercase().contains(&search_lower);
            let matches_source = entry.source.to_lowercase().contains(&search_lower);
            if !matches_message && !matches_source {
                return false;
            }
        }

        // Check time range
        if let Some((start, end)) = self.time_range {
            if entry.timestamp < start || entry.timestamp > end {
                return false;
            }
        }

        // Check source filter
        if !self.sources.is_empty() && !self.sources.contains(&entry.source) {
            return false;
        }

        true
    }
}

/// Span status for trace visualization.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpanStatus {
    /// Successful span.
    #[default]
    Ok,
    /// Error span.
    Error,
    /// Timeout span.
    Timeout,
}

impl SpanStatus {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Error => "error",
            Self::Timeout => "timeout",
        }
    }

    /// Returns the CSS color.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Ok => "var(--fx-color-success, #52c41a)",
            Self::Error => "var(--fx-color-error, #ff4d4f)",
            Self::Timeout => "var(--fx-color-warning, #faad14)",
        }
    }
}

/// Span in a distributed trace.
#[derive(Debug, Clone)]
pub struct Span {
    /// Span ID.
    pub span_id: String,
    /// Parent span ID.
    pub parent_id: Option<String>,
    /// Operation name.
    pub operation: String,
    /// Service name.
    pub service: String,
    /// Start time in milliseconds.
    pub start_time: i64,
    /// Duration in milliseconds.
    pub duration_ms: u64,
    /// Span status.
    pub status: SpanStatus,
    /// Child spans.
    pub children: Vec<Span>,
    /// Span attributes.
    pub attributes: HashMap<String, String>,
}

impl Span {
    /// Create a new span.
    pub fn new(
        span_id: impl Into<String>,
        operation: impl Into<String>,
        service: impl Into<String>,
        start_time: i64,
        duration_ms: u64,
    ) -> Self {
        Self {
            span_id: span_id.into(),
            parent_id: None,
            operation: operation.into(),
            service: service.into(),
            start_time,
            duration_ms,
            status: SpanStatus::Ok,
            children: Vec::new(),
            attributes: HashMap::new(),
        }
    }

    /// Set parent ID.
    pub fn parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// Set status.
    pub fn status(mut self, status: SpanStatus) -> Self {
        self.status = status;
        self
    }

    /// Add child span.
    pub fn child(mut self, child: Span) -> Self {
        self.children.push(child);
        self
    }

    /// Add attribute.
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

/// Distributed trace.
#[derive(Debug, Clone)]
pub struct Trace {
    /// Trace ID.
    pub trace_id: String,
    /// Root span.
    pub root_span: Span,
    /// Total duration in milliseconds.
    pub duration_ms: u64,
    /// Number of services involved.
    pub service_count: usize,
}

impl Trace {
    /// Create a new trace.
    pub fn new(trace_id: impl Into<String>, root_span: Span) -> Self {
        let duration_ms = root_span.duration_ms;
        let service_count = count_services(&root_span);

        Self {
            trace_id: trace_id.into(),
            root_span,
            duration_ms,
            service_count,
        }
    }
}

/// Alert severity levels.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Informational alert.
    #[default]
    Info,
    /// Warning alert.
    Warning,
    /// Critical alert.
    Critical,
}

impl Severity {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Critical => "critical",
        }
    }

    /// Returns the display label.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Warning => "WARN",
            Self::Critical => "CRIT",
        }
    }

    /// Returns the CSS color.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Info => "var(--fx-color-info, #1890ff)",
            Self::Warning => "var(--fx-color-warning, #faad14)",
            Self::Critical => "var(--fx-color-error, #ff4d4f)",
        }
    }

    /// Returns the icon.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::Info => "ℹ",
            Self::Warning => "⚠",
            Self::Critical => "⛔",
        }
    }
}

/// Alert state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AlertState {
    /// Alert is pending (not yet fired).
    #[default]
    Pending,
    /// Alert is firing.
    Firing,
    /// Alert has been resolved.
    Resolved,
}

impl AlertState {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Firing => "firing",
            Self::Resolved => "resolved",
        }
    }

    /// Returns the display label.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Firing => "Firing",
            Self::Resolved => "Resolved",
        }
    }
}

/// Alert data.
#[derive(Debug, Clone)]
pub struct Alert {
    /// Alert ID.
    pub id: String,
    /// Alert name.
    pub name: String,
    /// Alert severity.
    pub severity: Severity,
    /// Alert state.
    pub state: AlertState,
    /// Alert message.
    pub message: String,
    /// Time when alert fired (milliseconds).
    pub fired_at: i64,
    /// Alert labels.
    pub labels: HashMap<String, String>,
}

impl Alert {
    /// Create a new alert.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        severity: Severity,
        message: impl Into<String>,
        fired_at: i64,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            severity,
            state: AlertState::Firing,
            message: message.into(),
            fired_at,
            labels: HashMap::new(),
        }
    }

    /// Set the state.
    pub fn state(mut self, state: AlertState) -> Self {
        self.state = state;
        self
    }

    /// Add a label.
    pub fn label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }
}

/// Type alias for alert ID.
pub type AlertId = String;

/// Type alias for span ID.
pub type SpanId = String;

/// Type alias for activity ID.
pub type ActivityId = String;

/// Activity category for grouping.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ActivityCategory {
    /// System-level activities (app start, shutdown, config).
    #[default]
    System,
    /// User activities (login, logout, settings).
    User,
    /// Plugin activities (install, activate, errors).
    Plugin,
    /// Network activities (peer connections).
    Network,
    /// Transaction activities.
    Transaction,
    /// Storage activities.
    Storage,
    /// Wallet activities.
    Wallet,
    /// Custom/other activities.
    Custom,
}

impl ActivityCategory {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Plugin => "plugin",
            Self::Network => "network",
            Self::Transaction => "transaction",
            Self::Storage => "storage",
            Self::Wallet => "wallet",
            Self::Custom => "custom",
        }
    }

    /// Returns the display label.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::System => "System",
            Self::User => "User",
            Self::Plugin => "Plugin",
            Self::Network => "Network",
            Self::Transaction => "Transaction",
            Self::Storage => "Storage",
            Self::Wallet => "Wallet",
            Self::Custom => "Custom",
        }
    }

    /// Returns the CSS color.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::System => "var(--fx-color-text-secondary, #a6a6a6)",
            Self::User => "var(--fx-color-info, #1890ff)",
            Self::Plugin => "var(--fx-color-geekblue, #2f54eb)",
            Self::Network => "var(--fx-color-cyan, #13c2c2)",
            Self::Transaction => "var(--fx-color-green, #52c41a)",
            Self::Storage => "var(--fx-color-orange, #fa8c16)",
            Self::Wallet => "var(--fx-color-gold, #faad14)",
            Self::Custom => "var(--fx-color-purple, #722ed1)",
        }
    }

    /// Returns the icon.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::System => "⚙",
            Self::User => "👤",
            Self::Plugin => "🔌",
            Self::Network => "🌐",
            Self::Transaction => "💸",
            Self::Storage => "💾",
            Self::Wallet => "💳",
            Self::Custom => "📋",
        }
    }
}

/// Device activity entry.
#[derive(Debug, Clone)]
pub struct Activity {
    /// Unique activity ID.
    pub id: String,
    /// Activity category.
    pub category: ActivityCategory,
    /// Activity name/type.
    pub name: String,
    /// Timestamp in milliseconds.
    pub timestamp: i64,
    /// Whether this is an error activity.
    pub is_error: bool,
    /// Description or message.
    pub description: String,
    /// Device ID.
    pub device_id: Option<String>,
    /// Session ID.
    pub session_id: Option<String>,
    /// User ID.
    pub user_id: Option<String>,
    /// Additional metadata.
    pub metadata: HashMap<String, String>,
}

impl Activity {
    /// Create a new activity.
    pub fn new(
        category: ActivityCategory,
        name: impl Into<String>,
        description: impl Into<String>,
        timestamp: i64,
    ) -> Self {
        Self {
            id: format!("{}-{}", timestamp, rand_id()),
            category,
            name: name.into(),
            description: description.into(),
            timestamp,
            is_error: false,
            device_id: None,
            session_id: None,
            user_id: None,
            metadata: HashMap::new(),
        }
    }

    /// Mark as error.
    pub fn error(mut self) -> Self {
        self.is_error = true;
        self
    }

    /// Set device ID.
    pub fn device(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    /// Set session ID.
    pub fn session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    /// Set user ID.
    pub fn user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Add metadata.
    pub fn meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Activity filter configuration.
#[derive(Debug, Clone, Default)]
pub struct ActivityFilter {
    /// Filter by categories.
    pub categories: Vec<ActivityCategory>,
    /// Search text.
    pub search: String,
    /// Time range (start, end) in milliseconds.
    pub time_range: Option<(i64, i64)>,
    /// Only show errors.
    pub errors_only: bool,
}

impl ActivityFilter {
    /// Create a new empty filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by categories.
    pub fn categories(mut self, categories: Vec<ActivityCategory>) -> Self {
        self.categories = categories;
        self
    }

    /// Filter by search text.
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = search.into();
        self
    }

    /// Filter by time range.
    pub fn time_range(mut self, start: i64, end: i64) -> Self {
        self.time_range = Some((start, end));
        self
    }

    /// Show only errors.
    pub fn errors_only(mut self) -> Self {
        self.errors_only = true;
        self
    }

    /// Check if an activity matches this filter.
    pub fn matches(&self, activity: &Activity) -> bool {
        // Check category filter
        if !self.categories.is_empty() && !self.categories.contains(&activity.category) {
            return false;
        }

        // Check search filter
        if !self.search.is_empty() {
            let search_lower = self.search.to_lowercase();
            let matches_name = activity.name.to_lowercase().contains(&search_lower);
            let matches_desc = activity.description.to_lowercase().contains(&search_lower);
            if !matches_name && !matches_desc {
                return false;
            }
        }

        // Check time range
        if let Some((start, end)) = self.time_range {
            if activity.timestamp < start || activity.timestamp > end {
                return false;
            }
        }

        // Check errors only
        if self.errors_only && !activity.is_error {
            return false;
        }

        true
    }
}

// Helper functions

fn rand_id() -> String {
    // Simple random ID generator for demo purposes
    format!("{:08x}", js_sys::Math::random() as u32 * u32::MAX)
}

fn count_services(span: &Span) -> usize {
    let mut services = std::collections::HashSet::new();
    collect_services(span, &mut services);
    services.len()
}

fn collect_services(span: &Span, services: &mut std::collections::HashSet<String>) {
    services.insert(span.service.clone());
    for child in &span.children {
        collect_services(child, services);
    }
}

// =============================================================================
// P0 Observability Component Types
// =============================================================================

/// Node role types for P2P network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeRole {
    /// Storage provider role.
    Storage,
    /// Relay/routing role.
    Relay,
    /// Compute provider role.
    Compute,
    /// DHT participant role.
    DHT,
    /// Validator role.
    Validator,
    /// Bootstrap node role.
    Bootstrap,
}

impl NodeRole {
    /// Returns the display label.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Storage => "Storage",
            Self::Relay => "Relay",
            Self::Compute => "Compute",
            Self::DHT => "DHT",
            Self::Validator => "Validator",
            Self::Bootstrap => "Bootstrap",
        }
    }

    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Storage => "storage",
            Self::Relay => "relay",
            Self::Compute => "compute",
            Self::DHT => "dht",
            Self::Validator => "validator",
            Self::Bootstrap => "bootstrap",
        }
    }

    /// Returns the CSS color.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Storage => "var(--fx-color-orange, #fa8c16)",
            Self::Relay => "var(--fx-color-cyan, #13c2c2)",
            Self::Compute => "var(--fx-color-purple, #722ed1)",
            Self::DHT => "var(--fx-color-geekblue, #2f54eb)",
            Self::Validator => "var(--fx-color-green, #52c41a)",
            Self::Bootstrap => "var(--fx-color-magenta, #eb2f96)",
        }
    }

    /// Returns the icon for this role.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::Storage => "💾",
            Self::Relay => "🔄",
            Self::Compute => "⚡",
            Self::DHT => "🌐",
            Self::Validator => "✓",
            Self::Bootstrap => "🚀",
        }
    }

    /// Returns all available roles.
    pub fn all() -> Vec<Self> {
        vec![
            Self::Storage,
            Self::Relay,
            Self::Compute,
            Self::DHT,
            Self::Validator,
            Self::Bootstrap,
        ]
    }
}

/// Network health status.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NetworkStatus {
    /// Network is fully operational.
    #[default]
    Healthy,
    /// Network has some issues but is operational.
    Degraded,
    /// Network has critical issues.
    Critical,
    /// Network is offline.
    Offline,
    /// Network is connecting/initializing.
    Connecting,
}

impl NetworkStatus {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Critical => "critical",
            Self::Offline => "offline",
            Self::Connecting => "connecting",
        }
    }

    /// Returns the display label.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Healthy => "Healthy",
            Self::Degraded => "Degraded",
            Self::Critical => "Critical",
            Self::Offline => "Offline",
            Self::Connecting => "Connecting...",
        }
    }

    /// Returns the CSS color.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Healthy => "var(--fx-color-success, #52c41a)",
            Self::Degraded => "var(--fx-color-warning, #faad14)",
            Self::Critical => "var(--fx-color-error, #ff4d4f)",
            Self::Offline => "var(--fx-color-text-tertiary, #8c8c8c)",
            Self::Connecting => "var(--fx-color-info, #1890ff)",
        }
    }

    /// Returns the icon for this status.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::Healthy => "✓",
            Self::Degraded => "⚠",
            Self::Critical => "⛔",
            Self::Offline => "○",
            Self::Connecting => "◉",
        }
    }
}

/// Node operational state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NodeState {
    /// Node is online and operational.
    #[default]
    Online,
    /// Node is offline.
    Offline,
    /// Node is syncing with the network.
    Syncing,
    /// Node is starting up.
    Starting,
    /// Node is shutting down.
    Stopping,
    /// Node has encountered an error.
    Error,
}

impl NodeState {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Offline => "offline",
            Self::Syncing => "syncing",
            Self::Starting => "starting",
            Self::Stopping => "stopping",
            Self::Error => "error",
        }
    }

    /// Returns the display label.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Online => "Online",
            Self::Offline => "Offline",
            Self::Syncing => "Syncing",
            Self::Starting => "Starting",
            Self::Stopping => "Stopping",
            Self::Error => "Error",
        }
    }

    /// Returns the CSS color.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Online => "var(--fx-color-success, #52c41a)",
            Self::Offline => "var(--fx-color-text-tertiary, #8c8c8c)",
            Self::Syncing => "var(--fx-color-info, #1890ff)",
            Self::Starting => "var(--fx-color-warning, #faad14)",
            Self::Stopping => "var(--fx-color-warning, #faad14)",
            Self::Error => "var(--fx-color-error, #ff4d4f)",
        }
    }

    /// Returns whether the node is considered active.
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Online | Self::Syncing)
    }
}

/// Uptime indicator size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum UptimeSize {
    /// Small size (48px).
    Small,
    /// Default size (64px).
    #[default]
    Default,
    /// Large size (96px).
    Large,
}

impl UptimeSize {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "md",
            Self::Large => "lg",
        }
    }

    /// Returns the size in pixels.
    pub fn as_pixels(&self) -> u32 {
        match self {
            Self::Small => 48,
            Self::Default => 64,
            Self::Large => 96,
        }
    }
}

/// Uptime color mode for threshold coloring.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum UptimeColorMode {
    /// Standard thresholds: >99% green, >95% yellow, <95% red.
    #[default]
    Standard,
    /// Use custom thresholds.
    Custom,
}

/// Reputation gauge size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReputationGaugeSize {
    /// Small size (120px).
    Small,
    /// Default size (180px).
    #[default]
    Default,
    /// Large size (240px).
    Large,
}

impl ReputationGaugeSize {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "md",
            Self::Large => "lg",
        }
    }

    /// Returns the size in pixels.
    pub fn as_pixels(&self) -> u32 {
        match self {
            Self::Small => 120,
            Self::Default => 180,
            Self::Large => 240,
        }
    }
}

/// Reputation score thresholds for color zones.
#[derive(Debug, Clone)]
pub struct ReputationThresholds {
    /// Below this value = red (critical).
    pub low: f64,
    /// Below this value = yellow (warning), above = green (good).
    pub medium: f64,
}

impl Default for ReputationThresholds {
    fn default() -> Self {
        Self {
            low: 40.0,
            medium: 70.0,
        }
    }
}

impl ReputationThresholds {
    /// Create new thresholds.
    pub fn new(low: f64, medium: f64) -> Self {
        Self { low, medium }
    }

    /// Returns the color for a given score.
    pub fn color_for_score(&self, score: f64) -> &'static str {
        if score < self.low {
            "var(--fx-color-error, #ff4d4f)"
        } else if score < self.medium {
            "var(--fx-color-warning, #faad14)"
        } else {
            "var(--fx-color-success, #52c41a)"
        }
    }

    /// Returns the zone suffix for a given score.
    pub fn zone_for_score(&self, score: f64) -> &'static str {
        if score < self.low {
            "low"
        } else if score < self.medium {
            "medium"
        } else {
            "high"
        }
    }
}

/// Earnings category for breakdown display.
#[derive(Debug, Clone)]
pub struct EarningsCategory {
    /// Category name.
    pub name: String,
    /// Earnings amount.
    pub amount: f64,
    /// Optional custom color.
    pub color: Option<String>,
    /// Optional icon (emoji or icon class).
    pub icon: Option<String>,
}

impl EarningsCategory {
    /// Create a new earnings category.
    pub fn new(name: impl Into<String>, amount: f64) -> Self {
        Self {
            name: name.into(),
            amount,
            color: None,
            icon: None,
        }
    }

    /// Set a custom color.
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set an icon (emoji or icon class).
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Create a Storage earnings category with default styling.
    pub fn storage(amount: f64) -> Self {
        Self::new("Storage", amount)
            .with_color("#fa8c16")
            .with_icon("💾")
    }

    /// Create a Relay earnings category with default styling.
    pub fn relay(amount: f64) -> Self {
        Self::new("Relay", amount)
            .with_color("#13c2c2")
            .with_icon("🔄")
    }

    /// Create a Compute earnings category with default styling.
    pub fn compute(amount: f64) -> Self {
        Self::new("Compute", amount)
            .with_color("#722ed1")
            .with_icon("⚡")
    }

    /// Create a DHT earnings category with default styling.
    pub fn dht(amount: f64) -> Self {
        Self::new("DHT", amount)
            .with_color("#2f54eb")
            .with_icon("🌐")
    }

    /// Create a Validator earnings category with default styling.
    pub fn validator(amount: f64) -> Self {
        Self::new("Validator", amount)
            .with_color("#52c41a")
            .with_icon("✓")
    }

    /// Create a Staking earnings category with default styling.
    pub fn staking(amount: f64) -> Self {
        Self::new("Staking", amount)
            .with_color("#eb2f96")
            .with_icon("🔒")
    }
}

/// Trend direction for indicators.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TrendDirection {
    /// Value is increasing.
    Up,
    /// Value is decreasing.
    Down,
    /// Value is stable.
    #[default]
    Flat,
}

impl TrendDirection {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Flat => "flat",
        }
    }

    /// Returns the arrow icon.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::Up => "↑",
            Self::Down => "↓",
            Self::Flat => "→",
        }
    }

    /// Returns the CSS color (standard mode: up=green, down=red).
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Up => "var(--fx-color-success, #52c41a)",
            Self::Down => "var(--fx-color-error, #ff4d4f)",
            Self::Flat => "var(--fx-color-text-secondary, #a6a6a6)",
        }
    }

    /// Calculate trend from current and previous values.
    pub fn from_values(current: f64, previous: f64) -> Self {
        if current > previous {
            Self::Up
        } else if current < previous {
            Self::Down
        } else {
            Self::Flat
        }
    }

    /// Calculate trend from current and previous integer values.
    pub fn from_counts(current: u32, previous: u32) -> Self {
        if current > previous {
            Self::Up
        } else if current < previous {
            Self::Down
        } else {
            Self::Flat
        }
    }
}
