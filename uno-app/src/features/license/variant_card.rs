//! Unified Variant Card component
//!
//! A single, reusable variant card component that works in both contexts:
//! - Page display mode: Full card with claim button (license page)
//! - Compact/selection mode: Simplified card for wizard selection
//!
//! This consolidates the previously separate implementations:
//! - components/license/variant_card.rs
//! - components/wizard/components/variant_card.rs

use leptos::prelude::*;
use crate::hooks::t;
use crate::types::LicenseVariant;
use super::UrgencyBar;

/// Variant card display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VariantCardMode {
    /// Full display with urgency bar and claim button (for license page)
    #[default]
    Full,
    /// Compact display for selection (for wizard)
    Compact,
}

/// Unified variant card component
///
/// # Examples
///
/// ```ignore
/// // Full mode (license page) with claim handler
/// view! { <VariantCard variant=variant mode=VariantCardMode::Full on_action=on_claim_callback /> }
///
/// // Compact mode with selection callback (wizard)
/// view! { <VariantCard variant=variant mode=VariantCardMode::Compact on_action=on_select_callback /> }
/// ```
#[component]
pub fn VariantCard(
    /// The license variant to display
    variant: LicenseVariant,
    /// Display mode (full or compact)
    #[prop(optional, default = VariantCardMode::Full)]
    mode: VariantCardMode,
    /// Action callback - called when claim/select button is clicked
    /// Receives the variant that was clicked
    #[prop(optional, into)]
    on_action: Option<Callback<LicenseVariant>>,
) -> impl IntoView {
    let variant_for_action = variant.clone();
    let is_featured = variant.is_featured;
    let is_available = variant.is_available();
    let split_display = variant.split_display();
    let progress = variant.progress();
    let remaining = variant.remaining();
    let user_share = variant.user_share_percentage;
    let lease_months = variant.lease_duration_months;
    let min_earnings = variant.min_monthly_earnings;
    let max_earnings = variant.max_monthly_earnings;
    let total_quantity = variant.total_quantity;
    let display_name = variant.display_name.clone();

    // Handle action click
    let handle_action = move |_| {
        if let Some(callback) = &on_action {
            callback.run(variant_for_action.clone());
        }
    };

    match mode {
        VariantCardMode::Full => {
            view! {
                <div
                    class="variant-card"
                    class:featured=is_featured
                    class:unavailable=!is_available
                >
                    {is_featured.then(|| view! {
                        <span class="featured-badge">{move || t("licenses.special_offer")}</span>
                    })}

                    <div class="card-header">
                        <h3 class="split-display">{split_display} " " {move || t("licenses.split")}</h3>
                        {display_name.map(|name| view! {
                            <p class="variant-name">{name}</p>
                        })}
                    </div>

                    <div class="card-body">
                        <p class="share-info">
                            {move || t("licenses.you_keep")} " " <strong>{user_share}"%"</strong> " " {move || t("licenses.of_earnings")}
                        </p>

                        <UrgencyBar
                            progress=progress
                            remaining=remaining
                            total=total_quantity
                        />

                        <div class="earnings-estimate">
                            {move || {
                                match (min_earnings, max_earnings) {
                                    (Some(min), Some(max)) => view! {
                                        <p>{move || t("licenses.est")} " " <strong>{format!("${:.2} - ${:.2}", min, max)}</strong>{move || t("licenses.per_month")}</p>
                                    }.into_any(),
                                    _ => view! {
                                        <p>{move || t("licenses.earnings_vary")}</p>
                                    }.into_any()
                                }
                            }}
                        </div>

                        <div class="lease-info">
                            <span>{lease_months} " " {move || t("licenses.month_lease")}</span>
                        </div>
                    </div>

                    <div class="card-footer">
                        <button
                            class="btn-primary claim-btn"
                            disabled=move || !is_available
                            on:click=handle_action
                        >
                            {move || {
                                if !is_available {
                                    t("licenses.unavailable")
                                } else {
                                    t("licenses.claim")
                                }
                            }}
                        </button>
                    </div>
                </div>
            }.into_any()
        }

        VariantCardMode::Compact => {
            view! {
                <button
                    type="button"
                    class="wizard-variant-card"
                    class:featured=is_featured
                    on:click=handle_action
                >
                    {is_featured.then(|| view! {
                        <div class="featured-badge">
                            {move || t("licenses.special_offer")}
                        </div>
                    })}

                    <div class="variant-split">
                        <span class="split-value">{split_display}</span>
                        <span class="split-label">{move || t("licenses.split")}</span>
                    </div>

                    <div class="variant-details">
                        <div class="detail-row">
                            <span class="detail-label">{move || t("wizard.variant.your_share")}</span>
                            <span class="detail-value">{format!("{}%", user_share)}</span>
                        </div>

                        {min_earnings.zip(max_earnings).map(|(min, max)| view! {
                            <div class="detail-row earnings">
                                <span class="detail-label">{move || t("licenses.earnings")}</span>
                                <span class="detail-value earnings-value">
                                    {format!("${:.0} - ${:.0}", min, max)}
                                </span>
                            </div>
                        })}

                        <div class="detail-row remaining">
                            <span class="remaining-count">{remaining}</span>
                            " "
                            <span class="remaining-label">{move || t("licenses.remaining")}</span>
                        </div>
                    </div>

                    <div class="select-indicator">
                        <span class="select-text">{move || t("wizard.variant.select_btn")}</span>
                        <span class="select-arrow">"→"</span>
                    </div>
                </button>
            }.into_any()
        }
    }
}
