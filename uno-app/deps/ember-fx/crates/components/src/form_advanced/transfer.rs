//! Transfer Leptos component.

use leptos::prelude::*;
use super::types::TransferItem;
use crate::try_use_theme;

/// Transfer component.
///
/// Double column transfer component for moving items between lists.
///
/// # Props
///
/// - `data_source` - All available items
/// - `target_keys` - Keys of items in the right column
/// - `titles` - Titles for left and right columns
/// - `disabled` - Disabled state
/// - `show_search` - Show search input
/// - `show_select_all` - Show select all checkbox
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::form_advanced::{Transfer, TransferItem};
///
/// let items = vec![
///     TransferItem::new("1", "Item 1"),
///     TransferItem::new("2", "Item 2"),
///     TransferItem::new("3", "Item 3"),
/// ];
/// let target_keys = RwSignal::new(vec!["2".to_string()]);
///
/// view! {
///     <Transfer data_source=items target_keys=target_keys />
/// }
/// ```
#[component]
pub fn Transfer(
    /// All available items.
    #[prop(into)]
    data_source: Vec<TransferItem>,
    /// Keys of items in the right column.
    #[prop(into)]
    target_keys: RwSignal<Vec<String>>,
    /// Titles for columns (left, right).
    #[prop(optional, into)]
    titles: Option<(String, String)>,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Show search input.
    #[prop(optional)]
    show_search: bool,
    /// Show select all checkbox.
    #[prop(optional)]
    show_select_all: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<Vec<String>>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let titles = titles.unwrap_or_else(|| ("Source".to_string(), "Target".to_string()));
    let show_select_all = show_select_all.unwrap_or(true);

    // Build CSS classes
    let transfer_prefix = format!("fx-transfer-{}", design_system);

    let transfer_prefix_for_class = transfer_prefix.clone();
    let transfer_prefix_for_left = transfer_prefix.clone();
    let transfer_prefix_for_right = transfer_prefix.clone();
    let transfer_prefix_for_ops = transfer_prefix.clone();

    // State
    let selected_left = RwSignal::new(Vec::<String>::new());
    let selected_right = RwSignal::new(Vec::<String>::new());
    let search_left = RwSignal::new(String::new());
    let search_right = RwSignal::new(String::new());

    // Store data source in signal for reactive access
    let data_source_signal = RwSignal::new(data_source.clone());

    let combined_class = {
        let class = class.clone();
        move || {
            let mut parts = vec![transfer_prefix_for_class.clone()];
            if disabled {
                parts.push(format!("{}-disabled", transfer_prefix_for_class));
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Get left items (not in target) - as a derived function
    let get_left_items = move || {
        let data_source = data_source_signal.get();
        let targets = target_keys.get();
        let search = search_left.get().to_lowercase();
        data_source
            .iter()
            .filter(|item| {
                !targets.contains(&item.key)
                    && (search.is_empty() || item.title.to_lowercase().contains(&search))
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    // Get right items (in target) - as a derived function
    let get_right_items = move || {
        let data_source = data_source_signal.get();
        let targets = target_keys.get();
        let search = search_right.get().to_lowercase();
        data_source
            .iter()
            .filter(|item| {
                targets.contains(&item.key)
                    && (search.is_empty() || item.title.to_lowercase().contains(&search))
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    // Move to right
    let move_to_right = move |_| {
        if disabled {
            return;
        }
        let selected = selected_left.get_untracked();
        if selected.is_empty() {
            return;
        }

        let mut targets = target_keys.get_untracked();
        for key in selected.iter() {
            if !targets.contains(key) {
                targets.push(key.clone());
            }
        }
        target_keys.set(targets.clone());
        selected_left.set(Vec::new());

        if let Some(ref cb) = on_change {
            cb.run(targets);
        }
    };

    // Move to left
    let move_to_left = move |_| {
        if disabled {
            return;
        }
        let selected = selected_right.get_untracked();
        if selected.is_empty() {
            return;
        }

        let mut targets = target_keys.get_untracked();
        targets.retain(|k| !selected.contains(k));
        target_keys.set(targets.clone());
        selected_right.set(Vec::new());

        if let Some(ref cb) = on_change {
            cb.run(targets);
        }
    };

    view! {
        <div class=combined_class>
            // Left panel
            <div class=format!("{}-list", transfer_prefix_for_left)>
                <div class=format!("{}-list-header", transfer_prefix_for_left)>
                    {if show_select_all {
                        let items = get_left_items.clone();
                        Some(view! {
                            <input
                                type="checkbox"
                                class=format!("{}-checkbox-all", transfer_prefix_for_left)
                                checked=move || {
                                    let items = items();
                                    let selected = selected_left.get();
                                    !items.is_empty() && items.iter().all(|i| selected.contains(&i.key))
                                }
                                on:change=move |_| {
                                    let items = get_left_items();
                                    let all_keys: Vec<String> = items.iter().map(|i| i.key.clone()).collect();
                                    let selected = selected_left.get();
                                    if items.iter().all(|i| selected.contains(&i.key)) {
                                        selected_left.set(Vec::new());
                                    } else {
                                        selected_left.set(all_keys);
                                    }
                                }
                            />
                        })
                    } else {
                        None
                    }}
                    <span class=format!("{}-list-title", transfer_prefix_for_left)>
                        {titles.0.clone()}
                    </span>
                    <span class=format!("{}-list-count", transfer_prefix_for_left)>
                        {move || {
                            let selected = selected_left.get().len();
                            let total = get_left_items().len();
                            if selected > 0 {
                                format!("{}/{}", selected, total)
                            } else {
                                format!("{} items", total)
                            }
                        }}
                    </span>
                </div>

                {if show_search {
                    let transfer_prefix = transfer_prefix_for_left.clone();
                    Some(view! {
                        <div class=format!("{}-list-search", transfer_prefix)>
                            <input
                                type="text"
                                placeholder="Search"
                                class=format!("{}-search-input", transfer_prefix)
                                on:input=move |ev| {
                                    search_left.set(event_target_value(&ev));
                                }
                            />
                        </div>
                    })
                } else {
                    None
                }}

                <ul class=format!("{}-list-content", transfer_prefix_for_left)>
                    {
                        let transfer_prefix_left_list = transfer_prefix_for_left.clone();
                        move || {
                            get_left_items().into_iter().map(|item| {
                                let key = item.key.clone();
                                let key_for_class = key.clone();
                                let key_for_check = key.clone();
                                let key_for_checkbox = key.clone();
                                let transfer_prefix = transfer_prefix_left_list.clone();
                                let transfer_prefix_for_text = transfer_prefix.clone();
                                view! {
                                    <li
                                        class=move || {
                                            let mut cls = vec![format!("{}-list-item", transfer_prefix)];
                                            if item.disabled {
                                                cls.push(format!("{}-list-item-disabled", transfer_prefix));
                                            }
                                            if selected_left.get().contains(&key_for_class) {
                                                cls.push(format!("{}-list-item-selected", transfer_prefix));
                                            }
                                            cls.join(" ")
                                        }
                                        on:click=move |_| {
                                            if item.disabled || disabled {
                                                return;
                                            }
                                            let mut selected = selected_left.get();
                                            if selected.contains(&key_for_check) {
                                                selected.retain(|k| k != &key_for_check);
                                            } else {
                                                selected.push(key_for_check.clone());
                                            }
                                            selected_left.set(selected);
                                        }
                                    >
                                        <input
                                            type="checkbox"
                                            checked=move || selected_left.get().contains(&key_for_checkbox)
                                            disabled=item.disabled || disabled
                                        />
                                        <span class=format!("{}-list-item-text", transfer_prefix_for_text)>
                                            {item.title.clone()}
                                        </span>
                                    </li>
                                }
                            }).collect_view()
                        }
                    }
                </ul>
            </div>

            // Operations
            <div class=format!("{}-operation", transfer_prefix_for_ops)>
                <button
                    class=format!("{}-operation-btn", transfer_prefix_for_ops)
                    disabled=move || disabled || selected_left.get().is_empty()
                    on:click=move_to_right
                >
                    ">"
                </button>
                <button
                    class=format!("{}-operation-btn", transfer_prefix_for_ops)
                    disabled=move || disabled || selected_right.get().is_empty()
                    on:click=move_to_left
                >
                    "<"
                </button>
            </div>

            // Right panel
            <div class=format!("{}-list", transfer_prefix_for_right)>
                <div class=format!("{}-list-header", transfer_prefix_for_right)>
                    {if show_select_all {
                        let items = get_right_items.clone();
                        Some(view! {
                            <input
                                type="checkbox"
                                class=format!("{}-checkbox-all", transfer_prefix_for_right)
                                checked=move || {
                                    let items = items();
                                    let selected = selected_right.get();
                                    !items.is_empty() && items.iter().all(|i| selected.contains(&i.key))
                                }
                                on:change=move |_| {
                                    let items = get_right_items();
                                    let all_keys: Vec<String> = items.iter().map(|i| i.key.clone()).collect();
                                    let selected = selected_right.get();
                                    if items.iter().all(|i| selected.contains(&i.key)) {
                                        selected_right.set(Vec::new());
                                    } else {
                                        selected_right.set(all_keys);
                                    }
                                }
                            />
                        })
                    } else {
                        None
                    }}
                    <span class=format!("{}-list-title", transfer_prefix_for_right)>
                        {titles.1.clone()}
                    </span>
                    <span class=format!("{}-list-count", transfer_prefix_for_right)>
                        {move || {
                            let selected = selected_right.get().len();
                            let total = get_right_items().len();
                            if selected > 0 {
                                format!("{}/{}", selected, total)
                            } else {
                                format!("{} items", total)
                            }
                        }}
                    </span>
                </div>

                {if show_search {
                    let transfer_prefix = transfer_prefix_for_right.clone();
                    Some(view! {
                        <div class=format!("{}-list-search", transfer_prefix)>
                            <input
                                type="text"
                                placeholder="Search"
                                class=format!("{}-search-input", transfer_prefix)
                                on:input=move |ev| {
                                    search_right.set(event_target_value(&ev));
                                }
                            />
                        </div>
                    })
                } else {
                    None
                }}

                <ul class=format!("{}-list-content", transfer_prefix_for_right)>
                    {
                        let transfer_prefix_right_list = transfer_prefix_for_right.clone();
                        move || {
                            get_right_items().into_iter().map(|item| {
                                let key = item.key.clone();
                                let key_for_class = key.clone();
                                let key_for_check = key.clone();
                                let key_for_checkbox = key.clone();
                                let transfer_prefix = transfer_prefix_right_list.clone();
                                let transfer_prefix_for_text = transfer_prefix.clone();
                                view! {
                                    <li
                                        class=move || {
                                            let mut cls = vec![format!("{}-list-item", transfer_prefix)];
                                            if item.disabled {
                                                cls.push(format!("{}-list-item-disabled", transfer_prefix));
                                            }
                                            if selected_right.get().contains(&key_for_class) {
                                                cls.push(format!("{}-list-item-selected", transfer_prefix));
                                            }
                                            cls.join(" ")
                                        }
                                        on:click=move |_| {
                                            if item.disabled || disabled {
                                                return;
                                            }
                                            let mut selected = selected_right.get();
                                            if selected.contains(&key_for_check) {
                                                selected.retain(|k| k != &key_for_check);
                                            } else {
                                                selected.push(key_for_check.clone());
                                            }
                                            selected_right.set(selected);
                                        }
                                    >
                                        <input
                                            type="checkbox"
                                            checked=move || selected_right.get().contains(&key_for_checkbox)
                                            disabled=item.disabled || disabled
                                        />
                                        <span class=format!("{}-list-item-text", transfer_prefix_for_text)>
                                            {item.title.clone()}
                                        </span>
                                    </li>
                                }
                            }).collect_view()
                        }
                    }
                </ul>
            </div>
        </div>
    }
}
