use leptos::prelude::*;
use crate::components::layout::Header;
use crate::components::charts::AreaChart;
use crate::components::lists::ProductList;
use crate::components::common::dropdown::PeriodDropdown;
use crate::state::use_dashboard;

/// Analytics page with sales stats and product list
#[component]
pub fn AnalyticsPage() -> impl IntoView {
    let dashboard = use_dashboard();

    view! {
        <div>
            <Header title="Analytics".to_string() />

            // Sales section with area chart
            <SalesSection />

            // Product list
            <ProductList
                products=dashboard.products_list
                period=dashboard.analytics_period
            />
        </div>
    }
}

/// Sales chart section
#[component]
fn SalesSection() -> impl IntoView {
    let dashboard = use_dashboard();

    view! {
        <div class="bg-white dark:bg-slate-800 rounded-2xl mx-4 mt-4 p-4">
            // Header
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-base font-semibold text-slate-900 dark:text-white">
                    "Chat Orders"
                </h3>
                <PeriodDropdown value=dashboard.analytics_period />
            </div>

            // Stats
            <div class="flex gap-8 mb-4">
                {move || {
                    let data = dashboard.sales_data.get();
                    view! {
                        <div>
                            <div class="text-2xl font-bold text-slate-900 dark:text-white">
                                {format_sales_count(data.total_sales)}
                            </div>
                            <div class="text-sm text-slate-500 dark:text-slate-400">
                                "Total Sales"
                            </div>
                        </div>
                        <div>
                            <div class="text-2xl font-bold text-slate-900 dark:text-white">
                                {format!("{}", data.average_sales)}
                            </div>
                            <div class="text-sm text-slate-500 dark:text-slate-400">
                                "Avg. Sales"
                            </div>
                        </div>
                    }
                }}
            </div>

            // Area Chart
            {move || {
                let data = dashboard.sales_data.get();
                view! { <AreaChart data=data.data_points /> }
            }}
        </div>
    }
}

/// Format sales count (e.g., 18000 -> "18K")
fn format_sales_count(count: u32) -> String {
    if count >= 1000 {
        format!("{}K", count / 1000)
    } else {
        count.to_string()
    }
}
