use leptos::prelude::*;
use crate::components::common::{Avatar, AvatarSize, NewOrderBadge, UsdCoinIcon};
use crate::state::OrderItem as OrderItemData;

/// Single order item in the orders list
#[component]
pub fn OrderItemRow(
    order: OrderItemData,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-3 py-3 border-b border-slate-100 dark:border-slate-700 last:border-b-0">
            // Avatar
            <Avatar
                name=order.user_name.clone()
                src=order.user_avatar.clone().unwrap_or_default()
                size=AvatarSize::Medium
            />

            // User info
            <div class="flex-1 min-w-0">
                <div class="font-medium text-slate-900 dark:text-white truncate">
                    {order.user_name}
                </div>
                <div class="text-sm text-slate-500 dark:text-slate-400">
                    {order.date}
                </div>
            </div>

            // Amount and badge
            <div class="flex flex-col items-end gap-1">
                <span class="font-semibold text-green-500 inline-flex items-center gap-0.5">
                    "+ "<UsdCoinIcon size=14 />{format!("{:.2}", order.amount)}
                </span>
                {order.is_new.then(|| view! { <NewOrderBadge /> })}
            </div>
        </div>
    }
}
