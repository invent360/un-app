//! Application-specific form validation utilities
//!
//! Provides pre-configured validation rules for common fields in the UNO app.
//! Built on top of ember-fx-components validation system.

use ember_fx_components::{ValidationRule, ValidationResult, validate_value, validate_all};
use leptos::prelude::*;

/// Pre-defined validation rules for email fields
pub fn email_rules() -> Vec<ValidationRule> {
    vec![
        ValidationRule::required("Email is required"),
        ValidationRule::email("Please enter a valid email address"),
    ]
}

/// Pre-defined validation rules for license key fields
pub fn license_key_rules() -> Vec<ValidationRule> {
    vec![
        ValidationRule::required("License key is required"),
        ValidationRule::length(19, "License key must be 19 characters (XXXX-XXXX-XXXX-XXXX)"),
        ValidationRule::pattern("-", "License key must contain dashes"),
    ]
}

/// Pre-defined validation rules for username fields
pub fn username_rules() -> Vec<ValidationRule> {
    vec![
        ValidationRule::required("Username is required"),
        ValidationRule::min_length(3, "Username must be at least 3 characters"),
        ValidationRule::max_length(32, "Username must be at most 32 characters"),
    ]
}

/// Pre-defined validation rules for password fields
pub fn password_rules() -> Vec<ValidationRule> {
    vec![
        ValidationRule::required("Password is required"),
        ValidationRule::min_length(8, "Password must be at least 8 characters"),
        ValidationRule::max_length(128, "Password must be at most 128 characters"),
    ]
}

/// Form validation hook that manages field validation state
///
/// # Example
///
/// ```ignore
/// use crate::hooks::use_form_validation;
///
/// let email = RwSignal::new(String::new());
/// let (validate_email, email_error) = use_form_validation(email, email_rules());
///
/// view! {
///     <input
///         type="email"
///         on:blur=move |_| validate_email()
///         prop:value=move || email.get()
///         on:input=move |e| email.set(event_target_value(&e))
///     />
///     {move || email_error.get().map(|e| view! { <span class="error">{e}</span> })}
/// }
/// ```
pub fn use_form_validation(
    value: RwSignal<String>,
    rules: Vec<ValidationRule>,
) -> (impl Fn() + Clone, Signal<Option<String>>) {
    let error = RwSignal::new(None::<String>);

    let validate = {
        let rules = rules.clone();
        move || {
            let current_value = value.get();
            let result = validate_value(&current_value, &rules);
            error.set(result.first_error().cloned());
        }
    };

    let error_signal = Signal::derive(move || error.get());

    (validate, error_signal)
}

/// Form validation hook that collects all errors
pub fn use_form_validation_all(
    value: RwSignal<String>,
    rules: Vec<ValidationRule>,
) -> (impl Fn() + Clone, Signal<Vec<String>>) {
    let errors = RwSignal::new(Vec::<String>::new());

    let validate = {
        let rules = rules.clone();
        move || {
            let current_value = value.get();
            let result = validate_all(&current_value, &rules);
            errors.set(result.errors);
        }
    };

    let errors_signal = Signal::derive(move || errors.get());

    (validate, errors_signal)
}

/// Multi-field form validation state
pub struct FormState {
    /// Whether the form has been submitted at least once
    pub submitted: RwSignal<bool>,
    /// Whether all fields are valid
    pub is_valid: Signal<bool>,
}

impl FormState {
    /// Create a new form state with validation checks
    pub fn new(validators: Vec<Signal<Option<String>>>) -> Self {
        let submitted = RwSignal::new(false);

        let is_valid = Signal::derive(move || {
            validators.iter().all(|v| v.get().is_none())
        });

        Self { submitted, is_valid }
    }

    /// Mark the form as submitted
    pub fn submit(&self) {
        self.submitted.set(true);
    }

    /// Check if form can be submitted
    pub fn can_submit(&self) -> bool {
        self.is_valid.get()
    }
}
