use leptos::prelude::*;
use crate::components::layout::Header;
use crate::api::{
    RewardAllocation,
    fetch_allocations_page_handler,
};

// SECURITY FIX: Removed compile-time JWT token embedding
// JWT tokens must NEVER be compiled into WASM builds
// Token is now retrieved from server-side context or user session
const PAGE_SIZE: u32 = 5;
const DOWNLOAD_PAGE_SIZE: u32 = 100;

/// Trigger a JSON file download in the browser
#[cfg(target_arch = "wasm32")]
fn download_json(data: &[RewardAllocation], filename: &str) {
    use wasm_bindgen::JsCast;

    let json = serde_json::to_string_pretty(data).unwrap_or_default();

    // Create blob using JavaScript
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&wasm_bindgen::JsValue::from_str(&json));

    let options = web_sys::BlobPropertyBag::new();
    options.set_type("application/json");

    if let Ok(blob) = web_sys::Blob::new_with_blob_sequence_and_options(&blob_parts, &options) {
        if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            if let Ok(elem) = document.create_element("a") {
                let anchor: web_sys::HtmlAnchorElement = elem.unchecked_into();
                anchor.set_href(&url);
                anchor.set_download(filename);
                anchor.click();
                let _ = web_sys::Url::revoke_object_url(&url);
            }
        }
    }
}

