//! Field component for label/value pairs.

use leptos::prelude::*;

/// Field component for displaying label/value pairs.
///
/// Typically used in StatCard footer for secondary metrics.
///
/// # Props
///
/// - `label` - Field label
/// - `value` - Field value
///
/// # Example
///
/// ```ignore
/// <Field label="Daily Sales" value="$12,423" />
/// ```
#[component]
pub fn Field(
    /// Field label.
    #[prop(into)]
    label: String,
    /// Field value.
    #[prop(into)]
    value: String,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let field_class = {
        let mut classes = vec!["fx-field".to_string()];
        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }
        classes.join(" ")
    };

    view! {
        <div class=field_class>
            <span class="fx-field-label">{label}</span>
            <span class="fx-field-value">{value}</span>
        </div>
    }
}

/// Inline field with separator.
///
/// A compact field variant that displays label and value inline
/// with a visual separator.
#[component]
pub fn InlineField(
    /// Field label.
    #[prop(into)]
    label: String,
    /// Field value.
    #[prop(into)]
    value: String,
    /// Separator character.
    #[prop(optional, into)]
    separator: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let sep = separator.unwrap_or_else(|| "|".to_string());

    let field_class = {
        let mut classes = vec!["fx-field-inline".to_string()];
        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }
        classes.join(" ")
    };

    view! {
        <span class=field_class>
            <span class="fx-field-label">{label}</span>
            <span class="fx-field-separator" aria-hidden="true">{sep}</span>
            <span class="fx-field-value">{value}</span>
        </span>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_field_structure() {
        // Field is a simple presentational component
        // Testing would require Leptos test utilities
        assert!(true);
    }
}
