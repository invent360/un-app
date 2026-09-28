//! Chart component types.
//!
//! This module provides data types for all chart components including:
//! - Basic types: DataPoint, XYPoint, CategoryPoint
//! - Financial: OHLCPoint, RangePoint
//! - Statistical: BoxPlotPoint, BubblePoint
//! - Categorical: PieSlice, FunnelSegment, HeatmapCell, TreemapNode
//! - Temporal: TimelineEvent
//! - Radial: RadarPoint

use std::collections::HashMap;

/// Data point for time-series charts.
#[derive(Debug, Clone, PartialEq)]
pub struct DataPoint {
    /// Unix timestamp in milliseconds.
    pub timestamp: i64,
    /// Value at this point.
    pub value: f64,
    /// Optional label for the point.
    pub label: Option<String>,
}

impl DataPoint {
    /// Create a new data point.
    pub fn new(timestamp: i64, value: f64) -> Self {
        Self {
            timestamp,
            value,
            label: None,
        }
    }

    /// Create a data point with a label.
    pub fn with_label(timestamp: i64, value: f64, label: impl Into<String>) -> Self {
        Self {
            timestamp,
            value,
            label: Some(label.into()),
        }
    }
}

/// Line chart configuration.
#[derive(Debug, Clone)]
pub struct LineChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Show legend.
    pub show_legend: bool,
    /// Show grid lines.
    pub show_grid: bool,
    /// Enable animations (web only).
    pub animate: bool,
    /// Minimum Y value (auto-scale if None).
    pub y_min: Option<f64>,
    /// Maximum Y value (auto-scale if None).
    pub y_max: Option<f64>,
    /// Line color.
    pub color: ChartColor,
    /// Line width in pixels.
    pub line_width: f32,
    /// Show data points.
    pub show_points: bool,
    /// Fill area under line.
    pub fill: bool,
}

impl Default for LineChartConfig {
    fn default() -> Self {
        Self {
            height: 200,
            show_legend: true,
            show_grid: true,
            animate: true,
            y_min: None,
            y_max: None,
            color: ChartColor::Primary,
            line_width: 2.0,
            show_points: true,
            fill: false,
        }
    }
}

/// Area chart series.
#[derive(Debug, Clone)]
pub struct AreaSeries {
    /// Series name.
    pub name: String,
    /// Data points.
    pub data: Vec<DataPoint>,
    /// Series color.
    pub color: ChartColor,
}

impl AreaSeries {
    /// Create a new area series.
    pub fn new(name: impl Into<String>, data: Vec<DataPoint>) -> Self {
        Self {
            name: name.into(),
            data,
            color: ChartColor::Primary,
        }
    }

    /// Set the series color.
    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = color;
        self
    }
}

/// Area chart configuration.
#[derive(Debug, Clone)]
pub struct AreaChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Stack areas.
    pub stacked: bool,
    /// Show legend.
    pub show_legend: bool,
    /// Show grid lines.
    pub show_grid: bool,
    /// Enable animations.
    pub animate: bool,
    /// Fill opacity (0.0 - 1.0).
    pub fill_opacity: f32,
    /// Show data points for interactivity.
    pub show_points: bool,
}

impl Default for AreaChartConfig {
    fn default() -> Self {
        Self {
            height: 200,
            stacked: false,
            show_legend: true,
            show_grid: true,
            animate: true,
            fill_opacity: 0.3,
            show_points: true,
        }
    }
}

/// Sparkline chart configuration.
#[derive(Debug, Clone)]
pub struct SparklineConfig {
    /// Chart width in pixels.
    pub width: u32,
    /// Chart height in pixels.
    pub height: u32,
    /// Line color.
    pub color: ChartColor,
    /// Line width.
    pub line_width: f32,
    /// Show min/max markers.
    pub show_markers: bool,
    /// Fill area under line.
    pub fill: bool,
}

impl Default for SparklineConfig {
    fn default() -> Self {
        Self {
            width: 100,
            height: 24,
            color: ChartColor::Primary,
            line_width: 1.5,
            show_markers: false,
            fill: false,
        }
    }
}

