//! ReputationGauge Leptos component.

use leptos::prelude::*;
use super::types::{ReputationGaugeSize, ReputationThresholds};
use crate::try_use_theme;

/// ReputationGauge component.
///
/// A GaugeChart variant optimized for 0-100 reputation scores with color zones.
///
/// # Props
///
/// - `score` - Reputation score (0-100)
/// - `thresholds` - Color zone thresholds
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::ReputationGauge;
///
/// view! {
///     <ReputationGauge
///         score=Signal::derive(move || 87.5)
///         show_label=true
///     />
/// }
/// ```
#[component]
pub fn ReputationGauge(
    /// Reputation score (0-100).
    #[prop(into)]
    score: Signal<f64>,
    /// Custom thresholds (default: 40=red, 70=yellow, 100=green).
    #[prop(optional)]
    thresholds: Option<ReputationThresholds>,
    /// Show "Reputation" label.
    #[prop(optional)]
    show_label: Option<bool>,
    /// Show numeric score in center.
    #[prop(optional)]
    show_score: Option<bool>,
    /// Size variant.
    #[prop(optional)]
    size: ReputationGaugeSize,
    /// On click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<f64>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let show_label = show_label.unwrap_or(true);
    let show_score = show_score.unwrap_or(true);
    let thresholds = thresholds.unwrap_or_default();

    // Clone thresholds for different closures
    let thresholds_for_class = thresholds.clone();
    let thresholds_for_arc = thresholds.clone();
    let thresholds_for_score = thresholds;

    // Build CSS classes
    let prefix = format!("fx-reputation-gauge-{}", design_system);

    // Pre-compute class names
    let base_class = prefix.clone();
    let size_class = format!("{}-{}", prefix, size.as_suffix());
    let svg_class = format!("{}-svg", prefix);
    let track_class = format!("{}-track", prefix);
    let arc_class = format!("{}-arc", prefix);
    let score_class = format!("{}-score", prefix);
    let label_class = format!("{}-label", prefix);

    let combined_class = {
        let base_class = base_class.clone();
        let size_class = size_class.clone();
        let class = class.clone();
        move || {
            let s = score.get().clamp(0.0, 100.0);
            let zone = thresholds_for_class.zone_for_score(s);
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

    // Gauge geometry
    // Semi-circle gauge: start at -135°, end at 135° (270° arc)
    let center_x = 100.0;
    let center_y = 100.0;
    let radius = 80.0;
    let stroke_width = 16.0;

    // Calculate arc path
    let arc_path = move || {
        let s = score.get().clamp(0.0, 100.0);
        let percentage = s / 100.0;

        // Start angle: -135° (bottom-left), End angle: 135° (bottom-right)
        let start_angle = -135.0_f64.to_radians();
        let end_angle_full = 135.0_f64.to_radians();
        let arc_length = end_angle_full - start_angle; // 270° in radians

        let current_angle = start_angle + (arc_length * percentage);

        // Start point
        let start_x = center_x + radius * start_angle.cos();
        let start_y = center_y + radius * start_angle.sin();

        // End point
        let end_x = center_x + radius * current_angle.cos();
        let end_y = center_y + radius * current_angle.sin();

        // Large arc flag: 1 if arc > 180°
        let large_arc = if percentage > 0.5 { 1 } else { 0 };

        format!(
            "M {:.2} {:.2} A {:.2} {:.2} 0 {} 1 {:.2} {:.2}",
            start_x, start_y, radius, radius, large_arc, end_x, end_y
        )
    };

    // Track path (full arc)
    let track_path = {
        let start_angle = -135.0_f64.to_radians();
        let end_angle = 135.0_f64.to_radians();

        let start_x = center_x + radius * start_angle.cos();
        let start_y = center_y + radius * start_angle.sin();
        let end_x = center_x + radius * end_angle.cos();
        let end_y = center_y + radius * end_angle.sin();

        format!(
            "M {:.2} {:.2} A {:.2} {:.2} 0 1 1 {:.2} {:.2}",
            start_x, start_y, radius, radius, end_x, end_y
        )
    };

    // Color based on score
    let arc_color = {
        move || {
            let s = score.get().clamp(0.0, 100.0);
            thresholds_for_arc.color_for_score(s)
        }
    };

    let click_handler = move |_| {
        if let Some(ref cb) = on_click {
            cb.run(score.get());
        }
    };

    view! {
        <div
            class=combined_class
            on:click=click_handler
        >
            <svg
                class=svg_class
                viewBox="0 0 200 140"
            >
                // Track (background arc)
                <path
                    class=track_class
                    d=track_path
                    fill="none"
                    stroke="var(--fx-color-bg-elevated, #2a2a2a)"
                    stroke-width=stroke_width
                    stroke-linecap="round"
                />
                // Value arc (foreground)
                <path
                    class=arc_class
                    d=arc_path
                    fill="none"
                    stroke=arc_color
                    stroke-width=stroke_width
                    stroke-linecap="round"
                />
            </svg>

            // Center score display
            {move || {
                let score_class = score_class.clone();
                let thresholds = thresholds_for_score.clone();
                if show_score {
                    Some(view! {
                        <div
                            class=score_class
                            style=move || {
                                let s = score.get().clamp(0.0, 100.0);
                                format!("color: {};", thresholds.color_for_score(s))
                            }
                        >
                            {move || format!("{:.0}", score.get().clamp(0.0, 100.0))}
                        </div>
                    })
                } else {
                    None
                }
            }}

            // Label
            {move || {
                if show_label {
                    Some(view! {
                        <div class=label_class.clone()>"REPUTATION"</div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
