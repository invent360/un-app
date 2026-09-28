//! Agent table component - using ember-fx styled table

use leptos::prelude::*;
use leptos::callback::Callback;
use crate::models::AgentPerformance;
use super::{UptimeDisplay, AmountDisplay};
// NOTE: ember_fx_components not available - using stub
// use ember_fx_components::try_use_theme;

/// Stub for try_use_theme (ember_fx_components not available)
fn try_use_theme() -> Option<StubThemeContext> {
    None
}

/// Stub theme context
#[allow(dead_code)]
struct StubThemeContext;
impl StubThemeContext {
    #[allow(dead_code)]
    fn class_prefix(&self) -> &str { "prime" }
}

/// Agent table row component
#[component]
pub fn AgentRow(
    agent: AgentPerformance,
    #[prop(into)] table_prefix: String,
    #[prop(into, optional)] on_click: Option<Box<dyn Fn(String) + 'static>>,
) -> impl IntoView {
    let agent_name = agent.agent_name.clone();
    let agent_for_click = agent_name.clone();
    let row_class = format!("{}-row", table_prefix);
    let cell_class = format!("{}-cell", table_prefix);

    view! {
        <tr
            class=row_class
            style="cursor: pointer"
            on:click=move |_| {
                if let Some(ref cb) = on_click {
                    cb(agent_for_click.clone());
                }
            }
        >
            <td class=cell_class.clone()>
                <span class="font-medium">
                    {agent_name}
                </span>
            </td>
            <td class=cell_class.clone() style="text-align: center">
                {agent.ulo_count}
            </td>
            <td class=cell_class.clone() style="text-align: center">
                {agent.license_count}
            </td>
            <td class=cell_class.clone()>
                <AmountDisplay amount_micros=agent.total_earnings_micros decimals=4 />
            </td>
            <td class=cell_class.clone()>
                <span class="font-medium text-green-500">
                    <AmountDisplay amount_micros=agent.agent_share_micros decimals=4 />
                </span>
            </td>
            <td class=cell_class.clone()>
                <UptimeDisplay uptime=agent.avg_uptime />
            </td>
            <td class=cell_class.clone() style="text-align: center">
                <span class=format!("{}", if agent.online_licenses > 0 { "text-green-500" } else { "text-slate-400" })>
                    {format!("{}/{}", agent.online_licenses, agent.license_count)}
                </span>
            </td>
        </tr>
    }
}

/// Agent table component using ember-fx styling
#[component]
pub fn AgentTable(
    agents: Vec<AgentPerformance>,
    #[prop(optional)] on_agent_click: Option<Callback<String>>,
    #[prop(default = false)] loading: bool,
) -> impl IntoView {
    // Get ember-fx theme context for CSS class prefix
    // NOTE: try_use_theme stubbed to return None, always uses "prime"
    let _theme_ctx = try_use_theme();
    let design_system = "prime";

    let table_prefix = format!("fx-table-{}", design_system);
    let wrapper_class = format!("{}-wrapper", table_prefix);
    let table_class = format!("{} {}-default {}-bordered {}-striped {}-hoverable",
        table_prefix, table_prefix, table_prefix, table_prefix, table_prefix);
    let thead_class = format!("{}-thead", table_prefix);
    let tbody_class = format!("{}-tbody", table_prefix);
    let row_class = format!("{}-row", table_prefix);
    let header_cell_class = format!("{}-cell {}-cell-header", table_prefix, table_prefix);
    let loading_class = format!("{}-loading", table_prefix);
    let empty_class = format!("{}-empty", table_prefix);

    if loading {
        return view! {
            <div class=wrapper_class>
                <div class=loading_class>
                    <span class=format!("{}-loading-spinner", table_prefix)>"Loading agents..."</span>
                </div>
            </div>
        }.into_any();
    }

    if agents.is_empty() {
        return view! {
            <div class=wrapper_class>
                <table class=table_class>
                    <tbody class=tbody_class>
                        <tr class=row_class>
                            <td class=empty_class colspan="7">"No agents found"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        }.into_any();
    }

    let table_prefix_for_rows = table_prefix.clone();

    view! {
        <div class=wrapper_class>
            <table class=table_class>
                <thead class=thead_class>
                    <tr class=row_class>
                        <th class=header_cell_class.clone()>"Agent"</th>
                        <th class=header_cell_class.clone() style="text-align: center">"ULOs"</th>
                        <th class=header_cell_class.clone() style="text-align: center">"Licenses"</th>
                        <th class=header_cell_class.clone()>"Earnings"</th>
                        <th class=header_cell_class.clone()>"Commission"</th>
                        <th class=header_cell_class.clone()>"Avg Uptime"</th>
                        <th class=header_cell_class.clone() style="text-align: center">"Online"</th>
                    </tr>
                </thead>
                <tbody class=tbody_class>
                    {match on_agent_click.clone() {
                        Some(cb) => {
                            agents.into_iter().map(move |agent| {
                                let cb_clone = cb.clone();
                                let cb_fn = Box::new(move |s: String| cb_clone.run(s)) as Box<dyn Fn(String) + 'static>;
                                let prefix = table_prefix_for_rows.clone();
                                view! {
                                    <AgentRow
                                        agent=agent
                                        table_prefix=prefix
                                        on_click=cb_fn
                                    />
                                }
                            }).collect::<Vec<_>>()
                        }
                        None => {
                            agents.into_iter().map(|agent| {
                                let prefix = table_prefix_for_rows.clone();
                                view! {
                                    <AgentRow agent=agent table_prefix=prefix />
                                }
                            }).collect::<Vec<_>>()
                        }
                    }}
                </tbody>
            </table>
        </div>
    }.into_any()
}

