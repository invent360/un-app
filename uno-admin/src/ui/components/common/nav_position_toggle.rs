use leptos::prelude::*;
use crate::context::{use_nav_position, NavPosition};

/// Toggle for switching navigation position
#[component]
pub fn NavPositionToggle() -> impl IntoView {
    let nav_ctx = use_nav_position();
    let position = nav_ctx.position;

    let set_top = move |_| nav_ctx.set_position(NavPosition::Top);
    let set_left = move |_| nav_ctx.set_position(NavPosition::Left);
    let set_bottom = move |_| nav_ctx.set_position(NavPosition::Bottom);

    view! {
        <div class="nav-position-toggle flex items-center gap-1 bg-slate-100 dark:bg-slate-700 rounded-lg p-1">
            <button
                class=move || format!(
                    "nav-pos-btn p-2 rounded-md transition-colors {}",
                    if position.get() == NavPosition::Top {
                        "bg-white dark:bg-slate-600 shadow-sm text-primary-500"
                    } else {
                        "text-slate-500 hover:text-slate-700 dark:hover:text-slate-300"
                    }
                )
                on:click=set_top
                title="Top navigation"
            >
                <NavTopIcon />
            </button>
            <button
                class=move || format!(
                    "nav-pos-btn p-2 rounded-md transition-colors {}",
                    if position.get() == NavPosition::Left {
                        "bg-white dark:bg-slate-600 shadow-sm text-primary-500"
                    } else {
                        "text-slate-500 hover:text-slate-700 dark:hover:text-slate-300"
                    }
                )
                on:click=set_left
                title="Left sidebar"
            >
                <NavLeftIcon />
            </button>
            <button
                class=move || format!(
                    "nav-pos-btn p-2 rounded-md transition-colors {}",
                    if position.get() == NavPosition::Bottom {
                        "bg-white dark:bg-slate-600 shadow-sm text-primary-500"
                    } else {
                        "text-slate-500 hover:text-slate-700 dark:hover:text-slate-300"
                    }
                )
                on:click=set_bottom
                title="Bottom navigation"
            >
                <NavBottomIcon />
            </button>
        </div>
    }
}

/// Top nav icon (horizontal bar at top)
#[component]
fn NavTopIcon() -> impl IntoView {
    view! {
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="4" rx="1"/>
            <rect x="3" y="10" width="18" height="11" rx="1" stroke-dasharray="3 2"/>
        </svg>
    }
}

/// Left nav icon (vertical bar at left)
#[component]
fn NavLeftIcon() -> impl IntoView {
    view! {
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="5" height="18" rx="1"/>
            <rect x="11" y="3" width="10" height="18" rx="1" stroke-dasharray="3 2"/>
        </svg>
    }
}

/// Bottom nav icon (horizontal bar at bottom)
#[component]
fn NavBottomIcon() -> impl IntoView {
    view! {
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="17" width="18" height="4" rx="1"/>
            <rect x="3" y="3" width="18" height="11" rx="1" stroke-dasharray="3 2"/>
        </svg>
    }
}
