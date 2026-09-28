//! Installation guide component

use leptos::prelude::*;
use crate::hooks::t;

/// Installation guide with steps
#[component]
pub fn InstallGuide() -> impl IntoView {
    view! {
        <div class="install-guide">
            <h3 class="guide-title">{move || t("install.title")}</h3>

            // Android installation
            <div class="platform-guide">
                <h4 class="platform-title">{move || t("install.android.title")}</h4>
                <ol class="guide-steps">
                    <li>
                        <span class="step-number">"1"</span>
                        <span class="step-text">{move || t("install.android.step1")}</span>
                    </li>
                    <li>
                        <span class="step-number">"2"</span>
                        <span class="step-text">{move || t("install.android.step2")}</span>
                    </li>
                    <li>
                        <span class="step-number">"3"</span>
                        <span class="step-text">{move || t("install.android.step3")}</span>
                    </li>
                    <li>
                        <span class="step-number">"4"</span>
                        <span class="step-text">{move || t("install.android.step4")}</span>
                    </li>
                </ol>
            </div>

            // iOS installation (if applicable)
            <div class="platform-guide">
                <h4 class="platform-title">{move || t("install.ios.title")}</h4>
                <ol class="guide-steps">
                    <li>
                        <span class="step-number">"1"</span>
                        <span class="step-text">{move || t("install.ios.step1")}</span>
                    </li>
                    <li>
                        <span class="step-number">"2"</span>
                        <span class="step-text">{move || t("install.ios.step2")}</span>
                    </li>
                </ol>
            </div>

            // Tips
            <div class="install-tips">
                <h4 class="tips-title">{move || t("install.tips.title")}</h4>
                <ul class="tips-list">
                    <li>{move || t("install.tips.tip1")}</li>
                    <li>{move || t("install.tips.tip2")}</li>
                    <li>{move || t("install.tips.tip3")}</li>
                </ul>
            </div>
        </div>
    }
}
