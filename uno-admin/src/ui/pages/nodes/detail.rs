//! Node detail page with database integration

use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_navigate};
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::handler::get_node_with_licenses;
use crate::models::entity::{NodeEntity, LicenseEntity};

/// Node detail page
#[component]
pub fn NodeDetailPage() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();

    let node_id = move || {
        params.get().get("id").map(|s| s.clone()).unwrap_or_default()
    };

    // Fetch node with licenses from database
    let node_resource = Resource::new(
        move || node_id(),
        |id| async move {
            if id.is_empty() {
                return Ok(None);
            }
            get_node_with_licenses(id).await
        }
    );

    // Navigation handlers
    let nav_license = navigate.clone();
    let on_license_click = move |license_id: String| {
        nav_license(&format!("/licenses/{}", license_id), Default::default());
    };

    view! {
        <div>
            <Header title="Node Details".to_string() show_search=false />

            <div class="px-4 py-4 space-y-4">
                // Back button
                <button
                    class="flex items-center gap-2 text-sm text-primary-500 hover:text-primary-600"
                    on:click=move |_| {
                        navigate("/nodes", Default::default());
                    }
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                    </svg>
                    "Back to Nodes"
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
                        match node_resource.get() {
                            None => view! {
                                <div class="text-center py-12 text-slate-500">"Loading..."</div>
                            }.into_any(),
                            Some(Err(e)) => view! {
                                <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
                                    <strong>"Error: "</strong>{e.to_string()}
                                </div>
                            }.into_any(),
                            Some(Ok(None)) => view! {
                                <div class="text-center py-12">
                                    <div class="text-slate-400 dark:text-slate-500 mb-4">
                                        <Icon name=IconName::Server size=48 class="mx-auto opacity-50".to_string() />
                                    </div>
                                    <p class="text-slate-500 dark:text-slate-400">
                                        "Node not found"
                                    </p>
                                </div>
                            }.into_any(),
                            Some(Ok(Some((node, licenses)))) => {
                                let license_click = on_license_click.clone();
                                view! {
                                    <NodeDetails
                                        node=node
                                        licenses=licenses
                                        on_license_click=license_click
                                    />
                                }.into_any()
                            }
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}

/// Node details component
#[component]
fn NodeDetails(
    node: NodeEntity,
    licenses: Vec<LicenseEntity>,
    on_license_click: impl Fn(String) + Clone + 'static,
) -> impl IntoView {
    let online_count = licenses.iter().filter(|l| l.is_online).count();
    let total_count = licenses.len();
    let avg_uptime = if total_count > 0 {
        licenses.iter().map(|l| l.uptime).sum::<f64>() / total_count as f64 * 100.0
    } else {
        0.0
    };

    view! {
        <div class="space-y-4">
            // Node info card
            <div class="bg-white dark:bg-slate-800 rounded-xl p-4 border border-slate-200 dark:border-slate-700">
                <div class="flex items-center gap-4">
                    <div class="w-14 h-14 rounded-full bg-slate-100 dark:bg-slate-700 flex items-center justify-center">
                        <Icon name=IconName::Server size=24 class="text-slate-600 dark:text-slate-300".to_string() />
                    </div>
                    <div class="flex-1">
                        <h2 class="text-sm font-medium text-slate-500 dark:text-slate-400">"Node ID"</h2>
                        <div class="font-mono text-sm text-slate-900 dark:text-white break-all">
                            {node.node_id.clone()}
                        </div>
                        {node.name.clone().map(|name| view! {
                            <div class="text-sm text-slate-600 dark:text-slate-300 mt-1">{name}</div>
                        })}
                    </div>
                </div>
            </div>

            // Summary stats
            <div class="grid grid-cols-3 gap-3">
                <SummaryCard label="Licenses".to_string() value=total_count.to_string() />
                <SummaryCard
                    label="Online".to_string()
                    value=format!("{} ({:.0}%)",
                        online_count,
                        if total_count > 0 { online_count as f64 / total_count as f64 * 100.0 } else { 0.0 }
                    )
                />
                <SummaryCard label="Avg Uptime".to_string() value=format!("{:.1}%", avg_uptime) />
            </div>

            // Licenses table
            <div class="bg-white dark:bg-slate-800 rounded-2xl overflow-hidden border border-slate-200 dark:border-slate-700">
                <div class="p-4 border-b border-slate-200 dark:border-slate-700">
                    <h3 class="text-base font-semibold text-slate-900 dark:text-white">
                        {format!("Licenses in this Node ({})", licenses.len())}
                    </h3>
                </div>

                {if licenses.is_empty() {
                    view! {
                        <div class="p-8 text-center text-slate-500 dark:text-slate-400">
                            "No licenses associated with this node"
                        </div>
                    }.into_any()
                } else {
                    let click_handler = on_license_click.clone();
                    view! {
                        <div class="divide-y divide-slate-200 dark:divide-slate-700">
                            {licenses.into_iter().map(|license| {
                                let id = license.license_id.clone();
                                let click = click_handler.clone();
                                view! {
                                    <LicenseRow
                                        license=license
                                        on_click=move || click(id.clone())
                                    />
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
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
        <div class="bg-white dark:bg-slate-800 rounded-xl p-4 border border-slate-200 dark:border-slate-700">
            <div class="text-sm text-slate-500 dark:text-slate-400 mb-1">{label}</div>
            <div class="text-xl font-semibold text-slate-900 dark:text-white">{value}</div>
        </div>
    }
}

/// License row component
#[component]
fn LicenseRow(
    license: LicenseEntity,
    on_click: impl Fn() + 'static,
) -> impl IntoView {
    let status_class = if license.is_online {
        "bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400"
    } else {
        "bg-slate-100 text-slate-500 dark:bg-slate-700 dark:text-slate-400"
    };

    view! {
        <div
            class="p-4 hover:bg-slate-50 dark:hover:bg-slate-700/50 cursor-pointer transition"
            on:click=move |_| on_click()
        >
            <div class="flex items-center justify-between">
                <div>
                    <div class="font-mono text-sm text-slate-900 dark:text-white">
                        {license.license_id_short()}
                    </div>
                    <div class="text-xs text-slate-500 dark:text-slate-400 mt-1">
                        {license.ulo_name.clone()}
                        {license.alias.clone().map(|a| format!(" • {}", a))}
                    </div>
                </div>
                <div class="flex items-center gap-3">
                    <div class="text-right">
                        <div class="text-sm text-slate-600 dark:text-slate-300">
                            {format!("{:.1}%", license.uptime_percentage())}
                        </div>
                    </div>
                    <span class={format!("px-2 py-0.5 rounded text-xs font-medium {}", status_class)}>
                        {if license.is_online { "Online" } else { "Offline" }}
                    </span>
                </div>
            </div>
        </div>
    }
}
