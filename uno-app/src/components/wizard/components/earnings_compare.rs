//! Earnings comparison component

use leptos::prelude::*;
use crate::hooks::t;
use crate::types::LicenseVariant;

/// Earnings comparison table
#[component]
pub fn EarningsCompare(
    variants: Vec<LicenseVariant>,
) -> impl IntoView {
    // Sort by user share percentage (highest first)
    let sorted_variants = {
        let mut v = variants.clone();
        v.sort_by(|a, b| b.user_share_percentage.cmp(&a.user_share_percentage));
        v
    };

    view! {
        <div class="earnings-compare">
            <h3 class="compare-title">{move || t("wizard.compare.title")}</h3>

            <table class="compare-table">
                <thead>
                    <tr>
                        <th>{move || t("wizard.compare.split_header")}</th>
                        <th>{move || t("wizard.compare.your_share_header")}</th>
                        <th>{move || t("wizard.compare.min_earnings_header")}</th>
                        <th>{move || t("wizard.compare.max_earnings_header")}</th>
                    </tr>
                </thead>
                <tbody>
                    {sorted_variants.iter().map(|variant| {
                        let split = variant.split_display();
                        let user_share = format!("{}%", variant.user_share_percentage);
                        let min_earnings = variant.min_monthly_earnings
                            .map(|e| format!("${:.0}", e))
                            .unwrap_or_else(|| "-".to_string());
                        let max_earnings = variant.max_monthly_earnings
                            .map(|e| format!("${:.0}", e))
                            .unwrap_or_else(|| "-".to_string());
                        let is_featured = variant.is_featured;

                        view! {
                            <tr class:featured=is_featured>
                                <td class="split-cell">{split}</td>
                                <td class="share-cell">{user_share}</td>
                                <td class="earnings-cell">{min_earnings}</td>
                                <td class="earnings-cell">{max_earnings}</td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>

            <p class="compare-note">{move || t("wizard.compare.note")}</p>
        </div>
    }
}
