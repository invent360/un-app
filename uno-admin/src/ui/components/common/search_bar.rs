use leptos::prelude::*;

/// Search bar component for header
#[component]
pub fn SearchBar(
    #[prop(default = "Search...")] placeholder: &'static str,
    #[prop(optional)] on_search: Option<Callback<String>>,
) -> impl IntoView {
    let (query, set_query) = signal(String::new());

    let handle_input = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        set_query.set(value.clone());
        if let Some(callback) = on_search {
            callback.run(value);
        }
    };

    view! {
        <div class="search-bar">
            // Search icon
            <svg class="search-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="11" cy="11" r="8"/>
                <path d="M21 21l-4.35-4.35"/>
            </svg>
            <input
                type="text"
                placeholder=placeholder
                class="search-input"
                prop:value=move || query.get()
                on:input=handle_input
            />
            {move || {
                if !query.get().is_empty() {
                    Some(view! {
                        <button
                            class="search-clear"
                            on:click=move |_| set_query.set(String::new())
                        >
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M18 6L6 18M6 6l12 12"/>
                            </svg>
                        </button>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
