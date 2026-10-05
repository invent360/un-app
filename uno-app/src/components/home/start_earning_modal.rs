//! Start Earning Modal component
//!
//! Frictionless claim flow: Terms → Claim → Dashboard
//! No authentication required, KYC is just a tooltip reminder.

use leptos::prelude::*;
use crate::hooks::t;

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
use crate::components::common::{Modal, ModalSize};

/// Start Earning Modal - frictionless license claim flow
#[component]
pub fn StartEarningModal(
    /// Signal controlling modal visibility
    is_open: RwSignal<bool>,
    /// Callback when claim is successful (receives license key)
    #[prop(optional)]
    on_success: Option<Callback<String>>,
) -> impl IntoView {
    // Form state
    let (terms_accepted, set_terms_accepted) = signal(false);
    let (is_claiming, set_is_claiming) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (claim_success, set_claim_success) = signal(None::<ClaimResult>);

    // Close modal handler
    let close_modal = move |_: ()| {
        is_open.set(false);
        // Reset state on close
        set_terms_accepted.set(false);
        set_error.set(None);
        set_claim_success.set(None);
    };

    // Handle claim action
    let handle_claim = move |_: ()| {
        if !terms_accepted.get() {
            return;
        }

        set_is_claiming.set(true);
        set_error.set(None);

        #[cfg(any(feature = "csr", feature = "hydrate"))]
        {
            use leptos::task::spawn_local;
            spawn_local(async move {
                // Call the anonymous claim API endpoint
                match call_anonymous_claim_api().await {
                    Ok(result) => {
                        set_claim_success.set(Some(result.clone()));
                        if let Some(callback) = on_success {
                            callback.run(result.license_key);
                        }
                    }
                    Err(e) => {
                        set_error.set(Some(e));
                    }
                }
                set_is_claiming.set(false);
            });
        }

        #[cfg(not(any(feature = "csr", feature = "hydrate")))]
        {
            set_is_claiming.set(false);
        }
    };

    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    {
        view! {
            <Modal
                open=Signal::from(is_open)
                on_close=Callback::new(move |_| close_modal(()))
                title=t("home.start.title").to_string()
                size=ModalSize::Default
            >
                <div class="start-earning-modal">
                    // Show success state or form
                    {move || {
                        if let Some(result) = claim_success.get() {
                            view! { <ClaimSuccessView result=result on_close=close_modal /> }.into_any()
                        } else {
                            view! {
                                <ClaimFormView
                                    terms_accepted=terms_accepted
                                    set_terms_accepted=set_terms_accepted
                                    is_claiming=is_claiming
                                    error=error
                                    on_claim=handle_claim
                                />
                            }.into_any()
                        }
                    }}
                </div>
            </Modal>
        }
    }

    #[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
    {
        view! { <div></div> }
    }
}

/// Claim result data
#[derive(Clone, Debug)]
pub struct ClaimResult {
    pub license_key: String,
    pub session_id: String,
}

/// Claim form view (before success)
#[component]
fn ClaimFormView(
    terms_accepted: ReadSignal<bool>,
    set_terms_accepted: WriteSignal<bool>,
    is_claiming: ReadSignal<bool>,
    error: ReadSignal<Option<String>>,
    on_claim: impl Fn(()) + 'static + Copy,
) -> impl IntoView {
    view! {
        <div class="claim-form">
            <p class="modal-subtitle">{move || t("home.start.subtitle")}</p>

            // KYC tooltip (non-blocking informational)
            <div class="kyc-tooltip">
                <span class="tooltip-icon">"ℹ️"</span>
                <span class="tooltip-text">{move || t("home.start.kyc_note")}</span>
            </div>

            // Terms checkbox
            <label class="terms-checkbox">
                <input
                    type="checkbox"
                    prop:checked=terms_accepted
                    on:change=move |ev| set_terms_accepted.set(event_target_checked(&ev))
                    disabled=is_claiming
                />
                <span class="terms-text">
                    {move || t("home.start.accept_terms")} " "
                    <a href="/terms" target="_blank" rel="noopener">{move || t("home.start.terms_link")}</a>
                    " " {move || t("home.start.and")} " "
                    <a href="/privacy" target="_blank" rel="noopener">{move || t("home.start.privacy_link")}</a>
                </span>
            </label>

            // Error display
            {move || error.get().map(|e| view! {
                <div class="claim-error">
                    <span class="error-icon">"⚠️"</span>
                    <span class="error-text">{e}</span>
                </div>
            })}

            // Claim button
            <button
                class="btn btn-primary btn-large claim-button"
                disabled=move || !terms_accepted.get() || is_claiming.get()
                on:click=move |_| on_claim(())
            >
                {move || {
                    if is_claiming.get() {
                        t("home.start.claiming")
                    } else {
                        t("home.start.claim_button")
                    }
                }}
            </button>

            // Additional info
            <p class="claim-info">
                {move || t("home.start.no_payment")} " • " {move || t("home.start.instant_access")}
            </p>
        </div>
    }
}

