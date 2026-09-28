//! License claim reveal page route

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::api::get_claim_data;
use crate::components::license::{LicenseReveal, HelpAccordion};
use crate::hooks::t;

/// Claim page component - displays claimed license details
#[component]
pub fn ClaimPage() -> impl IntoView {
    
    let params = use_params_map();
    let claim_token = move || params.read().get("id").unwrap_or_default();

    let claim_data = Resource::new(
        move || claim_token(),
        |token| async move {
            if token.is_empty() {
                return Err(ServerFnError::new("Invalid claim token"));
            }
            get_claim_data(token).await
        }
    );

    view! {
        <div class="page claim-page">
            <Suspense fallback=move || view! { <div class="loading">{move || t("claim.loading")}</div> }>
                {move || {
                    claim_data.get().map(|result| {
                        match result {
                            Ok(data) => view! {
                                <div class="claim-content">
                                    <header class="claim-header">
                                        <h1>{move || t("claim.success_title")}</h1>
                                        <p class="claim-subtitle">
                                            {move || t("claim.success_subtitle")}
                                        </p>
                                        <p class="split-info">
                                            {data.user_share} ":" {data.operator_share} " " {move || t("licenses.split")}
                                        </p>
                                    </header>

                                    <LicenseReveal
                                        license_key=data.license_key.clone()
                                        claim_token=data.claim_token.clone()
                                    />

                                    <section class="earnings-info">
                                        <h3>{move || t("claim.earnings_title")}</h3>
                                        <p class="earnings-range">
                                            {move || {
                                                match (data.min_earnings, data.max_earnings) {
                                                    (Some(min), Some(max)) => format!("${:.2} - ${:.2}{}", min, max, t("common.per_month")),
                                                    _ => t("claim.earnings_default")
                                                }
                                            }}
                                        </p>
                                    </section>

                                    <HelpAccordion />
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <div class="error-state">
                                    <h2>{move || t("claim.error_title")}</h2>
                                    <p>{move || t("claim.error_desc")} " " {e.to_string()}</p>
                                    <a href="/" class="btn-primary">{move || t("claim.error_cta")}</a>
                                </div>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
