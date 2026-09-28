//! Help accordion component for claim page

use leptos::prelude::*;
use crate::hooks::t;

#[component]
pub fn HelpAccordion() -> impl IntoView {
    view! {
        <div class="help-accordion">
            <h3>{move || t("help.title")}</h3>

            <details class="help-item">
                <summary>{move || t("help.install_q")}</summary>
                <div class="help-content">
                    <p>{move || t("help.install_step1")}</p>
                    <p>{move || t("help.install_step2")}</p>
                    <p>{move || t("help.install_step3")}</p>
                    <p>{move || t("help.install_step4")}</p>
                </div>
            </details>

            <details class="help-item">
                <summary>{move || t("help.register_q")}</summary>
                <div class="help-content">
                    <p>{move || t("help.register_step1")}</p>
                    <p>{move || t("help.register_step2")}</p>
                    <p>{move || t("help.register_step3")}</p>
                    <p>{move || t("help.register_step4")}</p>
                    <p>{move || t("help.register_step5")}</p>
                </div>
            </details>

            <details class="help-item">
                <summary>{move || t("help.trouble_q")}</summary>
                <div class="help-content">
                    <p>"• " {move || t("help.trouble_tip1")}</p>
                    <p>"• " {move || t("help.trouble_tip2")}</p>
                    <p>"• " {move || t("help.trouble_tip3")}</p>
                    <p>"• " {move || t("help.trouble_tip4")}</p>
                    <p>"• " {move || t("help.trouble_tip5")}</p>
                </div>
            </details>

            <details class="help-item">
                <summary>{move || t("help.earnings_q")}</summary>
                <div class="help-content">
                    <p>{move || t("help.earnings_a")}</p>
                </div>
            </details>
        </div>
    }
}
