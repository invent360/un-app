//! StackedStatusChart Leptos component.
//!
//! A stacked area chart for status breakdowns (success/warning/error).

use leptos::prelude::*;
use crate::try_use_theme;

/// Status level for a band.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StatusLevel {
    /// Success (green).
    #[default]
    Success,
    /// Warning (yellow).
    Warning,
    /// Error (red).
    Error,
    /// Info (blue).
    Info,
    /// Neutral (gray).
    Neutral,
}

impl StatusLevel {
    /// Returns the CSS color for this status.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Success => "#73bf69",
            Self::Warning => "#fade2a",
            Self::Error => "#f2495c",
            Self::Info => "#5794f2",
            Self::Neutral => "#8c8c8c",
        }
    }

    /// Returns the CSS fill color with opacity.
    pub fn as_fill_css(&self, opacity: f64) -> String {
        let rgb = match self {
            Self::Success => (115, 191, 105),
            Self::Warning => (250, 222, 42),
            Self::Error => (242, 73, 92),
            Self::Info => (87, 148, 242),
            Self::Neutral => (140, 140, 140),
        };
        format!("rgba({}, {}, {}, {})", rgb.0, rgb.1, rgb.2, opacity)
    }
}

/// A data point in a status band.
#[derive(Debug, Clone)]
pub struct StatusDataPoint {
    /// Timestamp in Unix milliseconds.
    pub timestamp: i64,
    /// Value at this timestamp.
    pub value: f64,
}

impl StatusDataPoint {
    /// Create a new data point.
    pub fn new(timestamp: i64, value: f64) -> Self {
        Self { timestamp, value }
    }
}

/// A status band (one layer in the stack).
#[derive(Debug, Clone)]
pub struct StatusBand {
    /// Band name.
    pub name: String,
    /// Data points.
    pub data: Vec<StatusDataPoint>,
    /// Status level (determines color).
    pub status: StatusLevel,
}

impl StatusBand {
    /// Create a new status band.
    pub fn new(name: impl Into<String>, data: Vec<StatusDataPoint>, status: StatusLevel) -> Self {
        Self {
            name: name.into(),
            data,
            status,
        }
    }
}

/// Format a timestamp for display.
#[allow(dead_code)]
fn format_time(timestamp: i64) -> String {
    let total_seconds = timestamp / 1000;
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    format!("{:02}:{:02}", hours, minutes)
}

