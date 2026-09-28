//! LineChart Leptos component.

use leptos::prelude::*;
use super::types::{DataPoint, LineChartConfig, ChartColor};
use crate::try_use_theme;
use web_sys::wasm_bindgen::JsCast;

/// Calculated point position for rendering.
#[derive(Clone, Debug)]
struct PointPosition {
    /// Index in the data array.
    index: usize,
    /// Original data point.
    point: DataPoint,
    /// X coordinate in SVG space.
    x: f64,
    /// Y coordinate in SVG space.
    y: f64,
}

/// LineChart component.
///
/// Time-series line chart visualization using SVG with full interactivity.
///
/// # Props
///
/// - `data` - Time-series data points
/// - `config` - Chart configuration
/// - `y_label` - Y-axis label
/// - `on_point_click` - Click handler for data points
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{LineChart, DataPoint};
///
/// let data = vec![
///     DataPoint::new(1716000000000, 45.0),
///     DataPoint::new(1716003600000, 52.0),
///     DataPoint::new(1716007200000, 48.0),
/// ];
///
/// view! {
///     <LineChart data=Signal::derive(move || data.clone()) />
/// }
/// ```
#[component]
pub fn LineChart(
    /// Time-series data points.
    data: Signal<Vec<DataPoint>>,
    /// Chart configuration.
    #[prop(optional)]
    config: Option<LineChartConfig>,
    /// Y-axis label.
    #[prop(optional, into)]
    y_label: Option<String>,
    /// X-axis label.
    #[prop(optional, into)]
    x_label: Option<String>,
    /// Click handler for data points.
    #[prop(optional, into)]
    on_point_click: Option<Callback<DataPoint>>,
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
    let chart_height = config.height;

    // Interactive state signals
    let hovered_point = RwSignal::new(None::<usize>);
    // Tooltip data: (timestamp, value, data_x, data_y, label)
    let tooltip_data = RwSignal::new(None::<(i64, f64, f64, f64, Option<String>)>);
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    // Build CSS classes
    let chart_prefix = format!("fx-chart-{}", design_system);

    let combined_class = {
        let mut parts = vec![chart_prefix.clone(), format!("{}-line", chart_prefix)];
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

    // Calculate point positions and path from data
    let point_positions = move || {
        let points = data.get();
        if points.is_empty() {
            return vec![];
        }

        // Calculate bounds
        let min_time = points.iter().map(|p| p.timestamp).min().unwrap_or(0);
        let max_time = points.iter().map(|p| p.timestamp).max().unwrap_or(1);
        let time_range = (max_time - min_time).max(1) as f64;

        let min_val = config.y_min.unwrap_or_else(|| {
            points.iter().map(|p| p.value).fold(f64::INFINITY, f64::min)
        });
        let max_val = config.y_max.unwrap_or_else(|| {
            points.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max)
        });
        let val_range = (max_val - min_val).max(0.001);

        points
            .iter()
            .enumerate()
            .map(|(i, point)| {
                let x = padding + ((point.timestamp - min_time) as f64 / time_range) * inner_width;
                let y = padding + inner_height - ((point.value - min_val) / val_range) * inner_height;
                PointPosition {
                    index: i,
                    point: point.clone(),
                    x,
                    y,
                }
            })
            .collect::<Vec<_>>()
    };

    // Calculate path from point positions
    let path = move || {
        let positions = point_positions();
        if positions.is_empty() {
            return String::new();
        }

        let mut path = String::new();
        for (i, pos) in positions.iter().enumerate() {
            if i == 0 {
                path.push_str(&format!("M {} {}", pos.x, pos.y));
            } else {
                path.push_str(&format!(" L {} {}", pos.x, pos.y));
            }
        }
        path
    };

    // Calculate fill path (area under curve) using point positions
    let fill_path = move || {
        if !config.fill {
            return String::new();
        }

        let positions = point_positions();
        if positions.is_empty() {
            return String::new();
        }

        let baseline_y = padding + inner_height;
        let mut path = String::new();

        // Start at bottom-left
        if let Some(first) = positions.first() {
            path.push_str(&format!("M {} {}", first.x, baseline_y));
        }

        // Draw line along data points
        for pos in &positions {
            path.push_str(&format!(" L {} {}", pos.x, pos.y));
        }

        // Close at bottom-right
        if let Some(last) = positions.last() {
            path.push_str(&format!(" L {} {} Z", last.x, baseline_y));
        }

        path
    };

    // Grid lines
    let grid_lines = move || {
        if !config.show_grid {
            return vec![];
        }

        let mut lines = vec![];
        // Horizontal grid lines (5 lines)
        for i in 0..=4 {
            let y = padding + (inner_height / 4.0) * i as f64;
            lines.push((padding, y, padding + inner_width, y));
        }
        // Vertical grid lines (6 lines)
        for i in 0..=5 {
            let x = padding + (inner_width / 5.0) * i as f64;
            lines.push((x, padding, x, padding + inner_height));
        }
        lines
    };

    let stroke_color = config.color.as_css();
    let fill_color = config.color.as_fill_css();
    let line_width = config.line_width;
    let point_radius = 4.0_f64;
    let show_points = config.show_points;

    view! {
        <div class=combined_class style="position: relative;" node_ref=container_ref>
            <svg
                width="100%"
                height=chart_height
                viewBox=format!("0 0 {} {}", chart_width, chart_height)
                preserveAspectRatio="xMidYMid meet"
                class=format!("{}-svg", chart_prefix)
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

                // Fill area
                <path
                    d=fill_path
                    fill=fill_color
                    class=format!("{}-fill", chart_prefix)
                />

                // Line
                <path
                    d=path
                    fill="none"
                    stroke=stroke_color
                    stroke-width=line_width
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class=format!("{}-line-path", chart_prefix)
                />

                // Vertical crosshair line at hovered position
                {move || {
                    tooltip_data.get().map(|(_, _, x, _, _)| {
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
                    let stroke = stroke_color;
                    move || {
                        let on_click = on_click.clone();
                        point_positions().into_iter().map({
                            let on_click = on_click.clone();
                            move |pos| {
                                let idx = pos.index;
                                let point_for_click = pos.point.clone();
                                let x = pos.x;
                                let y = pos.y;
                                let timestamp = pos.point.timestamp;
                                let value = pos.point.value;
                                let label = pos.point.label.clone();
                                let on_click = on_click.clone();

                                let is_hovered = move || hovered_point.get() == Some(idx);
                                let radius = move || {
                                    if is_hovered() {
                                        point_radius * 1.5
                                    } else if show_points {
                                        point_radius
                                    } else {
                                        point_radius * 0.5
                                    }
                                };
                                let opacity = move || {
                                    if is_hovered() || show_points {
                                        1.0
                                    } else {
                                        0.0
                                    }
                                };

                                view! {
                                    <circle
                                        cx=x
                                        cy=y
                                        r=radius
                                        fill=stroke
                                        stroke="var(--fx-color-bg, #141414)"
                                        stroke-width="2"
                                        style=move || format!(
                                            "cursor: pointer; opacity: {}; transition: r 0.15s ease, opacity 0.15s ease;",
                                            opacity()
                                        )
                                        on:mouseenter=move |ev| {
                                            hovered_point.set(Some(idx));
                                            tooltip_data.set(Some((timestamp, value, x, y, label.clone())));
                                            // Get mouse position relative to container
                                            if let Some(container) = container_ref.get() {
                                                let rect = container.get_bounding_client_rect();
                                                let mouse_x = ev.client_x() as f64 - rect.left();
                                                let mouse_y = ev.client_y() as f64 - rect.top();
                                                mouse_pos.set((mouse_x, mouse_y));
                                            }
                                        }
                                        on:mousemove=move |ev| {
                                            // Update mouse position on move for smooth tracking
                                            if let Some(container) = container_ref.get() {
                                                let rect = container.get_bounding_client_rect();
                                                let mouse_x = ev.client_x() as f64 - rect.left();
                                                let mouse_y = ev.client_y() as f64 - rect.top();
                                                mouse_pos.set((mouse_x, mouse_y));
                                            }
                                        }
                                        on:mouseleave=move |_| {
                                            hovered_point.set(None);
                                            tooltip_data.set(None);
                                        }
                                        on:click={
                                            let on_click = on_click.clone();
                                            let point = point_for_click.clone();
                                            move |_| {
                                                if let Some(ref callback) = on_click {
                                                    callback.run(point.clone());
                                                }
                                            }
                                        }
                                    />
                                }
                            }
                        }).collect::<Vec<_>>()
                    }
                }

                // Y-axis label
                {y_label.clone().map(|label| view! {
                    <text
                        x="10"
                        y=padding + inner_height / 2.0
                        fill="var(--fx-color-text-secondary, #8c8c8c)"
                        font-size="12"
                        text-anchor="middle"
                        transform=format!("rotate(-90, 10, {})", padding + inner_height / 2.0)
                    >
                        {label}
                    </text>
                })}

                // X-axis label
                {x_label.map(|label| view! {
                    <text
                        x=padding + inner_width / 2.0
                        y=chart_height as f64 - 5.0
                        fill="var(--fx-color-text-secondary, #8c8c8c)"
                        font-size="12"
                        text-anchor="middle"
                    >
                        {label}
                    </text>
                })}
            </svg>

            // Tooltip
            {
                let y_label_for_tooltip = y_label.clone();
                let chart_prefix_tooltip = chart_prefix.clone();
                move || {
                    tooltip_data.get().map(|(timestamp, value, _data_x, _data_y, label)| {
                        // Format timestamp as date/time
                        let formatted_time = format_timestamp(timestamp);
                        let value_label = y_label_for_tooltip.clone().unwrap_or_else(|| "Value".to_string());
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
                                // Label if present
                                {label.map(|l| view! {
                                    <div style="color: var(--fx-color-text, #fff); font-weight: 500; margin-bottom: 4px;">
                                        {l}
                                    </div>
                                })}
                                // Timestamp
                                <div style="color: var(--fx-color-text-secondary, #8c8c8c); margin-bottom: 2px;">
                                    {formatted_time}
                                </div>
                                // Value
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <span style=format!("width: 10px; height: 10px; border-radius: 2px; background: {};", stroke_color) />
                                    <span style="color: var(--fx-color-text, #fff);">
                                        {format!("{}: {:.2}", value_label, value)}
                                    </span>
                                </div>
                            </div>
                        }
                    })
                }
            }

            // Loading overlay
            {loading.then(|| view! {
                <div class=format!("{}-loading-overlay", chart_prefix)>
                    <span class=format!("{}-loading-spinner", chart_prefix)></span>
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
