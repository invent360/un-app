//! License table component - using ember-fx styled table

use leptos::prelude::*;
use leptos::callback::Callback;
use crate::models::LicenseConfig;
use super::{StatusBadge, UptimeDisplay, TruncatedId};
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

/// License table row component
#[component]
pub fn LicenseRow(
    license: LicenseConfig,
    #[prop(into)] table_prefix: String,
    #[prop(into, optional)] on_click: Option<Box<dyn Fn(String) + 'static>>,
) -> impl IntoView {
    let license_id = license.license_id.clone();
    let license_for_click = license_id.clone();
    let row_class = format!("{}-row", table_prefix);
    let cell_class = format!("{}-cell", table_prefix);

    view! {
        <tr
            class=row_class
            style="cursor: pointer"
            on:click=move |_| {
                if let Some(ref cb) = on_click {
                    cb(license_for_click.clone());
                }
            }
        >
            <td class=cell_class.clone()>
                <div class="flex flex-col">
                    <span class="font-medium">
                        {license.alias.clone()}
                    </span>
                    <TruncatedId id=license_id />
                </div>
            </td>
            <td class=cell_class.clone()>
                <TruncatedId id=license.node_id.clone() />
            </td>
            <td class=cell_class.clone()>
                {if license.agent_name.is_empty() { "-".to_string() } else { license.agent_name.clone() }}
            </td>
            <td class=cell_class.clone()>
                {if license.ulo_name.is_empty() { "-".to_string() } else { license.ulo_name.clone() }}
            </td>
            <td class=cell_class.clone()>
                <UptimeDisplay uptime=license.uptime />
            </td>
            <td class=cell_class.clone()>
                <StatusBadge
                    status=if license.is_online { "Online" } else { "Offline" }
                    online=license.is_online
                />
            </td>
        </tr>
    }
}

/// License table component using ember-fx styling
#[component]
pub fn LicenseTable(
    licenses: Vec<LicenseConfig>,
    #[prop(optional)] on_license_click: Option<Callback<String>>,
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
                    <span class=format!("{}-loading-spinner", table_prefix)>"Loading licenses..."</span>
                </div>
            </div>
        }.into_any();
    }

    if licenses.is_empty() {
        return view! {
            <div class=wrapper_class>
                <table class=table_class>
                    <tbody class=tbody_class>
                        <tr class=row_class>
                            <td class=empty_class colspan="6">"No licenses found"</td>
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
                        <th class=header_cell_class.clone()>"License"</th>
                        <th class=header_cell_class.clone()>"Node"</th>
                        <th class=header_cell_class.clone()>"Agent"</th>
                        <th class=header_cell_class.clone()>"ULO"</th>
                        <th class=header_cell_class.clone()>"Uptime"</th>
                        <th class=header_cell_class.clone()>"Status"</th>
                    </tr>
                </thead>
                <tbody class=tbody_class>
                    {match on_license_click.clone() {
                        Some(cb) => {
                            licenses.into_iter().map(move |license| {
                                let cb_clone = cb.clone();
                                let cb_fn = Box::new(move |s: String| cb_clone.run(s)) as Box<dyn Fn(String) + 'static>;
                                let prefix = table_prefix_for_rows.clone();
                                view! {
                                    <LicenseRow
                                        license=license
                                        table_prefix=prefix
                                        on_click=cb_fn
                                    />
                                }
                            }).collect::<Vec<_>>()
                        }
                        None => {
                            licenses.into_iter().map(|license| {
                                let prefix = table_prefix_for_rows.clone();
                                view! {
                                    <LicenseRow license=license table_prefix=prefix />
                                }
                            }).collect::<Vec<_>>()
                        }
                    }}
                </tbody>
            </table>
        </div>
    }.into_any()
}

/// Compact license card for mobile view
#[component]
pub fn LicenseCard(
    license: LicenseConfig,
    #[prop(optional)] on_click: Option<Box<dyn Fn(String) + 'static>>,
) -> impl IntoView {
    let license_id = license.license_id.clone();
    let license_for_click = license_id.clone();
    let alias = license.alias.clone();
    let agent_name = if license.agent_name.is_empty() { "-".to_string() } else { license.agent_name.clone() };
    let ulo_name = if license.ulo_name.is_empty() { "-".to_string() } else { license.ulo_name.clone() };
    let uptime = license.uptime;
    let is_online = license.is_online;
    let split_str = format!("{}%/{}%", license.splits.uno_share as i32, license.splits.agent_share as i32);

    view! {
        <div
            class="bg-white dark:bg-slate-800 rounded-xl p-4 border border-slate-200 dark:border-slate-700 hover:border-primary-300 dark:hover:border-primary-600 cursor-pointer transition-colors"
            on:click=move |_| {
                if let Some(ref cb) = on_click {
                    cb(license_for_click.clone());
                }
            }
        >
            <div class="flex items-start justify-between mb-3">
                <div>
                    <h4 class="font-medium text-slate-900 dark:text-white">
                        {alias}
                    </h4>
                    <TruncatedId id=license_id />
                </div>
                <StatusBadge
                    status=if is_online { "Online" } else { "Offline" }
                    online=is_online
                />
            </div>

            <div class="grid grid-cols-2 gap-2 text-sm">
                <div>
                    <span class="text-slate-500 dark:text-slate-400">"Agent: "</span>
                    <span class="text-slate-700 dark:text-slate-300">{agent_name}</span>
                </div>
                <div>
                    <span class="text-slate-500 dark:text-slate-400">"ULO: "</span>
                    <span class="text-slate-700 dark:text-slate-300">{ulo_name}</span>
                </div>
                <div>
                    <span class="text-slate-500 dark:text-slate-400">"Uptime: "</span>
                    <UptimeDisplay uptime=uptime />
                </div>
                <div>
                    <span class="text-slate-500 dark:text-slate-400">"Split: "</span>
                    <span class="text-slate-700 dark:text-slate-300">{split_str}</span>
                </div>
            </div>
        </div>
    }
}
