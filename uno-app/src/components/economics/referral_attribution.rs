//! Referral attribution display component
//!
//! Shows who referred the user and their commission attribution status.

use leptos::prelude::*;
use crate::hooks::t;
use super::split_display::{bps_to_percentage, REFERRAL_BPS};

/// Referral attribution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttributionStatus {
    /// Attribution pending (claim not yet confirmed)
    #[default]
    Pending,
    /// Attribution confirmed and immutable
    Confirmed,
    /// No referrer (referral share goes to platform)
    NoReferrer,
}

impl AttributionStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Confirmed => "Confirmed",
            Self::NoReferrer => "No Referrer",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Confirmed => "confirmed",
            Self::NoReferrer => "no-referrer",
        }
    }
}

/// Referral attribution display
#[component]
pub fn ReferralAttribution(
    /// Referrer name (if attributed)
    #[prop(default = None)]
    referrer_name: Option<String>,
    /// Referral code used
    #[prop(default = None)]
    referral_code: Option<String>,
    /// Attribution status
    #[prop(default = AttributionStatus::NoReferrer)]
    status: AttributionStatus,
    /// Total earnings attributed to referrer (in dollars)
    #[prop(default = 0.0)]
    referrer_earnings: f64,
    /// Whether to show detailed breakdown
    #[prop(default = false)]
    detailed: bool,
) -> impl IntoView {
    let referral_pct = bps_to_percentage(REFERRAL_BPS);
    let status_class = format!("referral-attribution {}", status.css_class());

    view! {
        <div class=status_class>
            <div class="attribution-header">
                <div class="attribution-icon">
                    {match status {
                        AttributionStatus::Confirmed => view! {
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="icon confirmed">
                                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                                <polyline points="22 4 12 14.01 9 11.01"/>
                            </svg>
                        }.into_any(),
                        AttributionStatus::Pending => view! {
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="icon pending">
                                <circle cx="12" cy="12" r="10"/>
                                <polyline points="12 6 12 12 16 14"/>
                            </svg>
                        }.into_any(),
                        AttributionStatus::NoReferrer => view! {
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="icon no-referrer">
                                <circle cx="12" cy="12" r="10"/>
                                <line x1="8" y1="12" x2="16" y2="12"/>
                            </svg>
                        }.into_any(),
                    }}
                </div>
                <h4 class="attribution-title">{move || t("economics.referral_attribution")}</h4>
                <span class=format!("status-badge {}", status.css_class())>
                    {status.display_name()}
                </span>
            </div>

            <div class="attribution-content">
                {match status {
                    AttributionStatus::Confirmed | AttributionStatus::Pending => {
                        let name = referrer_name.clone().unwrap_or_else(|| "Unknown".to_string());
                        let code = referral_code.clone().unwrap_or_default();
                        view! {
                            <div class="referrer-info">
                                <div class="referrer-avatar">
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                                        <circle cx="12" cy="7" r="4"/>
                                    </svg>
                                </div>
                                <div class="referrer-details">
                                    <p class="referrer-name">{name}</p>
                                    {(!code.is_empty()).then(|| view! {
                                        <p class="referrer-code">{move || t("economics.code")}: {code.clone()}</p>
                                    })}
                                </div>
                            </div>
                        }.into_any()
                    }
                    AttributionStatus::NoReferrer => {
                        view! {
                            <p class="no-referrer-message">
                                {move || t("economics.no_referrer_message")}
                            </p>
                        }.into_any()
                    }
                }}

                {detailed.then(|| view! {
                    <div class="attribution-breakdown">
                        <div class="breakdown-row">
                            <span class="breakdown-label">{move || t("economics.referrer_share")}</span>
                            <span class="breakdown-value">{referral_pct as u32}"%"</span>
                        </div>
                        <div class="breakdown-row">
                            <span class="breakdown-label">{move || t("economics.earned_for_referrer")}</span>
                            <span class="breakdown-value">"$"{format!("{:.2}", referrer_earnings)}</span>
                        </div>
                    </div>
                })}

                // Immutability notice
                {(status == AttributionStatus::Confirmed).then(|| view! {
                    <div class="immutability-notice">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="lock-icon">
                            <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                            <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                        </svg>
                        <span>{move || t("economics.attribution_immutable")}</span>
                    </div>
                })}
            </div>
        </div>
    }
}

/// Simple referrer badge for compact display
#[component]
pub fn ReferrerBadge(
    /// Referrer name
    name: String,
    /// Status
    #[prop(default = AttributionStatus::Confirmed)]
    status: AttributionStatus,
) -> impl IntoView {
    let badge_class = format!("referrer-badge {}", status.css_class());

    view! {
        <div class=badge_class>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="badge-icon">
                <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                <circle cx="8.5" cy="7" r="4"/>
                <line x1="20" y1="8" x2="20" y2="14"/>
                <line x1="23" y1="11" x2="17" y2="11"/>
            </svg>
            <span class="badge-name">{name}</span>
            {(status == AttributionStatus::Confirmed).then(|| view! {
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="verified-icon">
                    <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                    <polyline points="22 4 12 14.01 9 11.01"/>
                </svg>
            })}
        </div>
    }
}
