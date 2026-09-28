//! Select/Dropdown Leptos component.

use leptos::prelude::*;
use super::types::{SelectionSize, DropdownPlacement, SelectOption};
use crate::try_use_theme;

/// Select component.
///
/// A dropdown select for single value selection.
///
/// # Props
///
/// - `value` - Controlled selected value signal
/// - `options` - Available options
/// - `placeholder` - Placeholder text when no selection
/// - `size` - Size variant (Sm, Md, Lg)
/// - `disabled` - Whether the select is disabled
/// - `loading` - Show loading state
/// - `searchable` - Enable search/filter functionality
/// - `clearable` - Show clear button when value selected
/// - `placement` - Dropdown placement (Top, Bottom, Auto)
/// - `on_change` - Change handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::selection::{Select, SelectOption};
///
/// let selected = RwSignal::new(None::<String>);
/// let options = vec![
///     SelectOption::new("opt1", "Option 1"),
///     SelectOption::new("opt2", "Option 2"),
///     SelectOption::new("opt3", "Option 3"),
/// ];
///
/// view! {
///     <Select
///         value=selected
///         options=options
///         placeholder="Select an option"
///     />
/// }
/// ```
#[component]
pub fn Select(
    /// Controlled selected value.
    #[prop(optional, into)]
    value: Option<RwSignal<Option<String>>>,
    /// Default selected value (uncontrolled).
    #[prop(optional, into)]
    default_value: Option<String>,
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
    /// Show clear button.
    #[prop(optional)]
    clearable: bool,
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
    on_change: Option<Callback<Option<String>>>,
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
    let internal_value = value.unwrap_or_else(|| RwSignal::new(default_value));

    // Dropdown open state
    let is_open = RwSignal::new(false);

    // Search query state
    let search_query = RwSignal::new(String::new());

    // Build CSS classes
    let select_prefix = format!("fx-select-{}", design_system);

    let size_class = if size != SelectionSize::Md {
        size.class(&select_prefix)
    } else {
        String::new()
    };

    // Generate ID
    let select_id = id.unwrap_or_else(|| format!("select-{}", rand_id()));

    // Class names
    let selector_class = format!("{}-selector", select_prefix);
    let selection_class = format!("{}-selection-item", select_prefix);
    let placeholder_class = format!("{}-selection-placeholder", select_prefix);
    let arrow_class = format!("{}-arrow", select_prefix);
    let clear_class = format!("{}-clear", select_prefix);
    let dropdown_class = format!("{}-dropdown", select_prefix);
    let option_class = format!("{}-option", select_prefix);
    let search_class = format!("{}-search", select_prefix);
    let label_class = format!("{}-label", select_prefix);

    // Store options for closures
    let options_for_display = options.clone();
    let options_for_filter = options.clone();
    let options_for_select = options.clone();

    // Get selected label
    let get_selected_label = move || {
        let val = internal_value.get();
        val.as_ref().and_then(|v| {
            options_for_display.iter()
                .find(|opt| &opt.value == v)
                .map(|opt| opt.label.clone())
        })
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

    // Handle option select
    let handle_select = move |opt_value: String| {
        internal_value.set(Some(opt_value.clone()));
        is_open.set(false);
        search_query.set(String::new());
        if let Some(ref cb) = on_change {
            cb.run(Some(opt_value));
        }
    };

    // Handle clear
    let handle_clear = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        internal_value.set(None);
        if let Some(ref cb) = on_change {
            cb.run(None);
        }
    };

    // Handle toggle
    let handle_toggle = move |_| {
        if disabled || loading {
            return;
        }
        is_open.update(|v| *v = !*v);
    };

    // Handle click outside (simplified - in production use proper click outside detection)
    let handle_blur = move |_| {
        // Delay to allow option click to register
        // In production, use proper click-outside detection
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
                    if internal_value.get().is_some() {
                        cls.push(format!("{}-has-value", select_prefix_for_class));
                    }
                    if let Some(ref custom) = class {
                        cls.push(custom.clone());
                    }
                    cls.join(" ")
                }
                id=select_id
                on:blur=handle_blur
            >
                // Selector (the visible part)
                <div
                    class=selector_class.clone()
                    on:click=handle_toggle
                >
                    {move || {
                        let label = get_selected_label();
                        if let Some(l) = label {
                            view! {
                                <span class=selection_class.clone()>{l}</span>
                            }.into_any()
                        } else {
                            view! {
                                <span class=placeholder_class.clone()>
                                    {placeholder.clone().unwrap_or_default()}
                                </span>
                            }.into_any()
                        }
                    }}

                    // Clear button
                    {move || {
                        if clearable && internal_value.get().is_some() && !disabled {
                            Some(view! {
                                <span
                                    class=clear_class.clone()
                                    on:click=handle_clear
                                >
                                    "×"
                                </span>
                            })
                        } else {
                            None
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

                    // Options
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
                                        let opt_value_for_click = opt_value.clone();
                                        let opt_value_for_class = opt_value.clone();
                                        let option_class_for_vec = option_class.clone();
                                        let option_class_for_selected = option_class.clone();
                                        let option_class_for_disabled = option_class.clone();

                                        view! {
                                            <div
                                                class=move || {
                                                    let mut cls = vec![option_class_for_vec.clone()];
                                                    if internal_value.get().as_ref() == Some(&opt_value_for_class) {
                                                        cls.push(format!("{}-selected", option_class_for_selected));
                                                    }
                                                    if is_disabled {
                                                        cls.push(format!("{}-disabled", option_class_for_disabled));
                                                    }
                                                    cls.join(" ")
                                                }
                                                on:click=move |_| {
                                                    if !is_disabled {
                                                        handle_select(opt_value_for_click.clone());
                                                    }
                                                }
                                            >
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
    static COUNTER: AtomicU32 = AtomicU32::new(6000);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:x}", id)
}
