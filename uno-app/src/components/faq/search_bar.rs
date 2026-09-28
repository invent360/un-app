//! FAQ search bar component

use leptos::prelude::*;

/// Search bar component for filtering FAQ items
#[component]
pub fn SearchBar(
    /// Current search value (read signal)
    value: ReadSignal<String>,
    /// Callback when search value changes
    on_change: impl Fn(String) + Send + Sync + 'static + Copy,
    /// Placeholder text
    #[prop(default = "Search questions...")]
    placeholder: &'static str,
) -> impl IntoView {
    let has_value = move || !value.get().is_empty();

    let on_input = move |ev| {
        let new_value = event_target_value(&ev);
        on_change(new_value);
    };

    let clear_search = move |_| {
        on_change(String::new());
    };

    view! {
        <div class="search-bar">
            <span class="search-icon">"🔍"</span>
            <input
                type="text"
                class="search-input"
                placeholder=placeholder
                prop:value=move || value.get()
                on:input=on_input
            />
            {move || has_value().then(|| view! {
                <button
                    class="clear-btn"
                    type="button"
                    on:click=clear_search
                    title="Clear search"
                >
                    "✕"
                </button>
            })}
        </div>
    }
}