/// StackedStatusChart component.
///
/// A stacked area chart showing status breakdowns over time.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{StackedStatusChart, StatusBand, StatusLevel, StatusDataPoint};
///
/// let bands = vec![
///     StatusBand::new("success", success_data, StatusLevel::Success),
///     StatusBand::new("warning", warning_data, StatusLevel::Warning),
///     StatusBand::new("error", error_data, StatusLevel::Error),
/// ];
///
/// view! {
///     <StackedStatusChart
///         bands=Signal::derive(move || bands.clone())
///         title="Request Per Second".to_string()
///     />
/// }
/// ```
#[component]
pub fn StackedStatusChart(
    /// Status bands (bottom to top in render order).
    #[prop(into)]
    bands: Signal<Vec<StatusBand>>,
    /// Chart title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Chart height.
    #[prop(optional)]
    height: Option<u32>,
    /// Fill opacity.
    #[prop(optional)]
    fill_opacity: Option<f64>,
    /// Show legend.
    #[prop(optional)]
    show_legend: Option<bool>,
    /// Show grid.
    #[prop(optional)]
    show_grid: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let height = height.unwrap_or(150);
    let fill_opacity = fill_opacity.unwrap_or(0.8);
    let show_legend = show_legend.unwrap_or(true);
    let show_grid = show_grid.unwrap_or(true);

    let prefix = format!("fx-stacked-status-{}", design_system);

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
    let svg_width: f64 = 600.0;
    let svg_height: f64 = height as f64;
    let padding_left: f64 = 40.0;
    let padding_right: f64 = 10.0;
    let padding_top: f64 = 10.0;
    let padding_bottom: f64 = 25.0;

    let inner_width = svg_width - padding_left - padding_right;
    let inner_height = svg_height - padding_top - padding_bottom;

    // Calculate bounds
    let bounds = move || {
        let all_bands = bands.get();
        if all_bands.is_empty() {
            return (0i64, 0i64, 0.0f64, 100.0f64);
        }

        let mut min_time = i64::MAX;
        let mut max_time = i64::MIN;
        let mut max_stacked = 0.0f64;

        // Get all unique timestamps and calculate stacked max
        for band in &all_bands {
            for p in &band.data {
                min_time = min_time.min(p.timestamp);
                max_time = max_time.max(p.timestamp);
            }
        }

        // Calculate max stacked value at any timestamp
        // Simplified: assume all bands have same timestamps
        if let Some(first_band) = all_bands.first() {
            for (i, _) in first_band.data.iter().enumerate() {
                let mut stacked: f64 = 0.0;
                for band in &all_bands {
                    if let Some(p) = band.data.get(i) {
                        stacked += p.value;
                    }
                }
                max_stacked = max_stacked.max(stacked);
            }
        }

        (min_time, max_time, 0.0, max_stacked * 1.1)
    };

    // Generate stacked area paths (bottom to top)
    let stacked_paths = move || {
        let all_bands = bands.get();
        let (min_time, max_time, min_val, max_val) = bounds();
        let time_range = (max_time - min_time).max(1) as f64;
        let val_range = (max_val - min_val).max(0.001);

        if all_bands.is_empty() {
            return Vec::new();
        }

        let num_points = all_bands.first().map(|b| b.data.len()).unwrap_or(0);
        let mut cumulative: Vec<f64> = vec![0.0; num_points];
        let mut paths = Vec::new();

        for band in &all_bands {
            let mut path = String::new();

            // Start at bottom-left baseline
            if let Some(first) = band.data.first() {
                let x = padding_left + ((first.timestamp - min_time) as f64 / time_range) * inner_width;
                let y_base = padding_top + inner_height - (cumulative[0] / val_range) * inner_height;
                path.push_str(&format!("M {:.1} {:.1}", x, y_base));
            }

            // Draw top edge (current band + cumulative)
            for (i, p) in band.data.iter().enumerate() {
                let x = padding_left + ((p.timestamp - min_time) as f64 / time_range) * inner_width;
                let y_top = padding_top + inner_height - ((cumulative[i] + p.value - min_val) / val_range) * inner_height;
                path.push_str(&format!(" L {:.1} {:.1}", x, y_top));
            }

            // Draw bottom edge in reverse (baseline)
            for (i, p) in band.data.iter().enumerate().rev() {
                let x = padding_left + ((p.timestamp - min_time) as f64 / time_range) * inner_width;
                let y_base = padding_top + inner_height - ((cumulative[i] - min_val) / val_range) * inner_height;
                path.push_str(&format!(" L {:.1} {:.1}", x, y_base));
            }

            path.push_str(" Z");

            // Update cumulative for next band
            for (i, p) in band.data.iter().enumerate() {
                cumulative[i] += p.value;
            }

            paths.push((band.name.clone(), band.status, path));
        }

        // Reverse to render bottom bands first
        paths.reverse();
        paths
    };

    // Grid lines
    let grid_lines = move || {
        let (min_time, max_time, min_val, max_val) = bounds();
        let _time_range = (max_time - min_time).max(1) as f64;
        let val_range = (max_val - min_val).max(0.001);

        let mut lines = Vec::new();

        // Horizontal grid lines
        for i in 0..=4 {
            let y = padding_top + (i as f64 / 4.0) * inner_height;
            let val = max_val - (i as f64 / 4.0) * val_range;
            lines.push((
                format!("M {:.1} {:.1} L {:.1} {:.1}", padding_left, y, padding_left + inner_width, y),
                format!("{:.0}", val),
                padding_left - 5.0,
                y,
            ));
        }

        lines
    };

    view! {
        <div class=combined_class>
            // Title
            {title.clone().map(|t| {
                let prefix = prefix.clone();
                view! {
                    <div class=format!("{}-title", prefix)>
                        {t}
                    </div>
                }
            })}

            // Chart SVG
            <svg
                class=format!("{}-chart", prefix)
                viewBox=format!("0 0 {} {}", svg_width, svg_height)
                preserveAspectRatio="xMidYMid meet"
            >
                // Grid
                {move || {
                    if show_grid {
                        Some(grid_lines().into_iter().map(|(path, label, x, y)| {
                            view! {
                                <g>
                                    <path
                                        d=path
                                        fill="none"
                                        stroke="var(--fx-color-border, #303030)"
                                        stroke-width="1"
                                    />
                                    <text
                                        x=x
                                        y=y
                                        fill="var(--fx-color-text-tertiary, #6b7280)"
                                        font-size="10"
                                        text-anchor="end"
                                        dominant-baseline="middle"
                                    >
                                        {label}
                                    </text>
                                </g>
                            }
                        }).collect_view())
                    } else {
                        None
                    }
                }}

                // Stacked areas
                {move || {
                    stacked_paths().into_iter().map(|(name, status, path)| {
                        let fill_color = status.as_fill_css(fill_opacity);

                        view! {
                            <path
                                d=path
                                fill=fill_color
                                data-name=name
                            />
                        }
                    }).collect_view()
                }}
            </svg>

            // Legend
            {move || {
                if show_legend {
                    let all_bands = bands.get();
                    let prefix = prefix.clone();

                    Some(view! {
                        <div class=format!("{}-legend", prefix)>
                            {all_bands.iter().rev().map(|band| {
                                let prefix = prefix.clone();
                                view! {
                                    <div class=format!("{}-legend-item", prefix)>
                                        <span
                                            class=format!("{}-legend-color", prefix)
                                            style=format!("background-color: {};", band.status.as_css())
                                        />
                                        <span class=format!("{}-legend-name", prefix)>
                                            {band.name.clone()}
                                        </span>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
