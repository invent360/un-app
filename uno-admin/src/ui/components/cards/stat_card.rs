use leptos::prelude::*;
use crate::components::common::icon::{Icon, IconName, UsdCoinIcon};
use crate::state::StatVariant;

/// Individual stat card component
#[component]
pub fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    variant: StatVariant,
) -> impl IntoView {
    let (icon_name, icon_class) = match variant {
        StatVariant::Menus => (IconName::Menu, "stat-icon-menus"),
        StatVariant::Orders => (IconName::Order, "stat-icon-orders"),
        StatVariant::Clients => (IconName::User, "stat-icon-clients"),
        StatVariant::Revenue => (IconName::Revenue, "stat-icon-revenue"),
    };

    let is_revenue = matches!(variant, StatVariant::Revenue);

    view! {
        <div class="bg-white dark:bg-slate-800 rounded-2xl p-4 shadow-sm dark:shadow-slate-900/20">
            // Icon
            <div class=format!(
                "w-10 h-10 rounded-xl flex items-center justify-center mb-3 {}",
                icon_class
            )>
                <Icon name=icon_name size=20 />
            </div>

            // Value
            <div class="text-2xl font-bold text-slate-900 dark:text-white mb-1 inline-flex items-center gap-1">
                {if is_revenue {
                    view! { <UsdCoinIcon size=20 /> }.into_any()
                } else {
                    view! {}.into_any()
                }}
                {value}
            </div>

            // Label
            <div class="text-sm text-slate-500 dark:text-slate-400">
                {label}
            </div>
        </div>
    }
}

/// Format currency value (without $ sign - icon is added by component)
pub fn format_currency(value: f64) -> String {
    if value >= 1000.0 {
        let formatted = format!("{:.0}", value);
        let len = formatted.len();
        formatted
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if i > 0 && (len - i) % 3 == 0 {
                    vec![',', c]
                } else {
                    vec![c]
                }
            })
            .collect()
    } else {
        format!("{:.2}", value)
    }
}
