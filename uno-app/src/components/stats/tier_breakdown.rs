//! Tier distribution breakdown component

use leptos::prelude::*;

/// Tier data
#[derive(Debug, Clone)]
pub struct TierData {
    pub tier_name: String,
    pub count: u32,
    pub percentage: f64,
    pub color: String,
}

/// Tier breakdown chart component
#[component]
pub fn TierBreakdown(
    tiers: Vec<TierData>,
    #[prop(optional)] title: Option<String>,
) -> impl IntoView {
    let total: u32 = tiers.iter().map(|t| t.count).sum();

    view! {
        <div class="tier-breakdown">
            {title.map(|t| view! { <h3 class="chart-title">{t}</h3> })}

            // Stacked bar representation
            <div class="tier-stacked-bar">
                {tiers.iter().map(|tier| {
                    view! {
                        <div
                            class="tier-segment"
                            style=format!(
                                "width: {}%; background-color: {}",
                                tier.percentage,
                                tier.color
                            )
                            title=format!("{}: {} ({}%)", tier.tier_name, tier.count, tier.percentage as i32)
                        />
                    }
                }).collect_view()}
            </div>

            // Legend
            <div class="tier-legend">
                {tiers.into_iter().map(|tier| {
                    view! {
                        <div class="tier-legend-item">
                            <span
                                class="tier-color-box"
                                style=format!("background-color: {}", tier.color)
                            />
                            <span class="tier-label">{tier.tier_name.clone()}</span>
                            <span class="tier-count">{tier.count}</span>
                            <span class="tier-percentage">
                                {format!("({}%)", tier.percentage as i32)}
                            </span>
                        </div>
                    }
                }).collect_view()}
            </div>

            <div class="tier-total">
                <span class="total-label">"Total:"</span>
                <span class="total-value">{total}</span>
            </div>
        </div>
    }
}
