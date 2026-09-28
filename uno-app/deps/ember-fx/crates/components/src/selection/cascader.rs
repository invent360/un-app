//! Cascader Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{SelectionSize, CascaderOption, CascaderTrigger, DropdownPlacement};
use crate::try_use_theme;

/// Cascader component.
///
/// A multi-level cascading dropdown for hierarchical selection.
/// Each level shows in a separate column, similar to macOS Finder.
///
/// # Props
///
/// - `value` - Selected path (vector of values from root to leaf)
/// - `options` - Hierarchical options data
/// - `placeholder` - Placeholder text
/// - `size` - Component size (Sm, Md, Lg)
/// - `trigger` - How to expand next level (Click, Hover)
/// - `disabled` - Whether the cascader is disabled
/// - `allow_clear` - Show clear button
/// - `show_search` - Enable search filtering
/// - `change_on_select` - Fire onChange on each level selection (not just leaf)
/// - `placement` - Dropdown placement
/// - `expand_icon` - Custom expand icon
/// - `class` - Additional CSS classes
/// - `on_change` - Selection change handler (receives full path)
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{Cascader, CascaderOption};
///
/// let selected = RwSignal::new(Vec::<String>::new());
///
/// let options = vec![
///     CascaderOption::branch("usa", "United States", vec![
///         CascaderOption::branch("ca", "California", vec![
///             CascaderOption::leaf("sf", "San Francisco"),
///             CascaderOption::leaf("la", "Los Angeles"),
///         ]),
///         CascaderOption::leaf("ny", "New York"),
///     ]),
///     CascaderOption::leaf("uk", "United Kingdom"),
/// ];
///
/// view! {
///     <Cascader
///         value=selected
///         options=options
///         placeholder="Select location"
///         on_change=move |path| log::info!("Selected: {:?}", path)
///     />
/// }
/// ```
#[component]
pub fn Cascader<T>(
    /// Selected path (values from root to selection).
    #[prop(into)]
    value: RwSignal<Vec<T>>,
    /// Hierarchical options.
    #[prop(into)]
    options: Vec<CascaderOption<T>>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Component size.
    #[prop(optional, into)]
    size: Option<SelectionSize>,
    /// Trigger mode for expansion.
    #[prop(optional, into)]
    trigger: Option<CascaderTrigger>,
    /// Whether disabled.
    #[prop(optional)]
    disabled: bool,
    /// Show clear button.
    #[prop(optional)]
    allow_clear: bool,
    /// Enable search.
    #[prop(optional)]
    show_search: bool,
    /// Fire onChange on each level.
    #[prop(optional)]
    change_on_select: bool,
    /// Dropdown placement.
    #[prop(optional, into)]
    placement: Option<DropdownPlacement>,
    /// Custom expand icon.
    #[prop(optional, into)]
    expand_icon: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<Vec<T>>>,
) -> impl IntoView
where
    T: Clone + PartialEq + std::fmt::Debug + Send + Sync + 'static,
{
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let trigger = trigger.unwrap_or_default();
    let placement = placement.unwrap_or_default();
    let expand_icon = expand_icon.unwrap_or_else(|| "▶".to_string());

    // Internal state
    let is_open = RwSignal::new(false);
    let search_text = RwSignal::new(String::new());
    let active_path = RwSignal::new(Vec::<T>::new()); // Path of hovered/selected items

    // Store options
    let options_stored = StoredValue::new(options.clone());

    // Build CSS classes
    let cascader_prefix = format!("fx-cascader-{}", design_system);

    let wrapper_class = {
        let prefix = cascader_prefix.clone();
        move || {
            let mut parts = vec![
                prefix.clone(),
                size.class(&prefix),
                trigger.class(&prefix),
            ];

            if disabled {
                parts.push(format!("{}-disabled", prefix));
            }
            if is_open.get() {
                parts.push(format!("{}-open", prefix));
            }

            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }

            parts.join(" ")
        }
    };

    // Get display label from selected path
    let get_display_label = move || -> String {
        let path = value.get();
        if path.is_empty() {
            return String::new();
        }

        let opts = options_stored.get_value();
        let labels = collect_labels_for_path(&opts, &path);
        labels.join(" / ")
    };

    // Handle dropdown toggle
    let handle_toggle = move |_: ev::MouseEvent| {
        if !disabled {
            is_open.update(|open| *open = !*open);
            if is_open.get() {
                // Reset active path to current selection
                active_path.set(value.get());
            }
        }
    };

    // Handle option click/hover
    let on_change_clone = on_change.clone();
    let handle_select = move |level: usize, opt_value: T, is_leaf: bool| {
        active_path.update(|path| {
            // Truncate path to this level and add new value
            path.truncate(level);
            path.push(opt_value.clone());
        });

        if is_leaf || change_on_select {
            let new_path = active_path.get();
            value.set(new_path.clone());

            if let Some(ref cb) = on_change_clone {
                cb.run(new_path);
            }

            if is_leaf {
                is_open.set(false);
            }
        }
    };

    // Handle clear
    let on_change_clear = on_change.clone();
    let handle_clear = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        value.set(Vec::new());
        active_path.set(Vec::new());
        if let Some(ref cb) = on_change_clear {
            cb.run(Vec::new());
        }
    };

    // Handle search
    let handle_search = move |ev: ev::Event| {
        let input_value = event_target_value(&ev);
        search_text.set(input_value);
    };

    // Get options for each level
    let get_level_options = move |level: usize| -> Vec<CascaderOption<T>> {
        let opts = options_stored.get_value();
        let path = active_path.get();

        if level == 0 {
            return opts;
        }

        // Navigate to the correct level
        let mut current_opts = &opts;
        for (i, val) in path.iter().enumerate() {
            if i >= level {
                break;
            }
            if let Some(opt) = current_opts.iter().find(|o| &o.value == val) {
                current_opts = &opt.children;
            } else {
                return Vec::new();
            }
        }

        current_opts.clone()
    };

    // Determine how many levels to show
    let visible_levels = move || -> usize {
        let path = active_path.get();
        let opts = options_stored.get_value();

        let mut count = 1; // Always show first level
        let mut current_opts = &opts;

        for val in path.iter() {
            if let Some(opt) = current_opts.iter().find(|o| &o.value == val) {
                if !opt.children.is_empty() {
                    count += 1;
                    current_opts = &opt.children;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        count
    };

    // CSS class names
    let selector_class = format!("{}-selector", cascader_prefix);
    let selection_class = format!("{}-selection", cascader_prefix);
    let placeholder_class = format!("{}-placeholder", cascader_prefix);
    let clear_class = format!("{}-clear", cascader_prefix);
    let arrow_class = format!("{}-arrow", cascader_prefix);
    let dropdown_class = format!("{}-dropdown", cascader_prefix);
    let search_class = format!("{}-search", cascader_prefix);
    let menus_class = format!("{}-menus", cascader_prefix);
    let menu_class = format!("{}-menu", cascader_prefix);
    let option_class = format!("{}-option", cascader_prefix);

    view! {
        <div class=wrapper_class tabindex=0>
            // Selector
            <div class=selector_class on:click=handle_toggle>
                <div class=selection_class>
                    {move || {
                        let label = get_display_label();
                        if label.is_empty() {
                            view! { <span class=placeholder_class.clone()>{placeholder.clone().unwrap_or_default()}</span> }.into_any()
                        } else {
                            view! { <span>{label}</span> }.into_any()
                        }
                    }}
                </div>

                // Clear button
                {(allow_clear && !disabled).then(|| view! {
                    <button
                        type="button"
                        class=clear_class.clone()
                        class:hidden=move || value.get().is_empty()
                        tabindex=-1
                        on:click=handle_clear.clone()
                    >
                        "✕"
                    </button>
                })}

                // Arrow
                <span class=arrow_class>
                    {move || if is_open.get() { "▲" } else { "▼" }}
                </span>
            </div>

            // Dropdown
            <div
                class=dropdown_class
                class:hidden=move || !is_open.get()
            >
                // Search
                {show_search.then(|| view! {
                    <div class=search_class.clone()>
                        <input
                            type="text"
                            placeholder="Search..."
                            value=move || search_text.get()
                            on:input=handle_search.clone()
                        />
                    </div>
                })}

                // Cascading menus
                <div class=menus_class>
                    {move || {
                        let levels = visible_levels();
                        (0..levels).map(|level| {
                            let level_options = get_level_options(level);
                            let active = active_path.get();
                            let selected_at_level = active.get(level).cloned();

                            let menu_item_class = menu_class.clone();
                            let opt_item_class = option_class.clone();
                            let expand = expand_icon.clone();
                            let handle_sel = handle_select.clone();
                            let trig = trigger;

                            view! {
                                <ul class=menu_item_class role="listbox">
                                    {level_options.into_iter().map(|opt| {
                                        let opt_value = opt.value.clone();
                                        let opt_value_click = opt.value.clone();
                                        let opt_value_hover = opt.value.clone();
                                        let is_selected = selected_at_level.as_ref() == Some(&opt_value);
                                        let is_leaf = opt.is_leaf || opt.children.is_empty();
                                        let is_disabled = opt.disabled;
                                        let has_children = !opt.children.is_empty();

                                        let opt_class = {
                                            let mut classes = vec![opt_item_class.clone()];
                                            if is_selected {
                                                classes.push(format!("{}-selected", opt_item_class));
                                            }
                                            if is_disabled {
                                                classes.push(format!("{}-disabled", opt_item_class));
                                            }
                                            classes.join(" ")
                                        };

                                        let handle_click = {
                                            let handler = handle_sel.clone();
                                            move |_: ev::MouseEvent| {
                                                if !is_disabled {
                                                    handler(level, opt_value_click.clone(), is_leaf);
                                                }
                                            }
                                        };

                                        let handle_hover = {
                                            let handler = handle_sel.clone();
                                            move |_: ev::MouseEvent| {
                                                if trig == CascaderTrigger::Hover && !is_disabled && has_children {
                                                    handler(level, opt_value_hover.clone(), false);
                                                }
                                            }
                                        };

                                        view! {
                                            <li
                                                class=opt_class
                                                role="option"
                                                aria-selected=is_selected.to_string()
                                                on:click=handle_click
                                                on:mouseenter=handle_hover
                                            >
                                                <span class=format!("{}-label", opt_item_class.clone())>
                                                    {opt.label.clone()}
                                                </span>
                                                {has_children.then(|| view! {
                                                    <span class=format!("{}-expand-icon", opt_item_class.clone())>
                                                        {expand.clone()}
                                                    </span>
                                                })}
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </div>
    }
}

/// Collect labels for a path of values.
fn collect_labels_for_path<T: Clone + PartialEq>(
    options: &[CascaderOption<T>],
    path: &[T],
) -> Vec<String> {
    let mut labels = Vec::new();
    let mut current_opts = options;

    for val in path {
        if let Some(opt) = current_opts.iter().find(|o| &o.value == val) {
            labels.push(opt.label.clone());
            current_opts = &opt.children;
        } else {
            break;
        }
    }

    labels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_labels_for_path() {
        let options = vec![
            CascaderOption::branch("a".to_string(), "Option A", vec![
                CascaderOption::leaf("a1".to_string(), "Option A1"),
            ]),
        ];

        let path = vec!["a".to_string(), "a1".to_string()];
        let labels = collect_labels_for_path(&options, &path);

        assert_eq!(labels, vec!["Option A", "Option A1"]);
    }
}
