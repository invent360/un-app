//! Daily earnings bar chart with dark background, green bars, and hover tooltips

use leptos::prelude::*;
use crate::handler::DailyIncentive;
use super::chart_utils::{ChartDimensions, get_value_range};
use crate::state::ChartDataPoint;

/// Format date from YYYY-MM-DD to DD/MM/YYYY
fn format_date_ddmmyyyy(date: &str) -> String {
    // Input: "2026-06-23" or "Jun 23"
    // If already in short format, try to parse
    if date.contains('-') {
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{}", parts[2], parts[1], parts[0]);
        }
    }
    date.to_string()
}

/// Daily earnings bar chart with dark background, green bars, $ prefix on Y-axis, and hover tooltips
#[component]
pub fn DailyEarningsChart(
    data: Vec<DailyIncentive>,
    #[prop(optional)]
    syncing: Option<Signal<bool>>,
) -> impl IntoView {
    let data_len = data.len();

    if data.is_empty() {
        return view! {
            <div class="bg-slate-800 rounded-xl p-6 min-h-[300px] flex flex-col items-center justify-center text-slate-400">
                <svg class="w-12 h-12 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                </svg>
                <p class="text-sm">"No earnings data"</p>
            </div>
        }.into_any();
    }

    // Calculate dimensions - wider for 30 days with tighter bar spacing
    let width = (data_len as f64 * 32.0).max(600.0).min(1400.0);

    let dims = ChartDimensions {
        width,
        height: 320.0,
        padding_left: 55.0,  // More space for $ labels
        padding_right: 20.0,
        padding_top: 20.0,
        padding_bottom: 90.0,  // More space for rotated labels
    };

    let bar_gap = 2.0;

    // Convert to ChartDataPoint for value range calculation
    let chart_points: Vec<ChartDataPoint> = data.iter()
        .map(|d| ChartDataPoint::new(d.date.clone(), d.amount))
        .collect();

    // Calculate bar dimensions
    let inner_width = dims.inner_width();
    let inner_height = dims.inner_height();
    let n = data_len as f64;
    let bar_width = (inner_width - (n - 1.0) * bar_gap) / n;

    let label_y = dims.height - dims.padding_bottom + 18.0;

    // Calculate actual max value for Y-axis
    let (_, max_val) = get_value_range(&chart_points);
    let y_max = if max_val > 0.0 { max_val * 1.1 } else { 0.10 };  // Add 10% headroom

    // Show all labels - the rotated dates will fit
    let label_step = 1;

    // Format Y-axis labels with $ prefix
    let format_y_label = move |val: f64| -> String {
        if val >= 1.0 {
            format!("${:.0}", val)
        } else {
            format!("${:.2}", val)
        }
    };

    // Tooltip state
    let tooltip_visible = RwSignal::new(false);
    let tooltip_x = RwSignal::new(0.0f64);
    let tooltip_y = RwSignal::new(0.0f64);
    let tooltip_date = RwSignal::new(String::new());
    let tooltip_amount = RwSignal::new(0.0f64);
    let tooltip_licenses = RwSignal::new(0i32);

    view! {
        <div class="bg-slate-800 rounded-xl p-6">
            // Header with title and sync indicator
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-base font-semibold text-white">
                    "Daily Earnings (Last 30 Days)"
                </h3>
                {move || {
                    syncing.map(|s| {
                        view! {
                            <Show when=move || s.get()>
                                <div class="flex items-center gap-2 text-xs text-emerald-400">
                                    <span class="inline-block w-3 h-3 border-2 border-emerald-400 border-t-transparent rounded-full animate-spin"></span>
                                    <span>"Syncing..."</span>
                                </div>
                            </Show>
                        }
                    })
                }}
            </div>

            // Chart SVG
            <div class="overflow-x-auto relative">
                <svg
                    viewBox=format!("0 0 {} {}", dims.width, dims.height)
                    class="w-full min-w-[600px]"
                    style="height: 280px;"
                    preserveAspectRatio="xMidYMid meet"
                >
                    // Grid lines (horizontal) - subtle
                    {(0..5).map(|i| {
                        let y = dims.padding_top + (i as f64 / 4.0) * inner_height;
                        view! {
                            <line
                                x1=dims.padding_left
                                y1=y
                                x2=dims.width - dims.padding_right
                                y2=y
                                stroke="#334155"
                                stroke-width="1"
                            />
                        }
                    }).collect::<Vec<_>>()}

                    // Y-axis labels with $ prefix
                    {(0..5).map(|i| {
                        let y = dims.padding_top + (i as f64 / 4.0) * inner_height;
                        let val = ((4 - i) as f64 / 4.0) * y_max;
                        let label = format_y_label(val);
                        view! {
                            <text
                                x=dims.padding_left - 8.0
                                y=y + 4.0
                                fill="#94a3b8"
                                style="text-anchor: end; font-size: 12px; font-family: system-ui, sans-serif;"
                            >
                                {label}
                            </text>
                        }
                    }).collect::<Vec<_>>()}

                    // Bars with hover events - emerald/green color
                    {data.iter().enumerate().map(|(i, incentive)| {
                        let x = dims.padding_left + i as f64 * (bar_width + bar_gap);
                        let height_ratio = if y_max > 0.0 { incentive.amount / y_max } else { 0.0 };
                        let bar_height = height_ratio * inner_height;
                        let y = dims.padding_top + inner_height - bar_height;

                        let date = incentive.date.clone();
                        let amount = incentive.amount;
                        let licenses = incentive.license_count;
                        let bar_center_x = x + bar_width / 2.0;

                        view! {
                            <rect
                                x=x
                                y=y
                                width=bar_width
                                height=bar_height.max(1.0)
                                rx=2.0
                                fill="#10b981"
                                class="cursor-pointer transition-opacity hover:opacity-80"
                                on:mouseenter=move |_| {
                                    tooltip_visible.set(true);
                                    tooltip_x.set(bar_center_x);
                                    tooltip_y.set(y - 10.0);
                                    tooltip_date.set(format_date_ddmmyyyy(&date));
                                    tooltip_amount.set(amount);
                                    tooltip_licenses.set(licenses);
                                }
                                on:mouseleave=move |_| {
                                    tooltip_visible.set(false);
                                }
                            />
                        }
                    }).collect::<Vec<_>>()}

                    // X-axis labels (rotated dates at 45 degrees)
                    {data.iter().enumerate().filter_map(|(i, incentive)| {
                        if i % label_step == 0 {
                            let x = dims.padding_left + i as f64 * (bar_width + bar_gap) + bar_width / 2.0;
                            let formatted_label = format_date_ddmmyyyy(&incentive.date);
                            Some(view! {
                                <text
                                    x=x
                                    y=label_y
                                    fill="#94a3b8"
                                    transform=format!("rotate(-45, {}, {})", x, label_y)
                                    style="text-anchor: end; font-size: 10px; font-family: system-ui, sans-serif;"
                                >
                                    {formatted_label}
                                </text>
                            })
                        } else {
                            None
                        }
                    }).collect::<Vec<_>>()}

                    // Tooltip (rendered inside SVG for proper positioning)
                    <Show when=move || tooltip_visible.get()>
                        {move || {
                            let tx = tooltip_x.get();
                            let ty = tooltip_y.get();
                            let date_str = tooltip_date.get();
                            let amount_val = tooltip_amount.get();
                            let license_count = tooltip_licenses.get();

                            // Calculate tooltip dimensions
                            let tooltip_width = 140.0;
                            let tooltip_height = 60.0;

                            // Adjust x position to keep tooltip within bounds
                            let adjusted_x = if tx - tooltip_width / 2.0 < dims.padding_left {
                                dims.padding_left
                            } else if tx + tooltip_width / 2.0 > dims.width - dims.padding_right {
                                dims.width - dims.padding_right - tooltip_width
                            } else {
                                tx - tooltip_width / 2.0
                            };

                            // Adjust y to show above bar (or below if too high)
                            let adjusted_y = if ty - tooltip_height - 5.0 < 0.0 {
                                ty + 20.0
                            } else {
                                ty - tooltip_height - 5.0
                            };

                            view! {
                                <g>
                                    // Tooltip background
                                    <rect
                                        x=adjusted_x
                                        y=adjusted_y
                                        width=tooltip_width
                                        height=tooltip_height
                                        rx=6.0
                                        fill="#1e293b"
                                        stroke="#475569"
                                        stroke-width="1"
                                    />
                                    // Date
                                    <text
                                        x=adjusted_x + 10.0
                                        y=adjusted_y + 18.0
                                        fill="#94a3b8"
                                        style="font-size: 11px; font-family: system-ui, sans-serif;"
                                    >
                                        {date_str}
                                    </text>
                                    // Amount
                                    <text
                                        x=adjusted_x + 10.0
                                        y=adjusted_y + 35.0
                                        fill="#10b981"
                                        style="font-size: 13px; font-weight: 600; font-family: system-ui, sans-serif;"
                                    >
                                        {format!("${:.4}", amount_val)}
                                    </text>
                                    // License count
                                    <text
                                        x=adjusted_x + 10.0
                                        y=adjusted_y + 52.0
                                        fill="#94a3b8"
                                        style="font-size: 11px; font-family: system-ui, sans-serif;"
                                    >
                                        {format!("{} license{}", license_count, if license_count == 1 { "" } else { "s" })}
                                    </text>
                                </g>
                            }
                        }}
                    </Show>
                </svg>
            </div>
        </div>
    }.into_any()
}
