use leptos::prelude::*;
use super::stat_card::{StatCard, format_currency};
use crate::state::{DashboardStats, StatVariant};

/// 2x2 grid of stat cards
#[component]
pub fn StatGrid(
    stats: RwSignal<DashboardStats>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 gap-4 px-4 py-4">
            {move || {
                let stats = stats.get();
                view! {
                    <StatCard
                        label="Total Menus"
                        value=stats.total_menus.to_string()
                        variant=StatVariant::Menus
                    />
                    <StatCard
                        label="Total Orders"
                        value=stats.total_orders.to_string()
                        variant=StatVariant::Orders
                    />
                    <StatCard
                        label="Total Clients"
                        value=stats.total_clients.to_string()
                        variant=StatVariant::Clients
                    />
                    <StatCard
                        label="Total Revenue"
                        value=format_currency(stats.total_revenue)
                        variant=StatVariant::Revenue
                    />
                }
            }}
        </div>
    }
}
