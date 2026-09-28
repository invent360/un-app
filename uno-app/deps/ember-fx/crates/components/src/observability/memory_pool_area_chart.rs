//! MemoryPoolAreaChart Leptos component.
//!
//! A stacked area chart for JVM memory metrics with inline legend.

use leptos::prelude::*;
use crate::try_use_theme;

/// Color options for memory pool series.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MemoryPoolColor {
    /// Used memory (green).
    #[default]
    Used,
    /// Committed memory (yellow/orange).
    Committed,
    /// Max memory (blue/cyan).
    Max,
    /// Custom green.
    Green,
    /// Custom yellow.
    Yellow,
    /// Custom blue.
    Blue,
}

impl MemoryPoolColor {
    /// Returns the CSS color string.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Used | Self::Green => "#73d13d",
            Self::Committed | Self::Yellow => "#fadb14",
            Self::Max | Self::Blue => "#69c0ff",
        }
    }

    /// Returns the fill color with opacity.
    pub fn as_fill_css(&self, opacity: f64) -> String {
        let rgb = match self {
            Self::Used | Self::Green => (115, 209, 61),
            Self::Committed | Self::Yellow => (250, 219, 20),
            Self::Max | Self::Blue => (105, 192, 255),
        };
        format!("rgba({}, {}, {}, {})", rgb.0, rgb.1, rgb.2, opacity)
    }
}

/// A data point in a memory pool series.
#[derive(Debug, Clone)]
pub struct MemoryDataPoint {
    /// Timestamp in Unix milliseconds.
    pub timestamp: i64,
    /// Value in bytes.
    pub value: f64,
}

impl MemoryDataPoint {
    /// Create a new data point.
    pub fn new(timestamp: i64, value: f64) -> Self {
        Self { timestamp, value }
    }
}

/// A series in the memory pool chart.
#[derive(Debug, Clone)]
pub struct MemoryPoolSeries {
    /// Series name (e.g., "used", "committed", "max").
    pub name: String,
    /// Data points.
    pub data: Vec<MemoryDataPoint>,
    /// Series color.
    pub color: MemoryPoolColor,
}

impl MemoryPoolSeries {
    /// Create a new series.
    pub fn new(name: impl Into<String>, data: Vec<MemoryDataPoint>, color: MemoryPoolColor) -> Self {
        Self {
            name: name.into(),
            data,
            color,
        }
    }

    /// Get the maximum value in this series.
    pub fn max(&self) -> f64 {
        self.data.iter().map(|p| p.value).fold(0.0f64, f64::max)
    }

    /// Get the current (last) value in this series.
    pub fn current(&self) -> f64 {
        self.data.last().map(|p| p.value).unwrap_or(0.0)
    }
}

/// Configuration for the memory pool chart.
#[derive(Debug, Clone)]
pub struct MemoryPoolConfig {
    /// Chart height.
    pub height: u32,
    /// Show grid lines.
    pub show_grid: bool,
    /// Y-axis unit (e.g., "MiB", "GiB").
    pub y_unit: Option<String>,
    /// Fill opacity for areas.
    pub fill_opacity: f64,
}

impl Default for MemoryPoolConfig {
    fn default() -> Self {
        Self {
            height: 200,
            show_grid: true,
            y_unit: Some("MiB".to_string()),
            fill_opacity: 0.6,
        }
    }
}

/// Format bytes to human-readable string.
fn format_bytes(bytes: f64, unit: &str) -> String {
    let divisor = match unit {
        "KiB" => 1024.0,
        "MiB" => 1024.0 * 1024.0,
        "GiB" => 1024.0 * 1024.0 * 1024.0,
        "KB" => 1000.0,
        "MB" => 1000.0 * 1000.0,
        "GB" => 1000.0 * 1000.0 * 1000.0,
        _ => 1.0,
    };
    format!("{:.2} {}", bytes / divisor, unit)
}

