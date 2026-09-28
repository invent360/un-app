//! AreaChart Leptos component.

use leptos::prelude::*;
use super::types::{AreaSeries, AreaChartConfig, ChartColor, DataPoint};
use crate::try_use_theme;

/// Calculated point position for rendering in an area chart.
#[derive(Clone, Debug)]
struct AreaPointPosition {
    /// Series index.
    series_index: usize,
    /// Point index within the series.
    point_index: usize,
    /// Original data point.
    point: DataPoint,
    /// X coordinate in SVG space.
    x: f64,
    /// Y coordinate in SVG space.
    y: f64,
    /// Series name.
    series_name: String,
    /// Series color.
    color: ChartColor,
}

/// AreaChart component.
///
/// Stacked or layered area chart for multiple series with full interactivity.
///
/// # Props
///
/// - `data` - Multiple area series
/// - `config` - Chart configuration
/// - `on_point_click` - Click handler for data points (series_index, point_index, DataPoint)
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{AreaChart, AreaSeries, DataPoint, ChartColor};
///
/// let series = vec![
///     AreaSeries::new("CPU", vec![...]).color(ChartColor::Primary),
///     AreaSeries::new("Memory", vec![...]).color(ChartColor::Success),
/// ];
///
/// view! {
///     <AreaChart data=Signal::derive(move || series.clone()) stacked=true />
/// }
/// ```
#[component]
pub fn AreaChart(
    /// Area series data.
    data: Signal<Vec<AreaSeries>>,
    /// Chart configuration.
    #[prop(optional)]
    config: Option<AreaChartConfig>,
    /// Stack areas.
    #[prop(optional)]
    stacked: Option<bool>,
    /// Click handler for data points (series_index, point_index, DataPoint).
    #[prop(optional, into)]
    on_point_click: Option<Callback<(usize, usize, DataPoint)>>,
    /// Loading state.
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve config
    let config = config.unwrap_or_default();
    let stacked = stacked.unwrap_or(config.stacked);
    let chart_height = config.height;
    let fill_opacity = config.fill_opacity;
    let show_points = config.show_points;

    // Interactive state signals
    // (series_index, point_index)
    let hovered_point = RwSignal::new(None::<(usize, usize)>);
    // Hovered series index (from legend or point hover)
    let hovered_series = RwSignal::new(None::<usize>);
    // Tooltip data: (series_name, timestamp, value, x_position, y_position, color)
    let tooltip_data = RwSignal::new(None::<(String, i64, f64, f64, f64, String)>);
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    // Build CSS classes
    let chart_prefix = format!("fx-chart-{}", design_system);

    let combined_class = {
        let mut parts = vec![chart_prefix.clone(), format!("{}-area", chart_prefix)];
        if stacked {
            parts.push(format!("{}-stacked", chart_prefix));
        }
        if loading {
            parts.push(format!("{}-loading", chart_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Chart dimensions
    let padding = 40.0;
    let chart_width = 600.0;
    let inner_width = chart_width - padding * 2.0;
    let inner_height = chart_height as f64 - padding * 2.0;

    // Calculate bounds for coordinate transforms
    let bounds = move || {
        let series_list = data.get();
        if series_list.is_empty() {
            return (0i64, 1i64, 0.001f64);
        }

        let all_points: Vec<_> = series_list.iter().flat_map(|s| &s.data).collect();
        if all_points.is_empty() {
            return (0i64, 1i64, 0.001f64);
        }

        let min_time = all_points.iter().map(|p| p.timestamp).min().unwrap_or(0);
        let max_time = all_points.iter().map(|p| p.timestamp).max().unwrap_or(1);

        // For stacked charts, we need to calculate cumulative values
        let max_val = if stacked {
            // Find max sum at any time point (simplified)
            series_list.iter()
                .flat_map(|s| &s.data)
                .map(|p| p.value)
                .sum::<f64>()
                .max(0.001)
        } else {
            all_points.iter()
                .map(|p| p.value)
                .fold(f64::NEG_INFINITY, f64::max)
                .max(0.001)
        };

        (min_time, max_time, max_val)
    };

    // Calculate all point positions for interactivity
    let point_positions = move || {
        let series_list = data.get();
        if series_list.is_empty() {
            return vec![];
        }

        let (min_time, max_time, max_val) = bounds();
        let time_range = (max_time - min_time).max(1) as f64;

        let mut positions = vec![];

        for (series_idx, series) in series_list.iter().enumerate() {
            for (point_idx, point) in series.data.iter().enumerate() {
                let x = padding + ((point.timestamp - min_time) as f64 / time_range) * inner_width;
                let y = padding + inner_height - (point.value / max_val) * inner_height;

                positions.push(AreaPointPosition {
                    series_index: series_idx,
                    point_index: point_idx,
                    point: point.clone(),
                    x,
                    y,
                    series_name: series.name.clone(),
                    color: series.color,
                });
            }
        }

        positions
    };

    // Calculate all paths
    let paths = move || {
        let series_list = data.get();
        if series_list.is_empty() {
            return vec![];
        }

        let (min_time, max_time, max_val) = bounds();
        let time_range = (max_time - min_time).max(1) as f64;
        let baseline_y = padding + inner_height;

        series_list.iter().enumerate().map(|(series_idx, series)| {
            if series.data.is_empty() {
                return (series_idx, String::new(), String::new(), series.color, series.name.clone());
            }

            // Build area path
            let mut area_path = String::new();
            let mut line_path = String::new();

            // For stacked, we'd need cumulative values from previous series
            // Simplified: each series is independent here
            let points = &series.data;

            // Start at baseline
            if let Some(first) = points.first() {
                let x = padding + ((first.timestamp - min_time) as f64 / time_range) * inner_width;
                area_path.push_str(&format!("M {} {}", x, baseline_y));
            }

            // Draw top line
            for (i, point) in points.iter().enumerate() {
                let x = padding + ((point.timestamp - min_time) as f64 / time_range) * inner_width;
                let y = padding + inner_height - (point.value / max_val) * inner_height;

                area_path.push_str(&format!(" L {} {}", x, y));

                if i == 0 {
                    line_path.push_str(&format!("M {} {}", x, y));
                } else {
                    line_path.push_str(&format!(" L {} {}", x, y));
                }
            }

            // Close area at baseline
            if let Some(last) = points.last() {
                let x = padding + ((last.timestamp - min_time) as f64 / time_range) * inner_width;
                area_path.push_str(&format!(" L {} {} Z", x, baseline_y));
            }

            (series_idx, area_path, line_path, series.color, series.name.clone())
        }).collect()
    };

    // Legend items
    let legend_items = move || {
        if !config.show_legend {
            return vec![];
        }
        data.get().iter().enumerate().map(|(i, s)| (i, s.name.clone(), s.color)).collect()
    };

    // Grid lines
    let grid_lines = move || {
        if !config.show_grid {
            return vec![];
        }

        let mut lines = vec![];
        for i in 0..=4 {
            let y = padding + (inner_height / 4.0) * i as f64;
            lines.push((padding, y, padding + inner_width, y));
        }
        for i in 0..=5 {
            let x = padding + (inner_width / 5.0) * i as f64;
            lines.push((x, padding, x, padding + inner_height));
        }
        lines
    };

    // Clone chart_prefix for different closures
    let svg_class = format!("{}-svg", chart_prefix);
    let series_class = format!("{}-series", chart_prefix);
    let area_fill_class = format!("{}-area-fill", chart_prefix);
    let area_line_class = format!("{}-area-line", chart_prefix);
    let loading_overlay_class = format!("{}-loading-overlay", chart_prefix);
    let loading_spinner_class = format!("{}-loading-spinner", chart_prefix);

    let point_radius = 5.0_f64;

    view! {
        <div class=combined_class style="position: relative;" node_ref=container_ref>
            <svg
                width="100%"
                height=chart_height
                viewBox=format!("0 0 {} {}", chart_width, chart_height)
                preserveAspectRatio="xMidYMid meet"
                class=svg_class
            >
                // Grid lines
                {move || grid_lines().into_iter().map(|(x1, y1, x2, y2)| {
                    view! {
                        <line
                            x1=x1
                            y1=y1
                            x2=x2
                            y2=y2
                            stroke="var(--fx-color-border, #303030)"
                            stroke-width="1"
                            stroke-dasharray="2,2"
                        />
                    }
                }).collect::<Vec<_>>()}

                // Area paths (rendered in reverse for proper layering)
                {move || {
                    let series_class = series_class.clone();
                    let area_fill_class = area_fill_class.clone();
                    let area_line_class = area_line_class.clone();
                    paths().into_iter().rev().map(move |(series_idx, area_path, line_path, color, _name)| {
                        let fill_color = color.as_css();
                        let stroke_color = color.as_css();

                        // Determine opacity based on hover state
                        let area_opacity = move || {
                            match hovered_series.get() {
                                Some(h) if h != series_idx => fill_opacity * 0.4,
                                _ => fill_opacity,
                            }
                        };
                        let line_opacity = move || {
                            match hovered_series.get() {
                                Some(h) if h != series_idx => 0.4,
                                _ => 1.0,
                            }
                        };

                        view! {
                            <g class=series_class.clone()>
                                <path
                                    d=area_path
                                    fill=fill_color
                                    style=move || format!(
                                        "fill-opacity: {}; transition: fill-opacity 0.2s ease;",
                                        area_opacity()
                                    )
                                    class=area_fill_class.clone()
                                />
                                <path
                                    d=line_path
                                    fill="none"
                                    stroke=stroke_color
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    style=move || format!(
                                        "opacity: {}; transition: opacity 0.2s ease;",
                                        line_opacity()
                                    )
                                    class=area_line_class.clone()
                                />
                            </g>
                        }
                    }).collect::<Vec<_>>()
                }}

                // Vertical crosshair line at hovered position
                {move || {
                    tooltip_data.get().map(|(_, _, _, x, _, _)| {
                        view! {
                            <line
                                x1=x
                                y1=padding
                                x2=x
                                y2=padding + inner_height
                                stroke="var(--fx-color-text-tertiary, #595959)"
                                stroke-width="1"
                                stroke-dasharray="4,4"
                                style="pointer-events: none; transition: opacity 0.15s ease;"
                            />
                        }
                    })
                }}

                // Interactive data points
                {
                    let on_click = on_point_click.clone();
                    move || {
                        let on_click = on_click.clone();
                        point_positions().into_iter().map({
                            let on_click = on_click.clone();
                            move |pos| {
                                let series_idx = pos.series_index;
                                let point_idx = pos.point_index;
                                let point_for_click = pos.point.clone();
                                let x = pos.x;
                                let y = pos.y;
                                let timestamp = pos.point.timestamp;
                                let value = pos.point.value;
                                let series_name = pos.series_name.clone();
                                let color = pos.color;
                                let color_css = color.as_css().to_string();
                                let on_click = on_click.clone();

                                let is_hovered = move || hovered_point.get() == Some((series_idx, point_idx));
                                let is_series_hovered = move || {
                                    hovered_series.get() == Some(series_idx) || is_hovered()
                                };

                                let radius = move || {
                                    if is_hovered() {
                                        point_radius * 1.5
                                    } else if is_series_hovered() {
                                        point_radius
                                    } else {
                                        point_radius * 0.7
                                    }
                                };
                                let opacity = move || {
                                    if is_hovered() {
                                        1.0
                                    } else if is_series_hovered() {
                                        if show_points { 0.9 } else { 0.7 }
                                    } else if show_points {
                                        match hovered_series.get() {
                                            Some(_) => 0.3,
                                            None => 0.7,
                                        }
                                    } else {
                                        0.0
                                    }
                                };

                                let stroke_color = color.as_css();

                                view! {
                                    <circle
                                        cx=x
                                        cy=y
                                        r=radius
                                        fill=stroke_color
                                        stroke="var(--fx-color-bg, #141414)"
                                        stroke-width="2"
                                        style=move || format!(
                                            "cursor: pointer; opacity: {}; transition: r 0.15s ease, opacity 0.15s ease;",
                                            opacity()
                                        )
                                        on:mouseenter={
                                            let series_name = series_name.clone();
                                            let color_css = color_css.clone();
                                            move |ev| {
                                                hovered_point.set(Some((series_idx, point_idx)));
                                                hovered_series.set(Some(series_idx));
                                                tooltip_data.set(Some((
                                                    series_name.clone(),
                                                    timestamp,
                                                    value,
                                                    x,
                                                    y,
                                                    color_css.clone(),
                                                )));
                                                // Get mouse position relative to container
                                                if let Some(container) = container_ref.get() {
                                                    let rect = container.get_bounding_client_rect();
                                                    let mx = ev.client_x() as f64 - rect.left();
                                                    let my = ev.client_y() as f64 - rect.top();
                                                    mouse_pos.set((mx, my));
                                                }
                                            }
                                        }
                                        on:mousemove=move |ev| {
                                            // Update mouse position on move for smooth tracking
                                            if let Some(container) = container_ref.get() {
                                                let rect = container.get_bounding_client_rect();
                                                let mx = ev.client_x() as f64 - rect.left();
                                                let my = ev.client_y() as f64 - rect.top();
                                                mouse_pos.set((mx, my));
                                            }
                                        }
                                        on:mouseleave=move |_| {
                                            hovered_point.set(None);
                                            hovered_series.set(None);
                                            tooltip_data.set(None);
                                        }
                                        on:click={
                                            let on_click = on_click.clone();
                                            let point = point_for_click.clone();
                                            move |_| {
                                                if let Some(ref callback) = on_click {
                                                    callback.run((series_idx, point_idx, point.clone()));
                                                }
                                            }
                                        }
                                    />
                                }
                            }
                        }).collect::<Vec<_>>()
                    }
                }
            </svg>

            // Tooltip
            {
            let chart_prefix_tooltip = chart_prefix.clone();
            move || {
                tooltip_data.get().map(|(series_name, timestamp, value, _data_x, _data_y, color)| {
                    let formatted_time = format_timestamp(timestamp);
                    // Use actual mouse position for tooltip placement
                    let (mx, my) = mouse_pos.get();
                    let tooltip_x = mx + 15.0;
                    let tooltip_y = my;

                    view! {
                        <div
                            class=format!("{}-tooltip", chart_prefix_tooltip)
                            style=format!(
                                "position: absolute; left: {}px; top: {}px; background: var(--fx-color-bg-elevated, #1f1f1f); border: 1px solid var(--fx-color-border, #303030); border-radius: 6px; padding: 8px 12px; font-size: 12px; pointer-events: none; z-index: 10; box-shadow: 0 2px 8px rgba(0,0,0,0.3); transform: translateY(-50%);",
                                tooltip_x, tooltip_y
                            )
                        >
                            // Series name
                            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 4px;">
                                <span style=format!("width: 10px; height: 10px; border-radius: 2px; background: {};", color) />
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500;">
                                    {series_name}
                                </span>
                            </div>
                            // Timestamp
                            <div style="color: var(--fx-color-text-secondary, #8c8c8c); margin-bottom: 2px;">
                                {formatted_time}
                            </div>
                            // Value
                            <div style="color: var(--fx-color-text, #fff);">
                                {format!("Value: {:.2}", value)}
                            </div>
                        </div>
                    }
                })
            }}

            // Legend with interactivity
            {
            let chart_prefix_legend = chart_prefix.clone();
            move || {
                let items = legend_items();
                let items_clone = items.clone();
                let prefix = chart_prefix_legend.clone();
                (!items.is_empty()).then(move || view! {
                    <div
                        class=format!("{}-legend", prefix)
                        style="display: flex; flex-wrap: wrap; justify-content: center; gap: 12px; padding: 8px; font-size: 12px;"
                    >
                        {items_clone.into_iter().map(|(idx, name, color)| {
                            let color_css = color.as_css();
                            let is_hovered = move || hovered_series.get() == Some(idx);

                            view! {
                                <span
                                    style=move || format!(
                                        "display: flex; align-items: center; gap: 6px; color: var(--fx-color-text-secondary, #8c8c8c); cursor: pointer; padding: 2px 6px; border-radius: 4px; transition: all 0.2s ease; {}",
                                        if is_hovered() { "background: var(--fx-color-bg-elevated, #1f1f1f);" } else { "" }
                                    )
                                    on:mouseenter=move |_| {
                                        hovered_series.set(Some(idx));
                                    }
                                    on:mouseleave=move |_| {
                                        hovered_series.set(None);
                                    }
                                >
                                    <span style=format!(
                                        "width: 10px; height: 10px; border-radius: 2px; background: {};",
                                        color_css
                                    ) />
                                    {name}
                                </span>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                })
            }
            }

            // Loading overlay
            {loading.then(|| view! {
                <div class=loading_overlay_class>
                    <span class=loading_spinner_class></span>
                </div>
            })}
        </div>
    }
}

/// Format a Unix timestamp in milliseconds to a human-readable string.
fn format_timestamp(timestamp_ms: i64) -> String {
    // Convert milliseconds to seconds
    let secs = timestamp_ms / 1000;

    // Calculate date/time components
    // This is a simplified implementation - in production you'd use chrono
    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Calculate year, month, day (simplified - doesn't account for all leap years perfectly)
    let mut remaining_days = days_since_epoch;
    let mut year = 1970i64;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let days_in_months: [i64; 12] = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for days in days_in_months.iter() {
        if remaining_days < *days {
            break;
        }
        remaining_days -= *days;
        month += 1;
    }
    let day = remaining_days + 1;

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

/// Check if a year is a leap year.
fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
