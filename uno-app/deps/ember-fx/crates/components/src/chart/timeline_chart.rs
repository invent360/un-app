use leptos::prelude::*;

use super::types::{ColorPalette, TimelineChartConfig, TimelineEvent};
use super::utils::{chart_prefix, try_use_theme};

/// Format duration in a human-readable format
fn format_duration(ms: i64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60_000 {
        let secs = ms as f64 / 1000.0;
        format!("{:.1}s", secs)
    } else if ms < 3_600_000 {
        let mins = ms as f64 / 60_000.0;
        format!("{:.1}m", mins)
    } else if ms < 86_400_000 {
        let hours = ms as f64 / 3_600_000.0;
        format!("{:.1}h", hours)
    } else {
        let days = ms as f64 / 86_400_000.0;
        format!("{:.1}d", days)
    }
}

/// Tooltip data for timeline events
#[derive(Clone, Debug)]
struct TimelineTooltipData {
    /// Event label/name
    label: String,
    /// Row category name
    row: String,
    /// Start time
    start: i64,
    /// End time
    end: i64,
    /// Duration in milliseconds
    duration: i64,
    /// Event color
    color: String,
    /// X position for tooltip
    x: f64,
    /// Y position for tooltip
    y: f64,
}

/// Gantt-style timeline chart component
///
/// Interactive timeline/gantt chart visualization with hover effects and tooltips.
///
/// # Props
///
/// - `data` - Timeline event data
/// - `config` - Chart configuration
/// - `colors` - Color palette
/// - `on_event_click` - Click handler (event index)
/// - `loading` - Loading state
/// - `class` - Additional CSS classes
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{TimelineChart, TimelineEvent};
///
/// let data = vec![
///     TimelineEvent::new("Task 1", 0, 100).with_label("Setup"),
///     TimelineEvent::new("Task 2", 50, 200).with_label("Build"),
///     TimelineEvent::new("Task 3", 150, 300).with_label("Deploy"),
/// ];
///
/// view! {
///     <TimelineChart
///         data=Signal::derive(move || data.clone())
///         on_event_click=Callback::new(|idx| log::info!("Clicked event {}", idx))
///     />
/// }
/// ```
#[component]
pub fn TimelineChart(
    /// Timeline events data
    #[prop(into)]
    data: Signal<Vec<TimelineEvent>>,
    /// Configuration for the timeline chart
    #[prop(optional)]
    config: Option<TimelineChartConfig>,
    /// Optional color palette
    #[prop(optional, into)]
    colors: Option<Signal<ColorPalette>>,
    /// Click handler (event index)
    #[prop(optional, into)]
    on_event_click: Option<Callback<usize>>,
    /// Whether the chart is in a loading state
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme = try_use_theme();
    let prefix = chart_prefix("timeline");

    let _config = config.unwrap_or_default();

    // Interactive state
    let hovered_event = RwSignal::new(None::<usize>);
    let tooltip_data = RwSignal::new(None::<TimelineTooltipData>);
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    let default_colors = ColorPalette::default();
    let colors = colors.unwrap_or_else(|| Signal::stored(default_colors));

    // Chart dimensions
    let width = 800.0;
    let padding_left = 150.0;
    let padding_right = 40.0;
    let padding_top = 40.0;
    let padding_bottom = 60.0;
    let row_height = 40.0;
    let bar_height = 24.0;

    // Pre-clone prefix for use in closures
    let prefix_svg = prefix.clone();
    let prefix_event = prefix.clone();
    let prefix_loading = prefix.clone();
    let prefix_tooltip = prefix.clone();

    // Calculate timeline data
    let timeline_data = Memo::new(move |_| {
        let events = data.get();
        let palette = colors.get();

        if events.is_empty() {
            return (Vec::new(), 0.0, 0.0, 0.0, Vec::new());
        }

        // Find time range (convert i64 to f64 for calculations)
        let min_start = events.iter().map(|e| e.start).min().unwrap_or(0) as f64;
        let max_end = events.iter().map(|e| e.end).max().unwrap_or(0) as f64;
        let time_range = max_end - min_start;

        let chart_width = width - padding_left - padding_right;
        let height = padding_top + padding_bottom + (events.len() as f64 * row_height);

        // Bar data: (x, y, width, height, label, color, row, start, end, duration)
        let bars: Vec<(f64, f64, f64, f64, String, String, String, i64, i64, i64)> = events
            .iter()
            .enumerate()
            .map(|(i, event)| {
                let event_start = event.start as f64;
                let event_end = event.end as f64;
                let x = padding_left + ((event_start - min_start) / time_range) * chart_width;
                let bar_width = ((event_end - event_start) / time_range) * chart_width;
                let y = padding_top + (i as f64 * row_height) + (row_height - bar_height) / 2.0;

                let color = event
                    .color
                    .as_ref()
                    .map(|c| c.as_css().to_string())
                    .unwrap_or_else(|| palette.color_at(i).to_string());

                let duration = event.end - event.start;

                (
                    x,
                    y,
                    bar_width,
                    bar_height,
                    event.label.clone().unwrap_or_default(),
                    color,
                    event.row.clone(),
                    event.start,
                    event.end,
                    duration,
                )
            })
            .collect();

        // Time axis labels (use fixed 6 ticks)
        let num_ticks = 6usize;
        let time_labels: Vec<(f64, String)> = (0..=num_ticks)
            .map(|i| {
                let t = min_start + time_range * (i as f64 / num_ticks as f64);
                let x = padding_left + chart_width * (i as f64 / num_ticks as f64);
                let label = format!("{:.0}", t);
                (x, label)
            })
            .collect();

        (bars, height, min_start, max_end, time_labels)
    });

    let combined_class = {
        let mut parts = vec![prefix.clone()];
        if loading {
            parts.push(format!("{}-loading", prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class style="position: relative;" node_ref=container_ref>
            <svg
                viewBox=move || format!("0 0 {} {}", width, timeline_data.get().1.max(200.0))
                class=format!("{}-svg", prefix_svg)
            >
                // Grid lines
                {move || {
                    let (_, height, _, _, time_labels) = timeline_data.get();
                    time_labels.iter().map(|(x, _)| {
                        view! {
                            <line
                                x1=*x
                                y1=padding_top
                                x2=*x
                                y2=height - padding_bottom
                                stroke=move || if theme.is_some() { "var(--chart-grid)" } else { "#e5e7eb" }
                                stroke-width="1"
                                stroke-dasharray="4,4"
                            />
                        }
                    }).collect_view()
                }}

                // Time axis
                <line
                    x1=padding_left
                    y1=move || timeline_data.get().1 - padding_bottom
                    x2=width - padding_right
                    y2=move || timeline_data.get().1 - padding_bottom
                    stroke=move || if theme.is_some() { "var(--chart-axis)" } else { "#9ca3af" }
                    stroke-width="1"
                />

                // Time labels
                {move || {
                    let (_, height, _, _, time_labels) = timeline_data.get();
                    time_labels.into_iter().map(|(x, label)| {
                        view! {
                            <text
                                x=x
                                y=height - padding_bottom + 20.0
                                text-anchor="middle"
                                fill=move || if theme.is_some() { "var(--chart-text)" } else { "#6b7280" }
                                font-size="12"
                            >
                                {label}
                            </text>
                        }
                    }).collect_view()
                }}

                // Event bars
                {move || {
                    let (bars, _, _, _, _) = timeline_data.get();
                    let prefix_inner = prefix_event.clone();
                    let on_click = on_event_click.clone();

                    bars.into_iter().enumerate().map(|(i, (x, y, bar_width, bar_h, label, color, row, start, end, duration))| {
                        let prefix_g = prefix_inner.clone();
                        let on_click = on_click.clone();
                        let color_clone = color.clone();
                        let label_clone = label.clone();
                        let row_clone = row.clone();

                        // Hover state helpers
                        let is_hovered = move || hovered_event.get() == Some(i);
                        let opacity = move || {
                            match hovered_event.get() {
                                Some(h) if h != i => 0.5,
                                _ => 1.0,
                            }
                        };
                        let bar_filter = move || {
                            if is_hovered() {
                                "brightness(1.2)"
                            } else {
                                "brightness(1.0)"
                            }
                        };
                        let border_width = move || {
                            if is_hovered() { "2" } else { "1" }
                        };
                        let color_for_border = color_clone.clone();
                        let border_color = move || {
                            if is_hovered() {
                                "var(--fx-color-primary, #1890ff)".to_string()
                            } else {
                                color_for_border.clone()
                            }
                        };

                        view! {
                            <g
                                class=format!("{}-event", prefix_g)
                                style=move || format!(
                                    "opacity: {}; filter: {}; transition: all 0.2s ease; cursor: pointer;",
                                    opacity(),
                                    bar_filter()
                                )
                                on:mouseenter={
                                    let label = label_clone.clone();
                                    let row = row_clone.clone();
                                    let color = color.clone();
                                    move |ev| {
                                        hovered_event.set(Some(i));
                                        tooltip_data.set(Some(TimelineTooltipData {
                                            label: label.clone(),
                                            row: row.clone(),
                                            start,
                                            end,
                                            duration,
                                            color: color.clone(),
                                            x: x + bar_width / 2.0,
                                            y: y + bar_h / 2.0,
                                        }));
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
                                    hovered_event.set(None);
                                    tooltip_data.set(None);
                                }
                                on:click={
                                    let on_click = on_click.clone();
                                    move |_| {
                                        if let Some(ref callback) = on_click {
                                            callback.run(i);
                                        }
                                    }
                                }
                            >
                                // Row label
                                <text
                                    x=padding_left - 10.0
                                    y=y + bar_h / 2.0
                                    text-anchor="end"
                                    dominant-baseline="middle"
                                    fill=move || if theme.is_some() { "var(--chart-text)" } else { "#374151" }
                                    font-size="12"
                                    style="pointer-events: none;"
                                >
                                    {row}
                                </text>

                                // Background bar
                                <rect
                                    x=x
                                    y=y
                                    width=bar_width.max(2.0)
                                    height=bar_h
                                    fill=color.clone()
                                    opacity="0.3"
                                    rx="4"
                                />

                                // Main bar fill (full bar, no progress)
                                <rect
                                    x=x
                                    y=y
                                    width=bar_width.max(2.0)
                                    height=bar_h
                                    fill=color.clone()
                                    rx="4"
                                />

                                // Bar border (highlights on hover)
                                <rect
                                    x=x
                                    y=y
                                    width=bar_width.max(2.0)
                                    height=bar_h
                                    fill="none"
                                    stroke=border_color
                                    stroke-width=border_width
                                    rx="4"
                                    style="transition: stroke 0.2s ease, stroke-width 0.2s ease;"
                                />

                                // Bar label
                                <text
                                    x=x + bar_width / 2.0
                                    y=y + bar_h / 2.0
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    fill=move || if theme.is_some() { "var(--chart-text)" } else { "#1f2937" }
                                    font-size="11"
                                    font-weight="500"
                                    style="pointer-events: none;"
                                >
                                    {label}
                                </text>
                            </g>
                        }
                    }).collect_view()
                }}
            </svg>

            // Tooltip
            {move || {
                let prefix_tt = prefix_tooltip.clone();
                tooltip_data.get().map(|data| {
                    // Use actual mouse position for tooltip placement
                    let (mx, my) = mouse_pos.get();
                    let tooltip_x = mx + 15.0;
                    let tooltip_y = my;
                    view! {
                        <div
                            class=format!("{}-tooltip", prefix_tt)
                            style=format!(
                                "position: absolute; left: {}px; top: {}px; background: var(--fx-color-bg-elevated, #1f1f1f); border: 1px solid var(--fx-color-border, #303030); border-radius: 6px; padding: 10px 14px; font-size: 12px; pointer-events: none; z-index: 10; box-shadow: 0 2px 8px rgba(0,0,0,0.3); min-width: 180px; transform: translateY(-50%);",
                                tooltip_x, tooltip_y
                            )
                        >
                            // Header with color indicator and label
                            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 8px;">
                                <span style=format!("width: 12px; height: 12px; border-radius: 3px; background: {}; flex-shrink: 0;", data.color) />
                                <span style="color: var(--fx-color-text, #fff); font-weight: 600; font-size: 13px;">
                                    {if data.label.is_empty() { data.row.clone() } else { data.label.clone() }}
                                </span>
                            </div>

                            // Row name (if different from label)
                            {(!data.label.is_empty() && data.row != data.label).then(|| view! {
                                <div style="color: var(--fx-color-text-secondary, #8c8c8c); margin-bottom: 6px;">
                                    <span style="color: var(--fx-color-text-tertiary, #595959);">"Category: "</span>
                                    {data.row.clone()}
                                </div>
                            })}

                            // Time details
                            <div style="display: flex; flex-direction: column; gap: 4px; color: var(--fx-color-text-secondary, #8c8c8c);">
                                <div>
                                    <span style="color: var(--fx-color-text-tertiary, #595959);">"Start: "</span>
                                    {format!("{}", data.start)}
                                </div>
                                <div>
                                    <span style="color: var(--fx-color-text-tertiary, #595959);">"End: "</span>
                                    {format!("{}", data.end)}
                                </div>
                                <div style="margin-top: 4px; padding-top: 6px; border-top: 1px solid var(--fx-color-border, #303030);">
                                    <span style="color: var(--fx-color-text-tertiary, #595959);">"Duration: "</span>
                                    <span style="color: var(--fx-color-primary, #1890ff); font-weight: 500;">
                                        {format_duration(data.duration)}
                                    </span>
                                </div>
                            </div>
                        </div>
                    }
                })
            }}

            // Loading overlay
            {move || {
                let prefix_ld = prefix_loading.clone();
                if loading {
                    Some(view! {
                        <div class=format!("{}-loading", prefix_ld.clone())>
                            <div class=format!("{}-spinner", prefix_ld)></div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
