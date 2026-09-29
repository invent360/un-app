//! Economics Review stage component
//!
//! Shows the 50/40/10 revenue split transparency before proceeding with claim.
//! This stage ensures users understand how earnings are distributed.

use leptos::prelude::*;
use crate::hooks::t;
use crate::components::wizard::state::use_wizard_state;

/// Split allocation percentages (matching uno-api RevenueSplit defaults)
const ULO_PERCENTAGE: u32 = 50;
const UNO_PERCENTAGE: u32 = 40;
const REFERRAL_PERCENTAGE: u32 = 10;

/// Economics Review stage - shows 50/40/10 split transparency
#[component]
pub fn EconomicsReviewStage() -> impl IntoView {
    let state = use_wizard_state().expect("EconomicsReviewStage must be rendered within wizard context");

    let state_for_proceed = state.clone();

    // Handle proceeding to next stage
    let on_proceed = move |_| {
        state_for_proceed.accept_economics();
    };

    view! {
        <div class="wizard-stage wizard-economics-review">
            <div class="stage-header">
                <h2 class="economics-title">{move || t("wizard.economics.title")}</h2>
                <p class="economics-subtitle">{move || t("wizard.economics.subtitle")}</p>
            </div>

            // Revenue split visualization
            <div class="split-visualization">
                <h3 class="split-title">{move || t("wizard.economics.split_title")}</h3>

                // Split bar visualization
                <div class="split-bar">
                    <div class="split-segment ulo" style=format!("width: {}%", ULO_PERCENTAGE)>
                        <span class="segment-label">{ULO_PERCENTAGE}"%"</span>
                    </div>
                    <div class="split-segment uno" style=format!("width: {}%", UNO_PERCENTAGE)>
                        <span class="segment-label">{UNO_PERCENTAGE}"%"</span>
                    </div>
                    <div class="split-segment referral" style=format!("width: {}%", REFERRAL_PERCENTAGE)>
                        <span class="segment-label">{REFERRAL_PERCENTAGE}"%"</span>
                    </div>
                </div>

                // Split breakdown cards
                <div class="split-breakdown">
                    // ULO (You) - 50%
                    <div class="split-card ulo-card">
                        <div class="split-card-header">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="split-icon">
                                <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                                <circle cx="12" cy="7" r="4"/>
                            </svg>
                            <span class="split-percentage">{ULO_PERCENTAGE}"%"</span>
                        </div>
                        <h4 class="split-card-title">{move || t("wizard.economics.you_title")}</h4>
                        <p class="split-card-desc">{move || t("wizard.economics.you_desc")}</p>
                    </div>

                    // UNO (Platform) - 40%
                    <div class="split-card uno-card">
                        <div class="split-card-header">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="split-icon">
                                <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
                                <line x1="8" y1="21" x2="16" y2="21"/>
                                <line x1="12" y1="17" x2="12" y2="21"/>
                            </svg>
                            <span class="split-percentage">{UNO_PERCENTAGE}"%"</span>
                        </div>
                        <h4 class="split-card-title">{move || t("wizard.economics.platform_title")}</h4>
                        <p class="split-card-desc">{move || t("wizard.economics.platform_desc")}</p>
                    </div>

                    // Referral - 10%
                    <div class="split-card referral-card">
                        <div class="split-card-header">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="split-icon">
                                <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                                <circle cx="8.5" cy="7" r="4"/>
                                <line x1="20" y1="8" x2="20" y2="14"/>
                                <line x1="23" y1="11" x2="17" y2="11"/>
                            </svg>
                            <span class="split-percentage">{REFERRAL_PERCENTAGE}"%"</span>
                        </div>
                        <h4 class="split-card-title">{move || t("wizard.economics.referrer_title")}</h4>
                        <p class="split-card-desc">{move || t("wizard.economics.referrer_desc")}</p>
                    </div>
                </div>
            </div>

            // Important disclosures
            <div class="economics-disclosures">
                <h4 class="disclosures-title">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="info-icon">
                        <circle cx="12" cy="12" r="10"/>
                        <line x1="12" y1="16" x2="12" y2="12"/>
                        <line x1="12" y1="8" x2="12.01" y2="8"/>
                    </svg>
                    {move || t("wizard.economics.important_title")}
                </h4>
                <ul class="disclosures-list">
                    <li>{move || t("wizard.economics.disclosure_small_rewards")}</li>
                    <li>{move || t("wizard.economics.disclosure_credit_cost")}</li>
                    <li>{move || t("wizard.economics.disclosure_device_requirement")}</li>
                    <li>{move || t("wizard.economics.disclosure_uptime")}</li>
                </ul>
            </div>

            // Who should not join section
            <div class="who-should-not">
                <h4 class="who-should-not-title">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="warning-icon">
                        <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
                        <line x1="12" y1="9" x2="12" y2="13"/>
                        <line x1="12" y1="17" x2="12.01" y2="17"/>
                    </svg>
                    {move || t("wizard.economics.who_should_not_title")}
                </h4>
                <ul class="who-should-not-list">
                    <li>{move || t("wizard.economics.not_for_quick_money")}</li>
                    <li>{move || t("wizard.economics.not_for_unstable_internet")}</li>
                    <li>{move || t("wizard.economics.not_for_shared_devices")}</li>
                </ul>
            </div>

            // Proceed button
            <div class="stage-actions">
                <button
                    class="btn btn-primary btn-lg proceed-btn"
                    on:click=on_proceed
                >
                    <span>{move || t("wizard.economics.understand_proceed")}</span>
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="btn-icon">
                        <line x1="5" y1="12" x2="19" y2="12"/>
                        <polyline points="12 5 19 12 12 19"/>
                    </svg>
                </button>
            </div>
        </div>
    }
}

/// Split calculator component for showing earnings breakdown
#[component]
pub fn SplitCalculator(
    /// Monthly pool amount in dollars
    #[prop(default = 10.0)]
    pool_amount: f64,
) -> impl IntoView {
    let ulo_amount = pool_amount * (ULO_PERCENTAGE as f64 / 100.0);
    let uno_amount = pool_amount * (UNO_PERCENTAGE as f64 / 100.0);
    let referral_amount = pool_amount * (REFERRAL_PERCENTAGE as f64 / 100.0);

    view! {
        <div class="split-calculator">
            <div class="calculator-header">
                <span class="calculator-label">{move || t("wizard.economics.example_pool")}</span>
                <span class="calculator-amount">"$"{format!("{:.2}", pool_amount)}</span>
            </div>
            <div class="calculator-breakdown">
                <div class="calc-row ulo">
                    <span class="calc-label">{move || t("wizard.economics.you_title")}</span>
                    <span class="calc-amount">"$"{format!("{:.2}", ulo_amount)}</span>
                </div>
                <div class="calc-row uno">
                    <span class="calc-label">{move || t("wizard.economics.platform_title")}</span>
                    <span class="calc-amount">"$"{format!("{:.2}", uno_amount)}</span>
                </div>
                <div class="calc-row referral">
                    <span class="calc-label">{move || t("wizard.economics.referrer_title")}</span>
                    <span class="calc-amount">"$"{format!("{:.2}", referral_amount)}</span>
                </div>
            </div>
        </div>
    }
}
