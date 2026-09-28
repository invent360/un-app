//! Mini uptime chart component for inline table display
//!
//! A compact combined bar + line chart showing 7 days of uptime history.

use leptos::prelude::*;

/// Compact 7-day uptime chart combining bars and line for table cells
///
/// Displays bars for each day with a trend line connecting them:
/// - Green (≥90%): Good uptime
/// - Amber (≥75%): Warning
/// - Red (<75%): Poor uptime
#[component]
pub fn MiniUptimeChart(
    /// Uptime values (0.0-1.0) for each day, most recent last
    data: Vec<f64>,
    /// Width of the chart in pixels
    #[prop(default = 80.0)]
    width: f64,
    /// Height of the chart in pixels
    #[prop(default = 28.0)]
    height: f64,
) -> impl IntoView {
    // Always show 7 days - pad with 0s if needed
    let values: Vec<f64> = if data.is_empty() {
        vec![0.0; 7]
    } else if data.len() < 7 {
        // Pad with 0s at the beginning (older days)
        let mut padded = vec![0.0; 7 - data.len()];
        padded.extend(data.iter().cloned());
        padded
    } else {
        // Take last 7 values
        data.iter().rev().take(7).rev().cloned().collect()
    };

    // Chart dimensions with padding for axes
    let padding_left = 4.0;
    let padding_bottom = 4.0;
    let padding_top = 2.0;
    let padding_right = 2.0;

    let chart_width = width - padding_left - padding_right;
    let chart_height = height - padding_bottom - padding_top;

    // Bar dimensions
    let bar_count = 7.0;
    let gap = 2.0;
    let bar_width = (chart_width - (bar_count - 1.0) * gap) / bar_count;

    // Calculate bar positions and line points
    let bars: Vec<(f64, f64, f64, f64, &str)> = values.iter().enumerate().map(|(i, &uptime)| {
        let uptime = uptime.clamp(0.0, 1.0);
        let x = padding_left + i as f64 * (bar_width + gap);
        let bar_height = (uptime * chart_height).max(1.0); // min 1px height
        let y = padding_top + chart_height - bar_height;

        // Color based on uptime
        let color = if uptime >= 0.9 {
            "#22c55e" // green
        } else if uptime >= 0.75 {
            "#f59e0b" // amber
        } else {
            "#ef4444" // red
        };

        (x, y, bar_width, bar_height, color)
    }).collect();

    // Line points (center top of each bar)
    let line_points: Vec<(f64, f64)> = bars.iter().map(|(x, y, w, _, _)| {
        (*x + *w / 2.0, *y)
    }).collect();

    // Create SVG path for the trend line
    let line_path = line_points.iter().enumerate().map(|(i, (x, y))| {
        if i == 0 {
            format!("M {:.1} {:.1}", x, y)
        } else {
            format!(" L {:.1} {:.1}", x, y)
        }
    }).collect::<String>();

    // Axis positions
    let x_axis_y = padding_top + chart_height;
    let y_axis_x = padding_left;

    view! {
        <svg
            width=width
            height=height
            style="display: inline-block; vertical-align: middle;"
            viewBox={format!("0 0 {} {}", width, height)}
        >
            // Y-axis (left edge)
            <line
                x1=y_axis_x
                y1=padding_top
                x2=y_axis_x
                y2=x_axis_y
                stroke="#64748b"
                stroke-width="1"
            />
            // X-axis (bottom edge)
            <line
                x1=y_axis_x
                y1=x_axis_y
                x2={padding_left + chart_width}
                y2=x_axis_y
                stroke="#64748b"
                stroke-width="1"
            />
            // Bars
            {bars.iter().map(|(x, y, w, h, color)| {
                view! {
                    <rect
                        x=*x
                        y=*y
                        width=*w
                        height=*h
                        fill=*color
                        opacity="0.7"
                        rx="1"
                    />
                }
            }).collect::<Vec<_>>()}
            // Trend line connecting bar tops
            <path
                d=line_path
                fill="none"
                stroke="#22d3ee"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
            // Dots at each data point
            {line_points.iter().map(|(x, y)| {
                view! {
                    <circle
                        cx=*x
                        cy=*y
                        r="2"
                        fill="#22d3ee"
                    />
                }
            }).collect::<Vec<_>>()}
        </svg>
    }.into_any()
}
