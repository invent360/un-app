//! Reserve stage component
//!
//! Shows the atomic reservation in progress. This stage displays
//! a loading state while the license is being reserved exclusively
//! for the user.

use leptos::prelude::*;
use crate::hooks::t;
use crate::components::wizard::state::use_wizard_state;

/// Reserve stage - shows reservation in progress
#[component]
pub fn ReserveStage() -> impl IntoView {
    let state = use_wizard_state().expect("ReserveStage must be rendered within wizard context");

    // Check if there's an error
    let has_error = move || state.error.get().is_some();
    let error_message = move || state.error.get().unwrap_or_default();

    // Clone state for retry button
    let state_for_retry = state.clone();

    // Handle retry - go back to review stage
    let on_retry = move |_| {
        state_for_retry.error.set(None);
        state_for_retry.is_claiming.set(false);
        state_for_retry.stage.set(crate::components::wizard::state::ClaimWizardStage::Review);
    };

    view! {
        <div class="wizard-stage wizard-reserve">
            <Show
                when=move || !has_error()
                fallback=move || view! {
                    // Error state
                    <div class="reserve-error">
                        <div class="error-icon-container">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="error-icon">
                                <circle cx="12" cy="12" r="10"/>
                                <line x1="15" y1="9" x2="9" y2="15"/>
                                <line x1="9" y1="9" x2="15" y2="15"/>
                            </svg>
                        </div>
                        <h2 class="error-title">{move || t("wizard.reserve.error_title")}</h2>
                        <p class="error-message">{error_message}</p>
                        <button
                            class="btn btn-secondary retry-btn"
                            on:click=on_retry
                        >
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="btn-icon">
                                <polyline points="23 4 23 10 17 10"/>
                                <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>
                            </svg>
                            {move || t("wizard.reserve.try_again")}
                        </button>
                    </div>
                }
            >
                // Loading state - reservation in progress
                <div class="reserve-loading">
                    <div class="reserve-animation">
                        // Animated lock icon
                        <div class="lock-container">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="lock-icon animating">
                                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                                <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                            </svg>
                            <div class="lock-pulse"></div>
                        </div>

                        // Progress dots
                        <div class="progress-dots">
                            <span class="dot dot-1"></span>
                            <span class="dot dot-2"></span>
                            <span class="dot dot-3"></span>
                        </div>
                    </div>

                    <h2 class="reserve-title">{move || t("wizard.reserve.title")}</h2>
                    <p class="reserve-subtitle">{move || t("wizard.reserve.subtitle")}</p>

                    // What's happening explanation
                    <div class="reserve-steps">
                        <div class="step active">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="step-icon">
                                <circle cx="12" cy="12" r="10"/>
                                <polyline points="12 6 12 12 16 14"/>
                            </svg>
                            <span>{move || t("wizard.reserve.step_locking")}</span>
                        </div>
                        <div class="step pending">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="step-icon">
                                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                                <polyline points="22 4 12 14.01 9 11.01"/>
                            </svg>
                            <span>{move || t("wizard.reserve.step_verifying")}</span>
                        </div>
                        <div class="step pending">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="step-icon">
                                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                                <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                            </svg>
                            <span>{move || t("wizard.reserve.step_securing")}</span>
                        </div>
                    </div>

                    // Security notice
                    <div class="security-notice">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="security-icon">
                            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                        </svg>
                        <span>{move || t("wizard.reserve.security_notice")}</span>
                    </div>
                </div>
            </Show>
        </div>
    }
}
