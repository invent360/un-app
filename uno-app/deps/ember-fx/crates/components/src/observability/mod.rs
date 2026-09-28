//! Observability components for ember-fx.
//!
//! This module provides observability visualization components:
//!
//! ## Core Components
//! - LogViewer: Streaming log viewer with filtering
//! - TraceViewer: Waterfall trace visualization
//! - AlertCard: Alert status display
//! - AlertList: List of alerts
//! - MetricSelector: PromQL-aware metric picker
//! - NetworkTopologyMap: Network topology visualization
//! - ActivityViewer: Device activity log viewer
//!
//! ## P0 Components (Phase 0)
//! - UptimeIndicator: Circular uptime percentage ring
//! - ActiveRoleBadges: Node role chips
//! - NetworkHealthBanner: Network status header
//! - ConnectionCountIndicator: Peer count with trend
//! - EpochCountdownTimer: Epoch countdown display
//! - NodeStatusCard: StatCard with status dot
//! - EarningsSummaryCard: Token earnings card
//! - ReputationGauge: 0-100 reputation gauge
//!
//! ## P1 Components (Phase 1)
//! - ResourceUsageGauge: Dial/speedometer style gauge (CPU, RAM, Storage, Swap presets)
//! - BandwidthMeter: Upload/download progress bars
//! - ComputeUtilizationBar: CPU utilization with task counts
//! - ProofSuccessRateCard: Success rate ring indicator
//! - SlashingRiskGauge: Risk level gauge
//! - FederationStatusPanel: Multi-chain connectivity
//! - RoutingTableStatus: DHT routing table health
//! - StakeManagerPanel: Staking management interface
//! - NetworkTopologyGraph: Radial network visualization
//! - RelayTrafficGauge: Relay traffic utilization
//! - ReliabilityFactorGauge: Reliability factor display
//!
//! # Example
//!
//! ```ignore
//! use ember_fx_components::observability::{LogViewer, LogEntry, LogLevel};
//!
//! let logs = vec![
//!     LogEntry::new(1716000000000, LogLevel::Info, "Connected", "p2p"),
//! ];
//!
//! view! {
//!     <LogViewer logs=Signal::derive(move || logs.clone()) />
//! }
//! ```

mod types;
mod log_viewer;
mod trace_viewer;
mod alert_card;
mod metric_selector;
mod topology_types;
mod network_topology;
mod activity_viewer;

// P0 Components
mod uptime_indicator;
mod active_role_badges;
mod network_health_banner;
mod connection_count;
mod epoch_countdown;
mod node_status_card;
mod earnings_summary_card;
mod reputation_gauge;

// P1 Components
mod resource_usage_gauge;
mod bandwidth_meter;
mod compute_utilization;
mod proof_success_rate_card;
mod slashing_risk_gauge;
mod federation_status_panel;
mod routing_table_status;
mod stake_manager_panel;
mod network_topology_graph;
mod relay_traffic_gauge;
mod reliability_factor_gauge;

// P2 Components (Group 03)
mod sparkline_stat_card;
mod slo_heatmap_panel;
mod metrics_time_series_panel;
mod stacked_status_chart;
mod latency_step_chart;
mod geo_activity_map;

// P2 Components (Group 04)
mod cluster_gauge_panel;
mod compact_time_series_panel;
mod dual_axis_time_series;
mod memory_pool_area_chart;
mod dashboard_filter_bar;

// P2 Components (Group 00/6)
mod concentric_progress_rings;
mod action_item_card;

// P2 Components (Group 00/7)
mod resource_timings_chart;
mod gradient_score_gauge;
mod discrete_status_timeline;
mod health_score_donut;

// P2 Components (Group 00/3)
mod honeycomb_health_grid;
mod job_progress_panel;

// P2 Components (Group 00/0)
mod system_health_card;

// P2 Components (Group 00/4)
mod multi_series_line_chart;

// P2 Components (Group 00/5)
mod battery_gauge;

// P2 Components (Group 00/1)
mod grade_stat_card;

// Core exports
pub use types::*;
pub use log_viewer::LogViewer;
pub use trace_viewer::TraceViewer;
pub use alert_card::{AlertCard, AlertList};
pub use metric_selector::{MetricSelector, MetricInfo, MetricType};
pub use topology_types::*;
pub use network_topology::NetworkTopologyMap;
pub use activity_viewer::ActivityViewer;

// P0 Component exports
pub use uptime_indicator::UptimeIndicator;
pub use active_role_badges::ActiveRoleBadges;
pub use network_health_banner::NetworkHealthBanner;
pub use connection_count::ConnectionCountIndicator;
pub use epoch_countdown::EpochCountdownTimer;
pub use node_status_card::NodeStatusCard;
pub use earnings_summary_card::EarningsSummaryCard;
pub use reputation_gauge::ReputationGauge;

