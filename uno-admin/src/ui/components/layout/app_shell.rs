use leptos::prelude::*;
use super::bottom_nav::BottomNav;
use super::sidebar_nav::SidebarNav;
use crate::context::{use_nav_position, NavPosition};
use crate::state::use_dashboard;
use crate::storage::get_jobs_last_seen;
use crate::handler::get_jobs_badge_count;

/// App shell providing the main layout structure with configurable navigation
#[component]
pub fn AppShell(children: Children) -> impl IntoView {
    let nav_ctx = use_nav_position();
    let position = nav_ctx.position;
    let dashboard = use_dashboard();

    // Load badge count on app initialization
    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            use wasm_bindgen_futures::spawn_local;
            spawn_local(async move {
                // Get the "last seen" timestamp from local storage
                let since_timestamp = get_jobs_last_seen().unwrap_or_else(|| {
                    // Default to a very old date if never seen
                    "2000-01-01T00:00:00Z".to_string()
                });

                // Fetch badge count from server
                match get_jobs_badge_count(since_timestamp).await {
                    Ok(count) => {
                        dashboard.set_jobs_badge(count);
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to load badge count: {}", e).into());
                    }
                }
            });
        });
    }

    // Create memos for position-based classes to avoid warnings
    let shell_class = Memo::new(move |_| {
        format!(
            "app-shell min-h-screen bg-slate-50 dark:bg-slate-900 transition-colors duration-200 {}",
            position.get().body_class()
        )
    });

    let sidebar_class = Memo::new(move |_| {
        if position.get() == NavPosition::Left { "" } else { "hidden" }
    });

    let bottom_nav_class = Memo::new(move |_| {
        format!(
            "bottom-nav-wrapper {}",
            match position.get() {
                NavPosition::Left | NavPosition::Bottom => "",
                NavPosition::Top => "hidden",
            }
        )
    });

    view! {
        <div class=move || shell_class.get()>
            // Sidebar navigation - shown when Left position, hidden via CSS otherwise
            <div class=move || sidebar_class.get()>
                <SidebarNav />
            </div>

            // Main content area (Header with integrated top nav is inside each page)
            <main class="main-content">
                {children()}
            </main>

            // Bottom navigation - shown for Left or Bottom position
            <div class=move || bottom_nav_class.get()>
                <BottomNav />
            </div>
        </div>
    }
}
