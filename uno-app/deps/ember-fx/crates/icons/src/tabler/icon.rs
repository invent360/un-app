//! TablerIcon component.

use leptos::prelude::*;
use crate::types::{IconVariant, IconSize, IconColor};
use super::TablerIconData;

/// Tabler Icon component for rendering icons from the Tabler icon set.
///
/// Supports both outline and filled variants, multiple sizes, and semantic colors.
///
/// # Example
///
/// ```ignore
/// use ember_fx_icons::tabler::{TablerIcon, ArrowIcon};
/// use ember_fx_icons::types::{IconSize, IconVariant};
///
/// view! {
///     // Basic usage
///     <TablerIcon icon=ArrowIcon::ArrowDown />
///
///     // With size and variant
///     <TablerIcon
///         icon=ArrowIcon::ArrowUp
///         size=IconSize::Lg
///         variant=IconVariant::Filled
///     />
///
///     // With custom styling
///     <TablerIcon
///         icon=ArrowIcon::ChevronRight
///         class="text-blue-500"
///         stroke_width=1.5
///     />
/// }
/// ```
#[component]
pub fn TablerIcon<T: TablerIconData + 'static>(
    /// The icon to render.
    icon: T,
    /// Icon variant (outline or filled).
    #[prop(optional)]
    variant: Option<IconVariant>,
    /// Icon size preset.
    #[prop(optional)]
    size: Option<IconSize>,
    /// Semantic color.
    #[prop(optional)]
    color: Option<IconColor>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Custom inline styles.
    #[prop(optional, into)]
    style: Option<String>,
    /// Accessibility label.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Custom stroke width (default: 2.0 for outline icons).
    #[prop(optional)]
    stroke_width: Option<f32>,
    /// Whether the icon should spin (for loading indicators).
    #[prop(optional)]
    spin: Option<bool>,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
    let color = color.unwrap_or_default();
    let spin = spin.unwrap_or(false);

    // Get the SVG content
    let mut svg_content = icon.svg(variant).to_string();

    // Apply custom stroke width if specified (only for outline variant)
    if let Some(sw) = stroke_width {
        if variant == IconVariant::Outline {
            svg_content = svg_content.replace("stroke-width=\"2\"", &format!("stroke-width=\"{}\"", sw));
        }
    }

    // Build CSS classes
    let mut classes = vec!["fx-icon".to_string(), "fx-icon-tabler".to_string()];
    classes.push(size.to_class().to_string());

    let color_class = color.to_class();
    if !color_class.is_empty() {
        classes.push(color_class.to_string());
    }

    if spin {
        classes.push("fx-icon-spin".to_string());
    }

    if let Some(ref custom_class) = class {
        classes.push(custom_class.clone());
    }

    let class_str = classes.join(" ");
    let has_label = aria_label.is_some();

    // Build inline style if needed
    let style_str = style.unwrap_or_default();

    view! {
        <span
            class=class_str
            style=style_str
            role=if has_label { Some("img") } else { None }
            aria-label=aria_label
            aria-hidden=if has_label { None } else { Some("true") }
            inner_html=svg_content
        />
    }
}
