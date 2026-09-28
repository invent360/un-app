//! Public statistics dashboard page

use leptos::prelude::*;
use crate::api::get_network_stats;
use crate::hooks::t;

/// Statistics dashboard page component
#[component]
pub fn StatsPage() -> impl IntoView {
    let stats = Resource::new(|| (), |_| get_network_stats());

    view! {
        <div class="page stats-page">
            <header class="page-header">
                <h1>{move || t("stats.title")}</h1>
                <p>{move || t("stats.subtitle")}</p>
            </header>

            <Suspense fallback=move || view! { <div class="loading">{move || t("stats.loading")}</div> }>
                {move || {
                    stats.get().map(|result| {
                        match result {
                            Ok(data) => view! {
                                <section class="stats-overview">
                                    <div class="stat-card">
                                        <h3>{move || t("stats.total_licenses")}</h3>
                                        <p class="stat-value">{data.total_licenses}</p>
                                    </div>
                                    <div class="stat-card">
                                        <h3>{move || t("stats.claimed_licenses")}</h3>
                                        <p class="stat-value">{data.claimed_licenses}</p>
                                    </div>
                                    <div class="stat-card">
                                        <h3>{move || t("stats.available")}</h3>
                                        <p class="stat-value">{data.total_licenses - data.claimed_licenses}</p>
                                    </div>
                                    <div class="stat-card">
                                        <h3>{move || t("stats.avg_user_share")}</h3>
                                        <p class="stat-value">{format!("{:.0}%", data.average_user_share)}</p>
                                    </div>
                                </section>

                                <section class="stats-detail">
                                    <h2>{move || t("stats.distribution")}</h2>
                                    <div class="distribution-info">
                                        <div class="distribution-bar">
                                            <div
                                                class="claimed-portion"
                                                style=move || {
                                                    let pct = if data.total_licenses > 0 {
                                                        (data.claimed_licenses as f64 / data.total_licenses as f64) * 100.0
                                                    } else {
                                                        0.0
                                                    };
                                                    format!("width: {}%", pct)
                                                }
                                            />
                                        </div>
                                        <div class="distribution-labels">
                                            <span class="claimed-label">
                                                {move || {
                                                    let pct = if data.total_licenses > 0 {
                                                        (data.claimed_licenses as f64 / data.total_licenses as f64) * 100.0
                                                    } else {
                                                        0.0
                                                    };
                                                    format!("{:.1}% {}", pct, t("stats.claimed"))
                                                }}
                                            </span>
                                            <span class="available-label">
                                                {move || {
                                                    let pct = if data.total_licenses > 0 {
                                                        ((data.total_licenses - data.claimed_licenses) as f64 / data.total_licenses as f64) * 100.0
                                                    } else {
                                                        0.0
                                                    };
                                                    format!("{:.1}% {}", pct, t("stats.available"))
                                                }}
                                            </span>
                                        </div>
                                    </div>
                                </section>

                                <section class="stats-cta">
                                    <h2>{move || t("stats.cta_title")}</h2>
                                    <p>{move || t("stats.cta_subtitle")}</p>
                                    <a href="/" class="btn-primary btn-lg">{move || t("hero.cta")}</a>
                                </section>
                            }.into_any(),
                            Err(e) => view! {
                                <div class="error-state">
                                    <p>{move || t("stats.error_prefix")} " " {e.to_string()}</p>
                                </div>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
