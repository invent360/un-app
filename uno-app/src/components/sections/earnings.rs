//! CMS-driven Earnings section component

use leptos::prelude::*;
use crate::api::HomeSection;

/// Earnings tier data from CMS
#[derive(Debug, Clone)]
struct TierData {
    name: String,
    min_earnings: f64,
    max_earnings: f64,
    period: String,
    features: Vec<String>,
    is_popular: bool,
}

/// CMS-driven Earnings section
#[component]
pub fn CmsEarningsSection(
    /// Section data from CMS
    section: HomeSection,
) -> impl IntoView {
    let title = section.title.clone();
    let subtitle = section.description.clone();

    // Helper to get nested data (handles both flat and nested "data.data" structure)
    let nested_data = section.data.get("data");

    // Extract tiers from section.data (check both nested and flat)
    let tiers: Vec<TierData> = nested_data
        .and_then(|d| d.get("tiers"))
        .or_else(|| section.data.get("tiers"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|t| {
                    Some(TierData {
                        name: t.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        min_earnings: t.get("min_earnings").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        max_earnings: t.get("max_earnings").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        period: t.get("period").and_then(|v| v.as_str()).unwrap_or("month").to_string(),
                        features: t.get("features")
                            .and_then(|v| v.as_array())
                            .map(|arr| arr.iter().filter_map(|f| f.as_str().map(String::from)).collect())
                            .unwrap_or_default(),
                        is_popular: t.get("is_popular").and_then(|v| v.as_bool()).unwrap_or(false),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    // Extract disclaimer (check both nested and flat)
    let disclaimer = nested_data
        .and_then(|d| d.get("disclaimer"))
        .or_else(|| section.data.get("disclaimer"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    view! {
        <section id="earnings" class="section earnings">
            <div class="container">
                <h2 class="section-title">{title}</h2>
                <p class="section-subtitle">{subtitle}</p>

                <div class="earnings-grid">
                    {tiers.into_iter().map(|tier| {
                        let card_class = if tier.is_popular {
                            "earnings-card featured"
                        } else {
                            "earnings-card"
                        };

                        // Format earnings range
                        let earnings_text = format!(
                            "${}-{}",
                            tier.min_earnings as i32,
                            tier.max_earnings as i32
                        );

                        let period_text = format!("/{}", tier.period);

                        view! {
                            <div class={card_class}>
                                {tier.is_popular.then(|| view! {
                                    <div class="earnings-card-badge">"Most Popular"</div>
                                })}
                                <div class="earnings-card-header">
                                    <span class="device-count">{tier.name}</span>
                                </div>
                                <div class="earnings-card-amount">{earnings_text}</div>
                                <div class="earnings-card-period">{period_text}</div>
                                <ul class="earnings-card-features">
                                    {tier.features.into_iter().map(|feature| {
                                        view! {
                                            <li>
                                                <CheckIcon/>
                                                {feature}
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            </div>
                        }
                    }).collect_view()}
                </div>

                {(!disclaimer.is_empty()).then(|| view! {
                    <p class="earnings-disclaimer">
                        {disclaimer}
                    </p>
                })}
            </div>
        </section>
    }
}

/// Check icon component
#[component]
fn CheckIcon() -> impl IntoView {
    view! {
        <svg class="icon icon-check" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
            <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
    }
}
