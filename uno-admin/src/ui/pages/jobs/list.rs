//! Jobs list page - displays sync job status with actions

use leptos::prelude::*;
use std::collections::HashSet;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::components::common::progress_spinner::{ProgressSpinner, SpinnerSize};
use crate::handler::{
    list_sync_jobs, run_sync_job, rerun_sync_job, delete_sync_job, run_license_sync_job,
    SyncJobsContextProvider, use_sync_jobs,
};
use crate::models::entity::{SyncJobContext, SyncJobEntity};
use crate::state::use_dashboard;
use crate::storage::{get_jobs_last_seen, set_jobs_last_seen};

#[cfg(target_arch = "wasm32")]
use crate::ui::hooks::{use_job_websocket, WsState};

/// WebSocket state enum for SSR (matches WASM version)
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum WsState {
    #[default]
    Connecting,
    Connected,
    Disconnected,
}

/// WebSocket connection status indicator - unified for SSR and WASM
///
/// Uses reactive signals with consistent DOM structure to prevent hydration mismatches.
/// SSR renders initial "Connecting..." state, WASM hydrates and updates reactively.
#[component]
fn WsStatusIndicator(
    /// WebSocket state signal (WASM passes real signal, SSR passes None)
    #[prop(optional)]
    ws_state: Option<ReadSignal<WsState>>,
    /// Reconnect callback (WASM only)
    #[prop(optional)]
    ws_reconnect: Option<Callback<()>>,
) -> impl IntoView {
    // Create internal state signal - starts at Connecting (matches SSR)
    let internal_state = RwSignal::new(WsState::Connecting);

    // On WASM, sync internal state with actual WebSocket state after hydration
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(external_state) = ws_state {
            Effect::new(move |_| {
                internal_state.set(external_state.get());
            });
        }
    }

    // Suppress unused variable warning on SSR
    #[cfg(not(target_arch = "wasm32"))]
    let _ = ws_state;

    // Derived values from internal state
    let dot_class = Memo::new(move |_| {
        match internal_state.get() {
            WsState::Connected => "bg-green-500",
            WsState::Connecting => "bg-yellow-500 animate-pulse",
            WsState::Disconnected => "bg-red-500",
        }
    });

    let text_class = Memo::new(move |_| {
        match internal_state.get() {
            WsState::Connected => "text-green-400",
            WsState::Connecting => "text-yellow-400",
            WsState::Disconnected => "text-red-400",
        }
    });

    let text_content = Memo::new(move |_| {
        match internal_state.get() {
            WsState::Connected => "Live",
            WsState::Connecting => "Connecting...",
            WsState::Disconnected => "Offline",
        }
    });

    let title_text = Memo::new(move |_| {
        match internal_state.get() {
            WsState::Connected => "WebSocket connected - real-time updates active",
            WsState::Connecting => "Connecting to WebSocket...",
            WsState::Disconnected => "WebSocket disconnected - click to reconnect",
        }
    });

    view! {
        <div
            class="flex items-center gap-2 cursor-pointer"
            title=move || title_text.get()
            on:click=move |_| {
                if internal_state.get() == WsState::Disconnected {
                    if let Some(ref reconnect) = ws_reconnect {
                        reconnect.run(());
                    }
                }
            }
        >
            <div class=move || format!("w-2.5 h-2.5 rounded-full {}", dot_class.get()) />
            <span class=move || format!("text-xs {}", text_class.get())>
                {move || text_content.get()}
            </span>
        </div>
    }
}

/// Status icon component for job status
#[component]
fn StatusBadge(status: String) -> impl IntoView {
    match status.as_str() {
        "pending" => view! {
            <div class="flex items-center justify-center animate-pulse" title="Pending">
                <Icon name=IconName::Clock size=20 class="text-yellow-400".to_string() />
            </div>
        }.into_any(),
        "running" => view! {
            <div class="flex items-center justify-center" title="Running">
                <ProgressSpinner size=SpinnerSize::Tiny />
            </div>
        }.into_any(),
        "completed" => view! {
            <div class="flex items-center justify-center" title="Success">
                <Icon name=IconName::CheckCircle size=20 class="text-green-500".to_string() />
            </div>
        }.into_any(),
        "failed" => view! {
            <div class="flex items-center justify-center" title="Failed">
                <Icon name=IconName::AlertCircle size=20 class="text-red-500".to_string() />
            </div>
        }.into_any(),
        _ => view! {
            <div class="flex items-center justify-center" title="Unknown">
                <Icon name=IconName::Info size=20 class="text-slate-400".to_string() />
            </div>
        }.into_any(),
    }
}

/// Format timestamp for display
fn format_timestamp(ts: &Option<String>) -> String {
    match ts {
        Some(s) if !s.is_empty() => {
            // Parse ISO timestamp and format nicely
            if let Some(date_part) = s.split('T').next() {
                if let Some(time_part) = s.split('T').nth(1) {
                    let time_short = time_part.split('.').next().unwrap_or(time_part);
                    return format!("{} {}", date_part, time_short);
                }
            }
            s.clone()
        }
        _ => "-".to_string(),
    }
}

