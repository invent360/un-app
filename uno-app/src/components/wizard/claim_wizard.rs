//! Main wizard container component
//!
//! This component provides the modal overlay and container for the
//! license claim wizard, using distinct stage components for each step.
//!
//! ## Wizard Flow
//!
//! 1. EconomicsReview - Show 50/40/10 split transparency
//! 2. Review - Accept terms and optional referral code
//! 3. Reserve - Atomic license reservation (auto-proceeds)
//! 4. Claim - Display and confirm license key
//! 5. WhatNext - Guides and next steps

use leptos::prelude::*;
use crate::hooks::t;
use crate::types::AvailabilityStatus;
use super::state::{ClaimWizardStage, ClaimWizardState};
use super::stages::{
    EconomicsReviewStage, ReviewStage, ReserveStage,
    ClaimStage, WhatNextStage, SoldOutStage, ComingSoonStage
};
use super::components::WizardProgressBar;

/// License claiming wizard container
#[component]
pub fn ClaimWizard(
    /// Wizard state (provided by parent)
    state: ClaimWizardState,
) -> impl IntoView {
    view! {
        <Show
            when=move || state.is_open.get()
            fallback=|| ()
        >
            <WizardOverlay state=state.clone() />
        </Show>
    }
}

/// Internal modal overlay component
#[component]
fn WizardOverlay(
    state: ClaimWizardState,
) -> impl IntoView {
    let state_for_overlay = state.clone();
    let state_for_close = state.clone();

    // Handle overlay click (close) - allow anytime unless claiming is in progress
    let on_overlay_click = move |_| {
        let s = state_for_overlay.clone();
        if !s.is_claiming.get() {
            s.close();
        }
    };

    view! {
        <div class="wizard-overlay" on:click=on_overlay_click>
            <div class="wizard-container" on:click=|e| e.stop_propagation()>
                // Header with progress
                <div class="wizard-header">
                    <h3 class="wizard-title">{move || t("wizard.welcome.title")}</h3>

                    // Close button - always visible
                    {
                        let s = state_for_close.clone();
                        view! {
                            <button
                                type="button"
                                class="wizard-close"
                                on:click=move |_| s.close()
                                aria-label=move || t("common.close")
                            >
                                "×"
                            </button>
                        }
                    }
                </div>

                // Progress bar (shows all 5 stages)
                <WizardProgressBar stage=state.stage />

                // Stage content - each stage has its own dedicated component
                <div class="wizard-content">
                    <StageContent state=state.clone() />
                </div>
            </div>
        </div>
    }
}

/// Stage content - reactively switches between distinct wizard stage components
#[component]
fn StageContent(
    state: ClaimWizardState,
) -> impl IntoView {
    view! {
        {move || {
            // Check if variants are still loading
            if state.is_loading_variants.get() {
                return view! {
                    <div class="wizard-stage wizard-loading">
                        <div class="loading-spinner"></div>
                        <p>{move || t("common.loading")}</p>
                    </div>
                }.into_any();
            }

            // Check availability status on EconomicsReview or Review stage
            let current_stage = state.stage.get();
            if current_stage == ClaimWizardStage::EconomicsReview || current_stage == ClaimWizardStage::Review {
                match state.global_availability.get() {
                    AvailabilityStatus::AllClaimed => {
                        return view! { <SoldOutStage /> }.into_any();
                    }
                    AvailabilityStatus::NoneInSystem => {
                        return view! { <ComingSoonStage /> }.into_any();
                    }
                    AvailabilityStatus::AllExpired => {
                        // Treat expired same as sold out for now
                        return view! { <SoldOutStage /> }.into_any();
                    }
                    AvailabilityStatus::Available => {
                        // Continue to normal stage rendering
                    }
                }
            }

            // Normal stage rendering
            match current_stage {
                ClaimWizardStage::EconomicsReview => view! {
                    <EconomicsReviewStage />
                }.into_any(),
                ClaimWizardStage::Review => view! {
                    <ReviewStage />
                }.into_any(),
                ClaimWizardStage::Reserve => view! {
                    <ReserveStage />
                }.into_any(),
                ClaimWizardStage::Claim => view! {
                    <ClaimStage />
                }.into_any(),
                ClaimWizardStage::WhatNext => view! {
                    <WhatNextStage />
                }.into_any(),
            }
        }}
    }
}
