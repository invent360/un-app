//! ResourceUsageGauge Leptos component.
//!
//! A dial/speedometer-style gauge for displaying resource utilization.

use leptos::prelude::*;
use crate::try_use_theme;

/// Resource type for preset configurations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResourceType {
    /// CPU usage.
    #[default]
    Cpu,
    /// RAM/Memory usage.
    Ram,
    /// Storage/Disk usage.
    Storage,
    /// Swap usage.
    Swap,
    /// Network/Bandwidth usage.
    Network,
    /// Generic percentage.
    Generic,
}

impl ResourceType {
    /// Returns the default label for this resource type.
    pub fn default_label(&self) -> &'static str {
        match self {
            Self::Cpu => "CPU usage",
            Self::Ram => "RAM usage",
            Self::Storage => "Storage used",
            Self::Swap => "Swap usage",
            Self::Network => "Bandwidth",
            Self::Generic => "Usage",
        }
    }
}

/// Size variants for the gauge.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResourceGaugeSize {
    /// Small size (100px).
    Small,
    /// Default size (140px).
    #[default]
    Default,
    /// Large size (180px).
    Large,
}

impl ResourceGaugeSize {
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

/// Threshold configuration for color zones.
#[derive(Debug, Clone)]
pub struct UsageThresholds {
    /// Below this = green (good).
    pub warning: f64,
    /// Above this = red (critical).
    pub critical: f64,
}

impl Default for UsageThresholds {
    fn default() -> Self {
        Self {
            warning: 60.0,
            critical: 85.0,
        }
    }
}

impl UsageThresholds {
    /// Create new thresholds.
    pub fn new(warning: f64, critical: f64) -> Self {
        Self { warning, critical }
    }

    /// Returns the color for a given value.
    pub fn color_for_value(&self, value: f64) -> &'static str {
        if value >= self.critical {
            "var(--fx-color-error, #ff4d4f)"
        } else if value >= self.warning {
            "var(--fx-color-warning, #faad14)"
        } else {
            "var(--fx-color-success, #52c41a)"
        }
    }

    /// Returns the zone suffix for a given value.
    pub fn zone_for_value(&self, value: f64) -> &'static str {
        if value >= self.critical {
            "critical"
        } else if value >= self.warning {
            "warning"
        } else {
            "normal"
        }
    }
}

