use leptos::prelude::*;
use crate::components::common::{Avatar, AvatarSize};
use crate::state::ProductStat;

/// Single product item in the analytics list
#[component]
pub fn ProductItemRow(
    product: ProductStat,
) -> impl IntoView {
    let change_class = if product.change_percent >= 0.0 {
        "text-green-500"
    } else {
        "text-red-500"
    };

    let change_sign = if product.change_percent >= 0.0 { "+" } else { "" };

    view! {
        <div class="flex items-center gap-3 py-3 border-b border-slate-100 dark:border-slate-700 last:border-b-0">
            // Product image/avatar
            <Avatar
                name=product.name.clone()
                src=product.image_url.clone().unwrap_or_default()
                size=AvatarSize::Medium
            />

            // Product info
            <div class="flex-1 min-w-0">
                <div class="font-medium text-slate-900 dark:text-white truncate">
                    {product.name}
                </div>
                <div class="text-sm text-slate-500 dark:text-slate-400">
                    {product.category}
                </div>
            </div>

            // Sales count and change
            <div class="flex flex-col items-end">
                <span class="font-semibold text-slate-900 dark:text-white">
                    {product.sales_count}
                </span>
                <span class=format!("text-sm {}", change_class)>
                    {format!("Sales {}{:.0}%", change_sign, product.change_percent)}
                </span>
            </div>
        </div>
    }
}
