//! CompactTimeSeriesPanel Leptos component.
//!
//! A time series chart with right-side compact legend showing averages.

use leptos::prelude::*;
use crate::try_use_theme;

/// Color options for compact series.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CompactSeriesColor {
    #[default]
    Orange,
    Red,
    Blue,
    Yellow,
    Green,
    Cyan,
    Magenta,
    Gray,
}

impl CompactSeriesColor {
    /// Returns the CSS color string.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Orange => "#fa8c16",
            Self::Red => "#f5222d",
            Self::Blue => "#1890ff",
            Self::Yellow => "#fadb14",
            Self::Green => "#52c41a",
            Self::Cyan => "#13c2c2",
            Self::Magenta => "#eb2f96",
            Self::Gray => "#8c8c8c",
        }
    }
}

/// A data point in a compact series.
#[derive(Debug, Clone)]
pub struct CompactDataPoint {
    /// Timestamp in Unix milliseconds.
    pub timestamp: i64,
    /// Value at this timestamp.
    pub value: f64,
}

impl CompactDataPoint {
    /// Create a new data point.
    pub fn new(timestamp: i64, value: f64) -> Self {
        Self { timestamp, value }
    }
}

/// A series in the compact time series chart.
#[derive(Debug, Clone)]
pub struct CompactSeries {
    /// Series name.
    pub name: String,
    /// Data points.
    pub data: Vec<CompactDataPoint>,
    /// Series color.
    pub color: CompactSeriesColor,
}

impl CompactSeries {
    /// Create a new series.
    pub fn new(name: impl Into<String>, data: Vec<CompactDataPoint>, color: CompactSeriesColor) -> Self {
        Self {
            name: name.into(),
            data,
            color,
        }
    }

    /// Calculate average value.
    pub fn avg(&self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.data.iter().map(|p| p.value).sum();
        sum / self.data.len() as f64
    }
}

/// Configuration for the compact time series panel.
#[derive(Debug, Clone)]
pub struct CompactTimeSeriesConfig {
    /// Panel height.
    pub height: u32,
    /// Show grid lines.
    pub show_grid: bool,
    /// Y-axis unit (e.g., "%", "GB").
    pub y_unit: Option<String>,
    /// Legend width in pixels.
    pub legend_width: u32,
}

impl Default for CompactTimeSeriesConfig {
    fn default() -> Self {
        Self {
            height: 200,
            show_grid: true,
            y_unit: None,
            legend_width: 80,
        }
    }
}

