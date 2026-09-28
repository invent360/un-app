//! ClusterGaugePanel Leptos component.
//!
//! A Grafana-style semi-circular gauge with gradient arc and stats footer.

use leptos::prelude::*;
use crate::try_use_theme;

/// Configuration for the cluster gauge.
#[derive(Debug, Clone)]
pub struct ClusterGaugeConfig {
    /// Panel width.
    pub width: u32,
    /// Panel height (including stats).
    pub height: u32,
    /// Show gradient colors on arc.
    pub show_gradient: bool,
    /// Show Used/Total stats below gauge.
    pub show_stats: bool,
    /// Arc stroke width.
    pub stroke_width: f64,
}

impl Default for ClusterGaugeConfig {
    fn default() -> Self {
        Self {
            width: 300,
            height: 220,
            show_gradient: false, // Solid color works better on curved arcs
            show_stats: true,
            stroke_width: 20.0,
        }
    }
}

/// ClusterGaugePanel component.
///
/// A semi-circular gauge with gradient coloring (green→yellow→orange→red)
/// and Used/Total statistics below.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::ClusterGaugePanel;
///
/// view! {
///     <ClusterGaugePanel
///         value=Signal::derive(move || 6.96)
///         used=Signal::derive(move || 0.14)
///         total=Signal::derive(move || 2.0)
///         title="Cluster CPU usage".to_string()
///         unit="cores".to_string()
///     />
/// }
/// ```
#[component]
pub fn ClusterGaugePanel(
    /// Current percentage value (0-100).
    #[prop(into)]
    value: Signal<f64>,
    /// Used amount (absolute value).
    #[prop(into)]
    used: Signal<f64>,
    /// Total amount (absolute value).
    #[prop(into)]
    total: Signal<f64>,
    /// Panel title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Unit label (e.g., "cores", "GB").
    #[prop(optional, into)]
    unit: Option<String>,
    /// Configuration.
    #[prop(optional)]
    config: Option<ClusterGaugeConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let width = config.width;
    let height = config.height;
    let show_gradient = config.show_gradient;
    let show_stats = config.show_stats;
    let stroke_width = config.stroke_width;

    let unit = unit.unwrap_or_default();
    let unit_clone = unit.clone();

    let prefix = format!("fx-cluster-gauge-{}", design_system);

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
    let svg_width = width as f64;
    let svg_height = (height as f64) * 0.7; // Gauge takes 70% of height
    let center_x = svg_width / 2.0;
    let center_y = svg_height - 20.0; // Center near bottom for semi-circle
    let radius = (svg_width / 2.0) - stroke_width - 10.0;

    // Calculate arc path for background (full semi-circle)
    let bg_arc_path = {
        let start_angle: f64 = 180.0; // Left side
        let end_angle: f64 = 0.0; // Right side (sweep clockwise)

        let start_rad = start_angle.to_radians();
        let end_rad = end_angle.to_radians();

        let start_x = center_x + radius * start_rad.cos();
        let start_y = center_y - radius * start_rad.sin();
        let end_x = center_x + radius * end_rad.cos();
        let end_y = center_y - radius * end_rad.sin();

        format!(
            "M {:.1} {:.1} A {:.1} {:.1} 0 1 1 {:.1} {:.1}",
            start_x, start_y, radius, radius, end_x, end_y
        )
    };

    // Calculate value arc path
    let value_arc_path = move || {
        let pct = value.get().clamp(0.0, 100.0);
        if pct <= 0.0 {
            return String::new();
        }

        let angle_range = 180.0; // Full sweep is 180 degrees
        let value_angle = (pct / 100.0) * angle_range;

        // Start from left side (180 degrees in standard coords)
        let start_angle: f64 = std::f64::consts::PI; // 180 degrees = left side
        // End at the position based on percentage (going clockwise/upward in screen coords)
        let end_angle = std::f64::consts::PI - value_angle.to_radians();

        let start_x = center_x + radius * start_angle.cos();
        let start_y = center_y - radius * start_angle.sin();
        let end_x = center_x + radius * end_angle.cos();
        let end_y = center_y - radius * end_angle.sin();

        // For a semi-circle gauge, large-arc-flag should be 0 for <= 180°
        // sweep-flag = 1 means clockwise (going upward in SVG where Y is inverted)
        let large_arc = 0;

        format!(
            "M {:.1} {:.1} A {:.1} {:.1} 0 {} 1 {:.1} {:.1}",
            start_x, start_y, radius, radius, large_arc, end_x, end_y
        )
    };

    // Calculate indicator thumb position
    let thumb_position = move || {
        let pct = value.get().clamp(0.0, 100.0);
        let angle_range = 180.0;
        let value_angle = (pct / 100.0) * angle_range;

        // Same calculation as value_arc_path end position
        let angle_rad = std::f64::consts::PI - value_angle.to_radians();

        let x = center_x + radius * angle_rad.cos();
        let y = center_y - radius * angle_rad.sin();
        let rotation = -(180.0 - value_angle); // Rotate thumb to align with arc tangent

        (x, y, rotation)
    };

    // Determine value color based on percentage
    let value_color = move || {
        let pct = value.get();
        if pct < 50.0 {
            "#52c41a" // Green
        } else if pct < 75.0 {
            "#faad14" // Yellow
        } else if pct < 90.0 {
            "#fa8c16" // Orange
        } else {
            "#f5222d" // Red
        }
    };

    // Format the percentage display
    let formatted_value = move || {
        format!("{:.2}%", value.get())
    };

    // Format used/total display
    let formatted_used = move || {
        format!("{:.2}", used.get())
    };

    let formatted_total = move || {
        format!("{:.2}", total.get())
    };

    // Generate unique gradient ID
    let gradient_id = format!("gauge-gradient-{}", design_system);
    let gradient_url = format!("url(#{})", gradient_id);

    view! {
        <div
            class=combined_class
            style=format!("width: {}px; height: {}px;", width, height)
        >
            // Title
            {title.clone().map(|t| {
                let prefix = prefix.clone();
                view! {
                    <div class=format!("{}-title", prefix)>
                        {t}
                    </div>
                }
            })}

            // Gauge SVG
            <svg
                class=format!("{}-gauge", prefix)
                viewBox=format!("0 0 {} {}", svg_width, svg_height)
                preserveAspectRatio="xMidYMid meet"
            >
                // Gradient definition
                {show_gradient.then(|| {
                    let gradient_id = gradient_id.clone();
                    view! {
                        <defs>
                            <linearGradient id=gradient_id x1="0%" y1="0%" x2="100%" y2="0%">
                                <stop offset="0%" stop-color="#52c41a"/>
                                <stop offset="50%" stop-color="#faad14"/>
                                <stop offset="75%" stop-color="#fa8c16"/>
                                <stop offset="100%" stop-color="#f5222d"/>
                            </linearGradient>
                        </defs>
                    }
                })}

                // Background arc (gray track)
                <path
                    d=bg_arc_path.clone()
                    fill="none"
                    stroke="var(--fx-color-bg-elevated, #2a2a2a)"
                    stroke-width=format!("{}", stroke_width)
                    stroke-linecap="round"
                />

                // Value arc
                <path
                    d=value_arc_path
                    fill="none"
                    stroke=move || if show_gradient { gradient_url.clone() } else { value_color().to_string() }
                    stroke-width=format!("{}", stroke_width)
                    stroke-linecap="round"
                />

                // Indicator thumb
                {move || {
                    let (x, y, rotation) = thumb_position();
                    let thumb_size = 12.0;
                    view! {
                        <rect
                            x=format!("{:.1}", x - thumb_size / 2.0)
                            y=format!("{:.1}", y - thumb_size / 2.0)
                            width=format!("{}", thumb_size)
                            height=format!("{}", thumb_size)
                            fill=value_color()
                            transform=format!("rotate({:.1} {:.1} {:.1})", rotation + 45.0, x, y)
                        />
                    }
                }}

                // Center percentage text
                <text
                    x=format!("{:.1}", center_x)
                    y=format!("{:.1}", center_y - 20.0)
                    fill=value_color
                    font-size="32"
                    font-weight="bold"
                    text-anchor="middle"
                    dominant-baseline="middle"
                >
                    {formatted_value}
                </text>
            </svg>

            // Stats footer
            {show_stats.then(|| {
                let prefix = prefix.clone();
                let unit = unit_clone.clone();
                view! {
                    <div class=format!("{}-stats", prefix)>
                        <div class=format!("{}-stat", prefix)>
                            <div class=format!("{}-stat-label", prefix)>"Used"</div>
                            <div class=format!("{}-stat-value", prefix)>
                                <span class=format!("{}-stat-number", prefix)>{formatted_used}</span>
                                <span class=format!("{}-stat-unit", prefix)>{unit.clone()}</span>
                            </div>
                        </div>
                        <div class=format!("{}-stat", prefix)>
                            <div class=format!("{}-stat-label", prefix)>"Total"</div>
                            <div class=format!("{}-stat-value", prefix)>
                                <span class=format!("{}-stat-number", prefix)>{formatted_total}</span>
                                <span class=format!("{}-stat-unit", prefix)>{unit}</span>
                            </div>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
