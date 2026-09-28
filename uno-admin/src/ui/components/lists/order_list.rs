use leptos::prelude::*;
use crate::components::common::dropdown::PeriodDropdown;
use crate::state::{OrderItem, Period};
use super::order_item::OrderItemRow;

/// Orders list section with header and items
#[component]
pub fn OrderList(
    orders: RwSignal<Vec<OrderItem>>,
    period: RwSignal<Period>,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-slate-800 rounded-2xl mx-4 mt-4 overflow-hidden">
            // Header
            <div class="flex items-center justify-between px-4 pt-4 pb-2">
                <h3 class="text-base font-semibold text-slate-900 dark:text-white">
                    "Orders List"
                </h3>
                <PeriodDropdown value=period />
            </div>

            // Items
            <div class="px-4 pb-2">
                {move || {
                    orders.get().into_iter().map(|order| {
                        view! { <OrderItemRow order=order /> }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