/// CompactTimeSeriesPanel component.
///
/// A time series chart with a compact right-side legend showing average values.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{CompactTimeSeriesPanel, CompactSeries, CompactDataPoint, CompactSeriesColor};
///
/// let series = vec![
///     CompactSeries::new("cpu-1", data1, CompactSeriesColor::Orange),
///     CompactSeries::new("cpu-2", data2, CompactSeriesColor::Red),
/// ];
///
/// view! {
///     <CompactTimeSeriesPanel
///         series=Signal::derive(move || series.clone())
///         title="CPU utilization per cluster [%]".to_string()
///     />
/// }
/// ```
#[component]
pub fn CompactTimeSeriesPanel(
    /// Data series.
    #[prop(into)]
    series: Signal<Vec<CompactSeries>>,
    /// Panel title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Configuration.
    #[prop(optional)]
    config: Option<CompactTimeSeriesConfig>,
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
    let y_unit = config.y_unit.clone();
    let legend_width = config.legend_width;

    let prefix = format!("fx-compact-ts-{}", design_system);

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

    // SVG dimensions (chart area, excluding legend)
    let svg_width: f64 = 600.0;
    let svg_height: f64 = height as f64;
    let padding_left: f64 = 50.0;
    let padding_right: f64 = 10.0;
    let padding_top: f64 = 10.0;
    let padding_bottom: f64 = 25.0;

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

        // Add some padding to max
        let val_range = max_val - min_val;
        max_val += val_range * 0.1;

        (min_time, max_time, min_val.min(0.0), max_val)
    };

    // Generate line paths for each series
    let series_paths = move || {
        let all_series = series.get();
        let (min_time, max_time, min_val, max_val) = bounds();
        let time_range = (max_time - min_time).max(1) as f64;
        let val_range = (max_val - min_val).max(0.001);

        all_series
            .iter()
            .map(|s| {
                let mut path = String::new();

                for (i, p) in s.data.iter().enumerate() {
                    let x = padding_left + ((p.timestamp - min_time) as f64 / time_range) * inner_width;
                    let y = padding_top + inner_height - ((p.value - min_val) / val_range) * inner_height;

                    if i == 0 {
                        path.push_str(&format!("M {:.1} {:.1}", x, y));
                    } else {
                        path.push_str(&format!(" L {:.1} {:.1}", x, y));
                    }
                }

                (s.name.clone(), s.color, path, s.avg())
            })
            .collect::<Vec<_>>()
    };

    // Grid lines
    let grid_lines = {
        let y_unit = y_unit.clone();
        move || {
            let (min_time, max_time, min_val, max_val) = bounds();
            let _time_range = (max_time - min_time).max(1) as f64;
            let val_range = (max_val - min_val).max(0.001);

            let mut lines = Vec::new();

            // Horizontal grid lines
            for i in 0..=4 {
                let y = padding_top + (i as f64 / 4.0) * inner_height;
                let val = max_val - (i as f64 / 4.0) * val_range;
                let label = if let Some(ref unit) = y_unit {
                    format!("{:.1}{}", val, unit)
                } else {
                    format!("{:.1}", val)
                };
                lines.push((
                    format!("M {:.1} {:.1} L {:.1} {:.1}", padding_left, y, padding_left + inner_width, y),
                    label,
                    padding_left - 5.0,
                    y,
                ));
            }

            lines
        }
    };

    // Time labels
    let time_labels = move || {
        let (min_time, max_time, _min_val, _max_val) = bounds();
        let time_range = (max_time - min_time).max(1) as f64;

        let mut labels = Vec::new();
        let num_labels = 6;

        for i in 0..num_labels {
            let t = min_time + ((i as f64 / (num_labels - 1) as f64) * time_range) as i64;
            let x = padding_left + (i as f64 / (num_labels - 1) as f64) * inner_width;

            // Format time (HH:MM)
            let total_seconds = t / 1000;
            let hours = (total_seconds / 3600) % 24;
            let minutes = (total_seconds / 60) % 60;
            let label = format!("{:02}:{:02}", hours, minutes);

            labels.push((x, label));
        }

        labels
    };

    // Legend items with averages
    let legend_items = {
        let y_unit = y_unit.clone();
        move || {
            let all_series = series.get();
            all_series
                .iter()
                .map(|s| {
                    let avg = s.avg();
                    let formatted = if let Some(ref unit) = y_unit {
                        format!("{:.3}{}", avg, unit)
                    } else {
                        format!("{:.3}", avg)
                    };
                    (s.name.clone(), s.color, formatted)
                })
                .collect::<Vec<_>>()
        }
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

            // Main content (chart + legend)
            <div class=format!("{}-content", prefix) style="display: flex;">
                // Chart SVG
                <svg
                    class=format!("{}-chart", prefix)
                    viewBox=format!("0 0 {} {}", svg_width, svg_height)
                    preserveAspectRatio="xMidYMid meet"
                    style=format!("flex: 1; height: {}px;", height)
                >
                    // Grid
                    {move || {
                        if show_grid {
                            Some(grid_lines().into_iter().map(|(path, label, x, y)| {
                                view! {
                                    <g>
                                        <path
                                            d=path
                                            fill="none"
                                            stroke="var(--fx-color-border, #303030)"
                                            stroke-width="1"
                                        />
                                        <text
                                            x=x
                                            y=y
                                            fill="var(--fx-color-text-tertiary, #6b7280)"
                                            font-size="10"
                                            text-anchor="end"
                                            dominant-baseline="middle"
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

                    // Series lines
                    {move || {
                        series_paths().into_iter().map(|(_name, color, path, _avg)| {
                            view! {
                                <path
                                    d=path
                                    fill="none"
                                    stroke=color.as_css()
                                    stroke-width="1.5"
                                />
                            }
                        }).collect_view()
                    }}
                </svg>

                // Right-side legend
                {
                    let prefix = prefix.clone();
                    let prefix_inner = prefix.clone();
                    view! {
                        <div
                            class=format!("{}-legend", prefix)
                            style=format!("width: {}px; padding: 8px;", legend_width)
                        >
                            <div class=format!("{}-legend-header", prefix) style="font-size: 10px; color: var(--fx-color-text-tertiary, #6b7280); margin-bottom: 4px;">
                                "avg"
                            </div>
                            {move || {
                                legend_items().into_iter().map(|(_name, color, avg)| {
                                    let prefix = prefix_inner.clone();
                                    view! {
                                        <div class=format!("{}-legend-item", prefix) style="display: flex; align-items: center; gap: 8px; margin-bottom: 4px;">
                                            <div
                                                class=format!("{}-legend-color", prefix)
                                                style=format!("width: 16px; height: 3px; background-color: {};", color.as_css())
                                            />
                                            <span style="font-size: 12px; color: var(--fx-color-text, #fff);">
                                                {avg}
                                            </span>
                                        </div>
                                    }
                                }).collect_view()
                            }}
                        </div>
                    }
                }
            </div>
        </div>
    }
}
