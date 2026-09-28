//! RelayTrafficGauge Leptos component.
//!
//! A dial/speedometer-style gauge for displaying relay traffic utilization.

use leptos::prelude::*;
use crate::try_use_theme;

/// Traffic direction for relay.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TrafficDirection {
    /// Combined inbound and outbound.
    #[default]
    Combined,
    /// Inbound traffic only.
    Inbound,
    /// Outbound traffic only.
    Outbound,
}

impl TrafficDirection {
    /// Returns the label for this direction.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Combined => "Relay Traffic",
            Self::Inbound => "Inbound",
            Self::Outbound => "Outbound",
        }
    }

    /// Returns the icon for this direction.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::Combined => "⇄",
            Self::Inbound => "↓",
            Self::Outbound => "↑",
        }
    }
}

/// Size variants for the gauge.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TrafficGaugeSize {
    /// Small size.
    Small,
    /// Default size.
    #[default]
    Default,
    /// Large size.
    Large,
}

impl TrafficGaugeSize {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "md",
            Self::Large => "lg",
        }
    }

    /// Returns the size in pixels.
    pub fn as_pixels(&self) -> u32 {
        match self {
            Self::Small => 100,
            Self::Default => 140,
            Self::Large => 180,
        }
    }
}

/// Traffic level classification.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TrafficLevel {
    /// Low traffic (idle).
    #[default]
    Low,
    /// Normal traffic.
    Normal,
    /// High traffic.
    High,
    /// Saturated (at capacity).
    Saturated,
}

impl TrafficLevel {
    /// Determine level from utilization percentage.
    pub fn from_percentage(value: f64) -> Self {
        if value >= 90.0 {
            Self::Saturated
        } else if value >= 70.0 {
            Self::High
        } else if value >= 30.0 {
            Self::Normal
        } else {
            Self::Low
        }
    }

    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
            Self::Saturated => "saturated",
        }
    }

    /// Returns the color for this level.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Low => "var(--fx-color-text-tertiary, #6b7280)",
            Self::Normal => "var(--fx-color-success, #52c41a)",
            Self::High => "var(--fx-color-warning, #faad14)",
            Self::Saturated => "var(--fx-color-error, #ff4d4f)",
        }
    }

    /// Returns the label for this level.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Normal => "Normal",
            Self::High => "High",
            Self::Saturated => "Saturated",
        }
    }
}

/// Format bandwidth value for display.
fn format_bandwidth(mbps: f64) -> String {
    if mbps >= 1000.0 {
        format!("{:.1} Gbps", mbps / 1000.0)
    } else if mbps >= 1.0 {
        format!("{:.1} Mbps", mbps)
    } else {
        format!("{:.0} Kbps", mbps * 1000.0)
    }
}

