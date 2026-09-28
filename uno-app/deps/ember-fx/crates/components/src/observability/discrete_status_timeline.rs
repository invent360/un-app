//! DiscreteStatusTimeline Leptos component.
//!
//! A horizontal bar showing discrete colored segments representing status over time.

use leptos::prelude::*;
use crate::try_use_theme;

/// Status level for a segment.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TimelineStatus {
    /// Normal/OK (green).
    #[default]
    Normal,
    /// Warning (yellow/orange).
    Warning,
    /// Critical/Error (red).
    Critical,
    /// Cold/Low (blue).
    Cold,
    /// Unknown/No data (gray).
    Unknown,
}

impl TimelineStatus {
    /// Returns the CSS color for this status.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Normal => "#22c55e",
            Self::Warning => "#f59e0b",
            Self::Critical => "#ef4444",
            Self::Cold => "#3b82f6",
            Self::Unknown => "#6b7280",
        }
    }
}

/// A single row in the discrete status timeline.
#[derive(Debug, Clone)]
pub struct TimelineRow {
    /// Row label.
    pub label: String,
    /// Status segments (each represents a time bucket).
    pub segments: Vec<TimelineStatus>,
    /// Current value to display.
    pub value: String,
    /// Unit suffix (e.g., "F", "%").
    pub unit: String,
}

impl TimelineRow {
    /// Create a new timeline row.
    pub fn new(
        label: impl Into<String>,
        segments: Vec<TimelineStatus>,
        value: impl Into<String>,
        unit: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            segments,
            value: value.into(),
            unit: unit.into(),
        }
    }
}

/// Configuration for the discrete status timeline.
#[derive(Debug, Clone)]
pub struct DiscreteTimelineConfig {
    /// Segment width in pixels.
    pub segment_width: u32,
    /// Segment height in pixels.
    pub segment_height: u32,
    /// Gap between segments in pixels.
    pub segment_gap: u32,
    /// Row spacing in pixels.
    pub row_spacing: u32,
}

impl Default for DiscreteTimelineConfig {
    fn default() -> Self {
        Self {
            segment_width: 8,
            segment_height: 20,
            segment_gap: 2,
            row_spacing: 16,
        }
    }
}

/// DiscreteStatusTimeline component.
///
/// Displays multiple rows of discrete colored segments showing status over time.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{
///     DiscreteStatusTimeline, TimelineRow, TimelineStatus,
/// };
///
/// let rows = vec![
///     TimelineRow::new(
///         "Supply Air Dewpoint",
///         vec![TimelineStatus::Cold; 10]
///             .into_iter()
///             .chain(vec![TimelineStatus::Normal; 40])
///             .chain(vec![TimelineStatus::Critical; 10])
///             .collect(),
///         "58.4",
///         "°F",
///     ),
/// ];
///
/// view! {
///     <DiscreteStatusTimeline
///         rows=Signal::derive(move || rows.clone())
///     />
/// }
/// ```
#[component]
pub fn DiscreteStatusTimeline(
    /// Timeline rows.
    #[prop(into)]
    rows: Signal<Vec<TimelineRow>>,
    /// Configuration.
    #[prop(optional)]
    config: Option<DiscreteTimelineConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let segment_width = config.segment_width;
    let segment_height = config.segment_height;
    let _segment_gap = config.segment_gap;
    let row_spacing = config.row_spacing;

    let prefix = format!("fx-discrete-timeline-{}", design_system);

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

    view! {
        <div
            class=combined_class
            style="display: flex; flex-direction: column; background: var(--fx-color-bg, #1a1a1a); padding: 16px;"
        >
            {move || {
                rows.get().into_iter().map(|row| {
                    let prefix = prefix.clone();
                    view! {
                        <div
                            class=format!("{}-row", prefix)
                            style=format!("display: flex; align-items: center; gap: 16px; margin-bottom: {}px;", row_spacing)
                        >
                            // Label
                            <div
                                class=format!("{}-label", prefix)
                                style="min-width: 160px; font-size: 14px; color: var(--fx-color-text-secondary, #9ca3af);"
                            >
                                {row.label.clone()}
                            </div>

                            // Segments
                            <div
                                class=format!("{}-segments", prefix)
                                style="display: flex; flex: 1; gap: 1px;"
                            >
                                {row.segments.iter().map(|status| {
                                    view! {
                                        <div
                                            style=format!(
                                                "width: {}px; height: {}px; background: {}; border-radius: 1px;",
                                                segment_width, segment_height, status.as_css()
                                            )
                                        />
                                    }
                                }).collect_view()}
                            </div>

                            // Value
                            <div
                                class=format!("{}-value", prefix)
                                style="min-width: 80px; text-align: right; font-size: 20px; font-weight: 500; color: var(--fx-color-text, #fff);"
                            >
                                <span>{row.value.clone()}</span>
                                <span style="font-size: 14px; color: var(--fx-color-text-secondary, #9ca3af); margin-left: 2px;">{row.unit.clone()}</span>
                            </div>
                        </div>
                    }
                }).collect_view()
            }}
        </div>
    }
}

