use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::components::common::icon::{Icon, IconName};
use crate::state::{NavTab, use_dashboard};

/// Single navigation tab item - extracted to prevent re-render loops
#[component]
fn NavTabItem(
    tab: NavTab,
    pathname: Memo<String>,
    jobs_badge_count: RwSignal<u32>,
) -> impl IntoView {
    let path = tab.path();

    // Use memo to compute active state reactively
    let is_active = Memo::new(move |_| {
        let current = pathname.get();
        if path == "/" {
            current == "/" || current.is_empty()
        } else {
            current.starts_with(path)
        }
    });

    view! {
        <a
            href=path
            class=move || format!(
                "flex flex-col items-center justify-center gap-1 px-3 py-2 min-w-[64px] transition-colors {}",
                if is_active.get() {
                    "text-primary-500"
                } else {
                    "text-slate-400 hover:text-slate-600 dark:hover:text-slate-300"
                }
            )
        >
            <div style="position: relative; display: inline-flex;">
                <NavIcon tab=tab />
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
            <span class="text-xs font-medium">{tab.label()}</span>
        </a>
    }
}

/// Bottom navigation bar for mobile
#[component]
pub fn BottomNav() -> impl IntoView {
    let location = use_location();
    let dashboard = use_dashboard();
    let pathname = location.pathname;
    let jobs_badge_count = dashboard.jobs_new_count;

    view! {
        <nav class="fixed bottom-0 left-0 right-0 bg-white dark:bg-slate-800 border-t border-slate-200 dark:border-slate-700 safe-area-bottom z-50">
            <div class="flex items-center justify-around h-16 max-w-lg mx-auto px-2">
                {NavTab::all().iter().map(|tab| {
                    view! { <NavTabItem tab=*tab pathname=pathname jobs_badge_count=jobs_badge_count /> }
                }).collect::<Vec<_>>()}
            </div>
        </nav>
    }
}

/// Icon for navigation tab
#[component]
fn NavIcon(tab: NavTab) -> impl IntoView {
    let icon_name = match tab {
        NavTab::Home => IconName::Home,
        NavTab::Marketplace => IconName::Globe,
        NavTab::Agents => IconName::Users,
        NavTab::Licenses => IconName::Key,
        NavTab::Nodes => IconName::Server,
        NavTab::Rewards => IconName::Rewards,
        NavTab::Jobs => IconName::Clock,
        NavTab::Cms => IconName::Folder,
        NavTab::Settings => IconName::Settings,
    };

    view! {
        <Icon name=icon_name size=24 />
    }
}
