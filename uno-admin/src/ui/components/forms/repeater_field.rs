//! Repeater field component for dynamic lists of complex items

use leptos::prelude::*;
use crate::api::schema_types::{RepeaterFieldConfig, FieldDefinition};

const LABEL_STYLE: &str = "display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;";
const ITEM_CONTAINER_STYLE: &str = "border: 1px solid #334155; border-radius: 0.375rem; margin-bottom: 0.5rem; overflow: hidden;";
const ITEM_HEADER_STYLE: &str = "display: flex; justify-content: space-between; align-items: center; background: #0f172a; padding: 0.5rem 0.75rem; border-bottom: 1px solid #334155;";
const ITEM_TITLE_STYLE: &str = "color: #e2e8f0; font-size: 0.875rem; font-weight: 500;";
const ITEM_CONTENT_STYLE: &str = "background: #1e293b; padding: 1rem;";
const ITEM_ACTIONS_STYLE: &str = "display: flex; gap: 0.5rem;";
const ACTION_BTN_STYLE: &str = "background: transparent; border: none; color: #94a3b8; cursor: pointer; padding: 0.25rem; font-size: 0.875rem;";
const ADD_BTN_STYLE: &str = "background: #334155; border: 1px dashed #475569; border-radius: 0.375rem; padding: 0.75rem; color: #94a3b8; font-size: 0.875rem; cursor: pointer; width: 100%; text-align: center;";
const HELP_TEXT_STYLE: &str = "color: #64748b; font-size: 0.75rem; margin-top: 0.5rem;";
const REQUIRED_STYLE: &str = "color: #ef4444; margin-left: 0.25rem;";

