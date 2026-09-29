//! Earnings transparency dashboard component
//!
//! Shows a complete breakdown of earnings with the 50/40/10 split,
//! including historical data and projections.

use leptos::prelude::*;
use crate::hooks::t;
use super::split_display::{SplitDisplay, bps_to_percentage, ULO_BPS};

/// Earnings period for display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EarningsPeriod {
    #[default]
    Daily,
    Weekly,
    Monthly,
}

impl EarningsPeriod {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Daily => "Daily",
            Self::Weekly => "Weekly",
            Self::Monthly => "Monthly",
        }
    }

    pub fn multiplier(&self) -> f64 {
        match self {
            Self::Daily => 1.0,
            Self::Weekly => 7.0,
            Self::Monthly => 30.0,
        }
    }
}

/// Earnings dashboard - shows complete earnings transparency
#[component]
pub fn EarningsDashboard(
    /// Total pool earnings (in dollars)
    #[prop(default = 0.0)]
    total_pool: f64,
    /// Your share (calculated from pool * 50%)
    #[prop(default = None)]
    your_share: Option<f64>,
    /// Pending earnings awaiting payout
    #[prop(default = 0.0)]
    pending: f64,
    /// Paid out earnings
    #[prop(default = 0.0)]
    paid: f64,
    /// Whether data is loading
    #[prop(default = false)]
    loading: bool,
) -> impl IntoView {
    // Calculate your share if not provided
    let calculated_share = your_share.unwrap_or_else(|| total_pool * bps_to_percentage(ULO_BPS) / 100.0);

    let (period, set_period) = signal(EarningsPeriod::Monthly);

    view! {
        <div class="earnings-dashboard">
            <div class="dashboard-header">
                <h2 class="dashboard-title">{move || t("economics.earnings_title")}</h2>
                <p class="dashboard-subtitle">{move || t("economics.earnings_subtitle")}</p>
            </div>

            // Period selector
            <div class="period-selector">
                <button
                    class=move || if period.get() == EarningsPeriod::Daily { "period-btn active" } else { "period-btn" }
                    on:click=move |_| set_period.set(EarningsPeriod::Daily)
                >
                    {EarningsPeriod::Daily.display_name()}
                </button>
                <button
                    class=move || if period.get() == EarningsPeriod::Weekly { "period-btn active" } else { "period-btn" }
                    on:click=move |_| set_period.set(EarningsPeriod::Weekly)
                >
                    {EarningsPeriod::Weekly.display_name()}
                </button>
                <button
                    class=move || if period.get() == EarningsPeriod::Monthly { "period-btn active" } else { "period-btn" }
                    on:click=move |_| set_period.set(EarningsPeriod::Monthly)
                >
                    {EarningsPeriod::Monthly.display_name()}
                </button>
            </div>

            // Main earnings display
            <Show
                when=move || !loading
                fallback=|| view! {
                    <div class="earnings-loading">
                        <div class="loading-spinner"></div>
                    </div>
                }
            >
                <div class="earnings-summary">
                    // Your earnings card (prominent)
                    <div class="earnings-card primary">
                        <div class="earnings-card-header">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="earnings-icon">
                                <line x1="12" y1="1" x2="12" y2="23"/>
                                <path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/>
                            </svg>
                            <span class="earnings-label">{move || t("economics.your_earnings")}</span>
                        </div>
                        <p class="earnings-amount">"$"{format!("{:.2}", calculated_share)}</p>
                        <p class="earnings-note">{move || t("economics.fifty_percent_share")}</p>
                    </div>

                    // Split visualization
                    <div class="earnings-card split">
                        <h4 class="card-title">{move || t("economics.how_split")}</h4>
                        <SplitDisplay pool_amount=Some(total_pool) show_amounts=true compact=true />
                    </div>
                </div>

                // Payout status
                <div class="payout-status">
                    <div class="payout-card pending">
                        <div class="payout-header">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="payout-icon">
                                <circle cx="12" cy="12" r="10"/>
                                <polyline points="12 6 12 12 16 14"/>
                            </svg>
                            <span class="payout-label">{move || t("economics.pending")}</span>
                        </div>
                        <p class="payout-amount">"$"{format!("{:.2}", pending)}</p>
                    </div>

                    <div class="payout-card paid">
                        <div class="payout-header">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="payout-icon">
                                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                                <polyline points="22 4 12 14.01 9 11.01"/>
                            </svg>
                            <span class="payout-label">{move || t("economics.paid")}</span>
                        </div>
                        <p class="payout-amount">"$"{format!("{:.2}", paid)}</p>
                    </div>
                </div>

                // Transparency notice
                <div class="transparency-notice">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="notice-icon">
                        <circle cx="12" cy="12" r="10"/>
                        <line x1="12" y1="16" x2="12" y2="12"/>
                        <line x1="12" y1="8" x2="12.01" y2="8"/>
                    </svg>
                    <p>{move || t("economics.transparency_notice")}</p>
                </div>
            </Show>
        </div>
    }
}

/// Compact earnings summary for embedding in other components
#[component]
pub fn EarningsSummaryCompact(
    /// Your total earnings
    earnings: f64,
    /// Show trend indicator
    #[prop(default = false)]
    show_trend: bool,
    /// Trend direction (true = up, false = down)
    #[prop(default = true)]
    trend_up: bool,
) -> impl IntoView {
    view! {
        <div class="earnings-summary-compact">
            <div class="earnings-value">
                <span class="currency">"$"</span>
                <span class="amount">{format!("{:.2}", earnings)}</span>
            </div>
            {show_trend.then(|| view! {
                <div class=if trend_up { "trend up" } else { "trend down" }>
                    {if trend_up {
                        view! {
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="18 15 12 9 6 15"/>
                            </svg>
                        }.into_any()
                    } else {
                        view! {
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="6 9 12 15 18 9"/>
                            </svg>
                        }.into_any()
                    }}
                </div>
            })}
        </div>
    }
}