/// Success view after successful claim
#[component]
fn ClaimSuccessView(
    result: ClaimResult,
    on_close: impl Fn(()) + 'static + Copy,
) -> impl IntoView {
    let (copied, set_copied) = signal(false);
    let license_key = result.license_key.clone();
    let license_key_for_copy = result.license_key.clone();

    let copy_to_clipboard = move |_| {
        #[cfg(any(feature = "csr", feature = "hydrate"))]
        {
            if let Some(window) = web_sys::window() {
                let clipboard = window.navigator().clipboard();
                let key = license_key_for_copy.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = wasm_bindgen_futures::JsFuture::from(
                        clipboard.write_text(&key)
                    ).await;
                });
                set_copied.set(true);
                // Reset after 2 seconds
                let set_copied_clone = set_copied;
                leptos::task::spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(2000).await;
                    set_copied_clone.set(false);
                });
            }
        }
    };

    view! {
        <div class="claim-success">
            <div class="success-icon">"🎉"</div>
            <h3 class="success-title">{move || t("home.start.success_title")}</h3>
            <p class="success-subtitle">{move || t("home.start.success_subtitle")}</p>

            // License key display
            <div class="license-key-box">
                <label class="key-label">{move || t("home.start.your_license_key")}</label>
                <div class="key-display">
                    <code class="key-value">{license_key.clone()}</code>
                    <button
                        class="copy-btn"
                        on:click=copy_to_clipboard
                        title="Copy to clipboard"
                    >
                        {move || if copied.get() { "✓" } else { "📋" }}
                    </button>
                </div>
                <p class="key-warning">{move || t("home.start.key_warning")}</p>
            </div>

            // Next steps
            <div class="next-steps">
                <h4 class="steps-title">{move || t("home.start.next_steps_title")}</h4>
                <ol class="steps-list">
                    <li>{move || t("home.start.step_1")}</li>
                    <li>{move || t("home.start.step_2")}</li>
                    <li>{move || t("home.start.step_3")}</li>
                </ol>
            </div>

            // Action buttons
            <div class="success-actions">
                <a href="/setup" class="btn btn-primary">
                    {move || t("home.start.setup_guide")}
                </a>
                <button class="btn btn-secondary" on:click=move |_| on_close(())>
                    {move || t("home.start.done")}
                </button>
            </div>
        </div>
    }
}

/// Request body for anonymous claim API
#[cfg(any(feature = "csr", feature = "hydrate"))]
#[derive(serde::Serialize)]
struct AnonymousClaimRequest {
    terms_accepted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    referral_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_fingerprint: Option<String>,
    consent_version: String,
}

/// Response from anonymous claim API
#[cfg(any(feature = "csr", feature = "hydrate"))]
#[derive(serde::Deserialize)]
struct AnonymousClaimResponse {
    session_id: String,
    license_id: String,
    license_key: String,
}

/// Call the anonymous claim API endpoint
#[cfg(any(feature = "csr", feature = "hydrate"))]
async fn call_anonymous_claim_api() -> Result<ClaimResult, String> {
    use gloo_net::http::Request;

    let request_body = AnonymousClaimRequest {
        terms_accepted: true,
        referral_code: None, // TODO: Could be extracted from URL params
        device_fingerprint: None,
        consent_version: "1.0".to_string(),
    };

    let response = Request::post("/api/v1/claim/anonymous")
        .header("Content-Type", "application/json")
        .json(&request_body)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() {
        // Try to extract error message from response body
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Claim failed: {}", error_text));
    }

    let claim_response: AnonymousClaimResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(ClaimResult {
        license_key: claim_response.license_key,
        session_id: claim_response.session_id,
    })
}

/// Start Earning Button component
/// Can be placed anywhere on the page to trigger the modal
#[component]
pub fn StartEarningButton(
    /// Modal open signal to control
    modal_open: RwSignal<bool>,
    /// Optional custom button text key
    #[prop(optional)]
    text_key: Option<&'static str>,
    /// Optional custom CSS class
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    let button_class = class.unwrap_or("btn btn-primary btn-large start-earning-btn");
    let text = text_key.unwrap_or("home.start.cta_button");

    view! {
        <button
            class=button_class
            on:click=move |_| modal_open.set(true)
        >
            {move || t(text)}
        </button>
    }
}
