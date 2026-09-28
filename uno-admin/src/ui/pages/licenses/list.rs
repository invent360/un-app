//! License list page - fetches directly from Unetwork API with server-side pagination

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::JsCast;
use std::collections::{HashSet, HashMap};
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::components::common::progress_spinner::{ProgressSpinner, SpinnerSize};
use crate::handler::{
    get_paginated_licenses, get_uno_licenses_summary, get_allocations_summary,
    search_license_by_id, LicenseDto, list_agents, bulk_save_license_settings,
    BulkSaveSettingsRequest, get_batch_uptime_history,
};
use crate::components::charts::MiniUptimeChart;
use leptos::task::spawn_local;
use crate::components::common::UsdCoinIcon;

/// View mode for license display
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    Grid,
    #[default]
    Table,
}

/// Status filter options
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusFilter {
    #[default]
    All,
    Online,
    Offline,
}

/// Bound filter options
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum BoundFilter {
    #[default]
    All,
    Bound,
    NotBound,
}

/// Leased filter options
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum LeasedFilter {
    #[default]
    All,
    Leased,
    NotLeased,
}

/// Marketplace filter options
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MarketplaceFilter {
    #[default]
    All,
    Uno,
    Unetwork,
}

/// PrimeReact-style range slider with document-level drag for smooth UX
#[component]
fn PrimeSlider(
    value: RwSignal<(f64, f64)>,
    #[prop(optional)] on_change_complete: Option<Callback<(f64, f64)>>,
) -> impl IntoView {
    let slider_ref = NodeRef::<leptos::html::Div>::new();
    let dragging_handle = RwSignal::new(Option::<usize>::None);

    // Calculate percentage from mouse position
    let calc_percent = move |client_x: i32| -> f64 {
        if let Some(el) = slider_ref.get() {
            let rect = el.get_bounding_client_rect();
            let x = client_x as f64 - rect.left();
            (x / rect.width() * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        }
    };

    // Update value during drag
    let update_value = move |client_x: i32| {
        if let Some(idx) = dragging_handle.get_untracked() {
            let pct = calc_percent(client_x);
            let (low, high) = value.get_untracked();

            if idx == 0 {
                value.set((pct.min(high), high));
            } else {
                value.set((low, pct.max(low)));
            }
        }
    };

    // Handle mousedown on a handle
    let on_handle_mousedown = move |idx: usize, ev: web_sys::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        dragging_handle.set(Some(idx));

        // Attach document-level listeners for smooth drag
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                // Clone for closures
                let on_change_complete = on_change_complete.clone();

                // Mousemove handler
                let move_cb = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
                    update_value(ev.client_x());
                });

                // Mouseup handler - cleanup and fire complete callback
                let dragging_handle_clone = dragging_handle;
                let value_clone = value;
                let up_cb = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_ev: web_sys::MouseEvent| {
                    dragging_handle_clone.set(None);
                    if let Some(ref cb) = on_change_complete {
                        cb.run(value_clone.get());
                    }
                });

                let _ = doc.add_event_listener_with_callback("mousemove", move_cb.as_ref().unchecked_ref());
                let _ = doc.add_event_listener_with_callback("mouseup", up_cb.as_ref().unchecked_ref());

                // Keep closures alive until mouseup
                move_cb.forget();
                up_cb.forget();
            }
        }
    };

    // Click on track moves nearest handle
    let on_track_click = move |ev: web_sys::MouseEvent| {
        if dragging_handle.get_untracked().is_some() {
            return;
        }

        let pct = calc_percent(ev.client_x());
        let (low, high) = value.get_untracked();

        // Move nearest handle
        if (pct - low).abs() <= (pct - high).abs() {
            value.set((pct, high));
        } else {
            value.set((low, pct));
        }

        if let Some(ref cb) = on_change_complete {
            cb.run(value.get());
        }
    };

    view! {
        <div
            node_ref=slider_ref
            style="width: 140px; height: 20px; position: relative; cursor: pointer;"
            on:click=on_track_click
        >
            // Background track (gray)
            <div style="position: absolute; left: 0; right: 0; top: 50%; transform: translateY(-50%); height: 4px; background: rgba(255,255,255,0.2); border-radius: 9999px;"></div>

            // Filled range (cyan)
            <div
                style=move || {
                    let (low, high) = value.get();
                    format!(
                        "position: absolute; top: 50%; transform: translateY(-50%); height: 4px; background: #06b6d4; border-radius: 9999px; left: {}%; width: {}%;",
                        low, high - low
                    )
                }
            ></div>

            // Min handle
            <div
                style=move || {
                    let (low, _) = value.get();
                    format!(
                        "position: absolute; top: 50%; left: {}%; width: 16px; height: 16px; margin-left: -8px; margin-top: -8px; background: #0f172a; border: 2px solid #06b6d4; border-radius: 50%; cursor: grab; z-index: 1;",
                        low
                    )
                }
                on:mousedown=move |ev| on_handle_mousedown(0, ev)
            ></div>

            // Max handle
            <div
                style=move || {
                    let (_, high) = value.get();
                    format!(
                        "position: absolute; top: 50%; left: {}%; width: 16px; height: 16px; margin-left: -8px; margin-top: -8px; background: #0f172a; border: 2px solid #06b6d4; border-radius: 50%; cursor: grab; z-index: 1;",
                        high
                    )
                }
                on:mousedown=move |ev| on_handle_mousedown(1, ev)
            ></div>
        </div>
    }
}

/// Single-value slider for edit modals (based on PrimeSlider pattern)
#[component]
fn SingleValueSlider(
    value: RwSignal<f64>,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(optional)] on_change_complete: Option<Callback<f64>>,
) -> impl IntoView {
    let slider_ref = NodeRef::<leptos::html::Div>::new();
    let dragging = RwSignal::new(false);

    // Convert value to percentage for positioning
    let value_to_percent = move |val: f64| -> f64 {
        ((val - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    };

    // Convert percentage to value
    let percent_to_value = move |pct: f64| -> f64 {
        min + (pct / 100.0) * (max - min)
    };

    // Calculate percentage from mouse position
    let calc_percent = move |client_x: i32| -> f64 {
        if let Some(el) = slider_ref.get() {
            let rect = el.get_bounding_client_rect();
            let x = client_x as f64 - rect.left();
            (x / rect.width() * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        }
    };

    // Update value during drag
    let update_value = move |client_x: i32| {
        if dragging.get_untracked() {
            let pct = calc_percent(client_x);
            value.set(percent_to_value(pct));
        }
    };

    // Handle mousedown on handle
    let on_handle_mousedown = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        dragging.set(true);

        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                let on_change_complete = on_change_complete.clone();

                let move_cb = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
                    update_value(ev.client_x());
                });

                let dragging_clone = dragging;
                let value_clone = value;
                let up_cb = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_ev: web_sys::MouseEvent| {
                    dragging_clone.set(false);
                    if let Some(ref cb) = on_change_complete {
                        cb.run(value_clone.get());
                    }
                });

                let _ = doc.add_event_listener_with_callback("mousemove", move_cb.as_ref().unchecked_ref());
                let _ = doc.add_event_listener_with_callback("mouseup", up_cb.as_ref().unchecked_ref());

                move_cb.forget();
                up_cb.forget();
            }
        }
    };

    // Click on track moves handle
    let on_track_click = move |ev: web_sys::MouseEvent| {
        if dragging.get_untracked() {
            return;
        }
        let pct = calc_percent(ev.client_x());
        value.set(percent_to_value(pct));

        if let Some(ref cb) = on_change_complete {
            cb.run(value.get());
        }
    };

    view! {
        <div
            node_ref=slider_ref
            style="width: 100%; height: 20px; position: relative; cursor: pointer;"
            on:click=on_track_click
        >
            // Background track (gray)
            <div style="position: absolute; left: 0; right: 0; top: 50%; transform: translateY(-50%); height: 4px; background: rgba(255,255,255,0.2); border-radius: 9999px;"></div>

            // Filled portion (cyan)
            <div
                style=move || {
                    let pct = value_to_percent(value.get());
                    format!(
                        "position: absolute; top: 50%; transform: translateY(-50%); height: 4px; background: #06b6d4; border-radius: 9999px; left: 0; width: {}%;",
                        pct
                    )
                }
            ></div>

            // Handle
            <div
                style=move || {
                    let pct = value_to_percent(value.get());
                    format!(
                        "position: absolute; top: 50%; left: {}%; width: 16px; height: 16px; margin-left: -8px; margin-top: -8px; background: #0f172a; border: 2px solid #06b6d4; border-radius: 50%; cursor: grab; z-index: 1;",
                        pct
                    )
                }
                on:mousedown=on_handle_mousedown
            ></div>
        </div>
    }
}

/// Helper function to render the license list with pagination
fn render_license_list(
    page_licenses: Vec<LicenseDto>,
    display_total: usize,
    start_display: usize,
    end_display: usize,
    total_pages_val: i32,
    current_pg: usize,
    current_page: ReadSignal<i32>,
    set_current_page: WriteSignal<i32>,
    mode: ViewMode,
    on_click: impl Fn(String) + Clone + Send + Sync + 'static,
    selection_mode: ReadSignal<bool>,
    selected_ids: ReadSignal<HashSet<String>>,
    set_selected_ids: WriteSignal<HashSet<String>>,
    // Inline filter props
    license_filter: RwSignal<String>,
    marketplace_filter: RwSignal<MarketplaceFilter>,
    status_filter: ReadSignal<StatusFilter>,
    set_status_filter: WriteSignal<StatusFilter>,
    // Expanded rows tracking
    expanded_licenses: RwSignal<HashSet<String>>,
    // Uptime filter
    uptime_range: RwSignal<(f64, f64)>,
    // Loading state
    is_loading: bool,
) -> impl IntoView {
    // For Grid view with empty results, show empty state
    // For Table view, always show the table (it handles empty state internally)
    if page_licenses.is_empty() && mode == ViewMode::Grid && !is_loading {
        view! {
            <div class="text-center py-12">
                <div class="text-slate-400 dark:text-slate-500 mb-4">
                    <Icon name=IconName::Key size=48 class="mx-auto opacity-50".to_string() />
                </div>
                <p class="text-slate-500 dark:text-slate-400">
                    "No licenses found matching filters"
                </p>
            </div>
        }.into_any()
    } else {
        view! {
            <div style="display: flex; flex-direction: column; gap: 5px;">
                // View content
                {if mode == ViewMode::Grid {
                    if is_loading {
                        view! {
                            <div class="flex items-center justify-center py-12">
                                <ProgressSpinner size=SpinnerSize::Default />
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <LicenseGrid
                                licenses=page_licenses.clone()
                                on_click=on_click.clone()
                                selection_mode=selection_mode
                                selected_ids=selected_ids
                                set_selected_ids=set_selected_ids
                            />
                        }.into_any()
                    }
                } else {
                    view! {
                        <LicenseTable
                            licenses=page_licenses.clone()
                            on_click=on_click.clone()
                            selection_mode=selection_mode
                            selected_ids=selected_ids
                            set_selected_ids=set_selected_ids
                            license_filter=license_filter
                            marketplace_filter=marketplace_filter
                            status_filter=status_filter
                            set_status_filter=set_status_filter
                            expanded_licenses=expanded_licenses
                            uptime_range=uptime_range
                            is_loading=is_loading
                        />
                    }.into_any()
                }}

                // Pagination
                <div style="background: #0f172a; border: 1px solid #1e293b; border-radius: 12px; padding: 12px 16px; display: flex; align-items: center; justify-content: space-between;">
                    <span class="text-sm text-slate-400">
                        {format!("Showing {}-{} of {} licenses", start_display, end_display, display_total)}
                    </span>
                    <div style="display: flex; align-items: center; gap: 12px;">
                        <button
                            style={move || format!(
                                "display: flex; align-items: center; gap: 6px; padding: 8px 16px; border-radius: 8px; font-size: 14px; font-weight: 500; border: none; cursor: {}; transition: all 0.15s; {}",
                                if current_page.get() <= 1 { "not-allowed" } else { "pointer" },
                                if current_page.get() <= 1 {
                                    "background: #334155; color: #64748b;"
                                } else {
                                    "background: #0ea5e9; color: white;"
                                }
                            )}
                            disabled=move || current_page.get() <= 1
                            on:click=move |_| {
                                set_current_page.update(|p| if *p > 1 { *p -= 1 });
                            }
                        >
                            "Previous"
                        </button>
                        <span style="color: #94a3b8; font-size: 14px;">
                            {format!("{} / {}", current_pg, total_pages_val)}
                        </span>
                        <button
                            style={
                                let max_page = total_pages_val;
                                move || format!(
                                    "display: flex; align-items: center; gap: 6px; padding: 8px 16px; border-radius: 8px; font-size: 14px; font-weight: 500; border: none; cursor: {}; transition: all 0.15s; {}",
                                    if current_page.get() >= max_page { "not-allowed" } else { "pointer" },
                                    if current_page.get() >= max_page {
                                        "background: #334155; color: #64748b;"
                                    } else {
                                        "background: #0ea5e9; color: white;"
                                    }
                                )
                            }
                            disabled={
                                let max_page = total_pages_val;
                                move || current_page.get() >= max_page
                            }
                            on:click=move |_| {
                                set_current_page.update(|p| *p += 1);
                            }
                        >
                            "Next"
                        </button>
                    </div>
                </div>
            </div>
        }.into_any()
    }
}