/// Chart color variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ChartColor {
    /// Primary theme color.
    #[default]
    Primary,
    /// Success color (green).
    Success,
    /// Warning color (yellow/orange).
    Warning,
    /// Error color (red).
    Error,
    /// Info color (blue).
    Info,
    /// Gray/neutral color.
    Gray,
}

impl ChartColor {
    /// Returns the CSS variable for this color.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Primary => "var(--fx-color-primary, #1890ff)",
            Self::Success => "var(--fx-color-success, #52c41a)",
            Self::Warning => "var(--fx-color-warning, #faad14)",
            Self::Error => "var(--fx-color-error, #ff4d4f)",
            Self::Info => "var(--fx-color-info, #1890ff)",
            Self::Gray => "var(--fx-color-text-tertiary, #8c8c8c)",
        }
    }

    /// Returns the fill CSS variable (with opacity).
    pub fn as_fill_css(&self) -> &'static str {
        match self {
            Self::Primary => "var(--fx-color-primary-bg, rgba(24, 144, 255, 0.1))",
            Self::Success => "var(--fx-color-success-bg, rgba(82, 196, 26, 0.1))",
            Self::Warning => "var(--fx-color-warning-bg, rgba(250, 173, 20, 0.1))",
            Self::Error => "var(--fx-color-error-bg, rgba(255, 77, 79, 0.1))",
            Self::Info => "var(--fx-color-info-bg, rgba(24, 144, 255, 0.1))",
            Self::Gray => "var(--fx-color-bg-elevated, rgba(140, 140, 140, 0.1))",
        }
    }
}

/// Multi-series chart data.
#[derive(Debug, Clone)]
pub struct MultiSeriesData {
    /// Series data.
    pub series: Vec<ChartSeries>,
}

/// Single chart series.
#[derive(Debug, Clone)]
pub struct ChartSeries {
    /// Series name.
    pub name: String,
    /// Data points.
    pub data: Vec<DataPoint>,
    /// Series color.
    pub color: ChartColor,
    /// Series type (for mixed charts).
    pub series_type: SeriesType,
}

impl ChartSeries {
    /// Create a new chart series.
    pub fn new(name: impl Into<String>, data: Vec<DataPoint>) -> Self {
        Self {
            name: name.into(),
            data,
            color: ChartColor::Primary,
            series_type: SeriesType::Line,
        }
    }

    /// Set the color.
    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = color;
        self
    }

    /// Set the series type.
    pub fn series_type(mut self, series_type: SeriesType) -> Self {
        self.series_type = series_type;
        self
    }
}

/// Chart series type.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SeriesType {
    /// Line chart.
    #[default]
    Line,
    /// Area chart.
    Area,
    /// Bar chart.
    Bar,
}

/// Axis configuration.
#[derive(Debug, Clone, Default)]
pub struct AxisConfig {
    /// Axis label.
    pub label: Option<String>,
    /// Min value.
    pub min: Option<f64>,
    /// Max value.
    pub max: Option<f64>,
    /// Tick count.
    pub tick_count: Option<usize>,
    /// Format function name.
    pub format: Option<String>,
}

/// Tooltip configuration.
#[derive(Debug, Clone, Default)]
pub struct TooltipConfig {
    /// Show tooltip.
    pub enabled: bool,
    /// Show crosshairs.
    pub crosshairs: bool,
    /// Custom formatter.
    pub formatter: Option<String>,
}

// =============================================================================
// Extended Point Types
// =============================================================================

/// Generic X/Y point for scatter and similar charts.
#[derive(Debug, Clone, PartialEq)]
pub struct XYPoint {
    /// X coordinate value.
    pub x: f64,
    /// Y coordinate value.
    pub y: f64,
    /// Optional label.
    pub label: Option<String>,
}

impl XYPoint {
    /// Create a new XY point.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, label: None }
    }

    /// Create an XY point with label.
    pub fn with_label(x: f64, y: f64, label: impl Into<String>) -> Self {
        Self {
            x,
            y,
            label: Some(label.into()),
        }
    }
}

/// Category-based data point for bar/column charts.
#[derive(Debug, Clone, PartialEq)]
pub struct CategoryPoint {
    /// Category name (x-axis).
    pub category: String,
    /// Value (y-axis).
    pub value: f64,
}

