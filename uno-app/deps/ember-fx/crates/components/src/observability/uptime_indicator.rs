//! UptimeIndicator Leptos component.

use leptos::prelude::*;
use super::types::{UptimeSize, UptimeColorMode};
use crate::try_use_theme;

/// UptimeIndicator component.
///
/// Circular progress ring showing uptime percentage (0-100%).
///
/// # Props
///
/// - `uptime` - Uptime value (0.0 to 1.0)
/// - `size` - Ring size variant
/// - `show_value` - Display percentage in center
/// - `color_mode` - Color threshold mode
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{UptimeIndicator, UptimeSize};
///
/// view! {
///     <UptimeIndicator
///         uptime=Signal::derive(move || 0.9975)
///         size=UptimeSize::Default
///         show_value=true
///     />
/// }
/// ```
#[component]
pub fn UptimeIndicator(
    /// Uptime value (0.0 to 1.0).
    #[prop(into)]
    uptime: Signal<f64>,
    /// Size variant.
    #[prop(optional)]
    size: UptimeSize,
    /// Show percentage value in center.
    #[prop(optional)]
    show_value: Option<bool>,
    /// Tooltip text.
    #[prop(optional, into)]
    tooltip: Option<String>,
    /// Color mode for thresholds.
    #[prop(optional)]
    _color_mode: UptimeColorMode,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let show_value = show_value.unwrap_or(true);

    // Build CSS classes
    let prefix = format!("fx-uptime-indicator-{}", design_system);

    // Pre-compute class names
    let base_class = prefix.clone();
    let size_class = format!("{}-{}", prefix, size.as_suffix());
    let svg_class = format!("{}-svg", prefix);
    let track_class = format!("{}-track", prefix);
    let ring_class = format!("{}-ring", prefix);
    let value_class = format!("{}-value", prefix);
    let value_size_class = format!("{}-value-{}", prefix, size.as_suffix());

    let combined_class = {
        let base_class = base_class.clone();
        let size_class = size_class.clone();
        let class = class.clone();
        move || {
            let val = uptime.get().clamp(0.0, 1.0);
            let mut parts = vec![base_class.clone(), size_class.clone()];

            // Add state class based on uptime value
            let state = if val >= 0.99 {
                "healthy"
            } else if val >= 0.95 {
                "warning"
            } else {
                "critical"
            };
            parts.push(format!("{}-{}", base_class, state));

            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Calculate ring geometry
    // SVG circle params: center at (50, 50), radius 40, stroke-width 8
    let circumference = 2.0 * std::f64::consts::PI * 40.0; // ~251.33

    let stroke_dasharray = move || {
        let val = uptime.get().clamp(0.0, 1.0);
        let filled = circumference * val;
        format!("{:.2} {:.2}", filled, circumference)
    };

    // Color based on uptime value
    let stroke_color = move || {
        let val = uptime.get().clamp(0.0, 1.0);
        if val >= 0.99 {
            "var(--fx-color-success, #52c41a)"
        } else if val >= 0.95 {
            "var(--fx-color-warning, #faad14)"
        } else {
            "var(--fx-color-error, #ff4d4f)"
        }
    };

    // Format percentage
    let percentage_text = move || {
        let val = uptime.get().clamp(0.0, 1.0) * 100.0;
        if val >= 99.99 {
            "100%".to_string()
        } else if val >= 99.9 {
            format!("{:.2}%", val)
        } else {
            format!("{:.1}%", val)
        }
    };

    view! {
        <div
            class=combined_class
            title=tooltip.clone()
        >
            <svg
                class=svg_class
                viewBox="0 0 100 100"
            >
                // Track circle (background)
                <circle
                    class=track_class
                    cx="50"
                    cy="50"
                    r="40"
                    fill="none"
                    stroke-width="8"
                />
                // Value arc (foreground)
                <circle
                    class=ring_class
                    cx="50"
                    cy="50"
                    r="40"
                    fill="none"
                    stroke-width="8"
                    stroke-dasharray=stroke_dasharray
                    stroke-dashoffset="0"
                    transform="rotate(-90 50 50)"
                    style=move || format!("stroke: {};", stroke_color())
                />
            </svg>
            // Center value text
            {move || {
                if show_value {
                    Some(view! {
                        <span class=format!("{} {}", value_class.clone(), value_size_class.clone())>
                            {percentage_text}
                        </span>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
