//! Reference picker component for selecting content items
//!
//! Provides a searchable dropdown for selecting referenced content.

use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::api::schema_types::{ReferenceFieldConfig, ReferenceListFieldConfig, ContentItemSummary};
use crate::api::schema_client::list_content_items;

const LABEL_STYLE: &str = "display: block; font-size: 0.875rem; font-weight: 500; color: #334155; margin-bottom: 0.375rem;";
const INPUT_STYLE: &str = "width: 100%; padding: 0.625rem 0.875rem; background: white; border: 1px solid #cbd5e1; border-radius: 0.5rem; font-size: 0.875rem; color: #1e293b; outline: none;";
const DARK_LABEL_STYLE: &str = "display: block; font-size: 0.875rem; font-weight: 500; color: #94a3b8; margin-bottom: 0.375rem;";
const DARK_INPUT_STYLE: &str = "width: 100%; padding: 0.625rem 0.875rem; background: #1e293b; border: 1px solid #334155; border-radius: 0.5rem; font-size: 0.875rem; color: #e2e8f0; outline: none;";

/// Reference picker for single content reference
#[component]
pub fn ReferencePicker(
    /// Field key for form binding
    field_key: String,
    /// Display label
    label: String,
    /// Current selected content ID
    value: RwSignal<String>,
    /// Reference field configuration
    config: ReferenceFieldConfig,
    /// Optional description
    description: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<String>>,
) -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let (is_open, set_is_open) = signal(false);
    let (selected_title, set_selected_title) = signal(String::new());
    let (items, set_items) = signal(Vec::<ContentItemSummary>::new());
    let (is_loading, set_is_loading) = signal(false);

    let allowed_types = config.allowed_types.clone();
    let display_field = config.display_field.clone();

    // Search for content items
    let search_items = move |query: String| {
        let types = allowed_types.clone();
        set_is_loading.set(true);

        spawn_local(async move {
            // Search in first allowed type, or all if none specified
            let schema_filter = types.first().cloned();
            let search = if query.is_empty() { None } else { Some(query) };

            match list_content_items(schema_filter, None, search, Some(1), Some(20)).await {
                Ok(response) => {
                    set_items.set(response.items);
                }
                Err(_) => {
                    set_items.set(vec![]);
                }
            }
            set_is_loading.set(false);
        });
    };

    // Initialize search on open
    let init_search = search_items.clone();
    Effect::new(move |_| {
        if is_open.get() {
            init_search(String::new());
        }
    });

    // Handle selection
    let select_item = move |item: ContentItemSummary| {
        value.set(item.id.clone());
        set_selected_title.set(item.title.clone());
        set_is_open.set(false);
        if let Some(cb) = on_change {
            cb.run(item.id);
        }
    };

    // Clear selection
    let clear_selection = move |_: web_sys::MouseEvent| {
        value.set(String::new());
        set_selected_title.set(String::new());
        if let Some(cb) = on_change {
            cb.run(String::new());
        }
    };

    view! {
        <div class="reference-picker" style="position: relative;">
            <label style=DARK_LABEL_STYLE>
                {label}
                {required.then(|| view! { <span style="color: #ef4444; margin-left: 4px;">"*"</span> })}
            </label>

            // Selected value display / trigger
            <div
                style="display: flex; align-items: center; gap: 8px;"
            >
                <div
                    style=format!("{}; cursor: pointer; flex: 1;", DARK_INPUT_STYLE)
                    on:click=move |_| set_is_open.update(|v| *v = !*v)
                >
                    {move || {
                        let v = value.get();
                        let title = selected_title.get();
                        if v.is_empty() {
                            view! { <span style="color: #64748b;">"Select content..."</span> }.into_any()
                        } else if !title.is_empty() {
                            view! { <span>{title}</span> }.into_any()
                        } else {
                            view! { <span style="color: #94a3b8;">{format!("ID: {}", v)}</span> }.into_any()
                        }
                    }}
                </div>
                {move || (!value.get().is_empty()).then(|| view! {
                    <button
                        style="padding: 8px; background: transparent; border: none; color: #94a3b8; cursor: pointer;"
                        on:click=clear_selection
                        title="Clear"
                    >
                        "×"
                    </button>
                })}
            </div>

            // Dropdown
            {move || is_open.get().then(|| {
                let search_fn = search_items.clone();
                view! {
                    <div style="position: absolute; top: 100%; left: 0; right: 0; z-index: 50; margin-top: 4px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; box-shadow: 0 10px 25px rgba(0,0,0,0.3); max-height: 300px; overflow: hidden;">
                        // Search input
                        <div style="padding: 8px; border-bottom: 1px solid #334155;">
                            <input
                                type="text"
                                style=DARK_INPUT_STYLE
                                placeholder="Search content..."
                                prop:value=move || search_query.get()
                                on:input=move |ev| {
                                    let q = event_target_value(&ev);
                                    set_search_query.set(q.clone());
                                    search_fn(q);
                                }
                            />
                        </div>

                        // Results
                        <div style="max-height: 240px; overflow-y: auto;">
                            {move || {
                                if is_loading.get() {
                                    view! {
                                        <div style="padding: 16px; text-align: center; color: #64748b;">
                                            "Loading..."
                                        </div>
                                    }.into_any()
                                } else {
                                    let item_list = items.get();
                                    if item_list.is_empty() {
                                        view! {
                                            <div style="padding: 16px; text-align: center; color: #64748b;">
                                                "No content found"
                                            </div>
                                        }.into_any()
                                    } else {
                                        item_list.into_iter().map(|item| {
                                            let item_for_click = item.clone();
                                            let is_selected = item.id == value.get();
                                            view! {
                                                <div
                                                    style=move || format!(
                                                        "padding: 10px 12px; cursor: pointer; border-bottom: 1px solid #334155; {}",
                                                        if is_selected { "background: #334155;" } else { "" }
                                                    )
                                                    class="hover:bg-slate-700"
                                                    on:click=move |_| select_item(item_for_click.clone())
                                                >
                                                    <div style="font-weight: 500; color: #e2e8f0; font-size: 14px;">
                                                        {item.title.clone()}
                                                    </div>
                                                    <div style="font-size: 12px; color: #64748b; display: flex; gap: 8px; margin-top: 2px;">
                                                        <span>{item.schema_id.clone()}</span>
                                                        <span>"•"</span>
                                                        <span>{item.status.display_name()}</span>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }
                            }}
                        </div>
                    </div>
                }
            })}

            // Description
            {description.map(|desc| view! {
                <p style="color: #64748b; font-size: 0.75rem; margin-top: 0.25rem;">{desc}</p>
            })}
        </div>
    }
}

/// Reference list picker for multiple content references
#[component]
pub fn ReferenceListPicker(
    /// Field key for form binding
    field_key: String,
    /// Display label
    label: String,
    /// Current selected content IDs (JSON array)
    value: RwSignal<serde_json::Value>,
    /// Reference list field configuration
    config: ReferenceListFieldConfig,
    /// Optional description
    description: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<serde_json::Value>>,
) -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let (is_open, set_is_open) = signal(false);
    let (items, set_items) = signal(Vec::<ContentItemSummary>::new());
    let (is_loading, set_is_loading) = signal(false);
    let selected_items = RwSignal::new(Vec::<(String, String)>::new()); // (id, title)

    let allowed_types = config.allowed_types.clone();

    // Parse initial value
    Effect::new(move |_| {
        let current = value.get();
        if let Some(arr) = current.as_array() {
            let ids: Vec<(String, String)> = arr.iter()
                .filter_map(|v| v.as_str().map(|s| (s.to_string(), s.to_string())))
                .collect();
            selected_items.set(ids);
        }
    });

    // Search for content items
    let search_items = move |query: String| {
        let types = allowed_types.clone();
        set_is_loading.set(true);

        spawn_local(async move {
            let schema_filter = types.first().cloned();
            let search = if query.is_empty() { None } else { Some(query) };

            match list_content_items(schema_filter, None, search, Some(1), Some(20)).await {
                Ok(response) => {
                    set_items.set(response.items);
                }
                Err(_) => {
                    set_items.set(vec![]);
                }
            }
            set_is_loading.set(false);
        });
    };

    // Initialize search on open
    let init_search = search_items.clone();
    Effect::new(move |_| {
        if is_open.get() {
            init_search(String::new());
        }
    });

    // Toggle item selection
    let toggle_item = move |item: ContentItemSummary| {
        let mut current = selected_items.get();
        if let Some(pos) = current.iter().position(|(id, _)| *id == item.id) {
            current.remove(pos);
        } else {
            current.push((item.id.clone(), item.title.clone()));
        }
        selected_items.set(current.clone());

        // Update value
        let ids: Vec<serde_json::Value> = current.iter()
            .map(|(id, _)| serde_json::Value::String(id.clone()))
            .collect();
        let json_value = serde_json::Value::Array(ids);
        value.set(json_value.clone());

        if let Some(cb) = on_change {
            cb.run(json_value);
        }
    };

    // Remove item
    let remove_item = move |id: String| {
        let mut current = selected_items.get();
        current.retain(|(item_id, _)| *item_id != id);
        selected_items.set(current.clone());

        let ids: Vec<serde_json::Value> = current.iter()
            .map(|(id, _)| serde_json::Value::String(id.clone()))
            .collect();
        let json_value = serde_json::Value::Array(ids);
        value.set(json_value.clone());

        if let Some(cb) = on_change {
            cb.run(json_value);
        }
    };

    view! {
        <div class="reference-list-picker" style="position: relative;">
            <label style=DARK_LABEL_STYLE>
                {label}
                {required.then(|| view! { <span style="color: #ef4444; margin-left: 4px;">"*"</span> })}
            </label>

            // Selected items
            <div style="display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 8px;">
                {move || selected_items.get().iter().map(|(id, title)| {
                    let id_for_remove = id.clone();
                    let display = if title.is_empty() || title == id { id.clone() } else { title.clone() };
                    view! {
                        <div style="display: flex; align-items: center; gap: 4px; background: #334155; padding: 4px 8px; border-radius: 4px; font-size: 13px; color: #e2e8f0;">
                            {display}
                            <button
                                style="background: transparent; border: none; color: #94a3b8; cursor: pointer; padding: 0 4px;"
                                on:click=move |_| remove_item(id_for_remove.clone())
                            >
                                "×"
                            </button>
                        </div>
                    }
                }).collect_view()}
            </div>

            // Add button
            <button
                style="width: 100%; padding: 10px; background: #1e293b; border: 1px dashed #334155; border-radius: 8px; color: #94a3b8; cursor: pointer; font-size: 14px;"
                on:click=move |_| set_is_open.update(|v| *v = !*v)
            >
                "+ Add Reference"
            </button>

            // Dropdown
            {move || is_open.get().then(|| {
                let search_fn = search_items.clone();
                view! {
                    <div style="position: absolute; top: 100%; left: 0; right: 0; z-index: 50; margin-top: 4px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; box-shadow: 0 10px 25px rgba(0,0,0,0.3); max-height: 300px; overflow: hidden;">
                        // Search input
                        <div style="padding: 8px; border-bottom: 1px solid #334155;">
                            <input
                                type="text"
                                style=DARK_INPUT_STYLE
                                placeholder="Search content..."
                                prop:value=move || search_query.get()
                                on:input=move |ev| {
                                    let q = event_target_value(&ev);
                                    set_search_query.set(q.clone());
                                    search_fn(q);
                                }
                            />
                        </div>

                        // Results
                        <div style="max-height: 240px; overflow-y: auto;">
                            {move || {
                                if is_loading.get() {
                                    view! {
                                        <div style="padding: 16px; text-align: center; color: #64748b;">
                                            "Loading..."
                                        </div>
                                    }.into_any()
                                } else {
                                    let item_list = items.get();
                                    if item_list.is_empty() {
                                        view! {
                                            <div style="padding: 16px; text-align: center; color: #64748b;">
                                                "No content found"
                                            </div>
                                        }.into_any()
                                    } else {
                                        let selected = selected_items.get();
                                        item_list.into_iter().map(|item| {
                                            let item_for_click = item.clone();
                                            let is_selected = selected.iter().any(|(id, _)| *id == item.id);
                                            view! {
                                                <div
                                                    style=move || format!(
                                                        "padding: 10px 12px; cursor: pointer; border-bottom: 1px solid #334155; display: flex; align-items: center; gap: 8px; {}",
                                                        if is_selected { "background: #334155;" } else { "" }
                                                    )
                                                    class="hover:bg-slate-700"
                                                    on:click=move |_| toggle_item(item_for_click.clone())
                                                >
                                                    <input
                                                        type="checkbox"
                                                        checked=is_selected
                                                        style="width: 16px; height: 16px;"
                                                    />
                                                    <div style="flex: 1;">
                                                        <div style="font-weight: 500; color: #e2e8f0; font-size: 14px;">
                                                            {item.title.clone()}
                                                        </div>
                                                        <div style="font-size: 12px; color: #64748b;">
                                                            {item.schema_id.clone()}
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }
                            }}
                        </div>

                        // Done button
                        <div style="padding: 8px; border-top: 1px solid #334155;">
                            <button
                                style="width: 100%; padding: 8px; background: #3b82f6; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 14px;"
                                on:click=move |_| set_is_open.set(false)
                            >
                                "Done"
                            </button>
                        </div>
                    </div>
                }
            })}

            // Description
            {description.map(|desc| view! {
                <p style="color: #64748b; font-size: 0.75rem; margin-top: 0.25rem;">{desc}</p>
            })}
        </div>
    }
}
