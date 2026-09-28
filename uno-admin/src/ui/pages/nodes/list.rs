//! Node list page with database integration

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::handler::{list_nodes, get_nodes_summary, NodesSummaryDto};
use crate::models::entity::NodeEntity;

/// Node list page
#[component]
pub fn NodeListPage() -> impl IntoView {
    let navigate = use_navigate();

    // Fetch nodes from database
    let nodes_resource = Resource::new(
        || (),
        |_| async move { list_nodes().await }
    );

    // Fetch summary
    let summary_resource = Resource::new(
        || (),
        |_| async move { get_nodes_summary().await }
    );

    // Navigation handler
    let nav = navigate.clone();
    let on_node_click = move |node_id: String| {
        nav(&format!("/nodes/{}", node_id), Default::default());
    };

    view! {
        <div>
            <Header title="Nodes".to_string() show_search=false />

            <div class="px-4 py-4 space-y-4">
                // Summary cards
                <Suspense fallback=move || view! {
                    <div class="grid grid-cols-3 gap-3">
                        <SummaryCard label="Nodes".to_string() value="...".to_string() />
                        <SummaryCard label="Licenses".to_string() value="...".to_string() />
                        <SummaryCard label="Online".to_string() value="...".to_string() />
                    </div>
                }>
                    {move || {
                        summary_resource.get().map(|result| match result {
                            Ok(summary) => view! {
                                <div class="grid grid-cols-3 gap-3">
                                    <SummaryCard label="Total Nodes".to_string() value=summary.total_nodes.to_string() />
                                    <SummaryCard label="Licenses".to_string() value=summary.total_licenses.to_string() />
                                    <SummaryCard label="Online".to_string() value=summary.total_online.to_string() />
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <div class="text-red-500 text-sm">{e.to_string()}</div>
                            }.into_any()
                        })
                    }}
                </Suspense>

                // Node list
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
                        match nodes_resource.get() {
                            None => view! {
                                <div class="text-center py-12 text-slate-500">"Loading..."</div>
                            }.into_any(),
                            Some(Err(e)) => view! {
                                <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
                                    <strong>"Error: "</strong>{e.to_string()}
                                </div>
                            }.into_any(),
                            Some(Ok(nodes)) => {
                                if nodes.is_empty() {
                                    view! {
                                        <div class="text-center py-12">
                                            <div class="text-slate-400 dark:text-slate-500 mb-4">
                                                <Icon name=IconName::Server size=48 class="mx-auto opacity-50".to_string() />
                                            </div>
                                            <p class="text-slate-500 dark:text-slate-400">
                                                "No nodes found. Nodes are created automatically when licenses are imported."
                                            </p>
                                        </div>
                                    }.into_any()
                                } else {
                                    let click_handler = on_node_click.clone();
                                    view! {
                                        <div class="bg-white dark:bg-slate-800 rounded-2xl overflow-hidden border border-slate-200 dark:border-slate-700">
                                            <div class="p-4 border-b border-slate-200 dark:border-slate-700">
                                                <h3 class="text-base font-semibold text-slate-900 dark:text-white">
                                                    {format!("Nodes ({})", nodes.len())}
                                                </h3>
                                            </div>
                                            <div class="divide-y divide-slate-200 dark:divide-slate-700">
                                                {nodes.into_iter().map(|node| {
                                                    let id = node.node_id.clone();
                                                    let click = click_handler.clone();
                                                    view! {
                                                        <NodeRow
                                                            node=node
                                                            on_click=move || click(id.clone())
                                                        />
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                            }
                        }
                    }}
                </Suspense>
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

/// Node row component
#[component]
fn NodeRow(
    node: NodeEntity,
    on_click: impl Fn() + 'static,
) -> impl IntoView {
    let online_class = if node.online_licenses > 0 {
        "text-green-500"
    } else {
        "text-slate-400"
    };

    view! {
        <div
            class="flex items-center justify-between p-4 hover:bg-slate-50 dark:hover:bg-slate-700/50 cursor-pointer transition-colors"
            on:click=move |_| on_click()
        >
            <div class="flex-1 min-w-0">
                <div class="font-mono text-sm text-slate-900 dark:text-white mb-1">
                    {node.node_id_short()}
                </div>
                <div class="text-xs text-slate-500 dark:text-slate-400">
                    {node.name.clone().unwrap_or_else(|| format!("{} licenses", node.total_licenses))}
                </div>
            </div>

            <div class="flex items-center gap-4">
                <div class="text-right">
                    <div class={format!("text-sm font-medium {}", online_class)}>
                        {format!("{}/{}", node.online_licenses, node.total_licenses)}
                    </div>
                    <div class="text-xs text-slate-400">"Online"</div>
                </div>

                <svg class="w-5 h-5 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
            </div>
        </div>
    }
}
