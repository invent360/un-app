//! SparklineStatCard Leptos component.
//!
//! A compact sparkline chart with a large value overlay.

use leptos::prelude::*;
use crate::try_use_theme;

/// Color variants for the sparkline.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SparklineColor {
    /// Primary color (blue).
    #[default]
    Primary,
    /// Success color (green).
    Success,
    /// Warning color (yellow/orange).
    Warning,
    /// Error color (red).
    Error,
    /// Info color (cyan).
    Info,
}

impl SparklineColor {
    /// Returns the CSS color value.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Primary => "var(--fx-color-primary, #1677ff)",
            Self::Success => "var(--fx-color-success, #52c41a)",
            Self::Warning => "var(--fx-color-warning, #faad14)",
            Self::Error => "var(--fx-color-error, #ff4d4f)",
            Self::Info => "var(--fx-color-info, #13c2c2)",
        }
    }

    /// Returns a fill color with opacity.
    pub fn as_fill_css(&self) -> &'static str {
        match self {
            Self::Primary => "rgba(22, 119, 255, 0.3)",
            Self::Success => "rgba(82, 196, 26, 0.3)",
            Self::Warning => "rgba(250, 173, 20, 0.3)",
            Self::Error => "rgba(255, 77, 79, 0.3)",
            Self::Info => "rgba(19, 194, 194, 0.3)",
        }
    }
}

/// Trend direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TrendDirection {
    /// Value is increasing.
    Up,
    /// Value is stable.
    #[default]
    Stable,
    /// Value is decreasing.
    Down,
}

impl TrendDirection {
    /// Determine trend from first and last values.
    pub fn from_values(first: f64, last: f64) -> Self {
        let threshold = 0.01; // 1% threshold
        let change = (last - first) / first.abs().max(1.0);
        if change > threshold {
            Self::Up
        } else if change < -threshold {
            Self::Down
        } else {
            Self::Stable
        }
    }

    /// Returns the color for this trend.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Up => "var(--fx-color-success, #52c41a)",
            Self::Stable => "var(--fx-color-text-secondary, #8c8c8c)",
            Self::Down => "var(--fx-color-error, #ff4d4f)",
        }
    }
}

