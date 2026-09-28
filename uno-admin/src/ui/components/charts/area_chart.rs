use leptos::prelude::*;
use crate::state::ChartDataPoint;
use super::chart_utils::{ChartDimensions, generate_area_path, generate_smooth_line_path, get_label_positions};

/// Area chart component
#[component]
pub fn AreaChart(
    data: Vec<ChartDataPoint>,
) -> impl IntoView {
    let dims = ChartDimensions::default();
    let area_path = generate_area_path(&data, &dims);
    let line_path = generate_smooth_line_path(&data, &dims);
    let labels = get_label_positions(&data, &dims);
    let label_y = dims.height - 5.0;

    view! {
        <svg
            viewBox=format!("0 0 {} {}", dims.width, dims.height)
            class="w-full h-40"
            preserveAspectRatio="xMidYMid meet"
        >
            // Grid lines (horizontal)
            {(0..4).map(|i| {
                let y = dims.padding_top + (i as f64 / 3.0) * dims.inner_height();
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

            // Gradient definition
            <defs>
                <linearGradient id="areaGradient" x1="0%" y1="0%" x2="0%" y2="100%">
                    <stop offset="0%" style="stop-color:var(--chart-area-stroke);stop-opacity:0.3" />
                    <stop offset="100%" style="stop-color:var(--chart-area-stroke);stop-opacity:0.05" />
                </linearGradient>
            </defs>

            // Area fill
            <path
                d=area_path
                fill="url(#areaGradient)"
                stroke="none"
            />

            // Line stroke
            <path d=line_path class="chart-line" style="stroke: var(--chart-area-stroke);" />

            // X-axis labels
            {labels.into_iter().map(|(x, label)| {
                view! {
                    <text x=x y=label_y class="chart-axis-label">
                        {label}
                    </text>
                }
            }).collect::<Vec<_>>()}
        </svg>
    }
}
