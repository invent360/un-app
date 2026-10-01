//! Suitability stepper component for R5-13
//!
//! A modal wizard that guides users through eligibility checking
//! before they can claim a license.

use leptos::prelude::*;
use super::state::{SuitabilityState, SuitabilityStage};
use super::stages::{
    CountryDeviceStage,
    ConnectivityStage,
    NetBenefitStage,
    TermsConsentStage,
};

/// Main suitability stepper component
#[component]
pub fn SuitabilityStepper(state: SuitabilityState) -> impl IntoView {
    let is_open = state.is_open;

    view! {
        {move || is_open.get().then(|| {
            view! { <SuitabilityModal state=state.clone() /> }
        })}
    }
}

/// Modal content (separated to avoid borrow issues)
#[component]
fn SuitabilityModal(state: SuitabilityState) -> impl IntoView {
    let stage = state.stage;
    let progress = Signal::derive(move || stage.get().progress());

    // Create reactive signals for progress steps
    let is_step1_active = Signal::derive(move || stage.get() == SuitabilityStage::CountryDevice);
    let is_step1_completed = Signal::derive(move || stage.get().number() > 1);
    let is_step2_active = Signal::derive(move || stage.get() == SuitabilityStage::Connectivity);
    let is_step2_completed = Signal::derive(move || stage.get().number() > 2);
    let is_step3_active = Signal::derive(move || stage.get() == SuitabilityStage::NetBenefit);
    let is_step3_completed = Signal::derive(move || stage.get().number() > 3);
    let is_step4_active = Signal::derive(move || stage.get() == SuitabilityStage::TermsConsent);
    let is_step4_completed = Signal::derive(move || stage.get().number() > 4);

    let state_for_close = state.clone();
    let state_for_close2 = state.clone();

    view! {
        <div class="suitability-modal-overlay" on:click=move |_| state_for_close.close()>
            <div class="suitability-modal" on:click=move |e| e.stop_propagation()>
                // Header
                <div class="suitability-header">
                    <h2 class="suitability-title">"Check Your Eligibility"</h2>
                    <button
                        class="suitability-close"
                        on:click=move |_| state_for_close2.close()
                        aria-label="Close"
                    >
                        <span class="close-icon">"×"</span>
                    </button>
                </div>

                // Progress bar
                <div class="suitability-progress">
                    <div class="progress-track">
                        <div
                            class="progress-fill"
                            style=move || format!("width: {}%", progress.get())
                        ></div>
                    </div>
                    <div class="progress-steps">
                        <ProgressStep number=1 label="Location" is_active=is_step1_active is_completed=is_step1_completed />
                        <ProgressStep number=2 label="Connection" is_active=is_step2_active is_completed=is_step2_completed />
                        <ProgressStep number=3 label="Earnings" is_active=is_step3_active is_completed=is_step3_completed />
                        <ProgressStep number=4 label="Terms" is_active=is_step4_active is_completed=is_step4_completed />
                    </div>
                </div>

                // Stage content
                <div class="suitability-content">
                    <StageContent state=state.clone() />
                </div>
            </div>
        </div>
    }
}

/// Progress step indicator
#[component]
fn ProgressStep(
    number: u8,
    label: &'static str,
    is_active: Signal<bool>,
    is_completed: Signal<bool>,
) -> impl IntoView {
    let class = move || {
        let mut cls = "progress-step".to_string();
        if is_active.get() {
            cls.push_str(" active");
        }
        if is_completed.get() {
            cls.push_str(" completed");
        }
        cls
    };

    view! {
        <div class=class>
            <div class="step-number">
                {move || if is_completed.get() {
                    view! { <span class="check-icon">"✓"</span> }.into_any()
                } else {
                    view! { <span>{number}</span> }.into_any()
                }}
            </div>
            <span class="step-label">{label}</span>
        </div>
    }
}

/// Stage content switcher
#[component]
fn StageContent(state: SuitabilityState) -> impl IntoView {
    let stage = state.stage;

    view! {
        {move || {
            match stage.get() {
                SuitabilityStage::CountryDevice => {
                    view! { <CountryDeviceStage state=state.clone() /> }.into_any()
                }
                SuitabilityStage::Connectivity => {
                    view! { <ConnectivityStage state=state.clone() /> }.into_any()
                }
                SuitabilityStage::NetBenefit => {
                    view! { <NetBenefitStage state=state.clone() /> }.into_any()
                }
                SuitabilityStage::TermsConsent => {
                    view! { <TermsConsentStage state=state.clone() /> }.into_any()
                }
                SuitabilityStage::Complete => {
                    view! { <CompleteStage state=state.clone() /> }.into_any()
                }
            }
        }}
    }
}

/// Completion stage
#[component]
fn CompleteStage(state: SuitabilityState) -> impl IntoView {
    let answers = state.answers;
    let state_for_claim = state.clone();
    let state_for_close = state.clone();

    view! {
        <div class="suitability-stage complete-stage">
            <div class="complete-icon">"✓"</div>
            <h3>"You're Eligible!"</h3>
            <p class="complete-message">
                "You meet the requirements to participate in the UNO network."
            </p>

            {move || {
                if let Some((min, max)) = answers.get().estimated_earnings() {
                    view! {
                        <div class="earnings-summary">
                            <p class="earnings-label">"Estimated Monthly Earnings"</p>
                            <p class="earnings-range">
                                {format!("${:.2} - ${:.2}", min, max)}
                            </p>
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}

            <div class="complete-actions">
                <button
                    class="btn-primary"
                    on:click=move |_| {
                        state_for_claim.close();
                        // Open the claim wizard
                        if let Some(wizard) = crate::components::wizard::use_wizard_state() {
                            wizard.open();
                        }
                    }
                >
                    "Claim Your License"
                </button>
                <button
                    class="btn-secondary"
                    on:click=move |_| state_for_close.close()
                >
                    "Maybe Later"
                </button>
            </div>
        </div>
    }
}