impl CategoryPoint {
    /// Create a new category point.
    pub fn new(category: impl Into<String>, value: f64) -> Self {
        Self {
            category: category.into(),
            value,
        }
    }
}

/// Bubble chart point with size dimension.
#[derive(Debug, Clone, PartialEq)]
pub struct BubblePoint {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Size/z value.
    pub z: f64,
    /// Optional label.
    pub label: Option<String>,
}

impl BubblePoint {
    /// Create a new bubble point.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            label: None,
        }
    }

    /// Create a bubble point with label.
    pub fn with_label(x: f64, y: f64, z: f64, label: impl Into<String>) -> Self {
        Self {
            x,
            y,
            z,
            label: Some(label.into()),
        }
    }
}

/// OHLC (Open-High-Low-Close) point for candlestick charts.
#[derive(Debug, Clone, PartialEq)]
pub struct OHLCPoint {
    /// Unix timestamp in milliseconds.
    pub timestamp: i64,
    /// Opening price.
    pub open: f64,
    /// Highest price.
    pub high: f64,
    /// Lowest price.
    pub low: f64,
    /// Closing price.
    pub close: f64,
    /// Optional volume.
    pub volume: Option<f64>,
}

impl OHLCPoint {
    /// Create a new OHLC point.
    pub fn new(timestamp: i64, open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            timestamp,
            open,
            high,
            low,
            close,
            volume: None,
        }
    }

    /// Add volume to the OHLC point.
    pub fn with_volume(mut self, volume: f64) -> Self {
        self.volume = Some(volume);
        self
    }

    /// Check if this is a bullish (up) candle.
    pub fn is_bullish(&self) -> bool {
        self.close >= self.open
    }
}

/// Range point with min/max values.
#[derive(Debug, Clone, PartialEq)]
pub struct RangePoint {
    /// X coordinate (or timestamp).
    pub x: f64,
    /// Minimum Y value.
    pub y_min: f64,
    /// Maximum Y value.
    pub y_max: f64,
}

impl RangePoint {
    /// Create a new range point.
    pub fn new(x: f64, y_min: f64, y_max: f64) -> Self {
        Self { x, y_min, y_max }
    }

    /// Create from timestamp.
    pub fn from_timestamp(timestamp: i64, y_min: f64, y_max: f64) -> Self {
        Self {
            x: timestamp as f64,
            y_min,
            y_max,
        }
    }
}

/// Box plot statistical data point.
#[derive(Debug, Clone, PartialEq)]
pub struct BoxPlotPoint {
    /// Category label.
    pub category: String,
    /// Minimum value.
    pub min: f64,
    /// First quartile (Q1, 25th percentile).
    pub q1: f64,
    /// Median (Q2, 50th percentile).
    pub median: f64,
    /// Third quartile (Q3, 75th percentile).
    pub q3: f64,
    /// Maximum value.
    pub max: f64,
    /// Optional outliers.
    pub outliers: Vec<f64>,
}

impl BoxPlotPoint {
    /// Create a new box plot point.
    pub fn new(category: impl Into<String>, min: f64, q1: f64, median: f64, q3: f64, max: f64) -> Self {
        Self {
            category: category.into(),
            min,
            q1,
            median,
            q3,
            max,
            outliers: Vec::new(),
        }
    }

    /// Add outliers.
    pub fn with_outliers(mut self, outliers: Vec<f64>) -> Self {
        self.outliers = outliers;
        self
    }

    /// Calculate interquartile range.
    pub fn iqr(&self) -> f64 {
        self.q3 - self.q1
    }
}

/// Heatmap cell data.
#[derive(Debug, Clone, PartialEq)]
pub struct HeatmapCell {
    /// X category/index.
    pub x: String,
    /// Y category/index.
    pub y: String,
    /// Cell value (determines color intensity).
    pub value: f64,
}

impl HeatmapCell {
    /// Create a new heatmap cell.
    pub fn new(x: impl Into<String>, y: impl Into<String>, value: f64) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            value,
        }
    }
}

