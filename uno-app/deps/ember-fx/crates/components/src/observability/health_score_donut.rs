//! HealthScoreDonut Leptos component.
//!
//! A donut chart with center score and breakdown legend, plus optional header stats.

use leptos::prelude::*;
use crate::try_use_theme;

/// Health level for a segment.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HealthLevel {
    /// Poor/Critical (red).
    Poor,
    /// Average/Warning (yellow/orange).
    Average,
    /// Good/Healthy (green).
    #[default]
    Good,
}

impl HealthLevel {
    /// Returns the CSS color for this level.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Poor => "#ef4444",
            Self::Average => "#f59e0b",
            Self::Good => "#22c55e",
        }
    }

    /// Returns the label for this level.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Poor => "Poor Health",
            Self::Average => "Avg. Health",
            Self::Good => "Good Health",
        }
    }
}

/// A segment in the health donut chart.
#[derive(Debug, Clone)]
pub struct HealthSegment {
    /// Health level.
    pub level: HealthLevel,
    /// Value (e.g., dollar amount).
    pub value: f64,
    /// Count (e.g., number of accounts).
    pub count: u32,
}

impl HealthSegment {
    /// Create a new health segment.
    pub fn new(level: HealthLevel, value: f64, count: u32) -> Self {
        Self { level, value, count }
    }
}

/// A trend indicator stat.
#[derive(Debug, Clone)]
pub struct TrendStat {
    /// Stat label.
    pub label: String,
    /// Value (number).
    pub value: i32,
    /// Is positive trend (up = good).
    pub is_positive: bool,
    /// Time period label.
    pub period: String,
}

impl TrendStat {
    /// Create a new trend stat.
    pub fn new(label: impl Into<String>, value: i32, is_positive: bool, period: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value,
            is_positive,
            period: period.into(),
        }
    }
}

/// Configuration for the health score donut.
#[derive(Debug, Clone)]
pub struct HealthDonutConfig {
    /// Donut size (diameter).
    pub size: u32,
    /// Donut stroke width.
    pub stroke_width: f64,
    /// Value prefix (e.g., "$").
    pub value_prefix: String,
    /// Value suffix (e.g., "M").
    pub value_suffix: String,
}

impl Default for HealthDonutConfig {
    fn default() -> Self {
        Self {
            size: 200,
            stroke_width: 24.0,
            value_prefix: "$".to_string(),
            value_suffix: "M".to_string(),
        }
    }
}

