//! TextInput Leptos component.

use leptos::prelude::*;
use super::types::{InputVariant, InputSize, ValidationState};
use crate::try_use_theme;
use ember_fx_common::validation::{ValidationRule, ValidateOn, validate_value};

#[cfg(feature = "template")]
use crate::template::use_input_template;

/// TextInput component.
///
/// A single-line text input with label, placeholder, and validation support.
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
/// - `value` - Controlled value signal
/// - `rules` - Validation rules to apply
/// - `validate_on` - When to trigger validation (Blur, Change, Submit)
/// - `on_input` - Input handler callback
/// - `class` - Additional CSS classes
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::input::{TextInput, ValidationState};
/// use ember_fx_common::ValidationRule;
///
/// let value = RwSignal::new(String::new());
///
/// view! {
///     <TextInput
///         label="Email"
///         placeholder="Enter your email"
///         value=value
///         rules=vec![
///             ValidationRule::required("Email is required"),
///             ValidationRule::email("Invalid email format"),
///         ]
///     />
/// }
/// ```
#[component]
pub fn TextInput(
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
    /// Maximum length of input.
    #[prop(optional, into)]
    max_length: Option<usize>,
    /// Show character count.
    #[prop(optional)]
    show_count: bool,
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
    /// Suffix content (icon or text).
    #[prop(optional)]
    suffix: Option<Children>,
    /// Input handler.
    #[prop(optional, into)]
    on_input: Option<Callback<String>>,
    /// Blur handler.
    #[prop(optional, into)]
    on_blur: Option<Callback<()>>,
    /// Focus handler.
    #[prop(optional, into)]
    on_focus: Option<Callback<()>>,
    /// Validation rules.
    #[prop(optional)]
    rules: Option<Vec<ValidationRule>>,
    /// When to trigger validation.
    #[prop(optional)]
    validate_on: Option<ValidateOn>,
    /// Callback when validation state changes.
    #[prop(optional, into)]
    on_validation: Option<Callback<(ValidationState, Option<String>)>>,
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
    let initial_validation = validation.unwrap_or_default();
    let validate_on = validate_on.unwrap_or_default();
    let rules = rules.unwrap_or_default();

    // Create internal value signal if not provided
    let internal_value = value.unwrap_or_else(|| RwSignal::new(String::new()));

    // Reactive validation state (can be overridden by rules or static validation prop)
    let validation_state = RwSignal::new(initial_validation);
    let validation_error = RwSignal::new(error_message.clone());

    // Validation function
    let do_validate = {
        let rules = rules.clone();
        let on_validation = on_validation.clone();
        move |value: &str| {
            if rules.is_empty() {
                return;
            }
            let result = validate_value(value, &rules);
            let (new_state, new_error) = if result.is_valid() {
                (ValidationState::Success, None)
            } else {
                (ValidationState::Error, result.first_error().cloned())
            };
            validation_state.set(new_state);
            validation_error.set(new_error.clone());
            if let Some(ref cb) = on_validation {
                cb.run((new_state, new_error));
            }
        }
    };

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

    // Has prefix/suffix - need wrapper
    let has_affix = prefix.is_some() || suffix.is_some();

    // Template class (if template feature enabled)
    #[cfg(feature = "template")]
    let template_class = {
        let prefix = input_prefix.clone();
        move || {
            let tmpl = template.get();
            format!("{}-template-{}", prefix, tmpl.as_str())
        }
    };

    // Reactive validation class (uses signal)
    let validation_class_prefix = input_prefix.clone();

    // State classes
    let state_classes = {
        let mut classes = Vec::new();
        if disabled {
            classes.push(format!("{}-disabled", input_prefix));
        }
        if readonly {
            classes.push(format!("{}-readonly", input_prefix));
        }
        classes.join(" ")
    };

    // Combine all classes for input element (reactive)
    #[cfg(feature = "template")]
    let input_class = {
        let base = base_class.clone();
        let size_cls = size_class.clone();
        let variant_cls = variant_class.clone();
        let state_cls = state_classes.clone();
        let custom_cls = class.clone();
        let val_prefix = validation_class_prefix.clone();
        move || {
            let mut parts = vec![base.clone()];
            if !size_cls.is_empty() {
                parts.push(size_cls.clone());
            }
            if !variant_cls.is_empty() {
                parts.push(variant_cls.clone());
            }
            // Reactive validation class
            let val_cls = validation_state.get().class(&val_prefix);
            if !val_cls.is_empty() {
                parts.push(val_cls);
            }
            parts.push(template_class());
            if !state_cls.is_empty() {
                parts.push(state_cls.clone());
            }
            if let Some(ref custom) = custom_cls {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    #[cfg(not(feature = "template"))]
    let input_class = {
        let base = base_class.clone();
        let size_cls = size_class.clone();
        let variant_cls = variant_class.clone();
        let state_cls = state_classes.clone();
        let custom_cls = class.clone();
        let val_prefix = validation_class_prefix.clone();
        move || {
            let mut parts = vec![base.clone()];
            if !size_cls.is_empty() {
                parts.push(size_cls.clone());
            }
            if !variant_cls.is_empty() {
                parts.push(variant_cls.clone());
            }
            // Reactive validation class
            let val_cls = validation_state.get().class(&val_prefix);
            if !val_cls.is_empty() {
                parts.push(val_cls);
            }
            if !state_cls.is_empty() {
                parts.push(state_cls.clone());
            }
            if let Some(ref custom) = custom_cls {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Input handler
    let do_validate_on_change = do_validate.clone();
    let handle_input = move |ev: leptos::ev::Event| {
        let new_value = event_target_value(&ev);
        internal_value.set(new_value.clone());
        if let Some(ref cb) = on_input {
            cb.run(new_value.clone());
        }
        // Validate on change if configured
        if validate_on == ValidateOn::Change {
            do_validate_on_change(&new_value);
        }
    };

    // Blur handler
    let do_validate_on_blur = do_validate.clone();
    let handle_blur = move |_| {
        if let Some(ref cb) = on_blur {
            cb.run(());
        }
        // Validate on blur if configured (default)
        if validate_on == ValidateOn::Blur {
            do_validate_on_blur(&internal_value.get());
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
    let affix_wrapper_class = format!("{}-affix-wrapper", input_prefix);
    let prefix_class = format!("{}-prefix", input_prefix);
    let suffix_class = format!("{}-suffix", input_prefix);

    // Generate unique ID if not provided
    let input_id = id.unwrap_or_else(|| format!("input-{}", rand_id()));

    // Build the input element
    let input_element = view! {
        <input
            type="text"
            id=input_id.clone()
            name=name
            class=input_class
            placeholder=placeholder
            disabled=disabled
            readonly=readonly
            required=required
            maxlength=max_length.map(|l| l.to_string())
            autocomplete=autocomplete
            prop:value=move || internal_value.get()
            on:input=handle_input
            on:blur=handle_blur
            on:focus=handle_focus
            aria-invalid=move || if validation_state.get().is_error() { Some("true") } else { None }
            aria-required=move || if required { Some("true") } else { None }
        />
    };

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

            // Input with optional affixes
            {if has_affix {
                view! {
                    <div class=affix_wrapper_class>
                        {prefix.map(|p| view! {
                            <span class=prefix_class.clone()>{p()}</span>
                        })}
                        {input_element}
                        {suffix.map(|s| view! {
                            <span class=suffix_class.clone()>{s()}</span>
                        })}
                    </div>
                }.into_any()
            } else {
                input_element.into_any()
            }}

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

            // Helper text or error message (reactive)
            {move || {
                let val_state = validation_state.get();
                let val_error = validation_error.get();

                if val_state.is_error() {
                    if let Some(err) = val_error {
                        return view! {
                            <span class=error_class.clone()>{err}</span>
                        }.into_any();
                    }
                }

                if let Some(ref helper) = helper_text {
                    view! {
                        <span class=helper_class.clone()>{helper.clone()}</span>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        </div>
    }
}

/// Generate a simple incremental ID for input elements.
fn rand_id() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:x}", id)
}
