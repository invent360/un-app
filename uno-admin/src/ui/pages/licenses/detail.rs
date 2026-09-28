//! License detail page - comprehensive view with API and database data

use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_navigate};
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName, UsdCoinIcon};
use crate::components::common::progress_spinner::{ProgressSpinner, SpinnerSize};
use crate::handler::{
    get_license_by_id, get_license_allocation_summary, get_license_uptime_analytics,
    get_license_settings, get_license_paginated_rewards,
    get_license_daily_rewards, LicenseDto, DailyRewardDto,
    sync_rewards_if_stale,
};
use crate::api::types::{
    AllocationsSummaryApi, UptimeAnalyticsPoint, LicenseSettingsResponse,
};
use crate::models::entity::{PaginatedResult, RewardEntity};
use chrono::{Utc, Duration};

/// License detail page
#[component]
pub fn LicenseDetailPage() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();

    let license_id = move || {
        params.get().get("id").map(|s| s.clone()).unwrap_or_default()
    };

    // Resource for license basic info
    let license_resource = Resource::new(
        move || license_id(),
        |id| async move {
            if id.is_empty() {
                return Ok(None);
            }
            get_license_by_id(id).await
        }
    );

    // Resource for allocation summary
    let summary_resource = Resource::new(
        move || license_id(),
        |id| async move {
            if id.is_empty() {
                return Ok(AllocationsSummaryApi::default());
            }
            get_license_allocation_summary(id).await
        }
    );

    // Resource for uptime analytics (all available data)
    let analytics_resource = Resource::new(
        move || license_id(),
        |id| async move {
            if id.is_empty() {
                return Ok(vec![]);
            }
            // Fetch all available uptime data from 2025-01-01 to far future
            let start = "2025-01-01".to_string();
            let end = "2030-01-01".to_string();
            get_license_uptime_analytics(id, start, end).await
        }
    );

    // Resource for license settings
    let settings_resource = Resource::new(
        move || license_id(),
        |id| async move {
            if id.is_empty() {
                return Ok(LicenseSettingsResponse::default());
            }
            get_license_settings(id).await
        }
    );

    // Resource for daily rewards (for chart)
    let daily_rewards_resource = Resource::new(
        move || license_id(),
        |id| async move {
            if id.is_empty() {
                return Ok(vec![]);
            }
            get_license_daily_rewards(id).await
        }
    );

    // Pagination state for rewards table
    let (current_page, set_current_page) = signal(1u32);
    let (page_size, set_page_size) = signal(10u32);

    // Resource for paginated rewards
    let rewards_resource = Resource::new(
        move || (license_id(), current_page.get(), page_size.get()),
        |(id, page, limit)| async move {
            if id.is_empty() {
                return Ok(PaginatedResult::new(vec![], 0, page, limit));
            }
            get_license_paginated_rewards(id, page, limit).await
        }
    );

    // Navigation handler
    let nav_back = navigate.clone();

    view! {
        <div class="min-h-screen bg-slate-50 dark:bg-slate-900">
            <Header title="License Details".to_string() show_search=false />

            <div class="px-4 py-4 space-y-4 max-w-6xl mx-auto">
                // Back button
                <button
                    class="flex items-center gap-2 text-sm text-primary-500 hover:text-primary-600 transition"
                    on:click=move |_| {
                        nav_back("/licenses", Default::default());
                    }
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                    </svg>
                    "Back to Licenses"
                </button>

                // Main content
                <Suspense fallback=move || view! {
                    <div class="flex items-center justify-center min-h-[50vh]">
                        <ProgressSpinner size=SpinnerSize::Default />
                    </div>
                }>
                    {move || {
                        match license_resource.get() {
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
                                    "License not found"
                                </div>
                            }.into_any(),
                            Some(Ok(Some(license))) => {
                                view! {
                                    <div class="license-detail-container space-y-6">
                                        // License info card
                                        <LicenseInfoCard license=license.clone() />

                                        // Stats grid
                                        <StatsGrid
                                            summary_resource=summary_resource
                                            settings_resource=settings_resource
                                            license=license.clone()
                                        />

                                        // Charts & Rewards Tabs
                                        <ChartsTabsSection
                                            analytics_resource=analytics_resource
                                            daily_rewards_resource=daily_rewards_resource
                                            rewards_resource=rewards_resource
                                            set_page_size=set_page_size
                                            set_current_page=set_current_page
                                        />
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

/// Copy text to clipboard using JavaScript
#[cfg(target_arch = "wasm32")]
fn copy_to_clipboard(text: &str) {
    use wasm_bindgen::JsCast;
    if let Some(window) = web_sys::window() {
        // Use js_sys::Reflect to access navigator without requiring web_sys::Navigator feature
        if let Ok(navigator) = js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("navigator")) {
            if !navigator.is_undefined() {
                if let Ok(clipboard) = js_sys::Reflect::get(&navigator, &wasm_bindgen::JsValue::from_str("clipboard")) {
                    if !clipboard.is_undefined() {
                        if let Ok(write_text) = js_sys::Reflect::get(&clipboard, &wasm_bindgen::JsValue::from_str("writeText")) {
                            if let Ok(func) = write_text.dyn_into::<js_sys::Function>() {
                                let _ = js_sys::Reflect::apply(
                                    &func,
                                    &clipboard,
                                    &js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(text)),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn copy_to_clipboard(_text: &str) {
    // No-op on server
}

/// License info card with status and basic details
#[component]
fn LicenseInfoCard(license: LicenseDto) -> impl IntoView {
    let display_name = license.alias.clone()
        .filter(|a| !a.is_empty())
        .unwrap_or_else(|| license.license_id_short());

    // Format license ID as masked (0x02...de5b9fd9)
    let license_id_full = license.license_id.clone();
    let license_id_masked = if license_id_full.len() > 14 {
        format!("{}...{}", &license_id_full[..6], &license_id_full[license_id_full.len()-8..])
    } else {
        license_id_full.clone()
    };

    // Format node ID as masked
    let node_id_full = license.node_id.clone();
    let node_id_masked = if node_id_full.len() > 14 {
        format!("{}...{}", &node_id_full[..10], &node_id_full[node_id_full.len()-6..])
    } else {
        node_id_full.clone()
    };

    view! {
        <div class="mx-4 my-2 bg-gradient-to-br from-white to-slate-50 dark:from-slate-800 dark:to-slate-800/50 rounded-2xl p-6 border border-slate-200 dark:border-slate-700 shadow-sm">
            <div class="flex items-center gap-6">
                // Status indicator with icon
                <div class={format!(
                    "shrink-0 rounded-2xl flex items-center justify-center shadow-lg {}",
                    if license.is_online { "bg-gradient-to-br from-green-400 to-green-600" } else { "bg-gradient-to-br from-slate-400 to-slate-600" }
                )} style="width: 72px; height: 72px;">
                    <Icon name=IconName::Key size=32 />
                </div>

                // Content
                <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2">
                        <h2 class="text-xl font-bold text-slate-900 dark:text-white">{display_name}</h2>
                        // Status with colored dot
                        <span class={format!(
                            "inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium {}",
                            if license.is_online { "bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400" } else { "bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-400" }
                        )}>
                            <span class={format!(
                                "w-3 h-3 rounded {}",
                                if license.is_online { "bg-green-500" } else { "bg-red-500" }
                            )}></span>
                            {if license.is_online { "Online" } else { "Offline" }}
                        </span>
                    </div>

                    // License ID with copy button
                    <div class="flex items-center gap-2 mt-1">
                        <p class="text-sm text-slate-500 dark:text-slate-400 font-mono">{license_id_masked.clone()}</p>
                        <button
                            class="p-1 hover:bg-slate-200 dark:hover:bg-slate-600 rounded transition"
                            title="Copy License ID"
                            on:click={
                                let id = license_id_full.clone();
                                move |_| {
                                    copy_to_clipboard(&id);
                                }
                            }
                        >
                            <svg class="w-5 h-5 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                            </svg>
                        </button>
                    </div>

                    <div class="flex flex-wrap items-center gap-3 mt-3">
                        // Node ID with copy button
                        <span class="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg bg-slate-100 dark:bg-slate-700 text-xs font-medium text-slate-600 dark:text-slate-300">
                            <Icon name=IconName::Server size=14 />
                            <span class="font-mono">{node_id_masked.clone()}</span>
                            <button
                                class="p-0.5 hover:bg-slate-200 dark:hover:bg-slate-600 rounded transition ml-1"
                                title="Copy Node ID"
                                on:click={
                                    let id = node_id_full.clone();
                                    move |_| {
                                        copy_to_clipboard(&id);
                                    }
                                }
                            >
                                <svg class="w-4 h-4 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                                </svg>
                            </button>
                        </span>

                        // Leased badge
                        {license.is_leased.then(|| view! {
                            <span class="inline-flex items-center px-3 py-1.5 rounded-lg bg-blue-50 dark:bg-blue-900/30 text-xs font-medium text-blue-600 dark:text-blue-400">
                                "Leased"
                            </span>
                        })}

                        {license.is_bound.then(|| view! {
                            <span class="inline-flex items-center px-3 py-1.5 rounded-lg bg-purple-50 dark:bg-purple-900/30 text-xs font-medium text-purple-600 dark:text-purple-400">
                                "Bound"
                            </span>
                        })}

                        <span class={format!(
                            "inline-flex items-center px-3 py-1.5 rounded-lg text-xs font-medium {}",
                            if license.uptime_percentage() >= 90.0 { "bg-green-50 dark:bg-green-900/30 text-green-600 dark:text-green-400" }
                            else if license.uptime_percentage() >= 70.0 { "bg-amber-50 dark:bg-amber-900/30 text-amber-600 dark:text-amber-400" }
                            else { "bg-red-50 dark:bg-red-900/30 text-red-600 dark:text-red-400" }
                        )}>
                            {format!("{:.1}% Uptime", license.uptime_percentage())}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Stats tabs showing earnings, settings, and lease info
#[component]
fn StatsGrid(
    summary_resource: Resource<Result<AllocationsSummaryApi, ServerFnError>>,
    settings_resource: Resource<Result<LicenseSettingsResponse, ServerFnError>>,
    license: LicenseDto,
) -> impl IntoView {
    let (active_tab, set_active_tab) = signal(0u8);

    // Sync state for manual reward sync
    let (syncing, set_syncing) = signal(false);
    let (sync_message, set_sync_message) = signal(Option::<(bool, String)>::None);

    view! {
        <div class="mx-4 my-2 bg-white dark:bg-slate-800 rounded-2xl border border-slate-200 dark:border-slate-700 shadow-sm overflow-hidden">
            // Tab headers - cyan background with white text for active
            <div class="flex border-b border-slate-200 dark:border-slate-700">
                <button
                    class=move || format!(
                        "flex-1 px-4 py-3 text-sm font-medium transition-all {}",
                        if active_tab.get() == 0 {
                            "bg-cyan-500 text-white"
                        } else {
                            "text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:bg-slate-50 dark:hover:bg-slate-700/50"
                        }
                    )
                    on:click=move |_| set_active_tab.set(0)
                >
                    "Total Earnings"
                </button>
                <button
                    class=move || format!(
                        "flex-1 px-4 py-3 text-sm font-medium transition-all border-l border-slate-200 dark:border-slate-700 {}",
                        if active_tab.get() == 1 {
                            "bg-cyan-500 text-white"
                        } else {
                            "text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:bg-slate-50 dark:hover:bg-slate-700/50"
                        }
                    )
                    on:click=move |_| set_active_tab.set(1)
                >
                    "License Settings"
                </button>
                <button
                    class=move || format!(
                        "flex-1 px-4 py-3 text-sm font-medium transition-all border-l border-slate-200 dark:border-slate-700 {}",
                        if active_tab.get() == 2 {
                            "bg-cyan-500 text-white"
                        } else {
                            "text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:bg-slate-50 dark:hover:bg-slate-700/50"
                        }
                    )
                    on:click=move |_| set_active_tab.set(2)
                >
                    "Lease Details"
                </button>
            </div>

            // Tab content
            <div class="p-4">
                // Total Earnings Tab
                <div class=move || if active_tab.get() == 0 { "block" } else { "hidden" }>
                    <Suspense fallback=move || view! {
                        <div class="flex items-center justify-center py-4">
                            <ProgressSpinner size=SpinnerSize::Small />
                        </div>
                    }>
                        {move || {
                            summary_resource.get().map(|result| {
                                match result {
                                    Ok(summary) => {
                                        view! {
                                            <div class="py-1">
                                                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                                                    <div class="bg-emerald-500/10 dark:bg-emerald-500/10 rounded-xl py-6 px-4 text-center border border-emerald-200 dark:border-emerald-800">
                                                        <div class="text-emerald-600 dark:text-emerald-400 text-xs font-medium mb-2">"Total Earnings"</div>
                                                        <div class="font-bold text-xl text-emerald-600 dark:text-emerald-400 flex items-center justify-center gap-1">
                                                            <UsdCoinIcon size=18 />
                                                            {format!("{:.2}", summary.total_amount())}
                                                        </div>
                                                    </div>
                                                    <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl py-6 px-4 text-center border border-slate-200 dark:border-slate-600">
                                                        <div class="text-slate-500 dark:text-slate-400 text-xs font-medium mb-2">"Last 7 Days"</div>
                                                        <div class="font-bold text-xl text-slate-900 dark:text-white">{format!("${:.2}", summary.last_7_days_amount())}</div>
                                                    </div>
                                                    <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl py-6 px-4 text-center border border-slate-200 dark:border-slate-600">
                                                        <div class="text-slate-500 dark:text-slate-400 text-xs font-medium mb-2">"This Week"</div>
                                                        <div class="font-bold text-xl text-slate-900 dark:text-white">{format!("${:.2}", summary.this_week_amount())}</div>
                                                    </div>
                                                    <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl py-6 px-4 text-center border border-slate-200 dark:border-slate-600">
                                                        <div class="text-slate-500 dark:text-slate-400 text-xs font-medium mb-2">"Today"</div>
                                                        <div class="font-bold text-xl text-slate-900 dark:text-white">{format!("${:.2}", summary.today_amount())}</div>
                                                    </div>
                                                </div>

                                                // Sync Now button
                                                <div class="mt-4 flex items-center justify-end gap-3">
                                                    // Sync message
                                                    {move || sync_message.get().map(|(is_success, msg)| {
                                                        let class = if is_success {
                                                            "text-xs px-3 py-1.5 rounded-lg bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400"
                                                        } else {
                                                            "text-xs px-3 py-1.5 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400"
                                                        };
                                                        view! { <div class=class>{msg}</div> }
                                                    })}

                                                    <button
                                                        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-lg bg-cyan-500 text-white hover:bg-cyan-600 transition shadow-sm disabled:opacity-50 disabled:cursor-not-allowed"
                                                        disabled=move || syncing.get()
                                                        on:click=move |_| {
                                                            set_syncing.set(true);
                                                            set_sync_message.set(None);

                                                            #[cfg(target_arch = "wasm32")]
                                                            {
                                                                use wasm_bindgen_futures::spawn_local;
                                                                spawn_local(async move {
                                                                    match sync_rewards_if_stale(String::new()).await {
                                                                        Ok(result) => {
                                                                            if result.was_stale {
                                                                                set_sync_message.set(Some((true, format!("Synced {} rewards", result.rewards_synced))));
                                                                                summary_resource.refetch();
                                                                            } else {
                                                                                set_sync_message.set(Some((true, "Already up to date".to_string())));
                                                                            }
                                                                        }
                                                                        Err(e) => {
                                                                            set_sync_message.set(Some((false, format!("Sync failed: {}", e))));
                                                                        }
                                                                    }
                                                                    set_syncing.set(false);
                                                                });
                                                            }
                                                        }
                                                    >
                                                        {move || if syncing.get() {
                                                            view! {
                                                                <ProgressSpinner size=SpinnerSize::Small />
                                                                "Syncing..."
                                                            }.into_any()
                                                        } else {
                                                            view! {
                                                                <Icon name=IconName::Refresh size=16 />
                                                                "Sync Now"
                                                            }.into_any()
                                                        }}
                                                    </button>
                                                </div>
                                            </div>
                                        }.into_any()
                                    }
                                    Err(_) => view! {
                                        <div class="text-slate-400 text-sm">"Unable to load earnings"</div>
                                    }.into_any()
                                }
                            })
                        }}
                    </Suspense>
                </div>

                // License Settings Tab
                <div class=move || if active_tab.get() == 1 { "block" } else { "hidden" }>
                    <Suspense fallback=move || view! {
                        <div class="flex items-center justify-center py-4">
                            <ProgressSpinner size=SpinnerSize::Small />
                        </div>
                    }>
                        {move || {
                            settings_resource.get().map(|result| {
                                match result {
                                    Ok(settings) => {
                                        view! {
                                            <div class="py-1">
                                                <div class="grid grid-cols-3 gap-4">
                                                    <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl py-6 px-4 text-center border border-slate-200 dark:border-slate-600">
                                                        <div class="text-slate-500 dark:text-slate-400 text-xs font-medium mb-2">"Share %"</div>
                                                        <div class="font-bold text-xl text-slate-900 dark:text-white">{format!("{}%", settings.lease_default_share_percentage)}</div>
                                                    </div>
                                                    <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl py-6 px-4 text-center border border-slate-200 dark:border-slate-600">
                                                        <div class="text-slate-500 dark:text-slate-400 text-xs font-medium mb-2">"Min Uptime"</div>
                                                        <div class="font-bold text-xl text-slate-900 dark:text-white">{format!("{}%", settings.lease_default_min_uptime_percentage)}</div>
                                                    </div>
                                                    <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl py-6 px-4 text-center border border-slate-200 dark:border-slate-600">
                                                        <div class="text-slate-500 dark:text-slate-400 text-xs font-medium mb-2">"Marketplace"</div>
                                                        <span class={format!(
                                                            "inline-block px-3 py-1 rounded-full text-sm font-semibold {}",
                                                            if settings.is_in_marketplace { "bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400" } else { "bg-slate-200 dark:bg-slate-600 text-slate-600 dark:text-slate-300" }
                                                        )}>
                                                            {if settings.is_in_marketplace { "Listed" } else { "Not Listed" }}
                                                        </span>
                                                    </div>
                                                </div>
                                                // Action buttons
                                                <div class="mt-4 p-4 rounded-lg border border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800/50">
                                                    <div class="flex items-center justify-end gap-3">
                                                        <button class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-lg border border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 transition">
                                                            <Icon name=IconName::Refresh size=16 />
                                                            "Reset"
                                                        </button>
                                                        <button class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-lg bg-cyan-500 text-white hover:bg-cyan-600 transition shadow-sm">
                                                            <Icon name=IconName::Edit size=16 />
                                                            "Edit"
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        }.into_any()
                                    }
                                    Err(_) => view! {
                                        <div class="text-slate-400 text-sm">"Unable to load settings"</div>
                                    }.into_any()
                                }
                            })
                        }}
                    </Suspense>
                </div>

                // Lease Details Tab - uses license data directly
                <div class=move || if active_tab.get() == 2 { "block" } else { "hidden" }>
                    {if license.is_leased || license.lease_from.is_some() {
                        let start_date = license.lease_from.as_ref().map(|s| {
                            let date_part = s.split('T').next().unwrap_or(s);
                            if date_part.len() >= 10 {
                                format!("{}/{}/{}", &date_part[8..10], &date_part[5..7], &date_part[0..4])
                            } else {
                                date_part.to_string()
                            }
                        }).unwrap_or_else(|| "-".to_string());
                        let end_date = license.lease_to.as_ref().map(|s| {
                            let date_part = s.split('T').next().unwrap_or(s);
                            if date_part.len() >= 10 {
                                format!("{}/{}/{}", &date_part[8..10], &date_part[5..7], &date_part[0..4])
                            } else {
                                date_part.to_string()
                            }
                        }).unwrap_or_else(|| "-".to_string());
                        view! {
                            <div class="py-1">
                                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                                    <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl py-6 px-4 text-center border border-slate-200 dark:border-slate-600">
                                        <div class="text-slate-500 dark:text-slate-400 text-xs font-medium mb-2">"Share"</div>
                                        <div class="font-bold text-xl text-slate-900 dark:text-white">{format!("{}%", license.lease_share_percentage)}</div>
                                    </div>
                                    <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl py-6 px-4 text-center border border-slate-200 dark:border-slate-600">
                                        <div class="text-slate-500 dark:text-slate-400 text-xs font-medium mb-2">"Min Uptime"</div>
                                        <div class="font-bold text-xl text-slate-900 dark:text-white">{format!("{}%", license.lease_min_uptime_percentage)}</div>
                                    </div>
                                    <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl py-6 px-4 text-center border border-slate-200 dark:border-slate-600">
                                        <div class="text-slate-500 dark:text-slate-400 text-xs font-medium mb-2">"Start Date"</div>
                                        <div class="font-bold text-lg text-slate-900 dark:text-white">{start_date}</div>
                                    </div>
                                    <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl py-6 px-4 text-center border border-slate-200 dark:border-slate-600">
                                        <div class="text-slate-500 dark:text-slate-400 text-xs font-medium mb-2">"End Date"</div>
                                        <div class="font-bold text-lg text-slate-900 dark:text-white">{end_date}</div>
                                    </div>
                                </div>
                                // Action buttons
                                <div class="mt-4 p-4 rounded-lg border border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800/50">
                                    <div class="flex items-center justify-end gap-3">
                                        <button class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-lg border border-red-300 dark:border-red-600 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition">
                                            <Icon name=IconName::Trash size=16 />
                                            "Terminate"
                                        </button>
                                        <button class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-lg bg-cyan-500 text-white hover:bg-cyan-600 transition shadow-sm">
                                            <Icon name=IconName::Edit size=16 />
                                            "Edit"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="text-center py-8">
                                <div class="text-slate-400 dark:text-slate-500">"No active lease"</div>
                            </div>
                        }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}

/// Charts and Rewards tabbed section
#[component]
fn ChartsTabsSection(
    analytics_resource: Resource<Result<Vec<UptimeAnalyticsPoint>, ServerFnError>>,
    daily_rewards_resource: Resource<Result<Vec<DailyRewardDto>, ServerFnError>>,
    rewards_resource: Resource<Result<PaginatedResult<RewardEntity>, ServerFnError>>,
    set_page_size: WriteSignal<u32>,
    set_current_page: WriteSignal<u32>,
) -> impl IntoView {
    let (chart_tab, set_chart_tab) = signal(0u8);

    view! {
        <div class="mx-4 my-2 bg-white dark:bg-slate-800 rounded-2xl border border-slate-200 dark:border-slate-700 shadow-sm overflow-hidden">
            // Tab headers - cyan background with white text for active
            <div class="flex border-b border-slate-200 dark:border-slate-700">
                <button
                    class=move || format!(
                        "flex-1 px-4 py-3 text-sm font-medium transition-all {}",
                        if chart_tab.get() == 0 {
                            "bg-cyan-500 text-white"
                        } else {
                            "text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:bg-slate-50 dark:hover:bg-slate-700/50"
                        }
                    )
                    on:click=move |_| set_chart_tab.set(0)
                >
                    "Daily Earnings"
                </button>
                <button
                    class=move || format!(
                        "flex-1 px-4 py-3 text-sm font-medium transition-all border-l border-slate-200 dark:border-slate-700 {}",
                        if chart_tab.get() == 1 {
                            "bg-cyan-500 text-white"
                        } else {
                            "text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:bg-slate-50 dark:hover:bg-slate-700/50"
                        }
                    )
                    on:click=move |_| set_chart_tab.set(1)
                >
                    "Uptime History"
                </button>
            </div>

            // Tab content
            <div class="p-6">
                // Daily Earnings Tab (includes chart + rewards table)
                <div class=move || if chart_tab.get() == 0 { "block" } else { "hidden" }>
                    // Daily Earnings Chart
                    <div class="mb-6">
                        <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">"Daily Earnings (Last 30 Days)"</h3>
                        <Suspense fallback=move || view! {
                            <div class="h-40 flex items-center justify-center">
                                <div class="w-5 h-5 border-2 border-[#22d3ee] border-t-transparent rounded-full animate-spin"></div>
                            </div>
                        }>
                            {move || {
                                daily_rewards_resource.get().map(|result| {
                                    match result {
                                        Ok(data) if !data.is_empty() => {
                                            view! { <DailyEarningsChart daily_rewards=data /> }.into_any()
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

                    // Rewards Table
                    <div class="border-t border-slate-200 dark:border-slate-700 pt-6">
                        <div class="flex items-center justify-between mb-4">
                            <h3 class="text-lg font-semibold text-slate-900 dark:text-white">"Reward Allocations"</h3>
                            <div class="flex items-center gap-3">
                                <span class="text-sm text-slate-500 dark:text-slate-400">"Show:"</span>
                                <select
                                    class="px-4 py-2 text-sm rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-slate-900 dark:text-white"
                                    on:change=move |ev| {
                                        let value: u32 = event_target_value(&ev).parse().unwrap_or(20);
                                        set_page_size.set(value);
                                        set_current_page.set(1);
                                    }
                                >
                                    <option value="5" selected>"5"</option>
                                    <option value="10">"10"</option>
                                    <option value="20">"20"</option>
                                    <option value="50">"50"</option>
                                </select>
                            </div>
                        </div>

                        <Suspense fallback=move || view! {
                            <div class="h-32 flex items-center justify-center">
                                <div class="w-5 h-5 border-2 border-[#22d3ee] border-t-transparent rounded-full animate-spin"></div>
                            </div>
                        }>
                            {move || {
                                rewards_resource.get().map(|result| {
                                    match result {
                                        Ok(paginated) if !paginated.items.is_empty() => {
                                            view! {
                                                <RewardsTable
                                                    rewards=paginated.items.clone()
                                                    total=paginated.total
                                                    page=paginated.page
                                                    total_pages=paginated.total_pages
                                                    set_page=set_current_page
                                                />
                                            }.into_any()
                                        }
                                        Ok(_) => {
                                            view! {
                                                <div class="py-8 text-center text-slate-400 text-sm">
                                                    "No rewards found for this license"
                                                </div>
                                            }.into_any()
                                        }
                                        Err(e) => {
                                            view! {
                                                <div class="py-8 text-center text-red-500 text-sm">
                                                    {format!("Error loading rewards: {}", e)}
                                                </div>
                                            }.into_any()
                                        }
                                    }
                                })
                            }}
                        </Suspense>
                    </div>
                </div>

                // Uptime History Tab
                <div class=move || if chart_tab.get() == 1 { "block" } else { "hidden" }>
                    // Uptime Chart
                    <div class="mb-6">
                        <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">"Uptime History (Last 30 Days)"</h3>
                        <Suspense fallback=move || view! {
                            <div class="h-40 flex items-center justify-center">
                                <div class="w-5 h-5 border-2 border-[#22d3ee] border-t-transparent rounded-full animate-spin"></div>
                            </div>
                        }>
                            {move || {
                                analytics_resource.get().map(|result| {
                                    match result {
                                        Ok(data) if !data.is_empty() => {
                                            view! { <UptimeChart data=data /> }.into_any()
                                        }
                                        _ => {
                                            view! {
                                                <div class="h-40 flex items-center justify-center text-slate-400 text-sm">
                                                    "No uptime data available"
                                                </div>
                                            }.into_any()
                                        }
                                    }
                                })
                            }}
                        </Suspense>
                    </div>

                    // Uptime Table
                    <div class="border-t border-slate-200 dark:border-slate-700 pt-6">
                        <Suspense fallback=move || view! {
                            <div class="h-32 flex items-center justify-center">
                                <div class="w-5 h-5 border-2 border-[#22d3ee] border-t-transparent rounded-full animate-spin"></div>
                            </div>
                        }>
                            {move || {
                                analytics_resource.get().map(|result| {
                                    match result {
                                        Ok(data) if !data.is_empty() => {
                                            view! { <UptimeTable data=data /> }.into_any()
                                        }
                                        _ => {
                                            view! {
                                                <div class="py-8 text-center text-slate-400 text-sm">
                                                    "No uptime data available"
                                                </div>
                                            }.into_any()
                                        }
                                    }
                                })
                            }}
                        </Suspense>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Uptime history chart component
#[component]
fn UptimeChart(data: Vec<UptimeAnalyticsPoint>) -> impl IntoView {
    if data.is_empty() {
        return view! {
            <div class="h-40 flex items-center justify-center text-slate-400 text-sm">
                "No uptime data available"
            </div>
        }.into_any();
    }

    // Sort data chronologically (oldest first) so oldest date is on the left
    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.date.cmp(&b.date));

    // Filter to show only the last 30 days of data on the chart
    let chart_data: Vec<(String, f64)> = {
        let total_len = sorted_data.len();
        let start_idx = if total_len > 30 { total_len - 30 } else { 0 };
        sorted_data[start_idx..].iter()
            .map(|p| (p.date.clone(), p.uptime_percentage()))
            .collect()
    };

    let bar_count = chart_data.len();

    // Chart dimensions - balanced proportions
    let chart_height: f64 = 320.0;
    let chart_width: f64 = 800.0;
    let padding_left: f64 = 45.0;
    let padding_right: f64 = 15.0;
    let padding_top: f64 = 15.0;
    let padding_bottom: f64 = 70.0;
    let inner_width = chart_width - padding_left - padding_right;
    let inner_height = chart_height - padding_top - padding_bottom;
    let bar_gap: f64 = 3.0;
    let bar_width = if bar_count > 0 {
        ((inner_width - (bar_count as f64 - 1.0) * bar_gap) / bar_count as f64).min(25.0)
    } else {
        20.0
    };

    view! {
        <div class="w-full text-slate-600 dark:text-slate-300">
            <svg
                viewBox={format!("0 0 {} {}", chart_width, chart_height)}
                class="w-full h-72"
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

                // Reference line at 100%
                <line
                    x1={padding_left}
                    y1={padding_top}
                    x2={chart_width - padding_right}
                    y2={padding_top}
                    stroke="#22c55e"
                    stroke-opacity="0.3"
                    stroke-width="1"
                    stroke-dasharray="4,4"
                />

                // Y-axis labels
                {(0..=4).map(|i| {
                    let y = padding_top + (i as f64 / 4.0) * inner_height;
                    let val = 100.0 - (i as f64 / 4.0) * 100.0;
                    view! {
                        <text
                            x={padding_left - 8.0}
                            y={y + 4.0}
                            text-anchor="end"
                            fill="currentColor"
                            style="font-size: 14px;"
                        >
                            {format!("{}%", val as i32)}
                        </text>
                    }
                }).collect::<Vec<_>>()}

                // Bars
                {chart_data.iter().enumerate().map(|(idx, (date, uptime))| {
                    let bar_height = (*uptime / 100.0) * inner_height;
                    let x = padding_left + idx as f64 * (bar_width + bar_gap);
                    let y = padding_top + inner_height - bar_height;

                    // Color based on uptime
                    let color = if *uptime >= 90.0 { "#22c55e" }
                        else if *uptime >= 70.0 { "#f59e0b" }
                        else { "#ef4444" };

                    // Date label in DD/MM/YYYY format
                    let formatted_date = if date.len() >= 10 {
                        format!("{}/{}/{}", &date[8..10], &date[5..7], &date[0..4])
                    } else {
                        date.clone()
                    };

                    let label_x = x + bar_width / 2.0;
                    let label_y = padding_top + inner_height + 8.0;

                    view! {
                        <g>
                            <rect
                                x={x}
                                y={y}
                                width={bar_width.max(4.0)}
                                height={bar_height.max(2.0)}
                                fill={color}
                                class="hover:opacity-80 transition-opacity cursor-pointer"
                            >
                                <title>{format!("{}: {:.1}%", date, uptime)}</title>
                            </rect>
                            // X-axis label - every bar, vertical, DD/MM/YYYY format
                            <text
                                x={label_x}
                                y={label_y}
                                text-anchor="start"
                                transform={format!("rotate(90 {} {})", label_x, label_y)}
                                fill="currentColor"
                                style="font-size: 12px;"
                            >
                                {formatted_date}
                            </text>
                        </g>
                    }
                }).collect::<Vec<_>>()}
            </svg>
        </div>
    }.into_any()
}

/// Uptime table component showing daily uptime data with pagination
#[component]
fn UptimeTable(data: Vec<UptimeAnalyticsPoint>) -> impl IntoView {
    // Sort data by date descending (most recent first)
    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| b.date.cmp(&a.date));

    // Pagination state
    let (current_page, set_current_page) = signal(1u32);
    let (page_size, set_page_size) = signal(10u32);

    let total_items = sorted_data.len();

    view! {
        <div>
            // Header with page size selector
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-semibold text-slate-900 dark:text-white">"Uptime Details"</h3>
                <div class="flex items-center gap-3">
                    <span class="text-sm text-slate-500 dark:text-slate-400">"Show:"</span>
                    <select
                        class="px-4 py-2 text-sm rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-slate-900 dark:text-white"
                        on:change=move |ev| {
                            let value: u32 = event_target_value(&ev).parse().unwrap_or(5);
                            set_page_size.set(value);
                            set_current_page.set(1);
                        }
                    >
                        <option value="5" selected>"5"</option>
                        <option value="10">"10"</option>
                        <option value="20">"20"</option>
                        <option value="50">"50"</option>
                    </select>
                </div>
            </div>

            // Table with rounded container
            <div class="overflow-hidden rounded-lg border border-slate-200 dark:border-slate-700">
                <table class="w-full text-sm">
                    <thead>
                        <tr class="bg-slate-50 dark:bg-slate-700/50 text-left">
                            <th class="px-4 py-2.5 font-semibold text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">"Date"</th>
                            <th class="px-4 py-2.5 font-semibold text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">"Uptime %"</th>
                            <th class="px-4 py-2.5 font-semibold text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">"Online (est.)"</th>
                            <th class="px-4 py-2.5 font-semibold text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">"Offline (est.)"</th>
                            <th class="px-4 py-2.5 font-semibold text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">"Status"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-100 dark:divide-slate-700">
                        {move || {
                            let page = current_page.get() as usize;
                            let size = page_size.get() as usize;
                            let start = (page - 1) * size;
                            let end = (start + size).min(total_items);

                            sorted_data[start..end].iter().map(|point| {
                                let uptime = point.uptime_percentage();
                                let formatted_date = if point.date.len() >= 10 {
                                    format!("{}/{}/{}", &point.date[8..10], &point.date[5..7], &point.date[0..4])
                                } else {
                                    point.date.clone()
                                };

                                // Calculate estimated hours based on uptime percentage (24h day)
                                let online_hours = (uptime / 100.0) * 24.0;
                                let offline_hours = 24.0 - online_hours;

                                let (status_text, status_class) = if uptime >= 95.0 {
                                    ("Excellent", "bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400")
                                } else if uptime >= 90.0 {
                                    ("Good", "bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-400")
                                } else if uptime >= 70.0 {
                                    ("Fair", "bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400")
                                } else {
                                    ("Poor", "bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400")
                                };

                                view! {
                                    <tr class="hover:bg-slate-50 dark:hover:bg-slate-700/30 transition">
                                        <td class="px-4 py-2.5 text-slate-900 dark:text-white font-medium">{formatted_date}</td>
                                        <td class="px-4 py-2.5">
                                            <span class={format!(
                                                "font-semibold {}",
                                                if uptime >= 90.0 { "text-green-600 dark:text-green-400" }
                                                else if uptime >= 70.0 { "text-amber-600 dark:text-amber-400" }
                                                else { "text-red-600 dark:text-red-400" }
                                            )}>
                                                {format!("{:.1}%", uptime)}
                                            </span>
                                        </td>
                                        <td class="px-4 py-2.5 text-slate-600 dark:text-slate-300">{format!("{:.1}h", online_hours)}</td>
                                        <td class="px-4 py-2.5 text-slate-600 dark:text-slate-300">{format!("{:.1}h", offline_hours)}</td>
                                        <td class="px-4 py-2.5">
                                            <span class={format!("px-2 py-0.5 rounded-full text-xs font-medium {}", status_class)}>
                                                {status_text}
                                            </span>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </tbody>
                </table>
            </div>

            // Pagination controls
            <div class="mt-4 p-4 rounded-lg border border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800/50">
                <div class="flex items-center justify-between">
                    <div class="text-sm text-slate-500 dark:text-slate-400">
                        {move || {
                            let size = page_size.get() as usize;
                            let total_pages = (total_items + size - 1) / size;
                            format!("Page {} of {} ({} total)", current_page.get(), total_pages.max(1), total_items)
                        }}
                    </div>
                    <div class="flex items-center gap-2">
                        <button
                            class="px-4 py-2 text-sm font-medium rounded-lg bg-cyan-500 text-white hover:bg-cyan-600 disabled:opacity-40 disabled:cursor-not-allowed transition shadow-sm"
                            disabled=move || current_page.get() <= 1
                            on:click=move |_| {
                                if current_page.get() > 1 {
                                    set_current_page.set(current_page.get() - 1);
                                }
                            }
                        >
                            "Back"
                        </button>
                        <button
                            class="px-4 py-2 text-sm font-medium rounded-lg bg-cyan-500 text-white hover:bg-cyan-600 disabled:opacity-40 disabled:cursor-not-allowed transition shadow-sm"
                            disabled=move || {
                                let size = page_size.get() as usize;
                                let total_pages = (total_items + size - 1) / size;
                                current_page.get() as usize >= total_pages
                            }
                            on:click=move |_| {
                                let size = page_size.get() as usize;
                                let total_pages = (total_items + size - 1) / size;
                                if (current_page.get() as usize) < total_pages {
                                    set_current_page.set(current_page.get() + 1);
                                }
                            }
                        >
                            "Next"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Daily earnings chart component
#[component]
fn DailyEarningsChart(daily_rewards: Vec<DailyRewardDto>) -> impl IntoView {
    use std::collections::HashMap;
    use chrono::NaiveDate;

    // Build a map of date -> amount and find the latest date
    let mut latest_date: Option<NaiveDate> = None;
    let rewards_map: HashMap<String, f64> = daily_rewards.into_iter()
        .map(|r| {
            // Track the latest date
            if let Ok(date) = NaiveDate::parse_from_str(&r.date, "%Y-%m-%d") {
                if latest_date.is_none() || date > latest_date.unwrap() {
                    latest_date = Some(date);
                }
            }
            (r.date, r.total_amount_micros as f64 / 1_000_000.0)
        })
        .collect();

    // Use latest reward date as end date, or today if no rewards
    let end_date = latest_date.unwrap_or_else(|| Utc::now().date_naive());

    // Generate 30 days ending at the latest reward date
    let mut chart_data: Vec<(String, f64)> = Vec::with_capacity(30);
    for i in (0..30).rev() {
        let date = end_date - chrono::Duration::days(i);
        let date_str = date.format("%Y-%m-%d").to_string();
        let amount = rewards_map.get(&date_str).copied().unwrap_or(0.0);
        chart_data.push((date_str, amount));
    }

    let max_value = chart_data.iter().map(|(_, v)| *v).fold(0.0f64, f64::max);
    let scale_max = if max_value > 0.0 { max_value * 1.1 } else { 1.0 };
    let bar_count = chart_data.len(); // Always 30

    // Chart dimensions - balanced proportions
    let chart_height: f64 = 320.0;
    let chart_width: f64 = 800.0;
    let padding_left: f64 = 50.0;
    let padding_right: f64 = 15.0;
    let padding_top: f64 = 15.0;
    let padding_bottom: f64 = 70.0;
    let inner_width = chart_width - padding_left - padding_right;
    let inner_height = chart_height - padding_top - padding_bottom;
    let bar_gap: f64 = 2.0;
    let bar_width = (inner_width - (bar_count as f64 - 1.0) * bar_gap) / bar_count as f64;

    view! {
        <div class="w-full text-slate-600 dark:text-slate-300">
            <svg
                viewBox={format!("0 0 {} {}", chart_width, chart_height)}
                class="w-full h-72"
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

                // Y-axis labels
                {(0..=4).map(|i| {
                    let y = padding_top + (i as f64 / 4.0) * inner_height;
                    let val = scale_max * (1.0 - i as f64 / 4.0);
                    view! {
                        <text
                            x={padding_left - 8.0}
                            y={y + 4.0}
                            text-anchor="end"
                            fill="currentColor"
                            style="font-size: 14px;"
                        >
                            {format!("${:.2}", val)}
                        </text>
                    }
                }).collect::<Vec<_>>()}

                // Y-axis label
                <text
                    x="15"
                    y={padding_top + inner_height / 2.0}
                    text-anchor="middle"
                    transform={format!("rotate(-90 15 {})", padding_top + inner_height / 2.0)}
                    fill="#22c55e"
                    style="font-size: 14px; font-weight: 600;"
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

                    // Short date: just day number
                    let day = if date.len() >= 10 { &date[8..10] } else { date };

                    view! {
                        <g>
                            <rect
                                x={x}
                                y={y}
                                width={bar_width.max(4.0)}
                                height={bar_height.max(1.0)}
                                fill="#22c55e"
                                class="hover:fill-green-400 transition-colors cursor-pointer"
                            >
                                <title>{format!("{}: ${:.4}", date, value)}</title>
                            </rect>
                            // X-axis label - every bar, vertical, DD/MM/YYYY format
                            {
                                let short_date = if date.len() >= 10 {
                                    // Convert YYYY-MM-DD to DD/MM/YYYY
                                    format!("{}/{}/{}", &date[8..10], &date[5..7], &date[0..4])
                                } else {
                                    date.clone()
                                };
                                let label_x = x + bar_width / 2.0;
                                let label_y = padding_top + inner_height + 8.0;
                                view! {
                                    <text
                                        x={label_x}
                                        y={label_y}
                                        text-anchor="start"
                                        transform={format!("rotate(90 {} {})", label_x, label_y)}
                                        fill="currentColor"
                                        style="font-size: 12px;"
                                    >
                                        {short_date}
                                    </text>
                                }
                            }
                        </g>
                    }
                }).collect::<Vec<_>>()}
            </svg>
        </div>
    }.into_any()
}

/// Rewards table with pagination
#[component]
fn RewardsTable(
    rewards: Vec<RewardEntity>,
    total: i64,
    page: u32,
    total_pages: u32,
    set_page: WriteSignal<u32>,
) -> impl IntoView {
    view! {
        <div>
            // Table with rounded container
            <div class="overflow-hidden rounded-lg border border-slate-200 dark:border-slate-700">
                <table class="w-full text-sm">
                    <thead>
                        <tr class="bg-slate-50 dark:bg-slate-700/50 text-left">
                            <th class="py-2.5 px-4 font-semibold text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">"Date"</th>
                            <th class="py-2.5 px-4 font-semibold text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">"Type"</th>
                            <th class="py-2.5 px-4 font-semibold text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">"Task"</th>
                            <th class="py-2.5 px-4 font-semibold text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400 text-right">"Amount"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-100 dark:divide-slate-700">
                        {rewards.into_iter().map(|reward| {
                            // Convert YYYY-MM-DD to DD/MM/YYYY
                            let raw_date = reward.completed_at.split('T').next().unwrap_or(&reward.completed_at);
                            let date = if raw_date.len() >= 10 {
                                format!("{}/{}/{}", &raw_date[8..10], &raw_date[5..7], &raw_date[0..4])
                            } else {
                                raw_date.to_string()
                            };
                            view! {
                                <tr class="hover:bg-slate-50 dark:hover:bg-slate-700/30 transition">
                                    <td class="py-2.5 px-4 text-slate-900 dark:text-white font-medium">{date}</td>
                                    <td class="py-2.5 px-4">
                                        <span class="px-2 py-0.5 rounded text-xs font-medium bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-300">
                                            {reward.reward_type.clone()}
                                        </span>
                                    </td>
                                    <td class="py-2.5 px-4 text-slate-600 dark:text-slate-400">{reward.task_key.clone().unwrap_or_else(|| "-".to_string())}</td>
                                    <td class="py-2.5 px-4 text-right">
                                        <div class="inline-flex items-center gap-1">
                                            <UsdCoinIcon size=16 />
                                            <span class="font-semibold text-green-400">
                                                {format!("{:.4}", reward.amount())}
                                            </span>
                                        </div>
                                    </td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>

            // Pagination
            {if total_pages > 1 {
                let is_first_page = page <= 1;
                let is_last_page = page >= total_pages;
                view! {
                    <div class="mt-4 p-4 rounded-lg border border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800/50">
                        <div class="flex items-center justify-between">
                            <span class="text-sm text-slate-500 dark:text-slate-400">
                                {format!("Page {} of {} ({} total)", page, total_pages, total)}
                            </span>
                            <div class="flex items-center gap-2">
                                <button
                                    class="px-4 py-2 text-sm font-medium rounded-lg bg-cyan-500 text-white hover:bg-cyan-600 disabled:opacity-40 disabled:cursor-not-allowed transition shadow-sm"
                                    disabled=is_first_page
                                    on:click=move |_| set_page.update(|p| if *p > 1 { *p -= 1 })
                                >
                                    "Back"
                                </button>
                                <button
                                class="px-4 py-2 text-sm font-medium rounded-lg bg-cyan-500 text-white hover:bg-cyan-600 disabled:opacity-40 disabled:cursor-not-allowed transition shadow-sm"
                                disabled=is_last_page
                                on:click=move |_| set_page.update(|p| *p += 1)
                            >
                                "Next"
                            </button>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}
