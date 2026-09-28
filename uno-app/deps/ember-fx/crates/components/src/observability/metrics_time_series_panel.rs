//! MetricsTimeSeriesPanel Leptos component.
//!
//! A Grafana-style time-series chart with multi-series legend and statistics.

use leptos::prelude::*;
use crate::try_use_theme;

/// Color variants for metrics series.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MetricsColor {
    /// Green.
    #[default]
    Green,
    /// Yellow.
    Yellow,
    /// Orange.
    Orange,
    /// Blue.
    Blue,
    /// Cyan.
    Cyan,
    /// Magenta/Pink.
    Magenta,
    /// Red.
    Red,
    /// Gray.
    Gray,
}

impl MetricsColor {
    /// Returns the CSS color value.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Green => "#73bf69",
            Self::Yellow => "#fade2a",
            Self::Orange => "#ff9830",
            Self::Blue => "#5794f2",
            Self::Cyan => "#73bfb0",
            Self::Magenta => "#ff6eb4",
            Self::Red => "#f2495c",
            Self::Gray => "#8c8c8c",
        }
    }

    /// Returns a fill color with opacity.
    pub fn as_fill_css(&self, opacity: f64) -> String {
        let hex = match self {
            Self::Green => (115, 191, 105),
            Self::Yellow => (250, 222, 42),
            Self::Orange => (255, 152, 48),
            Self::Blue => (87, 148, 242),
            Self::Cyan => (115, 191, 176),
            Self::Magenta => (255, 110, 180),
            Self::Red => (242, 73, 92),
            Self::Gray => (140, 140, 140),
        };
        format!("rgba({}, {}, {}, {})", hex.0, hex.1, hex.2, opacity)
    }

    /// Get color by index (cycles through palette).
    pub fn from_index(index: usize) -> Self {
        match index % 8 {
            0 => Self::Green,
            1 => Self::Magenta,
            2 => Self::Yellow,
            3 => Self::Blue,
            4 => Self::Cyan,
            5 => Self::Orange,
            6 => Self::Red,
            _ => Self::Gray,
        }
    }
}

/// A data point in a metrics series.
#[derive(Debug, Clone)]
pub struct MetricsDataPoint {
    /// Timestamp in Unix milliseconds.
    pub timestamp: i64,
    /// Value at this timestamp.
    pub value: f64,
}

impl MetricsDataPoint {
    /// Create a new data point.
    pub fn new(timestamp: i64, value: f64) -> Self {
        Self { timestamp, value }
    }
}

/// A series of metrics data.
#[derive(Debug, Clone)]
pub struct MetricsSeries {
    /// Series name (displayed in legend).
    pub name: String,
    /// Data points.
    pub data: Vec<MetricsDataPoint>,
    /// Series color.
    pub color: MetricsColor,
}

impl MetricsSeries {
    /// Create a new series.
    pub fn new(name: impl Into<String>, data: Vec<MetricsDataPoint>, color: MetricsColor) -> Self {
        Self {
            name: name.into(),
            data,
            color,
        }
    }

    /// Calculate statistics for this series.
    pub fn stats(&self) -> SeriesStats {
        if self.data.is_empty() {
            return SeriesStats::default();
        }

        let values: Vec<f64> = self.data.iter().map(|p| p.value).collect();
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let sum: f64 = values.iter().sum();
        let avg = sum / values.len() as f64;
        let current = values.last().cloned().unwrap_or(0.0);

        SeriesStats { min, max, avg, current }
    }
}

/// Statistics for a series.
#[derive(Debug, Clone, Default)]
pub struct SeriesStats {
    /// Minimum value.
    pub min: f64,
    /// Maximum value.
    pub max: f64,
    /// Average value.
    pub avg: f64,
    /// Current (last) value.
    pub current: f64,
}

/// Configuration for the chart.
#[derive(Debug, Clone)]
pub struct MetricsChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Show grid lines.
    pub show_grid: bool,
    /// Show legend.
    pub show_legend: bool,
    /// Show statistics columns.
    pub show_stats: bool,
    /// Enable stacked mode.
    pub stacked: bool,
    /// Fill opacity for areas.
    pub fill_opacity: f64,
    /// Y-axis unit suffix.
    pub y_unit: Option<String>,
}

impl Default for MetricsChartConfig {
    fn default() -> Self {
        Self {
            height: 200,
            show_grid: true,
            show_legend: true,
            show_stats: true,
            stacked: false,
            fill_opacity: 0.1,
            y_unit: None,
        }
    }
}

/// Format a timestamp for display on the x-axis.
fn format_time(timestamp: i64) -> String {
    // Convert milliseconds to hours:minutes
    let total_seconds = timestamp / 1000;
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    format!("{:02}:{:02}", hours, minutes)
}