/// Compact agent card for mobile view
#[component]
pub fn AgentCard(
    agent: AgentPerformance,
    #[prop(optional)] on_click: Option<Box<dyn Fn(String) + 'static>>,
) -> impl IntoView {
    let agent_name = agent.agent_name.clone();
    let agent_for_click = agent_name.clone();

    view! {
        <div
            class="bg-white dark:bg-slate-800 rounded-xl p-4 border border-slate-200 dark:border-slate-700 hover:border-primary-300 dark:hover:border-primary-600 cursor-pointer transition-colors"
            on:click=move |_| {
                if let Some(ref cb) = on_click {
                    cb(agent_for_click.clone());
                }
            }
        >
            <div class="flex items-start justify-between mb-3">
                <h4 class="font-medium text-slate-900 dark:text-white">
                    {agent_name}
                </h4>
                <span class="text-xs px-2 py-1 rounded-full bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400">
                    {format!("{}/{} online", agent.online_licenses, agent.license_count)}
                </span>
            </div>

            <div class="grid grid-cols-2 gap-y-2 text-sm">
                <div>
                    <span class="text-slate-500 dark:text-slate-400">"ULOs: "</span>
                    <span class="text-slate-700 dark:text-slate-300 font-medium">{agent.ulo_count}</span>
                </div>
                <div>
                    <span class="text-slate-500 dark:text-slate-400">"Licenses: "</span>
                    <span class="text-slate-700 dark:text-slate-300 font-medium">{agent.license_count}</span>
                </div>
                <div>
                    <span class="text-slate-500 dark:text-slate-400">"Earnings: "</span>
                    <AmountDisplay amount_micros=agent.total_earnings_micros decimals=2 />
                </div>
                <div>
                    <span class="text-slate-500 dark:text-slate-400">"Commission: "</span>
                    <span class="text-green-500 font-medium">
                        <AmountDisplay amount_micros=agent.agent_share_micros decimals=2 />
                    </span>
                </div>
            </div>

            <div class="mt-3 pt-3 border-t border-slate-100 dark:border-slate-700">
                <div class="flex items-center justify-between text-xs">
                    <span class="text-slate-500 dark:text-slate-400">"Avg Uptime"</span>
                    <UptimeDisplay uptime=agent.avg_uptime />
                </div>
            </div>
        </div>
    }
}

/// Agent summary row for dashboard
#[component]
pub fn TopAgentRow(
    agent: AgentPerformance,
    rank: usize,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between py-2 border-b border-slate-100 dark:border-slate-700 last:border-0">
            <div class="flex items-center gap-3">
                <span class=format!("w-6 h-6 rounded-full flex items-center justify-center text-xs font-medium {}",
                    match rank {
                        1 => "bg-amber-100 dark:bg-amber-900/30 text-amber-600 dark:text-amber-400",
                        2 => "bg-slate-200 dark:bg-slate-600 text-slate-600 dark:text-slate-300",
                        3 => "bg-orange-100 dark:bg-orange-900/30 text-orange-600 dark:text-orange-400",
                        _ => "bg-slate-100 dark:bg-slate-700 text-slate-500 dark:text-slate-400"
                    }
                )>
                    {rank}
                </span>
                <span class="font-medium text-slate-900 dark:text-white text-sm">
                    {agent.agent_name}
                </span>
            </div>
            <div class="text-right">
                <div class="text-green-500 font-medium text-sm">
                    <AmountDisplay amount_micros=agent.agent_share_micros decimals=2 />
                </div>
                <div class="text-xs text-slate-400">
                    {format!("{} licenses", agent.license_count)}
                </div>
            </div>
        </div>
    }
}
