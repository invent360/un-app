//! FormField wrapper component for inputs with validation display.

use leptos::prelude::*;
use super::types::ValidationState;
use crate::try_use_theme;

/// Form field wrapper component.
///
/// Wraps any input component with consistent label, error, and helper text display.
/// Use this for custom inputs or when you need more control over the layout.
///
/// # Props
///
/// - `label` - Field label text
/// - `required` - Show required indicator
/// - `error` - Error message to display
/// - `helper_text` - Helper text below the input
/// - `validation` - Validation state for styling
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::input::{FormField, ValidationState};
///
/// let error = RwSignal::new(None::<String>);
///
/// view! {
///     <FormField
///         label="Email"
///         required=true
///         error=error.get()
///         helper_text="We'll never share your email"
///     >
///         <input type="email" />
///     </FormField>
/// }
/// ```
#[component]
pub fn FormField(
    /// Field label.
    #[prop(optional, into)]
    label: Option<String>,
    /// Whether the field is required.
    #[prop(optional)]
    required: bool,
    /// Error message to display.
    #[prop(optional, into)]
    error: Option<String>,
    /// Helper text below the input.
    #[prop(optional, into)]
    helper_text: Option<String>,
    /// Validation state for styling.
    #[prop(optional, into)]
    validation: Option<ValidationState>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Input element(s) to wrap.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Build CSS classes
    let field_prefix = format!("fx-form-field-{}", design_system);

    // Determine validation state from error or explicit validation prop
    let effective_validation = if error.is_some() {
        ValidationState::Error
    } else {
        validation.unwrap_or_default()
    };

    let validation_class = effective_validation.class(&field_prefix);

    let combined_class = {
        let mut parts = vec![field_prefix.clone()];
        if !validation_class.is_empty() {
            parts.push(validation_class);
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let label_class = format!("{}-label", field_prefix);
    let required_class = format!("{}-required", field_prefix);
    let content_class = format!("{}-content", field_prefix);
    let helper_class = format!("{}-helper", field_prefix);
    let error_class = format!("{}-error", field_prefix);

    view! {
        <div class=combined_class>
            // Label
            {label.map(|l| {
                let label_class = label_class.clone();
                let required_class = required_class.clone();
                view! {
                    <label class=label_class>
                        {l}
                        {if required {
                            Some(view! { <span class=required_class>" *"</span> })
                        } else {
                            None
                        }}
                    </label>
                }
            })}

            // Input content
            <div class=content_class>
                {children()}
            </div>

            // Error message (takes precedence over helper text)
            {error.clone().map(|e| {
                let error_class = error_class.clone();
                view! {
                    <div class=error_class role="alert">
                        {e}
                    </div>
                }
            })}

            // Helper text (only shown if no error)
            {if error.is_none() {
                helper_text.map(|h| {
                    let helper_class = helper_class.clone();
                    view! {
                        <div class=helper_class>
                            {h}
                        </div>
                    }
                })
            } else {
                None
            }}
        </div>
    }
}

/// Reactive form field wrapper.
///
/// Similar to `FormField` but accepts reactive signals for error state.
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::input::ReactiveFormField;
///
/// let error = RwSignal::new(None::<String>);
///
/// view! {
///     <ReactiveFormField
///         label="Username"
///         error=error
///     >
///         <input type="text" />
///     </ReactiveFormField>
/// }
/// ```
#[component]
pub fn ReactiveFormField(
    /// Field label.
    #[prop(optional, into)]
    label: Option<String>,
    /// Whether the field is required.
    #[prop(optional)]
    required: bool,
    /// Reactive error signal.
    #[prop(into)]
    error: Signal<Option<String>>,
    /// Helper text below the input.
    #[prop(optional, into)]
    helper_text: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Input element(s) to wrap.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Build CSS classes
    let field_prefix = format!("fx-form-field-{}", design_system);
    let field_prefix_for_class = field_prefix.clone();
    let field_prefix_for_error = field_prefix.clone();
    let field_prefix_for_helper = field_prefix.clone();

    let label_class = format!("{}-label", field_prefix);
    let required_class = format!("{}-required", field_prefix);
    let content_class = format!("{}-content", field_prefix);
    let helper_class = format!("{}-helper", field_prefix);
    let error_class = format!("{}-error", field_prefix);

    let combined_class = {
        let class = class.clone();
        move || {
            let mut parts = vec![field_prefix_for_class.clone()];
            if error.get().is_some() {
                parts.push(format!("{}-error-state", field_prefix_for_class));
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    view! {
        <div class=combined_class>
            // Label
            {label.map(|l| {
                let label_class = label_class.clone();
                let required_class = required_class.clone();
                view! {
                    <label class=label_class>
                        {l}
                        {if required {
                            Some(view! { <span class=required_class>" *"</span> })
                        } else {
                            None
                        }}
                    </label>
                }
            })}

            // Input content
            <div class=content_class>
                {children()}
            </div>

            // Error or helper text (reactive)
            {move || {
                let error_class = error_class.clone();
                let helper_class = helper_class.clone();
                let helper_text = helper_text.clone();
                let field_prefix = field_prefix_for_error.clone();

                if let Some(err) = error.get() {
                    view! {
                        <div class=format!("{}-message {}", field_prefix, error_class) role="alert">
                            {err}
                        </div>
                    }.into_any()
                } else if let Some(ref helper) = helper_text {
                    view! {
                        <div class=format!("{}-message {}", field_prefix_for_helper, helper_class)>
                            {helper.clone()}
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}
