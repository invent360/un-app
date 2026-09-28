//! MultiSeriesLineChart Leptos component.
//!
//! A multi-series time series chart with step-line or smooth interpolation
//! and a simple bottom legend.

use leptos::prelude::*;
use crate::try_use_theme;

/// Line interpolation style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineInterpolation {
    /// Smooth curved lines.
    #[default]
    Linear,
    /// Step-line (staircase) - horizontal then vertical.
    StepAfter,
    /// Step-line - vertical then horizontal.
    StepBefore,
}

/// Color variants for chart series.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ChartSeriesColor {
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
    /// White.
    White,
    /// Red.
    Red,
}

impl ChartSeriesColor {
    /// Returns the CSS color value.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Green => "#73bf69",
            Self::Yellow => "#fade2a",
            Self::Orange => "#ff9830",
            Self::Blue => "#5794f2",
            Self::Cyan => "#73bfb0",
            Self::White => "#ffffff",
            Self::Red => "#f2495c",
        }
    }

    /// Get color by index.
    pub fn from_index(index: usize) -> Self {
        match index % 7 {
            0 => Self::Green,
            1 => Self::Yellow,
            2 => Self::Blue,
            3 => Self::Orange,
            4 => Self::Cyan,
            5 => Self::White,
            _ => Self::Red,
        }
    }
}

/// A data point in a chart series.
#[derive(Debug, Clone)]
pub struct ChartDataPoint {
    /// Timestamp in Unix milliseconds.
    pub timestamp: i64,
    /// Value at this timestamp.
    pub value: f64,
}

impl ChartDataPoint {
    /// Create a new data point.
    pub fn new(timestamp: i64, value: f64) -> Self {
        Self { timestamp, value }
    }
}

/// A series of chart data.
#[derive(Debug, Clone)]
pub struct ChartSeries {
    /// Series name (displayed in legend).
    pub name: String,
    /// Data points.
    pub data: Vec<ChartDataPoint>,
    /// Series color.
    pub color: ChartSeriesColor,
}

impl ChartSeries {
    /// Create a new series.
    pub fn new(name: impl Into<String>, data: Vec<ChartDataPoint>, color: ChartSeriesColor) -> Self {
        Self {
            name: name.into(),
            data,
            color,
        }
    }

    /// Create with auto color from index.
    pub fn with_index(name: impl Into<String>, data: Vec<ChartDataPoint>, index: usize) -> Self {
        Self::new(name, data, ChartSeriesColor::from_index(index))
    }
}

/// Configuration for the multi-series line chart.
#[derive(Debug, Clone)]
pub struct MultiSeriesChartConfig {
    /// Chart height in pixels.
    pub height: u32,
    /// Line interpolation style.
    pub interpolation: LineInterpolation,
    /// Y-axis unit label (e.g., "°C", "%H", "ppm").
    pub y_unit: String,
    /// Show grid lines.
    pub show_grid: bool,
    /// Line stroke width.
    pub stroke_width: f64,
    /// Number of Y-axis ticks.
    pub y_ticks: usize,
    /// Number of X-axis time labels.
    pub x_labels: usize,
}

impl Default for MultiSeriesChartConfig {
    fn default() -> Self {
        Self {
            height: 200,
            interpolation: LineInterpolation::Linear,
            y_unit: String::new(),
            show_grid: true,
            stroke_width: 1.5,
            y_ticks: 5,
            x_labels: 6,
        }
    }
}

