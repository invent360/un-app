//! Availability status components for the claim wizard
//!
//! These components are shown when licenses are not available for claiming:
//! - SoldOutStage: All licenses have been claimed
//! - ComingSoonStage: No licenses exist in the system yet

use leptos::prelude::*;
use crate::hooks::t;
use crate::components::wizard::state::use_wizard_state;

/// Sold out stage - shown when all licenses have been claimed
#[component]
pub fn SoldOutStage() -> impl IntoView {
    let state = use_wizard_state().expect("SoldOutStage must be rendered within wizard context");
    let state_for_close = state.clone();
    let state_for_stats = state.clone();

    let on_close = move |_| {
        state_for_close.close();
    };

    view! {
        <div class="wizard-stage wizard-availability sold-out">
            <div class="availability-content">
                <div class="availability-icon sold-out-icon">
                    // Sad face icon
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <circle cx="12" cy="12" r="10"/>
                        <circle cx="9" cy="9" r="1" fill="currentColor"/>
                        <circle cx="15" cy="9" r="1" fill="currentColor"/>
                        <path d="M8 16c1.5-2 6.5-2 8 0"/>
                    </svg>
                </div>

                <h2 class="availability-title">{move || t("wizard.availability.sold_out_title")}</h2>

                <p class="availability-message">
                    {move || t("wizard.availability.sold_out_message")}
                </p>

                <div class="availability-actions">
                    <button
                        type="button"
                        class="btn-secondary"
                        on:click=on_close
                    >
                        {move || t("common.close")}
                    </button>
                </div>

                <div class="availability-stats">
                    {move || {
                        let variants = state_for_stats.variants.get();
                        let total_claimed: i32 = variants.iter().map(|v| v.claimed_count).sum();
                        if total_claimed > 0 {
                            Some(view! {
                                <p class="stats-text">
                                    <span class="stats-number">{total_claimed}</span>
                                    " "
                                    {move || t("wizard.availability.licenses_claimed")}
                                </p>
                            })
                        } else {
                            None
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

/// Coming soon stage - shown when no licenses exist in the system
#[component]
pub fn ComingSoonStage() -> impl IntoView {
    let state = use_wizard_state().expect("ComingSoonStage must be rendered within wizard context");
    let state_for_close = state.clone();

    let on_notify = move |_| {
        // Placeholder: log click for analytics and show feedback
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                let _ = window.alert_with_message("We'll notify you when license claiming goes live!");
            }
        }
        tracing::info!("User requested notification for license launch");
    };

    let on_close = move |_| {
        state_for_close.close();
    };

    view! {
        <div class="wizard-stage wizard-availability coming-soon">
            <div class="availability-content">
                <div class="availability-icon coming-soon-icon">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <circle cx="12" cy="12" r="10"/>
                        <polyline points="12 6 12 12 16 14"/>
                    </svg>
                </div>

                <h2 class="availability-title">{move || t("wizard.availability.coming_soon_title")}</h2>

                <p class="availability-message">
                    {move || t("wizard.availability.coming_soon_message")}
                </p>

                <div class="availability-actions">
                    <button
                        type="button"
                        class="btn-primary notify-btn"
                        on:click=on_notify
                    >
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="btn-icon">
                            <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
                            <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
                        </svg>
                        {move || t("wizard.availability.notify_live")}
                    </button>

                    <button
                        type="button"
                        class="btn-secondary"
                        on:click=on_close
                    >
                        {move || t("common.close")}
                    </button>
                </div>

                <a href="/licenses" class="learn-more-link">
                    {move || t("wizard.availability.learn_more")}
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="link-icon">
                        <line x1="5" y1="12" x2="19" y2="12"/>
                        <polyline points="12 5 19 12 12 19"/>
                    </svg>
                </a>
            </div>
        </div>
    }
}
