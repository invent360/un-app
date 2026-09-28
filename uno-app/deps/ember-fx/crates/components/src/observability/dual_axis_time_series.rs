//! DualAxisTimeSeriesChart Leptos component.
//!
//! A time series chart with two Y-axes for different metric types.

use leptos::prelude::*;
use crate::try_use_theme;

/// Which Y-axis a series or threshold belongs to.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ChartAxis {
    #[default]
    Left,
    Right,
}

/// Line style for series or thresholds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DualAxisLineStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

impl DualAxisLineStyle {
    /// Returns the SVG stroke-dasharray value.
    pub fn as_dasharray(&self) -> &'static str {
        match self {
            Self::Solid => "none",
            Self::Dashed => "8,4",
            Self::Dotted => "2,2",
        }
    }
}

/// Color options for dual axis series.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DualAxisColor {
    #[default]
    Green,
    Red,
    Blue,
    Yellow,
    Orange,
    Cyan,
    Magenta,
    Gray,
}

impl DualAxisColor {
    /// Returns the CSS color string.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Green => "#52c41a",
            Self::Red => "#f5222d",
            Self::Blue => "#1890ff",
            Self::Yellow => "#fadb14",
            Self::Orange => "#fa8c16",
            Self::Cyan => "#13c2c2",
            Self::Magenta => "#eb2f96",
            Self::Gray => "#8c8c8c",
        }
    }
}

/// A data point in a dual axis series.
#[derive(Debug, Clone)]
pub struct DualAxisDataPoint {
    /// Timestamp in Unix milliseconds.
    pub timestamp: i64,
    /// Value at this timestamp.
    pub value: f64,
}

impl DualAxisDataPoint {
    /// Create a new data point.
    pub fn new(timestamp: i64, value: f64) -> Self {
        Self { timestamp, value }
    }
}

/// A series in the dual axis chart.
#[derive(Debug, Clone)]
pub struct DualAxisSeries {
    /// Series name.
    pub name: String,
    /// Data points.
    pub data: Vec<DualAxisDataPoint>,
    /// Which axis this series belongs to.
    pub axis: ChartAxis,
    /// Series color.
    pub color: DualAxisColor,
    /// Line style.
    pub line_style: DualAxisLineStyle,
}

impl DualAxisSeries {
    /// Create a new series.
    pub fn new(
        name: impl Into<String>,
        data: Vec<DualAxisDataPoint>,
        axis: ChartAxis,
        color: DualAxisColor,
    ) -> Self {
        Self {
            name: name.into(),
            data,
            axis,
            color,
            line_style: DualAxisLineStyle::Solid,
        }
    }

    /// Create a series with a specific line style.
    pub fn with_style(
        name: impl Into<String>,
        data: Vec<DualAxisDataPoint>,
        axis: ChartAxis,
        color: DualAxisColor,
        line_style: DualAxisLineStyle,
    ) -> Self {
        Self {
            name: name.into(),
            data,
            axis,
            color,
            line_style,
        }
    }
}

/// A horizontal threshold line.
#[derive(Debug, Clone)]
pub struct ThresholdLine {
    /// Value on the axis.
    pub value: f64,
    /// Which axis this threshold belongs to.
    pub axis: ChartAxis,
    /// Line color (CSS string).
    pub color: String,
    /// Optional label.
    pub label: Option<String>,
    /// Line style.
    pub style: DualAxisLineStyle,
}

impl ThresholdLine {
    /// Create a new threshold line.
    pub fn new(value: f64, axis: ChartAxis, color: impl Into<String>) -> Self {
        Self {
            value,
            axis,
            color: color.into(),
            label: None,
            style: DualAxisLineStyle::Dotted,
        }
    }

    /// Create a threshold with a label.
    pub fn with_label(
        value: f64,
        axis: ChartAxis,
        color: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            value,
            axis,
            color: color.into(),
            label: Some(label.into()),
            style: DualAxisLineStyle::Dotted,
        }
    }
}

