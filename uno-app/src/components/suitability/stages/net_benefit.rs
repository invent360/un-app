//! Net benefit stage for R5-13
//!
//! Shows realistic earnings estimate based on user's situation.

use leptos::prelude::*;
use crate::components::suitability::state::SuitabilityState;

/// Net benefit/earnings estimate stage
#[component]
pub fn NetBenefitStage(state: SuitabilityState) -> impl IntoView {
    let answers = state.answers;
    let state_for_back = state.clone();
    let state_for_next = state.clone();

    // Calculate earnings based on answers
    let earnings = move || answers.get().estimated_earnings();

    view! {
        <div class="suitability-stage net-benefit-stage">
            <h3>"Your Estimated Earnings"</h3>
            <p class="stage-description">
                "Based on your connection and device availability, here's what you can expect."
            </p>

            // Earnings estimate card
            <div class="earnings-card">
                {move || {
                    if let Some((min, max)) = earnings() {
                        view! {
                            <div class="earnings-estimate">
                                <span class="earnings-label">"Monthly Estimate"</span>
                                <span class="earnings-amount">
                                    {format!("${:.2} - ${:.2}", min, max)}
                                </span>
                                <span class="earnings-note">
                                    "Actual earnings depend on task availability in your area"
                                </span>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <p class="no-estimate">"Complete the previous steps to see your estimate."</p>
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
