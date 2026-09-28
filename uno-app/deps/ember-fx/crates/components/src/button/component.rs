//! Button Leptos component.
//!
//! A comprehensive button component following Ant Design patterns with support
//! for both legacy variant API and modern color + style_variant system.

use leptos::prelude::*;
use leptos::ev;
use super::types::{
    ButtonVariant, ButtonSize, ButtonShape, ButtonColor, ButtonStyleVariant,
    IconPosition, HtmlButtonType,
};
use crate::try_use_theme;

#[cfg(feature = "template")]
use crate::template::use_button_template;

/// Button component.
///
/// A versatile button with multiple variants, sizes, shapes, and states.
/// Automatically integrates with the theme context for styling.
///
/// # API Versions
///
/// ## Legacy API (simple)
/// Use `variant` prop for common button styles:
/// ```ignore
/// <Button variant=ButtonVariant::Primary>"Submit"</Button>
/// <Button variant=ButtonVariant::Danger>"Delete"</Button>
/// ```
///
/// ## Modern API (flexible)
/// Use `color` + `style_variant` for full control:
/// ```ignore
/// <Button color=ButtonColor::Primary style_variant=ButtonStyleVariant::Solid>"Solid Primary"</Button>
/// <Button color=ButtonColor::Danger style_variant=ButtonStyleVariant::Outlined>"Outlined Danger"</Button>
/// ```
///
/// # Props
///
/// ## Style Props
/// - `variant` - Legacy variant (Primary, Secondary, Outline, Ghost, Danger, Link, Blue, Dashed, Text)
/// - `color` - Modern color (Default, Primary, Danger, + preset colors)
/// - `style_variant` - Modern style variant (Solid, Outlined, Dashed, Filled, Text, Link)
/// - `size` - Size (Xs, Sm, Md, Lg, Xl)
/// - `shape` - Shape (Default, Circle, Round)
/// - `ghost` - Transparent background with colored border/text
/// - `block` - Full width button
/// - `danger` - Apply danger styling (shorthand for color=Danger)
///
/// ## State Props
/// - `disabled` - Disabled state
/// - `loading` - Loading state with spinner
///
/// ## Icon Props
/// - `icon` - Icon content (HTML/SVG string)
/// - `icon_position` - Icon position (Start, End)
/// - `icon_only` - Square aspect ratio for icon-only buttons
///
/// ## Link Props
/// - `href` - Render as `<a>` tag with this URL
/// - `target` - Link target (_blank, _self, etc.)
///
/// ## Other Props
/// - `html_type` - HTML button type (Button, Submit, Reset)
/// - `class` - Additional CSS classes
/// - `aria_label` - Accessible label
/// - `on_click` - Click handler
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::{Button, ButtonVariant, ButtonSize, ButtonShape};
///
/// view! {
///     // Primary button
///     <Button variant=ButtonVariant::Primary>"Submit"</Button>
///
///     // Large danger button
///     <Button variant=ButtonVariant::Danger size=ButtonSize::Lg>"Delete"</Button>
///
///     // Round button with icon
///     <Button shape=ButtonShape::Round icon="🔍">"Search"</Button>
///
///     // Ghost button
///     <Button ghost=true>"Ghost"</Button>
///
///     // Link button
///     <Button href="/home">"Go Home"</Button>
///
///     // Loading button
///     <Button loading=true>"Processing..."</Button>
/// }
/// ```
#[component]
pub fn Button(
    // =========================================================================
    // STYLE PROPS
    // =========================================================================

    /// Legacy button variant (mutually exclusive with color/style_variant).
    #[prop(optional, into)]
    variant: Option<ButtonVariant>,

    /// Modern color prop (use with style_variant for full control).
    #[prop(optional, into)]
    color: Option<ButtonColor>,

    /// Modern style variant prop (use with color for full control).
    #[prop(optional, into)]
    style_variant: Option<ButtonStyleVariant>,

    /// Button size.
    #[prop(optional, into)]
    size: Option<ButtonSize>,

    /// Button shape (Default, Circle, Round).
    #[prop(optional, into)]
    shape: Option<ButtonShape>,

    /// Ghost mode - transparent background with colored border/text.
    #[prop(optional)]
    ghost: bool,

    /// Block mode - full width button.
    #[prop(optional)]
    block: bool,

    /// Danger styling shorthand (equivalent to color=Danger).
    #[prop(optional)]
    danger: bool,

    // =========================================================================
    // STATE PROPS
    // =========================================================================

    /// Whether the button is disabled.
    #[prop(optional)]
    disabled: bool,

    /// Whether the button is in loading state.
    #[prop(optional)]
    loading: bool,

    // =========================================================================
    // ICON PROPS
    // =========================================================================

    /// Icon content (HTML/SVG string or emoji).
    #[prop(optional, into)]
    icon: Option<String>,

    /// Icon position (Start or End).
    #[prop(optional, into)]
    icon_position: Option<IconPosition>,

    /// Whether this is an icon-only button (square aspect ratio).
    #[prop(optional)]
    icon_only: bool,

    // =========================================================================
    // LINK PROPS
    // =========================================================================

    /// URL to navigate to (renders as <a> tag).
    #[prop(optional, into)]
    href: Option<String>,

    /// Link target (_blank, _self, etc.).
    #[prop(optional, into)]
    target: Option<String>,

    // =========================================================================
    // OTHER PROPS
    // =========================================================================

    /// HTML button type attribute (button, submit, reset).
    #[prop(optional, into)]
    html_type: Option<HtmlButtonType>,

    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,

    /// Accessible label for screen readers.
    #[prop(optional, into)]
    aria_label: Option<String>,

    /// Title/tooltip text.
    #[prop(optional, into)]
    title: Option<String>,

    /// Tab index for keyboard navigation.
    #[prop(optional, into)]
    tab_index: Option<i32>,

    /// Click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,

    /// Button content.
    children: Children,
) -> impl IntoView {
    // Get theme context for design system
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.design_system().as_str())
        .unwrap_or("ant");

    // Get template from context
    #[cfg(feature = "template")]
    let template = use_button_template();

    // =========================================================================
    // RESOLVE STYLE PROPS
    // =========================================================================

    // Resolve color and style_variant from props
    // Priority: explicit color/style_variant > variant > defaults
    let (resolved_color, resolved_style) = if color.is_some() || style_variant.is_some() {
        // Modern API: use explicit color/style_variant
        let c = if danger {
            ButtonColor::Danger
        } else {
            color.unwrap_or(ButtonColor::Default)
        };
        let s = style_variant.unwrap_or(ButtonStyleVariant::Outlined);
        (c, s)
    } else if let Some(v) = variant {
        // Legacy API: convert variant to color + style
        let (c, s) = v.to_modern();
        let c = if danger { ButtonColor::Danger } else { c };
        (c, s)
    } else {
        // Default: Primary + Solid
        let c = if danger { ButtonColor::Danger } else { ButtonColor::Primary };
        (c, ButtonStyleVariant::Solid)
    };

    let size = size.unwrap_or_default();
    let shape = shape.unwrap_or_default();
    let icon_position = icon_position.unwrap_or_default();
    let html_type = html_type.unwrap_or_default();

    // =========================================================================
    // BUILD CSS CLASSES
    // =========================================================================

    let btn_prefix = format!("fx-btn-{}", design_system);

    // Base class
    let base_class = btn_prefix.clone();

    // Color class
    let color_class = resolved_color.class(&btn_prefix);

    // Style variant class
    let style_class = resolved_style.class(&btn_prefix);

    // Size class
    let size_class = size.class(&btn_prefix);

    // Shape class (empty for Default)
    let shape_class = shape.class(&btn_prefix);

    // Icon position class (empty for Start)
    let icon_pos_class = icon_position.class(&btn_prefix);

    // Template class (if template feature enabled)
    #[cfg(feature = "template")]
    let template_class = {
        let prefix = btn_prefix.clone();
        move || {
            let tmpl = template.get();
            format!("{}-template-{}", prefix, tmpl.as_str())
        }
    };

    // State classes
    let state_classes = {
        let prefix = btn_prefix.clone();
        let mut classes = Vec::new();

        if disabled || loading {
            classes.push(format!("{}-disabled", prefix));
        }
        if loading {
            classes.push(format!("{}-loading", prefix));
        }
        if block {
            classes.push(format!("{}-block", prefix));
        }
        if ghost {
            classes.push(format!("{}-ghost", prefix));
        }
        if icon_only {
            classes.push(format!("{}-icon-only", prefix));
        }
        if icon.is_some() && !icon_only {
            classes.push(format!("{}-with-icon", prefix));
        }

        classes.join(" ")
    };

    // Combine all classes
    #[cfg(feature = "template")]
    let combined_class = move || {
        let mut parts = vec![
            base_class.clone(),
            color_class.clone(),
            style_class.clone(),
            size_class.clone(),
            template_class(),
        ];
        if !shape_class.is_empty() {
            parts.push(shape_class.clone());
        }
        if !icon_pos_class.is_empty() {
            parts.push(icon_pos_class.clone());
        }
        if !state_classes.is_empty() {
            parts.push(state_classes.clone());
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    #[cfg(not(feature = "template"))]
    let combined_class = {
        let mut parts = vec![
            base_class.clone(),
            color_class.clone(),
            style_class.clone(),
            size_class.clone(),
        ];
        if !shape_class.is_empty() {
            parts.push(shape_class.clone());
        }
        if !icon_pos_class.is_empty() {
            parts.push(icon_pos_class.clone());
        }
        if !state_classes.is_empty() {
            parts.push(state_classes.clone());
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // =========================================================================
    // EVENT HANDLERS
    // =========================================================================

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

    // =========================================================================
    // INNER CLASSES
    // =========================================================================

    let spinner_class = format!("{}-spinner", btn_prefix);
    let icon_class = format!("{}-icon", btn_prefix);
    let content_class = format!("{}-content", btn_prefix);

    // Clone for use in view
    let icon_clone = icon.clone();
    let icon_for_view = icon.clone();
    let spinner_class_clone = spinner_class.clone();
    let icon_class_clone = icon_class.clone();

    // =========================================================================
    // RENDER ICON
    // =========================================================================

    let render_icon = move || {
        icon_clone.as_ref().map(|ico| {
            view! {
                <span class=icon_class_clone.clone() inner_html=ico.clone()></span>
            }
        })
    };

    // =========================================================================
    // RENDER ELEMENT
    // =========================================================================

    // Determine if we should render as link
    let is_link = href.is_some() && !disabled && !loading;

    if is_link {
        // Render as <a> tag
        let href_value = href.unwrap_or_default();
        let target_value = target.clone();
        let rel_value = if target.as_deref() == Some("_blank") {
            Some("noopener noreferrer".to_string())
        } else {
            None
        };

        view! {
            <a
                class=combined_class
                href=href_value
                target=target_value
                rel=rel_value
                aria-label=aria_label
                title=title
                tabindex=tab_index.unwrap_or(0)
                on:click=handle_click
            >
                {move || {
                    if loading {
                        view! {
                            <span class=spinner_class.clone()></span>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
                {render_icon}
                <span class=content_class.clone()>
                    {children()}
                </span>
            </a>
        }.into_any()
    } else {
        // Render as <button> tag
        let tab_idx = if disabled { -1 } else { tab_index.unwrap_or(0) };

        view! {
            <button
                type=html_type.as_str()
                class=combined_class
                disabled=disabled || loading
                aria-disabled=move || if disabled || loading { Some("true") } else { None }
                aria-busy=move || if loading { Some("true") } else { None }
                aria-label=aria_label
                title=title
                tabindex=tab_idx
                on:click=handle_click
            >
                {move || {
                    if loading {
                        view! {
                            <span class=spinner_class_clone.clone()></span>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
                {move || {
                    icon_for_view.as_ref().map(|ico| {
                        view! {
                            <span class=icon_class.clone() inner_html=ico.clone()></span>
                        }
                    })
                }}
                <span class=content_class>
                    {children()}
                </span>
            </button>
        }.into_any()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_variant_class() {
        let variant = ButtonVariant::Primary;
        assert_eq!(variant.class("fx-btn"), "fx-btn-primary");
    }

    #[test]
    fn test_button_size_class() {
        let size = ButtonSize::Lg;
        assert_eq!(size.class("fx-btn"), "fx-btn-lg");
    }

    #[test]
    fn test_button_color_class() {
        let color = ButtonColor::Primary;
        assert_eq!(color.class("fx-btn"), "fx-btn-color-primary");
    }

    #[test]
    fn test_button_style_variant_class() {
        let style = ButtonStyleVariant::Solid;
        assert_eq!(style.class("fx-btn"), "fx-btn-variant-solid");
    }

    #[test]
    fn test_button_shape_class() {
        assert_eq!(ButtonShape::Default.class("fx-btn"), "");
        assert_eq!(ButtonShape::Circle.class("fx-btn"), "fx-btn-circle");
        assert_eq!(ButtonShape::Round.class("fx-btn"), "fx-btn-round");
    }

    #[test]
    fn test_icon_position_class() {
        assert_eq!(IconPosition::Start.class("fx-btn"), "");
        assert_eq!(IconPosition::End.class("fx-btn"), "fx-btn-icon-end");
    }
}
