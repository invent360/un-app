use leptos::prelude::*;

/// Data point for pie chart
#[derive(Debug, Clone)]
pub struct PieSlice {
    pub label: String,
    pub value: f64,
    pub color: String,
}

impl PieSlice {
    pub fn new(label: impl Into<String>, value: f64, color: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value,
            color: color.into(),
        }
    }
}

/// Simple pie chart component using SVG
#[component]
pub fn PieChart(
    data: Vec<PieSlice>,
    #[prop(default = 120.0)] size: f64,
    #[prop(default = true)] show_legend: bool,
) -> impl IntoView {
    let total: f64 = data.iter().map(|s| s.value).sum();
    if total == 0.0 {
        return view! {
            <div class="flex items-center justify-center" style=format!("width: {}px; height: {}px;", size, size)>
                <span class="text-slate-400 text-sm">"No data"</span>
            </div>
        }.into_any();
    }

    let center = size / 2.0;
    let radius = (size / 2.0) - 4.0;

    // Calculate path data for each slice
    let mut paths: Vec<(String, String, f64)> = Vec::new();
    let mut current_angle = -std::f64::consts::PI / 2.0; // Start from top

    for slice in &data {
        if slice.value <= 0.0 {
            continue;
        }

        let angle = (slice.value / total) * 2.0 * std::f64::consts::PI;
        let start_angle = current_angle;
        let end_angle = current_angle + angle;

        // Calculate arc path
        let start_x = center + radius * start_angle.cos();
        let start_y = center + radius * start_angle.sin();
        let end_x = center + radius * end_angle.cos();
        let end_y = center + radius * end_angle.sin();

        let large_arc = if angle > std::f64::consts::PI { 1 } else { 0 };

        let path = format!(
            "M {} {} L {} {} A {} {} 0 {} 1 {} {} Z",
            center, center,
            start_x, start_y,
            radius, radius,
            large_arc,
            end_x, end_y
        );

        paths.push((path, slice.color.clone(), slice.value / total * 100.0));
        current_angle = end_angle;
    }

    let legend_items = data.clone();

    view! {
        <div class="flex flex-col items-center gap-2">
            <svg
                viewBox=format!("0 0 {} {}", size, size)
                class="pie-chart"
                style=format!("width: {}px; height: {}px;", size, size)
            >
                {paths.into_iter().map(|(path, color, _pct)| {
                    view! {
                        <path
                            d=path
                            fill=color
                            class="transition-all duration-200 hover:opacity-80"
                        />
                    }
                }).collect::<Vec<_>>()}

                // Center circle (donut effect)
                <circle
                    cx=center
                    cy=center
                    r=radius * 0.5
                    class="fill-white dark:fill-slate-800"
                />
            </svg>

            {move || if show_legend {
                Some(view! {
                    <div class="flex flex-wrap justify-center gap-x-4 gap-y-1 text-xs">
                        {legend_items.iter().map(|slice| {
                            let pct = (slice.value / total * 100.0).round() as i32;
                            let label = slice.label.clone();
                            let color = slice.color.clone();
                            view! {
                                <div class="flex items-center gap-1">
                                    <div
                                        class="w-2 h-2 rounded-full"
                                        style=format!("background-color: {}", color)
                                    />
                                    <span class="text-slate-600 dark:text-slate-400">
                                        {label}
                                    </span>
                                    <span class="text-slate-400 dark:text-slate-500">
                                        {format!("{}%", pct)}
                                    </span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_any())
            } else {
                None
            }}
        </div>
    }.into_any()
}

/// Stacked bar showing percentages (alternative to pie chart for narrow spaces)
#[component]
pub fn PercentageBar(
    data: Vec<PieSlice>,
    #[prop(default = 8.0)] height: f64,
) -> impl IntoView {
    let total: f64 = data.iter().map(|s| s.value).sum();
    if total == 0.0 {
        return view! {
            <div class="h-2 w-full bg-slate-100 dark:bg-slate-700 rounded-full"></div>
        }.into_any();
    }

    let segments: Vec<(f64, String)> = data
        .iter()
        .filter(|s| s.value > 0.0)
        .map(|s| (s.value / total * 100.0, s.color.clone()))
        .collect();

    view! {
        <div class="w-full rounded-full overflow-hidden flex" style=format!("height: {}px", height)>
            {segments.into_iter().map(|(pct, color)| {
                view! {
                    <div
                        class="h-full transition-all duration-300"
                        style=format!("width: {}%; background-color: {}", pct, color)
                    />
                }
            }).collect::<Vec<_>>()}
        </div>
    }.into_any()
}

/// Helper function to create standard ACME split slices
pub fn create_split_slices(uno_share: f64, agent_share: f64, ulo_share: f64) -> Vec<PieSlice> {
    vec![
        PieSlice::new("UNO (ACME)", uno_share, "#3b82f6"),     // blue
        PieSlice::new("Agent", agent_share, "#22c55e"),        // green
        PieSlice::new("ULO", ulo_share, "#f59e0b"),            // amber
    ]
}

/// Helper function to create earnings breakdown slices
pub fn create_earnings_slices(uno_earnings: f64, agent_earnings: f64) -> Vec<PieSlice> {
    vec![
        PieSlice::new("ACME Net", uno_earnings, "#3b82f6"),    // blue
        PieSlice::new("Agent Commission", agent_earnings, "#22c55e"), // green
    ]
}
