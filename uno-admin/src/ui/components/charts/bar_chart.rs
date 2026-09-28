use leptos::prelude::*;
use crate::state::ChartDataPoint;
use super::chart_utils::{ChartDimensions, calculate_bar_layout, get_bar_label_positions, get_value_range};

/// Bar chart component - scales to full container width
#[component]
pub fn BarChart(
    data: Vec<ChartDataPoint>,
) -> impl IntoView {
    let data_len = data.len();

    // Use wider viewBox for more bars with tighter spacing
    let width = (data_len as f64 * 32.0).max(400.0).min(1400.0);

    let dims = ChartDimensions {
        width,
        height: 280.0,
        padding_left: 45.0,
        padding_right: 15.0,
        padding_top: 15.0,
        padding_bottom: 90.0,  // More space for rotated labels
    };

    let bar_gap = 2.0;
    let bars = calculate_bar_layout(&data, &dims, bar_gap);
    let labels = get_bar_label_positions(&data, &dims, bar_gap);
    let label_y = dims.height - dims.padding_bottom + 15.0;

    // Calculate actual max value for Y-axis
    let (_, max_val) = get_value_range(&data);
    let y_max = if max_val > 0.0 { max_val } else { 100.0 };

    // Show all labels - the rotated dates will fit
    let label_step = 1;

    view! {
        <svg
            viewBox=format!("0 0 {} {}", dims.width, dims.height)
            class="w-full"
            style="height: 280px;"
            preserveAspectRatio="xMidYMid meet"
        >
            // Grid lines (horizontal)
            {(0..5).map(|i| {
                let y = dims.padding_top + (i as f64 / 4.0) * dims.inner_height();
                view! {
                    <line
                        x1=dims.padding_left
                        y1=y
                        x2=dims.width - dims.padding_right
                        y2=y
                        class="chart-grid"
                        stroke-dasharray="4 4"
                    />
                }
            }).collect::<Vec<_>>()}

            // Y-axis labels (actual values)
            {(0..5).map(|i| {
                let y = dims.padding_top + (i as f64 / 4.0) * dims.inner_height();
                let val = ((4 - i) as f64 / 4.0) * y_max;
                let label = if val >= 1000.0 {
                    format!("{:.1}k", val / 1000.0)
                } else {
                    format!("{:.0}", val)
                };
                view! {
                    <text x=dims.padding_left - 5.0 y=y + 4.0 class="chart-axis-label" style="text-anchor: end; font-size: 14px;">
                        {label}
                    </text>
                }
            }).collect::<Vec<_>>()}

            // Bars
            {bars.into_iter().map(|(x, y, width, height)| {
                view! {
                    <rect
                        x=x
                        y=y
                        width=width
                        height=height
                        rx=3.0
                        class="chart-bar"
                    />
                }
            }).collect::<Vec<_>>()}

            // X-axis labels (rotated at 45 degrees)
            {labels.into_iter().enumerate().filter_map(|(i, (x, label))| {
                if i % label_step == 0 {
                    Some(view! {
                        <text
                            x=x
                            y=label_y
                            class="chart-axis-label"
                            transform=format!("rotate(-45, {}, {})", x, label_y)
                            style="text-anchor: end; font-size: 10px;"
                        >
                            {label}
                        </text>
                    })
                } else {
                    None
                }
            }).collect::<Vec<_>>()}
        </svg>
    }
}
