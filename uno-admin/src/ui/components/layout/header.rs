use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::components::common::{Avatar, ThemeToggle, SearchBar};
use crate::components::common::icon::{Icon, IconName};
use crate::state::{use_dashboard, NavTab};
use crate::context::{use_nav_position, NavPosition};

/// Dashboard header with title, search bar, theme toggle, user avatar, and optional nav
#[component]
pub fn Header(
    #[prop(into)] title: String,
    #[prop(default = true)] show_search: bool,
) -> impl IntoView {
    let dashboard = use_dashboard();
    let nav_ctx = use_nav_position();
    let position = nav_ctx.position;

    // Mobile menu state
    let (menu_open, set_menu_open) = signal(false);
    let location = use_location();

    view! {
        <div class="header-wrapper">
            // Main header bar
            <header class="header flex items-center justify-between px-4 py-3 bg-white dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700 gap-3">
                // Left side: hamburger (mobile) + title
                <div class="flex items-center gap-3 flex-shrink-0">
                    // Hamburger menu button (mobile only, when top nav is active)
                    {move || {
                        if position.get() == NavPosition::Top {
                            Some(view! {
                                <button
                                    class="hamburger-btn p-2 -ml-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700"
                                    on:click=move |_| set_menu_open.update(|v| *v = !*v)
                                >
                                    <Icon name=IconName::Menu size=24 />
                                </button>
                            })
                        } else {
                            None
                        }
                    }}

                    <h1 class="header-title text-xl font-semibold text-slate-900 dark:text-white">
                        {title}
                    </h1>
                </div>

                // Search bar (always visible)
                {move || {
                    if show_search {
                        Some(view! {
                            <div class="search-container flex-1 max-w-md">
                                <SearchBar placeholder="Search..." />
                            </div>
                        })
                    } else {
                        None
                    }
                }}

                // Right side actions
                <div class="flex items-center gap-2 flex-shrink-0">
                    <ThemeToggle />

                    {move || {
                        let user = dashboard.user.get();
                        view! {
                            <Avatar
                                name=user.name.clone()
                                src=user.avatar_url.clone().unwrap_or_default()
                            />
                        }
                    }}
                </div>
            </header>

            // Top navigation bar (desktop - visible on md+ screens when top position selected)
            {move || {
                if position.get() == NavPosition::Top {
                    Some(view! {
                        <nav class="top-nav-desktop bg-white dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700">
                            <div class="flex items-center h-12 px-4 gap-1 overflow-x-auto">
                                {NavTab::all().iter().map(|tab| {
                                    let tab = *tab;
                                    let path = tab.path();
                                    let pathname = location.pathname;
                                    let is_active = move || {
                                        let current = pathname.get();
                                        if path == "/" {
                                            current == "/" || current.is_empty()
                                        } else {
                                            current.starts_with(path)
                                        }
                                    };

                                    view! {
                                        <a
                                            href=path
                                            class=move || format!(
                                                "flex items-center gap-2 px-4 py-2 rounded-lg whitespace-nowrap transition-colors {}",
                                                if is_active() {
                                                    "bg-primary-50 dark:bg-primary-900/50 text-primary-500"
                                                } else {
                                                    "text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-700 hover:text-slate-700 dark:hover:text-slate-300"
                                                }
                                            )
                                        >
                                            <NavIcon tab=tab />
                                            <span class="text-sm font-medium">{tab.label()}</span>
                                        </a>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </nav>
                    })
                } else {
                    None
                }
            }}

            // Mobile menu overlay
            {move || {
                if position.get() == NavPosition::Top && menu_open.get() {
                    Some(view! {
                        <div class="mobile-menu-overlay">
                            // Backdrop
                            <div
                                class="fixed inset-0 bg-black/50 z-40"
                                on:click=move |_| set_menu_open.set(false)
                            />

                            // Menu drawer
                            <div class="fixed top-0 left-0 h-full w-64 bg-white dark:bg-slate-800 shadow-xl z-50 transform transition-transform">
                                // Menu header
                                <div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
                                    <span class="text-lg font-semibold text-slate-900 dark:text-white">"Menu"</span>
                                    <button
                                        class="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700"
                                        on:click=move |_| set_menu_open.set(false)
                                    >
                                        <Icon name=IconName::Close size=20 />
                                    </button>
                                </div>

                                // Menu items
                                <nav class="p-2">
                                    {NavTab::all().iter().map(|tab| {
                                        let tab = *tab;
                                        let path = tab.path();
                                        let pathname = location.pathname;
                                        let is_active = move || {
                                            let current = pathname.get();
                                            if path == "/" {
                                                current == "/" || current.is_empty()
                                            } else {
                                                current.starts_with(path)
                                            }
                                        };

                                        view! {
                                            <a
                                                href=path
                                                class=move || format!(
                                                    "flex items-center gap-3 px-4 py-3 rounded-lg transition-colors {}",
                                                    if is_active() {
                                                        "bg-primary-50 dark:bg-primary-900/50 text-primary-500"
                                                    } else {
                                                        "text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700"
                                                    }
                                                )
                                                on:click=move |_| set_menu_open.set(false)
                                            >
                                                <NavIcon tab=tab />
                                                <span class="font-medium">{tab.label()}</span>
                                            </a>
                                        }
                                    }).collect::<Vec<_>>()}
                                </nav>
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}

/// Icon for navigation item
#[component]
fn NavIcon(tab: NavTab) -> impl IntoView {
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
