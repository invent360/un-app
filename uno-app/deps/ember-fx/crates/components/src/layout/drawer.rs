//! Drawer Leptos component.

use leptos::prelude::*;
use super::types::{DrawerPlacement, DrawerSize};
use crate::try_use_theme;
use ember_fx_utils::a11y::{generate_id, is_dismiss_key, FOCUSABLE_SELECTOR};

/// Drawer component.
///
/// A slide-in panel from the edge of the screen.
///
/// # Props
///
/// - `open` - Whether the drawer is visible
/// - `title` - Drawer title
/// - `placement` - Where the drawer slides in from (Top, Right, Bottom, Left)
/// - `size` - Drawer size (Small, Default, Large)
/// - `closable` - Whether to show close button
/// - `mask_closable` - Whether clicking mask closes drawer
/// - `mask` - Whether to show mask/backdrop
/// - `on_close` - Close callback
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::layout::Drawer;
/// use ember_fx::components::layout::DrawerPlacement;
///
/// let open = RwSignal::new(false);
///
/// view! {
///     <Drawer
///         open=open
///         title="Settings"
///         placement=DrawerPlacement::Right
///         on_close=move || open.set(false)
///     >
///         <p>"Drawer content here"</p>
///     </Drawer>
/// }
/// ```
#[component]
pub fn Drawer(
    /// Whether the drawer is open.
    #[prop(into)]
    open: Signal<bool>,
    /// Drawer title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Drawer placement.
    #[prop(optional, into)]
    placement: Option<DrawerPlacement>,
    /// Drawer size.
    #[prop(optional, into)]
    size: Option<DrawerSize>,
    /// Custom width (for left/right drawers, overrides size).
    #[prop(optional, into)]
    width: Option<String>,
    /// Custom height (for top/bottom drawers, overrides size).
    #[prop(optional, into)]
    height: Option<String>,
    /// Whether to show close button.
    #[prop(optional)]
    closable: Option<bool>,
    /// Whether clicking the mask closes the drawer.
    #[prop(optional)]
    mask_closable: Option<bool>,
    /// Whether to show mask/backdrop.
    #[prop(optional)]
    mask: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Close callback.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    /// Drawer content.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let placement = placement.unwrap_or_default();
    let size = size.unwrap_or_default();
    let closable = closable.unwrap_or(true);
    let mask_closable = mask_closable.unwrap_or(true);
    let mask = mask.unwrap_or(true);

    // Generate unique IDs for accessibility
    let drawer_id = generate_id("drawer");
    let title_id = format!("{}-title", drawer_id);

    // Build CSS classes
    let drawer_prefix = format!("fx-drawer-{}", design_system);
    let placement_class = placement.class(&drawer_prefix);

    // Class names
    let wrap_class = format!("{}-wrap", drawer_prefix);
    let mask_class = format!("{}-mask", drawer_prefix);
    let content_class = format!("{}-content", drawer_prefix);
    let wrapper_class = format!("{}-wrapper", drawer_prefix);
    let header_class = format!("{}-header", drawer_prefix);
    let title_class = format!("{}-title", drawer_prefix);
    let close_class = format!("{}-close", drawer_prefix);
    let body_class = format!("{}-body", drawer_prefix);

    // Clone for closures
    let drawer_prefix_for_class = drawer_prefix.clone();
    let on_close_for_mask = on_close.clone();

    let combined_class = move || {
        let mut parts = vec![drawer_prefix_for_class.clone(), placement_class.clone()];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Calculate size style based on placement
    let size_style = match placement {
        DrawerPlacement::Left | DrawerPlacement::Right => {
            width.clone().unwrap_or_else(|| format!("{}px", size.width()))
        }
        DrawerPlacement::Top | DrawerPlacement::Bottom => {
            height.clone().unwrap_or_else(|| format!("{}px", size.height()))
        }
    };

    let content_style = match placement {
        DrawerPlacement::Left | DrawerPlacement::Right => {
            format!("width: {};", size_style)
        }
        DrawerPlacement::Top | DrawerPlacement::Bottom => {
            format!("height: {};", size_style)
        }
    };

    // Handle close
    let handle_close = move |_| {
        if let Some(ref cb) = on_close {
            cb.run(());
        }
    };

    // Handle mask click
    let handle_mask_click = move |_| {
        if mask_closable {
            if let Some(ref cb) = on_close_for_mask {
                cb.run(());
            }
        }
    };

    // Clone for keyboard handler
    let on_close_for_key = on_close.clone();

    // Handle keyboard events (Escape to close, Tab trap)
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();

        // Close on Escape
        if is_dismiss_key(&key) {
            if let Some(ref cb) = on_close_for_key {
                cb.run(());
            }
            return;
        }

        // Focus trap on Tab
        if key == "Tab" {
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::JsCast;

                if let Some(dialog) = ev.current_target() {
                    if let Ok(container) = dialog.dyn_into::<web_sys::HtmlElement>() {
                        let focusable = container.query_selector_all(FOCUSABLE_SELECTOR);
                        if let Ok(elements) = focusable {
                            let len = elements.length();
                            if len == 0 {
                                return;
                            }

                            let first = elements.get(0);
                            let last = elements.get(len - 1);

                            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                                let active = document.active_element();

                                if ev.shift_key() {
                                    // Shift+Tab from first element -> wrap to last
                                    if let (Some(first_el), Some(active_el)) = (&first, &active) {
                                        let active_node: &web_sys::Node = active_el.as_ref();
                                        if first_el.is_same_node(Some(active_node)) {
                                            ev.prevent_default();
                                            if let Some(last_el) = last {
                                                if let Ok(html_el) = last_el.dyn_into::<web_sys::HtmlElement>() {
                                                    let _ = html_el.focus();
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    // Tab from last element -> wrap to first
                                    if let (Some(last_el), Some(active_el)) = (&last, &active) {
                                        let active_node: &web_sys::Node = active_el.as_ref();
                                        if last_el.is_same_node(Some(active_node)) {
                                            ev.prevent_default();
                                            if let Some(first_el) = first {
                                                if let Ok(html_el) = first_el.dyn_into::<web_sys::HtmlElement>() {
                                                    let _ = html_el.focus();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    // Render children
    let children_view = children();

    // Focus management: focus the drawer when it opens
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            if open.get() {
                // Focus the drawer container
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    let selector = "[role='dialog']";
                    if let Ok(Some(dialog)) = document.query_selector(selector) {
                        if let Ok(html_el) = dialog.dyn_into::<web_sys::HtmlElement>() {
                            let _ = html_el.focus();
                        }
                    }
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = open.get();
        }
    });

    view! {
        <div
            class=move || {
                if open.get() {
                    format!("{} {}-open", wrap_class, drawer_prefix)
                } else {
                    wrap_class.clone()
                }
            }
            style=move || {
                if open.get() { None } else { Some("display: none;") }
            }
        >
            // Mask/backdrop
            {if mask {
                Some(view! {
                    <div
                        class=mask_class.clone()
                        on:click=handle_mask_click
                    ></div>
                })
            } else {
                None
            }}

            // Drawer content wrapper
            <div class=wrapper_class.clone()>
                <div
                    class=combined_class
                    style=content_style.clone()
                    role="dialog"
                    aria-modal="true"
                    aria-labelledby=title.as_ref().map(|_| title_id.clone())
                    on:keydown=handle_keydown
                    tabindex="-1"
                >
                    <div class=content_class.clone()>
                        // Header
                        {if title.is_some() || closable {
                            Some(view! {
                                <div class=header_class.clone()>
                                    <div class=title_class.clone() id=title_id.clone()>
                                        {title.clone()}
                                    </div>
                                    {if closable {
                                        Some(view! {
                                            <button
                                                class=close_class.clone()
                                                on:click=handle_close
                                                aria-label="Close"
                                            >
                                                "×"
                                            </button>
                                        })
                                    } else {
                                        None
                                    }}
                                </div>
                            })
                        } else {
                            None
                        }}

                        // Body
                        <div class=body_class.clone()>
                            {children_view}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
