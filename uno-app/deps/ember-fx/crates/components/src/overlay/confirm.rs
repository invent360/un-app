//! Confirmation dialog component.

use leptos::prelude::*;
use super::types::{ConfirmType, ModalSize};
use super::icons::ConfirmIcon;
use crate::try_use_theme;
use ember_fx_utils::a11y::{generate_id, is_dismiss_key, FOCUSABLE_SELECTOR};

/// ConfirmModal component.
///
/// A specialized modal for confirmation dialogs with type-specific icons.
///
/// # Types
///
/// - `Info` - Blue info icon, single OK button
/// - `Success` - Green check icon, single OK button
/// - `Warning` - Orange exclamation icon, single OK button
/// - `Error` - Red X icon, single OK button
/// - `Confirm` - Orange exclamation icon, OK + Cancel buttons
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::overlay::{ConfirmModal, ConfirmType};
///
/// let open = RwSignal::new(false);
///
/// view! {
///     <button on:click=move |_| open.set(true)>"Delete"</button>
///     <ConfirmModal
///         open=Signal::derive(move || open.get())
///         confirm_type=ConfirmType::Confirm
///         title="Delete this item?"
///         content="This action cannot be undone."
///         ok_text="Delete"
///         ok_danger=true
///         on_ok=Callback::new(move |_| {
///             // Handle delete
///             open.set(false);
///         })
///         on_cancel=Callback::new(move |_| open.set(false))
///     />
/// }
/// ```
#[component]
pub fn ConfirmModal(
    /// Whether the modal is open.
    #[prop(into)]
    open: Signal<bool>,
    /// Confirmation type (determines icon and button layout).
    #[prop(optional)]
    confirm_type: Option<ConfirmType>,
    /// Dialog title.
    #[prop(into)]
    title: String,
    /// Dialog content/description.
    #[prop(optional, into)]
    content: Option<String>,
    /// Whether to center the modal vertically.
    #[prop(optional)]
    centered: bool,
    /// Custom width.
    #[prop(optional, into)]
    width: Option<String>,
    /// Whether to show close button.
    #[prop(optional)]
    closable: Option<bool>,
    /// Whether clicking mask closes the modal.
    #[prop(optional)]
    mask_closable: Option<bool>,
    /// Whether keyboard (Escape) closes the modal.
    #[prop(optional)]
    keyboard: Option<bool>,
    /// OK button text.
    #[prop(optional, into)]
    ok_text: Option<String>,
    /// Cancel button text.
    #[prop(optional, into)]
    cancel_text: Option<String>,
    /// Whether OK button is danger styled.
    #[prop(optional)]
    ok_danger: bool,
    /// Whether OK button shows loading state.
    #[prop(optional)]
    ok_loading: bool,
    /// Whether to show cancel button (overrides type default).
    #[prop(optional)]
    show_cancel: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// OK button callback.
    #[prop(optional, into)]
    on_ok: Option<Callback<()>>,
    /// Cancel button callback.
    #[prop(optional, into)]
    on_cancel: Option<Callback<()>>,
    /// Close callback (called after OK or Cancel).
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let confirm_type = confirm_type.unwrap_or_default();
    let closable = closable.unwrap_or(false);
    let mask_closable = mask_closable.unwrap_or(false);
    let keyboard = keyboard.unwrap_or(true);
    let show_cancel = show_cancel.unwrap_or_else(|| confirm_type.has_cancel());

    // Build CSS classes
    let modal_prefix = format!("fx-modal-{}", design_system);
    let btn_prefix = format!("fx-btn-{}", design_system);

    // Icon class
    let icon_class = confirm_type.icon_class(&modal_prefix);

    // Generate unique IDs
    let modal_id = generate_id("confirm-modal");
    let title_id = format!("{}-title", modal_id);
    let title_id_for_aria = title_id.clone();

    // Clone for closures
    let on_close_for_mask = on_close.clone();
    let on_close_for_key = on_close.clone();
    let on_close_for_ok = on_close.clone();
    let on_close_for_cancel = on_close.clone();

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
        if let Some(ref cb) = on_close_for_ok {
            cb.run(());
        }
    };

    // Handle Cancel click
    let handle_cancel = move |_| {
        if let Some(ref cb) = on_cancel {
            cb.run(());
        }
        if let Some(ref cb) = on_close_for_cancel {
            cb.run(());
        }
    };

    // Handle close button
    let handle_close = move |_| {
        if let Some(ref cb) = on_close {
            cb.run(());
        }
    };

    // Handle keyboard events
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

    // Width style
    let width_style = width.map(|w| format!("width: {};", w));

    // Focus management
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            if open.get() {
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

    // Button text
    let ok_text = ok_text.unwrap_or_else(|| confirm_type.default_ok_text().to_string());
    let cancel_text = cancel_text.unwrap_or_else(|| confirm_type.default_cancel_text().to_string());

    // Class names
    let mask_class = format!("{}-mask", modal_prefix);
    let wrap_class = if centered {
        format!("{}-wrap {}-wrap-centered", modal_prefix, modal_prefix)
    } else {
        format!("{}-wrap", modal_prefix)
    };

    let modal_class = {
        let mut parts = vec![
            modal_prefix.clone(),
            format!("{}-confirm", modal_prefix),
        ];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let body_class = format!("{}-body", modal_prefix);
    let close_class = format!("{}-close", modal_prefix);

    view! {
        <div
            class=move || {
                if open.get() {
                    format!("{}-root {}-open", modal_prefix, modal_prefix)
                } else {
                    format!("{}-root", modal_prefix)
                }
            }
            style=move || {
                if open.get() { None } else { Some("display: none;") }
            }
        >
            // Mask
            <div
                class=mask_class.clone()
                on:click=handle_mask_click
            />

            // Wrap
            <div class=wrap_class.clone()>
                // Modal
                <div
                    class=modal_class.clone()
                    style=width_style.clone()
                    role="dialog"
                    aria-modal="true"
                    aria-labelledby=title_id_for_aria
                    on:keydown=handle_keydown
                    tabindex="-1"
                >
                    // Close button (if closable)
                    {if closable {
                        Some(view! {
                            <button
                                class=close_class.clone()
                                on:click=handle_close
                                aria-label="Close"
                                type="button"
                            >
                                "×"
                            </button>
                        })
                    } else {
                        None
                    }}

                    // Body
                    <div class=body_class.clone()>
                        <div class=format!("{}-confirm-body-wrapper", modal_prefix)>
                            // Icon
                            <span class=format!("{}-confirm-icon {}", modal_prefix, icon_class)>
                                <ConfirmIcon confirm_type=confirm_type />
                            </span>

                            // Content
                            <div class=format!("{}-confirm-paragraph", modal_prefix)>
                                <span
                                    class=format!("{}-confirm-title", modal_prefix)
                                    id=title_id
                                >
                                    {title.clone()}
                                </span>
                                {content.clone().map(|c| view! {
                                    <div class=format!("{}-confirm-content", modal_prefix)>
                                        {c}
                                    </div>
                                })}
                            </div>
                        </div>

                        // Buttons
                        <div class=format!("{}-confirm-btns", modal_prefix)>
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
                                    let mut base = format!("{} {}-primary", btn_prefix, btn_prefix);
                                    if ok_danger {
                                        base = format!("{} {}-danger", btn_prefix, btn_prefix);
                                    }
                                    if ok_loading {
                                        base = format!("{} {}-loading", base, btn_prefix);
                                    }
                                    base
                                }
                                on:click=handle_ok
                                disabled=ok_loading
                                type="button"
                            >
                                {if ok_loading {
                                    view! {
                                        <span class="fx-btn-loading-icon">"..."</span>
                                    }.into_any()
                                } else {
                                    view! { <span>{ok_text.clone()}</span> }.into_any()
                                }}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
