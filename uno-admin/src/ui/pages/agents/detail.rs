//! Agent detail page showing recruited licenses with full API data

use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_navigate};
use server_fn::ServerFnError;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName, UsdCoinIcon};
use crate::handler::{get_agent_with_licenses, import_licenses_csv, delete_agent, get_license_daily_rewards, DailyRewardDto};
use crate::models::entity::{AgentEntity, LicenseEntity};
use crate::api::types::{LicenseApi, AllocationsSummaryApi, UptimeAnalyticsPoint, RewardAllocation};
use std::collections::HashMap;
use chrono::{Utc, Duration};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlInputElement, FileReader};

/// Combined license data from all API calls
#[derive(Debug, Clone, Default)]
pub struct LicenseFullData {
    pub license_info: Option<LicenseApi>,
    pub allocations: Option<AllocationsSummaryApi>,
    pub analytics: Vec<UptimeAnalyticsPoint>,
}

/// Agent detail page
#[component]
pub fn AgentDetailPage() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();

    let agent_id = move || {
        params.get().get("name").map(|s| s.clone()).unwrap_or_default()
    };

    // Note: Sync temporarily disabled - was causing page freeze
    // TODO: Re-enable once reactive issues are resolved

    // Resource to fetch agent with licenses
    let agent_resource = Resource::new(
        move || agent_id(),
        |id| async move {
            if id.is_empty() {
                return Ok(None);
            }
            get_agent_with_licenses(id).await
        }
    );

    // API allocations state for charts (fetched client-side)
    let (api_allocations, set_api_allocations) = signal(Vec::<RewardAllocation>::new());
    let (api_loading, set_api_loading) = signal(false);
    let (api_fetch_initiated, set_api_fetch_initiated) = signal(false);

    // CSV upload state
    let (show_csv_upload, set_show_csv_upload) = signal(false);
    let (csv_content, set_csv_content) = signal(String::new());
    let (csv_status, set_csv_status) = signal(Option::<String>::None);
    let (show_delete_confirm, set_show_delete_confirm) = signal(false);
    let (importing, set_importing) = signal(false);
    let (deleting, set_deleting) = signal(false);

    // License preview state for CSV upload
    let (fetched_licenses, set_fetched_licenses) = signal(Vec::<(String, String, Option<LicenseApi>)>::new());
    let (fetching, set_fetching) = signal(false);
    let (fetch_progress, set_fetch_progress) = signal(String::new());

    // Full license data state - stores rewards data for each license
    // Populated from database via server functions
    let (license_data, set_license_data) = signal(HashMap::<String, LicenseFullData>::new());

    // Note: Rewards fetching temporarily disabled to fix page freeze
    // TODO: Fetch rewards as part of get_agent_with_licenses server function
    let _ = set_license_data; // Suppress unused warning


    // Gear menu state
    let (show_gear_menu, set_show_gear_menu) = signal(false);

    // Summary toggle state (true = expanded, false = collapsed)
    let (summary_expanded, set_summary_expanded) = signal(true);

    // License pagination state
    let (license_page, set_license_page) = signal(0usize);
    let (license_page_size, set_license_page_size) = signal(5usize);

    // Navigation handlers
    let nav_back = navigate.clone();
    let nav_delete = navigate;

    // Note: API allocations fetching disabled - UnityApiClient was removed
    // Rewards data is now fetched via server functions from uno-api
    let _ = (set_api_fetch_initiated, set_api_loading);

    view! {
        <div class="min-h-screen bg-slate-50 dark:bg-slate-900">
            <Header title="Agent Details".to_string() show_search=false />

            <div class="px-4 py-4 space-y-4 max-w-6xl mx-auto" style="padding-left: 16px; padding-right: 16px;">
                // Back button
                <button
                    class="flex items-center gap-2 text-sm text-primary-500 hover:text-primary-600 transition"
                    on:click=move |_| {
                        nav_back("/agents", Default::default());
                    }
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                    </svg>
                    "Back to Agents"
                </button>

                // Main content
                <Suspense fallback=move || view! {
                    <div class="bg-white dark:bg-slate-800 rounded-2xl p-6 border border-slate-200 dark:border-slate-700 shadow-sm">
                        // Progress bar with animated fill - uniform pill shape
                        <div class="relative h-9 bg-slate-200 dark:bg-slate-700" style="border-radius: 18px;">
                            <div class="absolute inset-y-0 left-0 w-1/2 bg-cyan-500 animate-pulse flex items-center justify-center" style="border-radius: 18px;">
                                <span class="text-white text-sm font-medium">"Loading..."</span>
                            </div>
                        </div>
                    </div>
                }>
                    {move || {
                        match agent_resource.get() {
                            None => view! {
                                <div class="text-center py-12 text-slate-500">"Loading..."</div>
                            }.into_any(),
                            Some(Err(e)) => view! {
                                <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
                                    <strong>"Error: "</strong>{e.to_string()}
                                </div>
                            }.into_any(),
                            Some(Ok(None)) => view! {
                                <div class="text-center py-12 text-slate-500 dark:text-slate-400">
                                    "Agent not found"
                                </div>
                            }.into_any(),
                            Some(Ok(Some((agent, licenses)))) => {
                                let agent_id_for_import = agent.id.clone();
                                let agent_id_for_delete = agent.id.clone();
                                let agent_name_for_delete = agent.name.clone();

                                // License IDs are now extracted via Effect to avoid render loops

                                // Import handler
                                let do_import = {
                                    let agent_id = agent_id_for_import.clone();
                                    move || {
                                        let content = csv_content.get();
                                        if content.is_empty() {
                                            return;
                                        }
                                        set_importing.set(true);

                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            use wasm_bindgen_futures::spawn_local;
                                            let id = agent_id.clone();
                                            spawn_local(async move {
                                                match import_licenses_csv(id, content).await {
                                                    Ok(count) => {
                                                        set_csv_status.set(Some(format!("Successfully imported {} licenses", count)));
                                                        set_csv_content.set(String::new());
                                                        set_show_csv_upload.set(false);
                                                        agent_resource.refetch();
                                                    }
                                                    Err(e) => {
                                                        set_csv_status.set(Some(format!("Error: {}", e)));
                                                    }
                                                }
                                                set_importing.set(false);
                                            });
                                        }

                                        #[cfg(not(target_arch = "wasm32"))]
                                        {
                                            set_importing.set(false);
                                        }
                                    }
                                };

                                // Delete handler
                                let nav_for_delete = nav_delete.clone();
                                let do_delete = {
                                    let agent_id = agent_id_for_delete.clone();
                                    move || {
                                        set_deleting.set(true);

                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            use wasm_bindgen_futures::spawn_local;
                                            let id = agent_id.clone();
                                            let nav = nav_for_delete.clone();
                                            spawn_local(async move {
                                                match delete_agent(id).await {
                                                    Ok(true) => {
                                                        nav("/agents", Default::default());
                                                    }
                                                    _ => {
                                                        set_deleting.set(false);
                                                        set_show_delete_confirm.set(false);
                                                    }
                                                }
                                            });
                                        }

                                        #[cfg(not(target_arch = "wasm32"))]
                                        {
                                            set_deleting.set(false);
                                        }
                                    }
                                };

                                view! {
                                    <div class="space-y-6 px-4 py-4 sm:px-6 lg:px-8">
                                        // Agent info card with gear menu
                                        <AgentInfoCard
                                            agent=agent.clone()
                                            licenses_count=licenses.len()
                                            show_gear_menu=show_gear_menu
                                            set_show_gear_menu=set_show_gear_menu
                                            set_show_delete_confirm=set_show_delete_confirm
                                        />

                                        // Import licenses button (centered)
                                        <div class="flex justify-center py-2">
                                            <button
                                                class="px-4 py-2 text-sm font-medium rounded-lg bg-primary-500 text-white hover:bg-primary-600 transition flex items-center gap-2 shadow-sm"
                                                on:click=move |_| set_show_csv_upload.set(true)
                                            >
                                                <Icon name=IconName::Plus size=16 />
                                                "Import Licenses"
                                            </button>
                                        </div>

                                        // Aggregate stats (collapsible) - chart uses database
                                        <AggregateStats
                                            agent_id=agent.id.clone()
                                            licenses=licenses.clone()
                                            license_data=license_data
                                            api_allocations=api_allocations
                                            api_loading=api_loading
                                            expanded=summary_expanded
                                            set_expanded=set_summary_expanded
                                        />

                                        // License cards grid with pagination
                                        <div class="space-y-3 px-4 py-4">
                                            <div class="flex items-center justify-between">
                                                <h3 class="text-lg font-semibold text-slate-900 dark:text-white">
                                                    {format!("Licenses ({})", licenses.len())}
                                                </h3>
                                                // Page size selector
                                                <div class="flex items-center gap-2">
                                                    <span class="text-sm text-slate-500 dark:text-slate-400">"Show:"</span>
                                                    <select
                                                        class="px-3 py-1.5 text-sm rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-slate-900 dark:text-white"
                                                        on:change=move |ev| {
                                                            let value: usize = event_target_value(&ev).parse().unwrap_or(5);
                                                            set_license_page_size.set(value);
                                                            set_license_page.set(0); // Reset to first page
                                                        }
                                                    >
                                                        <option value="5" selected>"5"</option>
                                                        <option value="10">"10"</option>
                                                        <option value="20">"20"</option>
                                                        <option value="50">"50"</option>
                                                    </select>
                                                </div>
                                            </div>

                                            {if licenses.is_empty() {
                                                view! {
                                                    <div class="p-8 text-center bg-white dark:bg-slate-800 rounded-2xl border border-slate-200 dark:border-slate-700">
                                                        <div class="text-slate-400 dark:text-slate-500 mb-2">
                                                            <Icon name=IconName::Plus size=48 />
                                                        </div>
                                                        <p class="text-slate-500 dark:text-slate-400">
                                                            "No licenses yet. Click \"Import Licenses\" to add licenses."
                                                        </p>
                                                    </div>
                                                }.into_any()
                                            } else {
                                                // Pagination logic
                                                let total_licenses = licenses.len();
                                                let page_size = license_page_size.get();
                                                let current_page = license_page.get();
                                                let total_pages = (total_licenses + page_size - 1) / page_size;
                                                let start_idx = current_page * page_size;
                                                let end_idx = std::cmp::min(start_idx + page_size, total_licenses);
                                                let paginated_licenses: Vec<_> = licenses.into_iter()
                                                    .skip(start_idx)
                                                    .take(page_size)
                                                    .collect();

                                                view! {
                                                    <div class="space-y-4 px-4 py-4">
                                                        <div class="grid gap-4">
                                                            {paginated_licenses.into_iter().map(|license| {
                                                                let _license_id = license.license_id.clone();
                                                                view! {
                                                                    <LicenseCard
                                                                        license=license
                                                                        license_data=license_data
                                                                        api_allocations=api_allocations
                                                                    />
                                                                }
                                                            }).collect::<Vec<_>>()}
                                                        </div>

                                                        // Pagination controls
                                                        {if total_pages > 1 {
                                                            let total_pages_val = total_pages;
                                                            let start_display = start_idx + 1;
                                                            let end_display = end_idx;
                                                            let current_display = current_page + 1;
                                                            view! {
                                                                <div class="flex items-center justify-between pt-4 border-t border-slate-200 dark:border-slate-700">
                                                                    <span class="text-sm text-slate-500 dark:text-slate-400">
                                                                        {format!("Showing {}-{} of {}", start_display, end_display, total_licenses)}
                                                                    </span>
                                                                    <div class="flex items-center gap-2">
                                                                        <button
                                                                            class="px-4 py-2 text-sm font-medium rounded-lg bg-primary-500 text-white hover:bg-primary-600 disabled:opacity-50 disabled:cursor-not-allowed transition shadow-sm"
                                                                            disabled=move || license_page.get() == 0
                                                                            on:click=move |_| {
                                                                                set_license_page.update(|p| if *p > 0 { *p -= 1 });
                                                                            }
                                                                        >
                                                                            "Previous"
                                                                        </button>
                                                                        <span class="px-3 py-2 text-sm text-slate-600 dark:text-slate-300">
                                                                            {format!("{} / {}", current_display, total_pages_val)}
                                                                        </span>
                                                                        <button
                                                                            class="px-4 py-2 text-sm font-medium rounded-lg bg-primary-500 text-white hover:bg-primary-600 disabled:opacity-50 disabled:cursor-not-allowed transition shadow-sm"
                                                                            disabled={
                                                                                let max_page = total_pages_val;
                                                                                move || license_page.get() + 1 >= max_page
                                                                            }
                                                                            on:click=move |_| {
                                                                                set_license_page.update(|p| *p += 1);
                                                                            }
                                                                        >
                                                                            "Next"
                                                                        </button>
                                                                    </div>
                                                                </div>
                                                            }.into_any()
                                                        } else {
                                                            view! { <div></div> }.into_any()
                                                        }}
                                                    </div>
                                                }.into_any()
                                            }}
                                        </div>

                                        // CSV Upload Modal
                                        {move || {
                                            if show_csv_upload.get() {
                                                let import = do_import.clone();
                                                Some(view! {
                                                    <CsvUploadModal
                                                        csv_content=csv_content
                                                        set_csv_content=set_csv_content
                                                        csv_status=csv_status
                                                        set_csv_status=set_csv_status
                                                        fetched_licenses=fetched_licenses
                                                        set_fetched_licenses=set_fetched_licenses
                                                        fetching=fetching
                                                        set_fetching=set_fetching
                                                        fetch_progress=fetch_progress
                                                        set_fetch_progress=set_fetch_progress
                                                        importing=importing
                                                        on_close=move || {
                                                            set_show_csv_upload.set(false);
                                                            set_fetched_licenses.set(vec![]);
                                                            set_fetch_progress.set(String::new());
                                                        }
                                                        on_import=import
                                                    />
                                                })
                                            } else {
                                                None
                                            }
                                        }}

                                        // Delete Confirmation Modal
                                        {move || {
                                            if show_delete_confirm.get() {
                                                let delete = do_delete.clone();
                                                let name = agent_name_for_delete.clone();
                                                Some(view! {
                                                    <div class="fixed inset-0 z-50 flex items-center justify-center">
                                                        <div class="absolute inset-0 bg-black/50 backdrop-blur-sm" on:click=move |_| set_show_delete_confirm.set(false) />
                                                        <div class="relative bg-white dark:bg-slate-800 rounded-2xl p-6 w-full max-w-sm mx-4 shadow-2xl">
                                                            <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-2">"Delete Agent?"</h3>
                                                            <p class="text-sm text-slate-600 dark:text-slate-300 mb-6">
                                                                "Are you sure you want to delete " <strong>{name}</strong> "? This will also delete all their licenses."
                                                            </p>

                                                            <div class="flex gap-3">
                                                                <button
                                                                    class="flex-1 px-4 py-2.5 rounded-xl border border-slate-200 dark:border-slate-600 text-slate-700 dark:text-slate-300 font-medium hover:bg-slate-50 dark:hover:bg-slate-700 transition"
                                                                    on:click=move |_| set_show_delete_confirm.set(false)
                                                                >
                                                                    "Cancel"
                                                                </button>
                                                                <button
                                                                    class="flex-1 px-4 py-2.5 rounded-xl bg-red-500 text-white font-medium disabled:opacity-50 hover:bg-red-600 transition"
                                                                    disabled=move || deleting.get()
                                                                    on:click=move |_| delete()
                                                                >
                                                                    {move || if deleting.get() { "Deleting..." } else { "Delete" }}
                                                                </button>
                                                            </div>
                                                        </div>
                                                    </div>
                                                })
                                            } else {
                                                None
                                            }
                                        }}

                                        // Status message
                                        {move || csv_status.get().map(|msg| {
                                            let is_error = msg.starts_with("Error");
                                            view! {
                                                <div class={format!(
                                                    "p-4 rounded-xl text-sm font-medium {}",
                                                    if is_error {
                                                        "bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 border border-red-200 dark:border-red-800"
                                                    } else {
                                                        "bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400 border border-green-200 dark:border-green-800"
                                                    }
                                                )}>
                                                    {msg}
                                                </div>
                                            }
                                        })}
                                    </div>
                                }.into_any()
                            }
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}

/// Agent info card with avatar and gear menu
#[component]
fn AgentInfoCard(
    agent: AgentEntity,
    licenses_count: usize,
    show_gear_menu: ReadSignal<bool>,
    set_show_gear_menu: WriteSignal<bool>,
    set_show_delete_confirm: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="bg-gradient-to-br from-white to-slate-50 dark:from-slate-800 dark:to-slate-800/50 rounded-2xl p-6 border border-slate-200 dark:border-slate-700 shadow-sm">
            <div class="flex items-center gap-6">
                // Avatar - fixed size, won't shrink
                <div
                    class="shrink-0 rounded-2xl bg-gradient-to-br from-primary-400 to-primary-600 flex items-center justify-center shadow-lg"
                    style="width: 72px; height: 72px;"
                >
                    <span class="text-3xl font-bold text-white">
                        {agent.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                    </span>
                </div>
                // Content
                <div class="flex-1 min-w-0">
                    // Name with gear button
                    <div class="flex items-center gap-2">
                        <h2 class="text-xl font-bold text-slate-900 dark:text-white">{agent.name.clone()}</h2>
                        // Gear button (inline with name)
                        <div class="relative">
                            <button
                                class="p-1.5 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-600 transition"
                                on:click=move |_| set_show_gear_menu.update(|v| *v = !*v)
                            >
                                <svg class="w-4 h-4 text-slate-400 dark:text-slate-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                </svg>
                            </button>

                            // Dropdown menu
                            {move || {
                                if show_gear_menu.get() {
                                    Some(view! {
                                        // Backdrop to close on click outside
                                        <div
                                            class="fixed inset-0 z-5"
                                            on:click=move |_| set_show_gear_menu.set(false)
                                        />
                                        <div class="absolute left-0 bottom-[calc(100%-4px)] w-36 bg-white dark:bg-slate-800 rounded-xl shadow-lg border border-slate-200 dark:border-slate-700 py-1 z-10">
                                            <button
                                                class="w-full px-4 py-2 text-left text-sm text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 flex items-center gap-2"
                                                on:click=move |_| {
                                                    set_show_gear_menu.set(false);
                                                }
                                            >
                                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                                                </svg>
                                                "Edit"
                                            </button>
                                            <button
                                                class="w-full px-4 py-2 text-left text-sm text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 flex items-center gap-2"
                                                on:click=move |_| {
                                                    set_show_gear_menu.set(false);
                                                    set_show_delete_confirm.set(true);
                                                }
                                            >
                                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                                </svg>
                                                "Delete"
                                            </button>
                                        </div>
                                    })
                                } else {
                                    None
                                }
                            }}
                        </div>
                    </div>
                    <p class="text-sm text-slate-500 dark:text-slate-400 mt-1">{agent.email.clone()}</p>
                    <div class="flex flex-wrap items-center gap-3 mt-3">
                        <span class="inline-flex items-center px-4 py-2 rounded-lg bg-slate-100 dark:bg-slate-700 text-xs font-medium text-slate-600 dark:text-slate-300">
                            {agent.country.clone()}
                        </span>
                        <span class="inline-flex items-center px-4 py-2 rounded-lg bg-primary-50 dark:bg-primary-900/30 text-xs font-medium text-primary-600 dark:text-primary-400">
                            {format!("{}% commission", agent.commission_percent)}
                        </span>
                        <span class="inline-flex items-center px-4 py-2 rounded-lg bg-green-50 dark:bg-green-900/30 text-xs font-medium text-green-600 dark:text-green-400">
                            {format!("{} licenses", licenses_count)}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Aggregate statistics from all licenses (collapsible)
#[component]
fn AggregateStats(
    agent_id: String,
    licenses: Vec<LicenseEntity>,
    license_data: ReadSignal<std::collections::HashMap<String, LicenseFullData>>,
    api_allocations: ReadSignal<Vec<RewardAllocation>>,
    api_loading: ReadSignal<bool>,
    expanded: ReadSignal<bool>,
    set_expanded: WriteSignal<bool>,
) -> impl IntoView {
    use crate::handler::get_agent_daily_rewards;

    // Fetch daily rewards from database for the chart
    let agent_id_for_chart = agent_id.clone();
    let daily_rewards_resource = Resource::new(
        move || agent_id_for_chart.clone(),
        |id| async move {
            if id.is_empty() {
                return Ok(vec![]);
            }
            get_agent_daily_rewards(id).await
        }
    );
    let total = licenses.len();
    let online_count = licenses.iter().filter(|l| l.is_online).count();
    let avg_uptime = if total > 0 {
        licenses.iter().map(|l| l.uptime).sum::<f64>() / total as f64 * 100.0
    } else {
        0.0
    };
    let license_ids: Vec<String> = licenses.iter().map(|l| l.license_id.clone()).collect();

    // Clone for each closure
    let ids_earnings = license_ids.clone();
    let ids_7days = license_ids.clone();

    view! {
        <div class="bg-white dark:bg-slate-800 rounded-2xl border border-slate-200 dark:border-slate-700 overflow-hidden" style="padding-left: 16px; padding-right: 16px;">
            // Header with toggle button
            <div
                class="flex items-center justify-between px-4 py-3 bg-slate-50 dark:bg-slate-700/50 cursor-pointer hover:bg-slate-100 dark:hover:bg-slate-700 transition"
                on:click=move |_| set_expanded.update(|e| *e = !*e)
            >
                <h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">"Summary"</h3>
                <button class="w-8 h-8 flex items-center justify-center rounded-lg bg-primary-500 text-white hover:bg-primary-600 transition text-lg font-bold shadow-sm">
                    {move || if expanded.get() { "−" } else { "+" }}
                </button>
            </div>

            // Collapsible content
            {move || {
                if expanded.get() {
                    let ids_earnings = ids_earnings.clone();
                    let ids_7days = ids_7days.clone();

                    view! {
                        <div class="px-8 py-6 space-y-4">
                            // Stats row - all in one line, evenly spaced
                            <div class="w-full flex flex-wrap gap-4 justify-between rounded-xl bg-slate-100 dark:bg-slate-700/30" style="padding: 16px;">
                                // Total Earnings
                                <div class="bg-gradient-to-br from-emerald-500 to-green-600 rounded-lg px-5 py-2 text-white shadow-sm">
                                    <div class="text-emerald-100 text-xs font-medium">"Total Earnings"</div>
                                    <div class="text-lg font-bold leading-tight inline-flex items-center gap-0.5">
                                        <UsdCoinIcon size=14 />
                                        {move || {
                                            let allocations = api_allocations.get();
                                            let total_micros: i64 = ids_earnings.iter()
                                                .flat_map(|id| allocations.iter().filter(move |a| &a.license_id == id))
                                                .map(|a| a.amount_micros)
                                                .sum();
                                            format!("{:.2}", total_micros as f64 / 1_000_000.0)
                                        }}
                                    </div>
                                </div>

                                // Last 7 Days
                                <div class="bg-gradient-to-br from-blue-500 to-indigo-600 rounded-lg px-5 py-2 text-white shadow-sm">
                                    <div class="text-blue-100 text-xs font-medium">"Last 7 Days"</div>
                                    <div class="text-lg font-bold leading-tight inline-flex items-center gap-0.5">
                                        <UsdCoinIcon size=14 />
                                        {move || {
                                            let allocations = api_allocations.get();
                                            let now = Utc::now().date_naive();
                                            let seven_days_ago = now - Duration::days(7);
                                            let seven_days_ago_str = seven_days_ago.format("%Y-%m-%d").to_string();

                                            let total_micros: i64 = allocations.iter()
                                                .filter(|a| {
                                                    ids_7days.contains(&a.license_id) && {
                                                        let date_str = a.completed_at.split('T').next().unwrap_or("");
                                                        date_str >= seven_days_ago_str.as_str()
                                                    }
                                                })
                                                .map(|a| a.amount_micros)
                                                .sum();
                                            format!("{:.2}", total_micros as f64 / 1_000_000.0)
                                        }}
                                    </div>
                                </div>

                                // Online Licenses
                                <div class="bg-gradient-to-br from-violet-500 to-purple-600 rounded-lg px-5 py-2 text-white shadow-sm">
                                    <div class="text-violet-100 text-xs font-medium">"Online"</div>
                                    <div class="text-lg font-bold leading-tight">
                                        {move || {
                                            format!("{}/{}", online_count, total)
                                        }}
                                    </div>
                                </div>

                                // Average Uptime
                                <div class="bg-gradient-to-br from-amber-500 to-orange-600 rounded-lg px-5 py-2 text-white shadow-sm">
                                    <div class="text-amber-100 text-xs font-medium">"Avg Uptime"</div>
                                    <div class="text-lg font-bold leading-tight">
                                        {move || {
                                            if avg_uptime > 0.0 {
                                                format!("{:.1}%", avg_uptime)
                                            } else {
                                                "—".to_string()
                                            }
                                        }}
                                    </div>
                                </div>
                            </div>

                            // Bar Chart - full width below stats (uses database)
                            <div class="bg-slate-50 dark:bg-slate-700/30 rounded-xl p-4">
                                <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Daily Earnings (Last 30 Days)"</h4>
                                <Suspense fallback=move || view! {
                                    <div class="h-40 flex items-center justify-center">
                                        <div class="w-5 h-5 border-2 border-[#22d3ee] border-t-transparent rounded-full animate-spin"></div>
                                    </div>
                                }>
                                    {move || {
                                        daily_rewards_resource.get().map(|result| {
                                            match result {
                                                Ok(daily_data) if !daily_data.is_empty() => {
                                                    view! {
                                                        <AgentEarningsChart daily_rewards=daily_data />
                                                    }.into_any()
                                                }
                                                _ => {
                                                    view! {
                                                        <div class="h-40 flex items-center justify-center text-slate-400 text-sm">
                                                            "No earnings data available"
                                                        </div>
                                                    }.into_any()
                                                }
                                            }
                                        })
                                    }}
                                </Suspense>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}

/// Mini bar chart for license card overview (compact version) - uses real data
#[component]
fn MiniBarChart(license_id: String) -> impl IntoView {
    // Fetch daily rewards for this license
    let daily_rewards = Resource::new(
        move || license_id.clone(),
        |id| async move {
            if id.is_empty() {
                return Ok(vec![]);
            }
            get_license_daily_rewards(id).await
        }
    );

    view! {
        <Suspense fallback=move || view! {
            <div class="flex-shrink-0 bg-slate-100 dark:bg-slate-700/50 rounded-lg p-1.5 w-[90px] h-[40px] flex items-center justify-center">
                <div class="w-3 h-3 border border-cyan-500 border-t-transparent rounded-full animate-spin"></div>
            </div>
        }>
            {move || {
                daily_rewards.get().map(|result| {
                    // Get last 7 days of data
                    let chart_data: Vec<f64> = match result {
                        Ok(rewards) if !rewards.is_empty() => {
                            let mut data: Vec<_> = rewards.into_iter()
                                .map(|r| r.total_amount_micros as f64 / 1_000_000.0)
                                .collect();
                            // Take last 7 or pad with zeros
                            while data.len() < 7 {
                                data.insert(0, 0.0);
                            }
                            data.into_iter().rev().take(7).collect::<Vec<_>>().into_iter().rev().collect()
                        }
                        _ => vec![0.0; 7]
                    };

                    let max_value = chart_data.iter().fold(0.0f64, |a, &b| a.max(b));
                    let scale_max = if max_value > 0.0 { max_value * 1.1 } else { 1.0 };

                    // Chart dimensions with space for labels
                    let chart_width: f64 = 100.0;
                    let chart_height: f64 = 44.0;
                    let padding_left: f64 = 18.0;
                    let padding_bottom: f64 = 12.0;
                    let padding_top: f64 = 2.0;
                    let inner_width = chart_width - padding_left;
                    let inner_height = chart_height - padding_bottom - padding_top;
                    let bar_count = chart_data.len();
                    let bar_gap: f64 = 2.0;
                    let bar_width = (inner_width - (bar_count as f64 - 1.0) * bar_gap) / bar_count as f64;

                    // X-axis labels (days)
                    let x_labels = vec!["M", "T", "W", "T", "F", "S", "S"];

                    view! {
                        <div class="flex-shrink-0 bg-slate-100 dark:bg-slate-700/50 rounded-lg p-1.5 text-slate-500 dark:text-slate-400">
                            <svg
                                viewBox={format!("0 0 {} {}", chart_width, chart_height)}
                                width="90"
                                height="40"
                            >
                                // Y-axis label
                                <text
                                    x="2"
                                    y={padding_top + inner_height / 2.0}
                                    text-anchor="start"
                                    fill="#22c55e"
                                    style="font-size: 6px; font-weight: 600;"
                                >
                                    "USD"
                                </text>

                                // Bars
                                {chart_data.iter().enumerate().map(|(idx, &value)| {
                                    let bar_height = if scale_max > 0.0 {
                                        (value / scale_max) * inner_height
                                    } else {
                                        0.0
                                    };
                                    let x = padding_left + idx as f64 * (bar_width + bar_gap);
                                    let y = padding_top + inner_height - bar_height;

                                    view! {
                                        <rect
                                            x={x}
                                            y={y}
                                            width={bar_width.max(4.0)}
                                            height={bar_height.max(2.0)}
                                            rx="1.5"
                                            fill="#22d3ee"
                                        />
                                    }
                                }).collect::<Vec<_>>()}

                                // X-axis labels
                                {x_labels.iter().enumerate().map(|(idx, label)| {
                                    let x = padding_left + idx as f64 * (bar_width + bar_gap) + bar_width / 2.0;
                                    view! {
                                        <text
                                            x={x}
                                            y={chart_height - 2.0}
                                            text-anchor="middle"
                                            fill="currentColor"
                                            style="font-size: 5px;"
                                        >
                                            {*label}
                                        </text>
                                    }
                                }).collect::<Vec<_>>()}
                            </svg>
                        </div>
                    }.into_any()
                })
            }}
        </Suspense>
    }
}

/// Agent daily earnings bar chart (SVG-based) - uses API data
/// Y-axis: Total earnings ($), X-axis: Date
#[component]
fn AgentEarningsChart(
    daily_rewards: Vec<DailyRewardDto>,
) -> impl IntoView {
    let chart_data: Vec<(String, f64)> = daily_rewards.into_iter()
        .map(|r| (r.date, r.total_amount_micros as f64 / 1_000_000.0))
        .collect();

    if chart_data.is_empty() {
        return view! {
            <div class="h-36 flex items-center justify-center text-slate-400 dark:text-slate-500 text-sm">
                "No rewards data for the last 30 days"
            </div>
        }.into_any();
    }

    let max_value = chart_data.iter().map(|(_, v)| *v).fold(0.0f64, f64::max);
    let scale_max = if max_value > 0.0 { max_value * 1.1 } else { 1.0 };
    let bar_count = chart_data.len();

    // Chart dimensions
    let chart_height: f64 = 180.0;
    let chart_width: f64 = 300.0;
    let padding_left: f64 = 40.0;
    let padding_right: f64 = 10.0;
    let padding_top: f64 = 10.0;
    let padding_bottom: f64 = 50.0;
    let inner_width = chart_width - padding_left - padding_right;
    let inner_height = chart_height - padding_top - padding_bottom;
    let bar_gap: f64 = 6.0;
    let bar_width = if bar_count > 0 {
        ((inner_width - (bar_count as f64 - 1.0) * bar_gap) / bar_count as f64).min(28.0)
    } else {
        20.0
    };

    view! {
        <div class="w-full text-slate-600 dark:text-slate-300">
            <svg
                viewBox={format!("0 0 {} {}", chart_width, chart_height)}
                class="w-full h-52"
                preserveAspectRatio="xMidYMid meet"
            >
                // Y-axis line
                <line
                    x1={padding_left}
                    y1={padding_top}
                    x2={padding_left}
                    y2={padding_top + inner_height}
                    stroke="currentColor"
                    stroke-opacity="0.3"
                    stroke-width="1"
                />

                // X-axis line
                <line
                    x1={padding_left}
                    y1={padding_top + inner_height}
                    x2={chart_width - padding_right}
                    y2={padding_top + inner_height}
                    stroke="currentColor"
                    stroke-opacity="0.3"
                    stroke-width="1"
                />

                // Y-axis labels (3 ticks)
                {(0..=2).map(|i| {
                    let y = padding_top + (i as f64 / 2.0) * inner_height;
                    let val = scale_max * (1.0 - i as f64 / 2.0);
                    view! {
                        <text
                            x={padding_left - 5.0}
                            y={y + 3.0}
                            text-anchor="end"
                            fill="currentColor"
                            style="font-size: 8px;"
                        >
                            {format!("{:.2}", val)}
                        </text>
                    }
                }).collect::<Vec<_>>()}

                // Y-axis label
                <text
                    x="10"
                    y={padding_top + inner_height / 2.0}
                    text-anchor="middle"
                    transform={format!("rotate(-90 10 {})", padding_top + inner_height / 2.0)}
                    fill="#22c55e"
                    style="font-size: 9px; font-weight: 600;"
                >
                    "USD"
                </text>

                // Bars
                {chart_data.iter().enumerate().map(|(idx, (date, value))| {
                    let bar_height = if scale_max > 0.0 {
                        (*value / scale_max) * inner_height
                    } else {
                        0.0
                    };
                    let x = padding_left + idx as f64 * (bar_width + bar_gap);
                    let y = padding_top + inner_height - bar_height;

                    // Format date label (MM-DD)
                    let short_date = if date.len() >= 10 {
                        format!("{}/{}", &date[5..7], &date[8..10])
                    } else {
                        date.clone()
                    };

                    let date_clone = date.clone();
                    let value_clone = *value;

                    view! {
                        <g>
                            // Bar
                            <rect
                                x={x}
                                y={y}
                                width={bar_width.max(8.0)}
                                height={bar_height.max(2.0)}
                                fill="#67e8f9"
                                class="hover:fill-cyan-300 transition-colors cursor-pointer"
                            >
                                <title>{format!("{}: ${:.4}", date_clone, value_clone)}</title>
                            </rect>

                            // X-axis label (date) - vertical
                            <text
                                x={x + bar_width / 2.0}
                                y={padding_top + inner_height + 8.0}
                                text-anchor="start"
                                transform={format!("rotate(90 {} {})", x + bar_width / 2.0, padding_top + inner_height + 8.0)}
                                fill="currentColor"
                                style="font-size: 6px;"
                            >
                                {short_date}
                            </text>
                        </g>
                    }
                }).collect::<Vec<_>>()}
            </svg>
        </div>
    }.into_any()
}

/// Individual license card with Details button
#[component]
fn LicenseCard(
    license: LicenseEntity,
    license_data: ReadSignal<std::collections::HashMap<String, LicenseFullData>>,
    api_allocations: ReadSignal<Vec<RewardAllocation>>,
) -> impl IntoView {
    let navigate = use_navigate();
    // Use alias if available, otherwise fall back to truncated license ID
    let display_name = license.alias.clone()
        .filter(|a| !a.is_empty())
        .unwrap_or_else(|| license.license_id_short());
    let license_id_for_earnings = license.license_id.clone();

    // Clone IDs for each closure that needs it
    let id_for_status = license.license_id.clone();
    let id_for_info = license.license_id.clone();
    let id_for_chart = license.license_id.clone();
    let id_for_earnings = license.license_id.clone();
    let id_for_uptime = license.license_id.clone();
    let id_for_nav = license.license_id.clone();

    view! {
        <div class="bg-white dark:bg-slate-800 rounded-2xl border border-slate-200 dark:border-slate-700 overflow-hidden shadow-sm hover:shadow-md transition-shadow">
            <div class="p-4">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        // Status indicator
                        {move || {
                            let data = license_data.get();
                            let is_online = data.get(&id_for_status)
                                .and_then(|d| d.license_info.as_ref())
                                .map(|l| l.is_online)
                                .unwrap_or(license.is_online);

                            view! {
                                <div class={format!(
                                    "w-3 h-3 rounded-full {}",
                                    if is_online { "bg-green-500 shadow-green-500/50 shadow-lg" } else { "bg-slate-300 dark:bg-slate-600" }
                                )}></div>
                            }
                        }}

                        <div>
                            <div class="text-sm font-medium text-slate-900 dark:text-white">
                                {display_name}
                            </div>
                            <div class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
                                {move || {
                                    let data = license_data.get();
                                    data.get(&id_for_info)
                                        .and_then(|d| d.license_info.as_ref())
                                        .map(|l| l.node_id_short())
                                        .unwrap_or_else(|| license.ulo_name.clone())
                                }}
                            </div>
                        </div>
                    </div>

                    <div class="flex items-center gap-4">
                        // Mini bar chart
                        <MiniBarChart license_id=id_for_chart.clone() />

                        // Earnings badge - calculate 7-day earnings from allocations
                        {move || {
                            let allocations = api_allocations.get();
                            let license_id = license_id_for_earnings.clone();

                            // Calculate date 7 days ago
                            let now = Utc::now().date_naive();
                            let seven_days_ago = now - Duration::days(7);
                            let seven_days_ago_str = seven_days_ago.format("%Y-%m-%d").to_string();

                            // Sum earnings for this license in the last 7 days
                            let earnings: f64 = allocations.iter()
                                .filter(|a| {
                                    a.license_id == license_id && {
                                        let date_str = a.completed_at.split('T').next().unwrap_or("");
                                        date_str >= seven_days_ago_str.as_str()
                                    }
                                })
                                .map(|a| a.amount_micros as f64 / 1_000_000.0)
                                .sum();

                            view! {
                                <div class="text-right">
                                    <div class="text-sm font-semibold text-emerald-600 dark:text-emerald-400 inline-flex items-center gap-0.5">
                                        <UsdCoinIcon size=12 />{format!("{:.2}", earnings)}
                                    </div>
                                    <div class="text-xs text-slate-500 dark:text-slate-400">"7-day earnings"</div>
                                </div>
                            }
                        }}

                        // Uptime badge
                        {move || {
                            let data = license_data.get();
                            let uptime = data.get(&id_for_uptime)
                                .and_then(|d| d.license_info.as_ref())
                                .map(|l| l.uptime * 100.0)
                                .unwrap_or(license.uptime * 100.0);

                            let color = if uptime >= 90.0 {
                                "text-green-600 dark:text-green-400"
                            } else if uptime >= 70.0 {
                                "text-amber-600 dark:text-amber-400"
                            } else {
                                "text-red-600 dark:text-red-400"
                            };

                            view! {
                                <div class={format!("text-sm font-semibold {}", color)}>
                                    {format!("{:.1}%", uptime)}
                                </div>
                            }
                        }}

                        // Details button
                        {
                            let nav = navigate.clone();
                            let license_id = id_for_nav.clone();
                            view! {
                                <button
                                    class="px-4 py-2 text-sm font-medium rounded-lg bg-primary-500 text-white hover:bg-primary-600 transition shadow-sm"
                                    on:click=move |_| {
                                        nav(&format!("/licenses/{}", license_id), Default::default());
                                    }
                                >
                                    "Details"
                                </button>
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}

/// CSV Upload Modal component
#[component]
fn CsvUploadModal(
    csv_content: ReadSignal<String>,
    set_csv_content: WriteSignal<String>,
    csv_status: ReadSignal<Option<String>>,
    set_csv_status: WriteSignal<Option<String>>,
    fetched_licenses: ReadSignal<Vec<(String, String, Option<LicenseApi>)>>,
    set_fetched_licenses: WriteSignal<Vec<(String, String, Option<LicenseApi>)>>,
    fetching: ReadSignal<bool>,
    set_fetching: WriteSignal<bool>,
    fetch_progress: ReadSignal<String>,
    set_fetch_progress: WriteSignal<String>,
    importing: ReadSignal<bool>,
    on_close: impl Fn() + Clone + 'static,
    on_import: impl Fn() + Clone + 'static,
) -> impl IntoView {
    let on_close_bg = on_close.clone();
    let on_close_btn = on_close.clone();
    let on_close_cancel = on_close;

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center">
            <div
                class="absolute inset-0 bg-black/50 backdrop-blur-sm"
                on:click=move |_| on_close_bg()
            />
            <div class="relative bg-white dark:bg-slate-800 rounded-2xl p-6 w-full max-w-4xl mx-4 shadow-2xl max-h-[90vh] overflow-y-auto">
                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-lg font-semibold text-slate-900 dark:text-white">"Import Licenses CSV"</h3>
                    <button
                        class="p-2 rounded-xl hover:bg-slate-100 dark:hover:bg-slate-700 transition"
                        on:click=move |_| on_close_btn()
                    >
                        <Icon name=IconName::Close size=20 />
                    </button>
                </div>

                <div class="mb-4 text-sm text-slate-600 dark:text-slate-300">
                    <p class="mb-2">"Expected CSV format:"</p>
                    <code class="block p-3 bg-slate-100 dark:bg-slate-700 rounded-xl text-xs font-mono">
                        "license_id,lease_code"
                    </code>
                </div>

                // File upload
                <div class="mb-4">
                    <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                        "Upload CSV File"
                    </label>
                    <input
                        type="file"
                        accept=".csv,text/csv"
                        class="block w-full text-sm text-slate-500 dark:text-slate-400
                            file:mr-4 file:py-2 file:px-4
                            file:rounded-lg file:border-0
                            file:text-sm file:font-medium
                            file:bg-primary-500 file:text-white
                            file:shadow-sm
                            hover:file:bg-primary-600
                            file:transition
                            cursor-pointer"
                        on:change=move |ev| {
                            #[cfg(target_arch = "wasm32")]
                            {
                                let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
                                if let Some(files) = input.files() {
                                    if let Some(file) = files.get(0) {
                                        let reader = FileReader::new().unwrap();
                                        let reader_clone = reader.clone();

                                        let onload = Closure::wrap(Box::new(move || {
                                            if let Ok(result) = reader_clone.result() {
                                                if let Some(text) = result.as_string() {
                                                    set_csv_content.set(text);
                                                    set_csv_status.set(Some("File loaded".to_string()));
                                                    set_fetched_licenses.set(vec![]);
                                                }
                                            }
                                        }) as Box<dyn FnMut()>);

                                        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                                        onload.forget();
                                        let _ = reader.read_as_text(&file);
                                    }
                                }
                            }
                        }
                    />
                </div>

                <div class="text-xs text-slate-500 dark:text-slate-400 mb-2">"Or paste CSV content:"</div>

                <textarea
                    class="w-full h-32 px-4 py-3 rounded-xl border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-slate-900 dark:text-white font-mono text-sm resize-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition"
                    placeholder="Paste CSV content here..."
                    prop:value=move || csv_content.get()
                    on:input=move |ev| {
                        set_csv_content.set(event_target_value(&ev));
                        set_csv_status.set(None);
                        set_fetched_licenses.set(vec![]);
                    }
                />

                // Progress
                {move || {
                    let progress = fetch_progress.get();
                    if !progress.is_empty() {
                        Some(view! {
                            <div class="mt-3 p-3 rounded-xl bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400 text-sm flex items-center gap-2">
                                <div class="w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                                {progress}
                            </div>
                        })
                    } else {
                        None
                    }
                }}

                // Status
                {move || {
                    csv_status.get().map(|msg| {
                        let is_error = msg.starts_with("Error");
                        view! {
                            <div class={format!(
                                "mt-3 p-3 rounded-xl text-sm {}",
                                if is_error {
                                    "bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400"
                                } else {
                                    "bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400"
                                }
                            )}>
                                {msg}
                            </div>
                        }
                    })
                }}

                <div class="mt-4 flex gap-3">
                    <button
                        class="flex-1 px-4 py-2 text-sm font-medium rounded-lg border-2 border-primary-500 text-primary-600 dark:text-primary-400 hover:bg-primary-50 dark:hover:bg-primary-900/20 transition shadow-sm"
                        on:click=move |_| on_close_cancel()
                    >
                        "Cancel"
                    </button>
                    <button
                        class="flex-1 px-4 py-2 text-sm font-medium rounded-lg bg-primary-500 text-white hover:bg-primary-600 disabled:opacity-50 disabled:cursor-not-allowed transition shadow-sm"
                        disabled=move || importing.get() || csv_content.get().is_empty()
                        on:click=move |_| on_import()
                    >
                        {move || if importing.get() { "Importing..." } else { "Import" }}
                    </button>
                </div>
            </div>
        </div>
    }
}
