//! SlashingRiskGauge Leptos component.
//!
//! A gauge showing slashing/penalty risk level.

use leptos::prelude::*;
use crate::try_use_theme;

/// Risk level classification.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RiskLevel {
    /// Low risk (safe).
    #[default]
    Low,
    /// Medium risk (caution).
    Medium,
    /// High risk (danger).
    High,
    /// Critical risk (immediate action needed).
    Critical,
}

impl RiskLevel {
    /// Determine level from risk percentage.
    pub fn from_percentage(value: f64) -> Self {
        if value >= 75.0 {
            Self::Critical
        } else if value >= 50.0 {
            Self::High
        } else if value >= 25.0 {
            Self::Medium
        } else {
            Self::Low
        }
    }

    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    /// Returns the color for this level.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Low => "var(--fx-color-success, #52c41a)",
            Self::Medium => "var(--fx-color-warning, #faad14)",
            Self::High => "var(--fx-color-orange, #fa8c16)",
            Self::Critical => "var(--fx-color-error, #ff4d4f)",
        }
    }

    /// Returns the label for this level.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Low => "Low Risk",
            Self::Medium => "Medium Risk",
            Self::High => "High Risk",
            Self::Critical => "Critical Risk",
        }
    }

    /// Returns the icon for this level.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::Low => "✓",
            Self::Medium => "⚠",
            Self::High => "⚠",
            Self::Critical => "⛔",
        }
    }
}

/// SlashingRiskGauge component.
///
/// Displays slashing risk with inverted color scale (higher = worse).
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::SlashingRiskGauge;
///
/// view! {
///     <SlashingRiskGauge
///         risk=Signal::derive(move || 15.0)
///     />
/// }
/// ```
#[component]
pub fn SlashingRiskGauge(
    /// Risk percentage (0-100).
    #[prop(into)]
    risk: Signal<f64>,
    /// Recent violations count.
    #[prop(optional, into)]
    violations: Option<Signal<u32>>,
    /// Days since last violation.
    #[prop(optional, into)]
    days_clean: Option<Signal<u32>>,
    /// Potential slash amount.
    #[prop(optional, into)]
    potential_slash: Option<Signal<f64>>,
    /// Token symbol.
    #[prop(optional, into)]
    token_symbol: Option<String>,
    /// Card title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Show details.
    #[prop(optional)]
    show_details: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let title = title.unwrap_or_else(|| "Slashing Risk".to_string());
    let token_symbol = token_symbol.unwrap_or_else(|| "EMB".to_string());
    let show_details = show_details.unwrap_or(true);

    let prefix = format!("fx-slashing-risk-{}", design_system);

    let level = move || RiskLevel::from_percentage(risk.get());

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![
                prefix.clone(),
                format!("{}-{}", prefix, level().as_suffix()),
            ];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // SVG gauge parameters
    let view_size: f64 = 120.0;
    let center: f64 = view_size / 2.0;
    let radius: f64 = 45.0;
    let stroke_width: f64 = 10.0;

    // Arc from 135° to 405° (270° total)
    let start_angle: f64 = 135.0;
    let end_angle: f64 = 405.0;
    let total_arc: f64 = end_angle - start_angle;

    // Helper function to create arc path
    fn create_arc_path(center: f64, radius: f64, start: f64, end: f64) -> String {
        let angle_to_point = |angle: f64| -> (f64, f64) {
            let rad = angle.to_radians();
            (center + radius * rad.cos(), center + radius * rad.sin())
        };
        let (x1, y1) = angle_to_point(start);
        let (x2, y2) = angle_to_point(end);
        let large_arc = if (end - start).abs() > 180.0 { 1 } else { 0 };
        format!(
            "M {} {} A {} {} 0 {} 1 {} {}",
            x1, y1, radius, radius, large_arc, x2, y2
        )
    }

    // Background arc
    let bg_arc = create_arc_path(center, radius, start_angle, end_angle);

    // Progress arc based on risk
    let progress_arc = move || {
        let val = risk.get().clamp(0.0, 100.0);
        let value_angle = start_angle + (val / 100.0) * total_arc;
        create_arc_path(center, radius, start_angle, value_angle)
    };

    view! {
        <div class=combined_class>
            // Title
            <div class=format!("{}-title", prefix)>
                {title}
            </div>

            // Gauge
            <div class=format!("{}-gauge-container", prefix)>
                <svg
                    viewBox=format!("0 0 {} {}", view_size, view_size)
                    class=format!("{}-gauge", prefix)
                >
                    // Background
                    <path
                        d=bg_arc.clone()
                        fill="none"
                        stroke="var(--fx-color-border, #303030)"
                        stroke-width=stroke_width
                        stroke-linecap="round"
                    />
                    // Progress
                    <path
                        d=progress_arc
                        fill="none"
                        stroke=move || level().as_color()
                        stroke-width=stroke_width
                        stroke-linecap="round"
                    />
                </svg>

                // Center content
                <div class=format!("{}-gauge-content", prefix)>
                    <span
                        class=format!("{}-gauge-value", prefix)
                        style=move || format!("color: {};", level().as_color())
                    >
                        {move || format!("{:.0}%", risk.get())}
                    </span>
                    <span
                        class=format!("{}-gauge-label", prefix)
                        style=move || format!("color: {};", level().as_color())
                    >
                        {move || level().as_label()}
                    </span>
                </div>
            </div>

            // Details
            {move || {
                if show_details {
                    Some(view! {
                        <div class=format!("{}-details", prefix)>
                            {violations.map(|v| {
                                view! {
                                    <div class=format!("{}-detail-row", prefix)>
                                        <span class=format!("{}-detail-label", prefix)>
                                            "Violations"
                                        </span>
                                        <span class=format!("{}-detail-value", prefix)>
                                            {move || v.get()}
                                        </span>
                                    </div>
                                }
                            })}
                            {days_clean.map(|dc| {
                                view! {
                                    <div class=format!("{}-detail-row", prefix)>
                                        <span class=format!("{}-detail-label", prefix)>
                                            "Days clean"
                                        </span>
                                        <span
                                            class=format!("{}-detail-value {}-detail-clean", prefix, prefix)
                                        >
                                            {move || dc.get()}
                                        </span>
                                    </div>
                                }
                            })}
                            {potential_slash.map(|ps| {
                                let sym = token_symbol.clone();
                                view! {
                                    <div class=format!("{}-detail-row", prefix)>
                                        <span class=format!("{}-detail-label", prefix)>
                                            "At risk"
                                        </span>
                                        <span
                                            class=format!("{}-detail-value {}-detail-at-risk", prefix, prefix)
                                        >
                                            {move || format!("{:.2} {}", ps.get(), sym)}
                                        </span>
                                    </div>
                                }
                            })}
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
