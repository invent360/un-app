//! Tag/Chip Leptos component.

use leptos::prelude::*;
use super::types::TagColor;
use crate::try_use_theme;

/// Tag component.
///
/// Labels for categorization or status display.
///
/// # Props
///
/// - `color` - Tag color variant
/// - `closable` - Whether the tag can be closed/removed
/// - `icon` - Icon to display before text
/// - `bordered` - Whether to show border
/// - `on_close` - Close handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::notification::Tag;
///
/// view! {
///     <Tag color=TagColor::Blue closable=true>
///         "Feature"
///     </Tag>
/// }
/// ```
#[component]
pub fn Tag(
    /// Tag color variant.
    #[prop(optional, into)]
    color: Option<TagColor>,
    /// Whether the tag can be closed.
    #[prop(optional)]
    closable: bool,
    /// Icon content.
    #[prop(optional, into)]
    icon: Option<String>,
    /// Whether to show border.
    #[prop(optional)]
    bordered: Option<bool>,
    /// Custom background color.
    #[prop(optional, into)]
    custom_color: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Close handler.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    /// Tag content.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let color = color.unwrap_or_default();
    let bordered = bordered.unwrap_or(true);

    // Closed state
    let is_closed = RwSignal::new(false);

    // Build CSS classes
    let tag_prefix = format!("fx-tag-{}", design_system);
    let color_class = color.class(&tag_prefix);

    // Class names (before closure captures)
    let icon_class = format!("{}-icon", tag_prefix);
    let close_class = format!("{}-close", tag_prefix);

    // Clone for closure
    let tag_prefix_for_class = tag_prefix.clone();

    let combined_class = move || {
        let mut parts = vec![tag_prefix_for_class.clone(), color_class.clone()];
        if !bordered {
            parts.push(format!("{}-borderless", tag_prefix_for_class));
        }
        if closable {
            parts.push(format!("{}-closable", tag_prefix_for_class));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Handle close
    let handle_close = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        is_closed.set(true);
        if let Some(ref cb) = on_close {
            cb.run(());
        }
    };

    // Custom style
    let custom_style = custom_color.clone().map(|c| {
        format!("background-color: {}; border-color: {}", c, c)
    });

    // Render children once
    let children_view = children();

    view! {
        <span
            class=move || {
                if is_closed.get() {
                    "fx-tag-hidden".to_string()
                } else {
                    combined_class()
                }
            }
            style=move || {
                if is_closed.get() {
                    Some("display: none;".to_string())
                } else {
                    custom_style.clone()
                }
            }
        >
            {icon.clone().map(|i| view! {
                <span class=icon_class.clone()>{i}</span>
            })}
            {children_view}
            {if closable {
                Some(view! {
                    <span
                        class=close_class.clone()
                        on:click=handle_close
                        role="button"
                        aria-label="Remove"
                    >
                        "×"
                    </span>
                })
            } else {
                None
            }}
        </span>
    }
}

/// CheckableTag component.
///
/// A tag that can be checked/selected.
#[component]
pub fn CheckableTag(
    /// Whether the tag is checked.
    #[prop(optional, into)]
    checked: Option<RwSignal<bool>>,
    /// Default checked state.
    #[prop(optional)]
    default_checked: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<bool>>,
    /// Tag content.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Create internal state if not controlled
    let internal_checked = checked.unwrap_or_else(|| RwSignal::new(default_checked));

    // Build CSS classes
    let tag_prefix = format!("fx-tag-{}", design_system);

    let combined_class = move || {
        let mut parts = vec![tag_prefix.clone(), format!("{}-checkable", tag_prefix)];
        if internal_checked.get() {
            parts.push(format!("{}-checkable-checked", tag_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Handle click
    let handle_click = move |_| {
        let new_value = !internal_checked.get();
        internal_checked.set(new_value);
        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    view! {
        <span
            class=combined_class
            on:click=handle_click
            role="checkbox"
            aria-checked=move || internal_checked.get().to_string()
        >
            {children()}
        </span>
    }
}
