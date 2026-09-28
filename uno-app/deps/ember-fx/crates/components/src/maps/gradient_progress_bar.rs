//! GradientProgressBar component for linear progress visualization.

use leptos::prelude::*;
use crate::try_use_theme;
use super::types::{ProgressSegment, GradientProgressBarConfig, format_number};

/// GradientProgressBar component.
///
/// Displays a linear progress bar with gradient coloring or multiple segments.
///
/// # Example
///
/// ```ignore
/// let value = Signal::derive(|| 75.0);
///
/// view! {
///     <GradientProgressBar
///         value=value
///         label="Sales Figures"
///     />
/// }
/// ```
#[component]
pub fn GradientProgressBar(
    /// Current value (0 to max).
    value: Signal<f64>,
    /// Maximum value (default 100).
    #[prop(optional, default = 100.0)]
    max: f64,
    /// Segments for multi-segment mode.
    #[prop(optional)]
    segments: Option<Signal<Vec<ProgressSegment>>>,
    /// Label text.
    #[prop(optional, into)]
    label: Option<String>,
    /// Value prefix (e.g., "$").
    #[prop(optional, into)]
    value_prefix: Option<String>,
    /// Value suffix.
    #[prop(optional, into)]
    value_suffix: Option<String>,
    /// Configuration options.
    #[prop(optional)]
    config: Option<GradientProgressBarConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let prefix = format!("fx-progressbar-{}", design_system);

    // Extract config
    let height = config.height;
    let show_value = config.show_value;
    let gradient_colors = config.gradient_colors.clone();
    let border_radius = config.border_radius;
    let animate = config.animate;

    // Calculate percentage
    let percentage = move || {
        if max == 0.0 {
            0.0
        } else {
            (value.get() / max * 100.0).clamp(0.0, 100.0)
        }
    };

    // Format value for display
    let value_prefix_clone = value_prefix.clone();
    let value_suffix_clone = value_suffix.clone();
    let formatted_value = move || {
        let val = value.get();
        let prefix = value_prefix_clone.clone().unwrap_or_default();
        let suffix = value_suffix_clone.clone().unwrap_or_default();
        format!("{}{}{}", prefix, format_number(val), suffix)
    };

    // Build gradient string
    let gradient_str = gradient_colors.join(", ");

    let has_segments = segments.is_some();
    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    // Track style with animation
    let track_style = format!(
        "height: {}px; border-radius: {}px; background: var(--fx-color-bg-elevated, #1f1f1f); overflow: hidden;",
        height, border_radius
    );

    // Fill style
    let fill_style = move || {
        let transition = if animate { "width 0.5s ease" } else { "none" };
        format!(
            "height: 100%; width: {}%; background: linear-gradient(90deg, {}); border-radius: {}px; transition: {};",
            percentage(),
            gradient_str,
            border_radius,
            transition
        )
    };

    view! {
        <div class=combined_class>
            // Header with label and value
            {(label.is_some() || show_value).then(|| {
                let label_clone = label.clone();
                view! {
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                        {label_clone.map(|l| view! {
                            <span style="font-size: 14px; color: var(--fx-color-text-secondary, #888);">
                                {l}
                            </span>
                        })}
                        {show_value.then(|| view! {
                            <span style="font-size: 16px; font-weight: 600; color: var(--fx-color-text, #fff);">
                                {formatted_value}
                            </span>
                        })}
                    </div>
                }
            })}

            // Progress track
            <div style=track_style>
                // Single gradient bar
                {(!has_segments).then(|| view! {
                    <div style=fill_style />
                })}

                // Multi-segment bar
                {segments.map(|segs| {
                    let segments_view = move || {
                        let seg_list = segs.get();
                        let total: f64 = seg_list.iter().map(|s| s.value).sum();

                        seg_list.into_iter().map(|seg| {
                            let width_pct = if total > 0.0 {
                                seg.value / total * 100.0
                            } else {
                                0.0
                            };
                            let color = seg.color.unwrap_or_else(|| "var(--fx-color-primary)".to_string());

                            view! {
                                <div
                                    style=format!(
                                        "height: 100%; width: {}%; background: {}; display: inline-block;",
                                        width_pct, color
                                    )
                                    title=seg.label.unwrap_or_default()
                                />
                            }
                        }).collect_view()
                    };

                    view! {
                        <div style="display: flex; height: 100%;">
                            {segments_view}
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