/// Shorten a long ID for display
fn shorten_id(id: &str) -> String {
    if id.len() > 16 {
        format!("{}...{}", &id[..8], &id[id.len()-6..])
    } else {
        id.to_string()
    }
}

/// Page size options for pagination
const PAGE_SIZE_OPTIONS: [usize; 5] = [5, 10, 20, 50, 100];

/// Single job console component - renders a collapsible console for one job
#[component]
fn SingleJobConsole(
    job: SyncJobEntity,
    #[prop(optional)]
    is_collapsed: bool,
) -> impl IntoView {
    let (collapsed, set_collapsed) = signal(is_collapsed);

    let job_type_display = match job.job_type.as_str() {
        "rewards_sync" => "Rewards",
        "licenses_sync" => "Licenses",
        _ => &job.job_type,
    };
    let status_text = match job.status.as_str() {
        "running" => "Running...",
        "completed" => "Completed",
        "failed" => "Failed",
        "pending" => "Pending",
        s => s,
    };
    let info = format!("Job: {} | Type: {} | {}",
        shorten_id(&job.id), job_type_display, status_text);
    let is_running = job.status == "running";
    let status_color = if is_running { "bg-green-500 animate-pulse" }
        else if job.status == "completed" { "bg-green-500" }
        else if job.status == "failed" { "bg-red-500" }
        else { "bg-slate-600" };
    let logs = job.parse_logs();

    view! {
        <div class="bg-slate-900 rounded-xl border border-slate-700/50 overflow-hidden">
            <div
                class="flex items-center justify-between px-4 py-3 bg-slate-800/50 cursor-pointer hover:bg-slate-800 transition-colors"
                on:click=move |_| set_collapsed.update(|c| *c = !*c)
            >
                <div class="flex items-center gap-3">
                    <div class=format!("w-3 h-3 rounded-full {}", status_color) />
                    <h3 class="text-sm font-semibold text-slate-300">"Job Execution Console"</h3>
                    <span class="text-xs text-slate-500">{format!("- {}", info)}</span>
                </div>
                <div style="display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #475569; color: white; border: none; border-radius: 50%; cursor: pointer;">
                    {move || if collapsed.get() {
                        view! { <Icon name=IconName::Plus size=14 /> }.into_any()
                    } else {
                        view! { <Icon name=IconName::Minus size=14 /> }.into_any()
                    }}
                </div>
            </div>
            <Show when=move || !collapsed.get()>
                <div
                    class="font-mono text-xs p-4 console-scroll"
                    style="background: #0f172a; min-height: 150px; max-height: 250px;"
                >
                    {if logs.is_empty() {
                        view! {
                            <p class="text-slate-600 italic">
                                {if is_running { "Waiting for logs..." } else { "No logs available" }}
                            </p>
                        }.into_any()
                    } else {
                        view! {
                            <div>
                                {logs.iter().enumerate().map(|(idx, line)| {
                                    let color = if line.contains("ERROR") { "color: #f87171;" }
                                        else if line.contains("WARN") { "color: #fbbf24;" }
                                        else { "color: #4ade80;" };
                                    let line_clone = line.clone();
                                    view! {
                                        <div style=color>
                                            <span style="color: #475569;">{format!("{:3} ", idx + 1)}</span>
                                            {line_clone}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }}
                </div>
            </Show>
        </div>
    }
}

/// Job context details component (expandable content) with pagination
#[component]
fn JobContextDetails(context: Option<SyncJobContext>) -> impl IntoView {
    let (current_page, set_current_page) = signal(1usize);
    let (page_size, set_page_size) = signal(5usize); // Default to 5 items

    match context {
        Some(ctx) => {
            let total_items = ctx.license_rewards.len();
            let license_rewards = ctx.license_rewards.clone();

            view! {
                <div class="space-y-3 p-4 bg-slate-800/40 rounded-lg">
                    // Header row: Summary stats on left, pagination info on right
                    <div class="flex items-center justify-between px-2">
                        // Summary stats
                        <div class="flex items-center gap-6 text-sm">
                            <div>
                                <span class="text-slate-400">"Unique Licenses: "</span>
                                <span class="text-slate-200 font-medium">{ctx.unique_licenses}</span>
                            </div>
                            <div class="flex items-center gap-1">
                                <span class="text-slate-400">"Total Rewards: "</span>
                                <span style="color: #fbbf24;"><Icon name=IconName::Coin size=16 /></span>
                                <span style="color: #4ade80; font-weight: 500;">
                                    {format!("{:.6} UNO", ctx.total_amount_micros as f64 / 1_000_000.0)}
                                </span>
                            </div>
                        </div>
                        // Pagination info on top right: [ page_size ]/total
                        <div class="flex items-center gap-1 text-sm">
                            <span class="text-slate-500">""</span>
                            <select
                                style="padding: 2px 6px; background: #1e293b; border: 1px solid #334155; border-radius: 4px; color: #e2e8f0; font-size: 13px; cursor: pointer; outline: none;"
                                on:change=move |ev| {
                                    use leptos::prelude::event_target_value;
                                    if let Ok(size) = event_target_value(&ev).parse::<usize>() {
                                        set_page_size.set(size);
                                        set_current_page.set(1);
                                    }
                                }
                            >
                                {PAGE_SIZE_OPTIONS.iter().map(|&size| {
                                    view! {
                                        <option value=size.to_string() selected=move || page_size.get() == size>
                                            {size}
                                        </option>
                                    }
                                }).collect::<Vec<_>>()}
                            </select>
                            <span class="text-slate-500">"/"</span>
                            <span class="text-slate-300">{total_items}</span>
                        </div>
                    </div>

                    // License details table - no border, subtle styling
                    <div class="rounded-lg overflow-hidden">
                        <table class="w-full text-sm">
                            <thead class="text-xs text-slate-500 uppercase" style="border-bottom: 1px solid rgba(71, 85, 105, 0.4);">
                                <tr>
                                    <th class="px-4 py-1.5 text-left font-medium">"License ID"</th>
                                    <th class="px-4 py-1.5 text-left font-medium">"Type"</th>
                                    <th class="px-4 py-1.5 text-right font-medium">"Amount"</th>
                                    <th class="px-4 py-1.5 text-center font-medium w-16">"View"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || {
                                    let page = current_page.get();
                                    let size = page_size.get();
                                    let start = (page - 1) * size;
                                    let end = std::cmp::min(start + size, total_items);

                                    license_rewards[start..end].iter().enumerate().map(|(idx, lr)| {
                                        let license_id = lr.license_id.clone();
                                        let license_id_for_link = lr.license_id.clone();
                                        let amount = lr.amount_micros;
                                        let row_bg = if idx % 2 == 0 {
                                            "background: transparent;"
                                        } else {
                                            "background: rgba(51, 65, 85, 0.15);"
                                        };
                                        view! {
                                            <tr style=row_bg class="hover:bg-cyan-900/20 transition-colors">
                                                <td class="px-4 py-1 text-slate-300 font-mono text-xs">
                                                    {shorten_id(&license_id)}
                                                </td>
                                                <td class="px-4 py-1 text-slate-400 text-xs">
                                                    "license_owner"
                                                </td>
                                                <td class="px-4 py-1 text-right">
                                                    <div class="flex items-center justify-end gap-1">
                                                        <span style="color: #fbbf24;"><Icon name=IconName::Coin size=16 /></span>
                                                        <span style="color: #4ade80; font-weight: 500; font-size: 14px;">
                                                            {format!("{:.4}", amount as f64 / 1_000_000.0)}
                                                        </span>
                                                    </div>
                                                </td>
                                                <td class="px-4 py-1 text-center">
                                                    <a
                                                        href=format!("/licenses/{}", license_id_for_link)
                                                        style="display: inline-flex; align-items: center; justify-content: center; width: 24px; height: 24px; background: #475569; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                                                        title="View License"
                                                    >
                                                        <Icon name=IconName::Eye size=14 />
                                                    </a>
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>()
                                }}
                            </tbody>
                        </table>
                    </div>

                    // Centered navigation buttons at bottom
                    <div class="flex justify-center pt-2">
                        <div class="flex gap-2">
                            <button
                                style="display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: #06b6d4; color: white; border: none; border-radius: 6px; cursor: pointer; transition: opacity 0.15s;"
                                disabled=move || current_page.get() <= 1
                                on:click=move |_| set_current_page.update(|p| *p = (*p).saturating_sub(1).max(1))
                                class="disabled:opacity-40 disabled:cursor-not-allowed"
                            >
                                <Icon name=IconName::ChevronLeft size=18 />
                            </button>
                            <button
                                style="display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: #06b6d4; color: white; border: none; border-radius: 6px; cursor: pointer; transition: opacity 0.15s;"
                                disabled=move || {
                                    let size = page_size.get();
                                    let total_pages = (total_items + size - 1) / size;
                                    let total_pages = if total_pages == 0 { 1 } else { total_pages };
                                    current_page.get() >= total_pages
                                }
                                on:click=move |_| {
                                    let size = page_size.get();
                                    let total_pages = (total_items + size - 1) / size;
                                    let total_pages = if total_pages == 0 { 1 } else { total_pages };
                                    set_current_page.update(|p| *p = (*p + 1).min(total_pages))
                                }
                                class="disabled:opacity-40 disabled:cursor-not-allowed"
                            >
                                <Icon name=IconName::ChevronRight size=18 />
                            </button>
                        </div>
                    </div>
                </div>
            }.into_any()
        },
        None => view! {
            <div class="p-3 bg-slate-900/50 rounded-lg">
                <p class="text-slate-500 text-sm italic">"No context data available for this job"</p>
            </div>
        }.into_any(),
    }
}

/// Jobs table row component
#[component]
fn JobRow(
    job: SyncJobEntity,
    row_index: usize,
    on_refresh: Callback<()>,
    expanded_jobs: ReadSignal<HashSet<String>>,
    on_toggle_expand: Callback<String>,
    selected_job_id: ReadSignal<Option<String>>,
    on_select_job: Callback<String>,
) -> impl IntoView {
    let job_id = job.id.clone();
    let job_id_for_delete = job.id.clone();
    let job_id_for_expand = job.id.clone();
    let job_id_for_icon = job.id.clone();
    let job_id_for_details = job.id.clone();
    let job_id_for_select = job.id.clone();
    let job_id_for_selected_check = job.id.clone();
    let target_date = job.target_date.clone();
    let job_type = job.job_type.clone();
    // Extract time from created_at for display
    let created_time = job.created_at.split('T')
        .nth(1)
        .and_then(|t| t.split('.').next())
        .unwrap_or("")
        .to_string();
    let status = job.status.clone();
    let is_running = status == "running";
    let error_message = job.error_message.clone();

    // Zebra striping background
    let row_bg = if row_index % 2 == 0 {
        "background: rgba(30, 41, 59, 0.3);"
    } else {
        "background: rgba(51, 65, 85, 0.2);"
    };

    let (deleting, set_deleting) = signal(false);
    let (rerunning, set_rerunning) = signal(false);

    // Get duration
    let duration = job.duration_formatted();
    let job_context = job.parse_context();
    let job_logs = job.parse_logs();

    // Rerun job action
    let handle_rerun = move |_| {
        let job_id = job_id.clone();
        set_rerunning.set(true);

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen_futures::spawn_local;
            let on_refresh = on_refresh.clone();
            spawn_local(async move {
                match rerun_sync_job(job_id).await {
                    Ok(_) => {
                        on_refresh.run(());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to rerun job: {}", e).into());
                    }
                }
                set_rerunning.set(false);
            });
        }
    };

    // Delete job action
    let handle_delete = move |_| {
        let job_id = job_id_for_delete.clone();
        set_deleting.set(true);

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen_futures::spawn_local;
            let on_refresh = on_refresh.clone();
            spawn_local(async move {
                match delete_sync_job(job_id).await {
                    Ok(_) => {
                        on_refresh.run(());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to delete job: {}", e).into());
                    }
                }
                set_deleting.set(false);
            });
        }
    };

    view! {
        <>
            <tr style=format!("border-bottom: 1px solid rgba(51, 65, 85, 0.5); transition: background 0.15s; {}", row_bg) class="hover:bg-cyan-900/20">
                // Expand toggle - compact
                <td style="padding: 8px 4px 8px 8px; text-align: center;">
                    <button
                        style="display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #475569; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s; margin: 0 auto;"
                        on:click=move |_| on_toggle_expand.run(job_id_for_expand.clone())
                    >
                        {move || if expanded_jobs.get().contains(&job_id_for_icon) {
                            view! { <Icon name=IconName::Minus size=14 /> }.into_any()
                        } else {
                            view! { <Icon name=IconName::Plus size=14 /> }.into_any()
                        }}
                    </button>
                </td>

                // Target Date + Time (on same line)
                <td style="padding: 8px 12px; text-align: center;">
                    <span style="font-size: 13px; color: #cbd5e1;">{target_date}</span>
                    <span style="font-size: 12px; color: #64748b; margin-left: 6px;">{created_time}</span>
                </td>

                // Job Type Badge
                <td class="px-3 py-2 text-center">
                    {
                        let (badge_class, badge_text) = match job_type.as_str() {
                            "rewards_sync" => ("px-2 py-0.5 rounded-full text-xs font-medium bg-purple-500/20 text-purple-400", "Rewards"),
                            "licenses_sync" => ("px-2 py-0.5 rounded-full text-xs font-medium bg-cyan-500/20 text-cyan-400", "Licenses"),
                            _ => ("px-2 py-0.5 rounded-full text-xs font-medium bg-slate-500/20 text-slate-400", "Other"),
                        };
                        view! { <span class=badge_class>{badge_text}</span> }
                    }
                </td>

                // Status
                <td class="px-3 py-2 text-center">
                    <StatusBadge status=status.clone() />
                </td>

                // Duration
                <td class="px-3 py-2 text-sm text-slate-300 text-center">{duration}</td>

                // Records
                <td class="px-3 py-2 text-sm text-slate-300 text-center">
                    {format!("{} / {}", job.records_fetched, job.records_inserted)}
                </td>

                // Actions
                <td class="px-3 py-2 text-center">
                    <div style="display: flex; gap: 4px; justify-content: center; align-items: center;">
                        // Console button - Show logs in execution console
                        <button
                            style=move || {
                                let is_selected = selected_job_id.get().as_ref() == Some(&job_id_for_selected_check);
                                if is_selected {
                                    "display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #06b6d4; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                                } else {
                                    "display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #475569; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                                }
                            }
                            title="Show in Console"
                            on:click=move |_| on_select_job.run(job_id_for_select.clone())
                        >
                            <Icon name=IconName::Terminal size=14 />
                        </button>

                        // Rerun button - Green play icon
                        <button
                            style="display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #22c55e; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                            title="Re-run Job"
                            disabled=move || is_running || rerunning.get()
                            on:click=handle_rerun
                        >
                            {move || if rerunning.get() {
                                view! { <ProgressSpinner size=SpinnerSize::Small /> }.into_any()
                            } else {
                                view! { <Icon name=IconName::Play size=14 /> }.into_any()
                            }}
                        </button>

                        // Delete button - Red square
                        <button
                            style="display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #ef4444; color: white; border: none; border-radius: 6px; cursor: pointer; transition: background 0.15s;"
                            title="Delete Job"
                            disabled=move || is_running || deleting.get()
                            on:click=handle_delete
                        >
                            {move || if deleting.get() {
                                view! { <ProgressSpinner size=SpinnerSize::Small /> }.into_any()
                            } else {
                                view! { <Icon name=IconName::Trash size=14 /> }.into_any()
                            }}
                        </button>
                    </div>
                </td>
            </tr>

            // Expandable detail row
            {move || expanded_jobs.get().contains(&job_id_for_details).then(|| {
                let logs = job_logs.clone();
                let err_msg = error_message.clone();
                view! {
                    <tr class="bg-slate-800/30">
                        <td colspan="7" class="px-3 py-2">
                            // Error message if present
                            {if let Some(ref err) = err_msg {
                                if !err.is_empty() {
                                    // Parse error message to extract URL if present
                                    let (error_text, url_text) = if let Some(url_start) = err.find("(http") {
                                        let main_error = err[..url_start].trim().to_string();
                                        let url = err[url_start+1..err.len()-1].to_string();
                                        (main_error, Some(url))
                                    } else {
                                        (err.clone(), None)
                                    };

                                    view! {
                                        <div class="mb-3 p-4 bg-red-500/10 border border-red-500/30 rounded-lg">
                                            <div class="flex items-center gap-2 text-red-400 font-medium mb-2">
                                                <Icon name=IconName::AlertCircle size=18 />
                                                "Error"
                                            </div>
                                            <p class="text-red-300 text-sm mb-2">{error_text}</p>
                                            {url_text.map(|url| view! {
                                                <div class="mt-2 p-2 bg-slate-900/50 rounded border border-slate-700/50">
                                                    <span class="text-slate-500 text-xs">"URL: "</span>
                                                    <code class="text-red-400 text-xs break-all">{url}</code>
                                                </div>
                                            })}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            } else {
                                view! { <div></div> }.into_any()
                            }}

                            <JobContextDetails context=job_context.clone() />

                            // Job execution logs
                            {if !logs.is_empty() {
                                view! {
                                    <div class="mt-3">
                                        <h4 class="text-xs font-medium text-slate-500 uppercase mb-2">"Execution Logs"</h4>
                                        <div class="bg-slate-900 rounded-lg p-3 font-mono text-xs console-scroll" style="min-height: 150px; max-height: 250px;">
                                            {logs.iter().enumerate().map(|(idx, line)| {
                                                let color = if line.contains("ERROR") {
                                                    "color: #f87171;"
                                                } else if line.contains("WARN") {
                                                    "color: #fbbf24;"
                                                } else {
                                                    "color: #4ade80;"
                                                };
                                                let line_clone = line.clone();
                                                view! {
                                                    <div style=color>
                                                        <span style="color: #475569;">{format!("{:3} ", idx + 1)}</span>
                                                        {line_clone}
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}
                        </td>
                    </tr>
                }
            })}
        </>
    }
}

/// Jobs table component with pagination
#[component]
fn JobsTable(
    jobs: Vec<SyncJobEntity>,
    on_refresh: Callback<()>,
    expanded_jobs: ReadSignal<HashSet<String>>,
    set_expanded_jobs: WriteSignal<HashSet<String>>,
    selected_job_id: ReadSignal<Option<String>>,
    set_selected_job_id: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (current_page, set_current_page) = signal(1usize);
    let (page_size, set_page_size) = signal(5usize);
    let total_items = jobs.len();

    let toggle_expand = Callback::new(move |job_id: String| {
        set_expanded_jobs.update(|set| {
            if set.contains(&job_id) {
                set.remove(&job_id);
            } else {
                set.insert(job_id);
            }
        });
    });

    let select_job = Callback::new(move |job_id: String| {
        set_selected_job_id.set(Some(job_id));
    });

    view! {
        <div>
            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead>
                        <tr class="text-center text-xs font-medium text-slate-400 uppercase tracking-wider" style="background: rgba(30, 41, 59, 0.5); border-bottom: 1px solid rgba(51, 65, 85, 0.8);">
                            <th style="padding: 8px 4px 8px 8px; width: 36px;"></th>
                            <th style="padding: 8px 12px;">"Target Date"</th>
                            <th class="px-3 py-2">"Job Type"</th>
                            <th class="px-3 py-2">"Status"</th>
                            <th class="px-3 py-2">"Duration"</th>
                            <th class="px-3 py-2">"Records"</th>
                            <th class="px-3 py-2">"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            let page = current_page.get();
                            let size = page_size.get();

                            // Sort: running jobs first, then by created_at descending
                            let mut sorted_jobs = jobs.clone();
                            sorted_jobs.sort_by(|a, b| {
                                let a_running = a.status == "running";
                                let b_running = b.status == "running";
                                match (a_running, b_running) {
                                    (true, false) => std::cmp::Ordering::Less,
                                    (false, true) => std::cmp::Ordering::Greater,
                                    _ => b.created_at.cmp(&a.created_at),
                                }
                            });

                            let start = (page - 1) * size;
                            let end = std::cmp::min(start + size, total_items);

                            sorted_jobs[start..end].iter().enumerate().map(|(idx, job)| {
                                let job = job.clone();
                                let on_refresh = on_refresh.clone();
                                let toggle = toggle_expand.clone();
                                let select = select_job.clone();
                                view! { <JobRow job=job row_index=idx on_refresh=on_refresh expanded_jobs=expanded_jobs on_toggle_expand=toggle selected_job_id=selected_job_id on_select_job=select /> }
                            }).collect::<Vec<_>>()
                        }}
                    </tbody>
                </table>
            </div>

            // Pagination controls
            <div class="flex items-center justify-between px-4 py-2 border-t border-slate-700/50">
                // Page info on left
                <div class="flex items-center gap-3">
                    {move || {
                        let page = current_page.get();
                        let size = page_size.get();
                        let total_pages = (total_items + size - 1) / size;
                        let total_pages = if total_pages == 0 { 1 } else { total_pages };

                        view! {
                            <span class="text-slate-400 text-sm">
                                {format!("Page {} of {} ({} total)", page, total_pages, total_items)}
                            </span>
                        }
                    }}
                </div>

                // Page size and navigation on right
                <div class="flex items-center gap-3">
                    <select
                        style="padding: 5px 8px; background: #334155; border: 1px solid #475569; border-radius: 6px; color: #e2e8f0; font-size: 13px; cursor: pointer; outline: none;"
                        on:change=move |ev| {
                            use leptos::prelude::event_target_value;
                            if let Ok(size) = event_target_value(&ev).parse::<usize>() {
                                set_page_size.set(size);
                                set_current_page.set(1);
                            }
                        }
                    >
                        {PAGE_SIZE_OPTIONS.iter().map(|&size| {
                            view! {
                                <option value=size.to_string() selected=move || page_size.get() == size>
                                    {size}
                                </option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                    <div class="flex gap-2">
                        <button
                            style="padding: 6px 14px; background: #06b6d4; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 500; transition: opacity 0.15s;"
                            disabled=move || current_page.get() <= 1
                            on:click=move |_| set_current_page.update(|p| *p = (*p).saturating_sub(1).max(1))
                            class="disabled:opacity-40 disabled:cursor-not-allowed"
                        >
                            "Back"
                        </button>
                        <button
                            style="padding: 6px 14px; background: #06b6d4; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 500; transition: opacity 0.15s;"
                            disabled=move || {
                                let size = page_size.get();
                                let total_pages = (total_items + size - 1) / size;
                                let total_pages = if total_pages == 0 { 1 } else { total_pages };
                                current_page.get() >= total_pages
                            }
                            on:click=move |_| {
                                let size = page_size.get();
                                let total_pages = (total_items + size - 1) / size;
                                let total_pages = if total_pages == 0 { 1 } else { total_pages };
                                set_current_page.update(|p| *p = (*p + 1).min(total_pages))
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
}

/// Empty state when no jobs exist
#[component]
fn EmptyState() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-12 text-center">
            <div class="w-16 h-16 rounded-full bg-slate-700/50 flex items-center justify-center mb-4">
                <Icon name=IconName::Clock size=32 class="text-slate-500".to_string() />
            </div>
            <h3 class="text-lg font-medium text-slate-300 mb-2">"No sync jobs yet"</h3>
            <p class="text-sm text-slate-500 max-w-sm">
                "Sync jobs will appear here when rewards are synchronized from the UNetwork API. Click \"Run Sync Now\" to trigger a manual sync."
            </p>
        </div>
    }
}

/// Main jobs list page
#[component]
pub fn JobsListPage() -> impl IntoView {
    // Use RwSignal for jobs so WebSocket hook can update it
    let jobs = RwSignal::new(Vec::<SyncJobEntity>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);
    let (running_sync, set_running_sync) = signal(false);
    let (running_license_sync, set_running_license_sync) = signal(false);
    let (show_date_picker, set_show_date_picker) = signal(false);
    let (target_date, set_target_date) = signal(String::new());
    let (expanded_jobs, set_expanded_jobs) = signal(HashSet::<String>::new());
    // Console state
    let (console_expanded, set_console_expanded) = signal(false);
    // Selected job for console display
    let (selected_job_id, set_selected_job_id) = signal(Option::<String>::None);
    // Console scroll ref for auto-scroll to bottom
    let console_ref = NodeRef::<leptos::html::Div>::new();

    // Auto-scroll console to bottom when expanded or job changes
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        // Track these signals to trigger scroll
        let _ = console_expanded.get();
        let _ = selected_job_id.get();
        let _ = jobs.get();

        // Scroll to bottom after a small delay to allow DOM to update
        if let Some(el) = console_ref.get() {
            use wasm_bindgen::JsCast;
            let el: &web_sys::HtmlElement = el.as_ref();
            el.set_scroll_top(el.scroll_height());
        }
    });

    // Derived state: check if any job is running (computed once, not in view)
    let has_running_job = Memo::new(move |_| {
        jobs.get().iter().any(|j| j.status == "running")
    });

    // Real-time polling for job updates (WASM only)
    #[cfg(target_arch = "wasm32")]
    let (ws_state, ws_reconnect) = use_job_websocket(jobs);

    // Status indicator view fragment - unified component for both SSR and WASM
    #[cfg(target_arch = "wasm32")]
    let status_indicator = view! { <WsStatusIndicator ws_state=ws_state ws_reconnect=ws_reconnect /> };
    #[cfg(not(target_arch = "wasm32"))]
    let status_indicator = view! { <WsStatusIndicator /> };

    // Get dashboard state for badge clearing
    let dashboard = use_dashboard();

    // Get yesterday's date as default
    let yesterday = {
        let now = chrono::Utc::now();
        let yesterday = now - chrono::Duration::days(1);
        yesterday.format("%Y-%m-%d").to_string()
    };
    set_target_date.set(yesterday.clone());

    // Fetch jobs function
    let fetch_jobs = move || {
        set_loading.set(true);
        set_error.set(None);

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen_futures::spawn_local;
            spawn_local(async move {
                match list_sync_jobs().await {
                    Ok(result) => {
                        jobs.set(result);
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to load jobs: {}", e)));
                    }
                }
                set_loading.set(false);
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            set_loading.set(false);
        }
    };

    // Initial load and clear badge - runs once on mount
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::prelude::untrack;
        Effect::new(move |_| {
            // Use untrack to prevent signal tracking - this should only run once
            untrack(|| {
                // Clear the notification badge when user views jobs
                dashboard.clear_jobs_badge();
                // Update "last seen" timestamp to now
                let now = chrono::Utc::now().to_rfc3339();
                set_jobs_last_seen(&now);
                fetch_jobs();
            });
        });
    }

    // Auto-expand console disabled - was causing potential reactive loops
    // The console can still be manually expanded/collapsed
    // Users can use the Refresh button to update the job list

    // Refresh callback for child components
    let on_refresh = Callback::new(move |_: ()| {
        fetch_jobs();
    });

    // Run manual license sync
    let handle_run_license_sync = move |_| {
        set_running_license_sync.set(true);

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen_futures::spawn_local;
            spawn_local(async move {
                match run_license_sync_job().await {
                    Ok(_job) => {
                        // Refresh the list
                        fetch_jobs();
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to run license sync: {}", e).into());
                        set_error.set(Some(format!("Failed to run license sync: {}", e)));
                    }
                }
                set_running_license_sync.set(false);
            });
        }
    };

    // Run manual sync
    let handle_run_sync = move |_| {
        let date = target_date.get();
        if date.is_empty() {
            return;
        }

        set_running_sync.set(true);
        set_show_date_picker.set(false);

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen_futures::spawn_local;
            spawn_local(async move {
                match run_sync_job(date).await {
                    Ok(_job) => {
                        // Refresh the list
                        fetch_jobs();
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to run sync: {}", e).into());
                        set_error.set(Some(format!("Failed to run sync: {}", e)));
                    }
                }
                set_running_sync.set(false);
            });
        }
    };

    view! {
        <div class="flex flex-col h-full">
            <Header title="Sync Jobs".to_string() />

            <div class="flex-1 p-6 overflow-auto">
                // Action bar
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-4">
                        <h2 class="text-lg font-semibold text-slate-200">"Sync Jobs"</h2>
                        <span class="text-sm text-slate-500">
                            {move || format!("{} jobs", jobs.get().len())}
                        </span>
                    </div>

                    <div class="flex items-center gap-3">
                        // Live polling status indicator
                        {status_indicator}

                        // Refresh button
                        <button
                            class="px-3 py-2 rounded-lg bg-slate-700 hover:bg-slate-600 text-slate-300 hover:text-white transition-colors flex items-center gap-2"
                            on:click=move |_| fetch_jobs()
                            disabled=move || loading.get()
                        >
                            <Icon name=IconName::Refresh size=16 />
                            "Refresh"
                        </button>

                        // Sync Licenses button
                        <button
                            class="px-4 py-2 rounded-lg bg-cyan-600 hover:bg-cyan-500 text-white font-medium transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
                            on:click=handle_run_license_sync
                            disabled=move || running_license_sync.get()
                        >
                            {move || if running_license_sync.get() {
                                view! {
                                    <ProgressSpinner size=SpinnerSize::Small />
                                    "Syncing..."
                                }.into_any()
                            } else {
                                view! {
                                    <Icon name=IconName::Server size=16 />
                                    "Sync Licenses"
                                }.into_any()
                            }}
                        </button>

                        // Run Sync button with date picker
                        <div class="relative">
                            <button
                                class="px-4 py-2 rounded-lg bg-primary-600 hover:bg-primary-500 text-white font-medium transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
                                on:click=move |_| set_show_date_picker.update(|v| *v = !*v)
                                disabled=move || running_sync.get()
                            >
                                {move || if running_sync.get() {
                                    view! {
                                        <ProgressSpinner size=SpinnerSize::Small />
                                        "Running..."
                                    }.into_any()
                                } else {
                                    view! {
                                        <Icon name=IconName::Play size=16 />
                                        "Run Sync Now"
                                    }.into_any()
                                }}
                            </button>

                            // Date picker modal with opaque backdrop
                            {move || show_date_picker.get().then(|| view! {
                                // Backdrop - click to close
                                <div
                                    style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.7); z-index: 100; display: flex; align-items: center; justify-content: center;"
                                    on:click=move |_| set_show_date_picker.set(false)
                                >
                                    // Modal content
                                    <div
                                        style="background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 24px; min-width: 300px;"
                                        on:click=move |e| e.stop_propagation()
                                    >
                                        <h3 style="font-size: 16px; font-weight: 600; color: #e2e8f0; margin-bottom: 16px;">"Select Target Date"</h3>
                                        <input
                                            type="date"
                                            style="width: 100%; padding: 10px 12px; background: #334155; border: 1px solid #475569; border-radius: 8px; color: #e2e8f0; font-size: 14px; outline: none;"
                                            prop:value=move || target_date.get()
                                            on:input=move |ev| {
                                                use leptos::prelude::event_target_value;
                                                set_target_date.set(event_target_value(&ev));
                                            }
                                        />
                                        <div style="display: flex; gap: 8px; margin-top: 20px;">
                                            <button
                                                style="flex: 1; padding: 10px 16px; background: #475569; color: #e2e8f0; border: none; border-radius: 8px; cursor: pointer; font-size: 14px;"
                                                on:click=move |_| set_show_date_picker.set(false)
                                            >
                                                "Cancel"
                                            </button>
                                            <button
                                                style="flex: 1; padding: 10px 16px; background: #3b82f6; color: white; border: none; border-radius: 8px; cursor: pointer; font-size: 14px; font-weight: 500;"
                                                on:click=handle_run_sync.clone()
                                            >
                                                "Start Sync"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            })}
                        </div>
                    </div>
                </div>

                // Error message
                {move || error.get().map(|e| view! {
                    <div class="mb-4 p-4 bg-red-500/10 border border-red-500/30 rounded-lg text-red-400">
                        {e}
                    </div>
                })}

                // Main content
                <div class="bg-slate-800/50 rounded-xl border border-slate-700/50">
                    {move || {
                        if loading.get() {
                            view! {
                                <div class="flex items-center justify-center py-12">
                                    <ProgressSpinner size=SpinnerSize::Large />
                                </div>
                            }.into_any()
                        } else {
                            let current_jobs = jobs.get();
                            if current_jobs.is_empty() {
                                view! { <EmptyState /> }.into_any()
                            } else {
                                view! { <JobsTable jobs=current_jobs on_refresh=on_refresh.clone() expanded_jobs=expanded_jobs set_expanded_jobs=set_expanded_jobs selected_job_id=selected_job_id set_selected_job_id=set_selected_job_id /> }.into_any()
                            }
                        }
                    }}
                </div>

                // Job Execution Consoles - selected job first, then running jobs
                <div class="mt-2 space-y-2">
                    {move || {
                        let current_jobs = jobs.get();
                        let sel_id = selected_job_id.get();

                        // Find selected job if any
                        let selected_job = sel_id.as_ref()
                            .and_then(|id| current_jobs.iter().find(|j| &j.id == id))
                            .cloned();

                        // Find running jobs (excluding selected if it's also running)
                        let other_running: Vec<_> = current_jobs.iter()
                            .filter(|j| j.status == "running" && sel_id.as_ref().map_or(true, |id| &j.id != id))
                            .cloned()
                            .collect();

                        // Build consoles list
                        let mut consoles: Vec<_> = Vec::new();

                        // Selected job console (expanded)
                        if let Some(job) = selected_job {
                            consoles.push(view! { <SingleJobConsole job=job is_collapsed=false /> }.into_any());
                        }

                        // Other running jobs (expanded)
                        for job in other_running {
                            consoles.push(view! { <SingleJobConsole job=job is_collapsed=false /> }.into_any());
                        }

                        // If no selected and no running, show most recent with logs (collapsed)
                        if consoles.is_empty() {
                            if let Some(job) = current_jobs.iter().find(|j| j.job_logs.is_some()).cloned() {
                                consoles.push(view! { <SingleJobConsole job=job is_collapsed=true /> }.into_any());
                            } else {
                                consoles.push(view! {
                                    <div class="bg-slate-900 rounded-xl border border-slate-700/50 p-4">
                                        <p class="text-slate-500 italic text-sm">"No job logs available. Run a sync to see execution console."</p>
                                    </div>
                                }.into_any());
                            }
                        }

                        consoles
                    }}
                </div>
            </div>
        </div>
    }
}
