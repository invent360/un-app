//! ReliabilityFactorGauge Leptos component.
//!
//! A gauge showing reliability factor (0.0-1.0 multiplier style).

use leptos::prelude::*;
use crate::try_use_theme;

/// Reliability rating classification.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReliabilityRating {
    /// Excellent reliability (>= 0.95).
    Excellent,
    /// Good reliability (>= 0.85).
    #[default]
    Good,
    /// Fair reliability (>= 0.70).
    Fair,
    /// Poor reliability (< 0.70).
    Poor,
}

impl ReliabilityRating {
    /// Determine rating from factor (0.0-1.0).
    pub fn from_factor(value: f64) -> Self {
        if value >= 0.95 {
            Self::Excellent
        } else if value >= 0.85 {
            Self::Good
        } else if value >= 0.70 {
            Self::Fair
        } else {
            Self::Poor
        }
    }

    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Excellent => "excellent",
            Self::Good => "good",
            Self::Fair => "fair",
            Self::Poor => "poor",
        }
    }

    /// Returns the color for this rating.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Excellent => "var(--fx-color-success, #52c41a)",
            Self::Good => "var(--fx-color-primary, #1677ff)",
            Self::Fair => "var(--fx-color-warning, #faad14)",
            Self::Poor => "var(--fx-color-error, #ff4d4f)",
        }
    }

    /// Returns the label for this rating.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Excellent => "Excellent",
            Self::Good => "Good",
            Self::Fair => "Fair",
            Self::Poor => "Poor",
        }
    }

    /// Returns the icon for this rating.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::Excellent => "★",
            Self::Good => "●",
            Self::Fair => "◐",
            Self::Poor => "○",
        }
    }
}

/// Size variants for the gauge.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReliabilityGaugeSize {
    /// Small size.
    Small,
    /// Default size.
    #[default]
    Default,
    /// Large size.
    Large,
}

impl ReliabilityGaugeSize {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "md",
            Self::Large => "lg",
        }
    }

    /// Returns the size in pixels.
    pub fn as_pixels(&self) -> u32 {
        match self {
            Self::Small => 100,
            Self::Default => 140,
            Self::Large => 180,
        }
    }
}

/// Trend direction for reliability.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReliabilityTrend {
    /// Improving.
    Up,
    /// Stable.
    #[default]
    Stable,
    /// Declining.
    Down,
}

impl ReliabilityTrend {
    /// Returns the icon for this trend.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::Up => "↑",
            Self::Stable => "→",
            Self::Down => "↓",
        }
    }

    /// Returns the color for this trend.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Up => "var(--fx-color-success, #52c41a)",
            Self::Stable => "var(--fx-color-text-secondary, #8c8c8c)",
            Self::Down => "var(--fx-color-error, #ff4d4f)",
        }
    }
}

