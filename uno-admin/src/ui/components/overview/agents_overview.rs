//! Agents overview section for dashboard

use leptos::prelude::*;
use crate::state::{
    DateRange, TimeGranularity, AgentsOverviewData,
    ChartDataPoint, GroupedEarnings, EarningsDistribution
};
use crate::components::common::UsdCoinIcon;
use crate::components::charts::{BarChart, PieChart, PieSlice};
use super::filters::OverviewFilters;
use super::top_performers::TopPerformers;

/// Agents overview section with stats, filters, charts, and top performers
#[component]
pub fn AgentsOverviewSection(
    data: Signal<Option<AgentsOverviewData>>,
    date_range: RwSignal<DateRange>,
    granularity: RwSignal<TimeGranularity>,
    loading: Signal<bool>,
) -> impl IntoView {
    view! {
        <section class="bg-white dark:bg-slate-800 rounded-2xl p-6 border border-slate-200 dark:border-slate-700 shadow-sm">
            // Header with title and filters
            <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
                <h2 class="text-xl font-bold text-slate-900 dark:text-white">"Agent Overview"</h2>
                <OverviewFilters
                    date_range=date_range
                    granularity=granularity
                    compact=true
                />
            </div>

            // Loading state
            <Show when=move || loading.get()>
                <div class="flex items-center justify-center py-12">
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-500"></div>
                </div>
            </Show>

            // Content when data is available
            <Show when=move || !loading.get() && data.get().is_some()>
                {move || data.get().map(|d| {
                    let top_performers = d.top_performers.clone();
                    let commissions_distribution = d.commissions_distribution.clone();
                    let commissions_over_time = d.commissions_over_time.clone();

                    view! {
                        <div class="space-y-6">
                            // Stats Cards Row - evenly spaced flex row
                            <div class="flex flex-row gap-4 w-full">
                                <div class="flex-1">
                                    <StatCard
                                        label="Total Agents"
                                        value=d.total_agents.to_string()
                                        variant="default"
                                    />
                                </div>
                                <div class="flex-1">
                                    <StatCard
                                        label="Active Agents"
                                        value=d.active_agents.to_string()
                                        variant="success"
                                    />
                                </div>
                                <div class="flex-1">
                                    <CommissionsCard
                                        label="Total Commissions"
                                        amount=d.total_commissions
                                    />
                                </div>
                                <div class="flex-1">
                                    <CommissionsCard
                                        label="Available"
                                        amount=d.available_commissions
                                    />
                                </div>
                            </div>

                            // Top Performers
                            <TopPerformers
                                performers=top_performers
                                title="Top Performing Agents".to_string()
                            />

                            // Pie Chart
                            <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl p-4">
                                <h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-4">
                                    "Commissions Distribution"
                                </h3>
                                <CommissionsPieChart data=commissions_distribution />
                            </div>

                            // Bar Chart - Full Width
                            <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl p-4">
                                <h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-4">
                                    "Commissions Over Time"
                                </h3>
                                <div class="w-full overflow-x-auto">
                                    <CommissionsBarChart data=commissions_over_time />
                                </div>
                            </div>
                        </div>
                    }
                })}
            </Show>

            // Empty state
            <Show when=move || !loading.get() && data.get().is_none()>
                <div class="flex flex-col items-center justify-center py-12 text-slate-500 dark:text-slate-400">
                    <svg class="w-12 h-12 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                    </svg>
                    <p class="text-sm">"No agent data available"</p>
                </div>
            </Show>
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

/// Commissions card with USD coin icon and centered content
#[component]
fn CommissionsCard(
    #[prop(into)] label: String,
    amount: f64,
) -> impl IntoView {
    // Ensure no negative zero display
    let display_amount = if amount.abs() < 0.01 { 0.0 } else { amount };
    view! {
        <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl p-4 text-center w-full h-full">
            <div class="text-xs text-slate-500 dark:text-slate-400 mb-1">{label}</div>
            <div class="text-xl font-bold text-green-500 flex items-center justify-center gap-1">
                <UsdCoinIcon size=18 />
                {format!("{:.2}", display_amount)}
            </div>
        </div>
    }
}

/// Pie chart wrapper for commissions distribution
#[component]
fn CommissionsPieChart(data: Vec<EarningsDistribution>) -> impl IntoView {
    // Color palette for pie slices (different from licenses for distinction)
    let colors = [
        "#10B981", // emerald
        "#3B82F6", // blue
        "#8B5CF6", // violet
        "#F59E0B", // amber
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
                "No commission data"
            </div>
        }.into_any();
    }

    view! {
        <div class="flex justify-center">
            <PieChart data=slices size=180.0 show_legend=true />
        </div>
    }.into_any()
}

/// Bar chart wrapper for commissions over time
#[component]
fn CommissionsBarChart(data: Vec<GroupedEarnings>) -> impl IntoView {
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
