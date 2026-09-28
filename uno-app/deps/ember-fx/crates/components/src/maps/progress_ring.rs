//! ProgressRing component for circular percentage indicators.

use leptos::prelude::*;
use crate::try_use_theme;
use super::types::{ProgressRingData, ProgressRingConfig, progress_arc_path};
use crate::chart::ChartColor;

/// ProgressRing component.
///
/// Displays a circular progress indicator with optional percentage
/// and label text in the center.
///
/// # Example
///
/// ```ignore
/// let data = Signal::derive(|| ProgressRingData::new(75.0, "Active Users"));
///
/// view! {
///     <ProgressRing data=data />
/// }
/// ```
#[component]
pub fn ProgressRing(
    /// Progress data (value, max, label).
    data: Signal<ProgressRingData>,
    /// Configuration options.
    #[prop(optional)]
    config: Option<ProgressRingConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let prefix = format!("fx-progressring-{}", design_system);

    // Extract config values
    let size = config.size;
    let stroke_width = config.stroke_width;
    let show_percentage = config.show_percentage;
    let show_label = config.show_label;
    let start_angle = config.start_angle;
    let bg_opacity = config.background_opacity;

    // Calculate dimensions
    let cx = size as f64 / 2.0;
    let cy = size as f64 / 2.0;
    let radius = (size as f64 / 2.0) - stroke_width;

    // Reactive calculations
    let percentage = move || data.get().percentage();
    let value_text = move || {
        let pct = percentage() * 100.0;
        format!("{}%", pct as i32)
    };
    let label_text = move || data.get().label.clone();
    let sublabel_text = move || data.get().sublabel.clone();

    // Calculate paths
    let bg_path = move || {
        progress_arc_path(cx, cy, radius, start_angle, start_angle + 359.99)
    };

    let value_path = move || {
        let pct = percentage();
        if pct <= 0.0 {
            String::new()
        } else {
            let end_angle = start_angle + pct * 360.0;
            progress_arc_path(cx, cy, radius, start_angle, end_angle)
        }
    };

    // Get color
    let stroke_color = move || {
        data.get()
            .color
            .map(|c| c.as_css().to_string())
            .unwrap_or_else(|| "var(--fx-color-primary, #1890ff)".to_string())
    };

    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    view! {
        <div
            class=combined_class
            style=format!("display: inline-block; width: {}px; height: {}px;", size, size)
        >
            <svg
                viewBox=format!("0 0 {} {}", size, size)
                style="width: 100%; height: 100%;"
            >
                // Background ring
                <path
                    d=bg_path
                    fill="none"
                    stroke="var(--fx-color-border, #434343)"
                    stroke-width=stroke_width
                    opacity=bg_opacity
                    stroke-linecap="round"
                />

                // Value ring
                <path
                    d=value_path
                    fill="none"
                    stroke=stroke_color
                    stroke-width=stroke_width
                    stroke-linecap="round"
                    style="transition: stroke-dashoffset 0.5s ease;"
                />

                // Center text
                {show_percentage.then(|| view! {
                    <text
                        x=cx
                        y=move || if show_label { cy - 8.0 } else { cy + 4.0 }
                        text-anchor="middle"
                        dominant-baseline="middle"
                        style="font-size: 24px; font-weight: 600; fill: var(--fx-color-text, #fff);"
                    >
                        {value_text}
                    </text>
                })}

                {show_label.then(|| view! {
                    <text
                        x=cx
                        y=cx + 12.0
                        text-anchor="middle"
                        dominant-baseline="middle"
                        style="font-size: 12px; fill: var(--fx-color-text-secondary, #888);"
                    >
                        {label_text}
                    </text>
                })}

                // Sublabel (below label)
                {move || sublabel_text().map(|sub| view! {
                    <text
                        x=cx
                        y=cx + 26.0
                        text-anchor="middle"
                        dominant-baseline="middle"
                        style="font-size: 10px; fill: var(--fx-color-text-tertiary, #666);"
                    >
                        {sub}
                    </text>
                })}
            </svg>
        </div>
    }
}
