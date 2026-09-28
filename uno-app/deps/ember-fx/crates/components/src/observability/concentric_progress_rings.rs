//! ConcentricProgressRings Leptos component.
//!
//! A chart showing multiple categories as colored arc segments at different radii.

use leptos::prelude::*;
use crate::try_use_theme;

/// Color options for ring segments.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RingColor {
    #[default]
    Cyan,
    Orange,
    Purple,
    Green,
    Red,
    Blue,
    Yellow,
    Pink,
}

impl RingColor {
    /// Returns the CSS color string.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Cyan => "#22d3ee",
            Self::Orange => "#fb923c",
            Self::Purple => "#a855f7",
            Self::Green => "#4ade80",
            Self::Red => "#f87171",
            Self::Blue => "#60a5fa",
            Self::Yellow => "#facc15",
            Self::Pink => "#f472b6",
        }
    }
}

/// A ring segment representing a category.
#[derive(Debug, Clone)]
pub struct RingSegment {
    /// Segment label.
    pub label: String,
    /// Value (0.0 to 1.0 representing percentage of full circle).
    pub value: f64,
    /// Segment color.
    pub color: RingColor,
}

impl RingSegment {
    /// Create a new ring segment.
    pub fn new(label: impl Into<String>, value: f64, color: RingColor) -> Self {
        Self {
            label: label.into(),
            value: value.clamp(0.0, 1.0),
            color,
        }
    }
}

/// Configuration for the concentric progress rings.
#[derive(Debug, Clone)]
pub struct ConcentricRingsConfig {
    /// Chart size (width and height).
    pub size: u32,
    /// Ring stroke width.
    pub stroke_width: f64,
    /// Gap between rings.
    pub ring_gap: f64,
    /// Show background track for each ring.
    pub show_track: bool,
    /// Start angle in degrees (0 = top, 90 = right).
    pub start_angle: f64,
}

impl Default for ConcentricRingsConfig {
    fn default() -> Self {
        Self {
            size: 200,
            stroke_width: 12.0,
            ring_gap: 4.0,
            show_track: true,
            start_angle: -90.0, // Start from top
        }
    }
}