/// HealthScoreDonut component.
///
/// A donut chart showing health breakdown with center score and legend.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{
///     HealthScoreDonut, HealthSegment, HealthLevel, TrendStat,
/// };
///
/// let segments = vec![
///     HealthSegment::new(HealthLevel::Poor, 84.52, 16),
///     HealthSegment::new(HealthLevel::Average, 22.32, 8),
///     HealthSegment::new(HealthLevel::Good, 19.17, 32),
/// ];
///
/// let trends = vec![
///     TrendStat::new("HEALTH IMPROVED", 25, true, "LAST 14 DAYS"),
///     TrendStat::new("HEALTH DECLINED", 15, false, "LAST 14 DAYS"),
/// ];
///
/// view! {
///     <HealthScoreDonut
///         score=Signal::derive(move || 94)
///         score_label="QUALITY SCORE".to_string()
///         segments=Signal::derive(move || segments.clone())
///         header_stats=vec![
///             ("PORTFOLIO VALUE".to_string(), "$15.99M".to_string()),
///             ("ACCOUNTS".to_string(), "132".to_string()),
///         ]
///         trends=trends
///     />
/// }
/// ```
#[component]
pub fn HealthScoreDonut(
    /// Center score value (0-100).
    #[prop(into)]
    score: Signal<u32>,
    /// Score label.
    #[prop(optional, into)]
    score_label: Option<String>,
    /// Health segments.
    #[prop(into)]
    segments: Signal<Vec<HealthSegment>>,
    /// Header stats (label, value pairs).
    #[prop(optional, into)]
    header_stats: Option<Vec<(String, String)>>,
    /// Trend stats.
    #[prop(optional, into)]
    trends: Option<Vec<TrendStat>>,
    /// Configuration.
    #[prop(optional)]
    config: Option<HealthDonutConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let size = config.size;
    let stroke_width = config.stroke_width;
    let value_prefix = config.value_prefix;
    let value_suffix = config.value_suffix;

    let score_label = score_label.unwrap_or_else(|| "SCORE".to_string());

    let prefix = format!("fx-health-donut-{}", design_system);

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![prefix.clone()];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // SVG dimensions
    let svg_size = size as f64;
    let center = svg_size / 2.0;
    let radius = (svg_size / 2.0) - stroke_width / 2.0 - 5.0;
    let _circumference = 2.0 * std::f64::consts::PI * radius;

    // Calculate segment paths
    let segment_paths = move || {
        let segs = segments.get();
        let total: f64 = segs.iter().map(|s| s.value).sum();
        if total <= 0.0 {
            return Vec::new();
        }

        let mut start_angle = -90.0f64; // Start from top
        let mut paths = Vec::new();

        for seg in &segs {
            let pct = seg.value / total;
            let angle_span = pct * 360.0;

            if angle_span <= 0.0 {
                continue;
            }

            let start_rad = start_angle.to_radians();
            let end_angle = start_angle + angle_span;
            let end_rad = end_angle.to_radians();

            let start_x = center + radius * start_rad.cos();
            let start_y = center + radius * start_rad.sin();
            let end_x = center + radius * end_rad.cos();
            let end_y = center + radius * end_rad.sin();

            let large_arc = if angle_span > 180.0 { 1 } else { 0 };

            let path = if angle_span >= 360.0 {
                // Full circle
                let mid_rad = start_rad + std::f64::consts::PI;
                let mid_x = center + radius * mid_rad.cos();
                let mid_y = center + radius * mid_rad.sin();
                format!(
                    "M {:.1} {:.1} A {:.1} {:.1} 0 1 1 {:.1} {:.1} A {:.1} {:.1} 0 1 1 {:.1} {:.1}",
                    start_x, start_y, radius, radius, mid_x, mid_y, radius, radius, start_x, start_y
                )
            } else {
                format!(
                    "M {:.1} {:.1} A {:.1} {:.1} 0 {} 1 {:.1} {:.1}",
                    start_x, start_y, radius, radius, large_arc, end_x, end_y
                )
            };

            paths.push((path, seg.level));
            start_angle = end_angle;
        }

        paths
    };

    // Clone for use in views
    let value_prefix_clone = value_prefix.clone();
    let value_suffix_clone = value_suffix.clone();

    view! {
        <div
            class=combined_class
            style="display: flex; flex-direction: column; align-items: center; background: #fff; padding: 24px; border-radius: 12px; color: #1f2937;"
        >
            // Header stats
            {header_stats.clone().map(|stats| {
                view! {
                    <div style="display: flex; justify-content: space-around; width: 100%; margin-bottom: 24px; gap: 32px;">
                        {stats.into_iter().map(|(label, value)| {
                            view! {
                                <div style="text-align: center;">
                                    <div style="font-size: 12px; color: #6b7280; font-weight: 500; margin-bottom: 4px;">
                                        {label}
                                    </div>
                                    <div style="font-size: 28px; font-weight: 700; color: #1f2937;">
                                        {value}
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }
            })}

            // Legend
            <div style="display: flex; flex-direction: column; gap: 8px; margin-bottom: 20px;">
                {move || {
                    segments.get().into_iter().map(|seg| {
                        let value_prefix = value_prefix_clone.clone();
                        let value_suffix = value_suffix_clone.clone();
                        view! {
                            <div style="display: flex; align-items: center; gap: 8px;">
                                <span style="font-size: 14px; color: #374151; min-width: 90px;">
                                    {seg.level.label()}
                                </span>
                                <span
                                    style=format!("width: 8px; height: 8px; border-radius: 50%; background: {};", seg.level.as_css())
                                />
                                <span style=format!("font-size: 14px; color: {}; font-weight: 500; min-width: 70px;", seg.level.as_css())>
                                    {format!("{}{:.2}{}", value_prefix, seg.value, value_suffix)}
                                </span>
                                <span style="font-size: 14px; color: #6b7280;">
                                    {format!("| {}", seg.count)}
                                </span>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>

            // Donut chart
            <div style="position: relative; margin-bottom: 24px;">
                <svg
                    width=format!("{}", size)
                    height=format!("{}", size)
                    viewBox=format!("0 0 {} {}", svg_size, svg_size)
                >
                    // Background track
                    <circle
                        cx=format!("{:.1}", center)
                        cy=format!("{:.1}", center)
                        r=format!("{:.1}", radius)
                        fill="none"
                        stroke="#f3f4f6"
                        stroke-width=format!("{}", stroke_width)
                    />

                    // Segments
                    {move || {
                        segment_paths().into_iter().map(|(path, level)| {
                            view! {
                                <path
                                    d=path
                                    fill="none"
                                    stroke=level.as_css()
                                    stroke-width=format!("{}", stroke_width)
                                    stroke-linecap="butt"
                                />
                            }
                        }).collect_view()
                    }}
                </svg>

                // Center score
                <div style=format!(
                    "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); text-align: center;"
                )>
                    <div style="font-size: 11px; color: #6b7280; font-weight: 500; letter-spacing: 0.5px;">
                        {score_label.clone()}
                    </div>
                    <div style="font-size: 48px; font-weight: 700; color: #22c55e; line-height: 1;">
                        {move || score.get()}
                    </div>
                </div>
            </div>

            // Trend stats
            {trends.clone().map(|trend_stats| {
                view! {
                    <div style="display: flex; justify-content: space-around; width: 100%; border-top: 1px solid #e5e7eb; padding-top: 20px; gap: 32px;">
                        {trend_stats.into_iter().map(|stat| {
                            let arrow_color = if stat.is_positive { "#22c55e" } else { "#ef4444" };
                            let arrow = if stat.is_positive { "▲" } else { "▼" };
                            view! {
                                <div style="text-align: center;">
                                    <div style="font-size: 11px; color: #6b7280; font-weight: 500; margin-bottom: 8px;">
                                        {stat.label}
                                    </div>
                                    <div style="display: flex; align-items: center; justify-content: center; gap: 4px;">
                                        <span style=format!("color: {}; font-size: 14px;", arrow_color)>
                                            {arrow}
                                        </span>
                                        <span style="font-size: 32px; font-weight: 700; color: #1f2937;">
                                            {stat.value}
                                        </span>
                                    </div>
                                    <div style="font-size: 10px; color: #9ca3af; margin-top: 4px;">
                                        {stat.period}
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }
            })}
        </div>
    }
}

/// A compact version of the health donut for use in cards.
#[component]
pub fn CompactHealthDonut(
    /// Center score value (0-100).
    #[prop(into)]
    score: Signal<u32>,
    /// Health segments.
    #[prop(into)]
    segments: Signal<Vec<HealthSegment>>,
    /// Donut size.
    #[prop(optional)]
    size: Option<u32>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let size = size.unwrap_or(80);
    let stroke_width = 8.0;

    let prefix = format!("fx-compact-health-{}", design_system);

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![prefix.clone()];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // SVG dimensions
    let svg_size = size as f64;
    let center = svg_size / 2.0;
    let radius = (svg_size / 2.0) - stroke_width / 2.0 - 2.0;

    // Calculate segment paths
    let segment_paths = move || {
        let segs = segments.get();
        let total: f64 = segs.iter().map(|s| s.value).sum();
        if total <= 0.0 {
            return Vec::new();
        }

        let mut start_angle = -90.0f64;
        let mut paths = Vec::new();

        for seg in &segs {
            let pct = seg.value / total;
            let angle_span = pct * 360.0;

            if angle_span <= 0.0 {
                continue;
            }

            let start_rad = start_angle.to_radians();
            let end_angle = start_angle + angle_span;
            let end_rad = end_angle.to_radians();

            let start_x = center + radius * start_rad.cos();
            let start_y = center + radius * start_rad.sin();
            let end_x = center + radius * end_rad.cos();
            let end_y = center + radius * end_rad.sin();

            let large_arc = if angle_span > 180.0 { 1 } else { 0 };

            let path = format!(
                "M {:.1} {:.1} A {:.1} {:.1} 0 {} 1 {:.1} {:.1}",
                start_x, start_y, radius, radius, large_arc, end_x, end_y
            );

            paths.push((path, seg.level));
            start_angle = end_angle;
        }

        paths
    };

    view! {
        <div class=combined_class style="position: relative; display: inline-block;">
            <svg
                width=format!("{}", size)
                height=format!("{}", size)
                viewBox=format!("0 0 {} {}", svg_size, svg_size)
            >
                // Background track
                <circle
                    cx=format!("{:.1}", center)
                    cy=format!("{:.1}", center)
                    r=format!("{:.1}", radius)
                    fill="none"
                    stroke="#f3f4f6"
                    stroke-width=format!("{}", stroke_width)
                />

                // Segments
                {move || {
                    segment_paths().into_iter().map(|(path, level)| {
                        view! {
                            <path
                                d=path
                                fill="none"
                                stroke=level.as_css()
                                stroke-width=format!("{}", stroke_width)
                                stroke-linecap="butt"
                            />
                        }
                    }).collect_view()
                }}
            </svg>

            // Center score
            <div style=format!(
                "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); font-size: 20px; font-weight: 700; color: #1f2937;"
            )>
                {move || score.get()}
            </div>
        </div>
    }
}