/// RelayTrafficGauge component.
///
/// A dial-style gauge showing relay traffic utilization with Mbps display.
///
/// # Props
///
/// - `current` - Current bandwidth in Mbps
/// - `max_capacity` - Maximum capacity in Mbps
/// - `direction` - Traffic direction (Combined, Inbound, Outbound)
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::RelayTrafficGauge;
///
/// view! {
///     <RelayTrafficGauge
///         current=Signal::derive(move || 75.5)
///         max_capacity=100.0
///     />
/// }
/// ```
#[component]
pub fn RelayTrafficGauge(
    /// Current bandwidth in Mbps.
    #[prop(into)]
    current: Signal<f64>,
    /// Maximum capacity in Mbps.
    #[prop(optional)]
    max_capacity: Option<f64>,
    /// Traffic direction.
    #[prop(optional)]
    direction: TrafficDirection,
    /// Custom title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Size variant.
    #[prop(optional)]
    size: TrafficGaugeSize,
    /// Show traffic level label.
    #[prop(optional)]
    show_level: Option<bool>,
    /// Show direction icon.
    #[prop(optional)]
    show_icon: Option<bool>,
    /// Active connections count.
    #[prop(optional, into)]
    connections: Option<Signal<u32>>,
    /// Packets per second.
    #[prop(optional, into)]
    packets_per_sec: Option<Signal<u32>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let title = title.unwrap_or_else(|| direction.as_label().to_string());
    let max_capacity = max_capacity.unwrap_or(100.0);
    let show_level = show_level.unwrap_or(true);
    let show_icon = show_icon.unwrap_or(true);
    let gauge_size = size.as_pixels();

    let prefix = format!("fx-relay-traffic-{}", design_system);

    // Calculate utilization percentage
    let utilization = move || {
        let curr = current.get();
        ((curr / max_capacity) * 100.0).clamp(0.0, 100.0)
    };

    let level = move || TrafficLevel::from_percentage(utilization());

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![
                prefix.clone(),
                format!("{}-{}", prefix, size.as_suffix()),
                format!("{}-{}", prefix, level().as_suffix()),
            ];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // SVG gauge parameters
    let view_size: f64 = 100.0;
    let center: f64 = view_size / 2.0;
    let radius: f64 = 40.0;
    let stroke_width: f64 = 8.0;

    // Arc from 135° to 405° (270° total)
    let start_angle: f64 = 135.0;
    let end_angle: f64 = 405.0;
    let total_arc: f64 = end_angle - start_angle;

    // Helper function to create arc path
    fn create_arc_path(center: f64, radius: f64, start: f64, end: f64) -> String {
        let angle_to_point = |angle: f64| -> (f64, f64) {
            let rad = angle.to_radians();
            (center + radius * rad.cos(), center + radius * rad.sin())
        };
        let (x1, y1) = angle_to_point(start);
        let (x2, y2) = angle_to_point(end);
        let large_arc = if (end - start).abs() > 180.0 { 1 } else { 0 };
        format!(
            "M {} {} A {} {} 0 {} 1 {} {}",
            x1, y1, radius, radius, large_arc, x2, y2
        )
    }

    // Background arc
    let bg_arc = create_arc_path(center, radius, start_angle, end_angle);

    // Progress arc
    let progress_arc = move || {
        let val = utilization();
        let value_angle = start_angle + (val / 100.0) * total_arc;
        if value_angle <= start_angle {
            String::new()
        } else {
            create_arc_path(center, radius, start_angle, value_angle)
        }
    };

    // Clone for closures
    let prefix_gauge = prefix.clone();
    let prefix_content = prefix.clone();
    let prefix_details = prefix.clone();

    view! {
        <div
            class=combined_class
            style=format!("width: {}px;", gauge_size)
        >
            // Title with optional icon
            <div class=format!("{}-header", prefix)>
                {show_icon.then(|| view! {
                    <span class=format!("{}-icon", prefix)>
                        {direction.as_icon()}
                    </span>
                })}
                <span class=format!("{}-title", prefix)>
                    {title.clone()}
                </span>
            </div>

            // Gauge
            <div
                class=format!("{}-gauge-container", prefix_gauge)
                style=format!("width: {}px; height: {}px;", gauge_size, gauge_size)
            >
                <svg
                    viewBox=format!("0 0 {} {}", view_size, view_size)
                    class=format!("{}-gauge", prefix_gauge)
                >
                    // Background
                    <path
                        d=bg_arc.clone()
                        fill="none"
                        stroke="var(--fx-color-border, #303030)"
                        stroke-width=stroke_width
                        stroke-linecap="round"
                    />
                    // Progress
                    <path
                        d=progress_arc
                        fill="none"
                        stroke=move || level().as_color()
                        stroke-width=stroke_width
                        stroke-linecap="round"
                    />
                </svg>

                // Center content
                <div class=format!("{}-gauge-content", prefix_content)>
                    <span
                        class=format!("{}-gauge-value", prefix_content)
                        style=move || format!("color: {};", level().as_color())
                    >
                        {move || format_bandwidth(current.get())}
                    </span>
                    {show_level.then(|| {
                        let prefix = prefix_content.clone();
                        view! {
                            <span
                                class=format!("{}-gauge-level", prefix)
                                style=move || format!("color: {};", level().as_color())
                            >
                                {move || level().as_label()}
                            </span>
                        }
                    })}
                </div>
            </div>

            // Details row
            <div class=format!("{}-details", prefix_details)>
                <div class=format!("{}-detail", prefix_details)>
                    <span class=format!("{}-detail-label", prefix_details)>"Capacity"</span>
                    <span class=format!("{}-detail-value", prefix_details)>
                        {format_bandwidth(max_capacity)}
                    </span>
                </div>
                <div class=format!("{}-detail", prefix_details)>
                    <span class=format!("{}-detail-label", prefix_details)>"Utilization"</span>
                    <span
                        class=format!("{}-detail-value", prefix_details)
                        style=move || format!("color: {};", level().as_color())
                    >
                        {move || format!("{:.0}%", utilization())}
                    </span>
                </div>
            </div>

            // Optional stats
            {connections.map(|conn| {
                let prefix = prefix_details.clone();
                view! {
                    <div class=format!("{}-stat", prefix)>
                        <span class=format!("{}-stat-label", prefix)>"Connections"</span>
                        <span class=format!("{}-stat-value", prefix)>
                            {move || conn.get()}
                        </span>
                    </div>
                }
            })}
            {packets_per_sec.map(|pps| {
                let prefix = prefix_details.clone();
                view! {
                    <div class=format!("{}-stat", prefix)>
                        <span class=format!("{}-stat-label", prefix)>"Packets/sec"</span>
                        <span class=format!("{}-stat-value", prefix)>
                            {move || format!("{}", pps.get())}
                        </span>
                    </div>
                }
            })}
        </div>
    }
}
