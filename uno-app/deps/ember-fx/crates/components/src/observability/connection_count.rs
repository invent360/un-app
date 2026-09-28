//! ConnectionCountIndicator Leptos component.

use leptos::prelude::*;
use super::types::TrendDirection;
use crate::try_use_theme;

/// ConnectionCountIndicator component.
///
/// Shows current peer count with trend arrow.
///
/// # Props
///
/// - `count` - Current connection count
/// - `previous_count` - Previous count for trend calculation
/// - `target` - Target/minimum peer count threshold
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::ConnectionCountIndicator;
///
/// view! {
///     <ConnectionCountIndicator
///         count=Signal::derive(move || 24)
///         previous_count=22
///         target=10
///     />
/// }
/// ```
#[component]
pub fn ConnectionCountIndicator(
    /// Current peer/connection count.
    #[prop(into)]
    count: Signal<u32>,
    /// Previous count for trend.
    #[prop(optional)]
    previous_count: Option<u32>,
    /// Target/minimum threshold.
    #[prop(optional)]
    target: Option<u32>,
    /// Show trend arrow.
    #[prop(optional)]
    show_trend: Option<bool>,
    /// Label text.
    #[prop(optional, into)]
    label: Option<String>,
    /// Compact mode.
    #[prop(optional)]
    compact: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let show_trend = show_trend.unwrap_or(true);
    let compact = compact.unwrap_or(false);
    let label = label.unwrap_or_else(|| "Peers".to_string());

    // Build CSS classes
    let prefix = format!("fx-connection-count-{}", design_system);

    // Pre-compute class names
    let base_class = prefix.clone();
    let compact_class = format!("{}-compact", prefix);
    let icon_class = format!("{}-icon", prefix);
    let value_class = format!("{}-value", prefix);
    let label_class = format!("{}-label", prefix);
    let trend_class = format!("{}-trend", prefix);
    let below_target_class = format!("{}-below-target", prefix);

    let combined_class = {
        let base_class = base_class.clone();
        let compact_class = compact_class.clone();
        let below_target_class = below_target_class.clone();
        let class = class.clone();
        move || {
            let current = count.get();
            let mut parts = vec![base_class.clone()];
            if compact {
                parts.push(compact_class.clone());
            }
            if let Some(t) = target {
                if current < t {
                    parts.push(below_target_class.clone());
                }
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Calculate trend direction
    let trend = move || {
        if let Some(prev) = previous_count {
            let current = count.get();
            TrendDirection::from_counts(current, prev)
        } else {
            TrendDirection::Flat
        }
    };

    let trend_full_class = {
        let trend_class = trend_class.clone();
        move || {
            let direction = trend();
            format!("{} {}-{}", trend_class, trend_class, direction.as_suffix())
        }
    };

    view! {
        <div class=combined_class>
            // Network icon
            <span class=icon_class.clone()>"🌐"</span>

            // Count value
            <span class=value_class.clone()>
                {move || count.get().to_string()}
            </span>

            // Label
            <span class=label_class.clone()>{label.clone()}</span>

            // Trend indicator
            {move || {
                if show_trend && previous_count.is_some() {
                    let direction = trend();
                    Some(view! {
                        <span
                            class=trend_full_class.clone()
                            style=format!("color: {};", direction.as_color())
                        >
                            {direction.as_icon()}
                        </span>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
