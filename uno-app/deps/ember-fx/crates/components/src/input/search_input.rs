//! SearchInput Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{InputSize, InputVariant, ValidationState};
use crate::try_use_theme;

/// SearchInput component.
///
/// A text input specialized for search functionality with optional button.
/// Supports loading states, clear button, and enter-to-search.
///
/// # Props
///
/// - `value` - Controlled value signal
/// - `placeholder` - Placeholder text (default: "Search...")
/// - `size` - Input size (Sm, Md, Lg)
/// - `variant` - Visual style variant
/// - `loading` - Whether search is in progress
/// - `disabled` - Whether the input is disabled
/// - `allow_clear` - Show clear button when has value
/// - `enter_button` - Show search button (can be true or custom text)
/// - `prefix` - Prefix content (usually search icon)
/// - `suffix` - Suffix content
/// - `validation` - Validation state
/// - `class` - Additional CSS classes
/// - `on_search` - Search handler (triggered on enter or button click)
/// - `on_change` - Change handler
/// - `on_clear` - Clear handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::SearchInput;
///
/// let query = RwSignal::new(String::new());
///
/// view! {
///     // Basic search
///     <SearchInput
///         value=query
///         on_search=move |q| log::info!("Searching: {}", q)
///     />
///
///     // With search button
///     <SearchInput
///         value=query
///         enter_button=true
///         loading=searching.get()
///         on_search=handle_search
///     />
///
///     // With custom button text
///     <SearchInput
///         value=query
///         enter_button="Go"
///         allow_clear=true
///     />
/// }
/// ```
#[component]
pub fn SearchInput(
    /// Controlled value signal.
    #[prop(into)]
    value: RwSignal<String>,
    /// Placeholder text.
    #[prop(optional, into)]
    #[prop(default = "Search...".to_string())]
    placeholder: String,
    /// Input size.
    #[prop(optional, into)]
    size: Option<InputSize>,
    /// Visual variant.
    #[prop(optional, into)]
    variant: Option<InputVariant>,
    /// Whether search is loading.
    #[prop(optional)]
    loading: bool,
    /// Whether the input is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Show clear button.
    #[prop(optional)]
    #[prop(default = true)]
    allow_clear: bool,
    /// Show search button (true for icon, string for custom text).
    #[prop(optional, into)]
    enter_button: Option<SearchButton>,
    /// Prefix content.
    #[prop(optional, into)]
    prefix: Option<String>,
    /// Suffix content.
    #[prop(optional, into)]
    suffix: Option<String>,
    /// Validation state.
    #[prop(optional, into)]
    validation: Option<ValidationState>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Search handler.
    #[prop(optional, into)]
    on_search: Option<Callback<String>>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
    /// Clear handler.
    #[prop(optional, into)]
    on_clear: Option<Callback<()>>,
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
    let has_button = enter_button.is_some();

    // Build CSS classes
    let search_prefix = format!("fx-search-input-{}", design_system);

    let wrapper_class = {
        let mut parts = vec![
            search_prefix.clone(),
            size.class(&search_prefix),
            variant.class(&search_prefix),
        ];

        if disabled {
            parts.push(format!("{}-disabled", search_prefix));
        }
        if loading {
            parts.push(format!("{}-loading", search_prefix));
        }
        if has_button {
            parts.push(format!("{}-with-button", search_prefix));
        }

        let val_class = validation.class(&search_prefix);
        if !val_class.is_empty() {
            parts.push(val_class);
        }

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }

        parts.join(" ")
    };

    // Trigger search
    let on_search_clone = on_search.clone();
    let trigger_search = move || {
        if !disabled && !loading {
            if let Some(ref cb) = on_search_clone {
                cb.run(value.get());
            }
        }
    };

    // Input change handler
    let on_change_clone = on_change.clone();
    let handle_input = move |ev: ev::Event| {
        let input_value = event_target_value(&ev);
        value.set(input_value.clone());
        if let Some(ref cb) = on_change_clone {
            cb.run(input_value);
        }
    };

    // Keyboard handler
    let trigger_search_kb = trigger_search.clone();
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            ev.prevent_default();
            trigger_search_kb();
        }
    };

    // Clear handler
    let on_clear_clone = on_clear.clone();
    let on_change_clear = on_change.clone();
    let handle_clear = move |_: ev::MouseEvent| {
        value.set(String::new());
        if let Some(ref cb) = on_clear_clone {
            cb.run(());
        }
        if let Some(ref cb) = on_change_clear {
            cb.run(String::new());
        }
    };

    // Search button click
    let trigger_search_btn = trigger_search.clone();
    let handle_search_click = move |_: ev::MouseEvent| {
        trigger_search_btn();
    };

    // CSS class names
    let input_wrapper_class = format!("{}-input-wrapper", search_prefix);
    let input_class = format!("{}-input", search_prefix);
    let prefix_class = format!("{}-prefix", search_prefix);
    let suffix_class = format!("{}-suffix", search_prefix);
    let clear_class = format!("{}-clear", search_prefix);
    let button_class = format!("{}-button", search_prefix);
    let spinner_class = format!("{}-spinner", search_prefix);
    let icon_class = format!("{}-icon", search_prefix);

    // Check if has value (for clear button visibility)
    let has_value = move || !value.get().is_empty();

    // Default search icon
    let default_prefix = prefix.unwrap_or_else(|| "🔍".to_string());

    view! {
        <div class=wrapper_class>
            <div class=input_wrapper_class>
                // Search icon prefix
                <span class=prefix_class.clone()>
                    {if loading {
                        view! { <span class=spinner_class.clone()>"⟳"</span> }.into_any()
                    } else {
                        view! { <span class=icon_class.clone()>{default_prefix.clone()}</span> }.into_any()
                    }}
                </span>

                <input
                    type="search"
                    class=input_class
                    value=move || value.get()
                    placeholder=placeholder.clone()
                    disabled=disabled
                    aria-label="Search"
                    on:input=handle_input
                    on:keydown=handle_keydown
                />

                // Suffix content
                {suffix.map(|s| view! {
                    <span class=suffix_class.clone()>{s}</span>
                })}

                // Clear button
                {(allow_clear && !disabled).then(|| view! {
                    <button
                        type="button"
                        class=format!("{} {}", clear_class.clone(), if has_value() { "" } else { "hidden" })
                        class:hidden=move || !has_value()
                        aria-label="Clear search"
                        tabindex=-1
                        on:click=handle_clear.clone()
                    >
                        "✕"
                    </button>
                })}
            </div>

            // Search button
            {enter_button.map(|btn| {
                let btn_text = match btn {
                    SearchButton::Icon => None,
                    SearchButton::Text(t) => Some(t),
                };

                view! {
                    <button
                        type="button"
                        class=button_class.clone()
                        disabled=disabled || loading
                        aria-label="Search"
                        on:click=handle_search_click.clone()
                    >
                        {match btn_text {
                            Some(text) => view! { <span>{text}</span> }.into_any(),
                            None => view! { <span class=icon_class.clone()>"🔍"</span> }.into_any(),
                        }}
                    </button>
                }
            })}
        </div>
    }
}

/// Search button configuration.
#[derive(Debug, Clone, PartialEq)]
pub enum SearchButton {
    /// Show search icon button.
    Icon,
    /// Show button with custom text.
    Text(String),
}

impl From<bool> for SearchButton {
    fn from(value: bool) -> Self {
        if value {
            SearchButton::Icon
        } else {
            // This shouldn't happen due to Option wrapping, but provide fallback
            SearchButton::Icon
        }
    }
}

impl From<&str> for SearchButton {
    fn from(value: &str) -> Self {
        SearchButton::Text(value.to_string())
    }
}

impl From<String> for SearchButton {
    fn from(value: String) -> Self {
        SearchButton::Text(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_button_from_bool() {
        let btn: SearchButton = true.into();
        assert_eq!(btn, SearchButton::Icon);
    }

    #[test]
    fn test_search_button_from_str() {
        let btn: SearchButton = "Go".into();
        assert_eq!(btn, SearchButton::Text("Go".to_string()));
    }
}
