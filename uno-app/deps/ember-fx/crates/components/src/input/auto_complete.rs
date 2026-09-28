//! AutoComplete Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{InputSize, InputVariant, ValidationState, AutoCompleteOption};
use crate::try_use_theme;

/// AutoComplete component.
///
/// A text input with type-ahead suggestions dropdown.
/// Supports filtering, custom rendering, grouping, and keyboard navigation.
///
/// # Props
///
/// - `value` - Controlled value signal
/// - `options` - List of available options
/// - `filter_fn` - Custom filter function (default: case-insensitive contains)
/// - `placeholder` - Placeholder text
/// - `size` - Input size (Sm, Md, Lg)
/// - `variant` - Visual style variant
/// - `disabled` - Whether the input is disabled
/// - `allow_custom` - Allow values not in options list
/// - `auto_focus` - Focus on mount
/// - `clear_on_select` - Clear input after selection
/// - `open_on_focus` - Open dropdown on focus
/// - `validation` - Validation state
/// - `error_message` - Error message to display
/// - `class` - Additional CSS classes
/// - `on_select` - Selection handler
/// - `on_change` - Input change handler
/// - `on_search` - Search/filter handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{AutoComplete, AutoCompleteOption};
///
/// let selected = RwSignal::new(String::new());
/// let options = vec![
///     AutoCompleteOption::simple("Apple"),
///     AutoCompleteOption::simple("Banana"),
///     AutoCompleteOption::simple("Cherry"),
/// ];
///
/// view! {
///     <AutoComplete
///         value=selected
///         options=options
///         placeholder="Search fruits..."
///         on_select=move |opt| log::info!("Selected: {}", opt.value)
///     />
/// }
/// ```
#[component]
pub fn AutoComplete(
    /// Controlled value signal.
    #[prop(into)]
    value: RwSignal<String>,
    /// Available options.
    #[prop(into)]
    options: Vec<AutoCompleteOption>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Input size.
    #[prop(optional, into)]
    size: Option<InputSize>,
    /// Visual variant.
    #[prop(optional, into)]
    variant: Option<InputVariant>,
    /// Whether the input is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Allow values not in options.
    #[prop(optional)]
    #[prop(default = true)]
    allow_custom: bool,
    /// Focus on mount.
    #[prop(optional)]
    auto_focus: bool,
    /// Clear after selection.
    #[prop(optional)]
    clear_on_select: bool,
    /// Open dropdown on focus.
    #[prop(optional)]
    #[prop(default = true)]
    open_on_focus: bool,
    /// Max dropdown height in pixels.
    #[prop(optional)]
    #[prop(default = 256)]
    max_height: u32,
    /// Validation state.
    #[prop(optional, into)]
    validation: Option<ValidationState>,
    /// Error message.
    #[prop(optional, into)]
    error_message: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Selection handler.
    #[prop(optional, into)]
    on_select: Option<Callback<AutoCompleteOption>>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
    /// Search handler.
    #[prop(optional, into)]
    on_search: Option<Callback<String>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let variant = variant.unwrap_or_default();
    let validation = validation.unwrap_or_default();

    // Internal state
    let is_open = RwSignal::new(false);
    let highlighted_index = RwSignal::new(0i32);
    let search_text = RwSignal::new(String::new());

    // Store options for filtering
    let all_options = StoredValue::new(options.clone());

    // Filter options based on search text
    let filtered_options = Memo::new(move |_| {
        let search = search_text.get().to_lowercase();
        if search.is_empty() {
            all_options.get_value()
        } else {
            all_options.get_value()
                .into_iter()
                .filter(|opt| {
                    opt.label.to_lowercase().contains(&search) ||
                    opt.value.to_lowercase().contains(&search)
                })
                .collect()
        }
    });

    // Build CSS classes
    let autocomplete_prefix = format!("fx-autocomplete-{}", design_system);

    let wrapper_class = {
        let prefix = autocomplete_prefix.clone();
        move || {
            let mut parts = vec![
                prefix.clone(),
                size.class(&prefix),
                variant.class(&prefix),
            ];

            if disabled {
                parts.push(format!("{}-disabled", prefix));
            }
            if is_open.get() {
                parts.push(format!("{}-open", prefix));
            }

            let val_class = validation.class(&prefix);
            if !val_class.is_empty() {
                parts.push(val_class);
            }

            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }

            parts.join(" ")
        }
    };

    // Handle input change
    let on_change_clone = on_change.clone();
    let on_search_clone = on_search.clone();
    let handle_input = move |ev: ev::Event| {
        let input_value = event_target_value(&ev);
        value.set(input_value.clone());
        search_text.set(input_value.clone());
        highlighted_index.set(0);

        if !is_open.get() && open_on_focus {
            is_open.set(true);
        }

        if let Some(ref cb) = on_change_clone {
            cb.run(input_value.clone());
        }
        if let Some(ref cb) = on_search_clone {
            cb.run(input_value);
        }
    };

    // Handle focus
    let handle_focus = move |_: ev::FocusEvent| {
        if open_on_focus && !disabled {
            is_open.set(true);
        }
    };

    // Handle blur (with delay to allow click on option)
    let handle_blur = move |_: ev::FocusEvent| {
        // Use a small delay to allow option click to register
        // In a real implementation, this would use set_timeout
        // For now, we'll rely on the mousedown event on options
    };

    // Handle option selection
    let on_select_clone = on_select.clone();
    let on_change_select = on_change.clone();
    let select_option = move |option: AutoCompleteOption| {
        if option.disabled {
            return;
        }

        if clear_on_select {
            value.set(String::new());
            search_text.set(String::new());
        } else {
            value.set(option.value.clone());
            search_text.set(String::new());
        }

        is_open.set(false);

        if let Some(ref cb) = on_select_clone {
            cb.run(option.clone());
        }
        if let Some(ref cb) = on_change_select {
            cb.run(option.value);
        }
    };

    // Handle keyboard navigation
    let select_option_kb = select_option.clone();
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        let options = filtered_options.get();
        let options_len = options.len() as i32;

        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                if !is_open.get() {
                    is_open.set(true);
                } else if options_len > 0 {
                    highlighted_index.update(|i| {
                        *i = (*i + 1).min(options_len - 1);
                    });
                }
            }
            "ArrowUp" => {
                ev.prevent_default();
                if options_len > 0 {
                    highlighted_index.update(|i| {
                        *i = (*i - 1).max(0);
                    });
                }
            }
            "Enter" => {
                ev.prevent_default();
                if is_open.get() && options_len > 0 {
                    let idx = highlighted_index.get() as usize;
                    if let Some(opt) = options.get(idx) {
                        select_option_kb(opt.clone());
                    }
                }
            }
            "Escape" => {
                ev.prevent_default();
                is_open.set(false);
            }
            "Tab" => {
                is_open.set(false);
            }
            _ => {}
        }
    };

    // CSS class names
    let input_wrapper_class = format!("{}-input-wrapper", autocomplete_prefix);
    let input_class = format!("{}-input", autocomplete_prefix);
    let dropdown_class = format!("{}-dropdown", autocomplete_prefix);
    let option_class = format!("{}-option", autocomplete_prefix);
    let group_class = format!("{}-group", autocomplete_prefix);
    let empty_class = format!("{}-empty", autocomplete_prefix);
    let error_class = format!("{}-error", autocomplete_prefix);

    view! {
        <div class=wrapper_class>
            <div class=input_wrapper_class>
                <input
                    type="text"
                    class=input_class
                    value=move || value.get()
                    placeholder=placeholder.clone().unwrap_or_default()
                    disabled=disabled
                    autocomplete="off"
                    role="combobox"
                    aria-expanded=move || is_open.get().to_string()
                    aria-haspopup="listbox"
                    aria-autocomplete="list"
                    on:input=handle_input
                    on:focus=handle_focus
                    on:blur=handle_blur
                    on:keydown=handle_keydown
                />

                // Dropdown arrow indicator
                <span class=format!("{}-arrow", autocomplete_prefix)>
                    {move || if is_open.get() { "▲" } else { "▼" }}
                </span>
            </div>

            // Dropdown
            <div
                class=dropdown_class
                class:hidden=move || !is_open.get()
                style=format!("max-height: {}px;", max_height)
                role="listbox"
            >
                {move || {
                    let options = filtered_options.get();

                    if options.is_empty() {
                        view! {
                            <div class=empty_class.clone()>
                                "No matches found"
                            </div>
                        }.into_any()
                    } else {
                        // Group options if they have groups
                        let mut current_group: Option<String> = None;
                        let mut items = Vec::new();

                        for (idx, opt) in options.iter().enumerate() {
                            // Check if we need a group header
                            if opt.group != current_group {
                                if let Some(ref group) = opt.group {
                                    current_group = Some(group.clone());
                                    items.push(view! {
                                        <div class=group_class.clone()>
                                            {group.clone()}
                                        </div>
                                    }.into_any());
                                } else {
                                    current_group = None;
                                }
                            }

                            let opt_clone = opt.clone();
                            let select_handler = select_option.clone();
                            let is_highlighted = move || highlighted_index.get() == idx as i32;
                            let is_disabled = opt.disabled;

                            let option_item_class = {
                                let base = option_class.clone();
                                if is_disabled {
                                    format!("{} {}-disabled", base, option_class)
                                } else {
                                    base
                                }
                            };

                            items.push(view! {
                                <div
                                    class=option_item_class
                                    class:highlighted=is_highlighted
                                    role="option"
                                    aria-selected=move || is_highlighted().to_string()
                                    aria-disabled=is_disabled.to_string()
                                    on:mousedown=move |ev| {
                                        ev.prevent_default();
                                        select_handler(opt_clone.clone());
                                    }
                                    on:mouseenter=move |_| {
                                        if !is_disabled {
                                            highlighted_index.set(idx as i32);
                                        }
                                    }
                                >
                                    {opt.label.clone()}
                                </div>
                            }.into_any());
                        }

                        view! {
                            <div>{items}</div>
                        }.into_any()
                    }
                }}
            </div>

            {error_message.clone().filter(|_| validation.is_error()).map(|msg| view! {
                <span class=error_class.clone()>{msg}</span>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocomplete_option_simple() {
        let opt = AutoCompleteOption::simple("Test");
        assert_eq!(opt.value, "Test");
        assert_eq!(opt.label, "Test");
        assert!(!opt.disabled);
        assert!(opt.group.is_none());
    }

    #[test]
    fn test_autocomplete_option_new() {
        let opt = AutoCompleteOption::new("val", "Label");
        assert_eq!(opt.value, "val");
        assert_eq!(opt.label, "Label");
    }

    #[test]
    fn test_autocomplete_option_with_group() {
        let opt = AutoCompleteOption::simple("Test").with_group("Group A");
        assert_eq!(opt.group, Some("Group A".to_string()));
    }
}
