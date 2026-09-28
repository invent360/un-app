//! Generic data table components

use leptos::prelude::*;
use crate::ui::components::common::UsdCoinIcon;

/// Pagination controls component
#[component]
pub fn Pagination(
    current_page: u32,
    total_pages: u32,
    #[prop(optional)] on_prev: Option<Box<dyn Fn() + 'static>>,
    #[prop(optional)] on_next: Option<Box<dyn Fn() + 'static>>,
    #[prop(default = false)] loading: bool,
) -> impl IntoView {
    let has_prev = current_page > 0;
    let has_next = current_page + 1 < total_pages;

    view! {
        <div class="flex items-center justify-between p-4 border-t border-slate-200 dark:border-slate-700">
            <span class="text-sm text-slate-500 dark:text-slate-400">
                {format!("Page {} of {}", current_page + 1, total_pages.max(1))}
            </span>
            <div class="flex gap-2">
                <button
                    class="pagination-btn px-4 py-2 rounded-lg text-sm font-medium transition-colors disabled:opacity-50 bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600"
                    disabled=move || !has_prev || loading
                    on:click=move |_| {
                        if let Some(ref cb) = on_prev {
                            cb();
                        }
                    }
                >
                    "Previous"
                </button>
                <button
                    class="pagination-btn px-4 py-2 rounded-lg text-sm font-medium transition-colors disabled:opacity-50 bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600"
                    disabled=move || !has_next || loading
                    on:click=move |_| {
                        if let Some(ref cb) = on_next {
                            cb();
                        }
                    }
                >
                    {if loading { "Loading..." } else { "Next" }}
                </button>
            </div>
        </div>
    }
}

/// Status indicator badge
#[component]
pub fn StatusBadge(
    #[prop(into)] status: String,
    #[prop(default = false)] online: bool,
) -> impl IntoView {
    let (bg_class, text_class) = if online {
        ("bg-green-100 dark:bg-green-900/30", "text-green-600 dark:text-green-400")
    } else {
        ("bg-slate-100 dark:bg-slate-700", "text-slate-500 dark:text-slate-400")
    };

    view! {
        <span class=format!("inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium {} {}", bg_class, text_class)>
            <span class=format!("w-1.5 h-1.5 rounded-full {}", if online { "bg-green-500" } else { "bg-slate-400" })></span>
            {status}
        </span>
    }
}

/// Uptime percentage display
#[component]
pub fn UptimeDisplay(
    uptime: f64,
) -> impl IntoView {
    let percentage = uptime * 100.0;
    let color_class = if percentage >= 99.0 {
        "text-green-500"
    } else if percentage >= 95.0 {
        "text-yellow-500"
    } else {
        "text-red-500"
    };

    view! {
        <span class=format!("font-medium {}", color_class)>
            {format!("{:.1}%", percentage)}
        </span>
    }
}

/// Amount display with formatting and USD coin icon
#[component]
pub fn AmountDisplay(
    amount_micros: i64,
    #[prop(default = 4)] decimals: usize,
    #[prop(default = false)] show_label: bool,
    #[prop(default = true)] show_icon: bool,
    #[prop(default = "text-slate-900 dark:text-white".to_string())] class: String,
) -> impl IntoView {
    let amount = amount_micros as f64 / 1_000_000.0;
    let formatted = format!("{:.1$}", amount, decimals);

    view! {
        <span class=format!("{} inline-flex items-center gap-0.5", class)>
            {if show_icon {
                view! { <UsdCoinIcon size=14 /> }.into_any()
            } else {
                view! {}.into_any()
            }}
            {formatted}
            {if show_label {
                view! { <span class="text-xs text-slate-400 ml-1">"UNITY"</span> }.into_any()
            } else {
                view! {}.into_any()
            }}
        </span>
    }
}

/// Table header cell with optional sorting
#[component]
pub fn TableHeader(
    #[prop(into)] label: String,
    #[prop(default = false)] sortable: bool,
    #[prop(default = false)] sorted: bool,
    #[prop(default = false)] sort_desc: bool,
    #[prop(optional)] on_sort: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let cursor_class = if sortable { "cursor-pointer hover:text-primary-500" } else { "" };

    view! {
        <th
            class=format!("px-4 py-3 text-left text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider {}", cursor_class)
            on:click=move |_| {
                if sortable {
                    if let Some(ref cb) = on_sort {
                        cb();
                    }
                }
            }
        >
            <div class="flex items-center gap-1">
                {label}
                {if sorted {
                    view! {
                        <span class="text-primary-500">
                            {if sort_desc { "↓" } else { "↑" }}
                        </span>
                    }.into_any()
                } else if sortable {
                    view! {
                        <span class="text-slate-300 dark:text-slate-600">"↕"</span>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </div>
        </th>
    }
}

/// Empty state for tables
#[component]
pub fn EmptyState(
    #[prop(into)] message: String,
    #[prop(optional)] icon: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-12 text-slate-500 dark:text-slate-400">
            {icon.map(|i| view! {
                <span class="text-4xl mb-3">{i}</span>
            })}
            <span class="text-sm">{message}</span>
        </div>
    }
}

/// Loading state for tables
#[component]
pub fn TableLoading() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center py-12">
            <div class="flex flex-col items-center gap-3">
                <div class="w-8 h-8 border-4 border-primary-500 border-t-transparent rounded-full animate-spin"></div>
                <span class="text-sm text-slate-500 dark:text-slate-400">"Loading..."</span>
            </div>
        </div>
    }
}

/// Truncated ID display with tooltip
#[component]
pub fn TruncatedId(
    #[prop(into)] id: String,
    #[prop(default = 16)] max_length: usize,
) -> impl IntoView {
    let truncated = if id.len() > max_length {
        format!("{}...{}", &id[..10], &id[id.len() - 6..])
    } else {
        id.clone()
    };

    view! {
        <span
            class="font-mono text-xs text-slate-600 dark:text-slate-400"
            title=id
        >
            {truncated}
        </span>
    }
}
