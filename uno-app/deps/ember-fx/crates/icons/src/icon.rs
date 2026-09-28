//! Icon component.

use leptos::prelude::*;
use crate::registry::{IconName, get_icon_svg};

/// Icon component for rendering SVG icons.
///
/// Can render icons by name (from registered icon sets) or from custom SVG content.
///
/// # Props
///
/// - `name` - Icon name from the registry (optional)
/// - `svg` - Custom SVG content (optional, used if `name` not provided)
/// - `class` - CSS classes for styling (typically for sizing)
/// - `aria_label` - Accessible label for screen readers
///
/// # Example
///
/// ```ignore
/// use ember_fx_icons::{Icon, IconName};
///
/// // Using a named icon
/// view! {
///     <Icon name=IconName::Check class="w-4 h-4" />
/// }
///
/// // Using custom SVG
/// view! {
///     <Icon svg="<svg>...</svg>" class="w-4 h-4" />
/// }
/// ```
#[component]
pub fn Icon(
    /// Icon name from the registry.
    #[prop(optional)]
    name: Option<IconName>,
    /// Custom SVG content (used if `name` is not provided).
    #[prop(optional, into)]
    svg: Option<String>,
    /// CSS classes for the icon wrapper.
    #[prop(optional, into)]
    class: Option<String>,
    /// Accessible label for screen readers.
    #[prop(optional, into)]
    aria_label: Option<String>,
) -> impl IntoView {
    // Get SVG content from name or use provided svg
    let svg_content = name
        .map(|n| get_icon_svg(n).to_string())
        .or(svg)
        .unwrap_or_default();

    let class = class.unwrap_or_default();
    let has_label = aria_label.is_some();

    view! {
        <span
            class=format!("fx-icon {}", class)
            role=if has_label { Some("img") } else { None }
            aria-label=aria_label
            aria-hidden=if has_label { None } else { Some("true") }
            inner_html=svg_content
        />
    }
}
