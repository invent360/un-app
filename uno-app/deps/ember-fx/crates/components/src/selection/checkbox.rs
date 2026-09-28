//! Checkbox Leptos component.

use leptos::prelude::*;
use super::types::{SelectionSize, CheckStyle};
use crate::try_use_theme;

/// Checkbox component.
///
/// A single checkbox with label support.
///
/// # Props
///
/// - `checked` - Controlled checked state signal
/// - `label` - Label text
/// - `size` - Size variant (Sm, Md, Lg)
/// - `style` - Visual style (Standard, Button, Card)
/// - `disabled` - Whether the checkbox is disabled
/// - `indeterminate` - Show indeterminate state
/// - `on_change` - Change handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::selection::Checkbox;
///
/// let checked = RwSignal::new(false);
///
/// view! {
///     <Checkbox
///         label="Accept terms"
///         checked=checked
///     />
/// }
/// ```
#[component]
pub fn Checkbox(
    /// Controlled checked state.
    #[prop(optional, into)]
    checked: Option<RwSignal<bool>>,
    /// Default checked state (uncontrolled).
    #[prop(optional)]
    default_checked: bool,
    /// Checkbox label.
    #[prop(optional, into)]
    label: Option<String>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<SelectionSize>,
    /// Visual style.
    #[prop(optional, into)]
    style: Option<CheckStyle>,
    /// Whether the checkbox is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Show indeterminate state.
    #[prop(optional)]
    indeterminate: bool,
    /// Checkbox value (for groups).
    #[prop(optional, into)]
    value: Option<String>,
    /// Checkbox name attribute.
    #[prop(optional, into)]
    name: Option<String>,
    /// Checkbox id attribute.
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
    let style = style.unwrap_or_default();

    // Create internal state if not controlled
    let internal_checked = checked.unwrap_or_else(|| RwSignal::new(default_checked));

    // Build CSS classes
    let checkbox_prefix = format!("fx-checkbox-{}", design_system);
    let wrapper_class = format!("{}-wrapper", checkbox_prefix);

    let size_class = if size != SelectionSize::Md {
        size.class(&checkbox_prefix)
    } else {
        String::new()
    };

    let style_class = if style != CheckStyle::Standard {
        style.class(&checkbox_prefix)
    } else {
        String::new()
    };

    // Build combined class
    let combined_class = {
        let mut parts = vec![wrapper_class.clone()];
        if !size_class.is_empty() {
            parts.push(size_class);
        }
        if !style_class.is_empty() {
            parts.push(style_class);
        }
        if disabled {
            parts.push(format!("{}-disabled", checkbox_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Generate ID
    let checkbox_id = id.unwrap_or_else(|| format!("checkbox-{}", rand_id()));

    // Inner checkbox class
    let inner_class = format!("{}-inner", checkbox_prefix);
    let input_class = format!("{}-input", checkbox_prefix);
    let label_class = format!("{}-label", checkbox_prefix);

    // Handle change
    let handle_change = move |ev: leptos::ev::Event| {
        if disabled {
            return;
        }
        let new_value = event_target_checked(&ev);
        internal_checked.set(new_value);
        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    view! {
        <label class=combined_class>
            <span
                class=move || {
                    let mut cls = vec![checkbox_prefix.clone()];
                    if internal_checked.get() {
                        cls.push(format!("{}-checked", checkbox_prefix));
                    }
                    if indeterminate && !internal_checked.get() {
                        cls.push(format!("{}-indeterminate", checkbox_prefix));
                    }
                    cls.join(" ")
                }
            >
                <input
                    type="checkbox"
                    class=input_class
                    id=checkbox_id.clone()
                    name=name
                    value=value
                    disabled=disabled
                    prop:checked=move || internal_checked.get()
                    prop:indeterminate=indeterminate
                    on:change=handle_change
                />
                <span class=inner_class></span>
            </span>
            {label.map(|l| view! {
                <span class=label_class>{l}</span>
            })}
        </label>
    }
}

/// CheckboxGroup component.
///
/// A group of checkboxes for multiple selections.
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::selection::{CheckboxGroup, Checkbox};
///
/// let selected = RwSignal::new(vec!["option1".to_string()]);
///
/// view! {
///     <CheckboxGroup value=selected>
///         <Checkbox value="option1" label="Option 1" />
///         <Checkbox value="option2" label="Option 2" />
///         <Checkbox value="option3" label="Option 3" />
///     </CheckboxGroup>
/// }
/// ```
#[component]
pub fn CheckboxGroup(
    /// Selected values.
    #[prop(optional, into)]
    value: Option<RwSignal<Vec<String>>>,
    /// Layout direction.
    #[prop(optional, into)]
    layout: Option<super::types::GroupLayout>,
    /// Size for all checkboxes.
    #[prop(optional, into)]
    size: Option<SelectionSize>,
    /// Whether all checkboxes are disabled.
    #[prop(optional)]
    disabled: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<Vec<String>>>,
    /// Child checkboxes.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let layout = layout.unwrap_or_default();

    // Build CSS classes
    let group_prefix = format!("fx-checkbox-group-{}", design_system);
    let layout_class = layout.class(&group_prefix);

    let combined_class = {
        let mut parts = vec![group_prefix.clone(), layout_class];
        if disabled {
            parts.push(format!("{}-disabled", group_prefix));
        }
        if let Some(custom) = class {
            parts.push(custom);
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class role="group">
            {children()}
        </div>
    }
}

/// Generate incremental ID.
fn rand_id() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(3000);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:x}", id)
}