/// Repeater field for complex nested items
#[component]
pub fn RepeaterField(
    /// Field key for identification
    #[prop(into)]
    field_key: String,
    /// Field label
    #[prop(into)]
    label: String,
    /// Current values (array of objects)
    value: RwSignal<Vec<serde_json::Value>>,
    /// Field configuration with nested field definitions
    config: RepeaterFieldConfig,
    /// Description/help text
    
    description: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// On change callback
    #[prop(optional)]
    on_change: Option<Callback<Vec<serde_json::Value>>>,
    /// Render function for nested fields
    /// This allows the parent to control how nested fields are rendered
    #[prop(optional)]
    render_fields: Option<Callback<(usize, Vec<FieldDefinition>, RwSignal<serde_json::Value>), AnyView>>,
) -> impl IntoView {
    let item_label = config.item_label.clone();
    let min_items = config.min_items;
    let max_items = config.max_items;
    let orderable = config.orderable;
    let collapsible = config.collapsible;
    let fields = config.fields.clone();

    // Track collapsed state for each item
    let collapsed_items = RwSignal::new(std::collections::HashSet::<usize>::new());

    // Use a stable signal map to persist item signals across renders
    // This prevents signal recreation when the parent value changes
    // Initialize with current items to avoid empty first render
    let initial_items = value.get_untracked();
    let initial_signals: Vec<RwSignal<serde_json::Value>> = initial_items
        .iter()
        .map(|item| RwSignal::new(item.clone()))
        .collect();

    let item_signals: RwSignal<Vec<RwSignal<serde_json::Value>>> = RwSignal::new(initial_signals);

    // Single Effect to sync all item signals to parent value
    // This watches item_signals and all individual signals
    Effect::new(move |_| {
        let signals = item_signals.get();
        let mut new_values: Vec<serde_json::Value> = Vec::with_capacity(signals.len());

        for sig in signals.iter() {
            new_values.push(sig.get());
        }

        // Compare with current value and update if different
        let current = value.get_untracked();
        if current != new_values {
            value.set(new_values.clone());
            if let Some(cb) = on_change {
                cb.run(new_values);
            }
        }
    });

    let add_item = move |_| {
        let current = value.get();

        // Check max items
        if let Some(max) = max_items {
            if current.len() >= max {
                return;
            }
        }

        // Create empty item with default values for each field
        let new_item = serde_json::json!({});

        // Create new signal and add to item_signals
        // The sync Effect will automatically propagate to value
        let sig = RwSignal::new(new_item);
        let mut signals = item_signals.get();
        signals.push(sig);
        item_signals.set(signals);
    };

    let remove_item = move |index: usize| {
        let signals = item_signals.get();

        // Check min items
        if let Some(min) = min_items {
            if signals.len() <= min {
                return;
            }
        }

        if index < signals.len() {
            let mut signals = signals;
            signals.remove(index);
            item_signals.set(signals);
            // The sync Effect will update value automatically
        }
    };

    let move_item = move |from: usize, to: usize| {
        let mut signals = item_signals.get();
        if from < signals.len() && to < signals.len() && from != to {
            let sig = signals.remove(from);
            signals.insert(to, sig);
            item_signals.set(signals);
            // The sync Effect will update value automatically
        }
    };

    let toggle_collapsed = move |index: usize| {
        let mut collapsed = collapsed_items.get();
        if collapsed.contains(&index) {
            collapsed.remove(&index);
        } else {
            collapsed.insert(index);
        }
        collapsed_items.set(collapsed);
    };

    let can_add = move || {
        if let Some(max) = max_items {
            item_signals.get().len() < max
        } else {
            true
        }
    };

    let can_remove = move || {
        if let Some(min) = min_items {
            item_signals.get().len() > min
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

            // Items list
            <div>
                {
                    let item_label_for_list = item_label.clone();
                    move || {
                        let signals = item_signals.get();
                        let item_count = signals.len();
                        let collapsed = collapsed_items.get();

                        signals.iter().enumerate().map(|(index, item_signal)| {
                            let item_title = format!("{} {}", item_label_for_list, index + 1);
                            let is_collapsed = collapsed.contains(&index);
                            let can_move_up = orderable && index > 0;
                            let can_move_down = orderable && index < item_count - 1;

                            // Use the stable signal from item_signals
                            let item_signal = *item_signal;

                        view! {
                            <div style=ITEM_CONTAINER_STYLE>
                                <div style=ITEM_HEADER_STYLE>
                                    <span style=ITEM_TITLE_STYLE>{item_title}</span>
                                    <div style=ITEM_ACTIONS_STYLE>
                                        {if can_move_up {
                                            view! {
                                                <button
                                                    type="button"
                                                    style=ACTION_BTN_STYLE
                                                    on:click=move |_| move_item(index, index - 1)
                                                    title="Move up"
                                                >
                                                    "↑"
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                        {if can_move_down {
                                            view! {
                                                <button
                                                    type="button"
                                                    style=ACTION_BTN_STYLE
                                                    on:click=move |_| move_item(index, index + 1)
                                                    title="Move down"
                                                >
                                                    "↓"
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                        {if collapsible {
                                            view! {
                                                <button
                                                    type="button"
                                                    style=ACTION_BTN_STYLE
                                                    on:click=move |_| toggle_collapsed(index)
                                                >
                                                    {if is_collapsed { "+" } else { "-" }}
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                        {if can_remove() {
                                            view! {
                                                <button
                                                    type="button"
                                                    style=ACTION_BTN_STYLE
                                                    on:click=move |_| remove_item(index)
                                                    title="Remove"
                                                >
                                                    "x"
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </div>
                                </div>

                                {if !is_collapsed {
                                    view! {
                                        <div style=ITEM_CONTENT_STYLE>
                                            // Render nested fields using the provided render function
                                            // or a placeholder if not provided
                                            {if let Some(ref render_fn) = render_fields {
                                                render_fn.run((index, fields.clone(), item_signal))
                                            } else {
                                                view! {
                                                    <p style="color: #64748b; font-size: 0.875rem;">
                                                        "Nested field rendering not configured"
                                                    </p>
                                                }.into_any()
                                            }}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }}
                            </div>
                        }
                        }).collect_view()
                    }
                }
            </div>

            // Add button
            <button
                type="button"
                style=ADD_BTN_STYLE
                on:click=add_item
                disabled=move || !can_add()
            >
                "+ Add " {item_label.clone()}
            </button>

            {move || {
                let count = item_signals.get().len();
                let constraints = vec![
                    min_items.map(|m| format!("min: {}", m)),
                    max_items.map(|m| format!("max: {}", m)),
                ].into_iter().flatten().collect::<Vec<_>>().join(", ");

                if let Some(ref desc) = description {
                    view! { <p style=HELP_TEXT_STYLE>{desc.clone()}</p> }.into_any()
                } else if !constraints.is_empty() {
                    view! { <p style=HELP_TEXT_STYLE>{format!("{} items ({})", count, constraints)}</p> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}
