use leptos::prelude::*;

use super::types::{ColorPalette, FunnelChartConfig, FunnelSegment};
use super::utils::{chart_prefix, try_use_theme};

/// Tooltip data for funnel segment
#[derive(Clone, Debug)]
struct FunnelTooltipData {
    /// Segment label
    label: String,
    /// Segment value
    value: f64,
    /// Percentage of total (first segment)
    percentage: f64,
    /// Conversion rate from previous segment (100% for first)
    conversion_rate: f64,
    /// Segment color
    color: String,
    /// X position for tooltip
    x: f64,
    /// Y position for tooltip
    y: f64,
}

/// Funnel chart component
///
/// Interactive funnel chart visualization showing progressive data reduction.
/// Supports hover effects, tooltips with conversion rates, and click handling.
///
/// # Props
///
/// - `data` - Funnel segment data
/// - `config` - Chart configuration
/// - `colors` - Optional color palette
/// - `on_segment_click` - Click handler (segment index)
/// - `loading` - Loading state
/// - `class` - Additional CSS classes
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{FunnelChart, FunnelSegment};
///
/// let data = vec![
///     FunnelSegment::new("Visitors", 1000.0),
///     FunnelSegment::new("Leads", 500.0),
///     FunnelSegment::new("Prospects", 200.0),
///     FunnelSegment::new("Customers", 100.0),
/// ];
///
/// view! {
///     <FunnelChart
///         data=Signal::derive(move || data.clone())
///         on_segment_click=Callback::new(|idx| log::info!("Clicked segment {}", idx))
///     />
/// }
/// ```
#[component]
pub fn FunnelChart(
    /// Funnel segment data
    #[prop(into)]
    data: Signal<Vec<FunnelSegment>>,
    /// Configuration for the funnel chart
    #[prop(optional)]
    config: Option<FunnelChartConfig>,
    /// Optional color palette
    #[prop(optional, into)]
    colors: Option<Signal<ColorPalette>>,
    /// Click handler (segment index)
    #[prop(optional, into)]
    on_segment_click: Option<Callback<usize>>,
    /// Whether the chart is in a loading state
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme = try_use_theme();
    let prefix = chart_prefix("funnel");

    let config = config.unwrap_or_default();

    let default_colors = ColorPalette::default();
    let colors = colors.unwrap_or_else(|| Signal::stored(default_colors));

    // Interactive state
    let hovered_segment = RwSignal::new(None::<usize>);
    let tooltip_data = RwSignal::new(None::<FunnelTooltipData>);
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    // Chart dimensions
    let width = 500.0;
    let height = 400.0;
    let padding = 40.0;
    let label_width = 120.0;

    // Store config values for closures
    let show_labels = config.show_labels;
    let is_pyramid = config.pyramid;

    // Calculate funnel geometry
    let funnel_data = Memo::new(move |_| {
        let segments = data.get();
        let palette = colors.get();

        if segments.is_empty() {
            return Vec::new();
        }

        let max_value = segments.iter().map(|s| s.value).fold(0.0_f64, f64::max);
        let first_value = segments.first().map(|s| s.value).unwrap_or(1.0);
        let chart_width = width - padding * 2.0 - label_width;
        let chart_height = height - padding * 2.0;
        let segment_height = chart_height / segments.len() as f64;
        let gap = 4.0;
        let is_inverted = is_pyramid;

        let center_x = padding + label_width + chart_width / 2.0;

        let mut result: Vec<(
            usize,           // index
            String,          // path
            f64,             // x1
            f64,             // x2
            f64,             // center_y
            f64,             // value
            f64,             // segment_width
            f64,             // next_segment_width
            String,          // label
            String,          // color
            f64,             // percentage of first
            f64,             // conversion rate from previous
        )> = Vec::new();

        for (i, segment) in segments.iter().enumerate() {
            let normalized = segment.value / max_value;

            // Calculate width at this level (trapezoid)
            let idx = if is_inverted { segments.len() - 1 - i } else { i };
            let width_factor = 1.0 - (idx as f64 / segments.len() as f64) * 0.7;
            let segment_width = chart_width * width_factor * normalized.max(0.1);

            let next_idx = if is_inverted {
                if idx > 0 { idx - 1 } else { idx }
            } else {
                idx + 1
            };
            let next_width_factor = if next_idx < segments.len() {
                1.0 - (next_idx as f64 / segments.len() as f64) * 0.7
            } else {
                width_factor * 0.5
            };
            let next_normalized = if i + 1 < segments.len() {
                segments[i + 1].value / max_value
            } else {
                normalized * 0.3
            };
            let next_segment_width = chart_width * next_width_factor * next_normalized.max(0.1);

            let y = padding + (i as f64 * segment_height);
            let x1 = center_x - segment_width / 2.0;
            let x2 = center_x + segment_width / 2.0;
            let x3 = center_x + next_segment_width / 2.0;
            let x4 = center_x - next_segment_width / 2.0;

            let color = segment.color
                .map(|c| c.as_css().to_string())
                .unwrap_or_else(|| palette.color_at(i).to_string());

            // Create trapezoid path
            let path = format!(
                "M {} {} L {} {} L {} {} L {} {} Z",
                x1, y + gap / 2.0,
                x2, y + gap / 2.0,
                x3, y + segment_height - gap / 2.0,
                x4, y + segment_height - gap / 2.0
            );

            let center_y = y + segment_height / 2.0;

            // Calculate percentage of first segment
            let percentage = (segment.value / first_value) * 100.0;

            // Calculate conversion rate from previous segment
            let conversion_rate = if i > 0 {
                let prev_value = segments[i - 1].value;
                if prev_value > 0.0 {
                    (segment.value / prev_value) * 100.0
                } else {
                    0.0
                }
            } else {
                100.0 // First segment is always 100%
            };

            result.push((
                i,
                path,
                x1,
                x2,
                center_y,
                segment.value,
                segment_width,
                next_segment_width,
                segment.label.clone(),
                color,
                percentage,
                conversion_rate,
            ));
        }

        result
    });

    // Pre-clone prefix strings for closures
    let prefix_class = prefix.clone();
    let prefix_svg = prefix.clone();
    let prefix_segment = prefix.clone();
    let prefix_path = prefix.clone();
    let prefix_loading = prefix.clone();
    let prefix_spinner = prefix.clone();
    let prefix_tooltip = prefix.clone();

    let combined_class = {
        let mut parts = vec![prefix_class.clone()];
        if loading {
            parts.push(format!("{}-loading", prefix_class));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class style="position: relative;" node_ref=container_ref>
            <svg viewBox=format!("0 0 {} {}", width, height) class=format!("{}-svg", prefix_svg)>
                // Funnel segments
                {move || {
                    let prefix_seg = prefix_segment.clone();
                    let prefix_p = prefix_path.clone();
                    let on_click = on_segment_click.clone();

                    funnel_data.get().into_iter().map({
                        let prefix_seg = prefix_seg.clone();
                        let prefix_p = prefix_p.clone();
                        let on_click = on_click.clone();

                        move |(idx, path, _x1, _x2, center_y, value, _seg_width, _next_width, label, color, percentage, conversion_rate)| {
                            let seg_class = format!("{}-segment", prefix_seg);
                            let path_class = format!("{}-path", prefix_p);

                            // Hover state helpers
                            let is_hovered = move || hovered_segment.get() == Some(idx);

                            // Visual feedback: brightness and opacity
                            let opacity = move || {
                                match hovered_segment.get() {
                                    Some(h) if h != idx => 0.5, // Dim non-hovered segments
                                    _ => 1.0,
                                }
                            };

                            // Slight scale on hover
                            let transform = move || {
                                if is_hovered() {
                                    "translate(0, -2)".to_string()
                                } else {
                                    String::new()
                                }
                            };

                            // Filter for brightness on hover
                            let filter = move || {
                                if is_hovered() {
                                    "brightness(1.15)"
                                } else {
                                    "brightness(1.0)"
                                }
                            };

                            let label_clone = label.clone();
                            let color_clone = color.clone();
                            let on_click = on_click.clone();

                            view! {
                                <g
                                    class=seg_class
                                    style=move || format!(
                                        "transform: {}; opacity: {}; filter: {}; transition: all 0.2s ease; cursor: pointer;",
                                        transform(),
                                        opacity(),
                                        filter()
                                    )
                                    on:mouseenter=move |ev| {
                                        hovered_segment.set(Some(idx));
                                        tooltip_data.set(Some(FunnelTooltipData {
                                            label: label_clone.clone(),
                                            value,
                                            percentage,
                                            conversion_rate,
                                            color: color_clone.clone(),
                                            x: padding + label_width + (width - padding * 2.0 - label_width) / 2.0,
                                            y: center_y,
                                        }));
                                        // Get mouse position relative to container
                                        if let Some(container) = container_ref.get() {
                                            let rect = container.get_bounding_client_rect();
                                            let mx = ev.client_x() as f64 - rect.left();
                                            let my = ev.client_y() as f64 - rect.top();
                                            mouse_pos.set((mx, my));
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
                                        hovered_segment.set(None);
                                        tooltip_data.set(None);
                                    }
                                    on:click=move |_| {
                                        if let Some(ref callback) = on_click {
                                            callback.run(idx);
                                        }
                                    }
                                >
                                    // Segment shape
                                    <path
                                        d=path.clone()
                                        fill=color.clone()
                                        stroke=move || if theme.is_some() { "var(--chart-background)" } else { "#ffffff" }
                                        stroke-width="1"
                                        class=path_class
                                    />

                                    // Label on left
                                    {if show_labels {
                                        Some(view! {
                                            <text
                                                x=padding + label_width - 10.0
                                                y=center_y
                                                text-anchor="end"
                                                dominant-baseline="middle"
                                                fill=move || if theme.is_some() { "var(--chart-text)" } else { "#374151" }
                                                font-size="13"
                                                font-weight="500"
                                                style="pointer-events: none;"
                                            >
                                                {label.clone()}
                                            </text>
                                        })
                                    } else {
                                        None
                                    }}

                                    // Value/percentage inside segment
                                    {
                                        let display_text = format!("{:.0} ({:.1}%)", value, percentage);

                                        Some(view! {
                                            <text
                                                x=padding + label_width + (width - padding * 2.0 - label_width) / 2.0
                                                y=center_y
                                                text-anchor="middle"
                                                dominant-baseline="middle"
                                                fill="white"
                                                font-size="12"
                                                font-weight="600"
                                                style="pointer-events: none;"
                                            >
                                                {display_text}
                                            </text>
                                        })
                                    }
                                </g>
                            }
                        }
                    }).collect_view()
                }}
            </svg>

            // Tooltip
            {move || {
                let tooltip_class = format!("{}-tooltip", prefix_tooltip);
                tooltip_data.get().map(|data| {
                    // Use actual mouse position for tooltip placement
                    let (mx, my) = mouse_pos.get();
                    let tooltip_x = mx + 15.0;
                    let tooltip_y = my;
                    view! {
                        <div
                            class=tooltip_class
                            style=format!(
                                "position: absolute; left: {}px; top: {}px; background: var(--fx-color-bg-elevated, #1f1f1f); border: 1px solid var(--fx-color-border, #303030); border-radius: 6px; padding: 10px 14px; font-size: 12px; pointer-events: none; z-index: 10; box-shadow: 0 2px 8px rgba(0,0,0,0.3); min-width: 160px; transform: translateY(-50%);",
                                tooltip_x, tooltip_y
                            )
                        >
                            // Header with color indicator and label
                            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 8px;">
                                <span style=format!("width: 12px; height: 12px; border-radius: 2px; background: {};", data.color) />
                                <span style="color: var(--fx-color-text, #fff); font-weight: 600; font-size: 13px;">{data.label}</span>
                            </div>

                            // Stats grid
                            <div style="display: grid; grid-template-columns: auto auto; gap: 4px 12px; color: var(--fx-color-text-secondary, #8c8c8c);">
                                <span>"Value:"</span>
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500; text-align: right;">
                                    {format!("{:.0}", data.value)}
                                </span>

                                <span>"% of Total:"</span>
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500; text-align: right;">
                                    {format!("{:.1}%", data.percentage)}
                                </span>

                                <span>"Conversion:"</span>
                                <span style=format!(
                                    "font-weight: 500; text-align: right; color: {};",
                                    if data.conversion_rate >= 50.0 {
                                        "var(--fx-color-success, #52c41a)"
                                    } else if data.conversion_rate >= 25.0 {
                                        "var(--fx-color-warning, #faad14)"
                                    } else {
                                        "var(--fx-color-error, #ff4d4f)"
                                    }
                                )>
                                    {format!("{:.1}%", data.conversion_rate)}
                                </span>
                            </div>
                        </div>
                    }
                })
            }}

            // Loading overlay
            {move || {
                let loading_class = format!("{}-loading", prefix_loading);
                let spinner_class = format!("{}-spinner", prefix_spinner);
                if loading {
                    Some(view! {
                        <div class=loading_class>
                            <div class=spinner_class></div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
