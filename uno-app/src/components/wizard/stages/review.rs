//! Review stage component - simplified license claim flow

use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::hooks::t;
use crate::api::{reserve_next_license, validate_referral_code};
use crate::components::wizard::state::use_wizard_state;

/// Review stage - simplified claim flow with terms and optional referral
#[component]
pub fn ReviewStage() -> impl IntoView {
    let state = use_wizard_state().expect("ReviewStage must be rendered within wizard context");

    let state_for_terms = state.clone();
    let state_for_claim = state.clone();
    let state_for_referral_check = state.clone();

    // Handle terms checkbox
    let on_terms_change = move |_| {
        state_for_terms.terms_accepted.update(|v| *v = !*v);
    };

    // Handle referral checkbox
    let on_referral_check_change = move |_| {
        let s = state_for_referral_check.clone();
        s.has_referral.update(|v| *v = !*v);
        // Clear referral state when unchecking
        if !s.has_referral.get() {
            s.referral_code.set(String::new());
            s.referral_valid.set(None);
            s.referral_name.set(None);
        }
    };

    // Handle claim - reserves the next available license
    let on_claim = move |_| {
        let s = state_for_claim.clone();

        // Get referral code if provided and valid
        let referral_code = if s.has_referral.get() && s.referral_valid.get() == Some(true) {
            Some(s.referral_code.get())
        } else {
            None
        };

        // Transition to Reserve stage (shows loading animation)
        s.start_reservation();

        // Call the reserve API - gets the next available license
        spawn_local(async move {
            match reserve_next_license(referral_code.clone()).await {
                Ok(response) => {
                    if response.success {
                        // Transition from Reserve to Claim stage
                        s.set_reservation_success(
                            response.license_id,
                            response.license_key,
                            referral_code,
                        );
                    } else {
                        // Show error on Reserve stage (has retry button)
                        s.set_claim_error(response.error.unwrap_or_else(|| "No licenses available".to_string()));
                    }
                }
                Err(e) => {
                    s.set_claim_error(e.to_string());
                }
            }
        });
    };

    view! {
        <div class="wizard-stage wizard-review">
            <div class="stage-header">
                <h2 class="review-title">{move || t("wizard.review.title")}</h2>
            </div>

            // License info card - simplified
            <div class="license-details-card compact">
                // What you get section
                <div class="what-you-get">
                    <h4 class="what-you-get-title">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="what-you-get-icon">
                            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                            <polyline points="22 4 12 14.01 9 11.01"/>
                        </svg>
                        {move || t("wizard.review.what_you_get_title")}
                    </h4>
                    <ul class="what-you-get-list">
                        <li>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="20 6 9 17 4 12"/>
                            </svg>
                            {move || t("wizard.review.benefit_free_license")}
                        </li>
                        <li>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="20 6 9 17 4 12"/>
                            </svg>
                            {move || t("wizard.review.benefit_support")}
                        </li>
                        <li>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="20 6 9 17 4 12"/>
                            </svg>
                            {move || t("wizard.review.benefit_zero_fees")}
                        </li>
                    </ul>
                </div>

                // Uptime reward conditions
                <div class="uptime-conditions">
                    <h4 class="conditions-title">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="conditions-icon">
                            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                        </svg>
                        {move || t("wizard.review.uptime_conditions_title")}
                    </h4>
                    <ul class="conditions-list">
                        <li>{move || t("wizard.review.condition_1")}</li>
                        <li>{move || t("wizard.review.condition_2")}</li>
                    </ul>
                </div>
            </div>

            // Referral code section (optional)
            {
                let state_ref = state.clone();
                view! {
                    <div class="referral-section">
                        <label class="referral-checkbox-label">
                            <input
                                type="checkbox"
                                checked=move || state_ref.has_referral.get()
                                on:change=on_referral_check_change
                                disabled=move || state_ref.is_claiming.get()
                            />
                            <span class="checkbox-custom"></span>
                            <span class="referral-checkbox-text">
                                {move || t("wizard.review.referral_checkbox")}
                            </span>
                        </label>

                        // Referral code input - only show when checkbox is checked
                        {
                            let state_inner = state_ref.clone();
                            move || {
                                let s_input = state_inner.clone();
                                let s_blur = state_inner.clone();
                                let s_view = state_inner.clone();
                                if state_inner.has_referral.get() {
                                    view! {
                                        <div class="referral-input-wrapper">
                                            <input
                                                type="text"
                                                class="referral-input"
                                                placeholder=move || t("wizard.review.referral_placeholder")
                                                prop:value=move || s_view.referral_code.get()
                                                on:input=move |ev| {
                                                    let value = event_target_value(&ev);
                                                    s_input.referral_code.set(value);
                                                    s_input.referral_valid.set(None);
                                                    s_input.referral_name.set(None);
                                                }
                                                on:blur=move |_| {
                                                    let s = s_blur.clone();
                                                    let code = s.referral_code.get();
                                                    if code.trim().is_empty() {
                                                        s.referral_valid.set(None);
                                                        s.referral_name.set(None);
                                                        return;
                                                    }
                                                    s.referral_checking.set(true);
                                                    spawn_local(async move {
                                                        match validate_referral_code(code).await {
                                                            Ok(response) => {
                                                                s.referral_valid.set(Some(response.valid));
                                                                s.referral_name.set(response.referral_name);
                                                            }
                                                            Err(_) => {
                                                                s.referral_valid.set(Some(false));
                                                                s.referral_name.set(None);
                                                            }
                                                        }
                                                        s.referral_checking.set(false);
                                                    });
                                                }
                                                disabled=move || s_view.is_claiming.get()
                                            />
                                            // Validation status icon
                                            <div class="referral-validation-icon">
                                                {
                                                    let s_icon = s_view.clone();
                                                    move || {
                                                        if s_icon.referral_checking.get() {
                                                            view! {
                                                                <svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                                    <circle cx="12" cy="12" r="10" stroke-dasharray="31.4" stroke-dashoffset="10"/>
                                                                </svg>
                                                            }.into_any()
                                                        } else {
                                                            match s_icon.referral_valid.get() {
                                                                Some(true) => {
                                                                    view! {
                                                                        <svg class="valid" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                                            <polyline points="20 6 9 17 4 12"/>
                                                                        </svg>
                                                                    }.into_any()
                                                                }
                                                                Some(false) => {
                                                                    view! {
                                                                        <svg class="invalid" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                                            <line x1="18" y1="6" x2="6" y2="18"/>
                                                                            <line x1="6" y1="6" x2="18" y2="18"/>
                                                                        </svg>
                                                                    }.into_any()
                                                                }
                                                                None => view! { <></> }.into_any()
                                                            }
                                                        }
                                                    }
                                                }
                                            </div>
                                        </div>
                                        // Validation message
                                        <div class="referral-validation-message">
                                            {
                                                let s_msg = s_view.clone();
                                                move || {
                                                    if s_msg.referral_checking.get() {
                                                        Some(t("wizard.review.referral_checking"))
                                                    } else {
                                                        match s_msg.referral_valid.get() {
                                                            Some(true) => {
                                                                let name = s_msg.referral_name.get().unwrap_or_default();
                                                                Some(format!("{} - {}", t("wizard.review.referral_valid"), name))
                                                            }
                                                            Some(false) => Some(t("wizard.review.referral_invalid").to_string()),
                                                            None => None
                                                        }
                                                    }
                                                }
                                            }
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }
                            }
                        }
                    </div>
                }
            }

            // Error display
            {move || state.error.get().map(|error| view! {
                <div class="review-error">
                    <p>{error}</p>
                </div>
            })}

            // Terms checkbox and Claim button
            <div class="review-actions-row">
                <label class="terms-checkbox-label">
                    <input
                        type="checkbox"
                        id="terms-checkbox"
                        checked=move || state.terms_accepted.get()
                        on:change=on_terms_change
                        disabled=move || state.is_claiming.get()
                    />
                    <span class="checkbox-custom"></span>
                    <span class="terms-text">
                        {move || t("wizard.review.terms_checkbox")}
                        " "
                        <a href="/terms" target="_blank" class="terms-link">
                            {move || t("wizard.review.terms_link")}
                        </a>
                    </span>
                </label>
                <button
                    type="button"
                    class="btn-primary"
                    on:click=on_claim
                    disabled=move || !state.terms_accepted.get() || state.is_claiming.get()
                >
                    {move || {
                        if state.is_claiming.get() {
                            t("wizard.review.claiming")
                        } else {
                            t("wizard.review.claim_now")
                        }
                    }}
                </button>
            </div>
        </div>
    }
}
