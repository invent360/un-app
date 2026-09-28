//! Marketplace page with tabs for Licenses, Referrers, and Visitors

use leptos::prelude::*;
use leptos::task::spawn_local;
use std::collections::HashSet;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::components::common::progress_spinner::{ProgressSpinner, SpinnerSize};
use crate::handler::{
    get_marketplace_licenses, unpublish_from_marketplace, publish_to_marketplace,
    poll_marketplace_status,
    get_uno_app_referrals, get_uno_app_visitor_stats,
    UnoAppReferral, CountryVisitorStat,
};

/// Page size options for pagination
const PAGE_SIZE_OPTIONS: [usize; 5] = [5, 10, 20, 50, 100];

/// Main tab options for the marketplace page
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MainTab {
    #[default]
    Licenses,
    Referrers,
    Visitors,
}

impl MainTab {
    fn label(&self) -> &'static str {
        match self {
            MainTab::Licenses => "Licenses",
            MainTab::Referrers => "Referrers",
            MainTab::Visitors => "Visitors",
        }
    }

    fn icon(&self) -> IconName {
        match self {
            MainTab::Licenses => IconName::Key,
            MainTab::Referrers => IconName::Users,
            MainTab::Visitors => IconName::Globe,
        }
    }
}

/// License filter options
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum LicenseFilter {
    #[default]
    All,
    UnoMarketplace,
    UnetworkMarketplace,
    Claimed,
    Available,
}

impl LicenseFilter {
    fn label(&self) -> &'static str {
        match self {
            LicenseFilter::All => "All",
            LicenseFilter::UnoMarketplace => "UNO",
            LicenseFilter::UnetworkMarketplace => "Unetwork",
            LicenseFilter::Claimed => "Claimed",
            LicenseFilter::Available => "Available",
        }
    }
}

/// Published filter options
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PublishedFilter {
    #[default]
    All,
    Unpublished,
    Published,
}

impl PublishedFilter {
    fn label(&self) -> &'static str {
        match self {
            PublishedFilter::All => "All",
            PublishedFilter::Unpublished => "Unpublished",
            PublishedFilter::Published => "Published",
        }
    }
}

/// Status filter options
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusFilter {
    #[default]
    All,
    Available,
    Claimed,
}

impl StatusFilter {
    fn label(&self) -> &'static str {
        match self {
            StatusFilter::All => "All",
            StatusFilter::Available => "Available",
            StatusFilter::Claimed => "Claimed",
        }
    }
}

/// Modal action type for licenses
#[derive(Clone, PartialEq)]
pub enum LicenseModalAction {
    Push(String),   // Push to UNO marketplace
    Reset(String),  // Reset/unpublish from marketplace
}

/// Modal action type for referrers
#[derive(Clone, PartialEq)]
pub enum ReferrerModalAction {
    Approve(i32, String),  // id, username
    Reject(i32, String),
    Suspend(i32, String),
}

/// Shorten a long ID for display
fn shorten_id(id: &str) -> String {
    if id.len() > 16 {
        format!("{}...{}", &id[..8], &id[id.len()-6..])
    } else {
        id.to_string()
    }
}

