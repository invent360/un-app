//! LatencyStepChart Leptos component.
//!
//! A step/area chart with scatter overlay for latency distribution.

use leptos::prelude::*;
use crate::try_use_theme;

/// A latency data point.
#[derive(Debug, Clone)]
pub struct LatencyPoint {
    /// Timestamp in Unix milliseconds.
    pub timestamp: i64,
    /// Latency value in milliseconds.
    pub value: f64,
}

impl LatencyPoint {
    /// Create a new latency point.
    pub fn new(timestamp: i64, value: f64) -> Self {
        Self { timestamp, value }
    }
}

/// Configuration for the latency chart.
#[derive(Debug, Clone)]
pub struct LatencyChartConfig {
    /// Chart height.
    pub height: u32,
    /// Show scatter points overlay.
    pub show_scatter: bool,
    /// Fill opacity.
    pub fill_opacity: f64,
    /// Show grid.
    pub show_grid: bool,
    /// Line color (CSS).
    pub color: String,
    /// Scatter point radius.
    pub scatter_radius: f64,
}

impl Default for LatencyChartConfig {
    fn default() -> Self {
        Self {
            height: 150,
            show_scatter: true,
            fill_opacity: 0.3,
            show_grid: true,
            color: "#b7eb8f".to_string(),
            scatter_radius: 2.0,
        }
    }
}

/// Format milliseconds for display.
fn format_ms(value: f64) -> String {
    if value >= 1000.0 {
        format!("{:.1}s", value / 1000.0)
    } else {
        format!("{:.0}ms", value)
    }
}

/// LatencyStepChart component.
///
/// A step chart with scatter overlay for latency percentiles.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{LatencyStepChart, LatencyPoint};
///
/// let data = vec![
///     LatencyPoint::new(1000, 50.0),
///     LatencyPoint::new(2000, 75.0),
///     LatencyPoint::new(3000, 200.0),
/// ];
///
/// view! {
///     <LatencyStepChart
///         data=Signal::derive(move || data.clone())
///         title="Latency (Grafana Metrics)".to_string()
///     />
/// }
/// ```
#[component]
pub fn LatencyStepChart(
    /// Latency data points.
    #[prop(into)]
    data: Signal<Vec<LatencyPoint>>,
    /// Chart title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Chart configuration.
    #[prop(optional)]
    config: Option<LatencyChartConfig>,
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
    let show_scatter = config.show_scatter;
    let fill_opacity = config.fill_opacity;
    let show_grid = config.show_grid;
    let color = config.color.clone();
    let scatter_radius = config.scatter_radius;

    let prefix = format!("fx-latency-step-{}", design_system);

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
    let svg_width: f64 = 600.0;
    let svg_height: f64 = height as f64;
    let padding_left: f64 = 60.0;
    let padding_right: f64 = 10.0;
    let padding_top: f64 = 10.0;
    let padding_bottom: f64 = 25.0;

    let inner_width = svg_width - padding_left - padding_right;
    let inner_height = svg_height - padding_top - padding_bottom;

    // Calculate bounds
    let bounds = move || {
        let points = data.get();
        if points.is_empty() {
            return (0i64, 0i64, 0.0f64, 100.0f64);
        }

        let min_time = points.iter().map(|p| p.timestamp).min().unwrap_or(0);
        let max_time = points.iter().map(|p| p.timestamp).max().unwrap_or(0);
        let min_val = 0.0; // Always start at 0 for latency
        let max_val = points.iter().map(|p| p.value).fold(0.0f64, f64::max) * 1.1;

        (min_time, max_time, min_val, max_val)
    };

    // Generate step path (with fill)
    let step_paths = {
        let _color = color.clone();
        move || {
            let points = data.get();
            let (min_time, max_time, min_val, max_val) = bounds();
            let time_range = (max_time - min_time).max(1) as f64;
            let val_range = (max_val - min_val).max(0.001);

            if points.is_empty() {
                return (String::new(), String::new(), Vec::new());
            }

            let mut line_path = String::new();
            let mut fill_path = String::new();
            let mut scatter_points = Vec::new();

            // Start fill at bottom-left
            let first_x = padding_left + ((points[0].timestamp - min_time) as f64 / time_range) * inner_width;
            fill_path.push_str(&format!("M {:.1} {:.1}", first_x, padding_top + inner_height));

            let mut prev_x = first_x;
            let mut prev_y = padding_top + inner_height;

            for (i, p) in points.iter().enumerate() {
                let x = padding_left + ((p.timestamp - min_time) as f64 / time_range) * inner_width;
                let y = padding_top + inner_height - ((p.value - min_val) / val_range) * inner_height;

                // Add scatter point
                scatter_points.push((x, y));

                if i == 0 {
                    // Step up from baseline
                    line_path.push_str(&format!("M {:.1} {:.1}", x, padding_top + inner_height));
                    line_path.push_str(&format!(" L {:.1} {:.1}", x, y));
                    fill_path.push_str(&format!(" L {:.1} {:.1}", x, y));
                    prev_y = y;
                } else {
                    // Horizontal step to new x
                    line_path.push_str(&format!(" L {:.1} {:.1}", x, prev_y));
                    fill_path.push_str(&format!(" L {:.1} {:.1}", x, prev_y));
                    // Vertical step to new y
                    line_path.push_str(&format!(" L {:.1} {:.1}", x, y));
                    fill_path.push_str(&format!(" L {:.1} {:.1}", x, y));
                    prev_y = y;
                }

                prev_x = x;
            }

            // Close fill path
            fill_path.push_str(&format!(" L {:.1} {:.1}", prev_x, padding_top + inner_height));
            fill_path.push_str(" Z");

            (line_path, fill_path, scatter_points)
        }
    };

    // Grid lines
    let grid_lines = move || {
        let (_min_time, _max_time, min_val, max_val) = bounds();
        let val_range = (max_val - min_val).max(0.001);

        let mut lines = Vec::new();

        // Horizontal grid lines with ms labels
        for i in 0..=4 {
            let y = padding_top + (i as f64 / 4.0) * inner_height;
            let val = max_val - (i as f64 / 4.0) * val_range;
            lines.push((
                format!("M {:.1} {:.1} L {:.1} {:.1}", padding_left, y, padding_left + inner_width, y),
                format_ms(val),
                padding_left - 5.0,
                y,
            ));
        }

        lines
    };

    // Clone color for SVG use
    let line_color = color.clone();
    let fill_color = format!("rgba(183, 235, 143, {})", fill_opacity);
    let scatter_color = color.clone();

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

                // Fill area
                {move || {
                    let (_, fill_path, _) = step_paths();
                    view! {
                        <path
                            d=fill_path
                            fill=fill_color.clone()
                        />
                    }
                }}

                // Step line
                {move || {
                    let (line_path, _, _) = step_paths();
                    view! {
                        <path
                            d=line_path
                            fill="none"
                            stroke=line_color.clone()
                            stroke-width="2"
                        />
                    }
                }}

                // Scatter points
                {move || {
                    if show_scatter {
                        let (_, _, scatter_points) = step_paths();
                        Some(scatter_points.into_iter().map(|(x, y)| {
                            view! {
                                <circle
                                    cx=format!("{:.1}", x)
                                    cy=format!("{:.1}", y)
                                    r=format!("{}", scatter_radius)
                                    fill=scatter_color.clone()
                                    opacity="0.6"
                                />
                            }
                        }).collect_view())
                    } else {
                        None
                    }
                }}
            </svg>
        </div>
    }
}
