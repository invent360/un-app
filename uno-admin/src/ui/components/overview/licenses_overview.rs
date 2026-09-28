//! Licenses overview section for dashboard

use leptos::prelude::*;
use crate::state::{
    DateRange, TimeGranularity, LicensesOverviewData,
    ChartDataPoint, GroupedEarnings, EarningsDistribution
};
use crate::components::common::UsdCoinIcon;
use crate::components::charts::{BarChart, PieChart, PieSlice, DailyEarningsChart};
use crate::handler::{UnoLicensesSummaryDto, AllocationsSummaryDto, DailyIncentive};
use super::filters::OverviewFilters;
use super::top_performers::TopPerformers;

/// Licenses overview section with stats, filters, charts, and top performers
#[component]
pub fn LicensesOverviewSection(
    data: Signal<Option<LicensesOverviewData>>,
    date_range: RwSignal<DateRange>,
    granularity: RwSignal<TimeGranularity>,
    loading: Signal<bool>,
    /// Optional UNO licenses summary from API
    #[prop(optional)]
    licenses_summary: Option<Signal<Option<Result<UnoLicensesSummaryDto, String>>>>,
    /// Optional allocations summary from API
    #[prop(optional)]
    allocations_summary: Option<Signal<Option<Result<AllocationsSummaryDto, String>>>>,
    /// Daily incentive chart data (last 30 days)
    #[prop(optional)]
    daily_incentives: Option<Signal<Option<Result<Vec<DailyIncentive>, String>>>>,
    /// Whether incentives are currently syncing
    #[prop(optional)]
    syncing_incentives: Option<Signal<bool>>,
) -> impl IntoView {
    view! {
        <section class="bg-white dark:bg-slate-800 rounded-2xl p-6 border border-slate-200 dark:border-slate-700 shadow-sm">
            // Header with title and filters
            <div class="flex flex-col gap-4 mb-6">
                <h2 class="text-xl font-bold text-slate-900 dark:text-white">"Licenses Overview"</h2>
                <div class="flex items-center gap-3">
                    <OverviewFilters
                        date_range=date_range
                        granularity=granularity
                        compact=true
                    />
                </div>
            </div>

            // Loading state
            <Show when=move || loading.get()>
                <div class="flex items-center justify-center py-12">
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-500"></div>
                </div>
            </Show>

            // Stats Cards Row - using API data when available
            <div class="space-y-6">
                // Stats Cards from API
                {move || {
                    // Get licenses summary
                    let lic_summary = licenses_summary.and_then(|s| s.get()).and_then(|r| r.ok());
                    let alloc_summary = allocations_summary.and_then(|s| s.get()).and_then(|r| r.ok());

                    // Use API data if available, otherwise show loading or fallback
                    match (lic_summary, alloc_summary) {
                        (Some(lic), Some(alloc)) => view! {
                            <div class="flex flex-row gap-4 w-full">
                                <div class="flex-1">
                                    <StatCard
                                        label="Total Licenses"
                                        value=lic.total_licenses_count.to_string()
                                        variant="default"
                                    />
                                </div>
                                <div class="flex-1">
                                    <StatCard
                                        label="Active"
                                        value=lic.active_licenses_count.to_string()
                                        variant="success"
                                    />
                                </div>
                                <div class="flex-1">
                                    <StatCard
                                        label="Leased"
                                        value=lic.leased_licenses_count.to_string()
                                        variant="default"
                                    />
                                </div>
                                <div class="flex-1">
                                    <StatCard
                                        label="Bound"
                                        value=lic.bound_licenses_count.to_string()
                                        variant="default"
                                    />
                                </div>
                                <div class="flex-1">
                                    <EarningsCard
                                        label="Total Earnings"
                                        amount=alloc.total_amount()
                                    />
                                </div>
                            </div>
                        }.into_any(),
                        _ if loading.get() => view! {
                            <div class="flex flex-row gap-4 w-full">
                                {(0..5).map(|_| view! {
                                    <div class="flex-1 bg-slate-50 dark:bg-slate-700/50 rounded-xl p-4 h-16 animate-pulse"></div>
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any(),
                        _ => view! {
                            <div class="flex flex-row gap-4 w-full">
                                <div class="flex-1">
                                    <StatCard label="Total Licenses" value="--" variant="default" />
                                </div>
                                <div class="flex-1">
                                    <StatCard label="Active" value="--" variant="success" />
                                </div>
                                <div class="flex-1">
                                    <StatCard label="Leased" value="--" variant="default" />
                                </div>
                                <div class="flex-1">
                                    <StatCard label="Bound" value="--" variant="default" />
                                </div>
                                <div class="flex-1">
                                    <EarningsCard label="Total Earnings" amount=0.0 />
                                </div>
                            </div>
                        }.into_any()
                    }
                }}

                // Content when data is available (charts and top performers)
                <Show when=move || !loading.get() && data.get().is_some()>
                    {move || data.get().map(|d| {
                        let top_performers = d.top_performers.clone();
                        let earnings_distribution = d.earnings_distribution.clone();
                        let earnings_over_time = d.earnings_over_time.clone();

                        // Get daily incentives data
                        let daily_data = daily_incentives
                            .and_then(|s| s.get())
                            .and_then(|r| r.ok())
                            .unwrap_or_default();

                        // Tab state for charts
                        let active_chart_tab = RwSignal::new(0usize); // 0 = Daily Earnings, 1 = Distribution

                        view! {
                            <>
                                // Top Performers
                                <TopPerformers
                                    performers=top_performers
                                    title="Top Performing Licenses".to_string()
                                />

                                // Tabbed Charts Section
                                <div class="bg-slate-800 rounded-xl overflow-hidden">
                                    // Tab Headers
                                    <div class="flex border-b border-slate-700">
                                        <button
                                            class=move || format!(
                                                "px-6 py-3 text-sm font-medium transition-colors {}",
                                                if active_chart_tab.get() == 0 {
                                                    "text-emerald-400 border-b-2 border-emerald-400 bg-slate-700/50"
                                                } else {
                                                    "text-slate-400 hover:text-slate-200"
                                                }
                                            )
                                            on:click=move |_| active_chart_tab.set(0)
                                        >
                                            "Daily Earnings"
                                        </button>
                                        <button
                                            class=move || format!(
                                                "px-6 py-3 text-sm font-medium transition-colors {}",
                                                if active_chart_tab.get() == 1 {
                                                    "text-emerald-400 border-b-2 border-emerald-400 bg-slate-700/50"
                                                } else {
                                                    "text-slate-400 hover:text-slate-200"
                                                }
                                            )
                                            on:click=move |_| active_chart_tab.set(1)
                                        >
                                            "Earnings Distribution"
                                        </button>
                                    </div>

                                    // Tab Content
                                    <div class="p-6">
                                        // Daily Earnings Tab
                                        <Show when=move || active_chart_tab.get() == 0>
                                            <div>
                                                <div class="flex items-center justify-between mb-4">
                                                    <h3 class="text-base font-semibold text-white">
                                                        "Daily Earnings (Last 30 Days)"
                                                    </h3>
                                                    {move || syncing_incentives.map(|s| {
                                                        view! {
                                                            <Show when=move || s.get()>
                                                                <div class="flex items-center gap-2 text-xs text-emerald-400">
                                                                    <span class="inline-block w-3 h-3 border-2 border-emerald-400 border-t-transparent rounded-full animate-spin"></span>
                                                                    <span>"Syncing..."</span>
                                                                </div>
                                                            </Show>
                                                        }
                                                    })}
                                                </div>
                                                <DailyEarningsBarOnly data=daily_data.clone() />
                                            </div>
                                        </Show>

                                        // Earnings Distribution Tab
                                        <Show when=move || active_chart_tab.get() == 1>
                                            <div>
                                                <h3 class="text-base font-semibold text-white mb-4">
                                                    "Earnings Distribution by License"
                                                </h3>
                                                <div class="flex justify-center">
                                                    <EarningsPieChartDark data=earnings_distribution.clone() />
                                                </div>
                                            </div>
                                        </Show>
                                    </div>
                                </div>

                                // Bar Chart - Full Width
                                <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl p-4">
                                    <h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-4">
                                        "Earnings Over Time"
                                    </h3>
                                    <div class="w-full overflow-x-auto">
                                        <EarningsBarChart data=earnings_over_time />
                                    </div>
                                </div>
                            </>
                        }
                    })}
                </Show>
            </div>

        </section>
    }
}

/// Stat card with centered content
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
        <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl p-4 text-center w-full h-full">
            <div class="text-xs text-slate-500 dark:text-slate-400 mb-1">{label}</div>
            <div class={format!("text-xl font-bold {}", value_class)}>{value}</div>
        </div>
    }
}

/// Earnings card with USD coin icon and centered content
#[component]
fn EarningsCard(
    #[prop(into)] label: String,
    amount: f64,
) -> impl IntoView {
    view! {
        <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl p-4 text-center w-full h-full">
            <div class="text-xs text-slate-500 dark:text-slate-400 mb-1">{label}</div>
            <div class="text-xl font-bold text-green-500 flex items-center justify-center gap-1">
                <UsdCoinIcon size=18 />
                {format!("{:.2}", amount)}
            </div>
        </div>
    }
}

/// Pie chart wrapper for earnings distribution
#[component]
fn EarningsPieChart(data: Vec<EarningsDistribution>) -> impl IntoView {
    // Color palette for pie slices
    let colors = [
        "#3B82F6", // blue
        "#10B981", // emerald
        "#F59E0B", // amber
        "#8B5CF6", // violet
        "#EC4899", // pink
        "#06B6D4", // cyan
        "#84CC16", // lime
        "#F97316", // orange
    ];

    let slices: Vec<PieSlice> = data
        .iter()
        .enumerate()
        .map(|(i, d)| {
            PieSlice::new(
                d.name.clone(),
                d.amount,
                colors[i % colors.len()].to_string(),
            )
        })
        .collect();

    if slices.is_empty() {
        return view! {
            <div class="flex items-center justify-center h-40 text-slate-400 text-sm">
                "No earnings data"
            </div>
        }.into_any();
    }

    view! {
        <div class="flex justify-center">
            <PieChart data=slices size=180.0 show_legend=true />
        </div>
    }.into_any()
}

/// Bar chart wrapper for earnings over time
#[component]
fn EarningsBarChart(data: Vec<GroupedEarnings>) -> impl IntoView {
    let chart_data: Vec<ChartDataPoint> = data
        .iter()
        .map(|d| ChartDataPoint::new(d.period.clone(), d.amount))
        .collect();

    if chart_data.is_empty() {
        return view! {
            <div class="flex items-center justify-center h-44 text-slate-400 text-sm">
                "No time series data"
            </div>
        }.into_any();
    }

    view! {
        <BarChart data=chart_data />
    }.into_any()
}

/// Format date from YYYY-MM-DD to DD/MM/YYYY
fn format_date_ddmmyyyy(date: &str) -> String {
    if date.contains('-') {
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{}", parts[2], parts[1], parts[0]);
        }
    }
    date.to_string()
}

/// Daily earnings bar chart (bar only, for use inside tabbed container)
/// With hover tooltips showing reward amount and license count
#[component]
fn DailyEarningsBarOnly(data: Vec<DailyIncentive>) -> impl IntoView {
    use super::super::charts::chart_utils::{ChartDimensions, get_value_range};

    let data_len = data.len();

    if data.is_empty() {
        return view! {
            <div class="flex flex-col items-center justify-center h-64 text-slate-400">
                <svg class="w-12 h-12 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                </svg>
                <p class="text-sm">"No earnings data"</p>
            </div>
        }.into_any();
    }

    let width = (data_len as f64 * 28.0).max(600.0).min(1400.0);

    let dims = ChartDimensions {
        width,
        height: 320.0,
        padding_left: 55.0,
        padding_right: 20.0,
        padding_top: 20.0,
        padding_bottom: 80.0,
    };

    let bar_gap = 3.0;

    // Convert DailyIncentive to ChartDataPoint for calculations
    let chart_points: Vec<ChartDataPoint> = data.iter()
        .map(|d| ChartDataPoint::new(d.date.clone(), d.amount))
        .collect();

    // Calculate bar positions
    let inner_width = dims.inner_width();
    let inner_height = dims.inner_height();
    let n = chart_points.len() as f64;
    let bar_width = (inner_width - (n - 1.0) * bar_gap) / n;

    let (_, max_val) = get_value_range(&chart_points);
    let y_max = if max_val > 0.0 { max_val * 1.1 } else { 0.10 };

    let label_y = dims.height - dims.padding_bottom + 18.0;
    let label_step = if data_len > 20 { 2 } else { 1 };

    let format_y_label = move |val: f64| -> String {
        if val >= 1.0 {
            format!("${:.0}", val)
        } else {
            format!("${:.2}", val)
        }
    };

    // Tooltip state
    let tooltip_visible = RwSignal::new(false);
    let tooltip_x = RwSignal::new(0.0f64);
    let tooltip_y = RwSignal::new(0.0f64);
    let tooltip_date = RwSignal::new(String::new());
    let tooltip_amount = RwSignal::new(0.0f64);
    let tooltip_licenses = RwSignal::new(0i32);

    view! {
        <div class="overflow-x-auto relative">
            <svg
                viewBox=format!("0 0 {} {}", dims.width, dims.height)
                class="w-full min-w-[600px]"
                style="height: 280px;"
                preserveAspectRatio="xMidYMid meet"
            >
                // Grid lines
                {(0..5).map(|i| {
                    let y = dims.padding_top + (i as f64 / 4.0) * inner_height;
                    view! {
                        <line
                            x1=dims.padding_left
                            y1=y
                            x2=dims.width - dims.padding_right
                            y2=y
                            stroke="#334155"
                            stroke-width="1"
                        />
                    }
                }).collect::<Vec<_>>()}

                // Y-axis labels
                {(0..5).map(|i| {
                    let y = dims.padding_top + (i as f64 / 4.0) * inner_height;
                    let val = ((4 - i) as f64 / 4.0) * y_max;
                    let label = format_y_label(val);
                    view! {
                        <text
                            x=dims.padding_left - 8.0
                            y=y + 4.0
                            fill="#94a3b8"
                            style="text-anchor: end; font-size: 12px; font-family: system-ui, sans-serif;"
                        >
                            {label}
                        </text>
                    }
                }).collect::<Vec<_>>()}

                // Bars with hover events
                {data.iter().enumerate().map(|(i, incentive)| {
                    let x = dims.padding_left + i as f64 * (bar_width + bar_gap);
                    let height_ratio = if y_max > 0.0 { incentive.amount / y_max } else { 0.0 };
                    let bar_height = height_ratio * inner_height;
                    let y = dims.padding_top + inner_height - bar_height;

                    let date = incentive.date.clone();
                    let amount = incentive.amount;
                    let licenses = incentive.license_count;
                    let bar_center_x = x + bar_width / 2.0;

                    view! {
                        <rect
                            x=x
                            y=y
                            width=bar_width
                            height=bar_height.max(1.0)
                            rx=2.0
                            fill="#10b981"
                            class="cursor-pointer transition-opacity hover:opacity-80"
                            on:mouseenter=move |_| {
                                tooltip_visible.set(true);
                                tooltip_x.set(bar_center_x);
                                tooltip_y.set(y - 10.0);
                                tooltip_date.set(format_date_ddmmyyyy(&date));
                                tooltip_amount.set(amount);
                                tooltip_licenses.set(licenses);
                            }
                            on:mouseleave=move |_| {
                                tooltip_visible.set(false);
                            }
                        />
                    }
                }).collect::<Vec<_>>()}

                // X-axis labels
                {data.iter().enumerate().filter_map(|(i, incentive)| {
                    if i % label_step == 0 {
                        let x = dims.padding_left + i as f64 * (bar_width + bar_gap) + bar_width / 2.0;
                        let formatted_label = format_date_ddmmyyyy(&incentive.date);
                        Some(view! {
                            <text
                                x=x
                                y=label_y
                                fill="#94a3b8"
                                transform=format!("rotate(-60, {}, {})", x, label_y)
                                style="text-anchor: end; font-size: 11px; font-family: system-ui, sans-serif;"
                            >
                                {formatted_label}
                            </text>
                        })
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()}

                // Tooltip (rendered inside SVG for proper positioning)
                <Show when=move || tooltip_visible.get()>
                    {move || {
                        let tx = tooltip_x.get();
                        let ty = tooltip_y.get();
                        let date_str = tooltip_date.get();
                        let amount_val = tooltip_amount.get();
                        let license_count = tooltip_licenses.get();

                        // Calculate tooltip dimensions
                        let tooltip_width = 140.0;
                        let tooltip_height = 60.0;

                        // Adjust x position to keep tooltip within bounds
                        let adjusted_x = if tx - tooltip_width / 2.0 < dims.padding_left {
                            dims.padding_left
                        } else if tx + tooltip_width / 2.0 > dims.width - dims.padding_right {
                            dims.width - dims.padding_right - tooltip_width
                        } else {
                            tx - tooltip_width / 2.0
                        };

                        // Adjust y to show above bar (or below if too high)
                        let adjusted_y = if ty - tooltip_height - 5.0 < 0.0 {
                            ty + 20.0
                        } else {
                            ty - tooltip_height - 5.0
                        };

                        view! {
                            <g>
                                // Tooltip background
                                <rect
                                    x=adjusted_x
                                    y=adjusted_y
                                    width=tooltip_width
                                    height=tooltip_height
                                    rx=6.0
                                    fill="#1e293b"
                                    stroke="#475569"
                                    stroke-width="1"
                                />
                                // Date
                                <text
                                    x=adjusted_x + 10.0
                                    y=adjusted_y + 18.0
                                    fill="#94a3b8"
                                    style="font-size: 11px; font-family: system-ui, sans-serif;"
                                >
                                    {date_str}
                                </text>
                                // Amount
                                <text
                                    x=adjusted_x + 10.0
                                    y=adjusted_y + 35.0
                                    fill="#10b981"
                                    style="font-size: 13px; font-weight: 600; font-family: system-ui, sans-serif;"
                                >
                                    {format!("${:.4}", amount_val)}
                                </text>
                                // License count
                                <text
                                    x=adjusted_x + 10.0
                                    y=adjusted_y + 52.0
                                    fill="#94a3b8"
                                    style="font-size: 11px; font-family: system-ui, sans-serif;"
                                >
                                    {format!("{} license{}", license_count, if license_count == 1 { "" } else { "s" })}
                                </text>
                            </g>
                        }
                    }}
                </Show>
            </svg>
        </div>
    }.into_any()
}

/// Pie chart wrapper for dark background (earnings distribution)
#[component]
fn EarningsPieChartDark(data: Vec<EarningsDistribution>) -> impl IntoView {
    let colors = [
        "#3B82F6", // blue
        "#10B981", // emerald
        "#F59E0B", // amber
        "#8B5CF6", // violet
        "#EC4899", // pink
        "#06B6D4", // cyan
        "#84CC16", // lime
        "#F97316", // orange
    ];

    let slices: Vec<PieSlice> = data
        .iter()
        .enumerate()
        .map(|(i, d)| {
            PieSlice::new(
                d.name.clone(),
                d.amount,
                colors[i % colors.len()].to_string(),
            )
        })
        .collect();

    if slices.is_empty() {
        return view! {
            <div class="flex items-center justify-center h-64 text-slate-400 text-sm">
                "No earnings data"
            </div>
        }.into_any();
    }

    view! {
        <div class="flex justify-center py-4">
            <PieChart data=slices size=200.0 show_legend=true />
        </div>
    }.into_any()
}