/// License list page with server-side pagination
#[component]
pub fn LicenseListPage() -> impl IntoView {
    let navigate = use_navigate();

    // View mode state - default to Table
    let (view_mode, set_view_mode) = signal(ViewMode::Table);

    // Pagination state (1-indexed for API)
    let (current_page, set_current_page) = signal(1i32);
    let (page_size, set_page_size) = signal(5i32);

    // Filter state (actual filters that trigger API fetch)
    let uptime_range = RwSignal::new((0.0f64, 100.0f64));
    let (status_filter, set_status_filter) = signal(StatusFilter::All);
    let (bound_filter, set_bound_filter) = signal(BoundFilter::All);
    let (leased_filter, set_leased_filter) = signal(LeasedFilter::All);
    let (show_filter_modal, set_show_filter_modal) = signal(false);

    // Inline filter state (client-side filtering)
    let license_filter = RwSignal::new(String::new());
    let marketplace_filter = RwSignal::new(MarketplaceFilter::All);

    // Expanded rows tracking for +/- toggle
    let expanded_licenses = RwSignal::new(HashSet::<String>::new());

    // Temporary modal state (only applied when clicking "Apply")
    let (modal_status, set_modal_status) = signal(StatusFilter::All);
    let (modal_bound, set_modal_bound) = signal(BoundFilter::All);
    let (modal_leased, set_modal_leased) = signal(LeasedFilter::All);

    // === Bulk Selection State ===
    let (selection_mode, set_selection_mode) = signal(false);
    let (selected_ids, set_selected_ids) = signal(HashSet::<String>::new());

    // === Edit Modal State ===
    let (show_edit_modal, set_show_edit_modal) = signal(false);
    let edit_share_percentage = RwSignal::new(50.0f64);
    let edit_lease_duration = RwSignal::new(6.0f64);
    let edit_uptime_requirement = RwSignal::new(75.0f64);
    // Marketplace visibility - dual checkboxes
    let edit_uno_marketplace = RwSignal::new(true);
    let edit_unetwork_marketplace = RwSignal::new(false);
    // Referral assignment
    let enable_referral = RwSignal::new(false);
    let selected_referral = RwSignal::new(String::new());

    // Agents resource for referral dropdown
    let agents_resource = Resource::new(
        || (),
        |_| async move { list_agents().await }
    );

    // === Bulk Revoke Modal State ===
    let (show_bulk_revoke_modal, set_show_bulk_revoke_modal) = signal(false);

    // === License Search State ===
    let (search_query, set_search_query) = signal(String::new());
    let (searching, set_searching) = signal(false);
    let (search_error, set_search_error) = signal(Option::<String>::None);
    let (search_result, set_search_result) = signal(Option::<LicenseDto>::None);
    let (is_search_active, set_is_search_active) = signal(false);

    // Selection toggle handler
    let toggle_select = move |license_id: String| {
        set_selected_ids.update(|ids| {
            if ids.contains(&license_id) {
                ids.remove(&license_id);
            } else {
                ids.insert(license_id);
            }
        });
    };

    // Reset page to 1 when license filter changes (using previous value tracking)
    let prev_license_filter = RwSignal::new(String::new());
    Effect::new(move |_| {
        let current = license_filter.get();
        let prev = prev_license_filter.get_untracked();
        if current != prev {
            prev_license_filter.set(current);
            // Only reset page if not on page 1 already
            if current_page.get_untracked() != 1 {
                set_current_page.set(1);
            }
        }
    });

    // Fetch paginated licenses with all filters (server-side filtering)
    let paginated_resource = Resource::new(
        move || {
            let (uptime_min, uptime_max) = uptime_range.get();
            // Include license_filter for backend search
            let search = {
                let filter_val = license_filter.get();
                if filter_val.is_empty() {
                    None
                } else {
                    // Prepend "0x" if not already present
                    Some(if filter_val.starts_with("0x") {
                        filter_val
                    } else {
                        format!("0x{}", filter_val)
                    })
                }
            };
            (
                current_page.get(),
                page_size.get(),
                uptime_min,
                uptime_max,
                status_filter.get(),
                bound_filter.get(),
                leased_filter.get(),
                search,
            )
        },
        |(page, size, uptime_min, uptime_max, status, bound, leased, search)| async move {
            let min_filter = if uptime_min > 0.0 { Some(uptime_min / 100.0) } else { None };
            let max_filter = if uptime_max < 100.0 { Some(uptime_max / 100.0) } else { None };

            // Convert filter enums to API parameters
            let is_online = match status {
                StatusFilter::Online => Some(true),
                StatusFilter::Offline => Some(false),
                StatusFilter::All => None,
            };
            let is_bound = match bound {
                BoundFilter::Bound => Some(true),
                BoundFilter::NotBound => Some(false),
                BoundFilter::All => None,
            };
            let is_leased = match leased {
                LeasedFilter::Leased => Some(true),
                LeasedFilter::NotLeased => Some(false),
                LeasedFilter::All => None,
            };

            get_paginated_licenses(page, size, is_leased, is_bound, is_online, min_filter, max_filter, search).await
        }
    );

    // Track table loading state for showing overlay spinner during refetch
    // Use a derived signal based on resource state to avoid hydration issues
    let is_table_loading = Signal::derive(move || {
        // Loading when resource has no data yet
        paginated_resource.get().is_none()
    });

    // Track previous dependency values to detect changes and show brief loading
    let show_loading_overlay = RwSignal::new(false);
    let prev_deps = RwSignal::new((0i32, 0i32, (0.0f64, 100.0f64), String::new(), StatusFilter::All, BoundFilter::All, LeasedFilter::All, MarketplaceFilter::All));

    // Effect to detect dependency changes and trigger loading overlay (client-side only)
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let current_deps = (
            current_page.get(),
            page_size.get(),
            uptime_range.get(),
            license_filter.get(),
            status_filter.get(),
            bound_filter.get(),
            leased_filter.get(),
            marketplace_filter.get(),
        );
        let prev = prev_deps.get_untracked();
        if current_deps != prev {
            prev_deps.set(current_deps);
            show_loading_overlay.set(true);
            // For client-side filters, hide overlay after brief display
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(200).await;
                show_loading_overlay.set(false);
            });
        }
    });

    // Fetch summary from UNO licenses summary API (shows correct 2200 total)
    let summary_resource = Resource::new(
        || (),
        |_| async move { get_uno_licenses_summary().await }
    );

    // Fetch allocations summary for Total Earnings
    let allocations_resource = Resource::new(
        || (),
        |_| async move { get_allocations_summary().await }
    );

    // Derived signal for summary total (used in pagination display)
    let summary_total_signal = Signal::derive(move || {
        summary_resource.get()
            .and_then(|r| r.ok())
            .map(|s| s.total_licenses_count)
            .unwrap_or(0)
    });

    // Navigation handler
    let nav = navigate.clone();
    let on_license_click = move |license_id: String| {
        nav(&format!("/licenses/{}", license_id), Default::default());
    };

    view! {
        <div>
            <Header title="Licenses".to_string() show_search=false />

            <div class="px-4 py-4 space-y-3">
                // Summary cards - single row like dashboard
                <Suspense fallback=move || view! {
                    <div class="flex items-center justify-center py-8">
                        <ProgressSpinner size=SpinnerSize::Default />
                    </div>
                }>
                    {move || {
                        let lic_summary = summary_resource.get().and_then(|r| r.ok());
                        let alloc_summary = allocations_resource.get().and_then(|r| r.ok());

                        match (lic_summary, alloc_summary) {
                            (Some(lic), Some(alloc)) => view! {
                                <div class="flex flex-row gap-4 w-full">
                                    <div class="flex-1">
                                        <StatCard label="Total Licenses" value=lic.total_licenses_count.to_string() variant="default" />
                                    </div>
                                    <div class="flex-1">
                                        <StatCard label="Active" value=lic.active_licenses_count.to_string() variant="success" />
                                    </div>
                                    <div class="flex-1">
                                        <StatCard label="Leased" value=lic.leased_licenses_count.to_string() variant="default" />
                                    </div>
                                    <div class="flex-1">
                                        <StatCard label="Bound" value=lic.bound_licenses_count.to_string() variant="default" />
                                    </div>
                                    <div class="flex-1">
                                        <EarningsCard label="Total Earnings" amount=alloc.total_amount() />
                                    </div>
                                </div>
                            }.into_any(),
                            _ => view! {
                                <div class="flex flex-row gap-4 w-full">
                                    <div class="flex-1"><StatCard label="Total Licenses" value="--".to_string() variant="default" /></div>
                                    <div class="flex-1"><StatCard label="Active" value="--".to_string() variant="success" /></div>
                                    <div class="flex-1"><StatCard label="Leased" value="--".to_string() variant="default" /></div>
                                    <div class="flex-1"><StatCard label="Bound" value="--".to_string() variant="default" /></div>
                                    <div class="flex-1"><EarningsCard label="Total Earnings" amount=0.0 /></div>
                                </div>
                            }.into_any()
                        }
                    }}
                </Suspense>

                // Top filter bar - Selection toggle, bulk actions, Search, Uptime range, More button
                <div class="rounded-2xl" style="background: #0f172a; border: 1px solid #1e293b; padding: 8px 12px; margin: 5px 0px 0px 0px;">
                    <div class="flex flex-wrap items-center justify-between gap-3">
                        // Group 1: Selection toggle and bulk actions (left)
                        <div class="flex items-center gap-2 flex-shrink-0">
                            // Selection mode toggle (styled per screenshots)
                            <button
                                style=move || {
                                    if selection_mode.get() {
                                        "display: flex; align-items: center; gap: 8px; padding: 6px 12px; background: #3b82f6; border: none; border-radius: 20px; cursor: pointer; transition: all 0.2s;"
                                    } else {
                                        "display: flex; align-items: center; gap: 8px; padding: 6px 12px; background: #334155; border: none; border-radius: 20px; cursor: pointer; transition: all 0.2s;"
                                    }
                                }
                                on:click=move |_| {
                                    let new_mode = !selection_mode.get();
                                    set_selection_mode.set(new_mode);
                                    if !new_mode {
                                        set_selected_ids.set(HashSet::new());
                                    }
                                }
                            >
                                // Checkmark or X icon based on state
                                {move || if selection_mode.get() {
                                    view! {
                                        <svg class="w-4 h-4" fill="none" stroke="white" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                        </svg>
                                    }.into_any()
                                } else {
                                    view! {
                                        <svg class="w-4 h-4" fill="none" stroke="#94a3b8" stroke-width="2" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                        </svg>
                                    }.into_any()
                                }}
                                // Toggle pill
                                <div
                                    style=move || {
                                        if selection_mode.get() {
                                            "width: 36px; height: 20px; background: rgba(255,255,255,0.3); border-radius: 10px; position: relative;"
                                        } else {
                                            "width: 36px; height: 20px; background: rgba(255,255,255,0.2); border-radius: 10px; position: relative;"
                                        }
                                    }
                                >
                                    <div
                                        style=move || {
                                            if selection_mode.get() {
                                                "position: absolute; top: 2px; right: 2px; width: 16px; height: 16px; background: white; border-radius: 50%; transition: all 0.2s;"
                                            } else {
                                                "position: absolute; top: 2px; left: 2px; width: 16px; height: 16px; background: white; border-radius: 50%; transition: all 0.2s;"
                                            }
                                        }
                                    ></div>
                                </div>
                            </button>

                            // Selection count (shown when items are selected)
                            <Show when=move || !selected_ids.get().is_empty()>
                                <span style="color: #94a3b8; font-size: 14px;">
                                    {move || format!("{} selected", selected_ids.get().len())}
                                </span>
                            </Show>

                            // Edit button (enabled when items selected)
                            <button
                                style=move || {
                                    if selected_ids.get().is_empty() {
                                        "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; color: #64748b; cursor: not-allowed; opacity: 0.5;"
                                    } else {
                                        "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #3b82f6; border: none; border-radius: 8px; color: white; cursor: pointer; transition: all 0.15s;"
                                    }
                                }
                                disabled=move || selected_ids.get().is_empty()
                                on:click=move |_| {
                                    if !selected_ids.get().is_empty() {
                                        set_show_edit_modal.set(true);
                                    }
                                }
                                title="Edit Selected"
                            >
                                <Icon name=IconName::Edit size=18 />
                            </button>

                            // Revoke button (enabled when items selected)
                            <button
                                style=move || {
                                    if selected_ids.get().is_empty() {
                                        "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; color: #64748b; cursor: not-allowed; opacity: 0.5;"
                                    } else {
                                        "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #ef4444; border: none; border-radius: 8px; color: white; cursor: pointer; transition: all 0.15s;"
                                    }
                                }
                                disabled=move || selected_ids.get().is_empty()
                                on:click=move |_| {
                                    if !selected_ids.get().is_empty() {
                                        set_show_bulk_revoke_modal.set(true);
                                    }
                                }
                                title="Revoke Selected"
                            >
                                <Icon name=IconName::Trash size=18 />
                            </button>
                        </div>

                        // Group 2: Search (center)
                        <div class="flex items-center gap-2" style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 4px 8px; min-height: 36px; flex: 1; max-width: 280px; min-width: 120px;">
                                <input
                                    type="text"
                                    placeholder="Search license ID..."
                                    class="text-sm text-white/90 focus:outline-none flex-1"
                                    style="background: transparent; border: none; min-width: 80px; padding: 4px;"
                                    prop:value=move || search_query.get()
                                    on:input=move |ev| {
                                        set_search_query.set(event_target_value(&ev));
                                        set_search_error.set(None);
                                    }
                                    on:keydown=move |ev: web_sys::KeyboardEvent| {
                                        if ev.key() == "Enter" && !search_query.get().is_empty() && !searching.get() {
                                            let query = search_query.get();
                                            set_searching.set(true);
                                            set_search_error.set(None);

                                            #[cfg(target_arch = "wasm32")]
                                            {
                                                use wasm_bindgen_futures::spawn_local;
                                                spawn_local(async move {
                                                    match search_license_by_id(query.clone()).await {
                                                        Ok(Some(license)) => {
                                                            set_search_result.set(Some(license));
                                                            set_is_search_active.set(true);
                                                            set_search_error.set(None);
                                                        }
                                                        Ok(None) => {
                                                            set_search_result.set(None);
                                                            set_is_search_active.set(true);
                                                            set_search_error.set(Some("License not found".to_string()));
                                                        }
                                                        Err(e) => {
                                                            set_search_result.set(None);
                                                            set_is_search_active.set(false);
                                                            set_search_error.set(Some(format!("Search failed: {}", e)));
                                                        }
                                                    }
                                                    set_searching.set(false);
                                                });
                                            }
                                        }
                                    }
                                />
                                <button
                                    style=move || {
                                        if searching.get() || search_query.get().is_empty() {
                                            "display: flex; align-items: center; justify-content: center; padding: 4px 10px; background: #334155; border: none; border-radius: 6px; color: #64748b; cursor: not-allowed; font-size: 13px; font-weight: 500;"
                                        } else {
                                            "display: flex; align-items: center; justify-content: center; padding: 4px 10px; background: #06b6d4; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 13px; font-weight: 500; transition: all 0.15s;"
                                        }
                                    }
                                    disabled=move || searching.get() || search_query.get().is_empty()
                                    on:click=move |_| {
                                        if !search_query.get().is_empty() && !searching.get() {
                                            let query = search_query.get();
                                            set_searching.set(true);
                                            set_search_error.set(None);

                                            #[cfg(target_arch = "wasm32")]
                                            {
                                                use wasm_bindgen_futures::spawn_local;
                                                spawn_local(async move {
                                                    match search_license_by_id(query.clone()).await {
                                                        Ok(Some(license)) => {
                                                            set_search_result.set(Some(license));
                                                            set_is_search_active.set(true);
                                                            set_search_error.set(None);
                                                        }
                                                        Ok(None) => {
                                                            set_search_result.set(None);
                                                            set_is_search_active.set(true);
                                                            set_search_error.set(Some("License not found".to_string()));
                                                        }
                                                        Err(e) => {
                                                            set_search_result.set(None);
                                                            set_is_search_active.set(false);
                                                            set_search_error.set(Some(format!("Search failed: {}", e)));
                                                        }
                                                    }
                                                    set_searching.set(false);
                                                });
                                            }
                                        }
                                    }
                                >
                                    {move || if searching.get() {
                                        view! { <ProgressSpinner size=SpinnerSize::Small /> }.into_any()
                                    } else {
                                        view! { "Go" }.into_any()
                                    }}
                                </button>
                                // Clear button (shown when search is active)
                                <Show when=move || is_search_active.get()>
                                    <button
                                        style="display: flex; align-items: center; justify-content: center; padding: 4px 8px; background: #ef4444; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 12px; font-weight: 500;"
                                        on:click=move |_| {
                                            set_search_query.set(String::new());
                                            set_search_result.set(None);
                                            set_is_search_active.set(false);
                                            set_search_error.set(None);
                                        }
                                        title="Clear Search"
                                    >
                                        "×"
                                    </button>
                                </Show>
                            </div>

                        // Group 3: More button (right)
                        <div class="flex items-center gap-2 flex-shrink-0">
                            // More filters button
                            <button
                                style="display: flex; align-items: center; gap: 6px; padding: 8px 14px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; color: #94a3b8; font-size: 14px; cursor: pointer; transition: all 0.15s;"
                                on:click=move |_| {
                                    set_modal_status.set(status_filter.get());
                                    set_modal_bound.set(bound_filter.get());
                                    set_modal_leased.set(leased_filter.get());
                                    set_show_filter_modal.set(true);
                                }
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
                                </svg>
                                "More"
                            </button>
                        </div>
                    </div>
                </div>

                // Bottom filter bar - Show and View toggle
                <div class="rounded-2xl" style="background: #0f172a; border: 1px solid #1e293b; padding: 5px; margin: 5px 0px 5px 0px; position: relative; z-index: 1;">
                    <div class="flex flex-wrap items-center justify-between gap-4">
                        <div class="flex items-center gap-4">
                            // Page size
                            <div class="flex items-center gap-2">
                                <label class="text-sm text-slate-400">"Show:"</label>
                                <select
                                    class="text-sm text-white/90 focus:outline-none focus:ring-2 focus:ring-primary-500 transition-all"
                                    style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 8px 32px 8px 12px; appearance: none; background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2212%22 height=%2212%22 viewBox=%220 0 12 12%22%3E%3Cpath fill=%22%2394a3b8%22 d=%22M2 4l4 4 4-4%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 12px center;"
                                    on:change=move |ev| {
                                        let value: i32 = event_target_value(&ev).parse().unwrap_or(10);
                                        set_page_size.set(value);
                                        set_current_page.set(1); // Reset to first page
                                    }
                                >
                                    <option value="5" selected>"5"</option>
                                    <option value="10">"10"</option>
                                    <option value="15">"15"</option>
                                    <option value="20">"20"</option>
                                    <option value="25">"25"</option>
                                    <option value="50">"50"</option>
                                    <option value="100">"100"</option>
                                </select>
                            </div>
                        </div>

                        // View toggle
                        <div class="flex items-center gap-1 p-1.5 rounded-lg" style="background: #1e293b;">
                            <button
                                class={move || format!(
                                    "px-4 py-2 rounded-md text-sm font-medium transition {}",
                                    if view_mode.get() == ViewMode::Grid {
                                        "bg-primary-500 text-white shadow-sm"
                                    } else {
                                        "text-slate-400 hover:text-white"
                                    }
                                )}
                                on:click=move |_| set_view_mode.set(ViewMode::Grid)
                            >
                                <span class="flex items-center gap-1.5">
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
                                    </svg>
                                    "Grid"
                                </span>
                            </button>
                            <button
                                class={move || format!(
                                    "px-4 py-2 rounded-md text-sm font-medium transition {}",
                                    if view_mode.get() == ViewMode::Table {
                                        "bg-primary-500 text-white shadow-sm"
                                    } else {
                                        "text-slate-400 hover:text-white"
                                    }
                                )}
                                on:click=move |_| set_view_mode.set(ViewMode::Table)
                            >
                                <span class="flex items-center gap-1.5">
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
                                    </svg>
                                    "Table"
                                </span>
                            </button>
                        </div>
                    </div>
                </div>

                // License list - use Transition to prevent hydration errors and keep content visible during loading
                <div style="position: relative;">
                    // Loading overlay - always rendered, visibility controlled via CSS to avoid hydration issues
                    <div
                        style=move || format!(
                            "position: absolute; inset: 0; background: rgba(15, 23, 42, 0.7); display: flex; align-items: center; justify-content: center; z-index: 10; border-radius: 12px; pointer-events: none; transition: opacity 0.15s; opacity: {}; visibility: {};",
                            if show_loading_overlay.get() || is_table_loading.get() { "1" } else { "0" },
                            if show_loading_overlay.get() || is_table_loading.get() { "visible" } else { "hidden" }
                        )
                    >
                        <ProgressSpinner size=SpinnerSize::Default />
                    </div>
                    {
                        let on_click_fallback = on_license_click.clone();
                        view! {
                            <Transition fallback=move || view! {
                                // Fallback shows table with loading state (header always visible)
                                {render_license_list(Vec::new(), 0, 0, 0, 1, 1, current_page, set_current_page, view_mode.get(), on_click_fallback.clone(), selection_mode, selected_ids, set_selected_ids, license_filter, marketplace_filter, status_filter, set_status_filter, expanded_licenses, uptime_range, true)}
                            }>
                    {move || {
                        let ps = page_size.get() as usize;
                        let current_pg = current_page.get() as usize;

                        // If search is active and has a result, show the search result
                        if is_search_active.get() {
                            if let Some(license) = search_result.get() {
                                // Show single search result
                                let page_licenses = vec![license];
                                return render_license_list(page_licenses, 1, 1, 1, 1, 1, current_page, set_current_page, view_mode.get(), on_license_click.clone(), selection_mode, selected_ids, set_selected_ids, license_filter, marketplace_filter, status_filter, set_status_filter, expanded_licenses, uptime_range, false).into_any();
                            } else {
                                // Search active but no result found - show table with empty state
                                return render_license_list(Vec::new(), 0, 0, 0, 1, 1, current_page, set_current_page, view_mode.get(), on_license_click.clone(), selection_mode, selected_ids, set_selected_ids, license_filter, marketplace_filter, status_filter, set_status_filter, expanded_licenses, uptime_range, false).into_any();
                            }
                        }

                        // Normal paginated view
                        match paginated_resource.get() {
                            None => {
                                // Loading state - show table with loading indicator in body
                                render_license_list(Vec::new(), 0, 0, 0, 1, current_pg, current_page, set_current_page, view_mode.get(), on_license_click.clone(), selection_mode, selected_ids, set_selected_ids, license_filter, marketplace_filter, status_filter, set_status_filter, expanded_licenses, uptime_range, true).into_any()
                            },
                            Some(Err(e)) => {
                                // Error state - show table with error message in body
                                view! {
                                    <div style="display: flex; flex-direction: column; gap: 5px;">
                                        <LicenseTable
                                            licenses=Vec::new()
                                            on_click=on_license_click.clone()
                                            selection_mode=selection_mode
                                            selected_ids=selected_ids
                                            set_selected_ids=set_selected_ids
                                            license_filter=license_filter
                                            marketplace_filter=marketplace_filter
                                            status_filter=status_filter
                                            set_status_filter=set_status_filter
                                            expanded_licenses=expanded_licenses
                                            uptime_range=uptime_range
                                            is_loading=false
                                        />
                                        <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
                                            <strong>"Error: "</strong>{e.to_string()}
                                        </div>
                                    </div>
                                }.into_any()
                            },
                            Some(Ok(response)) => {
                                let page_licenses = response.licenses;
                                let total_count = response.total_count as usize;
                                let total_pages_val = response.total_pages;

                                let start_display = if total_count > 0 { ((current_pg - 1) * ps) + 1 } else { 0 };
                                let end_display = if page_licenses.is_empty() { 0 } else { start_display + page_licenses.len() - 1 };
                                let display_total = total_count;

                                render_license_list(page_licenses, display_total, start_display, end_display, total_pages_val, current_pg, current_page, set_current_page, view_mode.get(), on_license_click.clone(), selection_mode, selected_ids, set_selected_ids, license_filter, marketplace_filter, status_filter, set_status_filter, expanded_licenses, uptime_range, false).into_any()
                            }
                        }
                    }}
                </Transition>
                        }
                    }
                </div>
            </div>

            // Filter modal
            <Show when=move || show_filter_modal.get()>
                <div
                    style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.6); display: flex; align-items: center; justify-content: center; z-index: 9999;"
                    on:click=move |_| set_show_filter_modal.set(false)
                >
                    <div
                        style="background: #1e293b; border: 1px solid #334155; border-radius: 16px; padding: 24px; max-width: 400px; width: 90%;"
                        on:click=move |e| e.stop_propagation()
                    >
                        // Modal header
                        <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 20px;">
                            <h3 style="color: #f1f5f9; font-size: 18px; font-weight: 600; margin: 0;">
                                "Filter Options"
                            </h3>
                            <button
                                style="background: transparent; border: none; color: #64748b; cursor: pointer; padding: 4px;"
                                on:click=move |_| set_show_filter_modal.set(false)
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            </button>
                        </div>

                        // Filter options (using temporary modal state)
                        <div style="display: flex; flex-direction: column; gap: 16px;">
                            // Status filter (Online/Offline)
                            <div style="display: flex; flex-direction: column; gap: 8px;">
                                <label style="color: #94a3b8; font-size: 14px; font-weight: 500;">"Status"</label>
                                <div style="display: flex; gap: 8px;">
                                    <button
                                        style={move || format!(
                                            "flex: 1; padding: 10px; border-radius: 8px; font-size: 14px; font-weight: 500; border: 1px solid {}; cursor: pointer; transition: all 0.15s; {}",
                                            if modal_status.get() == StatusFilter::All { "#0ea5e9" } else { "#334155" },
                                            if modal_status.get() == StatusFilter::All { "background: #0ea5e9; color: white;" } else { "background: #0f172a; color: #94a3b8;" }
                                        )}
                                        on:click=move |_| set_modal_status.set(StatusFilter::All)
                                    >
                                        "All"
                                    </button>
                                    <button
                                        style={move || format!(
                                            "flex: 1; padding: 10px; border-radius: 8px; font-size: 14px; font-weight: 500; border: 1px solid {}; cursor: pointer; transition: all 0.15s; {}",
                                            if modal_status.get() == StatusFilter::Online { "#22c55e" } else { "#334155" },
                                            if modal_status.get() == StatusFilter::Online { "background: #22c55e; color: white;" } else { "background: #0f172a; color: #94a3b8;" }
                                        )}
                                        on:click=move |_| set_modal_status.set(StatusFilter::Online)
                                    >
                                        "Online"
                                    </button>
                                    <button
                                        style={move || format!(
                                            "flex: 1; padding: 10px; border-radius: 8px; font-size: 14px; font-weight: 500; border: 1px solid {}; cursor: pointer; transition: all 0.15s; {}",
                                            if modal_status.get() == StatusFilter::Offline { "#64748b" } else { "#334155" },
                                            if modal_status.get() == StatusFilter::Offline { "background: #64748b; color: white;" } else { "background: #0f172a; color: #94a3b8;" }
                                        )}
                                        on:click=move |_| set_modal_status.set(StatusFilter::Offline)
                                    >
                                        "Offline"
                                    </button>
                                </div>
                            </div>
