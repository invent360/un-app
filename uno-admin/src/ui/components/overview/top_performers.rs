//! Top performers display component with medal badges

use leptos::prelude::*;
use crate::state::TopPerformer;
use crate::components::common::{MedalBadge, MedalRank, UsdCoinIcon};

/// Top performers component showing top 3 with medals
#[component]
pub fn TopPerformers(
    performers: Vec<TopPerformer>,
    #[prop(into)] title: String,
) -> impl IntoView {
    if performers.is_empty() {
        return view! {
            <div class="text-sm text-slate-500 dark:text-slate-400 italic">
                "No data available"
            </div>
        }.into_any();
    }

    view! {
        <div class="bg-slate-50 dark:bg-slate-700/50 rounded-xl p-4">
            <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">{title}</h4>
            <div class="flex flex-wrap gap-3">
                {performers.into_iter().map(|performer| {
                    let rank = MedalRank::from_rank(performer.rank);
                    view! {
                        <div class="flex items-center gap-2 px-4 py-2 bg-white dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-600 shadow-sm">
                            {rank.map(|r| view! { <MedalBadge rank=r size=24 /> })}
                            <div class="flex flex-col">
                                <span class="text-sm font-medium text-slate-900 dark:text-white truncate max-w-[120px]" title=performer.name.clone()>
                                    {performer.name.clone()}
                                </span>
                                <span class="text-xs text-green-500 flex items-center gap-0.5">
                                    <UsdCoinIcon size=12 />
                                    {format!("{:.2}", performer.earnings)}
                                </span>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }.into_any()
}

/// Compact horizontal top performers bar
#[component]
pub fn TopPerformersBar(
    performers: Vec<TopPerformer>,
) -> impl IntoView {
    if performers.is_empty() {
        return view! {
            <div class="text-sm text-slate-500 dark:text-slate-400 italic text-center py-2">
                "No top performers yet"
            </div>
        }.into_any();
    }

    view! {
        <div class="flex items-center justify-center gap-6">
            {performers.into_iter().take(3).map(|performer| {
                let rank = MedalRank::from_rank(performer.rank);
                let (order_class, size) = match performer.rank {
                    1 => ("order-2", 32_u32), // Gold in center, larger
                    2 => ("order-1", 28_u32), // Silver on left
                    3 => ("order-3", 28_u32), // Bronze on right
                    _ => ("order-4", 24_u32),
                };

                view! {
                    <div class={format!("flex flex-col items-center gap-1 {}", order_class)}>
                        {rank.map(|r| view! { <MedalBadge rank=r size=size /> })}
                        <span class="text-xs font-medium text-slate-700 dark:text-slate-300 truncate max-w-[80px]" title=performer.name.clone()>
                            {performer.name.clone()}
                        </span>
                        <span class="text-xs text-green-500 flex items-center gap-0.5">
                            <UsdCoinIcon size=10 />
                            {format!("{:.2}", performer.earnings)}
                        </span>
                    </div>
                }
            }).collect_view()}
        </div>
    }.into_any()
}

/// Vertical list of top performers
#[component]
pub fn TopPerformersList(
    performers: Vec<TopPerformer>,
    #[prop(into, optional)] title: Option<String>,
) -> impl IntoView {
    view! {
        <div class="space-y-2">
            {title.map(|t| view! {
                <h4 class="text-sm font-medium text-slate-700 dark:text-slate-300 mb-3">{t}</h4>
            })}
            {performers.into_iter().map(|performer| {
                let rank = MedalRank::from_rank(performer.rank);
                view! {
                    <div class="flex items-center justify-between p-2 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
                        <div class="flex items-center gap-3">
                            {rank.map(|r| view! { <MedalBadge rank=r size=24 /> })}
                            <span class="text-sm font-medium text-slate-900 dark:text-white">
                                {performer.name.clone()}
                            </span>
                        </div>
                        <span class="text-sm font-semibold text-green-500 flex items-center gap-0.5">
                            <UsdCoinIcon size=14 />
                            {format!("{:.2}", performer.earnings)}
                        </span>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
