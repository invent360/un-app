//! Earnings bar chart component

use leptos::prelude::*;

/// Earnings data by task type
#[derive(Debug, Clone)]
pub struct TaskEarnings {
    pub task_type: String,
    pub earnings: f64,
    pub color: String,
}

/// Earnings bar chart component
#[component]
pub fn EarningsChart(
    data: Vec<TaskEarnings>,
    #[prop(optional)] title: Option<String>,
) -> impl IntoView {
    let max_earnings = data.iter().map(|d| d.earnings).fold(0.0_f64, f64::max);

    view! {
        <div class="earnings-chart">
            {title.map(|t| view! { <h3 class="chart-title">{t}</h3> })}
            <div class="chart-container">
                {data.into_iter().map(|item| {
                    let percentage = if max_earnings > 0.0 {
                        (item.earnings / max_earnings) * 100.0
                    } else {
                        0.0
                    };

                    view! {
                        <div class="chart-bar-container">
                            <div class="chart-bar-label">{item.task_type.clone()}</div>
                            <div class="chart-bar-wrapper">
                                <div
                                    class="chart-bar"
                                    style=format!(
                                        "width: {}%; background-color: {}",
                                        percentage,
                                        item.color
                                    )
                                >
                                    <span class="chart-bar-value">
                                        {format!("${:.2}", item.earnings)}
                                    </span>
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