// Leased filter
                            <div style="display: flex; flex-direction: column; gap: 8px;">
                                <label style="color: #94a3b8; font-size: 14px; font-weight: 500;">"Leased Status"</label>
                                <div style="display: flex; gap: 8px;">
                                    <button
                                        style={move || format!(
                                            "flex: 1; padding: 10px; border-radius: 8px; font-size: 14px; font-weight: 500; border: 1px solid {}; cursor: pointer; transition: all 0.15s; {}",
                                            if modal_leased.get() == LeasedFilter::All { "#0ea5e9" } else { "#334155" },
                                            if modal_leased.get() == LeasedFilter::All { "background: #0ea5e9; color: white;" } else { "background: #0f172a; color: #94a3b8;" }
                                        )}
                                        on:click=move |_| set_modal_leased.set(LeasedFilter::All)
                                    >
                                        "All"
                                    </button>
                                    <button
                                        style={move || format!(
                                            "flex: 1; padding: 10px; border-radius: 8px; font-size: 14px; font-weight: 500; border: 1px solid {}; cursor: pointer; transition: all 0.15s; {}",
                                            if modal_leased.get() == LeasedFilter::Leased { "#06b6d4" } else { "#334155" },
                                            if modal_leased.get() == LeasedFilter::Leased { "background: #06b6d4; color: white;" } else { "background: #0f172a; color: #94a3b8;" }
                                        )}
                                        on:click=move |_| set_modal_leased.set(LeasedFilter::Leased)
                                    >
                                        "Leased"
                                    </button>
                                    <button
                                        style={move || format!(
                                            "flex: 1; padding: 10px; border-radius: 8px; font-size: 14px; font-weight: 500; border: 1px solid {}; cursor: pointer; transition: all 0.15s; {}",
                                            if modal_leased.get() == LeasedFilter::NotLeased { "#64748b" } else { "#334155" },
                                            if modal_leased.get() == LeasedFilter::NotLeased { "background: #64748b; color: white;" } else { "background: #0f172a; color: #94a3b8;" }
                                        )}
                                        on:click=move |_| set_modal_leased.set(LeasedFilter::NotLeased)
                                    >
                                        "Not Leased"
                                    </button>
                                </div>
                            </div>
                            // Bound filter
                            <div style="display: flex; flex-direction: column; gap: 8px;">
                                <label style="color: #94a3b8; font-size: 14px; font-weight: 500;">"Bound Status"</label>
                                <div style="display: flex; gap: 8px;">
                                    <button
                                        style={move || format!(
                                            "flex: 1; padding: 10px; border-radius: 8px; font-size: 14px; font-weight: 500; border: 1px solid {}; cursor: pointer; transition: all 0.15s; {}",
                                            if modal_bound.get() == BoundFilter::All { "#0ea5e9" } else { "#334155" },
                                            if modal_bound.get() == BoundFilter::All { "background: #0ea5e9; color: white;" } else { "background: #0f172a; color: #94a3b8;" }
                                        )}
                                        on:click=move |_| set_modal_bound.set(BoundFilter::All)
                                    >
                                        "All"
                                    </button>
                                    <button
                                        style={move || format!(
                                            "flex: 1; padding: 10px; border-radius: 8px; font-size: 14px; font-weight: 500; border: 1px solid {}; cursor: pointer; transition: all 0.15s; {}",
                                            if modal_bound.get() == BoundFilter::Bound { "#a855f7" } else { "#334155" },
                                            if modal_bound.get() == BoundFilter::Bound { "background: #a855f7; color: white;" } else { "background: #0f172a; color: #94a3b8;" }
                                        )}
                                        on:click=move |_| set_modal_bound.set(BoundFilter::Bound)
                                    >
                                        "Bound"
                                    </button>
                                    <button
                                        style={move || format!(
                                            "flex: 1; padding: 10px; border-radius: 8px; font-size: 14px; font-weight: 500; border: 1px solid {}; cursor: pointer; transition: all 0.15s; {}",
                                            if modal_bound.get() == BoundFilter::NotBound { "#64748b" } else { "#334155" },
                                            if modal_bound.get() == BoundFilter::NotBound { "background: #64748b; color: white;" } else { "background: #0f172a; color: #94a3b8;" }
                                        )}
                                        on:click=move |_| set_modal_bound.set(BoundFilter::NotBound)
                                    >
                                        "Not Bound"
                                    </button>
                                </div>
                            </div>
                        </div>

                        // Modal footer
                        <div style="display: flex; gap: 12px; justify-content: flex-end; margin-top: 24px; padding-top: 16px; border-top: 1px solid #334155;">
                            <button
                                style="padding: 10px 20px; background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 8px; cursor: pointer; font-size: 14px;"
                                on:click=move |_| {
                                    // Reset modal state to defaults
                                    set_modal_status.set(StatusFilter::All);
                                    set_modal_bound.set(BoundFilter::All);
                                    set_modal_leased.set(LeasedFilter::All);
                                }
                            >
                                "Reset"
                            </button>
                            <button
                                style="padding: 10px 20px; background: #0ea5e9; color: white; border: none; border-radius: 8px; cursor: pointer; font-size: 14px; font-weight: 500;"
                                on:click=move |_| {
                                    // Copy modal state to actual filter state (triggers resource refetch)
                                    set_status_filter.set(modal_status.get());
                                    set_bound_filter.set(modal_bound.get());
                                    set_leased_filter.set(modal_leased.get());
                                    set_current_page.set(1); // Reset to first page
                                    set_show_filter_modal.set(false);
                                }
                            >
                                "Apply"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>

            // Edit Settings Modal - Compact Layout
            <Show when=move || show_edit_modal.get()>
                <div
                    style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.6); display: flex; align-items: center; justify-content: center; z-index: 9999;"
                    on:click=move |_| set_show_edit_modal.set(false)
                >
                    <div
                        style="background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 0; width: 680px; max-width: 95vw;"
                        on:click=move |e| e.stop_propagation()
                    >
                        // Header
                        <div style="display: flex; justify-content: space-between; align-items: center; padding: 16px 20px; border-bottom: 1px solid #334155;">
                            <div>
                                <h2 style="color: #f1f5f9; font-size: 18px; font-weight: 600; margin: 0;">"Edit Settings"</h2>
                                <p style="color: #64748b; font-size: 12px; margin: 2px 0 0 0;">
                                    {move || format!("Applying to {} licenses", selected_ids.get().len())}
                                </p>
                            </div>
                            <button
                                style="background: transparent; border: none; color: #64748b; cursor: pointer; padding: 4px;"
                                on:click=move |_| set_show_edit_modal.set(false)
                            >
                                <Icon name=IconName::Close size=20 />
                            </button>
                        </div>

                        // Body - Two Column Grid Layout
                        <div style="padding: 16px 20px;">
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
                                // Left Column
                                <div style="display: flex; flex-direction: column; gap: 14px;">
                                    // Base Terms Section
                                    <div style="background: #0f172a; border: 1px solid #334155; border-radius: 8px; padding: 12px;">
                                        <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 8px;">
                                            <Icon name=IconName::Settings size=16 class="text-cyan-400".to_string() />
                                            <span style="color: #e2e8f0; font-weight: 500; font-size: 13px;">"Revenue Share"</span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 6px;">
                                            <span style="color: #94a3b8; font-size: 11px;">{move || format!("Node: {}%", (100.0 - edit_share_percentage.get()) as i32)}</span>
                                            <span style="color: #94a3b8; font-size: 11px;">{move || format!("Lease: {}%", edit_share_percentage.get() as i32)}</span>
                                        </div>
                                        <SingleValueSlider value=edit_share_percentage min=0.0 max=100.0 />
                                    </div>

                                    // Duration of Lease Section
                                    <div style="background: #0f172a; border: 1px solid #334155; border-radius: 8px; padding: 12px;">
                                        <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 8px;">
                                            <Icon name=IconName::Calendar size=16 class="text-cyan-400".to_string() />
                                            <span style="color: #e2e8f0; font-weight: 500; font-size: 13px;">"Lease Duration"</span>
                                            <span style="color: #22d3ee; font-size: 12px; margin-left: auto; font-weight: 600;">
                                                {move || format!("{} mo", edit_lease_duration.get() as i32)}
                                            </span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 6px;">
                                            <span style="color: #94a3b8; font-size: 11px;">"2 mo"</span>
                                            <span style="color: #94a3b8; font-size: 11px;">"12 mo"</span>
                                        </div>
                                        <SingleValueSlider value=edit_lease_duration min=2.0 max=12.0 />
                                    </div>

                                    // Uptime Requirement Section
                                    <div style="background: #0f172a; border: 1px solid #334155; border-radius: 8px; padding: 12px;">
                                        <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 8px;">
                                            <Icon name=IconName::Clock size=16 class="text-cyan-400".to_string() />
                                            <span style="color: #e2e8f0; font-weight: 500; font-size: 13px;">"Min Uptime"</span>
                                            <span style="color: #22d3ee; font-size: 12px; margin-left: auto; font-weight: 600;">
                                                {move || format!("{}%", edit_uptime_requirement.get() as i32)}
                                            </span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 6px;">
                                            <span style="color: #94a3b8; font-size: 11px;">"0%"</span>
                                            <span style="color: #94a3b8; font-size: 11px;">"100%"</span>
                                        </div>
                                        <SingleValueSlider value=edit_uptime_requirement min=0.0 max=100.0 />
                                    </div>
                                </div>

                                // Right Column
                                <div style="display: flex; flex-direction: column; gap: 14px;">
                                    // Marketplace Visibility Section
                                    <div style="background: #0f172a; border: 1px solid #334155; border-radius: 8px; padding: 12px;">
                                        <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 10px;">
                                            <Icon name=IconName::Store size=16 class="text-cyan-400".to_string() />
                                            <span style="color: #e2e8f0; font-weight: 500; font-size: 13px;">"Marketplace"</span>
                                        </div>
                                        <div style="display: flex; flex-direction: column; gap: 8px;">
                                            <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                                                <input
                                                    type="radio"
                                                    name="marketplace"
                                                    style="width: 16px; height: 16px; accent-color: #22d3ee; cursor: pointer;"
                                                    prop:checked=move || edit_uno_marketplace.get()
                                                    on:change=move |_| {
                                                        edit_uno_marketplace.set(true);
                                                        edit_unetwork_marketplace.set(false);
                                                    }
                                                />
                                                <span style="color: #cbd5e1; font-size: 13px;">"UNO marketplace"</span>
                                            </label>
                                            <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                                                <input
                                                    type="radio"
                                                    name="marketplace"
                                                    style="width: 16px; height: 16px; accent-color: #22d3ee; cursor: pointer;"
                                                    prop:checked=move || edit_unetwork_marketplace.get()
                                                    on:change=move |_| {
                                                        edit_unetwork_marketplace.set(true);
                                                        edit_uno_marketplace.set(false);
                                                    }
                                                />
                                                <span style="color: #cbd5e1; font-size: 13px;">"Unetwork marketplace"</span>
                                            </label>
                                        </div>
                                    </div>

                                    // Referral Assignment Section
                                    <div style="background: #0f172a; border: 1px solid #334155; border-radius: 8px; padding: 12px; flex: 1;">
                                        <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 10px;">
                                            <div style="display: flex; align-items: center; gap: 6px;">
                                                <Icon name=IconName::Users size=16 class="text-cyan-400".to_string() />
                                                <span style="color: #e2e8f0; font-weight: 500; font-size: 13px;">"Referral"</span>
                                            </div>
                                            <button
                                                style=move || {
                                                    if enable_referral.get() {
                                                        "width: 36px; height: 20px; background: #22d3ee; border: none; border-radius: 10px; cursor: pointer; position: relative; transition: background 0.2s;"
                                                    } else {
                                                        "width: 36px; height: 20px; background: #334155; border: none; border-radius: 10px; cursor: pointer; position: relative; transition: background 0.2s;"
                                                    }
                                                }
                                                on:click=move |_| {
                                                    enable_referral.set(!enable_referral.get());
                                                    if !enable_referral.get() {
                                                        selected_referral.set(String::new());
                                                    }
                                                }
                                            >
                                                <div
                                                    style=move || {
                                                        if enable_referral.get() {
                                                            "position: absolute; top: 2px; right: 2px; width: 16px; height: 16px; background: white; border-radius: 50%; transition: all 0.2s;"
                                                        } else {
                                                            "position: absolute; top: 2px; left: 2px; width: 16px; height: 16px; background: white; border-radius: 50%; transition: all 0.2s;"
                                                        }
                                                    }
                                                ></div>
                                            </button>
                                        </div>
                                        <Show when=move || enable_referral.get()>
                                            <Suspense fallback=move || view! {
                                                <div style="padding: 8px; background: #1e293b; border: 1px solid #334155; border-radius: 6px; color: #64748b; font-size: 12px;">
                                                    "Loading..."
                                                </div>
                                            }>
                                                {move || match agents_resource.get() {
                                                    Some(Ok(agents)) => view! {
                                                        <select
                                                            style="width: 100%; padding: 8px 10px; background: #1e293b; border: 1px solid #334155; border-radius: 6px; color: #f1f5f9; font-size: 12px; outline: none; cursor: pointer;"
                                                            on:change=move |ev| {
                                                                selected_referral.set(event_target_value(&ev));
                                                            }
                                                        >
                                                            <option value="" selected=move || selected_referral.get().is_empty()>
                                                                "Select agent..."
                                                            </option>
                                                            {agents.iter().map(|agent| {
                                                                let code = agent.referral_code.clone().unwrap_or_default();
                                                                let display = format!("{} ({})", agent.name, if code.is_empty() { "no code" } else { &code });
                                                                let code_value = code.clone();
                                                                view! {
                                                                    <option value=code_value>{display}</option>
                                                                }
                                                            }).collect::<Vec<_>>()}
                                                        </select>
                                                    }.into_any(),
                                                    Some(Err(_)) => view! {
                                                        <div style="padding: 8px; background: #1e293b; border: 1px solid #ef4444; border-radius: 6px; color: #ef4444; font-size: 12px;">
                                                            "Failed to load"
                                                        </div>
                                                    }.into_any(),
                                                    None => view! {
                                                        <div style="padding: 8px; background: #1e293b; border: 1px solid #334155; border-radius: 6px; color: #64748b; font-size: 12px;">
                                                            "Loading..."
                                                        </div>
                                                    }.into_any(),
                                                }}
                                            </Suspense>
                                        </Show>
                                        <Show when=move || !enable_referral.get()>
                                            <p style="color: #64748b; font-size: 11px; margin: 0;">"Enable to assign a referral agent"</p>
                                        </Show>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Footer
                        <div style="padding: 12px 20px; border-top: 1px solid #334155; display: flex; gap: 10px;">
                            <button
                                style="flex: 1; padding: 10px; background: #334155; color: #94a3b8; border: none; border-radius: 6px; font-size: 13px; font-weight: 500; cursor: pointer;"
                                on:click=move |_| set_show_edit_modal.set(false)
                            >
                                "Cancel"
                            </button>
                            <button
                                style="flex: 2; padding: 10px; background: #22d3ee; color: #0f172a; border: none; border-radius: 6px; font-size: 13px; font-weight: 600; cursor: pointer; transition: background 0.15s;"
                                on:click=move |_| {
                                    let ids: Vec<String> = selected_ids.get().iter().cloned().collect();
                                    if ids.is_empty() {
                                        set_show_edit_modal.set(false);
                                        return;
                                    }

                                    let request = BulkSaveSettingsRequest {
                                        license_ids: ids,
                                        share_percentage: edit_share_percentage.get(),
                                        lease_duration_months: edit_lease_duration.get() as i32,
                                        uptime_requirement: edit_uptime_requirement.get(),
                                        uno_marketplace: edit_uno_marketplace.get(),
                                        unetwork_marketplace: edit_unetwork_marketplace.get(),
                                        referral_code: if enable_referral.get() && !selected_referral.get().is_empty() {
                                            Some(selected_referral.get())
                                        } else {
                                            None
                                        },
                                    };

                                    spawn_local(async move {
                                        match bulk_save_license_settings(request).await {
                                            Ok(res) => {
                                                web_sys::console::log_1(
                                                    &format!("Bulk save complete: {} succeeded, {} failed", res.succeeded, res.failed).into()
                                                );
                                            }
                                            Err(e) => {
                                                web_sys::console::error_1(
                                                    &format!("Bulk save error: {}", e).into()
                                                );
                                            }
                                        }
                                    });

                                    set_show_edit_modal.set(false);
                                }
                            >
                                "Apply Settings"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>

            // Bulk Revoke Warning Modal
            <Show when=move || show_bulk_revoke_modal.get()>
                <div
                    style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.6); display: flex; align-items: center; justify-content: center; z-index: 9999;"
                    on:click=move |_| set_show_bulk_revoke_modal.set(false)
                >
                    <div
                        style="background: #1e293b; border: 1px solid #334155; border-radius: 16px; padding: 24px; max-width: 450px; width: 90%;"
                        on:click=move |e| e.stop_propagation()
                    >
                        // Warning icon and header
                        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 16px;">
                            <div style="width: 48px; height: 48px; background: rgba(239, 68, 68, 0.2); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                                <Icon name=IconName::Trash size=24 class="text-red-500".to_string() />
                            </div>
                            <h3 style="color: #f1f5f9; font-size: 20px; font-weight: 600; margin: 0;">
                                "Revoke Licenses"
                            </h3>
                        </div>

                        // Warning message
                        <p style="color: #94a3b8; font-size: 14px; line-height: 1.6; margin-bottom: 16px;">
                            "You are about to terminate the lease of the following licenses:"
                        </p>

                        // License IDs list
                        <div style="background: #0f172a; border: 1px solid #334155; border-radius: 8px; padding: 12px; max-height: 150px; overflow-y: auto; margin-bottom: 16px;">
                            {move || {
                                let ids: Vec<String> = selected_ids.get().iter().cloned().collect();
                                let display_ids: Vec<String> = ids.iter().take(5).map(|id| {
                                    if id.len() > 20 {
                                        format!("{}...{}", &id[..10], &id[id.len()-8..])
                                    } else {
                                        id.clone()
                                    }
                                }).collect();
                                let remaining = if ids.len() > 5 { ids.len() - 5 } else { 0 };

                                view! {
                                    <ul style="list-style: none; margin: 0; padding: 0;">
                                        {display_ids.into_iter().map(|id| {
                                            view! {
                                                <li style="color: #cbd5e1; font-family: monospace; font-size: 13px; padding: 4px 0;">
                                                    "• "{id}
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                        {if remaining > 0 {
                                            view! {
                                                <li style="color: #64748b; font-size: 13px; padding: 4px 0; font-style: italic;">
                                                    {format!("...and {} more", remaining)}
                                                </li>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </ul>
                                }
                            }}
                        </div>

                        // Warning text
                        <div style="background: rgba(239, 68, 68, 0.1); border: 1px solid rgba(239, 68, 68, 0.3); border-radius: 8px; padding: 12px; margin-bottom: 24px;">
                            <p style="color: #f87171; font-size: 14px; font-weight: 500; margin: 0; display: flex; align-items: center; gap: 8px;">
                                <svg style="width: 16px; height: 16px; flex-shrink: 0;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                                </svg>
                                "This action is non-reversible."
                            </p>
                        </div>

                        // Action buttons
                        <div style="display: flex; gap: 12px; justify-content: flex-end;">
                            <button
                                style="padding: 10px 20px; background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 8px; cursor: pointer; font-size: 14px;"
                                on:click=move |_| set_show_bulk_revoke_modal.set(false)
                            >
                                "Cancel"
                            </button>
                            <button
                                style="padding: 10px 20px; background: #ef4444; color: white; border: none; border-radius: 8px; cursor: pointer; font-size: 14px; font-weight: 500;"
                                on:click=move |_| {
                                    // TODO: Implement bulk revoke API call
                                    // For now, just close the modal and clear selection
                                    set_show_bulk_revoke_modal.set(false);
                                    set_selected_ids.set(HashSet::new());
                                    set_selection_mode.set(false);
                                }
                            >
                                "Revoke"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}

/// Summary card component (legacy)
#[component]
fn SummaryCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-slate-800 rounded-xl p-4 border border-slate-200 dark:border-slate-700">
            <div class="text-sm text-slate-500 dark:text-slate-400 mb-1">{label}</div>
            <div class="text-xl font-semibold text-slate-900 dark:text-white">{value}</div>
        </div>
    }
}

/// Stat card with centered content - matches dashboard style
#[component]
fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(default = "default")] variant: &'static str,
) -> impl IntoView {
    let value_class = match variant {
        "success" => "text-green-500",
        "warning" => "text-amber-500",
        "error" => "text-red-500",
        _ => "text-slate-900 dark:text-white",
    };

    view! {
        <div class="bg-slate-800 border border-slate-700 rounded-xl p-4 text-center w-full h-full">
            <div class="text-xs text-slate-400 mb-1">{label}</div>
            <div class={format!("text-xl font-bold {}", value_class)}>{value}</div>
        </div>
    }
}

/// Earnings card with USD coin icon - matches dashboard style
#[component]
fn EarningsCard(
    #[prop(into)] label: String,
    amount: f64,
) -> impl IntoView {
    view! {
        <div class="bg-slate-800 border border-slate-700 rounded-xl p-4 text-center w-full h-full">
            <div class="text-xs text-slate-400 mb-1">{label}</div>
            <div class="text-xl font-bold text-green-500 flex items-center justify-center gap-1">
                <UsdCoinIcon size=18 />
                {format!("{:.2}", amount)}
            </div>
        </div>
    }
}

/// License grid view (cards)
#[component]
fn LicenseGrid(
    licenses: Vec<LicenseDto>,
    on_click: impl Fn(String) + Clone + Send + Sync + 'static,
    selection_mode: ReadSignal<bool>,
    selected_ids: ReadSignal<HashSet<String>>,
    set_selected_ids: WriteSignal<HashSet<String>>,
) -> impl IntoView {
    view! {
        <div style="background: #0f172a; border-radius: 12px; padding: 16px; margin-top: 0;">
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
                {licenses.into_iter().map(|license| {
                    let id = license.license_id.clone();
                    let click = on_click.clone();
                    view! {
                        <LicenseGridCard
                            license=license
                            on_click=move || click(id.clone())
                            selection_mode=selection_mode
                            selected_ids=selected_ids
                            set_selected_ids=set_selected_ids
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// License grid card component
#[component]
fn LicenseGridCard(
    license: LicenseDto,
    on_click: impl Fn() + Clone + Send + Sync + 'static,
    selection_mode: ReadSignal<bool>,
    selected_ids: ReadSignal<HashSet<String>>,
    set_selected_ids: WriteSignal<HashSet<String>>,
) -> impl IntoView {
    let license_id = license.license_id.clone();
    let license_id_for_toggle = license.license_id.clone();
    let uptime = license.uptime_percentage();
    let min_uptime = license.lease_min_uptime_percentage.max(75.0);
    let on_click_clone = on_click.clone();

    let status_dot_class = if license.is_online {
        "bg-green-500 shadow-green-500/50 shadow-lg"
    } else {
        "bg-slate-400 dark:bg-slate-500"
    };

    let uptime_color = if uptime >= 90.0 {
        "bg-green-500"
    } else if uptime >= min_uptime {
        "bg-amber-500"
    } else {
        "bg-red-500"
    };

    let uptime_bg = if uptime >= 90.0 {
        "bg-green-200 dark:bg-green-900/30"
    } else if uptime >= min_uptime {
        "bg-amber-200 dark:bg-amber-900/30"
    } else {
        "bg-red-200 dark:bg-red-900/30"
    };

    // Device name: prefer device_name, fallback to alias, then node_id short
    let device_display = license.device_name.clone()
        .or(license.alias.clone())
        .unwrap_or_else(|| license.node_id_short());

    // Revoke confirmation modal state
    let (show_revoke_modal, set_show_revoke_modal) = signal(false);
    let license_id_for_modal = license.license_id.clone();

    // Check if this card is selected (using derived signal for multiple usage)
    let is_selected = {
        let id = license_id.clone();
        Signal::derive(move || selected_ids.get().contains(&id))
    };

    view! {
        <div
            style=move || {
                if is_selected.get() {
                    "background: #1e293b; border: 2px solid #3b82f6; border-radius: 12px; overflow: hidden; transition: all 0.15s;"
                } else {
                    "background: #1e293b; border: 1px solid #334155; border-radius: 12px; overflow: hidden; transition: all 0.15s;"
                }
            }
        >
            // Header
            <div class="p-4 flex items-center justify-between">
                <div class="flex items-center gap-3">
                    // Checkbox (shown when selection mode is active)
                    <Show when=move || selection_mode.get()>
                        <button
                            style=move || {
                                if is_selected.get() {
                                    "width: 20px; height: 20px; background: #3b82f6; border: none; border-radius: 4px; display: flex; align-items: center; justify-content: center; cursor: pointer;"
                                } else {
                                    "width: 20px; height: 20px; background: transparent; border: 2px solid #64748b; border-radius: 4px; display: flex; align-items: center; justify-content: center; cursor: pointer;"
                                }
                            }
                            on:click={
                                let toggle_id = license_id_for_toggle.clone();
                                move |e: web_sys::MouseEvent| {
                                    e.stop_propagation();
                                    let id_to_toggle = toggle_id.clone();
                                    set_selected_ids.update(move |ids| {
                                        if ids.contains(&id_to_toggle) {
                                            ids.remove(&id_to_toggle);
                                        } else {
                                            ids.insert(id_to_toggle);
                                        }
                                    });
                                }
                            }
                        >
                            <Show when=move || is_selected.get()>
                                <svg class="w-3 h-3" fill="none" stroke="white" stroke-width="3" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                </svg>
                            </Show>
                        </button>
                    </Show>
                    <div class={format!("w-3 h-3 rounded-full {}", status_dot_class)}></div>
                    <span class="font-medium text-white">{license.license_id_short()}</span>
                </div>
                <div class="flex items-center gap-1">
                    {if license.is_leased {
                        view! {
                            <span class="fx-tag-prime fx-tag-prime-cyan" style="font-size: 10px; padding: 2px 6px;">"Leased"</span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                    {if license.is_bound {
                        view! {
                            <span class="fx-tag-prime fx-tag-prime-purple" style="font-size: 10px; padding: 2px 6px;">"Bound"</span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
            </div>

            // Content
            <div class="px-4 pb-4 space-y-3">
                // Device row
                <div class="flex items-center justify-between text-sm">
                    <div class="flex items-center gap-2 text-slate-400">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" />
                        </svg>
                        "Device"
                    </div>
                    <span class="text-white">{device_display}</span>
                </div>

                // Uptime row
                <div class="flex items-center justify-between text-sm">
                    <div class="flex items-center gap-2 text-slate-400">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        "Uptime"
                    </div>
                    <span class="text-white font-medium">{format!("{:.2}%", uptime)}</span>
                </div>

                // Uptime progress bar
                <div class="relative">
                    <div class={format!("h-2 rounded-full {}", uptime_bg)}>
                        <div
                            class={format!("h-full rounded-full transition-all {}", uptime_color)}
                            style={format!("width: {}%", uptime.min(100.0))}
                        ></div>
                    </div>
                </div>

                // Min uptime requirement
                <div class="text-right text-xs text-slate-500">
                    {format!("Min. Uptime: {}%", min_uptime as i32)}
                </div>

                // Action buttons
                <div style="display: flex; gap: 8px; justify-content: flex-end; padding-top: 8px; border-top: 1px solid #334155;">
                    // View button
                    <button
                        style="display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: #475569; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                        on:click=move |e| {
                            e.stop_propagation();
                            on_click_clone();
                        }
                        title="View Details"
                    >
                        <Icon name=IconName::Eye size=16 />
                    </button>
                    // Edit button
                    <button
                        style="display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                        on:click=move |e| {
                            e.stop_propagation();
                            // TODO: Implement edit functionality
                        }
                        title="Edit License"
                    >
                        <Icon name=IconName::Edit size=16 />
                    </button>
                    // Revoke button
                    <button
                        style="display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: #ef4444; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                        on:click=move |e| {
                            e.stop_propagation();
                            set_show_revoke_modal.set(true);
                        }
                        title="Revoke License"
                    >
                        <Icon name=IconName::Trash size=16 />
                    </button>
                </div>
            </div>

            // Revoke confirmation modal
            <Show when=move || show_revoke_modal.get()>
                <div
                    style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.6); display: flex; align-items: center; justify-content: center; z-index: 9999;"
                    on:click=move |_| set_show_revoke_modal.set(false)
                >
                    <div
                        style="background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 24px; max-width: 400px; width: 90%;"
                        on:click=move |e| e.stop_propagation()
                    >
                        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 16px;">
                            <div style="width: 40px; height: 40px; background: rgba(239, 68, 68, 0.2); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                                <Icon name=IconName::Trash size=20 class="text-red-500".to_string() />
                            </div>
                            <h3 style="color: #f1f5f9; font-size: 18px; font-weight: 600; margin: 0;">
                                "Revoke License"
                            </h3>
                        </div>
                        <p style="color: #94a3b8; font-size: 14px; line-height: 1.5; margin-bottom: 24px;">
                            "Are you sure you want to revoke license "
                            <span style="color: #e2e8f0; font-weight: 500; font-family: monospace;">{license_id_for_modal.clone()}</span>
                            "? This action cannot be undone."
                        </p>
                        <div style="display: flex; gap: 12px; justify-content: flex-end;">
                            <button
                                style="padding: 8px 16px; background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 8px; cursor: pointer;"
                                on:click=move |_| set_show_revoke_modal.set(false)
                            >
                                "Cancel"
                            </button>
                            <button
                                style="padding: 8px 16px; background: #ef4444; color: white; border: none; border-radius: 8px; cursor: pointer;"
                                on:click=move |_| {
                                    // TODO: Implement revoke functionality
                                    set_show_revoke_modal.set(false);
                                }
                            >
                                "Revoke"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}

/// License table view - styled like marketplace table with zebra stripes
#[component]
fn LicenseTable(
    licenses: Vec<LicenseDto>,
    on_click: impl Fn(String) + Clone + Send + Sync + 'static,
    selection_mode: ReadSignal<bool>,
    selected_ids: ReadSignal<HashSet<String>>,
    set_selected_ids: WriteSignal<HashSet<String>>,
    // Inline filter props
    license_filter: RwSignal<String>,
    marketplace_filter: RwSignal<MarketplaceFilter>,
    status_filter: ReadSignal<StatusFilter>,
    set_status_filter: WriteSignal<StatusFilter>,
    // Expanded rows tracking
    expanded_licenses: RwSignal<HashSet<String>>,
    // Uptime filter
    uptime_range: RwSignal<(f64, f64)>,
    // Loading state
    #[prop(default = false)]
    is_loading: bool,
) -> impl IntoView {
    // State for uptime filter popup
    let (show_uptime_popup, set_show_uptime_popup) = signal(false);
    // Local state for slider (only applied on "Apply" click)
    let local_uptime_range = RwSignal::new(uptime_range.get_untracked());

    // Store uptime history for mini charts
    let uptime_history: RwSignal<HashMap<String, Vec<f64>>> = RwSignal::new(HashMap::new());

    // Fetch uptime history on mount (client-side only)
    #[cfg(target_arch = "wasm32")]
    {
        let license_ids: Vec<String> = licenses.iter().map(|l| l.license_id.clone()).collect();
        if !license_ids.is_empty() {
            spawn_local(async move {
                match get_batch_uptime_history(license_ids).await {
                    Ok(history) => {
                        uptime_history.set(history);
                    }
                    Err(e) => {
                        leptos::logging::warn!("Failed to fetch uptime history: {}", e);
                    }
                }
            });
        }
    }

    // Store licenses for use in reactive Memo
    let stored_licenses = StoredValue::new(licenses);

    // Apply client-side filtering with Memo for reactivity to filter changes
    let filtered_licenses_signal = Memo::new(move |_| {
        if is_loading {
            return Vec::new();
        }

        let all_licenses = stored_licenses.get_value();
        let mp_filter = marketplace_filter.get();
        let st_filter = status_filter.get();

        all_licenses.into_iter().filter(|l| {
            // Marketplace filter (client-side)
            let passes_marketplace = match mp_filter {
                MarketplaceFilter::All => true,
                MarketplaceFilter::Uno => l.is_on_uno_marketplace,
                MarketplaceFilter::Unetwork => l.is_on_marketplace,
            };

            // Status filter (client-side)
            let passes_status = match st_filter {
                StatusFilter::All => true,
                StatusFilter::Online => l.is_online,
                StatusFilter::Offline => !l.is_online,
            };

            passes_marketplace && passes_status
        }).collect()
    });

    view! {
        <div class="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-visible">
            <table class="w-full" style="border-collapse: collapse;">
                <thead>
                    // Column headers row - equidistant layout
                    <tr class="text-xs font-medium text-slate-400 uppercase text-center" style="background: rgba(30, 41, 59, 0.5);">
                        // Toggle (+/-) column
                        <th class="py-2 px-2 w-10"></th>
                        // Checkbox column header (shown when selection mode is active)
                        <Show when=move || selection_mode.get()>
                            <th class="py-2 px-4 w-10"></th>
                        </Show>
                        // Mini chart column (no header)
                        <th class="py-2 px-2"></th>
                        <th class="py-2 px-4">"License"</th>
                        <th class="py-2 px-4">"Marketplace"</th>
                        <th class="py-2 px-4">"Uptime"</th>
                        <th class="py-2 px-4">"SPLIT (ULO:REF:UNO)%"</th>
                        <th class="py-2 px-4">"Status"</th>
                        <th class="py-2 px-4">"Actions"</th>
                    </tr>
                    // Filter row
                    <tr style="background: rgba(30, 41, 59, 0.3); border-bottom: 1px solid rgba(51, 65, 85, 0.8);">
                        // Toggle (no filter)
                        <th class="py-2 px-2"></th>
                        <Show when=move || selection_mode.get()>
                            <th class="py-2 px-4"></th>
                        </Show>
                        // Mini chart column (no filter)
                        <th class="py-2 px-2"></th>
                        // License ID filter
                        <th class="py-2 px-4">
                            <div style="display: flex; align-items: center; justify-content: center; background: #1e293b; border: 1px solid #334155; border-radius: 6px; padding: 0 6px;">
                                <span class="text-xs text-slate-500 font-mono">"0x"</span>
                                <input
                                    type="text"
                                    placeholder="Enter hex..."
                                    class="text-xs text-white/90 font-mono focus:outline-none"
                                    style="background: transparent; border: none; padding: 6px 4px; width: 80px;"
                                    prop:value=move || license_filter.get()
                                    on:input=move |ev| {
                                        license_filter.set(event_target_value(&ev));
                                    }
                                />
                            </div>
                        </th>
                        // Marketplace filter dropdown
                        <th class="py-2 px-4">
                            <select
                                class="text-xs text-white/90 focus:outline-none transition-all w-full"
                                style="background: #1e293b; border: 1px solid #334155; border-radius: 6px; padding: 6px 24px 6px 8px; appearance: none; background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2212%22 height=%2212%22 viewBox=%220 0 12 12%22%3E%3Cpath fill=%22%2394a3b8%22 d=%22M2 4l4 4 4-4%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 8px center;"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    let filter = match value.as_str() {
                                        "uno" => MarketplaceFilter::Uno,
                                        "unetwork" => MarketplaceFilter::Unetwork,
                                        _ => MarketplaceFilter::All,
                                    };
                                    marketplace_filter.set(filter);
                                }
                            >
                                <option value="all" selected=move || marketplace_filter.get() == MarketplaceFilter::All>"All"</option>
                                <option value="uno" selected=move || marketplace_filter.get() == MarketplaceFilter::Uno>"UNO"</option>
                                <option value="unetwork" selected=move || marketplace_filter.get() == MarketplaceFilter::Unetwork>"Unetwork"</option>
                            </select>
                        </th>
                        // Uptime filter with popup
                        <th class="py-2 px-4" style="position: relative; z-index: 9999;">
                            <div style="position: relative;">
                                <button
                                    style={move || {
                                        let (min, max) = uptime_range.get();
                                        let is_active = min > 0.0 || max < 100.0;
                                        format!(
                                            "display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; margin: 0 auto; background: {}; border: 1px solid {}; border-radius: 6px; cursor: pointer; transition: all 0.15s; color: {};",
                                            if is_active { "rgba(14, 165, 233, 0.2)" } else { "#1e293b" },
                                            if is_active { "#0ea5e9" } else { "#334155" },
                                            if is_active { "#22d3ee" } else { "#94a3b8" }
                                        )
                                    }}
                                    on:click=move |e| {
                                        e.stop_propagation();
                                        // Sync local state when opening
                                        local_uptime_range.set(uptime_range.get());
                                        set_show_uptime_popup.update(|v| *v = !*v);
                                    }
                                    title="Filter by Uptime"
                                >
                                    <Icon name=IconName::Filter size=14 />
                                </button>
                                // Uptime filter popup (appears above trigger)
                                <Show when=move || show_uptime_popup.get() fallback=|| ()>
                                <div
                                    style="position: absolute; bottom: 100%; left: 50%; transform: translateX(-50%); z-index: 9999; margin-bottom: 8px; background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 12px 16px; white-space: nowrap; box-shadow: 0 -10px 25px rgba(0, 0, 0, 0.4);"
                                    on:click=move |e| e.stop_propagation()
                                >
                                    // All on one row: Uptime: min% ---slider--- max% [x]
                                    <div style="display: flex; align-items: center; gap: 12px;">
                                        <span style="color: #e2e8f0; font-size: 13px; font-weight: 500;">"Uptime:"</span>
                                        <span style="color: #94a3b8; font-size: 12px; min-width: 32px; text-align: right;">
                                            {move || format!("{:.0}%", local_uptime_range.get().0)}
                                        </span>
                                        <PrimeSlider
                                            value=local_uptime_range
                                            on_change_complete=Callback::new(move |_| {
                                                // Apply to main filter when drag completes
                                                uptime_range.set(local_uptime_range.get());
                                            })
                                        />
                                        <span style="color: #94a3b8; font-size: 12px; min-width: 32px;">
                                            {move || format!("{:.0}%", local_uptime_range.get().1)}
                                        </span>
                                        <button
                                            style="background: transparent; border: none; color: #64748b; cursor: pointer; padding: 2px; display: flex; align-items: center;"
                                            on:click=move |_| set_show_uptime_popup.set(false)
                                        >
                                            <Icon name=IconName::Close size=14 />
                                        </button>
                                    </div>
                                </div>
                                </Show>
                            </div>
                        </th>
                        // Lease % (no filter)
                        <th class="py-2 px-4"></th>
                        // Status filter dropdown
                        <th class="py-2 px-4">
                            <select
                                class="text-xs text-white/90 focus:outline-none transition-all w-full"
                                style="background: #1e293b; border: 1px solid #334155; border-radius: 6px; padding: 6px 24px 6px 8px; appearance: none; background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2212%22 height=%2212%22 viewBox=%220 0 12 12%22%3E%3Cpath fill=%22%2394a3b8%22 d=%22M2 4l4 4 4-4%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 8px center;"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    let filter = match value.as_str() {
                                        "online" => StatusFilter::Online,
                                        "offline" => StatusFilter::Offline,
                                        _ => StatusFilter::All,
                                    };
                                    set_status_filter.set(filter);
                                }
                            >
                                <option value="all" selected=move || status_filter.get() == StatusFilter::All>"All"</option>
                                <option value="online" selected=move || status_filter.get() == StatusFilter::Online>"Online"</option>
                                <option value="offline" selected=move || status_filter.get() == StatusFilter::Offline>"Offline"</option>
                            </select>
                        </th>
                        // Actions (no filter)
                        <th class="py-2 px-4"></th>
                    </tr>
                </thead>
                <tbody>
                    {if is_loading {
                        // Loading state - show spinner in table body
                        view! {
                            <tr>
                                <td colspan="9" style="padding: 48px 0; text-align: center;">
                                    <div style="display: flex; flex-direction: column; align-items: center; gap: 12px;">
                                        <ProgressSpinner size=SpinnerSize::Default />
                                        <span style="color: #64748b; font-size: 14px;">"Loading licenses..."</span>
                                    </div>
                                </td>
                            </tr>
                        }.into_any()
                    } else if filtered_licenses_signal.get().is_empty() {
                        // Empty state - show message in table body
                        view! {
                            <tr>
                                <td colspan="9" style="padding: 48px 0; text-align: center;">
                                    <div style="display: flex; flex-direction: column; align-items: center; gap: 12px;">
                                        <div style="opacity: 0.5;">
                                            <Icon name=IconName::Key size=48 />
                                        </div>
                                        <span style="color: #64748b; font-size: 14px;">"No licenses found matching filters"</span>
                                    </div>
                                </td>
                            </tr>
                        }.into_any()
                    } else {
                        // Data rows
                        view! {
                            {filtered_licenses_signal.get().into_iter().enumerate().map(|(idx, license)| {
                                let id = license.license_id.clone();
                                let click = on_click.clone();
                                view! {
                                    <LicenseTableRow
                                        license=license
                                        row_index=idx
                                        on_view=move || click(id.clone())
                                        selection_mode=selection_mode
                                        selected_ids=selected_ids
                                        set_selected_ids=set_selected_ids
                                        expanded_licenses=expanded_licenses
                                        uptime_history=uptime_history
                                    />
                                }
                            }).collect::<Vec<_>>()}
                        }.into_any()
                    }}
                </tbody>
            </table>
        </div>
    }
}

/// License table row component - styled with zebra stripes like marketplace table
#[component]
fn LicenseTableRow(
    license: LicenseDto,
    row_index: usize,
    on_view: impl Fn() + Clone + Send + Sync + 'static,
    selection_mode: ReadSignal<bool>,
    selected_ids: ReadSignal<HashSet<String>>,
    set_selected_ids: WriteSignal<HashSet<String>>,
    expanded_licenses: RwSignal<HashSet<String>>,
    uptime_history: RwSignal<HashMap<String, Vec<f64>>>,
) -> impl IntoView {
    let license_id = license.license_id.clone();
    let license_id_for_toggle = license.license_id.clone();
    let license_id_for_expand = license.license_id.clone();
    let license_id_for_expand_check = license.license_id.clone();
    let license_id_for_copy = license.license_id.clone();
    let license_id_for_chart = license.license_id.clone();
    let uptime = license.uptime_percentage();
    let license_id_display = license.license_id_short();

    // Alias for inline display (optional)
    let alias_display_opt = license.alias.clone();

    // Split display (ULO:UNO or ULO:REF:UNO)
    let split_display = license.split_display();

    // Marketplace display
    let marketplace_display = if license.is_on_uno_marketplace {
        "UNO"
    } else if license.is_on_marketplace {
        "Unetwork"
    } else {
        "-"
    };

    // Status tag styling
    let status_tag = if license.is_online {
        "fx-tag-prime fx-tag-prime-green"
    } else {
        "fx-tag-prime"
    };

    let lease_share = license.lease_share_percentage;

    // Details for expanded row
    let alias_display = license.alias.clone().unwrap_or_else(|| "-".to_string());
    let node_id_display = license.node_id_short();
    let owner_display = license.owner_wallet_address.clone()
        .map(|w| format!("{}...{}", &w[..6.min(w.len())], &w[w.len().saturating_sub(4)..]))
        .unwrap_or_else(|| "-".to_string());
    let activation_display = license.activation_start_at.clone()
        .map(|s| s.split('T').next().unwrap_or(&s).to_string())
        .unwrap_or_else(|| "-".to_string());

    // Uptime color
    let uptime_color = if uptime >= 90.0 {
        "#22c55e" // green
    } else if uptime >= 75.0 {
        "#f59e0b" // amber
    } else {
        "#ef4444" // red
    };

    // Zebra stripe background
    let row_bg = if row_index % 2 == 0 {
        "background: transparent;"
    } else {
        "background: rgba(51, 65, 85, 0.15);"
    };

    // Revoke confirmation modal state
    let (show_revoke_modal, set_show_revoke_modal) = signal(false);
    let license_id_for_modal = license.license_id.clone();
    let on_view_clone = on_view.clone();

    // Check if this row is selected (using derived signal for multiple usage)
    let is_selected = {
        let id = license_id.clone();
        Signal::derive(move || selected_ids.get().contains(&id))
    };

    // Column count for colspan in expanded row
    let col_count = if selection_mode.get_untracked() { 11 } else { 10 };

    view! {
        // Main row
        <tr
            style=move || {
                if is_selected.get() {
                    format!("border-bottom: 1px solid rgba(51, 65, 85, 0.5); transition: background 0.15s; background: rgba(59, 130, 246, 0.15); {}", row_bg)
                } else {
                    format!("border-bottom: 1px solid rgba(51, 65, 85, 0.5); transition: background 0.15s; {}", row_bg)
                }
            }
            class="hover:bg-cyan-900/20"
        >
            // Toggle (+/-) column
            <td class="py-3 px-2 text-center w-10">
                <button
                    style="display: inline-flex; align-items: center; justify-content: center; width: 24px; height: 24px; background: transparent; color: #94a3b8; border: 1px solid #475569; border-radius: 4px; cursor: pointer; transition: all 0.15s; margin: 0 auto;"
                    on:click={
                        let id = license_id_for_expand.clone();
                        move |_| {
                            let id = id.clone();
                            expanded_licenses.update(move |set| {
                                if set.contains(&id) {
                                    set.remove(&id);
                                } else {
                                    set.insert(id);
                                }
                            });
                        }
                    }
                >
                    {move || if expanded_licenses.get().contains(&license_id_for_expand_check) {
                        view! { <Icon name=IconName::Minus size=12 /> }.into_any()
                    } else {
                        view! { <Icon name=IconName::Plus size=12 /> }.into_any()
                    }}
                </button>
            </td>

            // Checkbox column (shown when selection mode is active)
            <Show when=move || selection_mode.get()>
                <td class="py-3 px-4 text-center">
                    <button
                        style=move || {
                            if is_selected.get() {
                                "width: 20px; height: 20px; background: #3b82f6; border: none; border-radius: 4px; display: flex; align-items: center; justify-content: center; cursor: pointer;"
                            } else {
                                "width: 20px; height: 20px; background: transparent; border: 2px solid #64748b; border-radius: 4px; display: flex; align-items: center; justify-content: center; cursor: pointer;"
                            }
                        }
                        on:click={
                            let toggle_id = license_id_for_toggle.clone();
                            move |e: web_sys::MouseEvent| {
                                e.stop_propagation();
                                let id_to_toggle = toggle_id.clone();
                                set_selected_ids.update(move |ids| {
                                    if ids.contains(&id_to_toggle) {
                                        ids.remove(&id_to_toggle);
                                    } else {
                                        ids.insert(id_to_toggle);
                                    }
                                });
                            }
                        }
                    >
                        <Show when=move || is_selected.get()>
                            <svg class="w-3 h-3" fill="none" stroke="white" stroke-width="3" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                            </svg>
                        </Show>
                    </button>
                </td>
            </Show>

            // Mini uptime chart (first data column)
            <td class="py-3 px-2 text-center">
                {move || {
                    let history = uptime_history.get();
                    let data = history.get(&license_id_for_chart).cloned().unwrap_or_default();
                    view! { <MiniUptimeChart data=data /> }
                }}
            </td>

            // License - ID and alias (centered, stacked)
            <td class="py-3 px-4 text-center">
                <div style="display: flex; flex-direction: column; align-items: center; gap: 2px;">
                    // License ID row
                    <div style="display: flex; align-items: center; gap: 4px;">
                        <span style="font-weight: 500; color: #e2e8f0; font-family: monospace; font-size: 13px;">
                            {license_id_display}
                        </span>
                        <button
                            style="display: inline-flex; align-items: center; justify-content: center; width: 20px; height: 20px; background: transparent; color: #64748b; border: none; border-radius: 4px; cursor: pointer; transition: all 0.15s;"
                            class="hover:bg-slate-700 hover:text-slate-300"
                            title="Copy License ID"
                            on:click={
                                let id_to_copy = license_id_for_copy.clone();
                                move |e: web_sys::MouseEvent| {
                                    e.stop_propagation();
                                    let id = id_to_copy.clone();
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        use wasm_bindgen::prelude::*;
                                        #[wasm_bindgen(inline_js = "export function copy_to_clipboard(text) { navigator.clipboard.writeText(text); }")]
                                        extern "C" {
                                            fn copy_to_clipboard(text: &str);
                                        }
                                        copy_to_clipboard(&id);
                                    }
                                }
                            }
                        >
                            <Icon name=IconName::Copy size=12 />
                        </button>
                    </div>
                    // Alias row (below license ID)
                    {alias_display_opt.map(|alias| view! {
                        <span style="font-style: italic; color: #64748b; font-size: 10px;">
                            {alias}
                        </span>
                    })}
                </div>
            </td>

            // Marketplace - centered
            <td class="py-3 px-4 text-center text-slate-300">
                {marketplace_display}
            </td>

            // Uptime - centered
            <td class="py-3 px-4 text-center">
                <span style={format!("font-weight: 600; color: {};", uptime_color)}>
                    {format!("{:.1}%", uptime)}
                </span>
            </td>

            // Split % - ULO:UNO or ULO:REF:UNO
            <td class="py-3 px-4 text-center text-slate-300" style="font-size: 12px;">
                {split_display}
            </td>

            // Status - centered
            <td class="py-3 px-4 text-center">
                <span class={status_tag}>
                    {if license.is_online { "Online" } else { "Offline" }}
                </span>
            </td>

            // Actions - centered
            <td class="py-3 px-4 text-center">
                <div style="display: flex; gap: 4px; justify-content: center; align-items: center;">
                    // View button
                    <button
                        style="display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: #475569; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                        on:click=move |_| on_view_clone()
                        title="View Details"
                    >
                        <Icon name=IconName::Eye size=16 />
                    </button>
                    // Edit button
                    <button
                        style="display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                        on:click=move |_| {
                            // TODO: Implement edit functionality
                        }
                        title="Edit License"
                    >
                        <Icon name=IconName::Edit size=16 />
                    </button>
                    // Revoke button
                    <button
                        style="display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: #ef4444; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                        on:click=move |_| set_show_revoke_modal.set(true)
                        title="Revoke License"
                    >
                        <Icon name=IconName::Trash size=16 />
                    </button>
                </div>

                // Revoke confirmation modal
                <Show when=move || show_revoke_modal.get()>
                    <div
                        style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.6); display: flex; align-items: center; justify-content: center; z-index: 9999;"
                        on:click=move |_| set_show_revoke_modal.set(false)
                    >
                        <div
                            style="background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 24px; max-width: 400px; width: 90%;"
                            on:click=move |e| e.stop_propagation()
                        >
                            <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 16px;">
                                <div style="width: 40px; height: 40px; background: rgba(239, 68, 68, 0.2); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                                    <Icon name=IconName::Trash size=20 class="text-red-500".to_string() />
                                </div>
                                <h3 style="color: #f1f5f9; font-size: 18px; font-weight: 600; margin: 0;">
                                    "Revoke License"
                                </h3>
                            </div>
                            <p style="color: #94a3b8; font-size: 14px; line-height: 1.5; margin-bottom: 24px;">
                                "Are you sure you want to revoke license "
                                <span style="color: #e2e8f0; font-weight: 500; font-family: monospace;">{license_id_for_modal.clone()}</span>
                                "? This action cannot be undone."
                            </p>
                            <div style="display: flex; gap: 12px; justify-content: flex-end;">
                                <button
                                    style="padding: 8px 16px; background: transparent; color: #94a3b8; border: 1px solid #334155; border-radius: 8px; cursor: pointer;"
                                    on:click=move |_| set_show_revoke_modal.set(false)
                                >
                                    "Cancel"
                                </button>
                                <button
                                    style="padding: 8px 16px; background: #ef4444; color: white; border: none; border-radius: 8px; cursor: pointer;"
                                    on:click=move |_| {
                                        // TODO: Implement revoke functionality
                                        set_show_revoke_modal.set(false);
                                    }
                                >
                                    "Revoke"
                                </button>
                            </div>
                        </div>
                    </div>
                </Show>
            </td>
        </tr>

        // Expanded detail row (shown when license is expanded)
        {move || expanded_licenses.get().contains(&license_id).then(|| {
            view! {
                <tr style="background: rgba(30, 41, 59, 0.5);">
                    <td colspan=col_count style="padding: 0;">
                        <div style="margin: 8px 16px; padding: 16px 24px; background: rgba(15, 23, 42, 0.6); border: 1px solid rgba(51, 65, 85, 0.5); border-radius: 8px;">
                            <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px;">
                                <div>
                                    <div class="text-xs text-slate-400 mb-1">"Alias"</div>
                                    <div class="text-sm text-white">{alias_display.clone()}</div>
                                </div>
                                <div>
                                    <div class="text-xs text-slate-400 mb-1">"Node ID"</div>
                                    <div class="text-sm text-white font-mono">{node_id_display.clone()}</div>
                                </div>
                                <div>
                                    <div class="text-xs text-slate-400 mb-1">"Owner Wallet"</div>
                                    <div class="text-sm text-white font-mono">{owner_display.clone()}</div>
                                </div>
                                <div>
                                    <div class="text-xs text-slate-400 mb-1">"Activation"</div>
                                    <div class="text-sm text-white">{activation_display.clone()}</div>
                                </div>
                            </div>
                        </div>
                    </td>
                </tr>
            }
        })}
    }
}
