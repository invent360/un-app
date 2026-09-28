//! List field component for simple string lists

use leptos::prelude::*;
use crate::api::schema_types::ListFieldConfig;

const LABEL_STYLE: &str = "display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;";
const INPUT_STYLE: &str = "flex: 1; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;";
const ADD_BTN_STYLE: &str = "background: #3b82f6; border: none; border-radius: 0.375rem; padding: 0.625rem 1rem; color: white; font-size: 0.875rem; cursor: pointer;";
const ITEM_STYLE: &str = "display: flex; align-items: center; gap: 0.5rem; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.5rem 0.75rem;";
const REMOVE_BTN_STYLE: &str = "background: transparent; border: none; color: #ef4444; cursor: pointer; padding: 0.25rem; font-size: 1rem;";
const HELP_TEXT_STYLE: &str = "color: #64748b; font-size: 0.75rem; margin-top: 0.25rem;";
const REQUIRED_STYLE: &str = "color: #ef4444; margin-left: 0.25rem;";
const LIST_CONTAINER_STYLE: &str = "display: flex; flex-direction: column; gap: 0.5rem; margin-top: 0.5rem;";

#[component]
pub fn ListField(
    /// Field key for identification
    #[prop(into)]
    field_key: String,
    /// Field label
    #[prop(into)]
    label: String,
    /// Current values
    value: RwSignal<Vec<String>>,
    /// Field configuration
    #[prop(optional)]
    config: Option<ListFieldConfig>,
    /// Description/help text
    
    description: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// Placeholder for input
    #[prop(optional)]
    placeholder: Option<String>,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<Vec<String>>>,
) -> impl IntoView {
    let config = config.unwrap_or_default();
    let min_items = config.min_items;
    let max_items = config.max_items;
    let placeholder = placeholder.unwrap_or_else(|| "Add new item...".to_string());

    let new_item = RwSignal::new(String::new());

    let do_add_item = move || {
        let item = new_item.get().trim().to_string();
        if item.is_empty() {
            return;
        }

        let mut current = value.get();

        // Check max items
        if let Some(max) = max_items {
            if current.len() >= max {
                return;
            }
        }

        current.push(item);
        value.set(current.clone());
        new_item.set(String::new());

        if let Some(cb) = on_change {
            cb.run(current);
        }
    };

    let remove_item = move |index: usize| {
        let mut current = value.get();
        if index < current.len() {
            current.remove(index);
            value.set(current.clone());
            if let Some(cb) = on_change {
                cb.run(current);
            }
        }
    };

    let on_keypress = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            ev.prevent_default();
            do_add_item();
        }
    };

    let add_item = move |_: web_sys::MouseEvent| {
        do_add_item();
    };

    let can_add = move || {
        if let Some(max) = max_items {
            value.get().len() < max
        } else {
            true
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

            // Add new item input
            <div style="display: flex; gap: 0.5rem;">
                <input
                    type="text"
                    style=INPUT_STYLE
                    prop:value=move || new_item.get()
                    on:input=move |ev| new_item.set(event_target_value(&ev))
                    on:keypress=on_keypress
                    placeholder=placeholder
                    disabled=move || !can_add()
                />
                <button
                    type="button"
                    style=ADD_BTN_STYLE
                    on:click=add_item
                    disabled=move || !can_add() || new_item.get().trim().is_empty()
                >
                    "Add"
                </button>
            </div>

            // List of items
            <div style=LIST_CONTAINER_STYLE>
                {move || {
                    value.get().iter().enumerate().map(|(index, item)| {
                        let item_text = item.clone();
                        view! {
                            <div style=ITEM_STYLE>
                                <span style="flex: 1; color: #e2e8f0; font-size: 0.875rem;">{item_text}</span>
                                <button
                                    type="button"
                                    style=REMOVE_BTN_STYLE
                                    on:click=move |_| remove_item(index)
                                >
                                    "x"
                                </button>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>

            {move || {
                let count = value.get().len();
                let constraints = vec![
                    min_items.map(|m| format!("min: {}", m)),
                    max_items.map(|m| format!("max: {}", m)),
                ].into_iter().flatten().collect::<Vec<_>>().join(", ");

                let help = if let Some(ref desc) = description {
                    format!("{} ({})", desc, if constraints.is_empty() { format!("{} items", count) } else { format!("{} items, {}", count, constraints) })
                } else if !constraints.is_empty() {
                    format!("{} items, {}", count, constraints)
                } else {
                    format!("{} items", count)
                };

                view! { <p style=HELP_TEXT_STYLE>{help}</p> }
            }}
        </div>
    }
}
