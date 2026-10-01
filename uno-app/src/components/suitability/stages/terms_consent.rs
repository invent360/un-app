//! Terms and consent stage for R5-13

use leptos::prelude::*;
use crate::components::suitability::state::SuitabilityState;

/// Terms acceptance and consent stage
#[component]
pub fn TermsConsentStage(state: SuitabilityState) -> impl IntoView {
    let answers = state.answers;

    // Clone state for closures
    let state_for_terms = state.clone();
    let state_for_marketing = state.clone();
    let state_for_back = state.clone();
    let state_for_next = state.clone();
    let state_for_proceed = state.clone();

    let terms_accepted = move || answers.get().terms_accepted;
    let marketing_consent = move || answers.get().marketing_consent;
    let can_proceed = move || state_for_proceed.can_proceed();

    view! {
        <div class="suitability-stage terms-consent-stage">
            <h3>"Terms of Service"</h3>
            <p class="stage-description">
                "Please review and accept our terms to continue."
            </p>

            // Terms summary
            <div class="terms-summary">
                <h4>"Key Points"</h4>
                <ul class="terms-list">
                    <li>
                        <strong>"Bandwidth Sharing:"</strong>
                        " You agree to share your unused internet bandwidth for network tasks"
                    </li>
                    <li>
                        <strong>"Device Requirements:"</strong>
                        " The UNO app must remain installed and active on your device"
                    </li>
                    <li>
                        <strong>"Earnings:"</strong>
                        " You will receive 50% of revenue from tasks completed through your device"
                    </li>
                    <li>
                        <strong>"Payouts:"</strong>
                        " Minimum payout threshold applies; earnings are paid monthly"
                    </li>
                    <li>
                        <strong>"Compliance:"</strong>
                        " All usage must comply with local laws and our acceptable use policy"
                    </li>
                    <li>
                        <strong>"License:"</strong>
                        " Licenses are non-transferable and bound to your account"
                    </li>
                </ul>
            </div>

            // Terms acceptance checkbox
            <div class="consent-group">
                <label class="checkbox-label required">
                    <input
                        type="checkbox"
                        checked=terms_accepted
                        on:change=move |ev| {
                            state_for_terms.set_terms_accepted(event_target_checked(&ev));
                        }
                    />
                    <span class="checkbox-text">
                        "I have read and agree to the "
                        <a href="/terms" target="_blank" class="link">"Terms of Service"</a>
                        " and "
                        <a href="/privacy" target="_blank" class="link">"Privacy Policy"</a>
                        " *"
                    </span>
                </label>
            </div>

            // Marketing consent checkbox (optional)
            <div class="consent-group optional">
                <label class="checkbox-label">
                    <input
                        type="checkbox"
                        checked=marketing_consent
                        on:change=move |ev| {
                            state_for_marketing.set_marketing_consent(event_target_checked(&ev));
                        }
                    />
                    <span class="checkbox-text">
                        "I'd like to receive updates about new features and promotions (optional)"
                    </span>
                </label>
            </div>

            // Data usage notice
            <div class="data-notice">
                <span class="info-icon">"i"</span>
                <p>
                    "We collect only the data necessary to operate the service. "
                    "Your personal information is never sold to third parties. "
                    <a href="/privacy" target="_blank" class="link">"Learn more"</a>
                </p>
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
                    disabled=move || !can_proceed()
                    on:click=move |_| state_for_next.next_stage()
                >
                    "Complete Setup"
                </button>
            </div>
        </div>
    }
}