/// Treemap node for hierarchical data.
#[derive(Debug, Clone, PartialEq)]
pub struct TreemapNode {
    /// Node name/label.
    pub name: String,
    /// Node value (determines size).
    pub value: f64,
    /// Child nodes.
    pub children: Vec<TreemapNode>,
    /// Optional color override.
    pub color: Option<ChartColor>,
}

impl TreemapNode {
    /// Create a leaf node with value.
    pub fn leaf(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
            children: Vec::new(),
            color: None,
        }
    }

    /// Create a parent node with children.
    pub fn parent(name: impl Into<String>, children: Vec<TreemapNode>) -> Self {
        let value = children.iter().map(|c| c.total_value()).sum();
        Self {
            name: name.into(),
            value,
            children,
            color: None,
        }
    }

    /// Set color.
    pub fn with_color(mut self, color: ChartColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Get total value (including children).
    pub fn total_value(&self) -> f64 {
        if self.children.is_empty() {
            self.value
        } else {
            self.children.iter().map(|c| c.total_value()).sum()
        }
    }
}

/// Timeline event for Gantt-style charts.
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineEvent {
    /// Row/category name.
    pub row: String,
    /// Event start time (Unix ms).
    pub start: i64,
    /// Event end time (Unix ms).
    pub end: i64,
    /// Event label.
    pub label: Option<String>,
    /// Event color.
    pub color: Option<ChartColor>,
}

impl TimelineEvent {
    /// Create a new timeline event.
    pub fn new(row: impl Into<String>, start: i64, end: i64) -> Self {
        Self {
            row: row.into(),
            start,
            end,
            label: None,
            color: None,
        }
    }

    /// Set event label.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set event color.
    pub fn with_color(mut self, color: ChartColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Get duration in milliseconds.
    pub fn duration_ms(&self) -> i64 {
        self.end - self.start
    }
}

/// Radar chart data point.
#[derive(Debug, Clone, PartialEq)]
pub struct RadarPoint {
    /// Axis/category name.
    pub axis: String,
    /// Value (typically 0-100 or normalized).
    pub value: f64,
}

impl RadarPoint {
    /// Create a new radar point.
    pub fn new(axis: impl Into<String>, value: f64) -> Self {
        Self {
            axis: axis.into(),
            value,
        }
    }
}

/// Radar series with multiple axes.
#[derive(Debug, Clone)]
pub struct RadarSeries {
    /// Series name.
    pub name: String,
    /// Data points (one per axis).
    pub data: Vec<RadarPoint>,
    /// Series color.
    pub color: ChartColor,
    /// Fill opacity.
    pub fill_opacity: f32,
}

impl RadarSeries {
    /// Create a new radar series.
    pub fn new(name: impl Into<String>, data: Vec<RadarPoint>) -> Self {
        Self {
            name: name.into(),
            data,
            color: ChartColor::Primary,
            fill_opacity: 0.25,
        }
    }

    /// Set color.
    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = color;
        self
    }

    /// Set fill opacity.
    pub fn fill_opacity(mut self, opacity: f32) -> Self {
        self.fill_opacity = opacity;
        self
    }
}

/// Pie/donut chart slice.
#[derive(Debug, Clone, PartialEq)]
pub struct PieSlice {
    /// Slice label.
    pub label: String,
    /// Slice value.
    pub value: f64,
    /// Optional color override.
    pub color: Option<ChartColor>,
}

impl PieSlice {
    /// Create a new pie slice.
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Set color.
    pub fn with_color(mut self, color: ChartColor) -> Self {
        self.color = Some(color);
        self
    }
}

/// Funnel chart segment.
#[derive(Debug, Clone, PartialEq)]
pub struct FunnelSegment {
    /// Segment label.
    pub label: String,
    /// Segment value.
    pub value: f64,
    /// Optional color override.
    pub color: Option<ChartColor>,
}

impl FunnelSegment {
    /// Create a new funnel segment.
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Set color.
    pub fn with_color(mut self, color: ChartColor) -> Self {
        self.color = Some(color);
        self
    }
}

// =============================================================================
// Generic Series Type
// =============================================================================

