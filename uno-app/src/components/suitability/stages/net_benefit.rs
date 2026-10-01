//! Net benefit stage for R5-13
//!
//! Shows realistic earnings potential based on user's situation.
//! Actual earnings vary by region and task availability.

use leptos::prelude::*;
use crate::components::suitability::state::SuitabilityState;
use crate::hooks::t;

/// Determine earnings potential level based on uptime potential
fn get_earnings_potential_level(min: f64, max: f64) -> (&'static str, &'static str) {
    let avg = (min + max) / 2.0;
    if avg >= 2.0 {
        ("High", "suitability.earnings_high_desc")
    } else if avg >= 1.0 {
        ("Medium", "suitability.earnings_medium_desc")
    } else {
        ("Low", "suitability.earnings_low_desc")
    }
}

/// Net benefit/earnings estimate stage
#[component]
pub fn NetBenefitStage(state: SuitabilityState) -> impl IntoView {
    let answers = state.answers;
    let state_for_back = state.clone();
    let state_for_next = state.clone();

    // Calculate earnings potential based on answers
    let earnings_potential = move || answers.get().estimated_earnings();

    view! {
        <div class="suitability-stage net-benefit-stage">
            <h3>{t("suitability.your_earnings_potential")}</h3>
            <p class="stage-description">
                {t("suitability.earnings_potential_intro")}
            </p>

            // Earnings potential card
            <div class="earnings-card">
                {move || {
                    if let Some((min, max)) = earnings_potential() {
                        let (level, desc_key) = get_earnings_potential_level(min, max);
                        view! {
                            <div class="earnings-estimate">
                                <span class="earnings-label">{t("suitability.earnings_potential_label")}</span>
                                <span class="earnings-amount earnings-level">
                                    {level}
                                </span>
                                <span class="earnings-note">
                                    {t(desc_key)}
                                </span>
                                <div class="earnings-variability">
                                    <span class="info-icon">"ℹ"</span>
                                    <span>{t("suitability.earnings_vary_message")}</span>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="earnings-estimate variable">
                                <span class="earnings-label">{t("suitability.earnings_potential_label")}</span>
                                <span class="earnings-note variable-message">
                                    {t("suitability.earnings_vary_region_task")}
                                </span>
                                <p class="variable-detail">
                                    {t("suitability.complete_steps_for_estimate")}
                                </p>
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // How it works breakdown
            <div class="earnings-breakdown">
                <h4>"How Earnings Work"</h4>
                <div class="breakdown-item">
                    <span class="breakdown-label">"Your Share"</span>
                    <span class="breakdown-value">"50%"</span>
                    <span class="breakdown-desc">"of all task revenue goes directly to you"</span>
                </div>
                <div class="breakdown-item">
                    <span class="breakdown-label">"Platform"</span>
                    <span class="breakdown-value">"40%"</span>
                    <span class="breakdown-desc">"covers infrastructure and operations"</span>
                </div>
                <div class="breakdown-item">
                    <span class="breakdown-label">"Referrer"</span>
                    <span class="breakdown-value">"10%"</span>
                    <span class="breakdown-desc">"rewards those who invite new participants"</span>
                </div>
            </div>

            // No hidden costs notice
            <div class="no-hidden-costs">
                <span class="check-icon">"✓"</span>
                <div class="cost-text">
                    <strong>"No hidden costs"</strong>
                    <p>"No deposit required. No phone purchase needed. Use your existing device."</p>
                </div>
            </div>

            // What affects earnings
            <div class="earnings-factors">
                <h4>"What Affects Your Earnings"</h4>
                <ul>
                    <li>"Device uptime - more hours online = more tasks"</li>
                    <li>"Connection quality - stable internet = reliable completion"</li>
                    <li>"Task availability - varies by location and demand"</li>
                    <li>"Network conditions - higher demand = better rates"</li>
                </ul>
            </div>

            // Navigation buttons
            <div class="stage-actions">
                <button
                    class="btn-secondary"
                    on:click=move |_| state_for_back.prev_stage()
                >
                    "Back"
                </button>
                <button
                    class="btn-primary"
                    on:click=move |_| state_for_next.next_stage()
                >
                    "Continue"
                </button>
            </div>
        </div>
    }
}