/// ResourceUsageGauge component.
///
/// A dial/speedometer-style gauge showing resource utilization with
/// color-coded zones (green → yellow → red).
///
/// # Props
///
/// - `value` - Current value (0-100)
/// - `label` - Label text
/// - `resource_type` - Type of resource (for preset styling)
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{ResourceUsageGauge, ResourceType};
///
/// view! {
///     <ResourceUsageGauge
///         value=Signal::derive(move || 45.0)
///         resource_type=ResourceType::Cpu
///     />
/// }
/// ```
#[component]
pub fn ResourceUsageGauge(
    /// Current value (0-100).
    #[prop(into)]
    value: Signal<f64>,
    /// Label text.
    #[prop(optional, into)]
    label: Option<String>,
    /// Resource type for preset styling.
    #[prop(optional)]
    resource_type: ResourceType,
    /// Custom thresholds.
    #[prop(optional)]
    thresholds: Option<UsageThresholds>,
    /// Size variant.
    #[prop(optional)]
    size: ResourceGaugeSize,
    /// Show percentage value.
    #[prop(optional)]
    show_value: Option<bool>,
    /// Value suffix (default: "%").
    #[prop(optional, into)]
    suffix: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let label = label.unwrap_or_else(|| resource_type.default_label().to_string());
    let thresholds = thresholds.unwrap_or_default();
    let show_value = show_value.unwrap_or(true);
    let suffix = suffix.unwrap_or_else(|| "%".to_string());
    let gauge_size = size.as_pixels();

    // Build CSS classes
    let prefix = format!("fx-resource-gauge-{}", design_system);

    let base_class = prefix.clone();
    let size_class = format!("{}-{}", prefix, size.as_suffix());

    // Clone for closures
    let thresholds_for_zone = thresholds.clone();
    let thresholds_for_color = thresholds.clone();

    let combined_class = {
        let base_class = base_class.clone();
        let size_class = size_class.clone();
        let class = class.clone();
        move || {
            let val = value.get().clamp(0.0, 100.0);
            let zone = thresholds_for_zone.zone_for_value(val);
            let mut parts = vec![
                base_class.clone(),
                size_class.clone(),
                format!("{}-{}", base_class, zone),
            ];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Calculate angles for the gauge
    // The gauge spans from 135° to 405° (270° total arc)
    // Start angle is at bottom-left, end is at bottom-right
    let start_angle: f64 = 135.0;
    let end_angle: f64 = 405.0;
    let total_arc: f64 = end_angle - start_angle;

    // Pre-calculate threshold angles for the colored background arc
    let warning_angle = start_angle + (thresholds.warning / 100.0) * total_arc;
    let critical_angle = start_angle + (thresholds.critical / 100.0) * total_arc;

    // SVG viewBox size
    let view_size: f64 = 100.0;
    let center: f64 = view_size / 2.0;
    let radius: f64 = 40.0;
    let stroke_width: f64 = 8.0;
    let inner_radius: f64 = radius - stroke_width / 2.0 - 2.0;

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

    // Create the three colored zone arcs
    let green_arc = create_arc_path(center, radius, start_angle, warning_angle);
    let yellow_arc = create_arc_path(center, radius, warning_angle, critical_angle);
    let red_arc = create_arc_path(center, radius, critical_angle, end_angle);
    let bg_arc = create_arc_path(center, radius, start_angle, end_angle);

    // Create filled wedge path (the "needle" indicator)
    let wedge_path = move || {
        let val = value.get().clamp(0.0, 100.0);
        let value_angle = start_angle + (val / 100.0) * total_arc;

        // Helper to convert angle to point (inline for closure)
        let angle_to_pt = |angle: f64| -> (f64, f64) {
            let rad = angle.to_radians();
            (center + inner_radius * rad.cos(), center + inner_radius * rad.sin())
        };

        // Create a pie wedge from center to the value angle
        let (x1, y1) = angle_to_pt(start_angle);
        let (x2, y2) = angle_to_pt(value_angle);
        let large_arc = if (value_angle - start_angle).abs() > 180.0 { 1 } else { 0 };

        format!(
            "M {} {} L {} {} A {} {} 0 {} 1 {} {} Z",
            center, center,  // Start at center
            x1, y1,          // Line to start of arc
            inner_radius, inner_radius, large_arc,  // Arc
            x2, y2           // End of arc
        )
    };

    // Clone thresholds for color closures
    let thresholds_for_display = thresholds.clone();

    // Calculate wedge color based on current value
    let wedge_color = move || {
        let val = value.get().clamp(0.0, 100.0);
        thresholds_for_color.color_for_value(val).to_string()
    };

    // Clone suffix for display closure
    let suffix_clone = suffix.clone();

    view! {
        <div
            class=combined_class
            style=format!("width: {}px; height: {}px;", gauge_size, gauge_size)
        >
            // Label
            <div class=format!("{}-label", prefix)>
                {label.clone()}
            </div>

            // SVG Gauge
            <svg
                viewBox=format!("0 0 {} {}", view_size, view_size)
                class=format!("{}-svg", prefix)
            >
                // Background track (gray)
                <path
                    d=bg_arc.clone()
                    fill="none"
                    stroke="var(--fx-color-border, #303030)"
                    stroke-width=stroke_width
                    stroke-linecap="round"
                />

                // Green zone
                <path
                    d=green_arc.clone()
                    fill="none"
                    stroke="var(--fx-color-success, #52c41a)"
                    stroke-width=stroke_width
                    stroke-linecap="round"
                    opacity="0.3"
                />

                // Yellow zone
                <path
                    d=yellow_arc.clone()
                    fill="none"
                    stroke="var(--fx-color-warning, #faad14)"
                    stroke-width=stroke_width
                    stroke-linecap="round"
                    opacity="0.3"
                />

                // Red zone
                <path
                    d=red_arc.clone()
                    fill="none"
                    stroke="var(--fx-color-error, #ff4d4f)"
                    stroke-width=stroke_width
                    stroke-linecap="round"
                    opacity="0.3"
                />

                // Filled wedge indicator
                <path
                    d=wedge_path
                    fill=wedge_color
                    opacity="0.9"
                />

                // Center circle (to create donut effect)
                <circle
                    cx=center
                    cy=center
                    r="20"
                    fill="var(--fx-color-bg-container, #141414)"
                />
            </svg>

            // Value display
            {move || {
                if show_value {
                    let val = value.get().clamp(0.0, 100.0);
                    let color = thresholds_for_display.color_for_value(val);
                    Some(view! {
                        <div
                            class=format!("{}-value", prefix)
                            style=format!("color: {};", color)
                        >
                            {format!("{:.0}{}", val, suffix_clone)}
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}

// Convenience type aliases for specific resource gauges

/// CPU usage gauge preset.
#[component]
pub fn CpuUsageGauge(
    #[prop(into)] value: Signal<f64>,
    #[prop(optional)] size: ResourceGaugeSize,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ResourceUsageGauge
            value=value
            resource_type=ResourceType::Cpu
            size=size
            class=class.unwrap_or_default()
        />
    }
}

/// RAM usage gauge preset.
#[component]
pub fn RamUsageGauge(
    #[prop(into)] value: Signal<f64>,
    #[prop(optional)] size: ResourceGaugeSize,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ResourceUsageGauge
            value=value
            resource_type=ResourceType::Ram
            size=size
            class=class.unwrap_or_default()
        />
    }
}

/// Storage usage gauge preset.
#[component]
pub fn StorageUsageGauge(
    #[prop(into)] value: Signal<f64>,
    #[prop(optional)] size: ResourceGaugeSize,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ResourceUsageGauge
            value=value
            resource_type=ResourceType::Storage
            size=size
            class=class.unwrap_or_default()
        />
    }
}

/// Swap usage gauge preset.
#[component]
pub fn SwapUsageGauge(
    #[prop(into)] value: Signal<f64>,
    #[prop(optional)] size: ResourceGaugeSize,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ResourceUsageGauge
            value=value
            resource_type=ResourceType::Swap
            size=size
            class=class.unwrap_or_default()
        />
    }
}
