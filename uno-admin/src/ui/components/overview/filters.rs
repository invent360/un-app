//! Filter controls for dashboard overview sections

use leptos::prelude::*;
use crate::state::{DateRange, DateRangePreset, TimeGranularity};

/// Overview filters component with date range and granularity dropdowns
#[component]
pub fn OverviewFilters(
    date_range: RwSignal<DateRange>,
    granularity: RwSignal<TimeGranularity>,
    #[prop(default = false)] compact: bool,
) -> impl IntoView {
    let select_class = if compact {
        "px-4 py-2 text-sm bg-slate-100 dark:bg-slate-700 border border-slate-200 dark:border-slate-600 rounded-lg text-slate-700 dark:text-slate-300 focus:outline-none focus:ring-2 focus:ring-primary-500 cursor-pointer"
    } else {
        "px-4 py-2 text-sm bg-slate-100 dark:bg-slate-700 border border-slate-200 dark:border-slate-600 rounded-lg text-slate-700 dark:text-slate-300 focus:outline-none focus:ring-2 focus:ring-primary-500 cursor-pointer"
    };

    view! {
        <div class="flex items-center gap-2">
            // Date Range Dropdown
            <select
                class=select_class
                on:change=move |ev| {
                    use leptos::wasm_bindgen::JsCast;
                    let target = event_target::<web_sys::HtmlSelectElement>(&ev);
                    let value = target.value();
                    let preset = match value.as_str() {
                        "7d" => DateRangePreset::Last7Days,
                        "30d" => DateRangePreset::Last30Days,
                        "90d" => DateRangePreset::Last90Days,
                        "1y" => DateRangePreset::LastYear,
                        "all" => DateRangePreset::AllTime,
                        _ => DateRangePreset::Last30Days,
                    };
                    date_range.set(DateRange::from_preset(preset));
                }
            >
                <option value="7d" selected=move || date_range.get().preset == DateRangePreset::Last7Days>
                    "Last 7 Days"
                </option>
                <option value="30d" selected=move || date_range.get().preset == DateRangePreset::Last30Days>
                    "Last 30 Days"
                </option>
                <option value="90d" selected=move || date_range.get().preset == DateRangePreset::Last90Days>
                    "Last 90 Days"
                </option>
                <option value="1y" selected=move || date_range.get().preset == DateRangePreset::LastYear>
                    "Last Year"
                </option>
                <option value="all" selected=move || date_range.get().preset == DateRangePreset::AllTime>
                    "All Time"
                </option>
            </select>

            // Granularity Dropdown
            <select
                class=select_class
                on:change=move |ev| {
                    use leptos::wasm_bindgen::JsCast;
                    let target = event_target::<web_sys::HtmlSelectElement>(&ev);
                    let value = target.value();
                    let g = match value.as_str() {
                        "daily" => TimeGranularity::Daily,
                        "weekly" => TimeGranularity::Weekly,
                        "monthly" => TimeGranularity::Monthly,
                        "quarterly" => TimeGranularity::Quarterly,
                        "yearly" => TimeGranularity::Yearly,
                        _ => TimeGranularity::Daily,
                    };
                    granularity.set(g);
                }
            >
                <option value="daily" selected=move || granularity.get() == TimeGranularity::Daily>
                    "Daily"
                </option>
                <option value="weekly" selected=move || granularity.get() == TimeGranularity::Weekly>
                    "Weekly"
                </option>
                <option value="monthly" selected=move || granularity.get() == TimeGranularity::Monthly>
                    "Monthly"
                </option>
                <option value="quarterly" selected=move || granularity.get() == TimeGranularity::Quarterly>
                    "Quarterly"
                </option>
                <option value="yearly" selected=move || granularity.get() == TimeGranularity::Yearly>
                    "Yearly"
                </option>
            </select>
        </div>
    }
}

/// Compact date range selector
#[component]
pub fn DateRangeSelector(
    date_range: RwSignal<DateRange>,
) -> impl IntoView {
    view! {
        <div class="flex gap-1">
            {DateRangePreset::all().iter().map(|preset| {
                let p = *preset;
                let is_active = move || date_range.get().preset == p;
                view! {
                    <button
                        class=move || {
                            if is_active() {
                                "px-3 py-1 text-xs font-medium bg-primary-500 text-white rounded-full"
                            } else {
                                "px-3 py-1 text-xs font-medium bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-400 rounded-full hover:bg-slate-200 dark:hover:bg-slate-600"
                            }
                        }
                        on:click=move |_| {
                            date_range.set(DateRange::from_preset(p));
                        }
                    >
                        {p.label()}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

/// Granularity selector as segmented control
#[component]
pub fn GranularitySelector(
    granularity: RwSignal<TimeGranularity>,
) -> impl IntoView {
    view! {
        <div class="flex bg-slate-100 dark:bg-slate-700 rounded-lg p-0.5">
            {TimeGranularity::all().iter().map(|g| {
                let granularity_val = *g;
                let is_active = move || granularity.get() == granularity_val;
                view! {
                    <button
                        class=move || {
                            if is_active() {
                                "px-3 py-1 text-xs font-medium bg-white dark:bg-slate-600 text-slate-900 dark:text-white rounded-md shadow-sm"
                            } else {
                                "px-3 py-1 text-xs font-medium text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white"
                            }
                        }
                        on:click=move |_| {
                            granularity.set(granularity_val);
                        }
                    >
                        {granularity_val.label()}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}
