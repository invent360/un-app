use leptos::prelude::*;
use crate::state::ChartDataPoint;
use super::chart_utils::{ChartDimensions, generate_smooth_line_path, get_label_positions};

/// Line chart component
#[component]
pub fn LineChart(
    data: Vec<ChartDataPoint>,
    #[prop(optional)] show_secondary: Option<Vec<ChartDataPoint>>,
) -> impl IntoView {
    let dims = ChartDimensions::default();
    let path = generate_smooth_line_path(&data, &dims);
    let labels = get_label_positions(&data, &dims);
    let label_y = dims.height - 5.0;

    let secondary_path = show_secondary
        .as_ref()
        .map(|sec_data| generate_smooth_line_path(sec_data, &dims));

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

            // Secondary line (if provided)
            {secondary_path.map(|sp| view! {
                <path d=sp class="chart-line-secondary" />
            })}

            // Main line
            <path d=path class="chart-line" />

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
