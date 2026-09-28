//! Tooltip Leptos component.

use leptos::prelude::*;
use super::types::{TooltipPlacement, TooltipTrigger};
use crate::try_use_theme;

/// Tooltip component.
///
/// Simple hover hint that appears on mouse hover.
///
/// # Props
///
/// - `title` - Tooltip content
/// - `placement` - Tooltip placement
/// - `trigger` - How to trigger the tooltip
/// - `open` - Control open state
/// - `children` - Trigger element
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::data::Tooltip;
///
/// view! {
///     <Tooltip title="This is a tooltip">
///         <button>"Hover me"</button>
///     </Tooltip>
/// }
/// ```
#[component]
pub fn Tooltip(
    /// Tooltip content.
    #[prop(into)]
    title: String,
    /// Tooltip placement.
    #[prop(optional, into)]
    placement: Option<TooltipPlacement>,
    /// Trigger type.
    #[prop(optional, into)]
    trigger: Option<TooltipTrigger>,
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
    let trigger = trigger.unwrap_or_default();
    let arrow = arrow.unwrap_or(true);

    // Internal state for hover/click control
    let is_visible = RwSignal::new(false);

    // Build CSS classes
    let tooltip_prefix = format!("fx-tooltip-{}", design_system);
    let placement_class = placement.class(&tooltip_prefix);

    let wrapper_class = format!("{}-wrapper", tooltip_prefix);

    // Clone for closures
    let tooltip_prefix_for_class = tooltip_prefix.clone();
    let tooltip_prefix_for_arrow = tooltip_prefix.clone();
    let tooltip_prefix_for_inner = tooltip_prefix.clone();

    let tooltip_class = move || {
        let mut parts = vec![tooltip_prefix_for_class.clone(), placement_class.clone()];

        // Check visibility
        let visible = open.map(|o| o.get()).unwrap_or_else(|| is_visible.get());
        if visible {
            parts.push(format!("{}-open", tooltip_prefix_for_class));
        } else {
            parts.push(format!("{}-hidden", tooltip_prefix_for_class));
        }

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Event handlers based on trigger type
    let on_mouse_enter = move |_| {
        if matches!(trigger, TooltipTrigger::Hover) && open.is_none() {
            is_visible.set(true);
        }
    };

    let on_mouse_leave = move |_| {
        if matches!(trigger, TooltipTrigger::Hover) && open.is_none() {
            is_visible.set(false);
        }
    };

    let on_focus = move |_| {
        if matches!(trigger, TooltipTrigger::Focus) && open.is_none() {
            is_visible.set(true);
        }
    };

    let on_blur = move |_| {
        if matches!(trigger, TooltipTrigger::Focus) && open.is_none() {
            is_visible.set(false);
        }
    };

    let on_click = move |_| {
        if matches!(trigger, TooltipTrigger::Click) && open.is_none() {
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
            <div class=tooltip_class role="tooltip">
                {if arrow {
                    Some(view! {
                        <div class=format!("{}-arrow", tooltip_prefix_for_arrow)></div>
                    })
                } else {
                    None
                }}
                <div class=format!("{}-inner", tooltip_prefix_for_inner)>
                    {title.clone()}
                </div>
            </div>
        </div>
    }
}
