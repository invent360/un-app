//! GradientScoreGauge Leptos component.
//!
//! A semi-circular gauge with gradient coloring and score rating display.

use leptos::prelude::*;
use crate::try_use_theme;

/// Score rating level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreRating {
    /// Very Poor (0-20).
    VeryPoor,
    /// Poor (20-40).
    Poor,
    /// Fair (40-60).
    Fair,
    /// Good (60-80).
    Good,
    /// Excellent (80-100).
    Excellent,
}

impl ScoreRating {
    /// Determine rating from score.
    pub fn from_score(score: f64) -> Self {
        if score < 20.0 {
            Self::VeryPoor
        } else if score < 40.0 {
            Self::Poor
        } else if score < 60.0 {
            Self::Fair
        } else if score < 80.0 {
            Self::Good
        } else {
            Self::Excellent
        }
    }

    /// Get the display label.
    pub fn label(&self) -> &'static str {
        match self {
            Self::VeryPoor => "VERY POOR",
            Self::Poor => "POOR",
            Self::Fair => "FAIR",
            Self::Good => "GOOD",
            Self::Excellent => "EXCELLENT",
        }
    }

    /// Get the color for this rating.
    pub fn color(&self) -> &'static str {
        match self {
            Self::VeryPoor => "#dc2626",
            Self::Poor => "#ea580c",
            Self::Fair => "#d97706",
            Self::Good => "#65a30d",
            Self::Excellent => "#16a34a",
        }
    }
}

/// History data point for the score trend chart.
#[derive(Debug, Clone)]
pub struct ScoreHistoryPoint {
    /// Label (e.g., "Jan", "Feb").
    pub label: String,
    /// Score value.
    pub value: f64,
}

