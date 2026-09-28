//! Popover Leptos component.

use leptos::prelude::*;
use super::types::{PopoverPlacement, PopoverTrigger};
use crate::try_use_theme;

/// Popover component.
///
/// Rich hover content card that appears on hover/click.
///
/// # Props
///
/// - `title` - Popover title
/// - `content` - Popover content
/// - `placement` - Popover placement
/// - `trigger` - How to trigger the popover
/// - `open` - Control open state
/// - `children` - Trigger element
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::data::Popover;
///
/// view! {
///     <Popover title="Title" content="This is popover content">
///         <button>"Click me"</button>
///     </Popover>
/// }
/// ```
#[component]
pub fn Popover(
    /// Popover title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Popover content.
    #[prop(into)]
    content: String,
    /// Popover placement.
    #[prop(optional, into)]
    placement: Option<PopoverPlacement>,
    /// Trigger type.
    #[prop(optional, into)]
    trigger: Option<PopoverTrigger>,
    /// Control open state.
    #[prop(optional, into)]
    open: Option<Signal<bool>>,
    /// Arrow visibility.
    #[prop(optional)]
    arrow: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Trigger element.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let placement = placement.unwrap_or_default();
    let trigger = trigger.unwrap_or(PopoverTrigger::Click);
    let arrow = arrow.unwrap_or(true);

    // Internal state for hover/click control
    let is_visible = RwSignal::new(false);

    // Build CSS classes
    let popover_prefix = format!("fx-popover-{}", design_system);
    let placement_class = placement.class(&popover_prefix);

    let wrapper_class = format!("{}-wrapper", popover_prefix);

    // Clone for closures
    let popover_prefix_for_class = popover_prefix.clone();
    let popover_prefix_for_arrow = popover_prefix.clone();
    let popover_prefix_for_inner = popover_prefix.clone();
    let popover_prefix_for_title = popover_prefix.clone();
    let popover_prefix_for_content = popover_prefix.clone();

    let popover_class = move || {
        let mut parts = vec![popover_prefix_for_class.clone(), placement_class.clone()];

        // Check visibility
        let visible = open.map(|o| o.get()).unwrap_or_else(|| is_visible.get());
        if visible {
            parts.push(format!("{}-open", popover_prefix_for_class));
        } else {
            parts.push(format!("{}-hidden", popover_prefix_for_class));
        }

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Event handlers based on trigger type
    let on_mouse_enter = move |_| {
        if matches!(trigger, PopoverTrigger::Hover) && open.is_none() {
            is_visible.set(true);
        }
    };

    let on_mouse_leave = move |_| {
        if matches!(trigger, PopoverTrigger::Hover) && open.is_none() {
            is_visible.set(false);
        }
    };

    let on_focus = move |_| {
        if matches!(trigger, PopoverTrigger::Focus) && open.is_none() {
            is_visible.set(true);
        }
    };

    let on_blur = move |_| {
        if matches!(trigger, PopoverTrigger::Focus) && open.is_none() {
            is_visible.set(false);
        }
    };

    let on_click = move |_| {
        if matches!(trigger, PopoverTrigger::Click) && open.is_none() {
            is_visible.update(|v| *v = !*v);
        }
    };

    view! {
        <div
            class=wrapper_class
            on:mouseenter=on_mouse_enter
            on:mouseleave=on_mouse_leave
            on:focus=on_focus
            on:blur=on_blur
            on:click=on_click
        >
            {children()}
            <div class=popover_class>
                {if arrow {
                    Some(view! {
                        <div class=format!("{}-arrow", popover_prefix_for_arrow)></div>
                    })
                } else {
                    None
                }}
                <div class=format!("{}-inner", popover_prefix_for_inner)>
                    {title.clone().map(|t| view! {
                        <div class=format!("{}-title", popover_prefix_for_title)>{t}</div>
                    })}
                    <div class=format!("{}-content", popover_prefix_for_content)>
                        {content.clone()}
                    </div>
                </div>
            </div>
        </div>
    }
}