/// Generate SVG path for a series.
fn generate_path(
    data: &[ChartDataPoint],
    min_time: i64,
    max_time: i64,
    min_val: f64,
    max_val: f64,
    width: f64,
    height: f64,
    padding: f64,
    interpolation: LineInterpolation,
) -> String {
    if data.is_empty() {
        return String::new();
    }

    let time_range = (max_time - min_time) as f64;
    let val_range = max_val - min_val;
    let chart_width = width - padding * 2.0;
    let chart_height = height - padding * 2.0;

    let mut path = String::new();

    for (i, point) in data.iter().enumerate() {
        let x = padding + ((point.timestamp - min_time) as f64 / time_range) * chart_width;
        let y = padding + chart_height - ((point.value - min_val) / val_range) * chart_height;

        if i == 0 {
            path.push_str(&format!("M {:.1} {:.1}", x, y));
        } else {
            match interpolation {
                LineInterpolation::Linear => {
                    path.push_str(&format!(" L {:.1} {:.1}", x, y));
                }
                LineInterpolation::StepAfter => {
                    // Horizontal first, then vertical
                    let prev = &data[i - 1];
                    let prev_y = padding + chart_height - ((prev.value - min_val) / val_range) * chart_height;
                    path.push_str(&format!(" L {:.1} {:.1}", x, prev_y));
                    path.push_str(&format!(" L {:.1} {:.1}", x, y));
                }
                LineInterpolation::StepBefore => {
                    // Vertical first, then horizontal
                    let prev = &data[i - 1];
                    let prev_x = padding + ((prev.timestamp - min_time) as f64 / time_range) * chart_width;
                    path.push_str(&format!(" L {:.1} {:.1}", prev_x, y));
                    path.push_str(&format!(" L {:.1} {:.1}", x, y));
                }
            }
        }
    }

    path
}

/// Format timestamp to time string.
fn format_time(timestamp: i64) -> String {
    // Simple hour:minute format (assumes Unix milliseconds)
    let total_minutes = (timestamp / 60000) % (24 * 60);
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    format!("{:02}:{:02}", hours, minutes)
}

