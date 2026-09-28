//! Schema-driven form component
//!
//! Renders a complete form based on a ContentSchema definition.

use leptos::prelude::*;
use crate::api::schema_types::*;
use super::FieldRenderer;

const FORM_STYLE: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 0.5rem; padding: 1.5rem;";
// Note: Don't use pointer-events: none here - it prevents expand/collapse from working
// Instead, pass read_only to child components so they can disable editing while allowing view interactions
const FORM_STYLE_READONLY: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 0.5rem; padding: 1.5rem; opacity: 0.9;";
const SECTION_STYLE: &str = "margin-bottom: 1.5rem;";
const SECTION_TITLE_STYLE: &str = "color: #e2e8f0; font-size: 1rem; font-weight: 600; margin-bottom: 1rem; padding-bottom: 0.5rem; border-bottom: 1px solid #1e293b;";
const VALIDATION_ERROR_STYLE: &str = "background: rgba(239, 68, 68, 0.1); border: 1px solid #ef4444; border-radius: 0.375rem; padding: 0.75rem; margin-bottom: 1rem;";
const ERROR_LIST_STYLE: &str = "color: #ef4444; font-size: 0.875rem; margin: 0; padding-left: 1rem;";

/// Schema-driven form that renders fields based on schema definition
#[component]
pub fn SchemaForm(
    /// The schema defining the form structure
    schema: ContentSchema,
    /// Current form data
    data: RwSignal<serde_json::Value>,
    /// Validation errors
    #[prop(optional)]
    errors: Option<Signal<Vec<ValidationError>>>,
    /// On change callback (called when any field changes)
    #[prop(optional)]
    on_change: Option<Callback<serde_json::Value>>,
    /// Whether form is read-only
    #[prop(default = false)]
    read_only: bool,
) -> impl IntoView {
    let fields = schema.fields.clone();

    let form_style = if read_only { FORM_STYLE_READONLY } else { FORM_STYLE };

    view! {
        <div style=form_style>
            // Validation errors summary
            {move || {
                if let Some(errs) = errors {
                    let error_list = errs.get();
                    if !error_list.is_empty() {
                        view! {
                            <div style=VALIDATION_ERROR_STYLE>
                                <ul style=ERROR_LIST_STYLE>
                                    {error_list.iter().map(|err| {
                                        view! {
                                            <li>{format!("{}: {}", err.field, err.message)}</li>
                                        }
                                    }).collect_view()}
                                </ul>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Render fields in schema order (preserving the order defined in the schema)
            <div style=SECTION_STYLE>
                {fields.iter().map(|field| {
                    let field_key = field.key.clone();
                    let field_clone = field.clone();

                    // Create signal for this field's value
                    let field_value = RwSignal::new(
                        data.get().get(&field_key).cloned().unwrap_or(serde_json::Value::Null)
                    );

                    // On change handler for this field
                    let on_field_change = Callback::new(move |new_value: serde_json::Value| {
                        let mut current_data = data.get();
                        if let serde_json::Value::Object(ref mut map) = current_data {
                            map.insert(field_key.clone(), new_value);
                        } else {
                            let mut map = serde_json::Map::new();
                            map.insert(field_key.clone(), new_value);
                            current_data = serde_json::Value::Object(map);
                        }
                        data.set(current_data.clone());
                        if let Some(cb) = on_change {
                            cb.run(current_data);
                        }
                    });

                    view! {
                        <FieldRenderer
                            field=field_clone
                            value=field_value
                            on_change=on_field_change
                            read_only=read_only
                        />
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Validation error type
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: String,
}

/// Validate form data against schema
pub fn validate_form_data(schema: &ContentSchema, data: &serde_json::Value) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    for field in &schema.fields {
        let value = data.get(&field.key);

        // Check required
        if field.required {
            let is_empty = match value {
                None => true,
                Some(v) => v.is_null() || (v.is_string() && v.as_str().unwrap_or("").is_empty()),
            };

            if is_empty {
                errors.push(ValidationError {
                    field: field.key.clone(),
                    message: format!("{} is required", field.label),
                    code: "required".to_string(),
                });
                continue;
            }
        }

        // Type-specific validation
        if let Some(v) = value {
            if let Some(err) = validate_field_value(v, &field.field_type, &field.key, &field.label) {
                errors.push(err);
            }
        }
    }

    errors
}

fn validate_field_value(
    value: &serde_json::Value,
    field_type: &FieldType,
    key: &str,
    label: &str,
) -> Option<ValidationError> {
    match field_type {
        FieldType::Text(config) => {
            if let Some(s) = value.as_str() {
                if let Some(min) = config.min_length {
                    if s.len() < min {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must be at least {} characters", label, min),
                            code: "min_length".to_string(),
                        });
                    }
                }
                if let Some(max) = config.max_length {
                    if s.len() > max {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must be at most {} characters", label, max),
                            code: "max_length".to_string(),
                        });
                    }
                }
            }
        }
        FieldType::Number(config) => {
            if let Some(n) = value.as_f64() {
                if let Some(min) = config.min {
                    if n < min {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must be at least {}", label, min),
                            code: "min".to_string(),
                        });
                    }
                }
                if let Some(max) = config.max {
                    if n > max {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must be at most {}", label, max),
                            code: "max".to_string(),
                        });
                    }
                }
            }
        }
        FieldType::Repeater(config) => {
            if let Some(arr) = value.as_array() {
                if let Some(min) = config.min_items {
                    if arr.len() < min {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must have at least {} items", label, min),
                            code: "min_items".to_string(),
                        });
                    }
                }
                if let Some(max) = config.max_items {
                    if arr.len() > max {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must have at most {} items", label, max),
                            code: "max_items".to_string(),
                        });
                    }
                }
            }
        }
        _ => {}
    }

    None
}
