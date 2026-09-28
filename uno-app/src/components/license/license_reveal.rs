//! License key reveal component

use leptos::prelude::*;
use crate::components::common::CopyButton;
use super::AndroidDownload;

#[component]
pub fn LicenseReveal(
    license_key: String,
    claim_token: String,
) -> impl IntoView {
    let key_for_download = license_key.clone();

    view! {
        <div class="license-reveal">
            <div class="success-header">
                <span class="success-icon">"🎉"</span>
                <h2>"License Claimed Successfully!"</h2>
            </div>

            <div class="license-key-section">
                <label class="section-label">"Your License Key"</label>
                <div class="key-display">
                    <code class="license-key">{license_key.clone()}</code>
                    <CopyButton text=license_key.clone() />
                </div>
                <p class="key-warning">
                    "⚠️ Save this key! You'll need it to activate the app."
                </p>
            </div>

            <div class="claim-token-section">
                <label class="section-label">"Claim Reference"</label>
                <p class="claim-token">{claim_token}</p>
                <p class="token-note">"Save this to recover your license if needed."</p>
            </div>

            <div class="divider"></div>

            <AndroidDownload license_key=key_for_download />
        </div>
    }
}
