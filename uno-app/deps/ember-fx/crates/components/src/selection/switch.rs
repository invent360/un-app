//! Switch/Toggle Leptos component.

use leptos::prelude::*;
use super::types::SelectionSize;
use crate::try_use_theme;

/// Switch component.
///
/// A toggle switch for boolean values.
///
/// # Props
///
/// - `checked` - Controlled checked state signal
/// - `default_checked` - Default checked state (uncontrolled)
/// - `size` - Size variant (Sm, Md, Lg)
/// - `disabled` - Whether the switch is disabled
/// - `loading` - Show loading state
/// - `checked_children` - Content shown when checked
/// - `unchecked_children` - Content shown when unchecked
/// - `on_change` - Change handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::selection::Switch;
///
/// let enabled = RwSignal::new(false);
///
/// view! {
///     <Switch
///         checked=enabled
///         checked_children="ON"
///         unchecked_children="OFF"
///     />
/// }
/// ```
#[component]
pub fn Switch(
    /// Controlled checked state.
    #[prop(optional, into)]
    checked: Option<RwSignal<bool>>,
    /// Default checked state (uncontrolled).
    #[prop(optional)]
    default_checked: bool,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<SelectionSize>,
    /// Whether the switch is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Show loading state.
    #[prop(optional)]
    loading: bool,
    /// Content shown when checked.
    #[prop(optional, into)]
    checked_children: Option<String>,
    /// Content shown when unchecked.
    #[prop(optional, into)]
    unchecked_children: Option<String>,
    /// Switch id attribute.
    #[prop(optional, into)]
    id: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<bool>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();

    // Create internal state if not controlled
    let internal_checked = checked.unwrap_or_else(|| RwSignal::new(default_checked));

    // Build CSS classes
    let switch_prefix = format!("fx-switch-{}", design_system);

    let size_class = if size != SelectionSize::Md {
        size.class(&switch_prefix)
    } else {
        String::new()
    };

    // Generate ID
    let switch_id = id.unwrap_or_else(|| format!("switch-{}", rand_id()));

    // Handle click
    let handle_click = move |_| {
        if disabled || loading {
            return;
        }
        let new_value = !internal_checked.get();
        internal_checked.set(new_value);
        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    // Inner handle class
    let handle_class = format!("{}-handle", switch_prefix);
    let inner_class = format!("{}-inner", switch_prefix);
    let loading_class = format!("{}-loading", switch_prefix);

    view! {
        <button
            type="button"
            role="switch"
            id=switch_id
            class=move || {
                let mut cls = vec![switch_prefix.clone()];
                if internal_checked.get() {
                    cls.push(format!("{}-checked", switch_prefix));
                }
                if !size_class.is_empty() {
                    cls.push(size_class.clone());
                }
                if disabled {
                    cls.push(format!("{}-disabled", switch_prefix));
                }
                if loading {
                    cls.push(loading_class.clone());
                }
                if let Some(ref custom) = class {
                    cls.push(custom.clone());
                }
                cls.join(" ")
            }
            aria-checked=move || internal_checked.get().to_string()
            disabled=disabled
            on:click=handle_click
        >
            <span class=handle_class.clone()>
                {if loading {
                    Some(view! {
                        <span class="fx-switch-loading-icon">
                            <svg viewBox="0 0 1024 1024" width="1em" height="1em">
                                <path d="M988 548c-19.9 0-36-16.1-36-36 0-59.4-11.6-117-34.6-171.3a440.45 440.45 0 00-94.3-139.9 437.71 437.71 0 00-139.9-94.3C629 83.6 571.4 72 512 72c-19.9 0-36-16.1-36-36s16.1-36 36-36c69.1 0 136.2 13.5 199.3 40.3C772.3 66 827 103 googler 176 134.7c40.7 40.7 77.7 95.3 103.9 156.3C820.7 411.9 834.2 479 834.2 548c0 19.9-16.1 36-36 36z"/>
                            </svg>
                        </span>
                    })
                } else {
                    None
                }}
            </span>
            <span class=inner_class.clone()>
                {move || {
                    if internal_checked.get() {
                        checked_children.clone()
                    } else {
                        unchecked_children.clone()
                    }
                }}
            </span>
        </button>
    }
}

/// Generate incremental ID.
fn rand_id() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(5000);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:x}", id)
}
