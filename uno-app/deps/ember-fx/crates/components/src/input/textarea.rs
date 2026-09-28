//! TextArea Leptos component.

use leptos::prelude::*;
use super::types::{InputVariant, InputSize, ValidationState};
use crate::try_use_theme;

#[cfg(feature = "template")]
use crate::template::use_input_template;

/// TextArea component.
///
/// A multi-line text input with auto-resize and character count support.
/// Automatically integrates with the theme context for styling.
///
/// # Props
///
/// - `variant` - Visual variant (Outlined, Filled, Borderless)
/// - `size` - Size variant (Sm, Md, Lg)
/// - `validation` - Validation state (None, Success, Warning, Error)
/// - `label` - Input label text
/// - `placeholder` - Placeholder text
/// - `helper_text` - Helper text below input
/// - `error_message` - Error message (shown when validation is Error)
/// - `required` - Whether the input is required
/// - `disabled` - Whether the input is disabled
/// - `readonly` - Whether the input is readonly
/// - `rows` - Number of visible rows
/// - `max_length` - Maximum character count
/// - `show_count` - Show character count
/// - `auto_size` - Auto-resize based on content
/// - `value` - Controlled value signal
/// - `on_input` - Input handler callback
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::input::TextArea;
///
/// let content = RwSignal::new(String::new());
///
/// view! {
///     <TextArea
///         label="Description"
///         placeholder="Enter a description..."
///         rows=4
///         value=content
///         show_count=true
///         max_length=500
///     />
/// }
/// ```
#[component]
pub fn TextArea(
    /// Input variant (visual style).
    #[prop(optional, into)]
    variant: Option<InputVariant>,
    /// Input size.
    #[prop(optional, into)]
    size: Option<InputSize>,
    /// Validation state.
    #[prop(optional, into)]
    validation: Option<ValidationState>,
    /// Input label.
    #[prop(optional, into)]
    label: Option<String>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Helper text displayed below input.
    #[prop(optional, into)]
    helper_text: Option<String>,
    /// Error message (displayed when validation is Error).
    #[prop(optional, into)]
    error_message: Option<String>,
    /// Whether the input is required.
    #[prop(optional)]
    required: bool,
    /// Whether the input is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Whether the input is readonly.
    #[prop(optional)]
    readonly: bool,
    /// Number of visible text rows.
    #[prop(optional)]
    #[prop(default = 4)]
    rows: u32,
    /// Maximum length of input.
    #[prop(optional, into)]
    max_length: Option<usize>,
    /// Show character count.
    #[prop(optional)]
    show_count: bool,
    /// Auto-resize based on content.
    #[prop(optional)]
    auto_size: bool,
    /// Resize behavior (none, vertical, horizontal, both).
    #[prop(optional, into)]
    resize: Option<String>,
    /// Controlled value signal.
    #[prop(optional, into)]
    value: Option<RwSignal<String>>,
    /// Input name attribute.
    #[prop(optional, into)]
    name: Option<String>,
    /// Input id attribute.
    #[prop(optional, into)]
    id: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Input handler.
    #[prop(optional, into)]
    on_input: Option<Callback<String>>,
    /// Blur handler.
    #[prop(optional, into)]
    on_blur: Option<Callback<()>>,
    /// Focus handler.
    #[prop(optional, into)]
    on_focus: Option<Callback<()>>,
) -> impl IntoView {
    // Get theme context for design system prefix
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Get template from context
    #[cfg(feature = "template")]
    let template = use_input_template();

    // Resolve defaults
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
    let validation = validation.unwrap_or_default();

    // Create internal value signal if not provided
    let internal_value = value.unwrap_or_else(|| RwSignal::new(String::new()));

    // Build CSS classes - pattern: fx-{component}-{design_system}
    let textarea_prefix = format!("fx-textarea-{}", design_system);
    let input_prefix = format!("fx-input-{}", design_system); // For shared styles
    let base_class = textarea_prefix.clone();
    let size_class = if size != InputSize::Md {
        size.class(&textarea_prefix)
    } else {
        String::new()
    };
    let variant_class = if variant != InputVariant::Outlined {
        variant.class(&textarea_prefix)
    } else {
        String::new()
    };

    // Template class (if template feature enabled)
    #[cfg(feature = "template")]
    let template_class = {
        let prefix = textarea_prefix.clone();
        move || {
            let tmpl = template.get();
            format!("{}-template-{}", prefix, tmpl.as_str())
        }
    };

    // Validation class
    let validation_class = if validation.has_state() {
        validation.class(&textarea_prefix)
    } else {
        String::new()
    };

    // State classes
    let state_classes = {
        let mut classes = Vec::new();
        if disabled {
            classes.push(format!("{}-disabled", textarea_prefix));
        }
        if readonly {
            classes.push(format!("{}-readonly", textarea_prefix));
        }
        if auto_size {
            classes.push(format!("{}-autosize", textarea_prefix));
        }
        classes.join(" ")
    };

    // Combine all classes for textarea element
    #[cfg(feature = "template")]
    let textarea_class = move || {
        let mut parts = vec![base_class.clone()];
        if !size_class.is_empty() {
            parts.push(size_class.clone());
        }
        if !variant_class.is_empty() {
            parts.push(variant_class.clone());
        }
        if !validation_class.is_empty() {
            parts.push(validation_class.clone());
        }
        parts.push(template_class());
        if !state_classes.is_empty() {
            parts.push(state_classes.clone());
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    #[cfg(not(feature = "template"))]
    let textarea_class = {
        let mut parts = vec![base_class.clone()];
        if !size_class.is_empty() {
            parts.push(size_class.clone());
        }
        if !variant_class.is_empty() {
            parts.push(variant_class.clone());
        }
        if !validation_class.is_empty() {
            parts.push(validation_class.clone());
        }
        if !state_classes.is_empty() {
            parts.push(state_classes.clone());
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Input handler
    let handle_input = move |ev: leptos::ev::Event| {
        let new_value = event_target_value(&ev);
        internal_value.set(new_value.clone());
        if let Some(ref cb) = on_input {
            cb.run(new_value);
        }
    };

    // Blur handler
    let handle_blur = move |_| {
        if let Some(ref cb) = on_blur {
            cb.run(());
        }
    };

    // Focus handler
    let handle_focus = move |_| {
        if let Some(ref cb) = on_focus {
            cb.run(());
        }
    };

    // Classes for wrapper and helper elements
    let wrapper_class = format!("{}-wrapper", input_prefix);
    let label_class = format!("{}-label", input_prefix);
    let helper_class = format!("{}-helper", input_prefix);
    let error_class = format!("{}-error-text", input_prefix);
    let count_class = format!("{}-count", input_prefix);

    // Generate unique ID if not provided
    let textarea_id = id.unwrap_or_else(|| format!("textarea-{}", rand_id()));

    // Determine which message to show
    let show_error = validation.is_error() && error_message.is_some();
    let message_to_show = if show_error {
        error_message.clone()
    } else {
        helper_text.clone()
    };
    let message_class = if show_error {
        error_class.clone()
    } else {
        helper_class.clone()
    };

    // Resize style
    let resize_style = resize.unwrap_or_else(|| {
        if auto_size { "none".to_string() } else { "vertical".to_string() }
    });

    view! {
        <div class=wrapper_class>
            // Label
            {label.map(|l| view! {
                <label class=label_class.clone() for=textarea_id.clone()>
                    {l}
                    {if required {
                        view! { <span class="fx-required-mark">" *"</span> }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }}
                </label>
            })}

            // Textarea
            <textarea
                id=textarea_id.clone()
                name=name
                class=textarea_class
                placeholder=placeholder
                disabled=disabled
                readonly=readonly
                required=required
                rows=rows
                maxlength=max_length.map(|l| l.to_string())
                style=format!("resize: {}", resize_style)
                prop:value=move || internal_value.get()
                on:input=handle_input
                on:blur=handle_blur
                on:focus=handle_focus
                aria-invalid=move || if validation.is_error() { Some("true") } else { None }
                aria-required=move || if required { Some("true") } else { None }
            ></textarea>

            // Footer with count and/or helper text
            <div class=format!("{}-footer", input_prefix)>
                // Helper text or error message
                {message_to_show.map(|msg| view! {
                    <span class=message_class.clone()>{msg}</span>
                })}

                // Character count
                {if show_count {
                    let count_text = move || {
                        let len = internal_value.get().len();
                        if let Some(max) = max_length {
                            format!("{} / {}", len, max)
                        } else {
                            len.to_string()
                        }
                    };
                    view! {
                        <span class=count_class>{count_text}</span>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }}
            </div>
        </div>
    }
}

/// Generate a simple incremental ID for input elements.
fn rand_id() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(2000);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:x}", id)
}
