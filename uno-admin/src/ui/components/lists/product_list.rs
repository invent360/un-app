use leptos::prelude::*;
use crate::components::common::dropdown::PeriodDropdown;
use crate::state::{ProductStat, Period};
use super::product_item::ProductItemRow;

/// Product sales list section
#[component]
pub fn ProductList(
    products: RwSignal<Vec<ProductStat>>,
    period: RwSignal<Period>,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-slate-800 rounded-2xl mx-4 mt-4 overflow-hidden">
            // Header
            <div class="flex items-center justify-between px-4 pt-4 pb-2">
                <h3 class="text-base font-semibold text-slate-900 dark:text-white">
                    "Chat Orders"
                </h3>
                <PeriodDropdown value=period />
            </div>

            // Items
            <div class="px-4 pb-2">
                {move || {
                    products.get().into_iter().map(|product| {
                        view! { <ProductItemRow product=product /> }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