/// Generic series container for any point type.
#[derive(Debug, Clone)]
pub struct Series<T> {
    /// Series name.
    pub name: String,
    /// Data points.
    pub data: Vec<T>,
    /// Series color.
    pub color: ChartColor,
    /// Stroke width.
    pub stroke_width: f32,
    /// Fill opacity.
    pub fill_opacity: f32,
}

impl<T> Series<T> {
    /// Create a new series.
    pub fn new(name: impl Into<String>, data: Vec<T>) -> Self {
        Self {
            name: name.into(),
            data,
            color: ChartColor::Primary,
            stroke_width: 2.0,
            fill_opacity: 0.3,
        }
    }

    /// Set color.
    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = color;
        self
    }

    /// Set stroke width.
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    /// Set fill opacity.
    pub fn fill_opacity(mut self, opacity: f32) -> Self {
        self.fill_opacity = opacity;
        self
    }
}

// =============================================================================
// Chart Configurations
// =============================================================================

/// Bar chart configuration.
#[derive(Debug, Clone)]
pub struct BarChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Horizontal bars (true) or vertical columns (false).
    pub horizontal: bool,
    /// Stack bars.
    pub stacked: bool,
    /// Bar width ratio (0.0 - 1.0).
    pub bar_width: f32,
    /// Show data labels on bars.
    pub show_data_labels: bool,
    /// Show grid lines.
    pub show_grid: bool,
    /// Show legend.
    pub show_legend: bool,
    /// Enable animations.
    pub animate: bool,
    /// Border radius for bars.
    pub border_radius: f32,
}

impl Default for BarChartConfig {
    fn default() -> Self {
        Self {
            height: 300,
            horizontal: true,
            stacked: false,
            bar_width: 0.7,
            show_data_labels: false,
            show_grid: true,
            show_legend: true,
            animate: true,
            border_radius: 4.0,
        }
    }
}

/// Pie/donut chart configuration.
#[derive(Debug, Clone)]
pub struct PieChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Render as donut.
    pub donut: bool,
    /// Donut inner radius ratio (0.0 - 1.0).
    pub donut_width: f32,
    /// Start angle in degrees.
    pub start_angle: f32,
    /// Show slice labels.
    pub show_labels: bool,
    /// Show legend.
    pub show_legend: bool,
    /// Use gradient fills.
    pub gradient: bool,
    /// Enable animations.
    pub animate: bool,
}

impl Default for PieChartConfig {
    fn default() -> Self {
        Self {
            height: 300,
            donut: false,
            donut_width: 0.55,
            start_angle: 0.0,
            show_labels: true,
            show_legend: true,
            gradient: false,
            animate: true,
        }
    }
}

/// Scatter chart configuration.
#[derive(Debug, Clone)]
pub struct ScatterChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Point radius.
    pub point_radius: f32,
    /// Show grid.
    pub show_grid: bool,
    /// Show legend.
    pub show_legend: bool,
    /// Enable animations.
    pub animate: bool,
    /// X-axis config.
    pub x_axis: AxisConfig,
    /// Y-axis config.
    pub y_axis: AxisConfig,
}

impl Default for ScatterChartConfig {
    fn default() -> Self {
        Self {
            height: 300,
            point_radius: 5.0,
            show_grid: true,
            show_legend: true,
            animate: true,
            x_axis: AxisConfig::default(),
            y_axis: AxisConfig::default(),
        }
    }
}

/// Bubble chart configuration.
#[derive(Debug, Clone)]
pub struct BubbleChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Min bubble radius.
    pub min_radius: f32,
    /// Max bubble radius.
    pub max_radius: f32,
    /// Show grid.
    pub show_grid: bool,
    /// Show legend.
    pub show_legend: bool,
    /// Bubble opacity.
    pub opacity: f32,
    /// Enable animations.
    pub animate: bool,
}

impl Default for BubbleChartConfig {
    fn default() -> Self {
        Self {
            height: 300,
            min_radius: 4.0,
            max_radius: 30.0,
            show_grid: true,
            show_legend: true,
            opacity: 0.7,
            animate: true,
        }
    }
}