/// Rewards page displaying allocation data with server-side pagination
#[component]
pub fn RewardsPage() -> impl IntoView {
    // Data state - store all fetched pages
    let (all_allocations, set_all_allocations) = signal(Vec::<RewardAllocation>::new());
    let (current_page, set_current_page) = signal(0u32);
    let (has_more, set_has_more) = signal(true);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);
    let (highest_fetched_page, set_highest_fetched_page) = signal(-1i32);

    // Download state
    let (downloading, set_downloading) = signal(false);
    let (download_progress, set_download_progress) = signal(String::new());

    // Current page data (derived from all_allocations)
    let current_page_data = Memo::new(move |_| {
        let all = all_allocations.get();
        let page = current_page.get();
        let start = (page * PAGE_SIZE) as usize;
        let end = std::cmp::min(start + PAGE_SIZE as usize, all.len());
        if start >= all.len() {
            vec![]
        } else {
            all[start..end].to_vec()
        }
    });

    // Summary stats (computed from all fetched data)
    let summary = Memo::new(move |_| {
        let all = all_allocations.get();
        let total_count = all.len();
        let total_micros: i64 = all.iter().map(|a| a.amount_micros).sum();
        let nodes: std::collections::HashSet<_> = all.iter().map(|a| &a.node_id).collect();
        let licenses: std::collections::HashSet<_> = all.iter().map(|a| &a.license_id).collect();
        (total_count, total_micros, nodes.len(), licenses.len())
    });

    let has_prev = Memo::new(move |_| current_page.get() > 0);
    let has_next = Memo::new(move |_| has_more.get() || (current_page.get() as i32) < highest_fetched_page.get());

    // Fetch a specific page
    #[cfg(target_arch = "wasm32")]
    let fetch_page = {
        use wasm_bindgen_futures::spawn_local;

        move |page: u32| {
            // SECURITY FIX: JWT tokens are no longer embedded at compile time.
            // Authentication must be handled via server functions.
            // This WASM path now requires the token to be retrieved from
            // a server-side session or wallet authentication flow.
            //
            // TODO: Integrate with wallet_auth.rs to get JWT after SIWE authentication
            // For now, show an authentication required message.

            if page == 0 {
                web_sys::console::warn_1(&"[Rewards] This feature requires authentication. Please use server-side fetching.".into());
            }

            // Try to get token from localStorage (set by wallet auth flow)
            let token = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
                .and_then(|s| s.get_item("unity_jwt_token").ok())
                .flatten()
                .unwrap_or_default();

            if token.is_empty() {
                set_error.set(Some("Authentication required. Please connect your wallet first.".to_string()));
                return;
            }

            set_loading.set(true);
            set_error.set(None);

            let token = token.clone();

            spawn_local(async move {
                web_sys::console::log_1(&format!("[Rewards] Fetching page {} (skip={}, take={})", page, page * PAGE_SIZE, PAGE_SIZE).into());

                match fetch_allocations_page_handler(token, page, PAGE_SIZE).await {
                    Ok(response) => {
                        web_sys::console::log_1(&format!("[Rewards] Got {} items, has_more={}", response.data.len(), response.has_more).into());

                        // Append new data
                        set_all_allocations.update(|all| {
                            all.extend(response.data);
                        });
                        set_has_more.set(response.has_more);
                        set_highest_fetched_page.set(page as i32);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("[Rewards] Error: {}", e).into());
                        set_error.set(Some(e));
                    }
                }
                set_loading.set(false);
            });
        }
    };

    // Initial fetch on mount
    #[cfg(target_arch = "wasm32")]
    {
        let fetch_page_clone = fetch_page.clone();
        Effect::new(move |_| {
            if highest_fetched_page.get() < 0 && !loading.get() {
                web_sys::console::log_1(&"[Rewards] Initial load - fetching page 0".into());
                fetch_page_clone(0);
            }
        });
    }

    // SSR placeholder
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (set_all_allocations, set_has_more, set_loading, set_error, set_highest_fetched_page);
    }

    // Signal to trigger fetching of next page
    let (fetch_trigger, set_fetch_trigger) = signal(Option::<u32>::None);

    // Effect to handle fetch trigger (WASM only)
    #[cfg(target_arch = "wasm32")]
    {
        let fetch_page_clone = fetch_page.clone();
        Effect::new(move |_| {
            if let Some(page) = fetch_trigger.get() {
                if page as i32 > highest_fetched_page.get() && has_more.get() && !loading.get() {
                    fetch_page_clone(page);
                }
            }
        });
    }

    // SSR placeholder for fetch_trigger
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = set_fetch_trigger;
    }

    // Download all handler - disabled (UnityApiClient was removed)
    // TODO: Re-implement using uno-api when rewards API is integrated
    #[cfg(target_arch = "wasm32")]
    let on_download_all = move |_| {
        set_download_progress.set("Download feature temporarily unavailable".to_string());
    };

    // SSR placeholder for download
    #[cfg(not(target_arch = "wasm32"))]
    let on_download_all = move |_: leptos::ev::MouseEvent| {};

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (set_downloading, set_download_progress);
    }

    // Navigation handlers
    let on_prev = move |_| {
        if current_page.get() > 0 {
            set_current_page.update(|p| *p -= 1);
        }
    };

    let on_next = move |_| {
        let next_page = current_page.get() + 1;
        set_current_page.set(next_page);
        set_fetch_trigger.set(Some(next_page));
    };

    view! {
        <div>
            <Header title="Rewards".to_string() show_search=false />

            <div class="rewards-page px-4 py-4 space-y-4">
                // Download section
                <div class="bg-white dark:bg-slate-800 rounded-xl p-4 border border-slate-200 dark:border-slate-700">
                    <div class="flex items-center justify-between">
                        <div>
                            <h3 class="font-medium text-slate-900 dark:text-white">"Export Data"</h3>
                            <p class="text-sm text-slate-500 dark:text-slate-400">"Download all allocations as JSON"</p>
                        </div>
                        <button
                            class="px-4 py-2 bg-primary-500 hover:bg-primary-600 disabled:bg-slate-400 text-white text-sm font-medium rounded-lg transition flex items-center gap-2"
                            on:click=on_download_all
                            disabled=move || downloading.get()
                        >
                            {move || if downloading.get() {
                                view! {
                                    <div class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                                    <span>"Downloading..."</span>
                                }.into_any()
                            } else {
                                view! {
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                                    </svg>
                                    <span>"Download All JSON"</span>
                                }.into_any()
                            }}
                        </button>
                    </div>
                    {move || {
                        let progress = download_progress.get();
                        if !progress.is_empty() {
                            Some(view! {
                                <div class="mt-3 text-sm text-slate-600 dark:text-slate-300 bg-slate-50 dark:bg-slate-700/50 rounded-lg p-2">
                                    {progress}
                                </div>
                            })
                        } else {
                            None
                        }
                    }}
                </div>

                // Loading state with progress bar (only show on initial load)
                {move || {
                    if loading.get() && all_allocations.get().is_empty() {
                        Some(view! {
                            <div class="bg-white dark:bg-slate-800 rounded-2xl p-6 border border-slate-200 dark:border-slate-700 shadow-sm">
                                // Progress bar with animated fill - uniform pill shape
                                <div class="relative h-9 bg-slate-200 dark:bg-slate-700" style="border-radius: 18px;">
                                    <div class="absolute inset-y-0 left-0 w-1/2 bg-cyan-500 animate-pulse flex items-center justify-center" style="border-radius: 18px;">
                                        <span class="text-white text-sm font-medium">"Loading..."</span>
                                    </div>
                                </div>
                            </div>
                        })
                    } else {
                        None
                    }
                }}

                // Error display
                {move || error.get().map(|e| view! {
                    <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
                        <strong>"Error: "</strong>{e}
                    </div>
                })}

                // Summary cards
                {move || {
                    let (total_count, total_micros, unique_nodes, unique_licenses) = summary.get();
                    if total_count > 0 {
                        let total_amount = total_micros as f64 / 1_000_000.0;
                        Some(view! {
                            <div class="grid grid-cols-2 gap-3">
                                <SummaryCard
                                    label="Total Loaded"
                                    value=format!("{} allocations", total_count)
                                />
                                <SummaryCard
                                    label="Total Rewards"
                                    value=format!("{:.4}", total_amount)
                                />
                                <SummaryCard
                                    label="Unique Nodes"
                                    value=unique_nodes.to_string()
                                />
                                <SummaryCard
                                    label="Unique Licenses"
                                    value=unique_licenses.to_string()
                                />
                            </div>
                        })
                    } else {
                        None
                    }
                }}

                // Allocations list
                {move || {
                    let data = current_page_data.get();
                    if !data.is_empty() {
                        Some(view! {
                            <div class="settings-card bg-white dark:bg-slate-800 rounded-2xl overflow-hidden">
                                <div class="p-4 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between">
                                    <h3 class="text-base font-semibold text-slate-900 dark:text-white">
                                        "Allocations"
                                    </h3>
                                    {move || if loading.get() {
                                        Some(view! {
                                            <div class="w-5 h-5 border-2 border-primary-500 border-t-transparent rounded-full animate-spin"></div>
                                        })
                                    } else {
                                        None
                                    }}
                                </div>

                                <div class="allocations-list">
                                    {data.into_iter().map(|allocation| {
                                        view! { <AllocationRow allocation=allocation /> }
                                    }).collect::<Vec<_>>()}
                                </div>

                                // Pagination
                                <div class="p-4 border-t border-slate-200 dark:border-slate-700 flex items-center justify-between">
                                    <span class="text-sm text-slate-500 dark:text-slate-400">
                                        {move || format!("Page {}", current_page.get() + 1)}
                                        {move || if !has_more.get() {
                                            format!(" of {}", highest_fetched_page.get() + 1)
                                        } else {
                                            String::new()
                                        }}
                                    </span>

                                    <div class="flex gap-2">
                                        <button
                                            class="pagination-btn-solid px-4 py-2 rounded-lg text-sm font-medium transition-colors disabled:opacity-50"
                                            disabled=move || !has_prev.get() || loading.get()
                                            on:click=on_prev
                                        >
                                            "Previous"
                                        </button>
                                        <button
                                            class="pagination-btn-solid px-4 py-2 rounded-lg text-sm font-medium transition-colors disabled:opacity-50"
                                            disabled=move || !has_next.get() || loading.get()
                                            on:click=on_next.clone()
                                        >
                                            {move || if loading.get() { "Loading..." } else { "Next" }}
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }.into_any())
                    } else if !loading.get() && highest_fetched_page.get() >= 0 {
                        Some(view! {
                            <div class="text-center py-12 text-slate-500 dark:text-slate-400">
                                "No allocations found"
                            </div>
                        }.into_any())
                    } else {
                        None
                    }
                }}
            </div>
        </div>
    }
}

