//! Modal/Dialog Leptos component.

use leptos::prelude::*;
use super::types::ModalSize;
use super::icons::CloseOutlined;
use crate::try_use_theme;
use ember_fx_utils::a11y::{generate_id, is_dismiss_key, FOCUSABLE_SELECTOR};

/// Modal component.
///
/// An overlay dialog for displaying content with full Ant Design styling.
///
/// # Props
///
/// - `open` - Whether the modal is visible
/// - `title` - Modal title
/// - `size` - Modal size (Small, Default, Large, ExtraLarge, FullScreen)
/// - `closable` - Whether to show close button
/// - `mask_closable` - Whether clicking mask closes modal
/// - `centered` - Whether to center the modal vertically
/// - `footer` - Custom footer content
/// - `ok_text` - Text for OK button (when using default footer)
/// - `cancel_text` - Text for Cancel button (when using default footer)
/// - `confirm_loading` - Loading state for OK button
/// - `on_close` - Close callback
/// - `on_ok` - OK button callback
/// - `on_cancel` - Cancel button callback
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::overlay::Modal;
///
/// let open = RwSignal::new(false);
///
/// view! {
///     <button on:click=move |_| open.set(true)>"Open Modal"</button>
///     <Modal
///         open=Signal::derive(move || open.get())
///         title="Confirm Action"
///         on_close=Callback::new(move |_| open.set(false))
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
    #[prop(optional)]
    size: Option<ModalSize>,
    /// Whether to show close button.
    #[prop(optional)]
    closable: Option<bool>,
    /// Whether clicking the mask closes the modal.
    #[prop(optional)]
    mask_closable: Option<bool>,
    /// Whether to show the mask/backdrop.
    #[prop(optional)]
    mask: Option<bool>,
    /// Whether to center the modal vertically.
    #[prop(optional)]
    centered: bool,
    /// Whether keyboard events (Escape) close the modal.
    #[prop(optional)]
    keyboard: Option<bool>,
    /// Custom width (overrides size).
    #[prop(optional, into)]
    width: Option<String>,
    /// Z-index for the modal.
    #[prop(optional)]
    z_index: Option<u32>,
    /// Custom footer content. If None, uses default OK/Cancel buttons.
    /// Pass empty string to hide footer completely.
    #[prop(optional, into)]
    footer: Option<String>,
    /// OK button text (for default footer).
    #[prop(optional, into)]
    ok_text: Option<String>,
    /// Cancel button text (for default footer).
    #[prop(optional, into)]
    cancel_text: Option<String>,
    /// Whether OK button shows loading state.
    #[prop(optional)]
    confirm_loading: bool,
    /// Whether to show cancel button in default footer.
    #[prop(optional)]
    show_cancel: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Close callback.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    /// OK button callback.
    #[prop(optional, into)]
    on_ok: Option<Callback<()>>,
    /// Cancel button callback.
    #[prop(optional, into)]
    on_cancel: Option<Callback<()>>,
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
    let mask = mask.unwrap_or(true);
    let keyboard = keyboard.unwrap_or(true);
    let show_cancel = show_cancel.unwrap_or(true);

    // Generate unique IDs for accessibility
    let modal_id = generate_id("modal");
    let title_id = format!("{}-title", modal_id);

    // Build CSS classes
    let modal_prefix = format!("fx-modal-{}", design_system);
    let size_class = size.class(&modal_prefix);
    let btn_prefix = format!("fx-btn-{}", design_system);

    // Clone for closures
    let modal_prefix_clone = modal_prefix.clone();
    let modal_prefix_for_class = modal_prefix.clone();
    let on_close_for_mask = on_close.clone();
    let on_close_for_key = on_close.clone();
    let on_close_for_ok = on_close.clone();
    let on_close_for_cancel = on_close.clone();

    // Combined class for the modal
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

    // Wrap class (includes open state)
    let wrap_class = {
        let modal_prefix = modal_prefix.clone();
        move || {
            let base = format!("{}-wrap", modal_prefix);
            if centered {
                format!("{} {}-wrap-centered", base, modal_prefix)
            } else {
                base
            }
        }
    };

    // Root class with open state
    let root_class = {
        let modal_prefix = modal_prefix.clone();
        move || {
            if open.get() {
                format!("{}-root {}-open", modal_prefix, modal_prefix)
            } else {
                format!("{}-root", modal_prefix)
            }
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

    // Handle OK click
    let handle_ok = move |_| {
        if let Some(ref cb) = on_ok {
            cb.run(());
        }
        // Also close the modal after OK
        if let Some(ref cb) = on_close_for_ok {
            cb.run(());
        }
    };

    // Handle Cancel click
    let handle_cancel = move |_| {
        if let Some(ref cb) = on_cancel {
            cb.run(());
        }
        // Also close the modal
        if let Some(ref cb) = on_close_for_cancel {
            cb.run(());
        }
    };

    // Handle keyboard events (Escape to close, Tab trap)
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if !keyboard {
            return;
        }

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
    let width_style = width.map(|w| format!("width: {};", w));

    // Z-index style
    let z_index_style = z_index.map(|z| format!("z-index: {};", z));

    // Combined inline style
    let inline_style = move || {
        let mut styles = Vec::new();
        if let Some(ref w) = width_style {
            styles.push(w.clone());
        }
        if let Some(ref z) = z_index_style {
            styles.push(z.clone());
        }
        if styles.is_empty() {
            None
        } else {
            Some(styles.join(" "))
        }
    };

    // Focus management: focus the dialog when it opens
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            if open.get() {
                // Focus the dialog container
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

    // Class names
    let mask_class = format!("{}-mask", modal_prefix_clone);
    let content_class = format!("{}-content", modal_prefix_clone);
    let header_class = format!("{}-header", modal_prefix_clone);
    let title_class = format!("{}-title", modal_prefix_clone);
    let close_class = format!("{}-close", modal_prefix_clone);
    let body_class = format!("{}-body", modal_prefix_clone);
    let footer_class = format!("{}-footer", modal_prefix_clone);

    // Determine if we should show footer
    let has_custom_footer = footer.is_some();
    let hide_footer = footer.as_ref().map(|f| f.is_empty()).unwrap_or(false);
    let show_default_footer = !has_custom_footer && !hide_footer;

    // Button text
    let ok_text = ok_text.unwrap_or_else(|| "OK".to_string());
    let cancel_text = cancel_text.unwrap_or_else(|| "Cancel".to_string());

    view! {
        <div
            class=root_class
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
                    />
                })
            } else {
                None
            }}

            // Modal wrap
            <div class=wrap_class>
                // Modal dialog
                <div
                    class=combined_class
                    style=inline_style
                    role="dialog"
                    aria-modal="true"
                    aria-labelledby=title.as_ref().map(|_| title_id.clone())
                    on:keydown=handle_keydown
                    tabindex="-1"
                >
                    <div class=content_class.clone()>
                        // Close button
                        {if closable {
                            Some(view! {
                                <button
                                    class=close_class.clone()
                                    on:click=handle_close
                                    aria-label="Close"
                                    type="button"
                                >
                                    <CloseOutlined />
                                </button>
                            })
                        } else {
                            None
                        }}

                        // Header
                        {if title.is_some() {
                            Some(view! {
                                <div class=header_class.clone()>
                                    <div class=title_class.clone() id=title_id.clone()>
                                        {title.clone()}
                                    </div>
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
                        {if hide_footer {
                            None
                        } else if let Some(ref custom_footer) = footer {
                            Some(view! {
                                <div class=footer_class.clone()>{custom_footer.clone()}</div>
                            }.into_any())
                        } else if show_default_footer {
                            Some(view! {
                                <div class=footer_class.clone()>
                                    {if show_cancel {
                                        Some(view! {
                                            <button
                                                class=format!("{} {}-default", btn_prefix, btn_prefix)
                                                on:click=handle_cancel
                                                type="button"
                                            >
                                                {cancel_text.clone()}
                                            </button>
                                        })
                                    } else {
                                        None
                                    }}
                                    <button
                                        class=move || {
                                            let base = format!("{} {}-primary", btn_prefix, btn_prefix);
                                            if confirm_loading {
                                                format!("{} {}-loading", base, btn_prefix)
                                            } else {
                                                base
                                            }
                                        }
                                        on:click=handle_ok
                                        disabled=confirm_loading
                                        type="button"
                                    >
                                        {if confirm_loading {
                                            view! {
                                                <span class="fx-btn-loading-icon">"..."</span>
                                            }.into_any()
                                        } else {
                                            view! { <span>{ok_text.clone()}</span> }.into_any()
                                        }}
                                    </button>
                                </div>
                            }.into_any())
                        } else {
                            None
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
