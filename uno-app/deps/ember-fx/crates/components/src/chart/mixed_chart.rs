use leptos::prelude::*;

use super::types::{ColorPalette, MixedChartConfig, MixedChartSeries, ChartType};
use super::utils::{chart_prefix, try_use_theme};

/// Mixed chart component supporting combined chart types
#[component]
pub fn MixedChart(
    /// Mixed series data with different chart types
    #[prop(into)]
    data: Signal<Vec<MixedChartSeries>>,
    /// X-axis labels
    #[prop(into)]
    labels: Signal<Vec<String>>,
    /// Configuration for the mixed chart
    #[prop(optional)]
    config: Option<MixedChartConfig>,
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
    let prefix = chart_prefix("mixed");

    let config = config.unwrap_or_default();

    // Pre-clone prefix for closures
    let prefix_class = prefix.clone();
    let prefix_svg = prefix.clone();
    let prefix_series = prefix.clone();
    let prefix_legend = prefix.clone();
    let prefix_loading = prefix.clone();

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

    // Store config values for closures
    let tick_count = config.tick_count.unwrap_or(6);
    let show_points = config.show_points.unwrap_or(true);
    let fill_opacity = config.fill_opacity.unwrap_or(0.3);
    let show_legend = config.show_legend.unwrap_or(true);

    // Calculate chart data
    let chart_data = Memo::new(move |_| {
        let series_list = data.get();
        let x_labels = labels.get();
        let palette = colors.get();

        if series_list.is_empty() || x_labels.is_empty() {
            return (Vec::new(), Vec::new(), Vec::new(), 0.0, 0.0);
        }

        // Find global min/max
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;
        for series in &series_list {
            for &val in &series.values {
                min_val = min_val.min(val);
                max_val = max_val.max(val);
            }
        }

        // Add padding
        let range = max_val - min_val;
        let value_padding = range * 0.1;
        min_val -= value_padding;
        max_val += value_padding;

        let x_step = chart_width / x_labels.len() as f64;
        let bar_width = x_step * 0.6;

        // Process each series
        let rendered_series: Vec<(ChartType, Vec<(f64, f64, f64)>, String, String)> = series_list
            .iter()
            .enumerate()
            .map(|(idx, series)| {
                let color = series.color.clone().unwrap_or_else(|| {
                    palette.color_at(idx).to_string()
                });

                let points: Vec<(f64, f64, f64)> = series.values
                    .iter()
                    .enumerate()
                    .map(|(i, &val)| {
                        let x = padding_left + (i as f64 * x_step) + x_step / 2.0;
                        let y = padding_top + chart_height * (1.0 - (val - min_val) / (max_val - min_val));
                        (x, y, val)
                    })
                    .collect();

                (series.chart_type.clone(), points, series.label.clone(), color)
            })
            .collect();

        // X-axis labels with positions
        let x_label_data: Vec<(f64, String)> = x_labels
            .iter()
            .enumerate()
            .map(|(i, label)| {
                let x = padding_left + (i as f64 * x_step) + x_step / 2.0;
                (x, label.clone())
            })
            .collect();

        // Y-axis labels
        let y_labels: Vec<(f64, String)> = (0..=tick_count)
            .map(|i| {
                let value = min_val + (max_val - min_val) * (i as f64 / tick_count as f64);
                let y = padding_top + chart_height * (1.0 - i as f64 / tick_count as f64);
                (y, format!("{:.1}", value))
            })
            .collect();

        (rendered_series, x_label_data, y_labels, bar_width, min_val)
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
            <svg viewBox=format!("0 0 {} {}", width, height) class=format!("{}-svg", prefix_svg)>
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
                    let (_, _, y_labels, _, _) = chart_data.get();
                    y_labels.into_iter().map(|(y, label)| {
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
                    }).collect_view()
                }}

                // X-axis labels
                {move || {
                    let (_, x_labels, _, _, _) = chart_data.get();
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

                // Render each series based on its type
                {
                    let prefix_series = prefix_series.clone();
                    move || {
                        let (rendered_series, _, _, bar_width, _min_val) = chart_data.get();
                        let prefix_inner = prefix_series.clone();

                        rendered_series.into_iter().enumerate().map(|(series_idx, (chart_type, points, _label, color))| {
                            let prefix_bar = prefix_inner.clone();
                            let prefix_line = prefix_inner.clone();
                            let prefix_area = prefix_inner.clone();
                            let prefix_scatter = prefix_inner.clone();

                            match chart_type {
                                ChartType::Bar => {
                                    // Render bars
                                    let bar_offset = (series_idx as f64 - 0.5) * bar_width * 0.5;
                                    let prefix_bar_inner = prefix_bar.clone();
                                    view! {
                                        <g class=format!("{}-bars", prefix_bar)>
                                            {points.into_iter().map(|(x, y, _val)| {
                                                let bar_height = height - padding_bottom - y;
                                                view! {
                                                    <rect
                                                        x=x - bar_width / 4.0 + bar_offset
                                                        y=y
                                                        width=bar_width / 2.0
                                                        height=bar_height.max(0.0)
                                                        fill=color.clone()
                                                        class=format!("{}-bar", prefix_bar_inner)
                                                    />
                                                }
                                            }).collect_view()}
                                        </g>
                                    }.into_any()
                                }
                                ChartType::Line => {
                                    // Render line
                                    let path = points.iter()
                                        .enumerate()
                                        .map(|(i, (x, y, _))| {
                                            if i == 0 {
                                                format!("M {} {}", x, y)
                                            } else {
                                                format!("L {} {}", x, y)
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                        .join(" ");

                                    let points_clone = points.clone();
                                    let color_clone = color.clone();

                                    view! {
                                        <g class=format!("{}-line", prefix_line)>
                                            <path
                                                d=path
                                                fill="none"
                                                stroke=color.clone()
                                                stroke-width="2"
                                            />
                                            {if show_points {
                                                Some(points_clone.into_iter().map(|(x, y, _)| {
                                                    view! {
                                                        <circle
                                                            cx=x
                                                            cy=y
                                                            r="4"
                                                            fill=color_clone.clone()
                                                            stroke="white"
                                                            stroke-width="2"
                                                        />
                                                    }
                                                }).collect_view())
                                            } else {
                                                None
                                            }}
                                        </g>
                                    }.into_any()
                                }
                                ChartType::Area => {
                                    // Render area
                                    let mut path = points.iter()
                                        .enumerate()
                                        .map(|(i, (x, y, _))| {
                                            if i == 0 {
                                                format!("M {} {}", x, y)
                                            } else {
                                                format!("L {} {}", x, y)
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                        .join(" ");

                                    // Close the area to the bottom
                                    if let (Some(first), Some(last)) = (points.first(), points.last()) {
                                        path.push_str(&format!(
                                            " L {} {} L {} {} Z",
                                            last.0, height - padding_bottom,
                                            first.0, height - padding_bottom
                                        ));
                                    }

                                    view! {
                                        <g class=format!("{}-area", prefix_area)>
                                            <path
                                                d=path
                                                fill=color.clone()
                                                fill-opacity=fill_opacity
                                                stroke=color.clone()
                                                stroke-width="2"
                                            />
                                        </g>
                                    }.into_any()
                                }
                                ChartType::Scatter => {
                                    // Render scatter points
                                    view! {
                                        <g class=format!("{}-scatter", prefix_scatter)>
                                            {points.into_iter().map(|(x, y, _)| {
                                                view! {
                                                    <circle
                                                        cx=x
                                                        cy=y
                                                        r="5"
                                                        fill=color.clone()
                                                        stroke="white"
                                                        stroke-width="2"
                                                    />
                                                }
                                            }).collect_view()}
                                        </g>
                                    }.into_any()
                                }
                            }
                        }).collect_view()
                    }
                }
            </svg>

            // Legend
            {
                let prefix_legend = prefix_legend.clone();
                move || {
                    if show_legend {
                        let series_list = data.get();
                        let palette = colors.get();
                        let prefix_legend_inner = prefix_legend.clone();

                        Some(view! {
                            <div class=format!("{}-legend", prefix_legend)>
                                {series_list.into_iter().enumerate().map(|(idx, series)| {
                                    let color = series.color.clone().unwrap_or_else(|| {
                                        palette.color_at(idx).to_string()
                                    });
                                    let prefix_item = prefix_legend_inner.clone();
                                    let prefix_color = prefix_legend_inner.clone();
                                    let prefix_label = prefix_legend_inner.clone();

                                    view! {
                                        <div class=format!("{}-legend-item", prefix_item)>
                                            <span
                                                class=format!("{}-legend-color", prefix_color)
                                                style=format!("background-color: {}", color)
                                            ></span>
                                            <span class=format!("{}-legend-label", prefix_label)>
                                                {series.label.clone()}
                                            </span>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        })
                    } else {
                        None
                    }
                }
            }

            // Loading overlay
            {
                let prefix_loading = prefix_loading.clone();
                move || {
                    if loading {
                        let prefix_spinner = prefix_loading.clone();
                        Some(view! {
                            <div class=format!("{}-loading", prefix_loading)>
                                <div class=format!("{}-spinner", prefix_spinner)></div>
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
