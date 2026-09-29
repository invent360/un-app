//! Split display components for 50/40/10 revenue visualization

use leptos::prelude::*;
use crate::hooks::t;

/// Split allocation percentages (matching uno-api RevenueSplit defaults)
pub const ULO_BPS: u32 = 5000;  // 50%
pub const UNO_BPS: u32 = 4000;  // 40%
pub const REFERRAL_BPS: u32 = 1000;  // 10%
pub const TOTAL_BPS: u32 = 10000;

/// Convert basis points to percentage
pub fn bps_to_percentage(bps: u32) -> f64 {
    bps as f64 / 100.0
}

/// Split display component - shows the revenue split breakdown
#[component]
pub fn SplitDisplay(
    /// Pool amount in dollars (for calculating split amounts)
    #[prop(default = None)]
    pool_amount: Option<f64>,
    /// Whether to show amounts (vs just percentages)
    #[prop(default = false)]
    show_amounts: bool,
    /// Compact mode (smaller, inline)
    #[prop(default = false)]
    compact: bool,
) -> impl IntoView {
    let ulo_pct = bps_to_percentage(ULO_BPS);
    let uno_pct = bps_to_percentage(UNO_BPS);
    let referral_pct = bps_to_percentage(REFERRAL_BPS);

    let container_class = if compact { "split-display compact" } else { "split-display" };

    view! {
        <div class=container_class>
            <SplitBar />

            <div class="split-cards">
                <SplitCard
                    title_key="economics.you"
                    percentage=ulo_pct
                    amount=pool_amount.map(|p| p * ulo_pct / 100.0)
                    show_amount=show_amounts
                    variant="ulo"
                    icon="user"
                />
                <SplitCard
                    title_key="economics.platform"
                    percentage=uno_pct
                    amount=pool_amount.map(|p| p * uno_pct / 100.0)
                    show_amount=show_amounts
                    variant="uno"
                    icon="server"
                />
                <SplitCard
                    title_key="economics.referrer"
                    percentage=referral_pct
                    amount=pool_amount.map(|p| p * referral_pct / 100.0)
                    show_amount=show_amounts
                    variant="referral"
                    icon="users"
                />
            </div>
        </div>
    }
}

/// Visual split bar showing proportional segments
#[component]
pub fn SplitBar() -> impl IntoView {
    let ulo_width = bps_to_percentage(ULO_BPS);
    let uno_width = bps_to_percentage(UNO_BPS);
    let referral_width = bps_to_percentage(REFERRAL_BPS);

    view! {
        <div class="split-bar" role="img" aria-label="Revenue split visualization">
            <div
                class="split-segment ulo"
                style=format!("width: {}%", ulo_width)
                title=format!("You: {}%", ulo_width as u32)
            >
                <span class="segment-label">{ulo_width as u32}"%"</span>
            </div>
            <div
                class="split-segment uno"
                style=format!("width: {}%", uno_width)
                title=format!("Platform: {}%", uno_width as u32)
            >
                <span class="segment-label">{uno_width as u32}"%"</span>
            </div>
            <div
                class="split-segment referral"
                style=format!("width: {}%", referral_width)
                title=format!("Referrer: {}%", referral_width as u32)
            >
                <span class="segment-label">{referral_width as u32}"%"</span>
            </div>
        </div>
    }
}

/// Individual split card showing one party's share
#[component]
pub fn SplitCard(
    /// Translation key for card title (e.g., "economics.you")
    title_key: &'static str,
    /// Percentage share
    percentage: f64,
    /// Dollar amount (if known)
    #[prop(default = None)]
    amount: Option<f64>,
    /// Whether to show the amount
    #[prop(default = false)]
    show_amount: bool,
    /// Card variant for styling (ulo, uno, referral)
    #[prop(default = "default")]
    variant: &'static str,
    /// Icon name (user, server, users)
    #[prop(default = "")]
    icon: &'static str,
) -> impl IntoView {
    let card_class = format!("split-card {}-card", variant);

    view! {
        <div class=card_class>
            <div class="split-card-header">
                {match icon {
                    "user" => view! {
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="split-icon">
                            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                            <circle cx="12" cy="7" r="4"/>
                        </svg>
                    }.into_any(),
                    "server" => view! {
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="split-icon">
                            <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
                            <line x1="8" y1="21" x2="16" y2="21"/>
                            <line x1="12" y1="17" x2="12" y2="21"/>
                        </svg>
                    }.into_any(),
                    "users" => view! {
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="split-icon">
                            <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                            <circle cx="8.5" cy="7" r="4"/>
                            <line x1="20" y1="8" x2="20" y2="14"/>
                            <line x1="23" y1="11" x2="17" y2="11"/>
                        </svg>
                    }.into_any(),
                    _ => view! { <></> }.into_any(),
                }}
                <span class="split-percentage">{percentage as u32}"%"</span>
            </div>
            <h4 class="split-card-title">{move || t(title_key)}</h4>
            {show_amount.then(|| amount.map(|a| view! {
                <p class="split-amount">"$"{format!("{:.2}", a)}</p>
            }))}
        </div>
    }
}