/// Configuration for the dual axis chart.
#[derive(Debug, Clone)]
pub struct DualAxisConfig {
    /// Chart height.
    pub height: u32,
    /// Left Y-axis unit.
    pub left_unit: Option<String>,
    /// Right Y-axis unit.
    pub right_unit: Option<String>,
    /// Show grid lines.
    pub show_grid: bool,
    /// Threshold lines.
    pub thresholds: Vec<ThresholdLine>,
}

impl Default for DualAxisConfig {
    fn default() -> Self {
        Self {
            height: 200,
            left_unit: None,
            right_unit: None,
            show_grid: true,
            thresholds: Vec::new(),
        }
    }
}

/// DualAxisTimeSeriesChart component.
///
/// A time series chart with two Y-axes for different metric types.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{
///     DualAxisTimeSeriesChart, DualAxisSeries, DualAxisDataPoint,
///     ChartAxis, DualAxisColor, ThresholdLine,
/// };
///
/// let series = vec![
///     DualAxisSeries::new("duration", duration_data, ChartAxis::Left, DualAxisColor::Green),
///     DualAxisSeries::new("status", status_data, ChartAxis::Right, DualAxisColor::Blue),
/// ];
///
/// let config = DualAxisConfig {
///     left_unit: Some("ms".to_string()),
///     right_unit: Some("code".to_string()),
///     thresholds: vec![
///         ThresholdLine::with_label(200.0, ChartAxis::Right, "#52c41a", "200 OK"),
///         ThresholdLine::with_label(400.0, ChartAxis::Right, "#f5222d", "400 Error"),
///     ],
///     ..Default::default()
/// };
///
/// view! {
///     <DualAxisTimeSeriesChart
///         series=Signal::derive(move || series.clone())
///         title="HTTP Probe".to_string()
///         config=config
///     />
/// }
/// ```
#[component]
pub fn DualAxisTimeSeriesChart(
    /// Data series.
    #[prop(into)]
    series: Signal<Vec<DualAxisSeries>>,
    /// Chart title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Configuration.
    #[prop(optional)]
    config: Option<DualAxisConfig>,
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
    let left_unit = config.left_unit.clone();
    let right_unit = config.right_unit.clone();
    let show_grid = config.show_grid;
    let thresholds = config.thresholds.clone();

    let prefix = format!("fx-dual-axis-{}", design_system);

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
    let svg_width: f64 = 700.0;
    let svg_height: f64 = height as f64;
    let padding_left: f64 = 60.0;
    let padding_right: f64 = 60.0;
    let padding_top: f64 = 10.0;
    let padding_bottom: f64 = 30.0;

    let inner_width = svg_width - padding_left - padding_right;
    let inner_height = svg_height - padding_top - padding_bottom;

    // Calculate bounds for each axis (using Memo for reactive caching)
    let bounds = {
        let thresholds = thresholds.clone();
        Memo::new(move |_| {
            let all_series = series.get();

            let mut min_time = i64::MAX;
            let mut max_time = i64::MIN;
            let mut left_min = f64::INFINITY;
            let mut left_max = f64::NEG_INFINITY;
            let mut right_min = f64::INFINITY;
            let mut right_max = f64::NEG_INFINITY;

            for s in &all_series {
                for p in &s.data {
                    min_time = min_time.min(p.timestamp);
                    max_time = max_time.max(p.timestamp);

                    match s.axis {
                        ChartAxis::Left => {
                            left_min = left_min.min(p.value);
                            left_max = left_max.max(p.value);
                        }
                        ChartAxis::Right => {
                            right_min = right_min.min(p.value);
                            right_max = right_max.max(p.value);
                        }
                    }
                }
            }

            // Include thresholds in bounds
            for t in &thresholds {
                match t.axis {
                    ChartAxis::Left => {
                        left_min = left_min.min(t.value);
                        left_max = left_max.max(t.value);
                    }
                    ChartAxis::Right => {
                        right_min = right_min.min(t.value);
                        right_max = right_max.max(t.value);
                    }
                }
            }

            // Default ranges if no data
            if left_min == f64::INFINITY {
                left_min = 0.0;
                left_max = 100.0;
            }
            if right_min == f64::INFINITY {
                right_min = 0.0;
                right_max = 100.0;
            }

            // Add padding
            let left_range = left_max - left_min;
            let right_range = right_max - right_min;
            left_max += left_range * 0.1;
            right_max += right_range * 0.1;

            (
                min_time,
                max_time,
                left_min.min(0.0),
                left_max,
                right_min.min(0.0),
                right_max,
            )
        })
    };

    // Generate line paths for each series
    let series_paths = move || {
        let all_series = series.get();
        let (min_time, max_time, left_min, left_max, right_min, right_max) = bounds.get();
        let time_range = (max_time - min_time).max(1) as f64;
        let left_range = (left_max - left_min).max(0.001);
        let right_range = (right_max - right_min).max(0.001);

        all_series
            .iter()
            .map(|s| {
                let mut path = String::new();

                let (min_val, val_range) = match s.axis {
                    ChartAxis::Left => (left_min, left_range),
                    ChartAxis::Right => (right_min, right_range),
                };

                for (i, p) in s.data.iter().enumerate() {
                    let x = padding_left + ((p.timestamp - min_time) as f64 / time_range) * inner_width;
                    let y = padding_top + inner_height - ((p.value - min_val) / val_range) * inner_height;

                    if i == 0 {
                        path.push_str(&format!("M {:.1} {:.1}", x, y));
                    } else {
                        path.push_str(&format!(" L {:.1} {:.1}", x, y));
                    }
                }

                (s.name.clone(), s.color, s.line_style, path)
            })
            .collect::<Vec<_>>()
    };

    // Generate threshold lines
    let threshold_paths = {
        let thresholds = thresholds.clone();
        move || {
            let (_, _, left_min, left_max, right_min, right_max) = bounds.get();
            let left_range = (left_max - left_min).max(0.001);
            let right_range = (right_max - right_min).max(0.001);

            thresholds
                .iter()
                .map(|t| {
                    let (min_val, val_range) = match t.axis {
                        ChartAxis::Left => (left_min, left_range),
                        ChartAxis::Right => (right_min, right_range),
                    };

                    let y = padding_top + inner_height - ((t.value - min_val) / val_range) * inner_height;
                    let path = format!(
                        "M {:.1} {:.1} L {:.1} {:.1}",
                        padding_left, y, padding_left + inner_width, y
                    );

                    let label_x = match t.axis {
                        ChartAxis::Left => padding_left + inner_width + 5.0,
                        ChartAxis::Right => padding_left + inner_width + 5.0,
                    };

                    (path, t.color.clone(), t.style, t.label.clone(), label_x, y)
                })
                .collect::<Vec<_>>()
        }
    };

    // Left Y-axis labels
    let left_axis_labels = {
        let left_unit = left_unit.clone();
        move || {
            let (_, _, left_min, left_max, _, _) = bounds.get();
            let left_range = (left_max - left_min).max(0.001);

            (0..=4)
                .map(|i| {
                    let y = padding_top + (i as f64 / 4.0) * inner_height;
                    let val = left_max - (i as f64 / 4.0) * left_range;
                    let label = if let Some(ref unit) = left_unit {
                        format!("{:.0}{}", val, unit)
                    } else {
                        format!("{:.0}", val)
                    };
                    (padding_left - 5.0, y, label)
                })
                .collect::<Vec<_>>()
        }
    };

    // Right Y-axis labels
    let right_axis_labels = {
        let right_unit = right_unit.clone();
        move || {
            let (_, _, _, _, right_min, right_max) = bounds.get();
            let right_range = (right_max - right_min).max(0.001);

            (0..=4)
                .map(|i| {
                    let y = padding_top + (i as f64 / 4.0) * inner_height;
                    let val = right_max - (i as f64 / 4.0) * right_range;
                    let label = if let Some(ref unit) = right_unit {
                        format!("{:.0}{}", val, unit)
                    } else {
                        format!("{:.0}", val)
                    };
                    (padding_left + inner_width + 5.0, y, label)
                })
                .collect::<Vec<_>>()
        }
    };

    // Grid lines
    let grid_lines = move || {
        (0..=4)
            .map(|i| {
                let y = padding_top + (i as f64 / 4.0) * inner_height;
                format!(
                    "M {:.1} {:.1} L {:.1} {:.1}",
                    padding_left, y, padding_left + inner_width, y
                )
            })
            .collect::<Vec<_>>()
    };

    // Time labels
    let time_labels = move || {
        let (min_time, max_time, _, _, _, _) = bounds.get();
        let time_range = (max_time - min_time).max(1) as f64;

        (0..6)
            .map(|i| {
                let t = min_time + ((i as f64 / 5.0) * time_range) as i64;
                let x = padding_left + (i as f64 / 5.0) * inner_width;

                let total_seconds = t / 1000;
                let hours = (total_seconds / 3600) % 24;
                let minutes = (total_seconds / 60) % 60;
                let label = format!("{:02}:{:02}", hours, minutes);

                (x, label)
            })
            .collect::<Vec<_>>()
    };

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
                        Some(grid_lines().into_iter().map(|path| {
                            view! {
                                <path
                                    d=path
                                    fill="none"
                                    stroke="var(--fx-color-border, #303030)"
                                    stroke-width="1"
                                />
                            }
                        }).collect_view())
                    } else {
                        None
                    }
                }}

                // Left Y-axis labels
                {move || {
                    left_axis_labels().into_iter().map(|(x, y, label)| {
                        view! {
                            <text
                                x=format!("{:.1}", x)
                                y=format!("{:.1}", y)
                                fill="var(--fx-color-text-tertiary, #6b7280)"
                                font-size="10"
                                text-anchor="end"
                                dominant-baseline="middle"
                            >
                                {label}
                            </text>
                        }
                    }).collect_view()
                }}

                // Right Y-axis labels
                {move || {
                    right_axis_labels().into_iter().map(|(x, y, label)| {
                        view! {
                            <text
                                x=format!("{:.1}", x)
                                y=format!("{:.1}", y)
                                fill="var(--fx-color-text-tertiary, #6b7280)"
                                font-size="10"
                                text-anchor="start"
                                dominant-baseline="middle"
                            >
                                {label}
                            </text>
                        }
                    }).collect_view()
                }}

                // Time labels
                {move || {
                    time_labels().into_iter().map(|(x, label)| {
                        view! {
                            <text
                                x=format!("{:.1}", x)
                                y=format!("{:.1}", svg_height - 5.0)
                                fill="var(--fx-color-text-tertiary, #6b7280)"
                                font-size="10"
                                text-anchor="middle"
                            >
                                {label}
                            </text>
                        }
                    }).collect_view()
                }}

                // Threshold lines
                {move || {
                    threshold_paths().into_iter().map(|(path, color, style, label, label_x, y)| {
                        view! {
                            <g>
                                <path
                                    d=path
                                    fill="none"
                                    stroke=color.clone()
                                    stroke-width="1.5"
                                    stroke-dasharray=style.as_dasharray()
                                />
                                {label.map(|l| {
                                    view! {
                                        <text
                                            x=format!("{:.1}", label_x)
                                            y=format!("{:.1}", y)
                                            fill=color
                                            font-size="10"
                                            text-anchor="start"
                                            dominant-baseline="middle"
                                        >
                                            {l}
                                        </text>
                                    }
                                })}
                            </g>
                        }
                    }).collect_view()
                }}

                // Series lines
                {move || {
                    series_paths().into_iter().map(|(_name, color, line_style, path)| {
                        view! {
                            <path
                                d=path
                                fill="none"
                                stroke=color.as_css()
                                stroke-width="2"
                                stroke-dasharray=line_style.as_dasharray()
                            />
                        }
                    }).collect_view()
                }}
            </svg>
        </div>
    }
}
