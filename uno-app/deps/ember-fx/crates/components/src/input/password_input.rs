//! PasswordInput Leptos component.

use leptos::prelude::*;
use super::types::{InputVariant, InputSize, ValidationState};
use crate::try_use_theme;

#[cfg(feature = "template")]
use crate::template::use_input_template;

/// PasswordInput component.
///
/// A password input field with show/hide toggle functionality.
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
/// - `show_toggle` - Whether to show the visibility toggle (default: true)
/// - `value` - Controlled value signal
/// - `on_input` - Input handler callback
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::input::PasswordInput;
///
/// let password = RwSignal::new(String::new());
///
/// view! {
///     <PasswordInput
///         label="Password"
///         placeholder="Enter your password"
///         value=password
///     />
/// }
/// ```
#[component]
pub fn PasswordInput(
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
    /// Whether to show the visibility toggle button.
    #[prop(optional)]
    #[prop(default = true)]
    show_toggle: bool,
    /// Maximum length of input.
    #[prop(optional, into)]
    max_length: Option<usize>,
    /// Controlled value signal.
    #[prop(optional, into)]
    value: Option<RwSignal<String>>,
    /// Input name attribute.
    #[prop(optional, into)]
    name: Option<String>,
    /// Input id attribute.
    #[prop(optional, into)]
    id: Option<String>,
    /// Autocomplete attribute.
    #[prop(optional, into)]
    autocomplete: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Prefix content (icon or text).
    #[prop(optional)]
    prefix: Option<Children>,
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

    // Password visibility state
    let visible = RwSignal::new(false);

    // Create internal value signal if not provided
    let internal_value = value.unwrap_or_else(|| RwSignal::new(String::new()));

    // Build CSS classes - pattern: fx-{component}-{design_system}
    let input_prefix = format!("fx-input-{}", design_system);
    let base_class = input_prefix.clone();
    let size_class = if size != InputSize::Md {
        size.class(&input_prefix)
    } else {
        String::new()
    };
    let variant_class = if variant != InputVariant::Outlined {
        variant.class(&input_prefix)
    } else {
        String::new()
    };

    // Template class (if template feature enabled)
    #[cfg(feature = "template")]
    let template_class = {
        let prefix = input_prefix.clone();
        move || {
            let tmpl = template.get();
            format!("{}-template-{}", prefix, tmpl.as_str())
        }
    };

    // Validation class
    let validation_class = validation.class(&input_prefix);

    // State classes
    let state_classes = {
        let mut classes = Vec::new();
        if disabled {
            classes.push(format!("{}-disabled", input_prefix));
        }
        classes.join(" ")
    };

    // Combine all classes for input element
    #[cfg(feature = "template")]
    let input_class = move || {
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
    let input_class = {
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

    // Toggle visibility handler
    let toggle_visibility = move |_| {
        visible.update(|v| *v = !*v);
    };

    // Classes for wrapper and helper elements
    let wrapper_class = format!("{}-wrapper", input_prefix);
    let label_class = format!("{}-label", input_prefix);
    let helper_class = format!("{}-helper", input_prefix);
    let error_class = format!("{}-error-text", input_prefix);
    let affix_wrapper_class = format!("{}-affix-wrapper", input_prefix);
    let prefix_class = format!("{}-prefix", input_prefix);
    let toggle_class = format!("{}-password-toggle", input_prefix);

    // Generate unique ID if not provided
    let input_id = id.unwrap_or_else(|| format!("password-{}", rand_id()));

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

    // Default autocomplete for password
    let autocomplete = autocomplete.unwrap_or_else(|| "current-password".to_string());

    view! {
        <div class=wrapper_class>
            // Label
            {label.map(|l| view! {
                <label class=label_class.clone() for=input_id.clone()>
                    {l}
                    {if required {
                        view! { <span class="fx-required-mark">" *"</span> }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }}
                </label>
            })}

            // Input with affixes
            <div class=affix_wrapper_class>
                {prefix.map(|p| view! {
                    <span class=prefix_class.clone()>{p()}</span>
                })}

                <input
                    type=move || if visible.get() { "text" } else { "password" }
                    id=input_id.clone()
                    name=name
                    class=input_class
                    placeholder=placeholder
                    disabled=disabled
                    required=required
                    maxlength=max_length.map(|l| l.to_string())
                    autocomplete=autocomplete
                    prop:value=move || internal_value.get()
                    on:input=handle_input
                    on:blur=handle_blur
                    on:focus=handle_focus
                    aria-invalid=move || if validation.is_error() { Some("true") } else { None }
                    aria-required=move || if required { Some("true") } else { None }
                />

                // Visibility toggle
                {if show_toggle {
                    view! {
                        <button
                            type="button"
                            class=toggle_class
                            on:click=toggle_visibility
                            disabled=disabled
                            aria-label=move || if visible.get() { "Hide password" } else { "Show password" }
                            tabindex="-1"
                        >
                            {move || if visible.get() { "👁" } else { "👁‍🗨" }}
                        </button>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }}
            </div>

            // Helper text or error message
            {message_to_show.map(|msg| view! {
                <span class=message_class.clone()>{msg}</span>
            })}
        </div>
    }
}

/// Generate a simple incremental ID for input elements.
fn rand_id() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(1000);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:x}", id)
}