/// MemoryPoolAreaChart component.
///
/// A stacked area chart for JVM memory metrics (used/committed/max).
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{
///     MemoryPoolAreaChart, MemoryPoolSeries, MemoryDataPoint, MemoryPoolColor,
/// };
///
/// let series = vec![
///     MemoryPoolSeries::new("used", used_data, MemoryPoolColor::Used),
///     MemoryPoolSeries::new("committed", committed_data, MemoryPoolColor::Committed),
///     MemoryPoolSeries::new("max", max_data, MemoryPoolColor::Max),
/// ];
///
/// view! {
///     <MemoryPoolAreaChart
///         series=Signal::derive(move || series.clone())
///         title="G1 Eden Space".to_string()
///     />
/// }
/// ```
#[component]
pub fn MemoryPoolAreaChart(
    /// Data series.
    #[prop(into)]
    series: Signal<Vec<MemoryPoolSeries>>,
    /// Chart title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Configuration.
    #[prop(optional)]
    config: Option<MemoryPoolConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let height = config.height;
    let show_grid = config.show_grid;
    let y_unit = config.y_unit.clone().unwrap_or_default();
    let fill_opacity = config.fill_opacity;

    let y_unit_clone = y_unit.clone();

    let prefix = format!("fx-memory-pool-{}", design_system);

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
    let padding_left: f64 = 60.0;
    let padding_right: f64 = 10.0;
    let padding_top: f64 = 10.0;
    let padding_bottom: f64 = 25.0;

    let inner_width = svg_width - padding_left - padding_right;
    let inner_height = svg_height - padding_top - padding_bottom;

    // Calculate bounds
    let bounds = move || {
        let all_series = series.get();
        if all_series.is_empty() {
            return (0i64, 0i64, 0.0f64, 100.0f64);
        }

        let mut min_time = i64::MAX;
        let mut max_time = i64::MIN;
        let mut max_val = 0.0f64;

        for s in &all_series {
            for p in &s.data {
                min_time = min_time.min(p.timestamp);
                max_time = max_time.max(p.timestamp);
                max_val = max_val.max(p.value);
            }
        }

        // Add padding to max
        max_val *= 1.1;

        (min_time, max_time, 0.0, max_val)
    };

    // Generate area paths (render in reverse order for proper stacking)
    let area_paths = move || {
        let all_series = series.get();
        let (min_time, max_time, min_val, max_val) = bounds();
        let time_range = (max_time - min_time).max(1) as f64;
        let val_range = (max_val - min_val).max(0.001);

        // Sort by typical priority: max first, then committed, then used
        // This ensures used is rendered last (on top)
        let mut paths: Vec<_> = all_series
            .iter()
            .map(|s| {
                let mut line_path = String::new();
                let mut fill_path = String::new();

                if !s.data.is_empty() {
                    // Start fill at bottom-left
                    let first_x = padding_left + ((s.data[0].timestamp - min_time) as f64 / time_range) * inner_width;
                    fill_path.push_str(&format!("M {:.1} {:.1}", first_x, padding_top + inner_height));

                    for (i, p) in s.data.iter().enumerate() {
                        let x = padding_left + ((p.timestamp - min_time) as f64 / time_range) * inner_width;
                        let y = padding_top + inner_height - ((p.value - min_val) / val_range) * inner_height;

                        if i == 0 {
                            line_path.push_str(&format!("M {:.1} {:.1}", x, y));
                            fill_path.push_str(&format!(" L {:.1} {:.1}", x, y));
                        } else {
                            line_path.push_str(&format!(" L {:.1} {:.1}", x, y));
                            fill_path.push_str(&format!(" L {:.1} {:.1}", x, y));
                        }
                    }

                    // Close fill path
                    let last_x = padding_left + ((s.data.last().unwrap().timestamp - min_time) as f64 / time_range) * inner_width;
                    fill_path.push_str(&format!(" L {:.1} {:.1} Z", last_x, padding_top + inner_height));
                }

                (s.name.clone(), s.color, line_path, fill_path, s.max(), s.current())
            })
            .collect();

        // Reverse to render larger values first (behind smaller values)
        paths.reverse();
        paths
    };

    // Y-axis labels
    let y_axis_labels = {
        let y_unit = y_unit.clone();
        move || {
            let (_, _, min_val, max_val) = bounds();
            let val_range = (max_val - min_val).max(0.001);

            (0..=4)
                .map(|i| {
                    let y = padding_top + (i as f64 / 4.0) * inner_height;
                    let val = max_val - (i as f64 / 4.0) * val_range;
                    let label = format_bytes(val, &y_unit);
                    (padding_left - 5.0, y, label)
                })
                .collect::<Vec<_>>()
        }
    };

    // Grid lines
    let grid_lines = move || {
        (0..=4)
            .map(|i| {
                let y = padding_top + (i as f64 / 4.0) * inner_height;
                format!(
                    "M {:.1} {:.1} L {:.1} {:.1}",
                    padding_left, y, padding_left + inner_width, y
                )
            })
            .collect::<Vec<_>>()
    };

    // Time labels
    let time_labels = move || {
        let (min_time, max_time, _, _) = bounds();
        let time_range = (max_time - min_time).max(1) as f64;

        (0..6)
            .map(|i| {
                let t = min_time + ((i as f64 / 5.0) * time_range) as i64;
                let x = padding_left + (i as f64 / 5.0) * inner_width;

                let total_seconds = t / 1000;
                let hours = (total_seconds / 3600) % 24;
                let minutes = (total_seconds / 60) % 60;
                let seconds = total_seconds % 60;
                let label = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

                (x, label)
            })
            .collect::<Vec<_>>()
    };

    // Legend items
    let legend_items = {
        let y_unit = y_unit_clone.clone();
        move || {
            let all_series = series.get();
            all_series
                .iter()
                .map(|s| {
                    let max_formatted = format_bytes(s.max(), &y_unit);
                    let current_formatted = format_bytes(s.current(), &y_unit);
                    (s.name.clone(), s.color, max_formatted, current_formatted)
                })
                .collect::<Vec<_>>()
        }
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
                // Grid lines
                {move || {
                    if show_grid {
                        Some(grid_lines().into_iter().map(|path| {
                            view! {
                                <path
                                    d=path
                                    fill="none"
                                    stroke="var(--fx-color-border, #303030)"
                                    stroke-width="1"
                                />
                            }
                        }).collect_view())
                    } else {
                        None
                    }
                }}

                // Y-axis labels
                {move || {
                    y_axis_labels().into_iter().map(|(x, y, label)| {
                        view! {
                            <text
                                x=format!("{:.1}", x)
                                y=format!("{:.1}", y)
                                fill="var(--fx-color-text-tertiary, #6b7280)"
                                font-size="10"
                                text-anchor="end"
                                dominant-baseline="middle"
                            >
                                {label}
                            </text>
                        }
                    }).collect_view()
                }}

                // Time labels
                {move || {
                    time_labels().into_iter().map(|(x, label)| {
                        view! {
                            <text
                                x=format!("{:.1}", x)
                                y=format!("{:.1}", svg_height - 5.0)
                                fill="var(--fx-color-text-tertiary, #6b7280)"
                                font-size="10"
                                text-anchor="middle"
                            >
                                {label}
                            </text>
                        }
                    }).collect_view()
                }}

                // Area fills (rendered first, behind lines)
                {move || {
                    area_paths().into_iter().map(|(_name, color, _line_path, fill_path, _max, _current)| {
                        view! {
                            <path
                                d=fill_path
                                fill=color.as_fill_css(fill_opacity)
                            />
                        }
                    }).collect_view()
                }}

                // Lines (rendered on top)
                {move || {
                    area_paths().into_iter().map(|(_name, color, line_path, _fill_path, _max, _current)| {
                        view! {
                            <path
                                d=line_path
                                fill="none"
                                stroke=color.as_css()
                                stroke-width="1.5"
                            />
                        }
                    }).collect_view()
                }}
            </svg>

            // Inline legend
            <div class=format!("{}-legend", prefix) style="display: flex; flex-wrap: wrap; gap: 16px; padding: 8px 0; font-size: 11px;">
                {move || {
                    legend_items().into_iter().map(|(name, color, max_val, current_val)| {
                        view! {
                            <div style="display: flex; align-items: center; gap: 4px;">
                                <span style=format!("color: {};", color.as_css())>"—"</span>
                                <span style="color: var(--fx-color-text-secondary, #a0a0a0);">{name}</span>
                                <span style="color: var(--fx-color-text-tertiary, #6b7280);">"Max:"</span>
                                <span style="color: var(--fx-color-text, #fff);">{max_val}</span>
                                <span style="color: var(--fx-color-text-tertiary, #6b7280);">"Current:"</span>
                                <span style="color: var(--fx-color-text, #fff);">{current_val}</span>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}