/// Heatmap chart configuration.
#[derive(Debug, Clone)]
pub struct HeatmapChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Color scale type.
    pub color_scale: ColorScale,
    /// Cell border radius.
    pub cell_radius: f32,
    /// Show values in cells.
    pub show_values: bool,
    /// Show legend/scale.
    pub show_legend: bool,
    /// Enable animations.
    pub animate: bool,
}

impl Default for HeatmapChartConfig {
    fn default() -> Self {
        Self {
            height: 300,
            color_scale: ColorScale::Sequential,
            cell_radius: 0.0,
            show_values: false,
            show_legend: true,
            animate: true,
        }
    }
}

/// Color scale type for heatmaps.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorScale {
    /// Sequential (single color gradient).
    #[default]
    Sequential,
    /// Diverging (two-color gradient from center).
    Diverging,
    /// Categorical (distinct colors).
    Categorical,
}

/// Candlestick chart configuration.
#[derive(Debug, Clone)]
pub struct CandlestickChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Bullish (up) candle color.
    pub up_color: ChartColor,
    /// Bearish (down) candle color.
    pub down_color: ChartColor,
    /// Show volume subplot.
    pub show_volume: bool,
    /// Show crosshair on hover.
    pub show_crosshair: bool,
    /// Show grid.
    pub show_grid: bool,
    /// Enable animations.
    pub animate: bool,
}

impl Default for CandlestickChartConfig {
    fn default() -> Self {
        Self {
            height: 400,
            up_color: ChartColor::Success,
            down_color: ChartColor::Error,
            show_volume: false,
            show_crosshair: true,
            show_grid: true,
            animate: true,
        }
    }
}

/// Radial bar chart configuration.
#[derive(Debug, Clone)]
pub struct RadialBarChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Start angle in degrees.
    pub start_angle: f32,
    /// End angle in degrees (360 for full circle).
    pub end_angle: f32,
    /// Track background color.
    pub track_color: ChartColor,
    /// Track opacity.
    pub track_opacity: f32,
    /// Show value labels.
    pub show_labels: bool,
    /// Hollow center ratio (0-1).
    pub hollow_size: f32,
    /// Enable animations.
    pub animate: bool,
}

impl Default for RadialBarChartConfig {
    fn default() -> Self {
        Self {
            height: 300,
            start_angle: 0.0,
            end_angle: 360.0,
            track_color: ChartColor::Gray,
            track_opacity: 0.15,
            show_labels: true,
            hollow_size: 0.65,
            animate: true,
        }
    }
}

/// Gauge chart configuration.
#[derive(Debug, Clone)]
pub struct GaugeChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Min value.
    pub min: f64,
    /// Max value.
    pub max: f64,
    /// Start angle in degrees.
    pub start_angle: f32,
    /// End angle in degrees.
    pub end_angle: f32,
    /// Show tick marks.
    pub show_ticks: bool,
    /// Show value label.
    pub show_value: bool,
    /// Track color.
    pub track_color: ChartColor,
    /// Enable animations.
    pub animate: bool,
}

impl Default for GaugeChartConfig {
    fn default() -> Self {
        Self {
            height: 250,
            min: 0.0,
            max: 100.0,
            start_angle: -135.0,
            end_angle: 135.0,
            show_ticks: true,
            show_value: true,
            track_color: ChartColor::Gray,
            animate: true,
        }
    }
}

/// Radar chart configuration.
#[derive(Debug, Clone)]
pub struct RadarChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Show grid circles.
    pub show_grid: bool,
    /// Show axis labels.
    pub show_labels: bool,
    /// Show legend.
    pub show_legend: bool,
    /// Grid type (circle or polygon).
    pub polygon_grid: bool,
    /// Fill opacity for series areas.
    pub fill_opacity: f32,
    /// Enable animations.
    pub animate: bool,
}

impl Default for RadarChartConfig {
    fn default() -> Self {
        Self {
            height: 350,
            show_grid: true,
            show_labels: true,
            show_legend: true,
            polygon_grid: true,
            fill_opacity: 0.25,
            animate: true,
        }
    }
}