/// MetricsTimeSeriesPanel component.
///
/// A Grafana-style time-series chart with legend and statistics.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{
///     MetricsTimeSeriesPanel, MetricsSeries, MetricsDataPoint, MetricsColor,
/// };
///
/// let series = vec![
///     MetricsSeries::new("cpu-user", data1, MetricsColor::Green),
///     MetricsSeries::new("cpu-system", data2, MetricsColor::Yellow),
/// ];
///
/// view! {
///     <MetricsTimeSeriesPanel
///         series=Signal::derive(move || series.clone())
///         title="CPU Usage".to_string()
///     />
/// }
/// ```
#[component]
pub fn MetricsTimeSeriesPanel(
    /// Series data to display.
    #[prop(into)]
    series: Signal<Vec<MetricsSeries>>,
    /// Chart title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Chart configuration.
    #[prop(optional)]
    config: Option<MetricsChartConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let height = config.height;
    let show_grid = config.show_grid;
    let show_legend = config.show_legend;
    let show_stats = config.show_stats;
    let _stacked = config.stacked;
    let fill_opacity = config.fill_opacity;
    let y_unit = config.y_unit.clone();

    let prefix = format!("fx-metrics-ts-{}", design_system);

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![prefix.clone()];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // SVG dimensions
    let svg_width: f64 = 800.0;
    let svg_height: f64 = height as f64;
    let padding_left: f64 = 50.0;
    let padding_right: f64 = 20.0;
    let padding_top: f64 = 20.0;
    let padding_bottom: f64 = 30.0;

    let inner_width = svg_width - padding_left - padding_right;
    let inner_height = svg_height - padding_top - padding_bottom;

    // Calculate bounds
    let bounds = move || {
        let all_series = series.get();
        if all_series.is_empty() {
            return (0i64, 0i64, 0.0f64, 100.0f64);
        }

        let mut min_time = i64::MAX;
        let mut max_time = i64::MIN;
        let mut min_val = f64::INFINITY;
        let mut max_val = f64::NEG_INFINITY;

        for s in &all_series {
            for p in &s.data {
                min_time = min_time.min(p.timestamp);
                max_time = max_time.max(p.timestamp);
                min_val = min_val.min(p.value);
                max_val = max_val.max(p.value);
            }
        }

        // Add some padding to y range
        let y_range = (max_val - min_val).max(0.001);
        min_val = (min_val - y_range * 0.05).max(0.0);
        max_val = max_val + y_range * 0.05;

        (min_time, max_time, min_val, max_val)
    };

    // Generate paths for each series
    let series_paths = move || {
        let all_series = series.get();
        let (min_time, max_time, min_val, max_val) = bounds();
        let time_range = (max_time - min_time).max(1) as f64;
        let val_range = (max_val - min_val).max(0.001);

        all_series.iter().map(|s| {
            let mut line_path = String::new();
            let mut fill_path = String::new();

            if !s.data.is_empty() {
                // Start fill at bottom left
                let first_x = padding_left + ((s.data[0].timestamp - min_time) as f64 / time_range) * inner_width;
                fill_path.push_str(&format!("M {:.1} {:.1}", first_x, padding_top + inner_height));

                for (i, p) in s.data.iter().enumerate() {
                    let x = padding_left + ((p.timestamp - min_time) as f64 / time_range) * inner_width;
                    let y = padding_top + inner_height - ((p.value - min_val) / val_range) * inner_height;

                    if i == 0 {
                        line_path.push_str(&format!("M {:.1} {:.1}", x, y));
                        fill_path.push_str(&format!(" L {:.1} {:.1}", x, y));
                    } else {
                        line_path.push_str(&format!(" L {:.1} {:.1}", x, y));
                        fill_path.push_str(&format!(" L {:.1} {:.1}", x, y));
                    }
                }

                // Close fill path
                let last_x = padding_left + ((s.data.last().unwrap().timestamp - min_time) as f64 / time_range) * inner_width;
                fill_path.push_str(&format!(" L {:.1} {:.1} Z", last_x, padding_top + inner_height));
            }

            (s.name.clone(), s.color, line_path, fill_path, s.stats())
        }).collect::<Vec<_>>()
    };

    // Generate grid lines
    let grid_lines = move || {
        let (min_time, max_time, min_val, max_val) = bounds();
        let time_range = (max_time - min_time).max(1) as f64;
        let val_range = (max_val - min_val).max(0.001);

        let mut lines = Vec::new();

        // Horizontal grid lines (5 lines)
        for i in 0..=4 {
            let y = padding_top + (i as f64 / 4.0) * inner_height;
            let val = max_val - (i as f64 / 4.0) * val_range;
            lines.push((
                format!("M {:.1} {:.1} L {:.1} {:.1}", padding_left, y, padding_left + inner_width, y),
                format!("{:.0}", val),
                padding_left - 5.0,
                y,
                true, // is horizontal
            ));
        }

        // Vertical grid lines (6 lines)
        for i in 0..=5 {
            let x = padding_left + (i as f64 / 5.0) * inner_width;
            let time = min_time + ((i as f64 / 5.0) * time_range) as i64;
            lines.push((
                format!("M {:.1} {:.1} L {:.1} {:.1}", x, padding_top, x, padding_top + inner_height),
                format_time(time),
                x,
                padding_top + inner_height + 15.0,
                false, // is vertical
            ));
        }

        lines
    };

    // Clone for use in closures
    let y_unit_clone = y_unit.clone();

    view! {
        <div class=combined_class>
            // Title
            {title.clone().map(|t| {
                let prefix = prefix.clone();
                view! {
                    <div class=format!("{}-title", prefix)>
                        {t}
                    </div>
                }
            })}

            // Chart SVG
            <svg
                class=format!("{}-chart", prefix)
                viewBox=format!("0 0 {} {}", svg_width, svg_height)
                preserveAspectRatio="xMidYMid meet"
            >
                // Grid lines
                {move || {
                    if show_grid {
                        Some(grid_lines().into_iter().map(|(path, label, x, y, is_h)| {
                            view! {
                                <g>
                                    <path
                                        d=path
                                        fill="none"
                                        stroke="var(--fx-color-border, #303030)"
                                        stroke-width="1"
                                        stroke-dasharray={if is_h { "none" } else { "2,2" }}
                                    />
                                    <text
                                        x=x
                                        y=y
                                        fill="var(--fx-color-text-tertiary, #6b7280)"
                                        font-size="10"
                                        text-anchor={if is_h { "end" } else { "middle" }}
                                        dominant-baseline={if is_h { "middle" } else { "hanging" }}
                                    >
                                        {label}
                                    </text>
                                </g>
                            }
                        }).collect_view())
                    } else {
                        None
                    }
                }}

                // Series paths
                {move || {
                    series_paths().into_iter().map(|(name, color, line_path, fill_path, _stats)| {
                        let fill_color = color.as_fill_css(fill_opacity);
                        let line_color = color.as_css();

                        view! {
                            <g class="series" data-name=name>
                                // Fill area
                                <path
                                    d=fill_path
                                    fill=fill_color
                                />
                                // Line
                                <path
                                    d=line_path
                                    fill="none"
                                    stroke=line_color
                                    stroke-width="1.5"
                                    stroke-linejoin="round"
                                />
                            </g>
                        }
                    }).collect_view()
                }}
            </svg>

            // Legend with stats
            {move || {
                if show_legend {
                    let paths = series_paths();
                    let prefix = prefix.clone();
                    let y_unit = y_unit_clone.clone();

                    Some(view! {
                        <div class=format!("{}-legend", prefix)>
                            // Header row
                            <div class=format!("{}-legend-header", prefix)>
                                <span class=format!("{}-legend-name-col", prefix)></span>
                                {show_stats.then(|| {
                                    let prefix = prefix.clone();
                                    view! {
                                        <>
                                            <span class=format!("{}-legend-stat-col", prefix)>"min"</span>
                                            <span class=format!("{}-legend-stat-col", prefix)>"max"</span>
                                            <span class=format!("{}-legend-stat-col", prefix)>"avg"</span>
                                            <span class=format!("{}-legend-stat-col {}-legend-current", prefix, prefix)>"current"</span>
                                        </>
                                    }
                                })}
                            </div>

                            // Series rows
                            {paths.into_iter().map(|(name, color, _, _, stats)| {
                                let prefix = prefix.clone();
                                let y_unit = y_unit.clone();
                                let unit = y_unit.clone().unwrap_or_default();

                                view! {
                                    <div class=format!("{}-legend-row", prefix)>
                                        <span class=format!("{}-legend-name", prefix)>
                                            <span
                                                class=format!("{}-legend-color", prefix)
                                                style=format!("background-color: {};", color.as_css())
                                            />
                                            {name}
                                        </span>
                                        {show_stats.then(|| {
                                            let prefix = prefix.clone();
                                            let unit = unit.clone();
                                            view! {
                                                <>
                                                    <span class=format!("{}-legend-stat", prefix)>
                                                        {format!("{:.0}{}", stats.min, unit)}
                                                    </span>
                                                    <span class=format!("{}-legend-stat", prefix)>
                                                        {format!("{:.0}{}", stats.max, unit)}
                                                    </span>
                                                    <span class=format!("{}-legend-stat", prefix)>
                                                        {format!("{:.0}{}", stats.avg, unit)}
                                                    </span>
                                                    <span class=format!("{}-legend-stat {}-legend-current", prefix, prefix)>
                                                        {format!("{:.0}{}", stats.current, unit)}
                                                    </span>
                                                </>
                                            }
                                        })}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
