use leptos::prelude::*;
use crate::context::{use_theme, Theme};

/// Theme toggle button - cycles through Light, Dark, and Unity Blue
#[component]
pub fn ThemeToggle(
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let theme_ctx = use_theme();

    let toggle = move |_| {
        theme_ctx.toggle();
    };

    view! {
        <button
            type="button"
            class=format!(
                "theme-toggle-btn flex items-center gap-1.5 px-2 py-1.5 rounded-lg transition-colors {}",
                class
            )
            on:click=toggle
            title=move || format!("Theme: {} (click to change)", theme_ctx.current().label())
        >
            {move || {
                match theme_ctx.current() {
                    Theme::Light => view! {
                        <svg class="theme-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="12" cy="12" r="5"/>
                            <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
                        </svg>
                    }.into_any(),
                    Theme::Dark => view! {
                        <svg class="theme-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
                        </svg>
                    }.into_any(),
                    Theme::DarkBlue => view! {
                        <svg class="theme-icon unity-blue" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="12" cy="12" r="10"/>
                            <path d="M12 2a7 7 0 0 0 0 14 7 7 0 0 0 0-14z" fill="currentColor" opacity="0.3"/>
                            <path d="M12 6v6l4 2"/>
                        </svg>
                    }.into_any(),
                }
            }}
        </button>
    }
}
