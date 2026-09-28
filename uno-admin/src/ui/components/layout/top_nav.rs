use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::components::common::icon::{Icon, IconName};
use crate::state::NavTab;

/// Top navigation bar (horizontal)
#[component]
pub fn TopNav() -> impl IntoView {
    let location = use_location();

    view! {
        <nav class="top-nav bg-white dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700 z-50">
            <div class="flex items-center h-14 px-4 gap-1 overflow-x-auto">
                {NavTab::all().iter().map(|tab| {
                    let tab = *tab;
                    let path = tab.path();
                    let is_active = {
                        let pathname = location.pathname;
                        move || {
                            let current = pathname.get();
                            if path == "/" {
                                current == "/" || current.is_empty()
                            } else {
                                current.starts_with(path)
                            }
                        }
                    };

                    view! {
                        <a
                            href=path
                            class=move || format!(
                                "flex items-center gap-2 px-4 py-2 rounded-lg whitespace-nowrap transition-colors {}",
                                if is_active() {
                                    "bg-primary-50 dark:bg-primary-900 text-primary-500"
                                } else {
                                    "text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-700 hover:text-slate-700 dark:hover:text-slate-300"
                                }
                            )
                        >
                            <TopNavIcon tab=tab />
                            <span class="text-sm font-medium">{tab.label()}</span>
                        </a>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </nav>
    }
}

/// Icon for top navigation item
#[component]
fn TopNavIcon(tab: NavTab) -> impl IntoView {
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
        <Icon name=icon_name size=18 />
    }
}
