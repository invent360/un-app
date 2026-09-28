use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::components::common::icon::{Icon, IconName};
use crate::state::{NavTab, use_dashboard};

/// Single sidebar navigation item - extracted to prevent re-render loops
#[component]
fn SidebarNavItem(
    tab: NavTab,
    pathname: Memo<String>,
    jobs_badge_count: RwSignal<u32>,
) -> impl IntoView {
    let path = tab.path();
    let label = tab.label();

    // Use memo to compute active state reactively
    let is_active = Memo::new(move |_| {
        let current = pathname.get();
        if tab == NavTab::Cms {
            // CMS is active if any CMS sub-path is active
            current.starts_with("/cms") ||
            current.starts_with("/content") ||
            current.starts_with("/reviews") ||
            current.starts_with("/publish")
        } else if path == "/" {
            current == "/" || current.is_empty()
        } else {
            current.starts_with(path)
        }
    });

    view! {
        <a
            href=path
            class=move || format!(
                "sidebar-icon-item {}",
                if is_active.get() { "active" } else { "" }
            )
            title=label
        >
            <div style="position: relative; display: inline-flex;">
                <SidebarIcon tab=tab />
                // Only show badge for Jobs tab
                {(tab == NavTab::Jobs).then(|| view! {
                    <Show when=move || { jobs_badge_count.get() > 0 }>
                        <span style="position: absolute; top: -8px; right: -10px; min-width: 16px; height: 16px; padding: 0 4px; display: flex; align-items: center; justify-content: center; font-size: 9px; font-weight: 700; color: white; background: #ef4444; border-radius: 8px; line-height: 1;">
                            {move || {
                                let c = jobs_badge_count.get();
                                if c > 99 { "99+".to_string() } else { c.to_string() }
                            }}
                        </span>
                    </Show>
                })}
            </div>
            <span class="sidebar-tooltip">{label}</span>
        </a>
    }
}

/// Sidebar navigation for desktop view (icons only with hover tooltips)
#[component]
pub fn SidebarNav() -> impl IntoView {
    let location = use_location();
    let dashboard = use_dashboard();
    let pathname = location.pathname;
    let jobs_badge_count = dashboard.jobs_new_count;

    view! {
        <nav class="sidebar bg-white dark:bg-slate-800 border-r border-slate-200 dark:border-slate-700 flex flex-col items-center">
            // Logo/Brand area
            <div class="p-1 border-b border-slate-200 dark:border-slate-700 w-full flex justify-center">
                <div class="w-10 h-10 rounded-xl bg-primary-500 flex items-center justify-center">
                    <span class="text-white font-bold text-lg">"U"</span>
                </div>
            </div>

            // Navigation items (icons only)
            <div class="flex-1 py-4 w-full flex flex-col items-center gap-2">
                {NavTab::all().iter().map(|tab| {
                    view! { <SidebarNavItem tab=*tab pathname=pathname jobs_badge_count=jobs_badge_count /> }
                }).collect::<Vec<_>>()}
            </div>

            // Bottom section with user avatar only
            <div class="p-3 border-t border-slate-200 dark:border-slate-700 w-full flex justify-center">
                <div class="sidebar-icon-item" title="Profile">
                    <div class="w-8 h-8 rounded-full bg-slate-300 dark:bg-slate-600 flex items-center justify-center">
                        <span class="text-sm font-medium text-slate-600 dark:text-slate-300">"JD"</span>
                    </div>
                    <span class="sidebar-tooltip">"John Doe"</span>
                </div>
            </div>
        </nav>
    }
}

/// Icon for sidebar navigation item
#[component]
fn SidebarIcon(tab: NavTab) -> impl IntoView {
    let icon_name = match tab {
        NavTab::Home => IconName::Home,
        NavTab::Marketplace => IconName::Globe,
        NavTab::Nodes => IconName::Server,
        NavTab::Licenses => IconName::Key,
        NavTab::Rewards => IconName::Rewards,
        NavTab::Jobs => IconName::Clock,
        NavTab::Agents => IconName::Users,
        NavTab::Cms => IconName::Folder,
        NavTab::Settings => IconName::Settings,
    };

    view! {
        <Icon name=icon_name size=20 />
    }
}
