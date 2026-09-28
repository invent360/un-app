use leptos::prelude::*;

use super::types::{ColorPalette, RadarChartConfig, RadarSeries};
use super::utils::{chart_prefix, try_use_theme};

/// Tooltip data for radar chart hover.
#[derive(Debug, Clone, PartialEq)]
pub struct RadarTooltipData {
    /// Series name.
    pub series_name: String,
    /// Axis label.
    pub axis_label: String,
    /// Value at this point.
    pub value: f64,
    /// Series color.
    pub color: String,
    /// Screen X position for tooltip.
    pub x: f64,
    /// Screen Y position for tooltip.
    pub y: f64,
}

/// Radar/spider chart component
///
/// Interactive radar/spider chart visualization with hover effects and tooltips.
///
/// # Props
///
/// - `data` - Radar series data
/// - `labels` - Axis labels for the radar chart
/// - `config` - Chart configuration
/// - `colors` - Optional color palette
/// - `on_point_click` - Click handler (series index, point index)
/// - `loading` - Loading state
/// - `class` - Additional CSS classes
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{RadarChart, RadarSeries, RadarPoint};
///
/// let data = vec![
///     RadarSeries::new("Team A", vec![
///         RadarPoint::new("Speed", 80.0),
///         RadarPoint::new("Power", 90.0),
///         RadarPoint::new("Skill", 85.0),
///     ]),
/// ];
///
/// view! {
///     <RadarChart
///         data=Signal::derive(move || data.clone())
///         labels=Signal::derive(|| vec!["Speed".into(), "Power".into(), "Skill".into()])
///         on_point_click=Callback::new(|(series, point)| {
///             log::info!("Clicked series {} point {}", series, point);
///         })
///     />
/// }
/// ```
#[component]
pub fn RadarChart(
    /// Radar series data
    #[prop(into)]
    data: Signal<Vec<RadarSeries>>,
    /// Axis labels for the radar chart
    #[prop(into)]
    labels: Signal<Vec<String>>,
    /// Configuration for the radar chart
    #[prop(optional)]
    config: Option<RadarChartConfig>,
    /// Optional color palette
    #[prop(optional, into)]
    colors: Option<Signal<ColorPalette>>,
    /// Click handler for data points (series_index, point_index)
    #[prop(optional, into)]
    on_point_click: Option<Callback<(usize, usize)>>,
    /// Whether the chart is in a loading state
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme = try_use_theme();
    let chart_prefix_str = chart_prefix("radar");
    let prefix_class = chart_prefix_str.clone();
    let prefix_svg = format!("{}-svg", chart_prefix_str);
    let prefix_series = format!("{}-series", chart_prefix_str);
    let prefix_legend = format!("{}-legend", chart_prefix_str);
    let prefix_legend_item = format!("{}-legend-item", chart_prefix_str);
    let prefix_legend_color = format!("{}-legend-color", chart_prefix_str);
    let prefix_legend_label = format!("{}-legend-label", chart_prefix_str);
    let prefix_loading = format!("{}-loading", chart_prefix_str);
    let prefix_spinner = format!("{}-spinner", chart_prefix_str);
    let prefix_tooltip = format!("{}-tooltip", chart_prefix_str);
    let prefix_point = format!("{}-point", chart_prefix_str);

    let config = config.unwrap_or_default();

    let default_colors = ColorPalette::default();
    let colors = colors.unwrap_or_else(|| Signal::stored(default_colors));

    // Interactive state signals
    // Hovered point: (series_index, point_index)
    let hovered_point = RwSignal::new(None::<(usize, usize)>);
    // Hovered series (from legend or polygon)
    let hovered_series = RwSignal::new(None::<usize>);
    // Tooltip data
    let tooltip_data = RwSignal::new(None::<RadarTooltipData>);

    // Chart dimensions
    let size = 400.0;
    let center = size / 2.0;
    let radius = 150.0;

    // Store config values for closures
    let fill_opacity = config.fill_opacity;
    let show_legend = config.show_legend;

    // Calculate radar chart geometry
    let radar_data = Memo::new(move |_| {
        let series_list = data.get();
        let axis_labels = labels.get();
        let palette = colors.get();

        let num_axes = axis_labels.len();
        if num_axes == 0 {
            return (Vec::new(), Vec::new(), Vec::new(), axis_labels.clone(), 0.0_f64);
        }

        let angle_step = 2.0 * std::f64::consts::PI / num_axes as f64;

        // Calculate max value from all series data
        let max_value = series_list.iter()
            .flat_map(|s| s.data.iter().map(|p| p.value))
            .fold(0.0_f64, |max, v| max.max(v));

        // Grid rings (5 by default)
        let num_rings = 5_usize;
        let rings: Vec<Vec<(f64, f64)>> = (1..=num_rings)
            .map(|ring| {
                let ring_radius = radius * (ring as f64 / num_rings as f64);
                (0..num_axes)
                    .map(|i| {
                        let angle = -std::f64::consts::PI / 2.0 + (i as f64 * angle_step);
                        let x = center + ring_radius * angle.cos();
                        let y = center + ring_radius * angle.sin();
                        (x, y)
                    })
                    .collect()
            })
            .collect();

        // Axis lines and labels
        let axes: Vec<(f64, f64, f64, f64, String)> = (0..num_axes)
            .map(|i| {
                let angle = -std::f64::consts::PI / 2.0 + (i as f64 * angle_step);
                let end_x = center + radius * angle.cos();
                let end_y = center + radius * angle.sin();
                let label_x = center + (radius + 20.0) * angle.cos();
                let label_y = center + (radius + 20.0) * angle.sin();
                let label = axis_labels.get(i).cloned().unwrap_or_default();
                (end_x, end_y, label_x, label_y, label)
            })
            .collect();

        // Series polygons with point data including values for tooltips
        // Format: (points_with_values, series_name, color, fill_opacity, series_index)
        // points_with_values: Vec<(x, y, value, point_index)>
        let series_polygons: Vec<(Vec<(f64, f64, f64, usize)>, String, String, f32, usize)> = series_list
            .iter()
            .enumerate()
            .map(|(idx, series)| {
                let points: Vec<(f64, f64, f64, usize)> = series.data
                    .iter()
                    .enumerate()
                    .map(|(i, point)| {
                        let normalized = if max_value > 0.0 { (point.value / max_value).min(1.0) } else { 0.0 };
                        let angle = -std::f64::consts::PI / 2.0 + (i as f64 * angle_step);
                        let point_radius = radius * normalized;
                        let x = center + point_radius * angle.cos();
                        let y = center + point_radius * angle.sin();
                        (x, y, point.value, i)
                    })
                    .collect();

                let color = palette.color_at(idx).to_string();

                (points, series.name.clone(), color, fill_opacity, idx)
            })
            .collect();

        (rings, axes, series_polygons, axis_labels.clone(), max_value)
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
        <div class=combined_class style="position: relative;">
            <svg viewBox=format!("0 0 {} {}", size, size) class=prefix_svg>
                // Grid rings
                {move || {
                    let (rings, _, _, _, _) = radar_data.get();
                    rings.into_iter().map(|ring_points| {
                        let path = ring_points.iter()
                            .enumerate()
                            .map(|(i, (x, y))| {
                                if i == 0 {
                                    format!("M {} {}", x, y)
                                } else {
                                    format!("L {} {}", x, y)
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" ") + " Z";

                        view! {
                            <path
                                d=path
                                fill="none"
                                stroke=move || if theme.is_some() { "var(--chart-grid)" } else { "#e5e7eb" }
                                stroke-width="1"
                            />
                        }
                    }).collect_view()
                }}

                // Axis lines
                {move || {
                    let (_, axes, _, _, _) = radar_data.get();
                    axes.iter().map(|(end_x, end_y, _, _, _)| {
                        view! {
                            <line
                                x1=center
                                y1=center
                                x2=*end_x
                                y2=*end_y
                                stroke=move || if theme.is_some() { "var(--chart-grid)" } else { "#d1d5db" }
                                stroke-width="1"
                            />
                        }
                    }).collect_view()
                }}

                // Axis labels
                {move || {
                    let (_, axes, _, _, _) = radar_data.get();
                    axes.into_iter().map(|(_, _, label_x, label_y, label)| {
                        view! {
                            <text
                                x=label_x
                                y=label_y
                                text-anchor="middle"
                                dominant-baseline="middle"
                                fill=move || if theme.is_some() { "var(--chart-text)" } else { "#374151" }
                                font-size="12"
                            >
                                {label}
                            </text>
                        }
                    }).collect_view()
                }}

                // Series polygons with interactivity
                {
                    let series_class = prefix_series.clone();
                    let point_class = prefix_point.clone();
                    let on_click = on_point_click.clone();
                    move || {
                        let (_, _, series_polygons, axis_labels, _) = radar_data.get();
                        let series_class = series_class.clone();
                        let point_class = point_class.clone();
                        let on_click = on_click.clone();
                        let axis_labels = axis_labels.clone();

                        series_polygons.into_iter().map({
                            let series_class = series_class.clone();
                            let point_class = point_class.clone();
                            let on_click = on_click.clone();
                            let axis_labels = axis_labels.clone();
                            move |(points, series_name, color, series_fill_opacity, series_idx)| {
                                // Build polygon path from points
                                let path = points.iter()
                                    .enumerate()
                                    .map(|(i, (x, y, _, _))| {
                                        if i == 0 {
                                            format!("M {} {}", x, y)
                                        } else {
                                            format!("L {} {}", x, y)
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .join(" ") + " Z";

                                let points_clone = points.clone();
                                let color_clone = color.clone();
                                let series_name_clone = series_name.clone();

                                // Determine polygon opacity based on hover state
                                let polygon_opacity = move || {
                                    match (hovered_series.get(), hovered_point.get()) {
                                        // If a series is hovered (from legend), dim others
                                        (Some(h), _) if h == series_idx => series_fill_opacity,
                                        (Some(_), _) => series_fill_opacity * 0.3,
                                        // If a point is hovered, brighten that series
                                        (None, Some((s, _))) if s == series_idx => series_fill_opacity * 1.2,
                                        (None, Some(_)) => series_fill_opacity * 0.5,
                                        // Default
                                        _ => series_fill_opacity,
                                    }
                                };

                                let stroke_opacity = move || {
                                    match (hovered_series.get(), hovered_point.get()) {
                                        (Some(h), _) if h == series_idx => 1.0,
                                        (Some(_), _) => 0.3,
                                        (None, Some((s, _))) if s == series_idx => 1.0,
                                        (None, Some(_)) => 0.5,
                                        _ => 1.0,
                                    }
                                };

                                view! {
                                    <g class=series_class.clone()
                                       style="transition: opacity 0.2s ease;"
                                    >
                                        // Filled polygon area
                                        <path
                                            d=path.clone()
                                            fill=color.clone()
                                            fill-opacity=polygon_opacity
                                            stroke=color.clone()
                                            stroke-width="2"
                                            stroke-opacity=stroke_opacity
                                            style="transition: fill-opacity 0.2s ease, stroke-opacity 0.2s ease; cursor: pointer;"
                                            on:mouseenter=move |_| {
                                                hovered_series.set(Some(series_idx));
                                            }
                                            on:mouseleave=move |_| {
                                                hovered_series.set(None);
                                            }
                                        />

                                        // Interactive data points
                                        {
                                            let point_class = point_class.clone();
                                            let color = color_clone.clone();
                                            let series_name = series_name_clone.clone();
                                            let on_click = on_click.clone();
                                            let axis_labels = axis_labels.clone();
                                            points_clone.into_iter().map({
                                                let point_class = point_class.clone();
                                                let color = color.clone();
                                                let series_name = series_name.clone();
                                                let on_click = on_click.clone();
                                                let axis_labels = axis_labels.clone();
                                                move |(px, py, value, point_idx)| {
                                                    let color = color.clone();
                                                    let series_name = series_name.clone();
                                                    let axis_label = axis_labels.get(point_idx).cloned().unwrap_or_default();
                                                    let on_click = on_click.clone();
                                                    let color_for_tooltip = color.clone();
                                                    let series_name_for_tooltip = series_name.clone();
                                                    let axis_label_for_tooltip = axis_label.clone();

                                                    // Point scale based on hover
                                                    let point_radius = move || {
                                                        match hovered_point.get() {
                                                            Some((s, p)) if s == series_idx && p == point_idx => 7.0,
                                                            _ => 4.0,
                                                        }
                                                    };

                                                    let point_stroke_width = move || {
                                                        match hovered_point.get() {
                                                            Some((s, p)) if s == series_idx && p == point_idx => 3.0,
                                                            _ => 2.0,
                                                        }
                                                    };

                                                    view! {
                                                        <circle
                                                            cx=px
                                                            cy=py
                                                            r=point_radius
                                                            fill=color.clone()
                                                            stroke="white"
                                                            stroke-width=point_stroke_width
                                                            class=point_class.clone()
                                                            style="transition: r 0.15s ease, stroke-width 0.15s ease; cursor: pointer;"
                                                            on:mouseenter=move |_| {
                                                                hovered_point.set(Some((series_idx, point_idx)));
                                                                hovered_series.set(Some(series_idx));
                                                                tooltip_data.set(Some(RadarTooltipData {
                                                                    series_name: series_name_for_tooltip.clone(),
                                                                    axis_label: axis_label_for_tooltip.clone(),
                                                                    value,
                                                                    color: color_for_tooltip.clone(),
                                                                    x: px,
                                                                    y: py,
                                                                }));
                                                            }
                                                            on:mouseleave=move |_| {
                                                                hovered_point.set(None);
                                                                hovered_series.set(None);
                                                                tooltip_data.set(None);
                                                            }
                                                            on:click=move |_| {
                                                                if let Some(ref callback) = on_click {
                                                                    callback.run((series_idx, point_idx));
                                                                }
                                                            }
                                                        />
                                                    }
                                                }
                                            }).collect_view()
                                        }
                                    </g>
                                }
                            }
                        }).collect_view()
                    }
                }
            </svg>

            // Tooltip
            {
                let tooltip_class = prefix_tooltip.clone();
                move || {
                    tooltip_data.get().map(|data| {
                        view! {
                            <div
                                class=tooltip_class.clone()
                                style=format!(
                                    "position: absolute; left: {}px; top: {}px; \
                                    transform: translate(-50%, -100%) translateY(-12px); \
                                    background: var(--fx-color-bg-elevated, #1f1f1f); \
                                    border: 1px solid var(--fx-color-border, #303030); \
                                    border-radius: 6px; padding: 8px 12px; \
                                    font-size: 12px; pointer-events: none; z-index: 100; \
                                    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3); \
                                    white-space: nowrap;",
                                    data.x, data.y
                                )
                            >
                                <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 4px;">
                                    <span style=format!(
                                        "width: 10px; height: 10px; border-radius: 2px; background: {};",
                                        data.color
                                    ) />
                                    <span style="color: var(--fx-color-text, #fff); font-weight: 500;">
                                        {data.series_name.clone()}
                                    </span>
                                </div>
                                <div style="color: var(--fx-color-text-secondary, #8c8c8c);">
                                    {data.axis_label.clone()}": "
                                    <span style="color: var(--fx-color-text, #fff); font-weight: 500;">
                                        {format!("{:.1}", data.value)}
                                    </span>
                                </div>
                            </div>
                        }
                    })
                }
            }

            // Interactive Legend
            {
                let legend_class = prefix_legend.clone();
                let item_class = prefix_legend_item.clone();
                let color_class = prefix_legend_color.clone();
                let label_class = prefix_legend_label.clone();
                move || {
                    if show_legend {
                        let series_list = data.get();
                        let palette = colors.get();
                        let legend_class = legend_class.clone();
                        let item_class = item_class.clone();
                        let color_class = color_class.clone();
                        let label_class = label_class.clone();

                        Some(view! {
                            <div
                                class=legend_class
                                style="display: flex; flex-wrap: wrap; justify-content: center; gap: 12px; padding: 8px; font-size: 12px;"
                            >
                                {series_list.into_iter().enumerate().map({
                                    let item_class = item_class.clone();
                                    let color_class = color_class.clone();
                                    let label_class = label_class.clone();
                                    move |(idx, series)| {
                                        let color = palette.color_at(idx).to_string();
                                        let is_hovered = move || hovered_series.get() == Some(idx);

                                        view! {
                                            <div
                                                class=item_class.clone()
                                                style=move || format!(
                                                    "display: flex; align-items: center; gap: 6px; \
                                                    color: var(--fx-color-text-secondary, #8c8c8c); \
                                                    cursor: pointer; padding: 4px 8px; border-radius: 4px; \
                                                    transition: all 0.2s ease; {}",
                                                    if is_hovered() {
                                                        "background: var(--fx-color-bg-elevated, #1f1f1f); \
                                                        color: var(--fx-color-text, #fff);"
                                                    } else { "" }
                                                )
                                                on:mouseenter=move |_| {
                                                    hovered_series.set(Some(idx));
                                                }
                                                on:mouseleave=move |_| {
                                                    hovered_series.set(None);
                                                }
                                            >
                                                <span
                                                    class=color_class.clone()
                                                    style=format!(
                                                        "width: 10px; height: 10px; border-radius: 2px; background: {};",
                                                        color
                                                    )
                                                ></span>
                                                <span class=label_class.clone()>
                                                    {series.name.clone()}
                                                </span>
                                            </div>
                                        }
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