/// Treemap chart configuration.
#[derive(Debug, Clone)]
pub struct TreemapChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Show labels.
    pub show_labels: bool,
    /// Enable distributed colors.
    pub distributed_colors: bool,
    /// Border width.
    pub border_width: f32,
    /// Enable animations.
    pub animate: bool,
}

impl Default for TreemapChartConfig {
    fn default() -> Self {
        Self {
            height: 350,
            show_labels: true,
            distributed_colors: false,
            border_width: 1.0,
            animate: true,
        }
    }
}

/// Timeline chart configuration.
#[derive(Debug, Clone)]
pub struct TimelineChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Show axis.
    pub show_axis: bool,
    /// Bar height.
    pub bar_height: f32,
    /// Row padding.
    pub row_padding: f32,
    /// Show labels on bars.
    pub show_labels: bool,
    /// Enable animations.
    pub animate: bool,
}

impl Default for TimelineChartConfig {
    fn default() -> Self {
        Self {
            height: 300,
            show_axis: true,
            bar_height: 24.0,
            row_padding: 4.0,
            show_labels: true,
            animate: true,
        }
    }
}

/// Box plot chart configuration.
#[derive(Debug, Clone)]
pub struct BoxPlotChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Horizontal layout.
    pub horizontal: bool,
    /// Show outliers.
    pub show_outliers: bool,
    /// Box width ratio.
    pub box_width: f32,
    /// Show grid.
    pub show_grid: bool,
    /// Enable animations.
    pub animate: bool,
}

impl Default for BoxPlotChartConfig {
    fn default() -> Self {
        Self {
            height: 300,
            horizontal: false,
            show_outliers: true,
            box_width: 0.7,
            show_grid: true,
            animate: true,
        }
    }
}

/// Funnel chart configuration.
#[derive(Debug, Clone)]
pub struct FunnelChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Pyramid shape (inverted funnel).
    pub pyramid: bool,
    /// Show data labels.
    pub show_labels: bool,
    /// Show legend.
    pub show_legend: bool,
    /// Min width ratio.
    pub min_width: f32,
    /// Enable animations.
    pub animate: bool,
}

impl Default for FunnelChartConfig {
    fn default() -> Self {
        Self {
            height: 350,
            pyramid: false,
            show_labels: true,
            show_legend: true,
            min_width: 0.2,
            animate: true,
        }
    }
}

/// Slope chart configuration.
#[derive(Debug, Clone)]
pub struct SlopeChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Show grid.
    pub show_grid: bool,
    /// Show legend.
    pub show_legend: bool,
    /// Point radius.
    pub point_radius: f32,
    /// Line width.
    pub line_width: f32,
    /// Enable animations.
    pub animate: bool,
}

impl Default for SlopeChartConfig {
    fn default() -> Self {
        Self {
            height: 300,
            show_grid: true,
            show_legend: true,
            point_radius: 6.0,
            line_width: 2.0,
            animate: true,
        }
    }
}

/// Polar area chart configuration.
#[derive(Debug, Clone)]
pub struct PolarAreaChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Show legend.
    pub show_legend: bool,
    /// Show labels.
    pub show_labels: bool,
    /// Start angle in degrees.
    pub start_angle: f32,
    /// Enable animations.
    pub animate: bool,
}

impl Default for PolarAreaChartConfig {
    fn default() -> Self {
        Self {
            height: 300,
            show_legend: true,
            show_labels: true,
            start_angle: 0.0,
            animate: true,
        }
    }
}

/// Range area chart configuration.
#[derive(Debug, Clone)]
pub struct RangeAreaChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Fill opacity.
    pub fill_opacity: f32,
    /// Show grid.
    pub show_grid: bool,
    /// Show legend.
    pub show_legend: bool,
    /// Enable animations.
    pub animate: bool,
}

impl Default for RangeAreaChartConfig {
    fn default() -> Self {
        Self {
            height: 300,
            fill_opacity: 0.3,
            show_grid: true,
            show_legend: true,
            animate: true,
        }
    }
}