/// SparklineStatCard component.
///
/// A compact card showing a sparkline chart with a large value overlay.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::SparklineStatCard;
///
/// let data = vec![10.0, 15.0, 12.0, 18.0, 25.0, 22.0, 30.0];
///
/// view! {
///     <SparklineStatCard
///         data=Signal::derive(move || data.clone())
///         value=Signal::derive(move || 895.0)
///         title="Current User Requests".to_string()
///     />
/// }
/// ```
#[component]
pub fn SparklineStatCard(
    /// Sparkline data points.
    #[prop(into)]
    data: Signal<Vec<f64>>,
    /// Current value to display.
    #[prop(into)]
    value: Signal<f64>,
    /// Card title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Value format string (uses {:.0} style).
    #[prop(optional)]
    decimals: Option<u32>,
    /// Value prefix (e.g., "$").
    #[prop(optional, into)]
    prefix: Option<String>,
    /// Value suffix (e.g., "ms").
    #[prop(optional, into)]
    suffix: Option<String>,
    /// Sparkline color.
    #[prop(optional)]
    color: SparklineColor,
    /// Show trend indicator.
    #[prop(optional)]
    show_trend: Option<bool>,
    /// Show axis (baseline and min/max labels).
    #[prop(optional)]
    show_axis: Option<bool>,
    /// Card width.
    #[prop(optional)]
    width: Option<u32>,
    /// Card height.
    #[prop(optional)]
    height: Option<u32>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let decimals = decimals.unwrap_or(0);
    let show_trend = show_trend.unwrap_or(true);
    let show_axis = show_axis.unwrap_or(false);
    let width = width.unwrap_or(300);
    let height = height.unwrap_or(120);
    let prefix = prefix.unwrap_or_default();
    let suffix = suffix.unwrap_or_default();

    let prefix_clone = prefix.clone();
    let suffix_clone = suffix.clone();

    let css_prefix = format!("fx-sparkline-stat-{}", design_system);

    let combined_class = {
        let css_prefix = css_prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![css_prefix.clone()];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // SVG dimensions
    let svg_width: f64 = width as f64;
    let svg_height: f64 = height as f64 * 0.6; // Sparkline takes 60% of height
    let padding: f64 = 4.0;
    let axis_padding_left: f64 = if show_axis { 45.0 } else { 0.0 };
    let axis_padding_bottom: f64 = if show_axis { 20.0 } else { 0.0 };

    // Calculate chart area dimensions
    let chart_left = padding + axis_padding_left;
    let chart_right = svg_width - padding;
    let chart_top = padding;
    let chart_bottom = svg_height - padding - axis_padding_bottom;
    let chart_width = chart_right - chart_left;
    let chart_height = chart_bottom - chart_top;

    // Generate sparkline path
    let sparkline_path = move || {
        let points = data.get();
        if points.is_empty() {
            return String::new();
        }

        let min_val = points.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = points.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let val_range = (max_val - min_val).max(0.001);

        let x_step = chart_width / (points.len() as f64 - 1.0).max(1.0);

        let mut path = String::new();
        for (i, &val) in points.iter().enumerate() {
            let x = chart_left + i as f64 * x_step;
            let y = chart_top + chart_height - ((val - min_val) / val_range) * chart_height;
            if i == 0 {
                path.push_str(&format!("M {:.1} {:.1}", x, y));
            } else {
                path.push_str(&format!(" L {:.1} {:.1}", x, y));
            }
        }
        path
    };

    // Generate fill path (closed area)
    let fill_path = move || {
        let points = data.get();
        if points.is_empty() {
            return String::new();
        }

        let min_val = points.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = points.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let val_range = (max_val - min_val).max(0.001);

        let x_step = chart_width / (points.len() as f64 - 1.0).max(1.0);

        let mut path = String::new();
        // Start at bottom-left
        path.push_str(&format!("M {:.1} {:.1}", chart_left, chart_bottom));

        // Line to first point
        let first_y = chart_top + chart_height - ((points[0] - min_val) / val_range) * chart_height;
        path.push_str(&format!(" L {:.1} {:.1}", chart_left, first_y));

        // Draw the sparkline
        for (i, &val) in points.iter().enumerate() {
            let x = chart_left + i as f64 * x_step;
            let y = chart_top + chart_height - ((val - min_val) / val_range) * chart_height;
            path.push_str(&format!(" L {:.1} {:.1}", x, y));
        }

        // Close at bottom-right
        let last_x = chart_left + (points.len() as f64 - 1.0) * x_step;
        path.push_str(&format!(" L {:.1} {:.1}", last_x, chart_bottom));
        path.push_str(" Z");

        path
    };

    // Get min/max values for axis labels
    let axis_values = move || {
        let points = data.get();
        if points.is_empty() {
            return (0.0, 100.0);
        }
        let min_val = points.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = points.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (min_val, max_val)
    };

    // Calculate trend
    let trend = move || {
        let points = data.get();
        if points.len() < 2 {
            return TrendDirection::Stable;
        }
        TrendDirection::from_values(points[0], points[points.len() - 1])
    };

    // Format value
    let formatted_value = move || {
        let v = value.get();
        match decimals {
            0 => format!("{}{:.0}{}", prefix_clone, v, suffix_clone),
            1 => format!("{}{:.1}{}", prefix_clone, v, suffix_clone),
            2 => format!("{}{:.2}{}", prefix_clone, v, suffix_clone),
            _ => format!("{}{:.3}{}", prefix_clone, v, suffix_clone),
        }
    };

    let line_color = color.as_css();
    let fill_color = color.as_fill_css();

    view! {
        <div
            class=combined_class
            style=format!("width: {}px; height: {}px;", width, height)
        >
            // Title
            {title.clone().map(|t| {
                let css_prefix = css_prefix.clone();
                view! {
                    <div class=format!("{}-title", css_prefix)>
                        {t}
                    </div>
                }
            })}

            // Value overlay
            <div class=format!("{}-value-container", css_prefix)>
                <span
                    class=format!("{}-value", css_prefix)
                    style=format!("color: {};", line_color)
                >
                    {formatted_value}
                </span>
                {show_trend.then(|| {
                    let css_prefix = css_prefix.clone();
                    view! {
                        <span
                            class=format!("{}-trend", css_prefix)
                            style=move || format!("color: {};", trend().as_color())
                        >
                            {move || match trend() {
                                TrendDirection::Up => "↑",
                                TrendDirection::Stable => "→",
                                TrendDirection::Down => "↓",
                            }}
                        </span>
                    }
                })}
            </div>

            // Sparkline SVG
            <svg
                class=format!("{}-sparkline", css_prefix)
                viewBox=format!("0 0 {} {}", svg_width, svg_height)
                preserveAspectRatio="none"
            >
                // Axis elements (when show_axis is true)
                {show_axis.then(|| {
                    view! {
                        <g class="sparkline-axis">
                            // Y-axis line
                            <line
                                x1=format!("{:.1}", chart_left)
                                y1=format!("{:.1}", chart_top)
                                x2=format!("{:.1}", chart_left)
                                y2=format!("{:.1}", chart_bottom)
                                stroke="var(--fx-color-border, #303030)"
                                stroke-width="1"
                            />
                            // X-axis line (baseline)
                            <line
                                x1=format!("{:.1}", chart_left)
                                y1=format!("{:.1}", chart_bottom)
                                x2=format!("{:.1}", chart_right)
                                y2=format!("{:.1}", chart_bottom)
                                stroke="var(--fx-color-border, #303030)"
                                stroke-width="1"
                            />
                            // Max value label
                            <text
                                x=format!("{:.1}", chart_left - 5.0)
                                y=format!("{:.1}", chart_top + 4.0)
                                fill="var(--fx-color-text-tertiary, #6b7280)"
                                font-size="10"
                                text-anchor="end"
                            >
                                {move || format!("{:.0}", axis_values().1)}
                            </text>
                            // Min value label
                            <text
                                x=format!("{:.1}", chart_left - 5.0)
                                y=format!("{:.1}", chart_bottom)
                                fill="var(--fx-color-text-tertiary, #6b7280)"
                                font-size="10"
                                text-anchor="end"
                            >
                                {move || format!("{:.0}", axis_values().0)}
                            </text>
                        </g>
                    }
                })}

                // Fill area
                <path
                    d=fill_path
                    fill=fill_color
                />
                // Line
                <path
                    d=sparkline_path
                    fill="none"
                    stroke=line_color
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
            </svg>
        </div>
    }
}