impl ScoreHistoryPoint {
    /// Create a new history point.
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// Configuration for the gradient score gauge.
#[derive(Debug, Clone)]
pub struct GradientScoreConfig {
    /// Gauge size (width).
    pub size: u32,
    /// Arc stroke width.
    pub stroke_width: f64,
    /// Show tick marks.
    pub show_ticks: bool,
    /// Show feedback buttons.
    pub show_feedback: bool,
    /// Show history chart.
    pub show_history: bool,
}

impl Default for GradientScoreConfig {
    fn default() -> Self {
        Self {
            size: 280,
            stroke_width: 20.0,
            show_ticks: true,
            show_feedback: true,
            show_history: true,
        }
    }
}

/// Interpolate between two hex colors.
fn interpolate_color(color1: &str, color2: &str, t: f64) -> String {
    let parse_hex = |s: &str| -> (u8, u8, u8) {
        let s = s.trim_start_matches('#');
        let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
        (r, g, b)
    };

    let (r1, g1, b1) = parse_hex(color1);
    let (r2, g2, b2) = parse_hex(color2);

    let r = (r1 as f64 + (r2 as f64 - r1 as f64) * t) as u8;
    let g = (g1 as f64 + (g2 as f64 - g1 as f64) * t) as u8;
    let b = (b1 as f64 + (b2 as f64 - b1 as f64) * t) as u8;

    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// Get gradient color at a given percentage (0-100).
fn gradient_color_at(pct: f64) -> String {
    // Color stops: red (0%) -> orange (25%) -> yellow (50%) -> lime (75%) -> green (100%)
    let stops: &[(f64, &str)] = &[
        (0.0, "#dc2626"),   // Red
        (20.0, "#ea580c"),  // Red-Orange
        (35.0, "#f97316"),  // Orange
        (50.0, "#eab308"),  // Yellow
        (65.0, "#a3e635"),  // Lime
        (80.0, "#22c55e"),  // Green
        (100.0, "#16a34a"), // Dark Green
    ];

    let pct = pct.clamp(0.0, 100.0);

    // Find the two stops to interpolate between
    for i in 0..stops.len() - 1 {
        let (p1, c1) = stops[i];
        let (p2, c2) = stops[i + 1];
        if pct >= p1 && pct <= p2 {
            let t = (pct - p1) / (p2 - p1);
            return interpolate_color(c1, c2, t);
        }
    }

    stops.last().unwrap().1.to_string()
}

/// Generate arc segment path for a portion of the semicircle.
fn arc_segment_path(
    center_x: f64,
    center_y: f64,
    radius: f64,
    start_pct: f64,
    end_pct: f64,
) -> String {
    // Map percentage to angle: 0% = PI (left), 100% = 0 (right)
    let start_angle = std::f64::consts::PI * (1.0 - start_pct / 100.0);
    let end_angle = std::f64::consts::PI * (1.0 - end_pct / 100.0);

    let start_x = center_x + radius * start_angle.cos();
    let start_y = center_y - radius * start_angle.sin();
    let end_x = center_x + radius * end_angle.cos();
    let end_y = center_y - radius * end_angle.sin();

    // For small arcs (< 180 degrees), use large-arc-flag = 0
    // sweep-flag = 0 for counterclockwise (going from left toward right through top)
    format!(
        "M {:.2} {:.2} A {:.2} {:.2} 0 0 0 {:.2} {:.2}",
        start_x, start_y, radius, radius, end_x, end_y
    )
}

/// GradientScoreGauge component.
///
/// A semi-circular gauge with red-orange-yellow-green gradient showing a score.
#[component]
pub fn GradientScoreGauge(
    /// Current score (0-100).
    #[prop(into)]
    score: Signal<f64>,
    /// Date or subtitle text.
    #[prop(optional, into)]
    date: Option<String>,
    /// History data points.
    #[prop(optional, into)]
    history: Option<Signal<Vec<ScoreHistoryPoint>>>,
    /// Callback for thumbs up click.
    #[prop(optional, into)]
    on_thumbs_up: Option<Callback<()>>,
    /// Callback for thumbs down click.
    #[prop(optional, into)]
    on_thumbs_down: Option<Callback<()>>,
    /// Configuration.
    #[prop(optional)]
    config: Option<GradientScoreConfig>,
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
    let show_ticks = config.show_ticks;
    let show_feedback = config.show_feedback;
    let show_history = config.show_history;

    let prefix = format!("fx-gradient-score-{}", design_system);

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

    // SVG dimensions for gauge
    let svg_width = size as f64;
    let svg_height = (size as f64) * 0.6;
    let center_x = svg_width / 2.0;
    let center_y = svg_height - 5.0;
    let radius = (svg_width / 2.0) - stroke_width / 2.0 - 25.0;

    // Generate many gradient arc segments for smooth appearance
    // More segments = smoother gradient (60 segments for near-continuous look)
    let num_segments = 60;
    let segment_size = 100.0 / num_segments as f64;
    let overlap = 0.3; // Small overlap to prevent gaps

    let arc_segments: Vec<(String, String)> = (0..num_segments)
        .map(|i| {
            let start_pct = i as f64 * segment_size;
            let end_pct = ((i + 1) as f64 * segment_size + overlap).min(100.0);
            let mid_pct = (start_pct + end_pct) / 2.0;
            let path = arc_segment_path(center_x, center_y, radius, start_pct, end_pct);
            let color = gradient_color_at(mid_pct);
            (path, color)
        })
        .collect();

    // Calculate indicator position
    let indicator_position = move || {
        let s = score.get().clamp(0.0, 100.0);
        let angle_rad = std::f64::consts::PI * (1.0 - s / 100.0);
        let x = center_x + radius * angle_rad.cos();
        let y = center_y - radius * angle_rad.sin();
        (x, y)
    };

    // Get rating
    let rating = move || ScoreRating::from_score(score.get());

    // Tick marks (0, 20, 40, 60, 80, 100)
    let tick_marks: Vec<(f64, f64, i32)> = (0..=5)
        .map(|i| {
            let val = i * 20;
            let angle_rad = std::f64::consts::PI * (1.0 - val as f64 / 100.0);
            let outer_r = radius + stroke_width / 2.0 + 18.0;
            let x = center_x + outer_r * angle_rad.cos();
            let y = center_y - outer_r * angle_rad.sin();
            (x, y, val)
        })
        .collect();

    // History chart path
    let history_path = move || {
        let history = history.as_ref()?;
        let points = history.get();
        if points.is_empty() {
            return None;
        }

        let chart_width = 200.0;
        let chart_height = 80.0;
        let padding = 20.0;
        let inner_width = chart_width - 2.0 * padding;
        let inner_height = chart_height - 2.0 * padding;

        let min_val = points.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
        let max_val = points.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max);
        let val_range = (max_val - min_val).max(1.0);

        let x_step = inner_width / (points.len() as f64 - 1.0).max(1.0);

        let mut path = String::new();
        let mut dots = Vec::new();

        for (i, point) in points.iter().enumerate() {
            let x = padding + i as f64 * x_step;
            let y = padding + inner_height - ((point.value - min_val) / val_range) * inner_height;

            if i == 0 {
                path.push_str(&format!("M {:.1} {:.1}", x, y));
            } else {
                path.push_str(&format!(" L {:.1} {:.1}", x, y));
            }
            dots.push((x, y, point.label.clone()));
        }

        Some((path, dots, chart_width, chart_height, points.iter().map(|p| p.label.clone()).collect::<Vec<_>>()))
    };

    // Handle feedback clicks
    let handle_thumbs_up = move |_| {
        if let Some(ref cb) = on_thumbs_up {
            cb.run(());
        }
    };

    let handle_thumbs_down = move |_| {
        if let Some(ref cb) = on_thumbs_down {
            cb.run(());
        }
    };

    view! {
        <div class=combined_class style="display: flex; flex-direction: column; align-items: center; background: #fff; padding: 24px; border-radius: 12px; position: relative;">
            // Info icon (top right)
            <div style="position: absolute; top: 16px; right: 16px;">
                <svg width="22" height="22" viewBox="0 0 22 22" style="cursor: pointer;">
                    <circle cx="11" cy="11" r="10" fill="none" stroke="#d1d5db" stroke-width="1.5"/>
                    <text x="11" y="15" text-anchor="middle" font-size="13" fill="#9ca3af" font-family="serif" font-style="italic">"i"</text>
                </svg>
            </div>

            // Gauge SVG
            <svg
                class=format!("{}-gauge", prefix)
                width=format!("{}", size)
                height=format!("{:.0}", svg_height)
                viewBox=format!("0 0 {} {}", svg_width, svg_height)
            >
                // Gradient arc segments (drawn as multiple colored paths for smooth gradient)
                {arc_segments.into_iter().map(|(path, color)| {
                    view! {
                        <path
                            d=path
                            fill="none"
                            stroke=color.clone()
                            stroke-width=format!("{}", stroke_width)
                            stroke-linecap="butt"
                        />
                    }
                }).collect_view()}

                // Tick marks and labels
                {show_ticks.then(|| {
                    tick_marks.iter().map(|(x, y, val)| {
                        view! {
                            <text
                                x=format!("{:.1}", x)
                                y=format!("{:.1}", y)
                                fill="#6b7280"
                                font-size="14"
                                font-weight="500"
                                text-anchor="middle"
                                dominant-baseline="middle"
                            >
                                {*val}
                            </text>
                        }
                    }).collect_view()
                })}

                // Indicator circle (on the arc)
                {move || {
                    let (x, y) = indicator_position();
                    view! {
                        <circle
                            cx=format!("{:.1}", x)
                            cy=format!("{:.1}", y)
                            r=format!("{:.0}", stroke_width / 2.0 + 2.0)
                            fill="#fff"
                            stroke="#1f2937"
                            stroke-width="3"
                        />
                    }
                }}

                // Needle line from center to indicator
                {move || {
                    let (x, y) = indicator_position();
                    view! {
                        <line
                            x1=format!("{:.1}", center_x)
                            y1=format!("{:.1}", center_y)
                            x2=format!("{:.1}", x)
                            y2=format!("{:.1}", y)
                            stroke="#374151"
                            stroke-width="1.5"
                            stroke-dasharray="4,3"
                        />
                    }
                }}
            </svg>

            // Rating label
            <div style=move || format!("font-size: 32px; font-weight: 700; color: {}; margin-top: 8px; margin-bottom: 4px;", rating().color())>
                {move || rating().label()}
            </div>

            // Date
            {date.clone().map(|d| {
                view! {
                    <div style="font-size: 14px; color: #9ca3af; margin-bottom: 16px;">
                        {d}
                    </div>
                }
            })}

            // Feedback buttons
            {show_feedback.then(|| {
                view! {
                    <div style="display: flex; gap: 20px; margin-bottom: 20px;">
                        <button
                            on:click=handle_thumbs_up
                            style="background: none; border: none; cursor: pointer; padding: 8px; color: #d1d5db; font-size: 20px;"
                        >
                            <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                <path d="M14 9V5a3 3 0 00-3-3l-4 9v11h11.28a2 2 0 002-1.7l1.38-9a2 2 0 00-2-2.3zM7 22H4a2 2 0 01-2-2v-7a2 2 0 012-2h3"/>
                            </svg>
                        </button>
                        <button
                            on:click=handle_thumbs_down
                            style="background: none; border: none; cursor: pointer; padding: 8px; color: #d1d5db; font-size: 20px;"
                        >
                            <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                <path d="M10 15v4a3 3 0 003 3l4-9V2H5.72a2 2 0 00-2 1.7l-1.38 9a2 2 0 002 2.3zm7-13h2.67A2.31 2.31 0 0122 4v7a2.31 2.31 0 01-2.33 2H17"/>
                            </svg>
                        </button>
                    </div>
                }
            })}

            // History chart
            {move || {
                if !show_history {
                    return None;
                }
                let history_data = history_path()?;
                let (path, dots, width, height, labels) = history_data;

                Some(view! {
                    <div style="width: 100%; border-top: 1px solid #e5e7eb; padding-top: 16px;">
                        <svg
                            width="100%"
                            height=format!("{:.0}", height + 30.0)
                            viewBox=format!("0 0 {} {}", width, height + 30.0)
                            preserveAspectRatio="xMidYMid meet"
                            style="max-width: 300px;"
                        >
                            // Y-axis labels
                            <text x="5" y="25" fill="#6b7280" font-size="10">"100"</text>
                            <text x="5" y="50" fill="#6b7280" font-size="10">"75"</text>
                            <text x="5" y="75" fill="#6b7280" font-size="10">"50"</text>
                            <text x="5" y="100" fill="#6b7280" font-size="10">"25"</text>
                            <text x="5" y="125" fill="#6b7280" font-size="10">"0"</text>

                            // Y-axis label
                            <text x="5" y="12" fill="#6b7280" font-size="10" font-weight="500">"Scores"</text>

                            // Line path
                            <path
                                d=path
                                fill="none"
                                stroke="#16a34a"
                                stroke-width="2"
                            />

                            // Data points
                            {dots.into_iter().map(|(x, y, _label)| {
                                view! {
                                    <circle
                                        cx=format!("{:.1}", x)
                                        cy=format!("{:.1}", y)
                                        r="4"
                                        fill="#f97316"
                                        stroke="#fff"
                                        stroke-width="1"
                                    />
                                }
                            }).collect_view()}

                            // X-axis labels
                            {labels.iter().enumerate().map(|(i, label)| {
                                let x = 20.0 + i as f64 * (160.0 / (labels.len() as f64 - 1.0).max(1.0));
                                view! {
                                    <text
                                        x=format!("{:.1}", x)
                                        y=format!("{:.0}", height + 20.0)
                                        fill="#6b7280"
                                        font-size="10"
                                        text-anchor="middle"
                                    >
                                        {label.clone()}
                                    </text>
                                }
                            }).collect_view()}

                            // X-axis label
                            <text
                                x=format!("{:.0}", width / 2.0)
                                y=format!("{:.0}", height + 28.0)
                                fill="#6b7280"
                                font-size="10"
                                text-anchor="middle"
                            >
                                "Duration"
                            </text>
                        </svg>
                    </div>
                })
            }}
        </div>
    }
}