/// MultiSeriesLineChart component.
///
/// A time series chart supporting multiple series with step-line or smooth interpolation.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{
///     MultiSeriesLineChart, ChartSeries, ChartDataPoint, ChartSeriesColor,
///     MultiSeriesChartConfig, LineInterpolation,
/// };
///
/// let series = vec![
///     ChartSeries::new("Entrance", data1, ChartSeriesColor::Green),
///     ChartSeries::new("Large Room", data2, ChartSeriesColor::Yellow),
/// ];
///
/// view! {
///     <MultiSeriesLineChart
///         title="Humidity".to_string()
///         series=Signal::derive(move || series.clone())
///         config=MultiSeriesChartConfig {
///             interpolation: LineInterpolation::StepAfter,
///             y_unit: "%H".to_string(),
///             ..Default::default()
///         }
///     />
/// }
/// ```
#[component]
pub fn MultiSeriesLineChart(
    /// Chart title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Chart series.
    #[prop(into)]
    series: Signal<Vec<ChartSeries>>,
    /// Configuration.
    #[prop(optional)]
    config: Option<MultiSeriesChartConfig>,
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
    let interpolation = config.interpolation;
    let y_unit = config.y_unit.clone();
    let show_grid = config.show_grid;
    let stroke_width = config.stroke_width;
    let y_ticks = config.y_ticks;
    let x_labels = config.x_labels;

    let prefix = format!("fx-multi-line-{}", design_system);

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

    // Chart dimensions
    let svg_width = 800.0;
    let svg_height = height as f64;
    let padding_left = 60.0;
    let padding_right = 20.0;
    let padding_top = 20.0;
    let padding_bottom = 40.0;
    let chart_width = svg_width - padding_left - padding_right;
    let chart_height = svg_height - padding_top - padding_bottom;

    // Calculate data bounds
    let bounds = move || {
        let all_series = series.get();
        if all_series.is_empty() {
            return (0i64, 1i64, 0.0, 1.0);
        }

        let mut min_time = i64::MAX;
        let mut max_time = i64::MIN;
        let mut min_val = f64::INFINITY;
        let mut max_val = f64::NEG_INFINITY;

        for s in &all_series {
            for point in &s.data {
                min_time = min_time.min(point.timestamp);
                max_time = max_time.max(point.timestamp);
                min_val = min_val.min(point.value);
                max_val = max_val.max(point.value);
            }
        }

        // Add some padding to value range
        let val_padding = (max_val - min_val) * 0.1;
        min_val -= val_padding;
        max_val += val_padding;

        (min_time, max_time, min_val, max_val)
    };

    // Generate Y-axis ticks
    let y_axis_ticks = move || {
        let (_, _, min_val, max_val) = bounds();
        let range = max_val - min_val;
        let step = range / (y_ticks - 1) as f64;

        (0..y_ticks)
            .map(|i| {
                let val = max_val - i as f64 * step;
                let y = padding_top + (i as f64 / (y_ticks - 1) as f64) * chart_height;
                (y, val)
            })
            .collect::<Vec<_>>()
    };

    // Generate X-axis labels
    let x_axis_labels = move || {
        let (min_time, max_time, _, _) = bounds();
        let time_range = max_time - min_time;
        let step = time_range / (x_labels - 1) as i64;

        (0..x_labels)
            .map(|i| {
                let time = min_time + i as i64 * step;
                let x = padding_left + (i as f64 / (x_labels - 1) as f64) * chart_width;
                (x, format_time(time))
            })
            .collect::<Vec<_>>()
    };

    // Generate series paths
    let series_paths = move || {
        let (min_time, max_time, min_val, max_val) = bounds();
        let all_series = series.get();

        all_series
            .iter()
            .map(|s| {
                let path = generate_path(
                    &s.data,
                    min_time,
                    max_time,
                    min_val,
                    max_val,
                    svg_width,
                    svg_height - padding_bottom + padding_top,
                    padding_left,
                    interpolation,
                );
                (s.name.clone(), s.color.as_css().to_string(), path)
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div
            class=combined_class
            style="background: var(--fx-color-bg-container, #181b1f); border-radius: 8px; padding: 16px;"
        >
            // Title
            {title.clone().map(|t| {
                view! {
                    <div style="color: var(--fx-color-text, #fff); font-size: 16px; font-weight: 500; margin-bottom: 12px;">
                        {t}
                    </div>
                }
            })}

            // Chart SVG
            <svg
                width="100%"
                height=format!("{}", height)
                viewBox=format!("0 0 {} {}", svg_width, svg_height)
                preserveAspectRatio="xMidYMid meet"
            >
                // Grid lines
                {move || show_grid.then(|| {
                    y_axis_ticks().into_iter().map(|(y, _)| {
                        view! {
                            <line
                                x1=format!("{}", padding_left)
                                y1=format!("{:.1}", y)
                                x2=format!("{}", svg_width - padding_right)
                                y2=format!("{:.1}", y)
                                stroke="#333"
                                stroke-width="1"
                                stroke-dasharray="2,2"
                            />
                        }
                    }).collect_view()
                })}

                // Y-axis labels
                {move || {
                    let unit = y_unit.clone();
                    y_axis_ticks().into_iter().map(|(y, val)| {
                        view! {
                            <text
                                x=format!("{}", padding_left - 8.0)
                                y=format!("{:.1}", y + 4.0)
                                fill="#8c8c8c"
                                font-size="11"
                                text-anchor="end"
                            >
                                {format!("{:.0} {}", val, unit)}
                            </text>
                        }
                    }).collect_view()
                }}

                // X-axis labels
                {move || {
                    x_axis_labels().into_iter().map(|(x, label)| {
                        view! {
                            <text
                                x=format!("{:.1}", x)
                                y=format!("{}", svg_height - 10.0)
                                fill="#8c8c8c"
                                font-size="11"
                                text-anchor="middle"
                            >
                                {label}
                            </text>
                        }
                    }).collect_view()
                }}

                // Series lines
                {move || {
                    series_paths().into_iter().map(|(_, color, path)| {
                        view! {
                            <path
                                d=path
                                fill="none"
                                stroke=color
                                stroke-width=format!("{}", stroke_width)
                                stroke-linejoin="round"
                            />
                        }
                    }).collect_view()
                }}
            </svg>

            // Bottom legend
            <div style="display: flex; gap: 24px; margin-top: 12px; flex-wrap: wrap;">
                {move || {
                    series.get().into_iter().map(|s| {
                        let color = s.color.as_css();
                        view! {
                            <div style="display: flex; align-items: center; gap: 6px;">
                                <div style=format!(
                                    "width: 16px; height: 3px; background: {}; border-radius: 1px;",
                                    color
                                )></div>
                                <span style="color: #8c8c8c; font-size: 12px;">
                                    {s.name}
                                </span>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}
