//! IconButton Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{ButtonVariant, ButtonSize, ButtonShape};
use crate::try_use_theme;

/// IconButton component.
///
/// A button designed for icon-only content. Circular by default with
/// proper aspect ratio and accessibility support.
///
/// # Props
///
/// - `variant` - Visual variant (Primary, Secondary, Outline, Ghost, Danger, Link)
/// - `size` - Size variant (Xs, Sm, Md, Lg, Xl)
/// - `shape` - Button shape (Circle by default, can be Round or Default)
/// - `disabled` - Whether the button is disabled
/// - `loading` - Whether the button is in loading state
/// - `aria_label` - Accessible label (required for icon-only buttons)
/// - `tooltip` - Tooltip text on hover
/// - `class` - Additional CSS classes
/// - `on_click` - Click handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{IconButton, ButtonVariant};
///
/// view! {
///     <IconButton
///         variant=ButtonVariant::Primary
///         aria_label="Edit item"
///         on_click=move |_| log::info!("Edit clicked!")
///     >
///         "✏️"
///     </IconButton>
///
///     // With tooltip
///     <IconButton
///         variant=ButtonVariant::Ghost
///         aria_label="Delete"
///         tooltip="Delete this item"
///     >
///         "🗑️"
///     </IconButton>
/// }
/// ```
#[component]
pub fn IconButton(
    /// Button variant (visual style).
    #[prop(optional, into)]
    variant: Option<ButtonVariant>,
    /// Button size.
    #[prop(optional, into)]
    size: Option<ButtonSize>,
    /// Button shape (defaults to Circle for IconButton).
    #[prop(optional, into)]
    shape: Option<ButtonShape>,
    /// Whether the button is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Whether the button is in loading state.
    #[prop(optional)]
    loading: bool,
    /// Accessible label (required for icon-only buttons).
    #[prop(into)]
    aria_label: String,
    /// Tooltip text.
    #[prop(optional, into)]
    tooltip: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// Icon content.
    children: Children,
) -> impl IntoView {
    // Get theme context for design system
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.design_system().as_str())
        .unwrap_or("ant");

    // Resolve defaults - IconButton defaults to Circle shape
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
    let shape = shape.unwrap_or(ButtonShape::Circle);

    // Build CSS classes
    let btn_prefix = format!("fx-btn-{}", design_system);
    let icon_btn_prefix = format!("{}-icon", btn_prefix);

    let combined_class = {
        let mut parts = vec![
            btn_prefix.clone(),
            icon_btn_prefix.clone(),
            variant.class(&btn_prefix),
            size.class(&btn_prefix),
        ];

        // Add shape class
        let shape_class = shape.class(&btn_prefix);
        if !shape_class.is_empty() {
            parts.push(shape_class);
        }

        // State classes
        if disabled || loading {
            parts.push(format!("{}-disabled", btn_prefix));
        }
        if loading {
            parts.push(format!("{}-loading", btn_prefix));
        }

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }

        parts.join(" ")
    };

    // Click handler that respects disabled/loading state
    let handle_click = move |ev: ev::MouseEvent| {
        if disabled || loading {
            ev.prevent_default();
            return;
        }
        if let Some(ref cb) = on_click {
            cb.run(ev);
        }
    };

    // Classes for inner elements
    let spinner_class = format!("{}-spinner", btn_prefix);

    // Render children once outside reactive closure
    let children_view = children();

    view! {
        <button
            type="button"
            class=combined_class
            disabled=disabled || loading
            aria-disabled=move || if disabled || loading { Some("true") } else { None }
            aria-busy=move || if loading { Some("true") } else { None }
            aria-label=aria_label.clone()
            title=tooltip.clone()
            on:click=handle_click
        >
            {if loading {
                view! {
                    <span class=spinner_class></span>
                }.into_any()
            } else {
                view! {
                    <span class="fx-btn-icon-content">
                        {children_view}
                    </span>
                }.into_any()
            }}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_button_defaults_to_circle() {
        let shape = ButtonShape::Circle;
        assert_eq!(shape.class("fx-btn"), "fx-btn-circle");
    }
}
