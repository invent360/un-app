//! ResourceTimingsChart Leptos component.
//!
//! A stacked bar chart for displaying timing breakdowns (e.g., network timing metrics).

use leptos::prelude::*;
use crate::try_use_theme;

/// Timing category with color and label.
#[derive(Debug, Clone)]
pub struct TimingCategory {
    /// Category name (e.g., "DNS lookup time").
    pub name: String,
    /// CSS color for this category.
    pub color: String,
}

impl TimingCategory {
    /// Create a new timing category.
    pub fn new(name: impl Into<String>, color: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            color: color.into(),
        }
    }
}

/// A single timing data point with values for each category.
#[derive(Debug, Clone)]
pub struct TimingDataPoint {
    /// Timestamp or label for X-axis.
    pub timestamp: i64,
    /// Values for each category (in same order as categories).
    pub values: Vec<f64>,
}

impl TimingDataPoint {
    /// Create a new timing data point.
    pub fn new(timestamp: i64, values: Vec<f64>) -> Self {
        Self { timestamp, values }
    }
}

/// Preset timing categories for web resource timing.
pub mod presets {
    use super::TimingCategory;

    /// Web resource timing categories.
    pub fn web_resource_timing() -> Vec<TimingCategory> {
        vec![
            TimingCategory::new("Redirect time", "#8ecae6"),
            TimingCategory::new("Service worker time", "#5b9bd5"),
            TimingCategory::new("DNS lookup time", "#70c1b3"),
            TimingCategory::new("TCP handshake time", "#b9a6d8"),
            TimingCategory::new("Request time", "#f4a261"),
            TimingCategory::new("Response time", "#e9c46a"),
            TimingCategory::new("TTFB", "#9b8fc4"),
        ]
    }
}

/// Configuration for the resource timings chart.
#[derive(Debug, Clone)]
pub struct ResourceTimingsConfig {
    /// Chart height.
    pub height: u32,
    /// Bar width as percentage of available space.
    pub bar_width_ratio: f64,
    /// Show grid lines.
    pub show_grid: bool,
    /// Y-axis unit (e.g., "s", "ms").
    pub y_unit: String,
}

impl Default for ResourceTimingsConfig {
    fn default() -> Self {
        Self {
            height: 200,
            bar_width_ratio: 0.6,
            show_grid: true,
            y_unit: "s".to_string(),
        }
    }
}