/// Mixed chart configuration.
#[derive(Debug, Clone)]
pub struct MixedChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Show grid.
    pub show_grid: bool,
    /// Show legend.
    pub show_legend: Option<bool>,
    /// Enable secondary Y-axis.
    pub dual_y_axis: bool,
    /// Enable animations.
    pub animate: bool,
    /// Number of Y-axis ticks.
    pub tick_count: Option<usize>,
    /// Show data points on lines.
    pub show_points: Option<bool>,
    /// Fill opacity for area series.
    pub fill_opacity: Option<f32>,
}

impl Default for MixedChartConfig {
    fn default() -> Self {
        Self {
            height: 350,
            show_grid: true,
            show_legend: Some(true),
            dual_y_axis: false,
            animate: true,
            tick_count: Some(6),
            show_points: Some(true),
            fill_opacity: Some(0.3),
        }
    }
}

/// Chart type for mixed charts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChartType {
    /// Line chart series.
    Line,
    /// Bar chart series.
    Bar,
    /// Area chart series.
    Area,
    /// Scatter plot series.
    Scatter,
}

impl Default for ChartType {
    fn default() -> Self {
        Self::Line
    }
}

/// Series data for mixed charts.
#[derive(Debug, Clone)]
pub struct MixedChartSeries {
    /// Series label.
    pub label: String,
    /// Series values.
    pub values: Vec<f64>,
    /// Chart type for this series.
    pub chart_type: ChartType,
    /// Optional color override.
    pub color: Option<String>,
}

impl MixedChartSeries {
    /// Create a new mixed chart series.
    pub fn new(label: impl Into<String>, values: Vec<f64>, chart_type: ChartType) -> Self {
        Self {
            label: label.into(),
            values,
            chart_type,
            color: None,
        }
    }

    /// Set series color.
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

/// Slope chart data point.
#[derive(Debug, Clone, PartialEq)]
pub struct SlopePoint {
    /// Label for this point.
    pub label: String,
    /// Start value (left side).
    pub start: f64,
    /// End value (right side).
    pub end: f64,
    /// Optional color override.
    pub color: Option<String>,
}

impl SlopePoint {
    /// Create a new slope point.
    pub fn new(label: impl Into<String>, start: f64, end: f64) -> Self {
        Self {
            label: label.into(),
            start,
            end,
            color: None,
        }
    }

    /// Set point color.
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

/// Polar area chart segment.
#[derive(Debug, Clone, PartialEq)]
pub struct PolarAreaSegment {
    /// Segment label.
    pub label: String,
    /// Segment value (determines radius).
    pub value: f64,
    /// Optional color override.
    pub color: Option<String>,
}

impl PolarAreaSegment {
    /// Create a new polar area segment.
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Set segment color.
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

// =============================================================================
// Color Palette
// =============================================================================

/// Predefined color palettes for charts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorPalette {
    /// Default theme colors.
    #[default]
    Default,
    /// Monochrome (single color shades).
    Monochrome,
    /// Pastel colors.
    Pastel,
    /// Vibrant colors.
    Vibrant,
}

impl ColorPalette {
    /// Get colors for this palette.
    pub fn colors(&self) -> &'static [&'static str] {
        match self {
            Self::Default => &[
                "var(--fx-color-primary, #1890ff)",
                "var(--fx-color-success, #52c41a)",
                "var(--fx-color-warning, #faad14)",
                "var(--fx-color-error, #ff4d4f)",
                "var(--fx-color-info, #722ed1)",
                "#13c2c2",
                "#eb2f96",
                "#fa8c16",
            ],
            Self::Monochrome => &[
                "#1890ff",
                "#40a9ff",
                "#69c0ff",
                "#91d5ff",
                "#bae7ff",
                "#e6f7ff",
            ],
            Self::Pastel => &[
                "#b5d8eb", "#c5e8b7", "#f5e6a3", "#f5c8a3",
                "#e8c5e8", "#c5e8e5", "#f5a3c5", "#e8e5c5",
            ],
            Self::Vibrant => &[
                "#ff6b6b", "#4ecdc4", "#45b7d1", "#f7dc6f",
                "#bb8fce", "#58d68d", "#f1948a", "#85c1e9",
            ],
        }
    }

    /// Get color at index (wraps around).
    pub fn color_at(&self, index: usize) -> &'static str {
        let colors = self.colors();
        colors[index % colors.len()]
    }
}