// P1 Component exports
pub use resource_usage_gauge::{
    ResourceUsageGauge, ResourceType, ResourceGaugeSize, UsageThresholds,
    CpuUsageGauge, RamUsageGauge, StorageUsageGauge, SwapUsageGauge,
};
pub use bandwidth_meter::BandwidthMeter;
pub use compute_utilization::ComputeUtilizationBar;
pub use proof_success_rate_card::ProofSuccessRateCard;
pub use slashing_risk_gauge::{SlashingRiskGauge, RiskLevel};
pub use federation_status_panel::{FederationStatusPanel, FederatedChain, ChainStatus};
pub use routing_table_status::{RoutingTableStatus, RoutingBucket};
pub use stake_manager_panel::{StakeManagerPanel, StakeAction, ValidatorInfo, StakeActionPayload};
pub use network_topology_graph::{NetworkTopologyGraph, GraphNode, GraphNodeType};
pub use relay_traffic_gauge::{
    RelayTrafficGauge, TrafficDirection, TrafficGaugeSize, TrafficLevel,
};
pub use reliability_factor_gauge::{
    ReliabilityFactorGauge, ReliabilityRating, ReliabilityGaugeSize, ReliabilityTrend,
};

// P2 Component exports (Group 03)
pub use sparkline_stat_card::{SparklineStatCard, SparklineColor, TrendDirection};
pub use slo_heatmap_panel::{SloHeatmapPanel, SloColumn, SloStatus};
pub use metrics_time_series_panel::{
    MetricsTimeSeriesPanel, MetricsSeries, MetricsDataPoint, MetricsColor, MetricsChartConfig,
};
pub use stacked_status_chart::{StackedStatusChart, StatusBand, StatusDataPoint, StatusLevel};
pub use latency_step_chart::{LatencyStepChart, LatencyPoint, LatencyChartConfig};
pub use geo_activity_map::{GeoActivityMap, ActivityMarker, GeoActivityMapConfig, locations};

// P2 Component exports (Group 04)
pub use cluster_gauge_panel::{ClusterGaugePanel, ClusterGaugeConfig};
pub use compact_time_series_panel::{
    CompactTimeSeriesPanel, CompactSeries, CompactDataPoint, CompactSeriesColor, CompactTimeSeriesConfig,
};
pub use dual_axis_time_series::{
    DualAxisTimeSeriesChart, DualAxisSeries, DualAxisDataPoint, ChartAxis, DualAxisColor,
    DualAxisLineStyle, ThresholdLine, DualAxisConfig,
};
pub use memory_pool_area_chart::{
    MemoryPoolAreaChart, MemoryPoolSeries, MemoryDataPoint, MemoryPoolColor, MemoryPoolConfig,
};
pub use dashboard_filter_bar::{DashboardFilterBar, FilterOption, FilterConfig, FilterState};

// P2 Component exports (Group 00/6)
pub use concentric_progress_rings::{
    ConcentricProgressRings, RingSegment, RingColor, ConcentricRingsConfig,
};
pub use action_item_card::{ActionItemCard, ActionItemList, ActionItemStatus, ActionItemCardConfig};

// P2 Component exports (Group 00/7)
pub use resource_timings_chart::{
    ResourceTimingsChart, TimingCategory, TimingDataPoint, ResourceTimingsConfig,
};
pub use gradient_score_gauge::{
    GradientScoreGauge, ScoreRating, ScoreHistoryPoint, GradientScoreConfig,
};
pub use discrete_status_timeline::{
    DiscreteStatusTimeline, TimelineRow, TimelineStatus, DiscreteTimelineConfig,
    MetricTimelineCard,
};
pub use health_score_donut::{
    HealthScoreDonut, HealthSegment, HealthLevel, TrendStat, HealthDonutConfig,
    CompactHealthDonut,
};

// P2 Component exports (Group 00/3)
pub use honeycomb_health_grid::{
    HoneycombHealthGrid, HexCell, HexHealthStatus, HoneycombConfig,
};
pub use job_progress_panel::{
    JobProgressPanel, JobProgress, JobStatus, JobProgressConfig,
};

// P2 Component exports (Group 00/0)
pub use system_health_card::{
    SystemHealthCard, SystemHealthRating, ServiceIndicator, ServiceIcon, ServiceStatus,
    SystemHealthCardConfig,
};

// P2 Component exports (Group 00/4)
pub use multi_series_line_chart::{
    MultiSeriesLineChart, ChartSeries, ChartDataPoint, ChartSeriesColor,
    LineInterpolation, MultiSeriesChartConfig,
};

// P2 Component exports (Group 00/5)
pub use battery_gauge::{BatteryGauge, CompactBatteryIndicator, BatteryGaugeConfig};

// P2 Component exports (Group 00/1)
pub use grade_stat_card::{
    GradeStatCard, GradeStatCardStack, GradeStatCardData, GradeLevel, GradeCardTheme, GradeStatCardConfig,
};