/// Summary card component
#[component]
fn SummaryCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <div class="summary-card bg-white dark:bg-slate-800 rounded-xl p-4 border border-slate-200 dark:border-slate-700">
            <div class="text-sm text-slate-500 dark:text-slate-400 mb-1">{label}</div>
            <div class="text-xl font-semibold text-slate-900 dark:text-white">{value}</div>
        </div>
    }
}

/// Single allocation row
#[component]
fn AllocationRow(allocation: RewardAllocation) -> impl IntoView {
    view! {
        <div class="allocation-row flex items-center justify-between p-4 border-b border-slate-100 dark:border-slate-700 last:border-b-0">
            <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-1">
                    <span class="allocation-type text-xs font-medium px-2 py-0.5 rounded-full bg-primary-50 dark:bg-primary-900/30 text-primary-600 dark:text-primary-400">
                        {allocation.allocation_type.replace("_", " ")}
                    </span>
                    <span class="text-xs text-slate-400">
                        {allocation.completed_date()}
                    </span>
                </div>
                <div class="text-sm text-slate-600 dark:text-slate-300 truncate" title=allocation.node_id.clone()>
                    "Node: " {allocation.node_id_short()}
                </div>
                <div class="text-xs text-slate-400 dark:text-slate-500 truncate" title=allocation.license_id.clone()>
                    "License: " {allocation.license_id_short()}
                </div>
            </div>
            <div class="text-right ml-4">
                <div class="text-lg font-semibold text-green-500">
                    {allocation.amount_formatted(6)}
                </div>
                <div class="text-xs text-slate-400">
                    {format!("{} micros", allocation.amount_micros)}
                </div>
            </div>
        </div>
    }
}