/// ResourceTimingsChart component.
///
/// A stacked bar chart showing timing breakdowns over time.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{
///     ResourceTimingsChart, TimingCategory, TimingDataPoint,
///     resource_timings_chart::presets,
/// };
///
/// let categories = presets::web_resource_timing();
/// let data = vec![
///     TimingDataPoint::new(1716000000000, vec![0.1, 0.2, 0.05, 0.15, 0.3, 0.2, 0.5]),
///     TimingDataPoint::new(1716000060000, vec![0.1, 0.3, 0.05, 0.1, 0.25, 0.15, 0.4]),
/// ];
///
/// view! {
///     <ResourceTimingsChart
///         categories=categories
///         data=Signal::derive(move || data.clone())
///         title="Resource timings".to_string()
///     />
/// }
/// ```
#[component]
pub fn ResourceTimingsChart(
    /// Timing categories (defines colors and legend).
    #[prop(into)]
    categories: Vec<TimingCategory>,
    /// Data points.
    #[prop(into)]
    data: Signal<Vec<TimingDataPoint>>,
    /// Chart title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Configuration.
    #[prop(optional)]
    config: Option<ResourceTimingsConfig>,
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
    let bar_width_ratio = config.bar_width_ratio;
    let show_grid = config.show_grid;
    let y_unit = config.y_unit;

    let prefix = format!("fx-resource-timings-{}", design_system);
    let categories_clone = categories.clone();

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
    let padding_bottom: f64 = 40.0;

    let inner_width = svg_width - padding_left - padding_right;
    let inner_height = svg_height - padding_top - padding_bottom;

    // Calculate bounds
    let bounds = move || {
        let points = data.get();
        if points.is_empty() {
            return (0.0f64, 6.0f64);
        }

        let mut max_stacked = 0.0f64;
        for point in &points {
            let total: f64 = point.values.iter().sum();
            max_stacked = max_stacked.max(total);
        }

        // Round up to nice number
        let nice_max = (max_stacked * 1.1).ceil();
        (0.0, nice_max.max(1.0))
    };

    // Generate bar data
    let bars = {
        let categories = categories.clone();
        move || {
            let points = data.get();
            let num_bars = points.len();
            if num_bars == 0 {
                return Vec::new();
            }

            let (min_val, max_val) = bounds();
            let val_range = max_val - min_val;

            let bar_spacing = inner_width / num_bars as f64;
            let bar_width = bar_spacing * bar_width_ratio;
            let bar_offset = (bar_spacing - bar_width) / 2.0;

            let mut result = Vec::new();

            for (i, point) in points.iter().enumerate() {
                let bar_x = padding_left + i as f64 * bar_spacing + bar_offset;
                let mut y_bottom = padding_top + inner_height;

                // Build segments for this bar (bottom to top)
                let mut segments = Vec::new();
                for (j, &value) in point.values.iter().enumerate() {
                    if value <= 0.0 {
                        continue;
                    }
                    let segment_height = (value / val_range) * inner_height;
                    let segment_y = y_bottom - segment_height;

                    let color = categories.get(j)
                        .map(|c| c.color.clone())
                        .unwrap_or_else(|| "#888".to_string());

                    segments.push((segment_y, segment_height, color));
                    y_bottom = segment_y;
                }

                // Format time label
                let total_seconds = point.timestamp / 1000;
                let hours = (total_seconds / 3600) % 24;
                let minutes = (total_seconds / 60) % 60;
                let time_label = format!("{:02}:{:02}", hours, minutes);

                result.push((bar_x, bar_width, segments, time_label));
            }

            result
        }
    };

    // Grid lines
    let grid_lines = move || {
        let (min_val, max_val) = bounds();
        let val_range = max_val - min_val;

        let mut lines = Vec::new();
        let num_lines = 6;

        for i in 0..=num_lines {
            let y = padding_top + (i as f64 / num_lines as f64) * inner_height;
            let val = max_val - (i as f64 / num_lines as f64) * val_range;
            let label = if val >= 1.0 {
                format!("{:.0} {}", val, y_unit)
            } else if val > 0.0 {
                format!("{:.0} ms", val * 1000.0)
            } else {
                format!("0 ms")
            };
            lines.push((y, label));
        }

        lines
    };

    view! {
        <div class=combined_class style="width: 100%;">
            // Title
            {title.clone().map(|t| {
                let prefix = prefix.clone();
                view! {
                    <div class=format!("{}-title", prefix) style="font-size: 16px; font-weight: 500; margin-bottom: 12px; color: var(--fx-color-text, #fff);">
                        {t}
                    </div>
                }
            })}

            // Chart SVG
            <svg
                class=format!("{}-chart", prefix)
                viewBox=format!("0 0 {} {}", svg_width, svg_height)
                preserveAspectRatio="xMidYMid meet"
                style="width: 100%; height: auto;"
            >
                // Grid lines
                {move || {
                    if show_grid {
                        Some(grid_lines().into_iter().map(|(y, label)| {
                            view! {
                                <g>
                                    <line
                                        x1=format!("{:.1}", padding_left)
                                        y1=format!("{:.1}", y)
                                        x2=format!("{:.1}", padding_left + inner_width)
                                        y2=format!("{:.1}", y)
                                        stroke="var(--fx-color-border, #303030)"
                                        stroke-width="1"
                                        stroke-dasharray="2,2"
                                    />
                                    <text
                                        x=format!("{:.1}", padding_left - 8.0)
                                        y=format!("{:.1}", y)
                                        fill="var(--fx-color-text-tertiary, #6b7280)"
                                        font-size="11"
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

                // Stacked bars
                {move || {
                    bars().into_iter().map(|(bar_x, bar_width, segments, time_label)| {
                        view! {
                            <g>
                                // Bar segments
                                {segments.into_iter().map(|(y, h, color)| {
                                    view! {
                                        <rect
                                            x=format!("{:.1}", bar_x)
                                            y=format!("{:.1}", y)
                                            width=format!("{:.1}", bar_width)
                                            height=format!("{:.1}", h)
                                            fill=color
                                        />
                                    }
                                }).collect_view()}
                                // Time label
                                <text
                                    x=format!("{:.1}", bar_x + bar_width / 2.0)
                                    y=format!("{:.1}", padding_top + inner_height + 20.0)
                                    fill="var(--fx-color-text-tertiary, #6b7280)"
                                    font-size="11"
                                    text-anchor="middle"
                                >
                                    {time_label}
                                </text>
                            </g>
                        }
                    }).collect_view()
                }}
            </svg>

            // Legend
            <div class=format!("{}-legend", prefix) style="display: flex; flex-wrap: wrap; gap: 16px; margin-top: 12px;">
                {categories_clone.iter().map(|cat| {
                    view! {
                        <div style="display: flex; align-items: center; gap: 6px;">
                            <span style=format!("width: 12px; height: 3px; background: {}; border-radius: 1px;", cat.color) />
                            <span style="font-size: 12px; color: var(--fx-color-text-secondary, #8c8c8c);">
                                {cat.name.clone()}
                            </span>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
