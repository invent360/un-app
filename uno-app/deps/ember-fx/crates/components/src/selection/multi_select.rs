//! MultiSelect Leptos component.

use leptos::prelude::*;
use super::types::{SelectionSize, DropdownPlacement, SelectOption};
use crate::try_use_theme;

/// MultiSelect component.
///
/// A dropdown select for multiple value selection with tag display.
///
/// # Props
///
/// - `value` - Controlled selected values signal
/// - `options` - Available options
/// - `placeholder` - Placeholder text when no selection
/// - `size` - Size variant (Sm, Md, Lg)
/// - `disabled` - Whether the select is disabled
/// - `loading` - Show loading state
/// - `searchable` - Enable search/filter functionality
/// - `max_tag_count` - Maximum tags to show before collapsing
/// - `placement` - Dropdown placement (Top, Bottom, Auto)
/// - `on_change` - Change handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::selection::{MultiSelect, SelectOption};
///
/// let selected = RwSignal::new(vec!["opt1".to_string()]);
/// let options = vec![
///     SelectOption::new("opt1", "Option 1"),
///     SelectOption::new("opt2", "Option 2"),
///     SelectOption::new("opt3", "Option 3"),
/// ];
///
/// view! {
///     <MultiSelect
///         value=selected
///         options=options
///         placeholder="Select options"
///     />
/// }
/// ```
#[component]
pub fn MultiSelect(
    /// Controlled selected values.
    #[prop(optional, into)]
    value: Option<RwSignal<Vec<String>>>,
    /// Default selected values (uncontrolled).
    #[prop(optional, into)]
    default_value: Option<Vec<String>>,
    /// Available options.
    #[prop(into)]
    options: Vec<SelectOption<String>>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Label text.
    #[prop(optional, into)]
    label: Option<String>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<SelectionSize>,
    /// Whether the select is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Show loading state.
    #[prop(optional)]
    loading: bool,
    /// Enable search functionality.
    #[prop(optional)]
    searchable: bool,
    /// Maximum tags to show before collapsing.
    #[prop(optional)]
    max_tag_count: Option<usize>,
    /// Dropdown placement.
    #[prop(optional, into)]
    placement: Option<DropdownPlacement>,
    /// Select id attribute.
    #[prop(optional, into)]
    id: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<Vec<String>>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let placement = placement.unwrap_or_default();

    // Create internal state if not controlled
    let internal_value = value.unwrap_or_else(|| {
        RwSignal::new(default_value.unwrap_or_default())
    });

    // Dropdown open state
    let is_open = RwSignal::new(false);

    // Search query state
    let search_query = RwSignal::new(String::new());

    // Build CSS classes
    let select_prefix = format!("fx-multi-select-{}", design_system);

    let size_class = if size != SelectionSize::Md {
        size.class(&select_prefix)
    } else {
        String::new()
    };

    // Generate ID
    let select_id = id.unwrap_or_else(|| format!("multi-select-{}", rand_id()));

    // Class names
    let selector_class = format!("{}-selector", select_prefix);
    let tag_class = format!("{}-tag", select_prefix);
    let tag_close_class = format!("{}-tag-close", select_prefix);
    let placeholder_class = format!("{}-placeholder", select_prefix);
    let arrow_class = format!("{}-arrow", select_prefix);
    let dropdown_class = format!("{}-dropdown", select_prefix);
    let option_class = format!("{}-option", select_prefix);
    let search_class = format!("{}-search", select_prefix);
    let label_class = format!("{}-label", select_prefix);
    let overflow_class = format!("{}-overflow", select_prefix);

    // Store options for closures
    let options_for_tags = options.clone();
    let options_for_filter = options.clone();

    // Get selected items with labels
    let get_selected_items = move || {
        let values = internal_value.get();
        values.iter()
            .filter_map(|v| {
                options_for_tags.iter()
                    .find(|opt| &opt.value == v)
                    .map(|opt| (opt.value.clone(), opt.label.clone()))
            })
            .collect::<Vec<_>>()
    };

    // Filter options based on search
    let filtered_options = move || {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            options_for_filter.clone()
        } else {
            options_for_filter.iter()
                .filter(|opt| opt.label.to_lowercase().contains(&query))
                .cloned()
                .collect()
        }
    };

    // Handle option toggle
    let handle_toggle_option = move |opt_value: String| {
        internal_value.update(|values| {
            if let Some(pos) = values.iter().position(|v| v == &opt_value) {
                values.remove(pos);
            } else {
                values.push(opt_value);
            }
        });
        search_query.set(String::new());
        if let Some(ref cb) = on_change {
            cb.run(internal_value.get());
        }
    };

    // Handle remove tag
    let handle_remove_tag = move |tag_value: String, ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        internal_value.update(|values| {
            if let Some(pos) = values.iter().position(|v| v == &tag_value) {
                values.remove(pos);
            }
        });
        if let Some(ref cb) = on_change {
            cb.run(internal_value.get());
        }
    };

    // Handle toggle dropdown
    let handle_dropdown_toggle = move |_| {
        if disabled || loading {
            return;
        }
        is_open.update(|v| *v = !*v);
    };

    // Clone select_prefix for different uses
    let wrapper_class = format!("{}-wrapper", select_prefix);
    let select_prefix_for_class = select_prefix.clone();

    view! {
        <div class=wrapper_class>
            {label.map(|l| view! {
                <label class=label_class.clone() for=select_id.clone()>{l}</label>
            })}
            <div
                class=move || {
                    let mut cls = vec![select_prefix_for_class.clone()];
                    if !size_class.is_empty() {
                        cls.push(size_class.clone());
                    }
                    if is_open.get() {
                        cls.push(format!("{}-open", select_prefix_for_class));
                    }
                    if disabled {
                        cls.push(format!("{}-disabled", select_prefix_for_class));
                    }
                    if loading {
                        cls.push(format!("{}-loading", select_prefix_for_class));
                    }
                    if !internal_value.get().is_empty() {
                        cls.push(format!("{}-has-value", select_prefix_for_class));
                    }
                    if let Some(ref custom) = class {
                        cls.push(custom.clone());
                    }
                    cls.join(" ")
                }
                id=select_id
            >
                // Selector with tags
                <div
                    class=selector_class.clone()
                    on:click=handle_dropdown_toggle
                >
                    {move || {
                        let items = get_selected_items();
                        if items.is_empty() {
                            view! {
                                <span class=placeholder_class.clone()>
                                    {placeholder.clone().unwrap_or_default()}
                                </span>
                            }.into_any()
                        } else {
                            let max_count = max_tag_count.unwrap_or(items.len());
                            let visible_items: Vec<_> = items.iter().take(max_count).cloned().collect();
                            let overflow_count = items.len().saturating_sub(max_count);

                            view! {
                                <div class="fx-multi-select-tags">
                                    {visible_items.into_iter().map(|(val, label)| {
                                        let tag_val = val.clone();
                                        view! {
                                            <span class=tag_class.clone()>
                                                {label}
                                                <span
                                                    class=tag_close_class.clone()
                                                    on:click=move |ev| handle_remove_tag(tag_val.clone(), ev)
                                                >
                                                    "×"
                                                </span>
                                            </span>
                                        }
                                    }).collect_view()}
                                    {if overflow_count > 0 {
                                        Some(view! {
                                            <span class=overflow_class.clone()>
                                                {format!("+{}", overflow_count)}
                                            </span>
                                        })
                                    } else {
                                        None
                                    }}
                                </div>
                            }.into_any()
                        }
                    }}

                    // Arrow
                    <span class=arrow_class.clone()>
                        {move || if is_open.get() { "▲" } else { "▼" }}
                    </span>
                </div>

                // Dropdown
                <div
                    class=move || {
                        let mut cls = vec![dropdown_class.clone()];
                        cls.push(format!("{}-placement-{}", dropdown_class, placement.as_suffix()));
                        if is_open.get() {
                            cls.push(format!("{}-open", dropdown_class));
                        }
                        cls.join(" ")
                    }
                >
                    // Search input
                    {if searchable {
                        Some(view! {
                            <div class=search_class.clone()>
                                <input
                                    type="text"
                                    placeholder="Search..."
                                    prop:value=move || search_query.get()
                                    on:input=move |ev| {
                                        search_query.set(event_target_value(&ev));
                                    }
                                />
                            </div>
                        })
                    } else {
                        None
                    }}

                    // Options with checkboxes
                    <div class=format!("{}-options", select_prefix)>
                        {
                            let option_class = option_class.clone();
                            move || {
                                let option_class = option_class.clone();
                                filtered_options().into_iter().map({
                                    let option_class = option_class.clone();
                                    move |opt| {
                                        let opt_value = opt.value.clone();
                                        let opt_label = opt.label.clone();
                                        let is_disabled = opt.disabled;
                                        let opt_value_for_toggle = opt_value.clone();
                                        let opt_value_for_class = opt_value.clone();
                                        let opt_value_for_checkbox = opt_value.clone();
                                        let option_class_for_vec = option_class.clone();
                                        let option_class_for_selected = option_class.clone();
                                        let option_class_for_disabled = option_class.clone();

                                        view! {
                                            <div
                                                class=move || {
                                                    let mut cls = vec![option_class_for_vec.clone()];
                                                    if internal_value.get().contains(&opt_value_for_class) {
                                                        cls.push(format!("{}-selected", option_class_for_selected));
                                                    }
                                                    if is_disabled {
                                                        cls.push(format!("{}-disabled", option_class_for_disabled));
                                                    }
                                                    cls.join(" ")
                                                }
                                                on:click=move |_| {
                                                    if !is_disabled {
                                                        handle_toggle_option(opt_value_for_toggle.clone());
                                                    }
                                                }
                                            >
                                                <span class="fx-multi-select-checkbox">
                                                    {move || {
                                                        if internal_value.get().contains(&opt_value_for_checkbox) {
                                                            "☑"
                                                        } else {
                                                            "☐"
                                                        }
                                                    }}
                                                </span>
                                                {opt_label}
                                            </div>
                                        }
                                    }
                                }).collect_view()
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Generate incremental ID.
fn rand_id() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(7000);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:x}", id)
}
