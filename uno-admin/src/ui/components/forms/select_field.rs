//! Select and MultiSelect field components

use leptos::prelude::*;
use crate::api::schema_types::{SelectFieldConfig, MultiSelectFieldConfig, SelectOption};

const LABEL_STYLE: &str = "display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;";
const SELECT_STYLE: &str = "width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none; cursor: pointer;";
const HELP_TEXT_STYLE: &str = "color: #64748b; font-size: 0.75rem; margin-top: 0.25rem;";
const REQUIRED_STYLE: &str = "color: #ef4444; margin-left: 0.25rem;";
const CHIP_CONTAINER_STYLE: &str = "display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 0.5rem;";
const CHIP_STYLE: &str = "display: inline-flex; align-items: center; gap: 0.25rem; background: #334155; color: #e2e8f0; padding: 0.25rem 0.5rem; border-radius: 0.25rem; font-size: 0.75rem;";
const CHIP_REMOVE_STYLE: &str = "background: transparent; border: none; color: #94a3b8; cursor: pointer; padding: 0; margin-left: 0.25rem; font-size: 1rem; line-height: 1;";
const CHECKBOX_CONTAINER_STYLE: &str = "display: flex; flex-direction: column; gap: 0.5rem; margin-top: 0.5rem;";
const CHECKBOX_ITEM_STYLE: &str = "display: flex; align-items: center; gap: 0.5rem;";
const CHECKBOX_STYLE: &str = "width: 1rem; height: 1rem; accent-color: #3b82f6;";

/// Single-select dropdown field
#[component]
pub fn SelectField(
    /// Field key for identification
    #[prop(into)]
    field_key: String,
    /// Field label
    #[prop(into)]
    label: String,
    /// Current value
    value: RwSignal<String>,
    /// Field configuration
    config: SelectFieldConfig,
    /// Description/help text
    
    description: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<String>>,
) -> impl IntoView {
    let options = config.options.clone();

    let on_select = move |ev: web_sys::Event| {
        let selected = event_target_value(&ev);
        value.set(selected.clone());
        if let Some(cb) = on_change {
            cb.run(selected);
        }
    };

    view! {
        <div class="form-field" style="margin-bottom: 1rem;">
            <label style=LABEL_STYLE>
                {label.clone()}
                {move || if required {
                    view! { <span style=REQUIRED_STYLE>"*"</span> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </label>

            <select
                style=SELECT_STYLE
                prop:value=move || value.get()
                on:change=on_select
            >
                <option value="" disabled selected=move || value.get().is_empty()>
                    "Select an option..."
                </option>
                {options.iter().map(|opt| {
                    let opt_value = opt.value.clone();
                    let opt_value_for_selected = opt.value.clone();
                    let opt_label = opt.label.clone();
                    view! {
                        <option value=opt_value selected=move || value.get() == opt_value_for_selected>
                            {opt_label}
                        </option>
                    }
                }).collect_view()}
            </select>

            {move || {
                if let Some(ref desc) = description {
                    view! { <p style=HELP_TEXT_STYLE>{desc.clone()}</p> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}

/// Multi-select field with checkboxes
#[component]
pub fn MultiSelectField(
    /// Field key for identification
    #[prop(into)]
    field_key: String,
    /// Field label
    #[prop(into)]
    label: String,
    /// Current values
    value: RwSignal<Vec<String>>,
    /// Field configuration
    config: MultiSelectFieldConfig,
    /// Description/help text
    description: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<Vec<String>>>,
) -> impl IntoView {
    let options = config.options.clone();
    let min_selections = config.min_selections;
    let max_selections = config.max_selections;

    let toggle_option = move |opt_value: String| {
        let mut current = value.get();
        if current.contains(&opt_value) {
            current.retain(|v| v != &opt_value);
        } else {
            // Check max selections
            if let Some(max) = max_selections {
                if current.len() >= max {
                    return;
                }
            }
            current.push(opt_value);
        }
        value.set(current.clone());
        if let Some(cb) = on_change {
            cb.run(current);
        }
    };

    let remove_option = move |opt_value: String| {
        let mut current = value.get();
        current.retain(|v| v != &opt_value);
        value.set(current.clone());
        if let Some(cb) = on_change {
            cb.run(current);
        }
    };

    view! {
        <div class="form-field" style="margin-bottom: 1rem;">
            <label style=LABEL_STYLE>
                {label.clone()}
                {move || if required {
                    view! { <span style=REQUIRED_STYLE>"*"</span> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </label>

            // Selected chips
            <div style=CHIP_CONTAINER_STYLE>
                {
                    let options_for_chips = options.clone();
                    move || {
                        let selected = value.get();
                        options_for_chips.iter()
                            .filter(|opt| selected.contains(&opt.value))
                            .map(|opt| {
                                let opt_value = opt.value.clone();
                                let opt_label = opt.label.clone();
                                let opt_value_for_remove = opt.value.clone();
                                view! {
                                    <span style=CHIP_STYLE>
                                        {opt_label}
                                        <button
                                            type="button"
                                            style=CHIP_REMOVE_STYLE
                                            on:click=move |_| remove_option(opt_value_for_remove.clone())
                                        >
                                            "x"
                                        </button>
                                    </span>
                                }
                            })
                            .collect_view()
                    }
                }
            </div>

            // Checkbox options
            <div style=CHECKBOX_CONTAINER_STYLE>
                {options.iter().map(|opt| {
                    let opt_value = opt.value.clone();
                    let opt_label = opt.label.clone();
                    let opt_value_for_check = opt.value.clone();
                    let opt_value_for_toggle = opt.value.clone();
                    view! {
                        <label style=CHECKBOX_ITEM_STYLE>
                            <input
                                type="checkbox"
                                style=CHECKBOX_STYLE
                                checked=move || value.get().contains(&opt_value_for_check)
                                on:change=move |_| toggle_option(opt_value_for_toggle.clone())
                            />
                            <span style="color: #e2e8f0; font-size: 0.875rem;">{opt_label}</span>
                        </label>
                    }
                }).collect_view()}
            </div>

            {move || {
                let help = if let Some(ref desc) = description {
                    desc.clone()
                } else if min_selections.is_some() || max_selections.is_some() {
                    let mut constraints = Vec::new();
                    if let Some(min) = min_selections {
                        constraints.push(format!("min: {}", min));
                    }
                    if let Some(max) = max_selections {
                        constraints.push(format!("max: {}", max));
                    }
                    constraints.join(", ")
                } else {
                    return view! { <span></span> }.into_any();
                };

                view! { <p style=HELP_TEXT_STYLE>{help}</p> }.into_any()
            }}
        </div>
    }
}
