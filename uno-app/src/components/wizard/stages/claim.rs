//! Claim stage component - shows license key after successful reservation
//! License is only marked as claimed when the user clicks the copy button.

use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::hooks::{t, use_clipboard};
use crate::components::wizard::state::use_wizard_state;
use crate::api::confirm_license_claim;

/// Mask a license key for display (e.g., "3215022a-dffc-469e-8b22-aa471649ebb8" -> "3215***ebb8")
fn mask_license_key(key: &str) -> String {
    if key.len() <= 8 {
        // Too short to mask meaningfully
        return "****".to_string();
    }
    let prefix_len = 4;
    let suffix_len = 4;
    let prefix = &key[..prefix_len];
    let suffix = &key[key.len() - suffix_len..];
    format!("{}***{}", prefix, suffix)
}

/// Claim stage - shows masked license key, reveals and confirms claim on copy
#[component]
pub fn ClaimStage() -> impl IntoView {
    let state = use_wizard_state().expect("ClaimStage must be rendered within wizard context");
    let (key_revealed, set_key_revealed) = signal(false);
    let (copied, copy_to_clipboard) = use_clipboard();

    view! {
        <div class="wizard-stage wizard-claim">
            // Check if we have a license key (reservation was successful)
            {move || {
                let key = state.claimed_license_key.get();
                let state_clone = state.clone();

                if let Some(k) = key {
                    let key_for_display = k.clone();
                    let key_for_copy = k.clone();
                    let state_for_back = state_clone.clone();
                    let state_for_copy = state_clone.clone();
                    let state_for_continue = state_clone.clone();
                    let copy_fn = copy_to_clipboard.clone();

                    // Handle copy button click - confirms claim and reveals key
                    let on_copy_click = move |_| {
                        let s = state_for_copy.clone();
                        let license_id = s.claimed_license_id.get();
                        let referral_code = s.referral_code_to_apply.get();
                        let key_to_copy = key_for_copy.clone();
                        let copy = copy_fn.clone();

                        // If already confirmed, just copy
                        if s.is_confirmed.get() {
                            copy(key_to_copy);
                            return;
                        }

                        // Start confirmation
                        s.start_confirming();

                        spawn_local(async move {
                            match confirm_license_claim(
                                license_id.unwrap_or_default(),
                                None, // device_id
                                referral_code,
                            ).await {
                                Ok(response) => {
                                    if response.success {
                                        // Copy the key to clipboard
                                        copy(key_to_copy);
                                        // Reveal the key and mark as confirmed
                                        set_key_revealed.set(true);
                                        s.set_confirm_success();
                                    } else {
                                        s.set_claim_error(response.error.unwrap_or_else(|| "Confirmation failed".to_string()));
                                    }
                                }
                                Err(e) => {
                                    s.set_claim_error(e.to_string());
                                }
                            }
                        });
                    };

                    // License has been reserved - show masked key first, reveal after copy
                    view! {
                        <div class="claim-success">
                            // Before copy: show instruction to copy the key
                            <Show when=move || !key_revealed.get()>
                                <h2 class="stage-title">{move || t("wizard.claim.copy_title")}</h2>
                                <p class="stage-subtitle">{move || t("wizard.claim.copy_subtitle")}</p>
                            </Show>

                            // After copy: show success icon and message
                            <Show when=move || key_revealed.get()>
                                <div class="success-icon-large">
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                                        <polyline points="22 4 12 14.01 9 11.01"/>
                                    </svg>
                                </div>
                                <h2 class="stage-title success">{move || t("wizard.claim.success_title")}</h2>
                                <p class="stage-subtitle">{move || t("wizard.claim.success_subtitle")}</p>
                            </Show>

                            // License key display - shows masked or full key
                            <div class="license-key-section">
                                <div class="key-label">{move || t("wizard.success.license_key")}</div>
                                <div class="key-display">
                                    <code class="license-key">
                                        {move || {
                                            let k = key_for_display.clone();
                                            if key_revealed.get() {
                                                k
                                            } else {
                                                mask_license_key(&k)
                                            }
                                        }}
                                    </code>
                                    <button
                                        class="copy-btn"
                                        class:copied=move || copied.get()
                                        class:confirming=move || state_clone.is_confirming.get()
                                        on:click=on_copy_click
                                        disabled=move || state_clone.is_confirming.get()
                                        title=move || {
                                            if key_revealed.get() {
                                                "Copy to clipboard"
                                            } else {
                                                "Click to reveal and copy your license key"
                                            }
                                        }
                                    >
                                        {move || {
                                            if state_clone.is_confirming.get() {
                                                // Show spinner during confirmation
                                                view! {
                                                    <svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                        <circle cx="12" cy="12" r="10" stroke-dasharray="31.4" stroke-dashoffset="10"/>
                                                    </svg>
                                                }.into_any()
                                            } else if copied.get() {
                                                // Show checkmark after copy
                                                view! {
                                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                        <polyline points="20 6 9 17 4 12"/>
                                                    </svg>
                                                }.into_any()
                                            } else {
                                                // Show copy icon
                                                view! {
                                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                        <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                                        <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
                                                    </svg>
                                                }.into_any()
                                            }
                                        }}
                                    </button>
                                </div>
                                <p class="key-warning">
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="warning-icon">
                                        <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
                                        <line x1="12" y1="9" x2="12" y2="13"/>
                                        <line x1="12" y1="17" x2="12.01" y2="17"/>
                                    </svg>
                                    {move || t("wizard.success.key_warning")}
                                </p>
                            </div>

                            // Error display
                            {move || state_clone.error.get().map(|error| view! {
                                <div class="claim-error">
                                    <p>{error}</p>
                                </div>
                            })}

                            // Footer with Back and Continue buttons
                            {
                                let state_for_back_btn = state_for_back.clone();
                                let state_for_continue_btn = state_for_continue.clone();
                                let state_for_continue_disabled = state_for_continue.clone();
                                view! {
                                    <div class="wizard-footer">
                                        <button
                                            type="button"
                                            class="btn-secondary"
                                            on:click=move |_| state_for_back.prev_stage()
                                            disabled=move || state_for_back_btn.is_confirming.get()
                                        >
                                            {move || t("wizard.common.back")}
                                        </button>
                                        <button
                                            type="button"
                                            class="btn-primary"
                                            on:click=move |_| state_for_continue_btn.next_stage()
                                            disabled=move || !key_revealed.get() || state_for_continue_disabled.is_confirming.get()
                                        >
                                            {move || t("wizard.claim.continue")}
                                        </button>
                                    </div>
                                }
                            }
                        </div>
                    }.into_any()
                } else {
                    // No key yet - this shouldn't normally happen as we navigate here after successful reservation
                    // But show a loading/pending state just in case
                    view! {
                        <div class="claim-pending">
                            <div class="spinner"></div>
                            <h2 class="stage-title">{move || t("wizard.claim.processing")}</h2>
                            <p class="stage-subtitle">{move || t("wizard.claim.please_wait")}</p>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
