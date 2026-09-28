//! Theme switcher toggle component

use leptos::prelude::*;
use crate::hooks::{use_theme, toggle_theme, initialize_theme, Theme};

/// Theme switcher toggle button
#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    let ctx = use_theme();

    // Initialize theme from localStorage on mount
    Effect::new(move |_| {
        initialize_theme();
    });

    let on_toggle = move |_| {
        toggle_theme();
    };

    // Show current theme icon
    let icon = move || {
        ctx.theme().icon()
    };

    // Show what clicking will switch to
    let title = move || {
        let next = ctx.theme().next();
        format!("Switch to {} mode", next.name())
    };

    view! {
        <button
            class="theme-toggle"
            on:click=on_toggle
            title=title
            aria-label=title
        >
            <span class="theme-icon">{icon}</span>
        </button>
    }
}