/// ReliabilityFactorGauge component.
///
/// A gauge showing reliability factor with trend indication.
///
/// # Props
///
/// - `factor` - Reliability factor (0.0-1.0)
/// - `trend` - Optional trend direction
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{ReliabilityFactorGauge, ReliabilityTrend};
///
/// view! {
///     <ReliabilityFactorGauge
///         factor=Signal::derive(move || 0.92)
///         trend=ReliabilityTrend::Up
///     />
/// }
/// ```
#[component]
pub fn ReliabilityFactorGauge(
    /// Reliability factor (0.0-1.0).
    #[prop(into)]
    factor: Signal<f64>,
    /// Trend direction.
    #[prop(optional)]
    trend: Option<ReliabilityTrend>,
    /// Custom title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Size variant.
    #[prop(optional)]
    size: ReliabilityGaugeSize,
    /// Show rating label.
    #[prop(optional)]
    show_rating: Option<bool>,
    /// Show trend indicator.
    #[prop(optional)]
    show_trend: Option<bool>,
    /// Successful operations count.
    #[prop(optional, into)]
    successes: Option<Signal<u64>>,
    /// Failed operations count.
    #[prop(optional, into)]
    failures: Option<Signal<u64>>,
    /// Total operations count.
    #[prop(optional, into)]
    total_ops: Option<Signal<u64>>,
    /// Time period label (e.g., "24h", "7d").
    #[prop(optional, into)]
    period: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let title = title.unwrap_or_else(|| "Reliability Factor".to_string());
    let show_rating = show_rating.unwrap_or(true);
    let show_trend = show_trend.unwrap_or(trend.is_some());
    let gauge_size = size.as_pixels();

    let prefix = format!("fx-reliability-{}", design_system);

    // Clamp factor to 0.0-1.0
    let clamped_factor = move || factor.get().clamp(0.0, 1.0);
    let rating = move || ReliabilityRating::from_factor(clamped_factor());

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![
                prefix.clone(),
                format!("{}-{}", prefix, size.as_suffix()),
                format!("{}-{}", prefix, rating().as_suffix()),
            ];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // SVG gauge parameters
    let view_size: f64 = 100.0;
    let center: f64 = view_size / 2.0;
    let radius: f64 = 40.0;
    let stroke_width: f64 = 8.0;

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

    // Progress arc
    let progress_arc = move || {
        let val = clamped_factor() * 100.0;
        let value_angle = start_angle + (val / 100.0) * total_arc;
        if value_angle <= start_angle {
            String::new()
        } else {
            create_arc_path(center, radius, start_angle, value_angle)
        }
    };

    // Clone for closures
    let prefix_gauge = prefix.clone();
    let prefix_content = prefix.clone();
    let prefix_details = prefix.clone();

    view! {
        <div
            class=combined_class
            style=format!("width: {}px;", gauge_size)
        >
            // Title
            <div class=format!("{}-header", prefix)>
                <span class=format!("{}-title", prefix)>
                    {title.clone()}
                </span>
                {period.clone().map(|p| {
                    let prefix = prefix.clone();
                    view! {
                        <span class=format!("{}-period", prefix)>
                            {p}
                        </span>
                    }
                })}
            </div>

            // Gauge
            <div
                class=format!("{}-gauge-container", prefix_gauge)
                style=format!("width: {}px; height: {}px;", gauge_size, gauge_size)
            >
                <svg
                    viewBox=format!("0 0 {} {}", view_size, view_size)
                    class=format!("{}-gauge", prefix_gauge)
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
                        stroke=move || rating().as_color()
                        stroke-width=stroke_width
                        stroke-linecap="round"
                    />
                </svg>

                // Center content
                <div class=format!("{}-gauge-content", prefix_content)>
                    <div class=format!("{}-gauge-value-row", prefix_content)>
                        <span
                            class=format!("{}-gauge-value", prefix_content)
                            style=move || format!("color: {};", rating().as_color())
                        >
                            {move || format!("{:.2}", clamped_factor())}
                        </span>
                        {(show_trend && trend.is_some()).then(|| {
                            let t = trend.unwrap();
                            let prefix = prefix_content.clone();
                            view! {
                                <span
                                    class=format!("{}-gauge-trend", prefix)
                                    style=format!("color: {};", t.as_color())
                                >
                                    {t.as_icon()}
                                </span>
                            }
                        })}
                    </div>
                    {show_rating.then(|| {
                        let prefix = prefix_content.clone();
                        view! {
                            <span
                                class=format!("{}-gauge-rating", prefix)
                                style=move || format!("color: {};", rating().as_color())
                            >
                                {move || rating().as_label()}
                            </span>
                        }
                    })}
                </div>
            </div>

            // Stats
            <div class=format!("{}-stats", prefix_details)>
                {successes.map(|s| {
                    let prefix = prefix_details.clone();
                    view! {
                        <div class=format!("{}-stat", prefix)>
                            <span class=format!("{}-stat-value {}-stat-success", prefix, prefix)>
                                {move || s.get()}
                            </span>
                            <span class=format!("{}-stat-label", prefix)>"Success"</span>
                        </div>
                    }
                })}
                {failures.map(|f| {
                    let prefix = prefix_details.clone();
                    view! {
                        <div class=format!("{}-stat", prefix)>
                            <span class=format!("{}-stat-value {}-stat-failure", prefix, prefix)>
                                {move || f.get()}
                            </span>
                            <span class=format!("{}-stat-label", prefix)>"Failed"</span>
                        </div>
                    }
                })}
                {total_ops.map(|t| {
                    let prefix = prefix_details.clone();
                    view! {
                        <div class=format!("{}-stat", prefix)>
                            <span class=format!("{}-stat-value", prefix)>
                                {move || t.get()}
                            </span>
                            <span class=format!("{}-stat-label", prefix)>"Total"</span>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
