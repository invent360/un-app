use leptos::prelude::*;

use super::types::{ColorPalette, RangeAreaChartConfig, RangePoint};
use super::utils::{chart_prefix, try_use_theme};

/// Range area chart component
#[component]
pub fn RangeAreaChart(
    /// Range point data (min/max values)
    #[prop(into)]
    data: Signal<Vec<RangePoint>>,
    /// Configuration for the range area chart
    #[prop(optional)]
    config: Option<RangeAreaChartConfig>,
    /// Optional color palette
    #[prop(optional, into)]
    colors: Option<Signal<ColorPalette>>,
    /// Whether the chart is in a loading state
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme = try_use_theme();
    let prefix = chart_prefix("rangearea");

    let config = config.unwrap_or_default();

    let default_colors = ColorPalette::default();
    let colors = colors.unwrap_or_else(|| Signal::stored(default_colors));

    // Chart dimensions
    let width = 600.0;
    let height = 400.0;
    let padding_left = 60.0;
    let padding_right = 40.0;
    let padding_top = 40.0;
    let padding_bottom = 60.0;
    let chart_width = width - padding_left - padding_right;
    let chart_height = height - padding_top - padding_bottom;

    // Pre-clone prefix for closures
    let prefix_area = prefix.clone();
    let prefix_loading = prefix.clone();
    let prefix_spinner = prefix.clone();

    // Store config values needed in closures
    let show_grid = config.show_grid;
    let fill_opacity = config.fill_opacity;

    // Calculate range area data
    let range_data = Memo::new(move |_| {
        let points = data.get();
        let palette = colors.get();

        if points.is_empty() {
            return (String::new(), Vec::new(), Vec::new(), String::new());
        }

        // Find min/max values
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;
        for point in &points {
            min_val = min_val.min(point.y_min);
            max_val = max_val.max(point.y_max);
        }

        // Add padding
        let range = max_val - min_val;
        let value_padding = range * 0.1;
        min_val -= value_padding;
        max_val += value_padding;

        let x_step = chart_width / (points.len() - 1).max(1) as f64;

        // Build the area path (upper line forward, lower line backward)
        let mut upper_points: Vec<(f64, f64)> = Vec::new();
        let mut lower_points: Vec<(f64, f64)> = Vec::new();

        for (i, point) in points.iter().enumerate() {
            let x = padding_left + (i as f64 * x_step);
            let y_high = padding_top + chart_height * (1.0 - (point.y_max - min_val) / (max_val - min_val));
            let y_low = padding_top + chart_height * (1.0 - (point.y_min - min_val) / (max_val - min_val));

            upper_points.push((x, y_high));
            lower_points.push((x, y_low));
        }

        // Create area path
        let mut path = String::new();
        for (i, (x, y)) in upper_points.iter().enumerate() {
            if i == 0 {
                path.push_str(&format!("M {} {}", x, y));
            } else {
                path.push_str(&format!(" L {} {}", x, y));
            }
        }
        // Go back along the lower line
        for (x, y) in lower_points.iter().rev() {
            path.push_str(&format!(" L {} {}", x, y));
        }
        path.push_str(" Z");

        let color = palette.color_at(0).to_string();

        // X-axis labels
        let x_labels: Vec<(f64, String)> = points
            .iter()
            .enumerate()
            .map(|(i, point)| {
                let x = padding_left + (i as f64 * x_step);
                let label = format!("{:.0}", point.x);
                (x, label)
            })
            .collect();

        // Y-axis labels
        let num_ticks = 6;
        let y_labels: Vec<(f64, String)> = (0..=num_ticks)
            .map(|i| {
                let value = min_val + (max_val - min_val) * (i as f64 / num_ticks as f64);
                let y = padding_top + chart_height * (1.0 - i as f64 / num_ticks as f64);
                (y, format!("{:.1}", value))
            })
            .collect();

        (path, x_labels, y_labels, color)
    });

    let combined_class = {
        let mut parts = vec![prefix.clone()];
        if loading {
            parts.push(format!("{}-loading", prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class>
            <svg viewBox=format!("0 0 {} {}", width, height) class=format!("{}-svg", prefix)>
                // Y-axis
                <line
                    x1=padding_left
                    y1=padding_top
                    x2=padding_left
                    y2=height - padding_bottom
                    stroke=move || if theme.is_some() { "var(--chart-axis)" } else { "#9ca3af" }
                    stroke-width="1"
                />

                // X-axis
                <line
                    x1=padding_left
                    y1=height - padding_bottom
                    x2=width - padding_right
                    y2=height - padding_bottom
                    stroke=move || if theme.is_some() { "var(--chart-axis)" } else { "#9ca3af" }
                    stroke-width="1"
                />

                // Grid lines and Y-axis labels
                {move || {
                    if show_grid {
                        let (_, _, y_labels, _) = range_data.get();
                        Some(y_labels.into_iter().map(|(y, label)| {
                            view! {
                                <g>
                                    <line
                                        x1=padding_left
                                        y1=y
                                        x2=width - padding_right
                                        y2=y
                                        stroke=move || if theme.is_some() { "var(--chart-grid)" } else { "#e5e7eb" }
                                        stroke-width="1"
                                        stroke-dasharray="4,4"
                                    />
                                    <text
                                        x=padding_left - 10.0
                                        y=y
                                        text-anchor="end"
                                        dominant-baseline="middle"
                                        fill=move || if theme.is_some() { "var(--chart-text)" } else { "#6b7280" }
                                        font-size="12"
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

                // X-axis labels
                {move || {
                    let (_, x_labels, _, _) = range_data.get();
                    x_labels.into_iter().map(|(x, label)| {
                        view! {
                            <text
                                x=x
                                y=height - padding_bottom + 20.0
                                text-anchor="middle"
                                fill=move || if theme.is_some() { "var(--chart-text)" } else { "#6b7280" }
                                font-size="12"
                            >
                                {label}
                            </text>
                        }
                    }).collect_view()
                }}

                // Range area
                {move || {
                    let (path, _, _, color) = range_data.get();
                    let prefix_area_clone = prefix_area.clone();

                    view! {
                        <path
                            d=path
                            fill=color.clone()
                            fill-opacity=fill_opacity
                            stroke=color.clone()
                            stroke-width="2"
                            class=format!("{}-area", prefix_area_clone)
                        />
                    }
                }}
            </svg>

            // Loading overlay
            {move || {
                if loading {
                    let prefix_loading_clone = prefix_loading.clone();
                    let prefix_spinner_clone = prefix_spinner.clone();
                    Some(view! {
                        <div class=format!("{}-loading", prefix_loading_clone)>
                            <div class=format!("{}-spinner", prefix_spinner_clone)></div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
