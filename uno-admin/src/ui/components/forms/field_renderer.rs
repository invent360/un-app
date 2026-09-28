//! Field renderer component that dispatches to the appropriate field type

use leptos::prelude::*;
use crate::api::schema_types::*;
use super::*;

/// Renders a field based on its definition
#[component]
pub fn FieldRenderer(
    /// Field definition from schema
    field: FieldDefinition,
    /// Current value (as JSON)
    value: RwSignal<serde_json::Value>,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<serde_json::Value>>,
    /// Whether the field is read-only (view mode)
    #[prop(default = false)]
    read_only: bool,
) -> impl IntoView {
    let field_key = field.key.clone();
    let label = field.label.clone();
    let description = field.description.clone();
    let required = field.required;
    let field_type = field.field_type.clone();

    // Create typed signals that sync with the JSON value
    match field_type {
        FieldType::Text(config) => {
            let text_value = RwSignal::new(
                value.get().as_str().unwrap_or_default().to_string()
            );

            // Sync text changes back to JSON
            let on_text_change = Callback::new(move |new_text: String| {
                let json_value = serde_json::Value::String(new_text);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            view! {
                <TextField
                    field_key=field_key
                    label=label
                    value=text_value
                    config=config
                    description=description
                    required=required
                    on_change=on_text_change
                />
            }.into_any()
        }

        FieldType::RichText(config) => {
            let text_value = RwSignal::new(
                value.get().as_str().unwrap_or_default().to_string()
            );

            let on_text_change = Callback::new(move |new_text: String| {
                let json_value = serde_json::Value::String(new_text);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            view! {
                <RichTextField
                    field_key=field_key
                    label=label
                    value=text_value
                    config=config
                    description=description
                    required=required
                    on_change=on_text_change
                />
            }.into_any()
        }

        FieldType::Number(config) => {
            let num_value = RwSignal::new(value.get().as_f64());

            let on_num_change = Callback::new(move |new_num: Option<f64>| {
                let json_value = new_num
                    .map(|n| serde_json::json!(n))
                    .unwrap_or(serde_json::Value::Null);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            view! {
                <NumberField
                    field_key=field_key
                    label=label
                    value=num_value
                    config=config
                    description=description
                    required=required
                    on_change=on_num_change
                />
            }.into_any()
        }

        FieldType::Boolean(config) => {
            let bool_value = RwSignal::new(
                value.get().as_bool().unwrap_or(config.default.unwrap_or(false))
            );

            let on_bool_change = Callback::new(move |new_bool: bool| {
                let json_value = serde_json::Value::Bool(new_bool);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            view! {
                <BooleanField
                    field_key=field_key
                    label=label
                    value=bool_value
                    config=config
                    description=description
                    on_change=on_bool_change
                />
            }.into_any()
        }

        FieldType::Select(config) => {
            let select_value = RwSignal::new(
                value.get().as_str().unwrap_or_default().to_string()
            );

            let on_select_change = Callback::new(move |new_value: String| {
                let json_value = serde_json::Value::String(new_value);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            view! {
                <SelectField
                    field_key=field_key
                    label=label
                    value=select_value
                    config=config
                    description=description
                    required=required
                    on_change=on_select_change
                />
            }.into_any()
        }

        FieldType::MultiSelect(config) => {
            let multi_value = RwSignal::new(
                value.get().as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default()
            );

            let on_multi_change = Callback::new(move |new_values: Vec<String>| {
                let json_value = serde_json::json!(new_values);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            view! {
                <MultiSelectField
                    field_key=field_key
                    label=label
                    value=multi_value
                    config=config
                    description=description
                    required=required
                    on_change=on_multi_change
                />
            }.into_any()
        }

        FieldType::List(config) => {
            let list_value = RwSignal::new(
                value.get().as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default()
            );

            let on_list_change = Callback::new(move |new_values: Vec<String>| {
                let json_value = serde_json::json!(new_values);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            view! {
                <ListField
                    field_key=field_key
                    label=label
                    value=list_value
                    config=config
                    description=description
                    required=required
                    on_change=on_list_change
                />
            }.into_any()
        }

        FieldType::Media(config) => {
            let media_value = RwSignal::new(
                value.get().as_str().map(String::from)
            );

            let on_media_change = Callback::new(move |new_value: Option<String>| {
                let json_value = new_value
                    .map(|v| serde_json::Value::String(v))
                    .unwrap_or(serde_json::Value::Null);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            view! {
                <MediaField
                    field_key=field_key
                    label=label
                    value=media_value
                    config=config
                    description=description
                    required=required
                    on_change=on_media_change
                />
            }.into_any()
        }

        FieldType::MediaList(config) => {
            // Use MediaListField for multiple image/file uploads with GCS support
            let list_value = RwSignal::new(
                value.get().as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default()
            );

            let on_list_change = Callback::new(move |new_values: Vec<String>| {
                let json_value = serde_json::json!(new_values);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            view! {
                <MediaListField
                    field_key=field_key
                    label=label
                    value=list_value
                    config=config
                    description=description
                    required=required
                    on_change=on_list_change
                />
            }.into_any()
        }

        FieldType::Repeater(config) => {
            // Check if a custom UI component is specified
            if config.ui_component.as_deref() == Some("section_editor") {
                // Use the SectionEditor component for Guide sections
                let repeater_value = RwSignal::new(
                    value.get().as_array().cloned().unwrap_or_default()
                );

                let on_section_change = Callback::new(move |new_values: Vec<serde_json::Value>| {
                    let json_value = serde_json::json!(new_values);
                    value.set(json_value.clone());
                    if let Some(cb) = on_change {
                        cb.run(json_value);
                    }
                });

                return view! {
                    <SectionEditor
                        value=repeater_value
                        label=label
                        required=required
                        on_change=on_section_change
                        read_only=read_only
                    />
                }.into_any();
            }

            // Use TaskSectionEditor for Task content sections
            if config.ui_component.as_deref() == Some("task_section_editor") {
                let repeater_value = RwSignal::new(
                    value.get().as_array().cloned().unwrap_or_default()
                );

                let on_section_change = Callback::new(move |new_values: Vec<serde_json::Value>| {
                    let json_value = serde_json::json!(new_values);
                    value.set(json_value.clone());
                    if let Some(cb) = on_change {
                        cb.run(json_value);
                    }
                });

                return view! {
                    <TaskSectionEditor
                        value=repeater_value
                        label=label
                        required=required
                        on_change=on_section_change
                        read_only=read_only
                    />
                }.into_any();
            }

            // Use HomeSectionEditor for Home page sections
            if config.ui_component.as_deref() == Some("home_section_editor") {
                let repeater_value = RwSignal::new(
                    value.get().as_array().cloned().unwrap_or_default()
                );

                let on_section_change = Callback::new(move |new_values: Vec<serde_json::Value>| {
                    let json_value = serde_json::json!(new_values);
                    value.set(json_value.clone());
                    if let Some(cb) = on_change {
                        cb.run(json_value);
                    }
                });

                return view! {
                    <HomeSectionEditor
                        value=repeater_value
                        label=label
                        on_change=on_section_change
                        read_only=read_only
                    />
                }.into_any();
            }

            // Default repeater behavior
            let repeater_value = RwSignal::new(
                value.get().as_array().cloned().unwrap_or_default()
            );

            let nested_fields = config.fields.clone();

            let on_repeater_change = Callback::new(move |new_values: Vec<serde_json::Value>| {
                let json_value = serde_json::json!(new_values);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            // Render function for nested fields - dispatch to appropriate field components
            let render_nested = Callback::new(move |(_index, fields, item_signal): (usize, Vec<FieldDefinition>, RwSignal<serde_json::Value>)| {
                view! {
                    <div>
                        {fields.iter().map(|nested_field| {
                            let nested_key = nested_field.key.clone();
                            let nested_field_clone = nested_field.clone();

                            // Create a derived signal for this specific nested field's value
                            let nested_key_for_value = nested_key.clone();
                            let nested_value = RwSignal::new(
                                item_signal.get()
                                    .get(&nested_key_for_value)
                                    .cloned()
                                    .unwrap_or(serde_json::Value::Null)
                            );

                            // Keep nested_value in sync with item_signal (fixes signal snapshot issue)
                            let nested_key_for_sync = nested_key.clone();
                            Effect::new(move |_| {
                                let current_item = item_signal.get();
                                if let Some(field_value) = current_item.get(&nested_key_for_sync) {
                                    let current_nested = nested_value.get_untracked();
                                    if *field_value != current_nested {
                                        nested_value.set(field_value.clone());
                                    }
                                }
                            });

                            // Create callback to update the parent item when nested field changes
                            let nested_key_for_update = nested_key.clone();
                            let on_nested_change = Callback::new(move |new_value: serde_json::Value| {
                                let mut item = item_signal.get_untracked();
                                if let serde_json::Value::Object(ref mut map) = item {
                                    map.insert(nested_key_for_update.clone(), new_value);
                                } else {
                                    let mut map = serde_json::Map::new();
                                    map.insert(nested_key_for_update.clone(), new_value);
                                    item = serde_json::Value::Object(map);
                                }
                                item_signal.set(item);
                            });

                            // Use FieldRenderer to properly dispatch to the right component
                            view! {
                                <FieldRenderer
                                    field=nested_field_clone
                                    value=nested_value
                                    on_change=on_nested_change
                                />
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            });

            view! {
                <RepeaterField
                    field_key=field_key
                    label=label
                    value=repeater_value
                    config=config
                    description=description
                    required=required
                    on_change=on_repeater_change
                    render_fields=render_nested
                />
            }.into_any()
        }

        FieldType::Group(config) => {
            // Group renders as a section with nested fields
            let group_value = RwSignal::new(value.get().clone());

            view! {
                <div style="border: 1px solid #334155; border-radius: 0.375rem; padding: 1rem; margin-bottom: 1rem;">
                    <h4 style="color: #e2e8f0; font-size: 0.875rem; font-weight: 500; margin-bottom: 1rem;">{label}</h4>
                    {config.fields.iter().map(|nested_field| {
                        let nested_key = nested_field.key.clone();
                        let nested_field_clone = nested_field.clone();

                        let nested_value = RwSignal::new(
                            group_value.get().get(&nested_key).cloned().unwrap_or(serde_json::Value::Null)
                        );

                        let on_nested_change = Callback::new(move |new_value: serde_json::Value| {
                            let mut group = group_value.get();
                            if let serde_json::Value::Object(ref mut map) = group {
                                map.insert(nested_key.clone(), new_value.clone());
                            } else {
                                let mut map = serde_json::Map::new();
                                map.insert(nested_key.clone(), new_value.clone());
                                group = serde_json::Value::Object(map);
                            }
                            group_value.set(group.clone());
                            value.set(group.clone());
                            if let Some(cb) = on_change {
                                cb.run(group);
                            }
                        });

                        view! {
                            <FieldRenderer
                                field=nested_field_clone
                                value=nested_value
                                on_change=on_nested_change
                            />
                        }
                    }).collect_view()}
                </div>
            }.into_any()
        }

        FieldType::Date(config) => {
            // Use text input for date (simplified)
            let date_value = RwSignal::new(
                value.get().as_str().unwrap_or_default().to_string()
            );

            let on_date_change = Callback::new(move |new_date: String| {
                let json_value = serde_json::Value::String(new_date);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            let input_type = if config.include_time { "datetime-local" } else { "date" };

            view! {
                <div class="form-field" style="margin-bottom: 1rem;">
                    <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                        {label}
                        {move || if required {
                            view! { <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span> }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </label>
                    <input
                        type=input_type
                        style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                        prop:value=move || date_value.get()
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            date_value.set(val.clone());
                            on_date_change.run(val);
                        }
                    />
                    {move || {
                        if let Some(ref desc) = description {
                            view! { <p style="color: #64748b; font-size: 0.75rem; margin-top: 0.25rem;">{desc.clone()}</p> }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}
                </div>
            }.into_any()
        }

        FieldType::Reference(config) => {
            let ref_value = RwSignal::new(
                value.get().as_str().unwrap_or_default().to_string()
            );

            let on_ref_change = Callback::new(move |new_ref: String| {
                let json_value = serde_json::Value::String(new_ref);
                value.set(json_value.clone());
                if let Some(cb) = on_change {
                    cb.run(json_value);
                }
            });

            view! {
                <ReferencePicker
                    field_key=field_key
                    label=label
                    value=ref_value
                    config=config
                    description=description
                    required=required
                    on_change=on_ref_change
                />
            }.into_any()
        }

        FieldType::ReferenceList(config) => {
            let on_list_change = Callback::new(move |new_value: serde_json::Value| {
                value.set(new_value.clone());
                if let Some(cb) = on_change {
                    cb.run(new_value);
                }
            });

            view! {
                <ReferenceListPicker
                    field_key=field_key
                    label=label
                    value=value
                    config=config
                    description=description
                    required=required
                    on_change=on_list_change
                />
            }.into_any()
        }

        FieldType::Json(_config) => {
            // JSON field - render as a JSON textarea
            let json_str = RwSignal::new(
                serde_json::to_string_pretty(&value.get()).unwrap_or_default()
            );

            let on_json_change = Callback::new(move |new_text: String| {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&new_text) {
                    value.set(parsed.clone());
                    if let Some(cb) = on_change {
                        cb.run(parsed);
                    }
                }
                json_str.set(new_text);
            });

            view! {
                <div class="form-field" style="margin-bottom: 1rem;">
                    <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                        {label}
                        {move || if required {
                            view! { <span style="color: #ef4444; margin-left: 0.25rem;">"*"</span> }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </label>
                    <textarea
                        style="width: 100%; min-height: 150px; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; font-family: monospace; outline: none;"
                        prop:value=move || json_str.get()
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            on_json_change.run(val);
                        }
                    />
                    {move || {
                        if let Some(ref desc) = description {
                            view! { <p style="color: #64748b; font-size: 0.75rem; margin-top: 0.25rem;">{desc.clone()}</p> }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}
                </div>
            }.into_any()
        }
    }
}