/// ConcentricProgressRings component.
///
/// Displays multiple categories as colored arc segments at different radii,
/// with each ring representing a different metric.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{
///     ConcentricProgressRings, RingSegment, RingColor,
/// };
///
/// let segments = vec![
///     RingSegment::new("Near Miss", 0.85, RingColor::Cyan),
///     RingSegment::new("First Aid", 0.65, RingColor::Orange),
///     RingSegment::new("Medical", 0.45, RingColor::Purple),
///     RingSegment::new("Lost Time", 0.25, RingColor::Green),
///     RingSegment::new("Restricted Work", 0.15, RingColor::Red),
/// ];
///
/// view! {
///     <ConcentricProgressRings
///         segments=Signal::derive(move || segments.clone())
///         title="Root Cause (Monthly)".to_string()
///     />
/// }
/// ```
#[component]
pub fn ConcentricProgressRings(
    /// Ring segments (outermost first).
    #[prop(into)]
    segments: Signal<Vec<RingSegment>>,
    /// Chart title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Configuration.
    #[prop(optional)]
    config: Option<ConcentricRingsConfig>,
    /// Show legend.
    #[prop(optional)]
    show_legend: Option<bool>,
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
    let ring_gap = config.ring_gap;
    let show_track = config.show_track;
    let start_angle = config.start_angle;
    let show_legend = show_legend.unwrap_or(true);

    let prefix = format!("fx-concentric-rings-{}", design_system);

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
    let max_radius = center - stroke_width / 2.0 - 5.0;

    // Generate ring paths
    let ring_paths = move || {
        let segs = segments.get();
        let num_rings = segs.len();
        if num_rings == 0 {
            return vec![];
        }

        let ring_space = stroke_width + ring_gap;

        segs.iter()
            .enumerate()
            .map(|(i, seg)| {
                // Outermost ring is first in array (index 0)
                let radius = max_radius - (i as f64 * ring_space);
                if radius <= 0.0 {
                    return (String::new(), String::new(), seg.color, seg.label.clone());
                }

                let _circumference = 2.0 * std::f64::consts::PI * radius;

                // Calculate arc path
                let start_rad = start_angle.to_radians();
                let end_angle_deg = start_angle + (seg.value * 360.0);
                let end_rad = end_angle_deg.to_radians();

                let start_x = center + radius * start_rad.cos();
                let start_y = center + radius * start_rad.sin();
                let end_x = center + radius * end_rad.cos();
                let end_y = center + radius * end_rad.sin();

                let large_arc = if seg.value > 0.5 { 1 } else { 0 };

                let arc_path = if seg.value <= 0.0 {
                    String::new()
                } else if seg.value >= 1.0 {
                    // Full circle needs two arcs
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

                // Track path (full circle)
                let track_path = format!(
                    "M {:.1} {:.1} A {:.1} {:.1} 0 1 1 {:.1} {:.1} A {:.1} {:.1} 0 1 1 {:.1} {:.1}",
                    center + radius, center,
                    radius, radius,
                    center - radius, center,
                    radius, radius,
                    center + radius, center
                );

                (arc_path, track_path, seg.color, seg.label.clone())
            })
            .collect::<Vec<_>>()
    };

    // Legend items
    let legend_items = move || {
        segments.get()
            .iter()
            .map(|seg| (seg.label.clone(), seg.color))
            .collect::<Vec<_>>()
    };

    view! {
        <div class=combined_class>
            // Title
            {title.clone().map(|t| {
                let prefix = prefix.clone();
                view! {
                    <div class=format!("{}-title", prefix) style="font-size: 14px; font-weight: 500; margin-bottom: 12px; color: var(--fx-color-text, #fff);">
                        {t}
                    </div>
                }
            })}

            <div class=format!("{}-content", prefix) style="display: flex; align-items: center; gap: 16px;">
                // Chart SVG
                <svg
                    class=format!("{}-chart", prefix)
                    width=format!("{}", size)
                    height=format!("{}", size)
                    viewBox=format!("0 0 {} {}", svg_size, svg_size)
                >
                    // Background circle guides (optional decorative)
                    <circle
                        cx=format!("{:.1}", center)
                        cy=format!("{:.1}", center)
                        r=format!("{:.1}", max_radius + stroke_width)
                        fill="none"
                        stroke="var(--fx-color-border, #e5e7eb)"
                        stroke-width="1"
                        opacity="0.2"
                    />

                    // Ring tracks and segments
                    {move || {
                        ring_paths().into_iter().map(|(arc_path, track_path, color, _label)| {
                            view! {
                                <g>
                                    // Track (background)
                                    {show_track.then(|| view! {
                                        <path
                                            d=track_path.clone()
                                            fill="none"
                                            stroke="var(--fx-color-border, #e5e7eb)"
                                            stroke-width=format!("{}", stroke_width)
                                            opacity="0.2"
                                        />
                                    })}
                                    // Value arc
                                    {(!arc_path.is_empty()).then(|| view! {
                                        <path
                                            d=arc_path.clone()
                                            fill="none"
                                            stroke=color.as_css()
                                            stroke-width=format!("{}", stroke_width)
                                            stroke-linecap="round"
                                        />
                                    })}
                                </g>
                            }
                        }).collect_view()
                    }}
                </svg>

                // Legend
                {show_legend.then(|| {
                    let prefix = prefix.clone();
                    view! {
                        <div class=format!("{}-legend", prefix) style="display: flex; flex-direction: column; gap: 8px;">
                            {move || {
                                legend_items().into_iter().map(|(label, color)| {
                                    view! {
                                        <div class="legend-item" style="display: flex; align-items: center; gap: 8px;">
                                            <div
                                                class="legend-color"
                                                style=format!("width: 16px; height: 16px; border-radius: 3px; background-color: {};", color.as_css())
                                            />
                                            <span style="font-size: 13px; color: var(--fx-color-text, #374151);">
                                                {label}
                                            </span>
                                        </div>
                                    }
                                }).collect_view()
                            }}
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
