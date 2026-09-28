use leptos::prelude::*;

use super::types::{ColorPalette, SlopeChartConfig, SlopePoint};
use super::utils::{chart_prefix, try_use_theme};

/// Slope chart component for comparing two values
#[component]
pub fn SlopeChart(
    /// Slope data points (pairs of values)
    #[prop(into)]
    data: Signal<Vec<SlopePoint>>,
    /// Configuration for the slope chart
    #[prop(optional)]
    config: Option<SlopeChartConfig>,
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
    let chart_prefix_str = chart_prefix("slope");
    let prefix_class = chart_prefix_str.clone();
    let prefix_svg = format!("{}-svg", chart_prefix_str);
    let prefix_slope = format!("{}-slope", chart_prefix_str);
    let prefix_line = format!("{}-line", chart_prefix_str);
    let prefix_loading = format!("{}-loading", chart_prefix_str);
    let prefix_spinner = format!("{}-spinner", chart_prefix_str);

    let _config = config.unwrap_or_default();

    let default_colors = ColorPalette::default();
    let colors = colors.unwrap_or_else(|| Signal::stored(default_colors));

    // Chart dimensions
    let width = 500.0;
    let height = 400.0;
    let padding_top = 60.0;
    let padding_bottom = 40.0;
    let padding_side = 100.0;
    let chart_height = height - padding_top - padding_bottom;

    // Calculate slope data
    let slope_data = Memo::new(move |_| {
        let points = data.get();
        let palette = colors.get();

        if points.is_empty() {
            return (Vec::new(), 0.0, 0.0, String::new(), String::new());
        }

        // Find min/max across both columns
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;
        for point in &points {
            min_val = min_val.min(point.start).min(point.end);
            max_val = max_val.max(point.start).max(point.end);
        }

        // Add padding to range
        let range = max_val - min_val;
        let value_padding = range * 0.1;
        min_val -= value_padding;
        max_val += value_padding;

        let x_start = padding_side;
        let x_end = width - padding_side;

        let slopes: Vec<(f64, f64, f64, f64, String, String, f64, f64, bool)> = points
            .iter()
            .enumerate()
            .map(|(i, point)| {
                let y_start = padding_top + chart_height * (1.0 - (point.start - min_val) / (max_val - min_val));
                let y_end = padding_top + chart_height * (1.0 - (point.end - min_val) / (max_val - min_val));

                let color = point.color.clone().unwrap_or_else(|| {
                    palette.color_at(i).to_string()
                });

                let is_increase = point.end >= point.start;

                (x_start, y_start, x_end, y_end, point.label.clone(), color, point.start, point.end, is_increase)
            })
            .collect();

        let start_label = "Start".to_string();
        let end_label = "End".to_string();

        (slopes, min_val, max_val, start_label, end_label)
    });

    let combined_class = {
        let mut parts = vec![prefix_class.clone()];
        if loading {
            parts.push(format!("{}-loading", prefix_class));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class>
            <svg viewBox=format!("0 0 {} {}", width, height) class=prefix_svg>
                // Column labels
                {move || {
                    let (_, _, _, start_label, end_label) = slope_data.get();
                    view! {
                        <>
                            <text
                                x=padding_side
                                y=padding_top - 30.0
                                text-anchor="middle"
                                fill=move || if theme.is_some() { "var(--chart-text)" } else { "#374151" }
                                font-size="14"
                                font-weight="600"
                            >
                                {start_label}
                            </text>
                            <text
                                x=width - padding_side
                                y=padding_top - 30.0
                                text-anchor="middle"
                                fill=move || if theme.is_some() { "var(--chart-text)" } else { "#374151" }
                                font-size="14"
                                font-weight="600"
                            >
                                {end_label}
                            </text>
                        </>
                    }
                }}

                // Vertical axis lines
                <line
                    x1=padding_side
                    y1=padding_top
                    x2=padding_side
                    y2=height - padding_bottom
                    stroke=move || if theme.is_some() { "var(--chart-axis)" } else { "#d1d5db" }
                    stroke-width="1"
                />
                <line
                    x1=width - padding_side
                    y1=padding_top
                    x2=width - padding_side
                    y2=height - padding_bottom
                    stroke=move || if theme.is_some() { "var(--chart-axis)" } else { "#d1d5db" }
                    stroke-width="1"
                />

                // Slope lines and points
                {
                    let slope_class = prefix_slope.clone();
                    let line_class = prefix_line.clone();
                    move || {
                        let (slopes, _, _, _, _) = slope_data.get();
                        let show_values = true;
                        let show_labels = true;
                        let slope_class = slope_class.clone();
                        let line_class = line_class.clone();

                        slopes.into_iter().map({
                            let slope_class = slope_class.clone();
                            let line_class = line_class.clone();
                            move |(x1, y1, x2, y2, label, color, start_val, end_val, _is_increase)| {
                                let line_color = color.clone();

                                view! {
                                    <g class=slope_class.clone()>
                                        // Connecting line
                                        <line
                                            x1=x1
                                            y1=y1
                                            x2=x2
                                            y2=y2
                                            stroke=line_color.clone()
                                            stroke-width="2"
                                            class=line_class.clone()
                                        />

                                        // Start point
                                        <circle
                                            cx=x1
                                            cy=y1
                                            r="6"
                                            fill=line_color.clone()
                                            stroke="white"
                                            stroke-width="2"
                                        />

                                        // End point
                                        <circle
                                            cx=x2
                                            cy=y2
                                            r="6"
                                            fill=line_color.clone()
                                            stroke="white"
                                            stroke-width="2"
                                        />

                                        // Start value label
                                        {if show_values {
                                            Some(view! {
                                                <text
                                                    x=x1 - 10.0
                                                    y=y1
                                                    text-anchor="end"
                                                    dominant-baseline="middle"
                                                    fill=move || if theme.is_some() { "var(--chart-text)" } else { "#374151" }
                                                    font-size="11"
                                                >
                                                    {format!("{:.1}", start_val)}
                                                </text>
                                            })
                                        } else {
                                            None
                                        }}

                                        // End value label
                                        {if show_values {
                                            Some(view! {
                                                <text
                                                    x=x2 + 10.0
                                                    y=y2
                                                    text-anchor="start"
                                                    dominant-baseline="middle"
                                                    fill=move || if theme.is_some() { "var(--chart-text)" } else { "#374151" }
                                                    font-size="11"
                                                >
                                                    {format!("{:.1}", end_val)}
                                                </text>
                                            })
                                        } else {
                                            None
                                        }}

                                        // Label (on left side)
                                        {if show_labels {
                                            Some(view! {
                                                <text
                                                    x=x1 - 50.0
                                                    y=y1
                                                    text-anchor="end"
                                                    dominant-baseline="middle"
                                                    fill=move || if theme.is_some() { "var(--chart-text)" } else { "#6b7280" }
                                                    font-size="12"
                                                >
                                                    {label}
                                                </text>
                                            })
                                        } else {
                                            None
                                        }}
                                    </g>
                                }
                            }
                        }).collect_view()
                    }
                }
            </svg>

            // Loading overlay
            {
                let loading_class = prefix_loading.clone();
                let spinner_class = prefix_spinner.clone();
                move || {
                    if loading {
                        Some(view! {
                            <div class=loading_class.clone()>
                                <div class=spinner_class.clone()></div>
                            </div>
                        })
                    } else {
                        None
                    }
                }
            }
        </div>
    }
}
