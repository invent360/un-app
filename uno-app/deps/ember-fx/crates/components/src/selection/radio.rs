//! Radio Leptos component.

use leptos::prelude::*;
use super::types::{SelectionSize, CheckStyle, GroupLayout};
use crate::try_use_theme;

/// Context for RadioGroup to share state with child Radio components.
#[derive(Clone)]
pub struct RadioGroupContext {
    /// Currently selected value.
    pub value: RwSignal<Option<String>>,
    /// Group name for form submission.
    pub name: String,
    /// Whether the group is disabled.
    pub disabled: bool,
    /// Size for all radios.
    pub size: SelectionSize,
}

/// Radio component.
///
/// A single radio button with label support.
///
/// # Props
///
/// - `value` - The value of this radio option
/// - `label` - Label text
/// - `size` - Size variant (Sm, Md, Lg)
/// - `style` - Visual style (Standard, Button, Card)
/// - `disabled` - Whether the radio is disabled
/// - `checked` - Controlled checked state (for standalone use)
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::selection::{Radio, RadioGroup};
///
/// let selected = RwSignal::new(Some("option1".to_string()));
///
/// view! {
///     <RadioGroup value=selected>
///         <Radio value="option1" label="Option 1" />
///         <Radio value="option2" label="Option 2" />
///     </RadioGroup>
/// }
/// ```
#[component]
pub fn Radio(
    /// The value of this radio option.
    #[prop(into)]
    value: String,
    /// Radio label.
    #[prop(optional, into)]
    label: Option<String>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<SelectionSize>,
    /// Visual style.
    #[prop(optional, into)]
    style: Option<CheckStyle>,
    /// Whether the radio is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Controlled checked state (for standalone use).
    #[prop(optional, into)]
    checked: Option<RwSignal<bool>>,
    /// Radio id attribute.
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

    // Try to get group context
    let group_ctx = use_context::<RadioGroupContext>();

    // Resolve values from group context or props
    let size = size.unwrap_or_else(|| {
        group_ctx.as_ref().map(|g| g.size).unwrap_or_default()
    });
    let style = style.unwrap_or_default();
    let is_disabled = disabled || group_ctx.as_ref().map(|g| g.disabled).unwrap_or(false);
    let radio_name = group_ctx.as_ref()
        .map(|g| g.name.clone())
        .unwrap_or_else(|| format!("radio-{}", rand_id()));

    // Build CSS classes
    let radio_prefix = format!("fx-radio-{}", design_system);
    let wrapper_class = format!("{}-wrapper", radio_prefix);

    let size_class = if size != SelectionSize::Md {
        size.class(&radio_prefix)
    } else {
        String::new()
    };

    let style_class = if style != CheckStyle::Standard {
        style.class(&radio_prefix)
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
        if is_disabled {
            parts.push(format!("{}-disabled", radio_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Generate ID
    let radio_id = id.unwrap_or_else(|| format!("radio-{}", rand_id()));

    // Inner radio class
    let inner_class = format!("{}-inner", radio_prefix);
    let input_class = format!("{}-input", radio_prefix);
    let label_class = format!("{}-label", radio_prefix);

    // Clone value for closures
    let value_for_class = value.clone();
    let value_for_prop = value.clone();
    let value_for_handler = value.clone();

    // Extract group value signal for use in closures
    let group_value_signal = group_ctx.as_ref().map(|ctx| ctx.value);

    // Handle change
    let handle_change = move |_ev: leptos::ev::Event| {
        if is_disabled {
            return;
        }

        if let Some(ref ctx) = use_context::<RadioGroupContext>() {
            ctx.value.set(Some(value_for_handler.clone()));
        } else if let Some(signal) = checked {
            signal.set(true);
        }

        if let Some(ref cb) = on_change {
            cb.run(true);
        }
    };

    view! {
        <label class=combined_class>
            <span
                class=move || {
                    let mut cls = vec![radio_prefix.clone()];
                    let is_checked = if let Some(signal) = group_value_signal {
                        signal.get().as_ref() == Some(&value_for_class)
                    } else if let Some(signal) = checked {
                        signal.get()
                    } else {
                        false
                    };
                    if is_checked {
                        cls.push(format!("{}-checked", radio_prefix));
                    }
                    cls.join(" ")
                }
            >
                <input
                    type="radio"
                    class=input_class
                    id=radio_id.clone()
                    name=radio_name
                    value=value
                    disabled=is_disabled
                    prop:checked=move || {
                        if let Some(signal) = group_value_signal {
                            signal.get().as_ref() == Some(&value_for_prop)
                        } else if let Some(signal) = checked {
                            signal.get()
                        } else {
                            false
                        }
                    }
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

/// RadioGroup component.
///
/// A group of radio buttons for single selection.
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::selection::{RadioGroup, Radio};
///
/// let selected = RwSignal::new(Some("option1".to_string()));
///
/// view! {
///     <RadioGroup value=selected name="my-radio-group">
///         <Radio value="option1" label="Option 1" />
///         <Radio value="option2" label="Option 2" />
///         <Radio value="option3" label="Option 3" />
///     </RadioGroup>
/// }
/// ```
#[component]
pub fn RadioGroup(
    /// Selected value.
    #[prop(optional, into)]
    value: Option<RwSignal<Option<String>>>,
    /// Default selected value (uncontrolled).
    #[prop(optional, into)]
    default_value: Option<String>,
    /// Group name for form submission.
    #[prop(optional, into)]
    name: Option<String>,
    /// Layout direction.
    #[prop(optional, into)]
    layout: Option<GroupLayout>,
    /// Size for all radios.
    #[prop(optional, into)]
    size: Option<SelectionSize>,
    /// Whether all radios are disabled.
    #[prop(optional)]
    disabled: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<Option<String>>>,
    /// Child radios.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let layout = layout.unwrap_or_default();
    let size = size.unwrap_or_default();

    // Create internal state if not controlled
    let internal_value = value.unwrap_or_else(|| RwSignal::new(default_value));

    // Generate group name
    let group_name = name.unwrap_or_else(|| format!("radio-group-{}", rand_id()));

    // Provide context to children
    let ctx = RadioGroupContext {
        value: internal_value,
        name: group_name,
        disabled,
        size,
    };
    provide_context(ctx);

    // Build CSS classes
    let group_prefix = format!("fx-radio-group-{}", design_system);
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
        <div class=combined_class role="radiogroup">
            {children()}
        </div>
    }
}

/// Generate incremental ID.
fn rand_id() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(4000);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:x}", id)
}