/// A single metric card with discrete timeline and sparkline.
#[component]
pub fn MetricTimelineCard(
    /// Metric label.
    #[prop(into)]
    label: String,
    /// Current value.
    #[prop(into)]
    value: Signal<f64>,
    /// Unit suffix.
    #[prop(optional, into)]
    unit: Option<String>,
    /// Sparkline data.
    #[prop(optional, into)]
    sparkline: Option<Signal<Vec<f64>>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let unit = unit.unwrap_or_default();
    let prefix = format!("fx-metric-timeline-{}", design_system);

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

    // Generate sparkline path
    let sparkline_path = move || {
        let sparkline = sparkline.as_ref()?;
        let points = sparkline.get();
        if points.is_empty() {
            return None;
        }

        let width = 120.0f64;
        let height = 40.0f64;
        let padding = 2.0f64;

        let min_val = points.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = points.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let val_range = (max_val - min_val).max(0.001);

        let x_step = (width - 2.0 * padding) / (points.len() as f64 - 1.0).max(1.0);

        let mut line_path = String::new();
        let mut fill_path = String::new();

        fill_path.push_str(&format!("M {:.1} {:.1}", padding, height - padding));

        for (i, &val) in points.iter().enumerate() {
            let x = padding + i as f64 * x_step;
            let y = padding + (height - 2.0 * padding) - ((val - min_val) / val_range) * (height - 2.0 * padding);

            if i == 0 {
                line_path.push_str(&format!("M {:.1} {:.1}", x, y));
                fill_path.push_str(&format!(" L {:.1} {:.1}", x, y));
            } else {
                line_path.push_str(&format!(" L {:.1} {:.1}", x, y));
                fill_path.push_str(&format!(" L {:.1} {:.1}", x, y));
            }
        }

        let last_x = padding + (points.len() as f64 - 1.0) * x_step;
        fill_path.push_str(&format!(" L {:.1} {:.1} Z", last_x, height - padding));

        Some((line_path, fill_path, width, height))
    };

    view! {
        <div
            class=combined_class
            style="display: flex; flex-direction: column; background: var(--fx-color-bg, #1a1a1a); padding: 12px 16px; min-width: 180px;"
        >
            // Label
            <div style="font-size: 13px; color: var(--fx-color-text-secondary, #9ca3af); margin-bottom: 4px;">
                {label}
            </div>

            // Value
            <div style="font-size: 28px; font-weight: 500; color: #4ade80; margin-bottom: 8px;">
                {move || format!("{:.1}", value.get())}
                <span style="font-size: 16px; margin-left: 2px;">{unit.clone()}</span>
            </div>

            // Sparkline
            {move || {
                sparkline_path().map(|(line_path, fill_path, width, height)| {
                    view! {
                        <svg
                            width="100%"
                            height=format!("{:.0}", height)
                            viewBox=format!("0 0 {} {}", width, height)
                            preserveAspectRatio="none"
                        >
                            <path
                                d=fill_path
                                fill="rgba(74, 222, 128, 0.2)"
                            />
                            <path
                                d=line_path
                                fill="none"
                                stroke="#4ade80"
                                stroke-width="1.5"
                            />
                        </svg>
                    }
                })
            }}
        </div>
    }
}
