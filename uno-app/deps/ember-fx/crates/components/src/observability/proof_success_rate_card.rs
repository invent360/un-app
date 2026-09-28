//! ProofSuccessRateCard Leptos component.
//!
//! A StatCard variant showing proof submission success rate.

use leptos::prelude::*;
use crate::panel::StatCardSize;
use crate::try_use_theme;

/// ProofSuccessRateCard component.
///
/// Displays proof/verification success rate with a ring indicator.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::ProofSuccessRateCard;
///
/// view! {
///     <ProofSuccessRateCard
///         success_rate=Signal::derive(move || 98.5)
///         total_proofs=Signal::derive(move || 1250)
///     />
/// }
/// ```
#[component]
pub fn ProofSuccessRateCard(
    /// Success rate percentage (0-100).
    #[prop(into)]
    success_rate: Signal<f64>,
    /// Total proofs submitted.
    #[prop(optional, into)]
    total_proofs: Option<Signal<u64>>,
    /// Failed proofs count.
    #[prop(optional, into)]
    failed_proofs: Option<Signal<u64>>,
    /// Card title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Card size.
    #[prop(optional)]
    size: StatCardSize,
    /// Show history sparkline.
    #[prop(optional)]
    show_history: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let title = title.unwrap_or_else(|| "Proof Success Rate".to_string());
    let _show_history = show_history.unwrap_or(false);

    let card_prefix = format!("fx-statcard-{}", design_system);
    let prefix = format!("fx-proof-rate-{}", design_system);

    let rate_color = move || {
        let rate = success_rate.get();
        if rate >= 98.0 {
            "var(--fx-color-success, #52c41a)"
        } else if rate >= 95.0 {
            "var(--fx-color-warning, #faad14)"
        } else {
            "var(--fx-color-error, #ff4d4f)"
        }
    };

    let combined_class = {
        let card_prefix = card_prefix.clone();
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![
                card_prefix.clone(),
                format!("{}-{}", card_prefix, size.as_str()),
                prefix.clone(),
            ];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // SVG ring parameters
    let ring_size = 64.0;
    let stroke_width = 6.0;
    let radius = (ring_size - stroke_width) / 2.0;
    let circumference = 2.0 * std::f64::consts::PI * radius;

    let stroke_dashoffset = move || {
        let rate = success_rate.get().clamp(0.0, 100.0);
        circumference * (1.0 - rate / 100.0)
    };

    view! {
        <div class=combined_class>
            // Title
            <div class=format!("{}-title", card_prefix)>
                {title}
            </div>

            // Ring indicator
            <div class=format!("{}-ring-container", prefix)>
                <svg
                    width=ring_size
                    height=ring_size
                    class=format!("{}-ring", prefix)
                >
                    // Background circle
                    <circle
                        cx=ring_size / 2.0
                        cy=ring_size / 2.0
                        r=radius
                        fill="none"
                        stroke="var(--fx-color-border, #303030)"
                        stroke-width=stroke_width
                    />
                    // Progress circle
                    <circle
                        cx=ring_size / 2.0
                        cy=ring_size / 2.0
                        r=radius
                        fill="none"
                        stroke=rate_color
                        stroke-width=stroke_width
                        stroke-linecap="round"
                        stroke-dasharray=circumference
                        stroke-dashoffset=stroke_dashoffset
                        transform=format!("rotate(-90 {} {})", ring_size / 2.0, ring_size / 2.0)
                    />
                </svg>
                // Percentage in center
                <div
                    class=format!("{}-ring-value", prefix)
                    style=move || format!("color: {};", rate_color())
                >
                    {move || format!("{:.1}%", success_rate.get())}
                </div>
            </div>

            // Stats
            <div class=format!("{}-stats", prefix)>
                {total_proofs.map(|tp| {
                    view! {
                        <div class=format!("{}-stat", prefix)>
                            <span class=format!("{}-stat-label", prefix)>"Total"</span>
                            <span class=format!("{}-stat-value", prefix)>
                                {move || format!("{}", tp.get())}
                            </span>
                        </div>
                    }
                })}
                {failed_proofs.map(|fp| {
                    view! {
                        <div class=format!("{}-stat", prefix)>
                            <span class=format!("{}-stat-label", prefix)>"Failed"</span>
                            <span
                                class=format!("{}-stat-value {}-stat-failed", prefix, prefix)
                            >
                                {move || format!("{}", fp.get())}
                            </span>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
