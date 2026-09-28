//! Modal/Dialog Leptos component.

use leptos::prelude::*;
use super::types::ModalSize;
use crate::try_use_theme;
use ember_fx_utils::a11y::{generate_id, is_dismiss_key, FOCUSABLE_SELECTOR};

/// Modal component.
///
/// An overlay dialog for displaying content.
///
/// # Props
///
/// - `open` - Whether the modal is visible
/// - `title` - Modal title
/// - `size` - Modal size (Small, Default, Large, FullScreen)
/// - `closable` - Whether to show close button
/// - `mask_closable` - Whether clicking mask closes modal
/// - `centered` - Whether to center the modal vertically
/// - `footer` - Footer content (buttons)
/// - `on_close` - Close callback
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::layout::Modal;
///
/// let open = RwSignal::new(false);
///
/// view! {
///     <Modal
///         open=open
///         title="Confirm Action"
///         on_close=move || open.set(false)
///     >
///         <p>"Are you sure you want to proceed?"</p>
///     </Modal>
/// }
/// ```
#[component]
pub fn Modal(
    /// Whether the modal is open.
    #[prop(into)]
    open: Signal<bool>,
    /// Modal title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Modal size.
    #[prop(optional, into)]
    size: Option<ModalSize>,
    /// Whether to show close button.
    #[prop(optional)]
    closable: Option<bool>,
    /// Whether clicking the mask closes the modal.
    #[prop(optional)]
    mask_closable: Option<bool>,
    /// Whether to center the modal vertically.
    #[prop(optional)]
    centered: bool,
    /// Footer content (typically buttons).
    #[prop(optional, into)]
    footer: Option<String>,
    /// Custom width (overrides size).
    #[prop(optional, into)]
    width: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Close callback.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    /// Modal content.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let closable = closable.unwrap_or(true);
    let mask_closable = mask_closable.unwrap_or(true);

    // Generate unique IDs for accessibility
    let modal_id = generate_id("modal");
    let title_id = format!("{}-title", modal_id);

    // Build CSS classes
    let modal_prefix = format!("fx-modal-{}", design_system);
    let size_class = size.class(&modal_prefix);

    // Class names
    let wrap_class = format!("{}-wrap", modal_prefix);
    let mask_class = format!("{}-mask", modal_prefix);
    let content_class = format!("{}-content", modal_prefix);
    let header_class = format!("{}-header", modal_prefix);
    let title_class = format!("{}-title", modal_prefix);
    let close_class = format!("{}-close", modal_prefix);
    let body_class = format!("{}-body", modal_prefix);
    let footer_class = format!("{}-footer", modal_prefix);

    // Clone for closures
    let modal_prefix_for_class = modal_prefix.clone();
    let on_close_for_mask = on_close.clone();

    let combined_class = move || {
        let mut parts = vec![modal_prefix_for_class.clone(), size_class.clone()];
        if centered {
            parts.push(format!("{}-centered", modal_prefix_for_class));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
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

    // Custom width style
    let width_style = width.clone().map(|w| format!("width: {};", w));

    // Focus management: focus the dialog when it opens
    let dialog_id = modal_id.clone();
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            if open.get() {
                // Focus the dialog container or first focusable element
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    // Try to find the dialog by its role and focus it
                    let selector = format!("[role='dialog']");
                    if let Ok(Some(dialog)) = document.query_selector(&selector) {
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

    // Suppress unused variable warning
    let _ = dialog_id;

    view! {
        <div
            class=move || {
                if open.get() {
                    format!("{} {}-open", wrap_class, modal_prefix)
                } else {
                    wrap_class.clone()
                }
            }
            style=move || {
                if open.get() { None } else { Some("display: none;") }
            }
        >
            // Mask/backdrop
            <div
                class=mask_class.clone()
                on:click=handle_mask_click
            ></div>

            // Modal dialog
            <div
                class=combined_class
                style=width_style.clone()
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

                    // Footer
                    {footer.clone().map(|f| view! {
                        <div class=footer_class.clone()>{f}</div>
                    })}
                </div>
            </div>
        </div>
    }
}