/// Copy text to clipboard using JavaScript
#[cfg(target_arch = "wasm32")]
fn copy_to_clipboard(text: &str) {
    use wasm_bindgen::JsCast;
    if let Some(window) = web_sys::window() {
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

/// Get country flag emoji from country code
fn country_flag(code: &str) -> String {
    if code.len() != 2 {
        return "🌍".to_string();
    }
    let code = code.to_uppercase();
    let first = char::from_u32(0x1F1E6 + (code.chars().next().unwrap() as u32 - 'A' as u32));
    let second = char::from_u32(0x1F1E6 + (code.chars().nth(1).unwrap() as u32 - 'A' as u32));
    match (first, second) {
        (Some(f), Some(s)) => format!("{}{}", f, s),
        _ => "🌍".to_string(),
    }
}

/// Summary card component
#[component]
fn SummaryCard<F>(
    title: &'static str,
    value: F,
    icon: IconName,
    color_class: &'static str,
) -> impl IntoView
where
    F: Fn() -> usize + Send + Sync + Clone + 'static,
{
    view! {
        <div class="flex-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
            <div class="flex items-center gap-3">
                <div class=format!("w-10 h-10 rounded-lg flex items-center justify-center {}", color_class)>
                    <Icon name=icon size=20 class="text-white".to_string() />
                </div>
                <span class="text-slate-400 text-sm">{title}</span>
                <span class="text-xl font-bold text-white ml-auto">{move || value()}</span>
            </div>
        </div>
    }
}

/// Confirmation modal component
#[component]
fn ConfirmModal<F1, F2, F3, F4>(
    show: ReadSignal<bool>,
    set_show: WriteSignal<bool>,
    title: F1,
    message: F2,
    confirm_text: F3,
    confirm_color: F4,
    on_confirm: Callback<()>,
    is_loading: ReadSignal<bool>,
) -> impl IntoView
where
    F1: Fn() -> String + Send + Sync + Clone + 'static,
    F2: Fn() -> String + Send + Sync + Clone + 'static,
    F3: Fn() -> String + Send + Sync + Clone + 'static,
    F4: Fn() -> &'static str + Send + Sync + Clone + 'static,
{
    view! {
        <Show when=move || show.get()>
            <div
                style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.7); z-index: 100; display: flex; align-items: center; justify-content: center;"
                on:click=move |_| set_show.set(false)
            >
                <div
                    style="background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 24px; min-width: 360px; max-width: 90vw;"
                    on:click=move |e| e.stop_propagation()
                >
                    <h3 style="font-size: 18px; font-weight: 600; color: #e2e8f0; margin-bottom: 12px;">
                        {title.clone()}
                    </h3>
                    <p style="font-size: 14px; color: #94a3b8; margin-bottom: 20px;">
                        {message.clone()}
                    </p>
                    <div style="display: flex; gap: 8px; justify-content: flex-end;">
                        <button
                            style="padding: 10px 20px; background: #475569; color: #e2e8f0; border: none; border-radius: 8px; cursor: pointer; font-size: 14px;"
                            on:click=move |_| set_show.set(false)
                            disabled=move || is_loading.get()
                        >
                            "Cancel"
                        </button>
                        <button
                            style={let color = confirm_color.clone(); move || format!("padding: 10px 20px; background: {}; color: white; border: none; border-radius: 8px; cursor: pointer; font-size: 14px; font-weight: 500; display: flex; align-items: center; gap: 6px;", color())}
                            on:click=move |_| on_confirm.run(())
                            disabled=move || is_loading.get()
                        >
                            {let ct = confirm_text.clone(); move || if is_loading.get() {
                                view! { <ProgressSpinner size=SpinnerSize::Small /> }.into_any()
                            } else {
                                view! { <span>{ct()}</span> }.into_any()
                            }}
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

/// Licenses tab content
#[component]
fn LicensesTab() -> impl IntoView {
    let active_filter = RwSignal::new(LicenseFilter::All);
    let selected_ids = RwSignal::new(HashSet::<String>::new());
    let select_all = RwSignal::new(false);
    let selection_mode = RwSignal::new(false);
    let is_publishing = RwSignal::new(false);
    let is_unpublishing = RwSignal::new(false);
    let is_refreshing = RwSignal::new(false);

    // Expanded rows state
    let expanded_licenses = RwSignal::new(HashSet::<String>::new());

    // Pagination state
    let current_page = RwSignal::new(1usize);
    let page_size = RwSignal::new(5usize);

    // Search state
    let search_query = RwSignal::new(String::new());
    let is_search_active = RwSignal::new(false);
    let searching = RwSignal::new(false);

    // Filter state for License ID, Referral, Published, Status
    let license_id_filter = RwSignal::new(String::new());
    let referral_filter = RwSignal::new(String::new());
    let published_filter = RwSignal::new(PublishedFilter::default());
    let status_filter = RwSignal::new(StatusFilter::default());

    // Modal state for single row actions
    let (show_modal, set_show_modal) = signal(false);
    let (modal_action, set_modal_action) = signal(Option::<LicenseModalAction>::None);
    let (modal_loading, set_modal_loading) = signal(false);

    // Loading state tracking for showing overlay spinner during filter changes
    let show_loading_overlay = RwSignal::new(false);
    let prev_filter_deps = RwSignal::new((
        String::new(),           // license_id_filter
        String::new(),           // referral_filter
        PublishedFilter::All,    // published_filter
        StatusFilter::All,       // status_filter
        LicenseFilter::All,      // active_filter
        false,                   // is_search_active
        String::new(),           // search_query
    ));

    let licenses = LocalResource::new(
        move || get_marketplace_licenses()
    );

    let stats = Memo::new(move |_| {
        let all = licenses.get().and_then(|r| r.ok()).unwrap_or_default();
        let total = all.len();
        let claimed = all.iter().filter(|l| l.marketplace_status.as_deref() == Some("claimed")).count();
        let available = total.saturating_sub(claimed);
        (total, claimed, available)
    });

    let filtered_licenses = Memo::new(move |_| {
        let all = licenses.get().and_then(|r| r.ok()).unwrap_or_default();
        let filter = active_filter.get();
        let query = search_query.get().to_lowercase();

        // Get new filter values
        let license_id_query = license_id_filter.get().to_lowercase();
        let referral_query = referral_filter.get().to_lowercase();
        let pub_filter = published_filter.get();
        let stat_filter = status_filter.get();

        all.into_iter().filter(|l| {
            let passes_filter = match filter {
                LicenseFilter::All => true,
                LicenseFilter::UnoMarketplace => l.is_on_uno_marketplace,
                LicenseFilter::UnetworkMarketplace => l.is_on_marketplace,
                LicenseFilter::Claimed => l.marketplace_status.as_deref() == Some("claimed"),
                LicenseFilter::Available => l.marketplace_status.as_deref() != Some("claimed"),
            };

            let passes_search = if is_search_active.get() && !query.is_empty() {
                l.license_id.to_lowercase().contains(&query)
            } else {
                true
            };

            // License ID filter (prepend "0x" to user input)
            let passes_license_id = if !license_id_query.is_empty() {
                let search_with_prefix = format!("0x{}", license_id_query);
                l.license_id.to_lowercase().contains(&search_with_prefix)
            } else {
                true
            };

            // Referral filter
            let passes_referral = if !referral_query.is_empty() {
                l.marketplace_referral_code
                    .as_ref()
                    .map(|r| r.to_lowercase().contains(&referral_query))
                    .unwrap_or(false)
            } else {
                true
            };

            // Published filter
            let passes_published = match pub_filter {
                PublishedFilter::All => true,
                PublishedFilter::Published => l.is_published,
                PublishedFilter::Unpublished => !l.is_published,
            };

            // Status filter
            let passes_status = match stat_filter {
                StatusFilter::All => true,
                StatusFilter::Available => l.marketplace_status.as_deref() != Some("claimed"),
                StatusFilter::Claimed => l.marketplace_status.as_deref() == Some("claimed"),
            };

            passes_filter && passes_search && passes_license_id && passes_referral && passes_published && passes_status
        }).collect::<Vec<_>>()
    });

    // Reset page when filter/search changes
    Effect::new(move |_| {
        let _ = active_filter.get();
        let _ = is_search_active.get();
        let _ = license_id_filter.get();
        let _ = referral_filter.get();
        let _ = published_filter.get();
        let _ = status_filter.get();
        current_page.set(1);
    });

    // Track filter changes and show loading overlay with minimum duration
    Effect::new(move |_| {
        let current_deps = (
            license_id_filter.get(),
            referral_filter.get(),
            published_filter.get(),
            status_filter.get(),
            active_filter.get(),
            is_search_active.get(),
            search_query.get(),
        );
        let prev = prev_filter_deps.get_untracked();
        if current_deps != prev {
            prev_filter_deps.set(current_deps);
            show_loading_overlay.set(true);
            // Hide after minimum display duration (200ms) for visual feedback
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(200).await;
                show_loading_overlay.set(false);
            });
        }
    });

    Effect::new(move |_| {
        if select_all.get() && selection_mode.get() {
            let ids: HashSet<String> = filtered_licenses.get().iter().map(|l| l.license_id.clone()).collect();
            selected_ids.set(ids);
        }
    });

    let toggle_select = move |id: String| {
        selected_ids.update(|set| {
            if set.contains(&id) { set.remove(&id); } else { set.insert(id); }
        });
    };

    let toggle_expand = move |id: String| {
        expanded_licenses.update(|set| {
            if set.contains(&id) { set.remove(&id); } else { set.insert(id); }
        });
    };

    // Computed: Check if selected licenses are all published, all unpublished, or mixed
    // Returns (can_publish, can_unpublish) - both false means mixed selection
    let selection_state = Memo::new(move |_| {
        let ids = selected_ids.get();
        if ids.is_empty() {
            return (false, false); // No selection
        }

        let all_licenses = licenses.get().and_then(|r| r.ok()).unwrap_or_default();
        let selected_licenses: Vec<_> = all_licenses.iter()
            .filter(|l| ids.contains(&l.license_id))
            .collect();

        if selected_licenses.is_empty() {
            return (false, false);
        }

        let all_published = selected_licenses.iter().all(|l| l.is_published);
        let all_unpublished = selected_licenses.iter().all(|l| !l.is_published);
        let any_claimed = selected_licenses.iter().any(|l| l.marketplace_status.as_deref() == Some("claimed"));

        // Can publish only if all are unpublished and none are claimed
        let can_publish = all_unpublished && !any_claimed;
        // Can unpublish only if all are published and none are claimed
        let can_unpublish = all_published && !any_claimed;

        (can_publish, can_unpublish)
    });

    let do_publish = move |_| {
        let ids: Vec<String> = selected_ids.get().iter().cloned().collect();
        if ids.is_empty() { return; }
        is_publishing.set(true);
        spawn_local(async move {
            match publish_to_marketplace(ids).await {
                Ok(_) => { selected_ids.set(HashSet::new()); select_all.set(false); licenses.refetch(); }
                Err(e) => web_sys::console::error_1(&format!("Failed: {:?}", e).into()),
            }
            is_publishing.set(false);
        });
    };

    let do_unpublish = move |_| {
        let ids: Vec<String> = selected_ids.get().iter().cloned().collect();
        if ids.is_empty() { return; }
        is_unpublishing.set(true);
        spawn_local(async move {
            match unpublish_from_marketplace(ids).await {
                Ok(_) => { selected_ids.set(HashSet::new()); select_all.set(false); licenses.refetch(); }
                Err(e) => web_sys::console::error_1(&format!("Failed: {:?}", e).into()),
            }
            is_unpublishing.set(false);
        });
    };

    let do_refresh = move |_| {
        is_refreshing.set(true);
        spawn_local(async move {
            match poll_marketplace_status().await {
                Ok(result) => {
                    web_sys::console::log_1(&format!("Refreshed: fetched {}, updated {}", result.fetched, result.updated).into());
                    licenses.refetch();
                }
                Err(e) => web_sys::console::error_1(&format!("Failed to refresh: {:?}", e).into()),
            }
            is_refreshing.set(false);
        });
    };

    let do_search = move |_| {
        if !search_query.get().is_empty() {
            is_search_active.set(true);
        }
    };

    let clear_search = move |_| {
        search_query.set(String::new());
        is_search_active.set(false);
    };

    // Handle modal confirm
    let handle_modal_confirm = Callback::new(move |_: ()| {
        if let Some(action) = modal_action.get() {
            set_modal_loading.set(true);
            match action {
                LicenseModalAction::Push(id) => {
                    spawn_local(async move {
                        match publish_to_marketplace(vec![id]).await {
                            Ok(_) => { licenses.refetch(); }
                            Err(e) => web_sys::console::error_1(&format!("Failed: {:?}", e).into()),
                        }
                        set_modal_loading.set(false);
                        set_show_modal.set(false);
                        set_modal_action.set(None);
                    });
                }
                LicenseModalAction::Reset(id) => {
                    spawn_local(async move {
                        match unpublish_from_marketplace(vec![id]).await {
                            Ok(_) => { licenses.refetch(); }
                            Err(e) => web_sys::console::error_1(&format!("Failed: {:?}", e).into()),
                        }
                        set_modal_loading.set(false);
                        set_show_modal.set(false);
                        set_modal_action.set(None);
                    });
                }
            }
        }
    });

    // Modal info based on action
    let modal_title = Memo::new(move |_| {
        match modal_action.get() {
            Some(LicenseModalAction::Push(_)) => "Push to Marketplace".to_string(),
            Some(LicenseModalAction::Reset(_)) => "Remove from Marketplace".to_string(),
            None => "".to_string(),
        }
    });

    let modal_message = Memo::new(move |_| {
        match modal_action.get() {
            Some(LicenseModalAction::Push(ref id)) => format!("Are you sure you want to push license {} to the UNO marketplace?", shorten_id(id)),
            Some(LicenseModalAction::Reset(ref id)) => format!("Are you sure you want to remove license {} from the marketplace and reset to default settings?", shorten_id(id)),
            None => "".to_string(),
        }
    });

    let modal_confirm_text = Memo::new(move |_| {
        match modal_action.get() {
            Some(LicenseModalAction::Push(_)) => "Confirm".to_string(),
            Some(LicenseModalAction::Reset(_)) => "Confirm".to_string(),
            None => "Confirm".to_string(),
        }
    });

    let modal_color = move || {
        match modal_action.get() {
            Some(LicenseModalAction::Push(_)) => "#22c55e",
            Some(LicenseModalAction::Reset(_)) => "#ef4444",
            None => "#3b82f6",
        }
    };

    view! {
        <div class="space-y-4">
            // Confirmation Modal
            <ConfirmModal
                show=show_modal
                set_show=set_show_modal
                title=move || modal_title.get()
                message=move || modal_message.get()
                confirm_text=move || modal_confirm_text.get()
                confirm_color=modal_color
                on_confirm=handle_modal_confirm
                is_loading=modal_loading
            />

            // Stats Container
            <div class="rounded-2xl" style="background: #0f172a; padding: 2px 4px;">
                <div style="display: flex; flex-direction: row; gap: 1rem; flex-wrap: wrap;">
                    <div style="flex: 1; min-width: 200px;">
                        <SummaryCard title="Total on Marketplace" value=move || stats.get().0 icon=IconName::Globe color_class="bg-blue-500/20" />
                    </div>
                    <div style="flex: 1; min-width: 200px;">
                        <SummaryCard title="Claimed" value=move || stats.get().1 icon=IconName::CheckCircle color_class="bg-green-500/20" />
                    </div>
                    <div style="flex: 1; min-width: 200px;">
                        <SummaryCard title="Available" value=move || stats.get().2 icon=IconName::Key color_class="bg-purple-500/20" />
                    </div>
                </div>
            </div>

            // Top Action Panel
            <div class="rounded-2xl" style="background: #0f172a; border: 1px solid #1e293b; padding: 8px 12px;">
                <div class="flex flex-wrap items-center justify-between gap-3">
                    // Group 1: Selection toggle and bulk actions (left)
                    <div class="flex items-center gap-2 flex-shrink-0">
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
                                selection_mode.set(new_mode);
                                if !new_mode {
                                    selected_ids.set(HashSet::new());
                                    select_all.set(false);
                                }
                            }
                        >
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

                        <Show when=move || !selected_ids.get().is_empty()>
                            <span style="color: #94a3b8; font-size: 14px;">
                                {move || format!("{} selected", selected_ids.get().len())}
                            </span>
                        </Show>

                        // Publish button - enabled only when all selected are unpublished
                        <button
                            style=move || {
                                let (can_publish, _) = selection_state.get();
                                if !can_publish {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; color: #64748b; cursor: not-allowed; opacity: 0.5;"
                                } else {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #22c55e; border: none; border-radius: 8px; color: white; cursor: pointer; transition: all 0.15s;"
                                }
                            }
                            disabled=move || { let (can_publish, _) = selection_state.get(); !can_publish || is_publishing.get() }
                            on:click=do_publish
                            title=move || {
                                let (can_publish, can_unpublish) = selection_state.get();
                                if selected_ids.get().is_empty() {
                                    "No licenses selected"
                                } else if !can_publish && !can_unpublish {
                                    "Mixed selection: select only published or only unpublished licenses"
                                } else if !can_publish {
                                    "Selected licenses are already published"
                                } else {
                                    "Publish selected licenses"
                                }
                            }
                        >
                            {move || if is_publishing.get() {
                                view! { <ProgressSpinner size=SpinnerSize::Small /> }.into_any()
                            } else {
                                view! { <Icon name=IconName::Upload size=18 /> }.into_any()
                            }}
                        </button>

                        // Unpublish button - enabled only when all selected are published
                        <button
                            style=move || {
                                let (_, can_unpublish) = selection_state.get();
                                if !can_unpublish {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; color: #64748b; cursor: not-allowed; opacity: 0.5;"
                                } else {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #ef4444; border: none; border-radius: 8px; color: white; cursor: pointer; transition: all 0.15s;"
                                }
                            }
                            disabled=move || { let (_, can_unpublish) = selection_state.get(); !can_unpublish || is_unpublishing.get() }
                            on:click=do_unpublish
                            title=move || {
                                let (can_publish, can_unpublish) = selection_state.get();
                                if selected_ids.get().is_empty() {
                                    "No licenses selected"
                                } else if !can_publish && !can_unpublish {
                                    "Mixed selection: select only published or only unpublished licenses"
                                } else if !can_unpublish {
                                    "Selected licenses are not published"
                                } else {
                                    "Unpublish selected licenses"
                                }
                            }
                        >
                            {move || if is_unpublishing.get() {
                                view! { <ProgressSpinner size=SpinnerSize::Small /> }.into_any()
                            } else {
                                view! { <Icon name=IconName::Download size=18 /> }.into_any()
                            }}
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
                            on:input=move |ev| search_query.set(event_target_value(&ev))
                            on:keydown=move |ev: web_sys::KeyboardEvent| {
                                if ev.key() == "Enter" && !search_query.get().is_empty() {
                                    is_search_active.set(true);
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
                            on:click=do_search
                        >
                            "Go"
                        </button>
                        <Show when=move || is_search_active.get()>
                            <button
                                style="display: flex; align-items: center; justify-content: center; padding: 4px 8px; background: #ef4444; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 12px; font-weight: 500;"
                                on:click=clear_search
                                title="Clear Search"
                            >
                                "×"
                            </button>
                        </Show>
                    </div>

                    // Group 3: Refresh and Publish buttons (right)
                    <div class="flex items-center gap-2 flex-shrink-0">
                        // Refresh button - polls marketplace for claimed status
                        <button
                            style=move || {
                                if is_refreshing.get() {
                                    "display: flex; align-items: center; gap: 6px; padding: 8px 14px; background: #0891b2; border: none; border-radius: 8px; color: white; cursor: wait; font-size: 14px; opacity: 0.7;"
                                } else {
                                    "display: flex; align-items: center; gap: 6px; padding: 8px 14px; background: #0891b2; border: none; border-radius: 8px; color: white; cursor: pointer; font-size: 14px; transition: all 0.15s;"
                                }
                            }
                            disabled=move || is_refreshing.get()
                            on:click=do_refresh
                            title="Refresh status from marketplace"
                        >
                            {move || if is_refreshing.get() {
                                view! { <ProgressSpinner size=SpinnerSize::Small /> }.into_any()
                            } else {
                                view! { <Icon name=IconName::Refresh size=16 /> }.into_any()
                            }}
                            "Refresh"
                        </button>
                    </div>
                </div>
            </div>

            // License Table with Pagination
            <div style="position: relative;">
                // CSS-based loading overlay (avoids hydration errors)
                <div
                    style=move || format!(
                        "position: absolute; inset: 0; background: rgba(15, 23, 42, 0.7); display: flex; align-items: center; justify-content: center; z-index: 10; border-radius: 12px; pointer-events: none; transition: opacity 0.15s; opacity: {}; visibility: {};",
                        if show_loading_overlay.get() { "1" } else { "0" },
                        if show_loading_overlay.get() { "visible" } else { "hidden" }
                    )
                >
                    <ProgressSpinner size=SpinnerSize::Default />
                </div>
                <Transition fallback=move || view! { <div class="flex items-center justify-center p-12"><ProgressSpinner size=SpinnerSize::Large /></div> }>
                {move || {
                    let list = filtered_licenses.get();
                    let total_items = list.len();
                    let page = current_page.get();
                    let size = page_size.get();
                    let start = (page - 1) * size;
                    let end = std::cmp::min(start + size, total_items);
                    let paginated_list: Vec<_> = if total_items > 0 { list[start..end].to_vec() } else { vec![] };
                    let total_pages = if total_items == 0 { 1 } else { (total_items + size - 1) / size };

                    view! {
                        <div class="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden">
                            <table class="w-full">
                                <thead>
                                        // Column headers row
                                        <tr class="text-xs font-medium text-slate-400 uppercase text-center" style="background: rgba(30, 41, 59, 0.5);">
                                            <th class="py-2 px-2 w-10"></th>
                                            <Show when=move || selection_mode.get()>
                                                <th class="py-2 px-4">
                                                    <input type="checkbox" class="w-4 h-4 rounded border-slate-600 bg-slate-700 text-cyan-500" prop:checked=move || select_all.get() on:change=move |_| { let v = !select_all.get(); select_all.set(v); if !v { selected_ids.set(HashSet::new()); } } />
                                                </th>
                                            </Show>
                                            <th class="py-2 px-4">"License ID"</th>
                                            <th class="py-2 px-4">"Published"</th>
                                            <th class="py-2 px-4">"Status"</th>
                                            <th class="py-2 px-4">"Referral"</th>
                                            <th class="py-2 px-4">"Actions"</th>
                                        </tr>
                                        // Filter row
                                        <tr style="background: rgba(30, 41, 59, 0.3); border-bottom: 1px solid rgba(51, 65, 85, 0.8);">
                                            <th class="py-2 px-2"></th>
                                            <Show when=move || selection_mode.get()>
                                                <th class="py-2 px-4"></th>
                                            </Show>
                                            // License ID filter
                                            <th class="py-2 px-4">
                                                <div style="display: flex; align-items: center; justify-content: center; background: #1e293b; border: 1px solid #334155; border-radius: 6px; padding: 0 6px;">
                                                    <span class="text-xs text-slate-500 font-mono">"0x"</span>
                                                    <input
                                                        type="text"
                                                        placeholder="Enter hex..."
                                                        class="text-xs text-white/90 font-mono focus:outline-none"
                                                        style="background: transparent; border: none; padding: 6px 4px; width: 80px;"
                                                        prop:value=move || license_id_filter.get()
                                                        on:input=move |ev| {
                                                            license_id_filter.set(event_target_value(&ev));
                                                        }
                                                    />
                                                </div>
                                            </th>
                                            // Published filter
                                            <th class="py-2 px-4">
                                                <select
                                                    class="text-xs text-white/90 focus:outline-none transition-all w-full"
                                                    style="background: #1e293b; border: 1px solid #334155; border-radius: 6px; padding: 6px 24px 6px 8px; appearance: none; background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2212%22 height=%2212%22 viewBox=%220 0 12 12%22%3E%3Cpath fill=%22%2394a3b8%22 d=%22M2 4l4 4 4-4%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 8px center;"
                                                    on:change=move |ev| {
                                                        let value = event_target_value(&ev);
                                                        let filter = match value.as_str() {
                                                            "published" => PublishedFilter::Published,
                                                            "unpublished" => PublishedFilter::Unpublished,
                                                            _ => PublishedFilter::All,
                                                        };
                                                        published_filter.set(filter);
                                                    }
                                                >
                                                    <option value="all">"All"</option>
                                                    <option value="unpublished">"Unpublished"</option>
                                                    <option value="published">"Published"</option>
                                                </select>
                                            </th>
                                            // Status filter
                                            <th class="py-2 px-4">
                                                <select
                                                    class="text-xs text-white/90 focus:outline-none transition-all w-full"
                                                    style="background: #1e293b; border: 1px solid #334155; border-radius: 6px; padding: 6px 24px 6px 8px; appearance: none; background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2212%22 height=%2212%22 viewBox=%220 0 12 12%22%3E%3Cpath fill=%22%2394a3b8%22 d=%22M2 4l4 4 4-4%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 8px center;"
                                                    on:change=move |ev| {
                                                        let value = event_target_value(&ev);
                                                        let filter = match value.as_str() {
                                                            "available" => StatusFilter::Available,
                                                            "claimed" => StatusFilter::Claimed,
                                                            _ => StatusFilter::All,
                                                        };
                                                        status_filter.set(filter);
                                                    }
                                                >
                                                    <option value="all">"All"</option>
                                                    <option value="available">"Available"</option>
                                                    <option value="claimed">"Claimed"</option>
                                                </select>
                                            </th>
                                            // Referral filter
                                            <th class="py-2 px-4">
                                                <input
                                                    type="text"
                                                    placeholder="Search..."
                                                    class="text-xs text-white/90 focus:outline-none transition-all w-full"
                                                    style="background: #1e293b; border: 1px solid #334155; border-radius: 6px; padding: 6px 8px;"
                                                    prop:value=move || referral_filter.get()
                                                    on:input=move |ev| {
                                                        referral_filter.set(event_target_value(&ev));
                                                    }
                                                />
                                            </th>
                                            // Actions (no filter)
                                            <th class="py-2 px-4"></th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {if paginated_list.is_empty() {
                                            view! {
                                                <tr>
                                                    <td colspan="7" class="py-12 text-center">
                                                        <div class="flex flex-col items-center justify-center">
                                                            <Icon name=IconName::Search size=32 class="text-slate-600 mb-3".to_string() />
                                                            <p class="text-slate-400 text-sm">"No licenses match the current filters"</p>
                                                        </div>
                                                    </td>
                                                </tr>
                                            }.into_any()
                                        } else {
                                            paginated_list.into_iter().enumerate().map(|(idx, license)| {
                                            let id = license.license_id.clone();
                                            let id_toggle = id.clone();
                                            let id_check = id.clone();
                                            let id_push = id.clone();
                                            let id_reset = id.clone();
                                            let id_expand = id.clone();
                                            let id_expand_check = id.clone();
                                            let id_copy = id.clone();
                                            let id_view = license.id.clone();
                                            let is_claimed = license.marketplace_status.as_deref() == Some("claimed");
                                            let referral = license.marketplace_referral_code.clone().unwrap_or("-".to_string());
                                            let is_published = license.is_published;
                                            let row_bg = if idx % 2 == 0 { "background: transparent;" } else { "background: rgba(51, 65, 85, 0.15);" };
                                            // Store license data for expanded view
                                            let alias = license.alias.clone().unwrap_or("-".to_string());
                                            let agent_id = license.agent_id.clone();
                                            let ulo_name = license.ulo_name.clone();
                                            let uptime = license.uptime;
                                            let col_count = if selection_mode.get_untracked() { 8 } else { 7 };
                                            view! {
                                                // Main row
                                                <tr style=row_bg class="hover:bg-cyan-900/20 transition-colors text-center">
                                                    // Expand/collapse button - centered
                                                    <td style="padding: 8px; text-align: center; vertical-align: middle;">
                                                        <button
                                                            style="display: inline-flex; align-items: center; justify-content: center; width: 24px; height: 24px; background: transparent; color: #94a3b8; border: 1px solid #475569; border-radius: 4px; cursor: pointer; transition: all 0.15s; margin: 0 auto;"
                                                            title="Toggle details"
                                                            on:click={
                                                                let id = id_expand.clone();
                                                                move |_| toggle_expand(id.clone())
                                                            }
                                                        >
                                                            {move || if expanded_licenses.get().contains(&id_expand_check) {
                                                                view! { <Icon name=IconName::Minus size=12 /> }.into_any()
                                                            } else {
                                                                view! { <Icon name=IconName::Plus size=12 /> }.into_any()
                                                            }}
                                                        </button>
                                                    </td>
                                                    <Show when=move || selection_mode.get()>
                                                        <td class="py-2 px-4"><input type="checkbox" class="w-4 h-4 rounded border-slate-600 bg-slate-700 text-cyan-500" prop:checked={let id_c = id_check.clone(); move || selected_ids.get().contains(&id_c)} on:change={let id_t = id_toggle.clone(); move |_| toggle_select(id_t.clone())} /></td>
                                                    </Show>
                                                    <td class="py-2 px-4">
                                                        <div style="display: flex; align-items: center; justify-content: center; gap: 8px;">
                                                            <span class="font-mono text-sm text-cyan-400">{shorten_id(&id)}</span>
                                                            <button
                                                                style="display: flex; align-items: center; justify-content: center; padding: 4px; background: transparent; border: none; color: #64748b; cursor: pointer; border-radius: 4px; transition: all 0.15s;"
                                                                title="Copy License ID"
                                                                on:click={
                                                                    let id = id_copy.clone();
                                                                    move |_| copy_to_clipboard(&id)
                                                                }
                                                            >
                                                                <Icon name=IconName::Copy size=14 />
                                                            </button>
                                                        </div>
                                                    </td>
                                                    // Published column - bright green tick when published, white/gray tick when not
                                                    <td class="py-2 px-4" style="text-align: center;">
                                                        {if is_published {
                                                            view! { <span style="display: inline-flex; align-items: center; justify-content: center; color: #22c55e;"><Icon name=IconName::CheckCircle size=18 /></span> }.into_any()
                                                        } else {
                                                            view! { <span style="display: inline-flex; align-items: center; justify-content: center; color: #cbd5e1;"><Icon name=IconName::CheckCircle size=18 /></span> }.into_any()
                                                        }}
                                                    </td>
                                                    <td class="py-2 px-4">
                                                        {if is_claimed { view! { <span style="display: inline-flex; align-items: center; gap: 6px; padding: 2px 8px; border-radius: 9999px; font-size: 12px; font-weight: 500; background: rgba(34, 197, 94, 0.2); color: #4ade80;"><Icon name=IconName::CheckCircle size=12 />"Claimed"</span> }.into_any() }
                                                        else { view! { <span style="display: inline-flex; align-items: center; gap: 6px; padding: 2px 8px; border-radius: 9999px; font-size: 12px; font-weight: 500; background: rgba(234, 179, 8, 0.2); color: #facc15;"><Icon name=IconName::Clock size=12 />"Available"</span> }.into_any() }}
                                                    </td>
                                                    <td class="py-2 px-4"><span class="font-mono text-sm text-slate-400">{referral.clone()}</span></td>
                                                    <td class="py-2 px-4">
                                                        <div style="display: flex; gap: 4px; justify-content: center; align-items: center;">
                                                            // Push to marketplace button (disabled if already published or claimed)
                                                            <button
                                                                style={if is_published || is_claimed {
                                                                    "display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #475569; color: #94a3b8; border: none; border-radius: 6px; cursor: not-allowed; opacity: 0.5;"
                                                                } else {
                                                                    "display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #22c55e; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                                                                }}
                                                                title={if is_claimed { "License is claimed" } else if is_published { "Already published" } else { "Push to UNO Marketplace" }}
                                                                disabled={is_published || is_claimed}
                                                                on:click={
                                                                    let id = id_push.clone();
                                                                    move |_| {
                                                                        if !is_published && !is_claimed {
                                                                            set_modal_action.set(Some(LicenseModalAction::Push(id.clone())));
                                                                            set_show_modal.set(true);
                                                                        }
                                                                    }
                                                                }
                                                            >
                                                                <Icon name=IconName::Upload size=14 />
                                                            </button>
                                                            // Unpublish from marketplace button (disabled if not published or claimed)
                                                            <button
                                                                style={if !is_published || is_claimed {
                                                                    "display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #475569; color: #94a3b8; border: none; border-radius: 6px; cursor: not-allowed; opacity: 0.5;"
                                                                } else {
                                                                    "display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #ef4444; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                                                                }}
                                                                title={if is_claimed { "Cannot unpublish claimed license" } else if !is_published { "Not published" } else { "Unpublish from Marketplace" }}
                                                                disabled={!is_published || is_claimed}
                                                                on:click={
                                                                    let id = id_reset.clone();
                                                                    move |_| {
                                                                        if is_published && !is_claimed {
                                                                            set_modal_action.set(Some(LicenseModalAction::Reset(id.clone())));
                                                                            set_show_modal.set(true);
                                                                        }
                                                                    }
                                                                }
                                                            >
                                                                <Icon name=IconName::Download size=14 />
                                                            </button>
                                                        </div>
                                                    </td>
                                                </tr>
                                                // Expanded details row
                                                {move || expanded_licenses.get().contains(&id).then(|| {
                                                    let id_view_clone = id_view.clone();
                                                    let alias_clone = alias.clone();
                                                    let agent_id_clone = agent_id.clone();
                                                    let ulo_name_clone = ulo_name.clone();
                                                    let referral_clone = referral.clone();
                                                    view! {
                                                        <tr style="background: rgba(30, 41, 59, 0.5);">
                                                            <td colspan=col_count style="padding: 0;">
                                                                <div style="margin: 8px 16px; padding: 16px 24px; background: rgba(15, 23, 42, 0.6); border: 1px solid rgba(51, 65, 85, 0.5); border-radius: 8px;">
                                                                    <div class="flex items-start gap-8">
                                                                        // License details
                                                                        <div class="flex-1 grid grid-cols-2 gap-x-12 gap-y-4 text-sm">
                                                                            <div class="flex items-center gap-3">
                                                                                <span class="text-slate-500 min-w-[100px]">"Alias:"</span>
                                                                                <span class="text-white font-medium">{alias_clone}</span>
                                                                            </div>
                                                                            <div class="flex items-center gap-3">
                                                                                <span class="text-slate-500 min-w-[100px]">"Agent:"</span>
                                                                                <span class="text-slate-300 font-mono text-xs">{shorten_id(&agent_id_clone)}</span>
                                                                            </div>
                                                                            <div class="flex items-center gap-3">
                                                                                <span class="text-slate-500 min-w-[100px]">"ULO:"</span>
                                                                                <span class="text-slate-300">{ulo_name_clone}</span>
                                                                            </div>
                                                                            <div class="flex items-center gap-3">
                                                                                <span class="text-slate-500 min-w-[100px]">"Uptime:"</span>
                                                                                <span class="text-slate-300">{format!("{:.1}%", uptime)}</span>
                                                                            </div>
                                                                            <div class="flex items-center gap-3">
                                                                                <span class="text-slate-500 min-w-[100px]">"Referral Code:"</span>
                                                                                <span class="text-slate-300 font-mono">{referral_clone}</span>
                                                                            </div>
                                                                            <div class="flex items-center gap-3">
                                                                                <span class="text-slate-500 min-w-[100px]">"Status:"</span>
                                                                                {if is_claimed {
                                                                                    view! { <span class="text-green-400 font-medium">"Claimed"</span> }.into_any()
                                                                                } else {
                                                                                    view! { <span class="text-yellow-400 font-medium">"Available"</span> }.into_any()
                                                                                }}
                                                                            </div>
                                                                        </div>
                                                                        // View button
                                                                        <a
                                                                            href=format!("/licenses/{}", id_view_clone)
                                                                            style="display: inline-flex; align-items: center; gap: 8px; padding: 10px 20px; background: #0891b2; color: white; border-radius: 8px; text-decoration: none; font-size: 14px; font-weight: 500; transition: background 0.15s; white-space: nowrap;"
                                                                        >
                                                                            <Icon name=IconName::Eye size=16 />
                                                                            "View License"
                                                                        </a>
                                                                    </div>
                                                                </div>
                                                            </td>
                                                        </tr>
                                                    }
                                                })}
                                            }
                                        }).collect::<Vec<_>>().into_any()
                                        }}
                                    </tbody>
                                </table>

                                // Pagination controls
                                <div class="flex items-center justify-between px-4 py-2 border-t border-slate-700/50">
                                    <div class="flex items-center gap-3">
                                        <span class="text-slate-400 text-sm">
                                            {format!("Page {} of {} ({} total)", page, total_pages, total_items)}
                                        </span>
                                    </div>
                                    <div class="flex items-center gap-3">
                                        <select
                                            style="padding: 5px 8px; background: #334155; border: 1px solid #475569; border-radius: 6px; color: #e2e8f0; font-size: 13px; cursor: pointer; outline: none;"
                                            on:change=move |ev| {
                                                if let Ok(size) = event_target_value(&ev).parse::<usize>() {
                                                    page_size.set(size);
                                                    current_page.set(1);
                                                }
                                            }
                                        >
                                            {PAGE_SIZE_OPTIONS.iter().map(|&s| {
                                                view! {
                                                    <option value=s.to_string() selected=move || page_size.get() == s>
                                                        {s}
                                                    </option>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </select>
                                        <div class="flex gap-2">
                                            <button
                                                style="padding: 6px 14px; background: #06b6d4; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 500; transition: opacity 0.15s;"
                                                disabled=move || current_page.get() <= 1
                                                on:click=move |_| current_page.update(|p| *p = (*p).saturating_sub(1).max(1))
                                                class="disabled:opacity-40 disabled:cursor-not-allowed"
                                            >
                                                "Back"
                                            </button>
                                            <button
                                                style="padding: 6px 14px; background: #06b6d4; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 500; transition: opacity 0.15s;"
                                                disabled=move || {
                                                    let size = page_size.get();
                                                    let total = filtered_licenses.get().len();
                                                    let tp = (total + size - 1) / size;
                                                    let tp = if tp == 0 { 1 } else { tp };
                                                    current_page.get() >= tp
                                                }
                                                on:click=move |_| {
                                                    let size = page_size.get();
                                                    let total = filtered_licenses.get().len();
                                                    let tp = (total + size - 1) / size;
                                                    let tp = if tp == 0 { 1 } else { tp };
                                                    current_page.update(|p| *p = (*p + 1).min(tp))
                                                }
                                                class="disabled:opacity-40 disabled:cursor-not-allowed"
                                            >
                                                "Next"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                }}
                </Transition>
            </div>
        </div>
    }
}

/// Referrers tab content
#[component]
fn ReferrersTab() -> impl IntoView {
    // Search state
    let search_query = RwSignal::new(String::new());
    let is_search_active = RwSignal::new(false);
    let is_refreshing = RwSignal::new(false);

    // Selection state
    let selected_ids = RwSignal::new(HashSet::<i32>::new());
    let select_all = RwSignal::new(false);
    let selection_mode = RwSignal::new(false);

    // Pagination state
    let current_page = RwSignal::new(1usize);
    let page_size = RwSignal::new(5usize);

    // Modal state
    let (show_modal, set_show_modal) = signal(false);
    let (modal_action, set_modal_action) = signal(Option::<ReferrerModalAction>::None);
    let (modal_loading, set_modal_loading) = signal(false);
    let (bulk_action_type, set_bulk_action_type) = signal(Option::<String>::None);

    let referrals = Resource::new(
        || (),
        |_| async move { get_uno_app_referrals().await }
    );

    // Compute stats from data
    let stats = Memo::new(move |_| {
        match referrals.get() {
            Some(Ok(data)) => {
                let total = data.referrals.len();
                let active = data.referrals.iter().filter(|r| r.status == "active").count();
                let pending = data.referrals.iter().filter(|r| r.status == "pending").count();
                (total, active, pending)
            }
            _ => (0, 0, 0)
        }
    });

    // Filtered referrals based on search
    let filtered_referrals = Memo::new(move |_| {
        match referrals.get() {
            Some(Ok(data)) => {
                let query = search_query.get().to_lowercase();
                if is_search_active.get() && !query.is_empty() {
                    data.referrals.into_iter().filter(|r| {
                        r.username.to_lowercase().contains(&query) ||
                        r.email.to_lowercase().contains(&query) ||
                        r.referral_code.to_lowercase().contains(&query)
                    }).collect::<Vec<_>>()
                } else {
                    data.referrals
                }
            }
            _ => vec![]
        }
    });

    // Reset page when search changes
    Effect::new(move |_| {
        let _ = is_search_active.get();
        current_page.set(1);
    });

    // Handle select all toggle
    Effect::new(move |_| {
        if select_all.get() && selection_mode.get() {
            let all_ids: HashSet<i32> = filtered_referrals.get().iter().map(|r| r.id).collect();
            selected_ids.set(all_ids);
        }
    });

    // Toggle selection for individual item
    let toggle_select = move |id: i32| {
        let mut current = selected_ids.get();
        if current.contains(&id) {
            current.remove(&id);
        } else {
            current.insert(id);
        }
        selected_ids.set(current);
    };

    let do_search = move |_| {
        if !search_query.get().is_empty() {
            is_search_active.set(true);
        }
    };

    let clear_search = move |_| {
        search_query.set(String::new());
        is_search_active.set(false);
    };

    let do_refresh = move |_| {
        is_refreshing.set(true);
        referrals.refetch();
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(500).await;
            is_refreshing.set(false);
        });
    };

    // Handle modal confirm - TODO: Implement actual server function calls
    let handle_modal_confirm = Callback::new(move |_: ()| {
        // Check if this is a bulk action
        if let Some(ref action_type) = bulk_action_type.get() {
            set_modal_loading.set(true);
            let ids: Vec<i32> = selected_ids.get().iter().cloned().collect();
            let action_str = action_type.clone();
            spawn_local(async move {
                web_sys::console::log_1(&format!("Would bulk {} referrers: {:?}", action_str, ids).into());
                gloo_timers::future::TimeoutFuture::new(500).await;
                set_modal_loading.set(false);
                set_show_modal.set(false);
                set_bulk_action_type.set(None);
                selected_ids.set(HashSet::new());
                select_all.set(false);
                referrals.refetch();
            });
        } else if let Some(action) = modal_action.get() {
            set_modal_loading.set(true);
            let new_status = match &action {
                ReferrerModalAction::Approve(_, _) => "active",
                ReferrerModalAction::Reject(_, _) => "rejected",
                ReferrerModalAction::Suspend(_, _) => "suspended",
            };
            // TODO: Call actual server function to update referrer status
            // For now, just log and close modal
            spawn_local(async move {
                web_sys::console::log_1(&format!("Would update referrer status to: {}", new_status).into());
                gloo_timers::future::TimeoutFuture::new(500).await;
                set_modal_loading.set(false);
                set_show_modal.set(false);
                set_modal_action.set(None);
            });
        }
    });

    // Modal info based on action
    let modal_title = Memo::new(move |_| {
        // Check bulk action first
        if let Some(ref action_type) = bulk_action_type.get() {
            let count = selected_ids.get().len();
            return match action_type.as_str() {
                "approve" => format!("Approve {} Referrers", count),
                "reject" => format!("Reject {} Referrers", count),
                "suspend" => format!("Suspend {} Referrers", count),
                _ => "".to_string(),
            };
        }
        match modal_action.get() {
            Some(ReferrerModalAction::Approve(_, ref name)) => format!("Approve {}", name),
            Some(ReferrerModalAction::Reject(_, ref name)) => format!("Reject {}", name),
            Some(ReferrerModalAction::Suspend(_, ref name)) => format!("Suspend {}", name),
            None => "".to_string(),
        }
    });

    let modal_message = Memo::new(move |_| {
        // Check bulk action first
        if let Some(ref action_type) = bulk_action_type.get() {
            let count = selected_ids.get().len();
            return match action_type.as_str() {
                "approve" => format!("Are you sure you want to approve {} referrers? Their status will be set to ACTIVE.", count),
                "reject" => format!("Are you sure you want to reject {} referrers? Their status will be set to REJECTED.", count),
                "suspend" => format!("Are you sure you want to suspend {} referrers? Their status will be set to SUSPENDED.", count),
                _ => "".to_string(),
            };
        }
        match modal_action.get() {
            Some(ReferrerModalAction::Approve(_, ref name)) => format!("Are you sure you want to approve {} as a referrer? Their status will be set to ACTIVE.", name),
            Some(ReferrerModalAction::Reject(_, ref name)) => format!("Are you sure you want to reject {}? Their status will be set to REJECTED.", name),
            Some(ReferrerModalAction::Suspend(_, ref name)) => format!("Are you sure you want to suspend {}? Their status will be set to SUSPENDED.", name),
            None => "".to_string(),
        }
    });

    let modal_confirm_text = Memo::new(move |_| {
        // Check bulk action first
        if let Some(ref action_type) = bulk_action_type.get() {
            return match action_type.as_str() {
                "approve" => "Approve All".to_string(),
                "reject" => "Reject All".to_string(),
                "suspend" => "Suspend All".to_string(),
                _ => "".to_string(),
            };
        }
        match modal_action.get() {
            Some(ReferrerModalAction::Approve(_, _)) => "Approve".to_string(),
            Some(ReferrerModalAction::Reject(_, _)) => "Reject".to_string(),
            Some(ReferrerModalAction::Suspend(_, _)) => "Suspend".to_string(),
            None => "".to_string(),
        }
    });

    let modal_color = move || {
        // Check bulk action first
        if let Some(ref action_type) = bulk_action_type.get() {
            return match action_type.as_str() {
                "approve" => "#22c55e",
                "reject" => "#ef4444",
                "suspend" => "#f59e0b",
                _ => "#3b82f6",
            };
        }
        match modal_action.get() {
            Some(ReferrerModalAction::Approve(_, _)) => "#22c55e",
            Some(ReferrerModalAction::Reject(_, _)) => "#ef4444",
            Some(ReferrerModalAction::Suspend(_, _)) => "#f59e0b",
            None => "#3b82f6",
        }
    };

    view! {
        <div class="space-y-4">
            // Confirmation Modal
            <ConfirmModal
                show=show_modal
                set_show=set_show_modal
                title=move || modal_title.get()
                message=move || modal_message.get()
                confirm_text=move || modal_confirm_text.get()
                confirm_color=modal_color
                on_confirm=handle_modal_confirm
                is_loading=modal_loading
            />

            // Stats Container
            <div class="rounded-2xl" style="background: #0f172a; padding: 2px 4px;">
                <div style="display: flex; flex-direction: row; gap: 1rem; flex-wrap: wrap;">
                    <div style="flex: 1; min-width: 200px;">
                        <SummaryCard title="Total Referrers" value=move || stats.get().0 icon=IconName::Users color_class="bg-cyan-500/20" />
                    </div>
                    <div style="flex: 1; min-width: 200px;">
                        <SummaryCard title="Active" value=move || stats.get().1 icon=IconName::CheckCircle color_class="bg-green-500/20" />
                    </div>
                    <div style="flex: 1; min-width: 200px;">
                        <SummaryCard title="Pending" value=move || stats.get().2 icon=IconName::Clock color_class="bg-yellow-500/20" />
                    </div>
                </div>
            </div>

            // Action Panel
            <div class="rounded-2xl" style="background: #0f172a; border: 1px solid #1e293b; padding: 8px 12px;">
                <div class="flex flex-wrap items-center justify-between gap-3">
                    // Group 1: Selection toggle and bulk actions (left)
                    <div class="flex items-center gap-2 flex-shrink-0">
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
                                selection_mode.set(new_mode);
                                if !new_mode {
                                    selected_ids.set(HashSet::new());
                                    select_all.set(false);
                                }
                            }
                        >
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

                        <Show when=move || !selected_ids.get().is_empty()>
                            <span style="color: #94a3b8; font-size: 14px;">
                                {move || format!("{} selected", selected_ids.get().len())}
                            </span>
                        </Show>

                        // Approve button
                        <button
                            style=move || {
                                if selected_ids.get().is_empty() {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; color: #64748b; cursor: not-allowed; opacity: 0.5;"
                                } else {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #22c55e; border: none; border-radius: 8px; color: white; cursor: pointer; transition: all 0.15s;"
                                }
                            }
                            disabled=move || selected_ids.get().is_empty()
                            title="Approve Selected"
                            on:click=move |_| {
                                set_bulk_action_type.set(Some("approve".to_string()));
                                set_show_modal.set(true);
                            }
                        >
                            <Icon name=IconName::CheckCircle size=18 />
                        </button>

                        // Reject button
                        <button
                            style=move || {
                                if selected_ids.get().is_empty() {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; color: #64748b; cursor: not-allowed; opacity: 0.5;"
                                } else {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #ef4444; border: none; border-radius: 8px; color: white; cursor: pointer; transition: all 0.15s;"
                                }
                            }
                            disabled=move || selected_ids.get().is_empty()
                            title="Reject Selected"
                            on:click=move |_| {
                                set_bulk_action_type.set(Some("reject".to_string()));
                                set_show_modal.set(true);
                            }
                        >
                            <Icon name=IconName::Close size=18 />
                        </button>

                        // Suspend button
                        <button
                            style=move || {
                                if selected_ids.get().is_empty() {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; color: #64748b; cursor: not-allowed; opacity: 0.5;"
                                } else {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #f59e0b; border: none; border-radius: 8px; color: white; cursor: pointer; transition: all 0.15s;"
                                }
                            }
                            disabled=move || selected_ids.get().is_empty()
                            title="Suspend Selected"
                            on:click=move |_| {
                                set_bulk_action_type.set(Some("suspend".to_string()));
                                set_show_modal.set(true);
                            }
                        >
                            <Icon name=IconName::Clock size=18 />
                        </button>
                    </div>

                    // Group 2: Search (center)
                    <div class="flex items-center gap-2" style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 4px 8px; min-height: 36px; flex: 1; max-width: 320px; min-width: 120px;">
                        <input
                            type="text"
                            placeholder="Search username, email, code..."
                            class="text-sm text-white/90 focus:outline-none flex-1"
                            style="background: transparent; border: none; min-width: 80px; padding: 4px;"
                            prop:value=move || search_query.get()
                            on:input=move |ev| search_query.set(event_target_value(&ev))
                            on:keydown=move |ev: web_sys::KeyboardEvent| {
                                if ev.key() == "Enter" && !search_query.get().is_empty() {
                                    is_search_active.set(true);
                                }
                            }
                        />
                        <button
                            style=move || {
                                if search_query.get().is_empty() {
                                    "display: flex; align-items: center; justify-content: center; padding: 4px 10px; background: #334155; border: none; border-radius: 6px; color: #64748b; cursor: not-allowed; font-size: 13px; font-weight: 500;"
                                } else {
                                    "display: flex; align-items: center; justify-content: center; padding: 4px 10px; background: #06b6d4; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 13px; font-weight: 500; transition: all 0.15s;"
                                }
                            }
                            disabled=move || search_query.get().is_empty()
                            on:click=do_search
                        >
                            "Go"
                        </button>
                        <Show when=move || is_search_active.get()>
                            <button
                                style="display: flex; align-items: center; justify-content: center; padding: 4px 8px; background: #ef4444; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 12px; font-weight: 500;"
                                on:click=clear_search
                                title="Clear Search"
                            >
                                "×"
                            </button>
                        </Show>
                    </div>

                    // Group 3: Refresh and count (right)
                    <div class="flex items-center gap-2 flex-shrink-0">
                        <button
                            style=move || {
                                if is_refreshing.get() {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #334155; border: none; border-radius: 8px; color: #64748b; cursor: not-allowed;"
                                } else {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #3b82f6; border: none; border-radius: 8px; color: white; cursor: pointer; transition: all 0.15s;"
                                }
                            }
                            disabled=move || is_refreshing.get()
                            on:click=do_refresh
                            title="Refresh Data"
                        >
                            {move || if is_refreshing.get() {
                                view! { <ProgressSpinner size=SpinnerSize::Small /> }.into_any()
                            } else {
                                view! { <Icon name=IconName::Refresh size=18 /> }.into_any()
                            }}
                        </button>
                        <span style="color: #94a3b8; font-size: 14px;">
                            {move || format!("{} referrers", filtered_referrals.get().len())}
                        </span>
                    </div>
                </div>
            </div>

            // Referrers Table with Pagination
            <Suspense fallback=move || view! { <div class="flex items-center justify-center p-12"><ProgressSpinner size=SpinnerSize::Large /></div> }>
                {move || match referrals.get() {
                    Some(Ok(_)) => {
                        let list = filtered_referrals.get();
                        let total_items = list.len();

                        if list.is_empty() {
                            view! {
                                <div class="bg-slate-800/50 border border-slate-700/50 rounded-xl p-12 text-center">
                                    <Icon name=IconName::Users size=48 class="text-slate-600 mx-auto mb-4".to_string() />
                                    <h3 class="text-lg font-medium text-slate-300 mb-2">"No referrers found"</h3>
                                    <p class="text-slate-500 text-sm">"Referrers from uno-app will appear here."</p>
                                </div>
                            }.into_any()
                        } else {
                            let page = current_page.get();
                            let size = page_size.get();
                            let start = (page - 1) * size;
                            let end = std::cmp::min(start + size, total_items);
                            let paginated_list: Vec<_> = list[start..end].to_vec();
                            let total_pages = (total_items + size - 1) / size;
                            let total_pages = if total_pages == 0 { 1 } else { total_pages };

                            view! {
                                <div class="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden">
                                    <table class="w-full">
                                        <thead>
                                            <tr class="text-xs font-medium text-slate-400 uppercase text-center" style="background: rgba(30, 41, 59, 0.5); border-bottom: 1px solid rgba(51, 65, 85, 0.8);">
                                                <Show when=move || selection_mode.get()>
                                                    <th class="py-2 px-4 w-10">
                                                        <input
                                                            type="checkbox"
                                                            style="width: 16px; height: 16px; cursor: pointer; accent-color: #06b6d4;"
                                                            prop:checked=move || select_all.get()
                                                            on:change=move |ev| {
                                                                let checked = event_target_checked(&ev);
                                                                select_all.set(checked);
                                                                if !checked {
                                                                    selected_ids.set(HashSet::new());
                                                                }
                                                            }
                                                        />
                                                    </th>
                                                </Show>
                                                <th class="py-2 px-4">"Username"</th>
                                                <th class="py-2 px-4">"Email"</th>
                                                <th class="py-2 px-4">"Country"</th>
                                                <th class="py-2 px-4">"Referral Code"</th>
                                                <th class="py-2 px-4">"Status"</th>
                                                <th class="py-2 px-4">"Actions"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {paginated_list.into_iter().enumerate().map(|(idx, r)| {
                                                let status_class = match r.status.as_str() {
                                                    "active" => "bg-green-500/20 text-green-400",
                                                    "pending" => "bg-yellow-500/20 text-yellow-400",
                                                    "suspended" => "bg-orange-500/20 text-orange-400",
                                                    "rejected" => "bg-red-500/20 text-red-400",
                                                    _ => "bg-slate-500/20 text-slate-400",
                                                };
                                                let username = r.username.clone();
                                                let email = r.email.clone();
                                                let country = r.country_code.clone();
                                                let referral_code = r.referral_code.clone();
                                                let status = r.status.clone();
                                                let id = r.id;
                                                let name_approve = username.clone();
                                                let name_reject = username.clone();
                                                let name_suspend = username.clone();
                                                let row_bg = if idx % 2 == 0 { "background: transparent;" } else { "background: rgba(51, 65, 85, 0.15);" };
                                                view! {
                                                    <tr style=row_bg class="hover:bg-cyan-900/20 transition-colors text-center">
                                                        <Show when=move || selection_mode.get()>
                                                            <td class="py-2 px-4 w-10">
                                                                <input
                                                                    type="checkbox"
                                                                    style="width: 16px; height: 16px; cursor: pointer; accent-color: #06b6d4;"
                                                                    prop:checked=move || selected_ids.get().contains(&id)
                                                                    on:change=move |_| toggle_select(id)
                                                                />
                                                            </td>
                                                        </Show>
                                                        <td class="py-2 px-4 text-sm text-white">{username}</td>
                                                        <td class="py-2 px-4 text-sm text-slate-300">{email}</td>
                                                        <td class="py-2 px-4">
                                                            <span class="text-lg">{country_flag(&country)}</span>
                                                            " "
                                                            <span class="text-sm">{country.clone()}</span>
                                                        </td>
                                                        <td class="py-2 px-4 font-mono text-sm text-cyan-400">{referral_code}</td>
                                                        <td class="py-2 px-4"><span class=format!("px-2 py-0.5 rounded-full text-xs font-medium {}", status_class)>{status}</span></td>
                                                        <td class="py-2 px-4">
                                                            <div style="display: flex; gap: 4px; justify-content: center; align-items: center;">
                                                                // Approve button (green tick)
                                                                <button
                                                                    style="display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #22c55e; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                                                                    title="Approve"
                                                                    on:click={
                                                                        let name = name_approve.clone();
                                                                        move |_| {
                                                                            set_modal_action.set(Some(ReferrerModalAction::Approve(id, name.clone())));
                                                                            set_show_modal.set(true);
                                                                        }
                                                                    }
                                                                >
                                                                    <Icon name=IconName::CheckCircle size=14 />
                                                                </button>
                                                                // Reject button (red X)
                                                                <button
                                                                    style="display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #ef4444; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                                                                    title="Reject"
                                                                    on:click={
                                                                        let name = name_reject.clone();
                                                                        move |_| {
                                                                            set_modal_action.set(Some(ReferrerModalAction::Reject(id, name.clone())));
                                                                            set_show_modal.set(true);
                                                                        }
                                                                    }
                                                                >
                                                                    <Icon name=IconName::Close size=14 />
                                                                </button>
                                                                // Suspend button (yellow pause)
                                                                <button
                                                                    style="display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #f59e0b; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                                                                    title="Suspend"
                                                                    on:click={
                                                                        let name = name_suspend.clone();
                                                                        move |_| {
                                                                            set_modal_action.set(Some(ReferrerModalAction::Suspend(id, name.clone())));
                                                                            set_show_modal.set(true);
                                                                        }
                                                                    }
                                                                >
                                                                    <Icon name=IconName::Clock size=14 />
                                                                </button>
                                                            </div>
                                                        </td>
                                                    </tr>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </tbody>
                                    </table>

                                    // Pagination controls
                                    <div class="flex items-center justify-between px-4 py-2 border-t border-slate-700/50">
                                        <div class="flex items-center gap-3">
                                            <span class="text-slate-400 text-sm">
                                                {format!("Page {} of {} ({} total)", page, total_pages, total_items)}
                                            </span>
                                        </div>
                                        <div class="flex items-center gap-3">
                                            <select
                                                style="padding: 5px 8px; background: #334155; border: 1px solid #475569; border-radius: 6px; color: #e2e8f0; font-size: 13px; cursor: pointer; outline: none;"
                                                on:change=move |ev| {
                                                    if let Ok(size) = event_target_value(&ev).parse::<usize>() {
                                                        page_size.set(size);
                                                        current_page.set(1);
                                                    }
                                                }
                                            >
                                                {PAGE_SIZE_OPTIONS.iter().map(|&s| {
                                                    view! {
                                                        <option value=s.to_string() selected=move || page_size.get() == s>
                                                            {s}
                                                        </option>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </select>
                                            <div class="flex gap-2">
                                                <button
                                                    style="padding: 6px 14px; background: #06b6d4; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 500; transition: opacity 0.15s;"
                                                    disabled=move || current_page.get() <= 1
                                                    on:click=move |_| current_page.update(|p| *p = (*p).saturating_sub(1).max(1))
                                                    class="disabled:opacity-40 disabled:cursor-not-allowed"
                                                >
                                                    "Back"
                                                </button>
                                                <button
                                                    style="padding: 6px 14px; background: #06b6d4; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 500; transition: opacity 0.15s;"
                                                    disabled=move || {
                                                        let size = page_size.get();
                                                        let total = filtered_referrals.get().len();
                                                        let tp = (total + size - 1) / size;
                                                        let tp = if tp == 0 { 1 } else { tp };
                                                        current_page.get() >= tp
                                                    }
                                                    on:click=move |_| {
                                                        let size = page_size.get();
                                                        let total = filtered_referrals.get().len();
                                                        let tp = (total + size - 1) / size;
                                                        let tp = if tp == 0 { 1 } else { tp };
                                                        current_page.update(|p| *p = (*p + 1).min(tp))
                                                    }
                                                    class="disabled:opacity-40 disabled:cursor-not-allowed"
                                                >
                                                    "Next"
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }
                    Some(Err(e)) => view! {
                        <div class="bg-red-500/10 border border-red-500/30 rounded-xl p-6 text-center">
                            <Icon name=IconName::AlertCircle size=48 class="text-red-500 mx-auto mb-4".to_string() />
                            <h3 class="text-lg font-medium text-red-400 mb-2">"Failed to load referrers"</h3>
                            <p class="text-red-300 text-sm">{format!("{}", e)}</p>
                        </div>
                    }.into_any(),
                    None => view! { <div class="flex items-center justify-center p-12"><ProgressSpinner size=SpinnerSize::Large /></div> }.into_any(),
                }}
            </Suspense>
        </div>
    }
}

/// Visitors tab content
#[component]
fn VisitorsTab() -> impl IntoView {
    let period = RwSignal::new("weekly".to_string());
    let search_query = RwSignal::new(String::new());
    let is_search_active = RwSignal::new(false);
    let is_refreshing = RwSignal::new(false);

    // Pagination state
    let current_page = RwSignal::new(1usize);
    let page_size = RwSignal::new(5usize);

    let visitor_stats = Resource::new(
        move || period.get(),
        |p| async move { get_uno_app_visitor_stats(Some(p), Some(100)).await }
    );

    // Compute stats from data
    let stats = Memo::new(move |_| {
        match visitor_stats.get() {
            Some(Ok(data)) => (data.total_visitors as usize, data.unique_visitors as usize, data.stats.len()),
            _ => (0, 0, 0)
        }
    });

    // Get period for display
    let period_display = Memo::new(move |_| {
        match visitor_stats.get() {
            Some(Ok(data)) => data.period.clone(),
            _ => period.get()
        }
    });

    // Filtered countries based on search
    let filtered_stats = Memo::new(move |_| {
        match visitor_stats.get() {
            Some(Ok(data)) => {
                let query = search_query.get().to_lowercase();
                if is_search_active.get() && !query.is_empty() {
                    data.stats.into_iter().filter(|s| {
                        s.country_code.to_lowercase().contains(&query)
                    }).collect::<Vec<_>>()
                } else {
                    data.stats
                }
            }
            _ => vec![]
        }
    });

    // Reset page when search/period changes
    Effect::new(move |_| {
        let _ = is_search_active.get();
        let _ = period.get();
        current_page.set(1);
    });

    let do_search = move |_| {
        if !search_query.get().is_empty() {
            is_search_active.set(true);
        }
    };

    let clear_search = move |_| {
        search_query.set(String::new());
        is_search_active.set(false);
    };

    let do_refresh = move |_| {
        is_refreshing.set(true);
        visitor_stats.refetch();
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(500).await;
            is_refreshing.set(false);
        });
    };

    view! {
        <div class="space-y-4">
            // Stats Container
            <div class="rounded-2xl" style="background: #0f172a; padding: 2px 4px;">
                <div style="display: flex; flex-direction: row; gap: 1rem; flex-wrap: wrap;">
                    <div style="flex: 1; min-width: 200px;">
                        <SummaryCard title="Total Visitors" value=move || stats.get().0 icon=IconName::Users color_class="bg-blue-500/20" />
                    </div>
                    <div style="flex: 1; min-width: 200px;">
                        <SummaryCard title="Unique Visitors" value=move || stats.get().1 icon=IconName::User color_class="bg-green-500/20" />
                    </div>
                    <div style="flex: 1; min-width: 200px;">
                        <SummaryCard title="Countries" value=move || stats.get().2 icon=IconName::Globe color_class="bg-purple-500/20" />
                    </div>
                </div>
            </div>

            // Action Panel
            <div class="rounded-2xl" style="background: #0f172a; border: 1px solid #1e293b; padding: 8px 12px;">
                <div class="flex flex-wrap items-center justify-between gap-3">
                    <div class="flex items-center gap-2 flex-shrink-0">
                        <button
                            style=move || {
                                if is_refreshing.get() {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #334155; border: none; border-radius: 8px; color: #64748b; cursor: not-allowed;"
                                } else {
                                    "display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; background: #3b82f6; border: none; border-radius: 8px; color: white; cursor: pointer; transition: all 0.15s;"
                                }
                            }
                            disabled=move || is_refreshing.get()
                            on:click=do_refresh
                            title="Refresh Data"
                        >
                            {move || if is_refreshing.get() {
                                view! { <ProgressSpinner size=SpinnerSize::Small /> }.into_any()
                            } else {
                                view! { <Icon name=IconName::Refresh size=18 /> }.into_any()
                            }}
                        </button>
                    </div>

                    <div class="flex items-center gap-2" style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 4px 8px; min-height: 36px; flex: 1; max-width: 280px; min-width: 120px;">
                        <input
                            type="text"
                            placeholder="Search country code..."
                            class="text-sm text-white/90 focus:outline-none flex-1"
                            style="background: transparent; border: none; min-width: 80px; padding: 4px;"
                            prop:value=move || search_query.get()
                            on:input=move |ev| search_query.set(event_target_value(&ev))
                            on:keydown=move |ev: web_sys::KeyboardEvent| {
                                if ev.key() == "Enter" && !search_query.get().is_empty() {
                                    is_search_active.set(true);
                                }
                            }
                        />
                        <button
                            style=move || {
                                if search_query.get().is_empty() {
                                    "display: flex; align-items: center; justify-content: center; padding: 4px 10px; background: #334155; border: none; border-radius: 6px; color: #64748b; cursor: not-allowed; font-size: 13px; font-weight: 500;"
                                } else {
                                    "display: flex; align-items: center; justify-content: center; padding: 4px 10px; background: #06b6d4; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 13px; font-weight: 500; transition: all 0.15s;"
                                }
                            }
                            disabled=move || search_query.get().is_empty()
                            on:click=do_search
                        >
                            "Go"
                        </button>
                        <Show when=move || is_search_active.get()>
                            <button
                                style="display: flex; align-items: center; justify-content: center; padding: 4px 8px; background: #ef4444; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 12px; font-weight: 500;"
                                on:click=clear_search
                                title="Clear Search"
                            >
                                "×"
                            </button>
                        </Show>
                    </div>

                    <div class="flex items-center gap-3 flex-shrink-0">
                        <div class="flex items-center gap-2">
                            <label class="text-sm text-slate-400">"Period:"</label>
                            <select
                                class="text-sm text-white/90 focus:outline-none focus:ring-2 focus:ring-primary-500 transition-all"
                                style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 8px 32px 8px 12px; appearance: none; background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2212%22 height=%2212%22 viewBox=%220 0 12 12%22%3E%3Cpath fill=%22%2394a3b8%22 d=%22M2 4l4 4 4-4%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 12px center;"
                                on:change=move |ev| period.set(event_target_value(&ev))
                            >
                                <option value="daily">"Daily"</option>
                                <option value="weekly" selected>"Weekly"</option>
                                <option value="monthly">"Monthly"</option>
                                <option value="all">"All Time"</option>
                            </select>
                        </div>
                        <span style="color: #94a3b8; font-size: 14px;">
                            {move || format!("{} countries", filtered_stats.get().len())}
                        </span>
                    </div>
                </div>
            </div>

            // Visitors Table with Pagination
            <Suspense fallback=move || view! { <div class="flex items-center justify-center p-12"><ProgressSpinner size=SpinnerSize::Large /></div> }>
                {move || match visitor_stats.get() {
                    Some(Ok(_)) => {
                        let list = filtered_stats.get();
                        let total_items = list.len();
                        let period_str = period_display.get();

                        if list.is_empty() {
                            view! {
                                <div class="bg-slate-800/50 border border-slate-700/50 rounded-xl p-12 text-center">
                                    <Icon name=IconName::Globe size=48 class="text-slate-600 mx-auto mb-4".to_string() />
                                    <h3 class="text-lg font-medium text-slate-300 mb-2">"No visitor data"</h3>
                                    <p class="text-slate-500 text-sm">"Visitor statistics will appear here."</p>
                                </div>
                            }.into_any()
                        } else {
                            let page = current_page.get();
                            let size = page_size.get();
                            let start = (page - 1) * size;
                            let end = std::cmp::min(start + size, total_items);
                            let paginated_list: Vec<_> = list[start..end].to_vec();
                            let total_pages = (total_items + size - 1) / size;
                            let total_pages = if total_pages == 0 { 1 } else { total_pages };

                            view! {
                                <div class="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden">
                                    <table class="w-full">
                                        <thead>
                                            <tr class="text-xs font-medium text-slate-400 uppercase text-center" style="background: rgba(30, 41, 59, 0.5); border-bottom: 1px solid rgba(51, 65, 85, 0.8);">
                                                <th class="py-2 px-4">"Country"</th>
                                                <th class="py-2 px-4">"Period"</th>
                                                <th class="py-2 px-4">"Total Visits"</th>
                                                <th class="py-2 px-4">"Unique Visitors"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {paginated_list.into_iter().enumerate().map(|(idx, s)| {
                                                let code = s.country_code.clone();
                                                let visits = s.visitor_count;
                                                let uniques = s.unique_visitors;
                                                let period_val = period_str.clone();
                                                let row_bg = if idx % 2 == 0 { "background: transparent;" } else { "background: rgba(51, 65, 85, 0.15);" };
                                                view! {
                                                    <tr style=row_bg class="hover:bg-cyan-900/20 transition-colors text-center">
                                                        <td class="py-2 px-4">
                                                            <span class="text-lg mr-2">{country_flag(&code)}</span>
                                                            <span class="text-sm text-white">{code.clone()}</span>
                                                        </td>
                                                        <td class="py-2 px-4 text-sm text-slate-400">{period_val}</td>
                                                        <td class="py-2 px-4 text-sm text-slate-300">{visits}</td>
                                                        <td class="py-2 px-4 text-sm text-slate-300">{uniques}</td>
                                                    </tr>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </tbody>
                                    </table>

                                    // Pagination controls
                                    <div class="flex items-center justify-between px-4 py-2 border-t border-slate-700/50">
                                        <div class="flex items-center gap-3">
                                            <span class="text-slate-400 text-sm">
                                                {format!("Page {} of {} ({} total)", page, total_pages, total_items)}
                                            </span>
                                        </div>
                                        <div class="flex items-center gap-3">
                                            <select
                                                style="padding: 5px 8px; background: #334155; border: 1px solid #475569; border-radius: 6px; color: #e2e8f0; font-size: 13px; cursor: pointer; outline: none;"
                                                on:change=move |ev| {
                                                    if let Ok(size) = event_target_value(&ev).parse::<usize>() {
                                                        page_size.set(size);
                                                        current_page.set(1);
                                                    }
                                                }
                                            >
                                                {PAGE_SIZE_OPTIONS.iter().map(|&s| {
                                                    view! {
                                                        <option value=s.to_string() selected=move || page_size.get() == s>
                                                            {s}
                                                        </option>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </select>
                                            <div class="flex gap-2">
                                                <button
                                                    style="padding: 6px 14px; background: #06b6d4; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 500; transition: opacity 0.15s;"
                                                    disabled=move || current_page.get() <= 1
                                                    on:click=move |_| current_page.update(|p| *p = (*p).saturating_sub(1).max(1))
                                                    class="disabled:opacity-40 disabled:cursor-not-allowed"
                                                >
                                                    "Back"
                                                </button>
                                                <button
                                                    style="padding: 6px 14px; background: #06b6d4; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 500; transition: opacity 0.15s;"
                                                    disabled=move || {
                                                        let size = page_size.get();
                                                        let total = filtered_stats.get().len();
                                                        let tp = (total + size - 1) / size;
                                                        let tp = if tp == 0 { 1 } else { tp };
                                                        current_page.get() >= tp
                                                    }
                                                    on:click=move |_| {
                                                        let size = page_size.get();
                                                        let total = filtered_stats.get().len();
                                                        let tp = (total + size - 1) / size;
                                                        let tp = if tp == 0 { 1 } else { tp };
                                                        current_page.update(|p| *p = (*p + 1).min(tp))
                                                    }
                                                    class="disabled:opacity-40 disabled:cursor-not-allowed"
                                                >
                                                    "Next"
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }
                    Some(Err(e)) => view! {
                        <div class="bg-red-500/10 border border-red-500/30 rounded-xl p-6 text-center">
                            <Icon name=IconName::AlertCircle size=48 class="text-red-500 mx-auto mb-4".to_string() />
                            <h3 class="text-lg font-medium text-red-400 mb-2">"Failed to load visitor stats"</h3>
                            <p class="text-red-300 text-sm">{format!("{}", e)}</p>
                        </div>
                    }.into_any(),
                    None => view! { <div class="flex items-center justify-center p-12"><ProgressSpinner size=SpinnerSize::Large /></div> }.into_any(),
                }}
            </Suspense>
        </div>
    }
}

/// Marketplace list page component
#[component]
pub fn MarketplaceListPage() -> impl IntoView {
    let active_tab = RwSignal::new(MainTab::Licenses);

    view! {
        <div class="p-6 space-y-6">
            <Header title="Marketplace" />

            // Main Tab Navigation
            <div class="flex items-center gap-2 border-b border-slate-700 pb-4">
                {[MainTab::Licenses, MainTab::Referrers, MainTab::Visitors].into_iter().map(|tab| {
                    let is_active = move || active_tab.get() == tab;
                    view! {
                        <button
                            class=move || format!(
                                "px-4 py-2 rounded-lg text-sm font-medium transition flex items-center gap-2 {}",
                                if is_active() { "bg-cyan-500 text-white" } else { "bg-slate-800 text-slate-400 hover:bg-slate-700" }
                            )
                            on:click=move |_| active_tab.set(tab)
                        >
                            <Icon name=tab.icon() size=16 />
                            {tab.label()}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Tab Content
            {move || match active_tab.get() {
                MainTab::Licenses => view! { <LicensesTab /> }.into_any(),
                MainTab::Referrers => view! { <ReferrersTab /> }.into_any(),
                MainTab::Visitors => view! { <VisitorsTab /> }.into_any(),
            }}
        </div>
    }
}
