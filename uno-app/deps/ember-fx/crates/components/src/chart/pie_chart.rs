//! PieChart Leptos component.

use leptos::prelude::*;
use std::f64::consts::PI;
use super::types::{PieSlice, PieChartConfig, ChartColor, ColorPalette};
use super::utils::pie_slice_path;
use crate::try_use_theme;

/// PieChart component.
///
/// Interactive pie and donut chart visualization with hover effects and tooltips.
///
/// # Props
///
/// - `data` - Pie slice data
/// - `config` - Chart configuration
/// - `on_slice_click` - Click handler (slice index)
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{PieChart, PieSlice};
///
/// let data = vec![
///     PieSlice::new("Product A", 35.0),
///     PieSlice::new("Product B", 25.0),
///     PieSlice::new("Product C", 20.0),
///     PieSlice::new("Other", 20.0),
/// ];
///
/// view! {
///     <PieChart data=Signal::derive(move || data.clone()) />
/// }
/// ```
#[component]
pub fn PieChart(
    /// Slice data.
    data: Signal<Vec<PieSlice>>,
    /// Chart configuration.
    #[prop(optional)]
    config: Option<PieChartConfig>,
    /// Click handler (slice index).
    #[prop(optional, into)]
    on_slice_click: Option<Callback<usize>>,
    /// Loading state.
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let chart_size = config.height;

    // Interactive state
    let hovered_slice = RwSignal::new(None::<usize>);
    let tooltip_data = RwSignal::new(None::<(String, f64, f64, String)>); // (label, value, percentage, color)
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    let chart_prefix = format!("fx-chart-{}", design_system);
    let prefix_svg = format!("{}-svg", chart_prefix);
    let prefix_slice = format!("{}-slice", chart_prefix);
    let combined_class = {
        let mut parts = vec![chart_prefix.clone(), format!("{}-pie", chart_prefix)];
        if config.donut {
            parts.push(format!("{}-donut", chart_prefix));
        }
        if loading {
            parts.push(format!("{}-loading", chart_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let palette = ColorPalette::Default;

    // Center and radius
    let cx = chart_size as f64 / 2.0;
    let cy = chart_size as f64 / 2.0;
    let outer_radius = (chart_size as f64 / 2.0) - 20.0;
    let inner_radius = if config.donut {
        outer_radius * config.donut_width as f64
    } else {
        0.0
    };

    // Calculate slices
    let slices = move || {
        let slices_data = data.get();
        let total: f64 = slices_data.iter().map(|s| s.value).sum();

        if total <= 0.0 || slices_data.is_empty() {
            return vec![];
        }

        let start_angle = config.start_angle as f64;
        let mut current_angle = start_angle;
        let mut result = vec![];

        for (i, slice) in slices_data.iter().enumerate() {
            let slice_angle = (slice.value / total) * 360.0;
            let end_angle = current_angle + slice_angle;

            // Get color
            let color = slice.color
                .map(|c| match c {
                    ChartColor::Primary => palette.color_at(i).to_string(),
                    ChartColor::Success => "var(--fx-color-success, #52c41a)".to_string(),
                    ChartColor::Warning => "var(--fx-color-warning, #faad14)".to_string(),
                    ChartColor::Error => "var(--fx-color-error, #ff4d4f)".to_string(),
                    ChartColor::Info => "var(--fx-color-info, #1890ff)".to_string(),
                    ChartColor::Gray => "var(--fx-color-text-tertiary, #8c8c8c)".to_string(),
                })
                .unwrap_or_else(|| palette.color_at(i).to_string());

            // Path
            let path = pie_slice_path(cx, cy, outer_radius, inner_radius, current_angle, end_angle);

            // Label position (midpoint of arc)
            let mid_angle = (current_angle + end_angle) / 2.0;
            let mid_rad = mid_angle * PI / 180.0;
            let label_radius = if config.donut {
                (outer_radius + inner_radius) / 2.0
            } else {
                outer_radius * 0.65
            };
            let label_x = cx + label_radius * mid_rad.cos();
            let label_y = cy + label_radius * mid_rad.sin();

            // Percentage
            let percentage = (slice.value / total) * 100.0;

            result.push((
                i,
                slice.label.clone(),
                path,
                color,
                label_x,
                label_y,
                percentage,
                slice.value,
            ));

            current_angle = end_angle;
        }

        result
    };

    view! {
        <div class=combined_class style="position: relative;" node_ref=container_ref>
            <svg
                width="100%"
                height=chart_size
                viewBox=format!("0 0 {} {}", chart_size, chart_size)
                preserveAspectRatio="xMidYMid meet"
                class=prefix_svg.clone()
            >
                // Slices with interactivity
                {
                    let prefix = prefix_slice.clone();
                    let on_click = on_slice_click.clone();
                    move || slices().into_iter().map({
                        let prefix = prefix.clone();
                        let on_click = on_click.clone();
                        move |(idx, label, path, color, _lx, _ly, pct, value)| {
                            let is_hovered = move || hovered_slice.get() == Some(idx);
                            let transform = move || {
                                if is_hovered() {
                                    format!("scale(1.03) translate({}, {})",
                                        -cx * 0.015, -cy * 0.015)
                                } else {
                                    String::new()
                                }
                            };
                            let opacity = move || {
                                match hovered_slice.get() {
                                    Some(h) if h != idx => 0.6,
                                    _ => 1.0,
                                }
                            };

                            let label_clone = label.clone();
                            let label_clone2 = label.clone();
                            let color_clone = color.clone();
                            let color_clone2 = color.clone();
                            let on_click = on_click.clone();

                            view! {
                                <path
                                    d=path
                                    fill=color.clone()
                                    stroke="var(--fx-color-bg, #141414)"
                                    stroke-width="2"
                                    class=prefix.clone()
                                    style=move || format!(
                                        "transform-origin: {}px {}px; transform: {}; opacity: {}; transition: all 0.2s ease; cursor: pointer;",
                                        cx, cy, transform(), opacity()
                                    )
                                    on:mouseenter=move |ev| {
                                        hovered_slice.set(Some(idx));
                                        tooltip_data.set(Some((label_clone.clone(), value, pct, color_clone.clone())));
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
                                        tooltip_data.set(Some((label_clone2.clone(), value, pct, color_clone2.clone())));
                                        if let Some(container) = container_ref.get() {
                                            let rect = container.get_bounding_client_rect();
                                            let mx = ev.client_x() as f64 - rect.left();
                                            let my = ev.client_y() as f64 - rect.top();
                                            mouse_pos.set((mx, my));
                                        }
                                    }
                                    on:mouseleave=move |_| {
                                        hovered_slice.set(None);
                                        tooltip_data.set(None);
                                    }
                                    on:click=move |_| {
                                        if let Some(ref callback) = on_click {
                                            callback.run(idx);
                                        }
                                    }
                                />
                            }
                        }
                    }).collect::<Vec<_>>()
                }

                // Labels
                {move || {
                    if !config.show_labels {
                        return vec![];
                    }
                    slices().into_iter()
                        .filter(|(_, _, _, _, _, _, pct, _)| *pct >= 5.0)
                        .map(|(idx, _, _, _, lx, ly, pct, _)| {
                            let is_hovered = move || hovered_slice.get() == Some(idx);
                            view! {
                                <text
                                    x=lx
                                    y=ly
                                    fill="var(--fx-color-text, #ffffff)"
                                    font-size="11"
                                    font-weight="500"
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    style=move || format!(
                                        "opacity: {}; transition: opacity 0.2s ease; pointer-events: none;",
                                        if is_hovered() { 1.0 } else { 0.9 }
                                    )
                                >
                                    {format!("{:.0}%", pct)}
                                </text>
                            }
                        }).collect::<Vec<_>>()
                }}

                // Center label for donut
                {config.donut.then(|| {
                    let total = move || data.get().iter().map(|s| s.value).sum::<f64>();
                    view! {
                        <text
                            x=cx
                            y=cy - 8.0
                            fill="var(--fx-color-text-secondary, #8c8c8c)"
                            font-size="12"
                            text-anchor="middle"
                        >
                            "Total"
                        </text>
                        <text
                            x=cx
                            y=cy + 10.0
                            fill="var(--fx-color-text, #ffffff)"
                            font-size="18"
                            font-weight="600"
                            text-anchor="middle"
                        >
                            {move || format!("{:.0}", total())}
                        </text>
                    }
                })}
            </svg>

            // Tooltip
            {
            let chart_prefix_tooltip = chart_prefix.clone();
            move || {
                tooltip_data.get().map(|(label, value, pct, color)| {
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
                            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 4px;">
                                <span style=format!("width: 10px; height: 10px; border-radius: 2px; background: {};", color) />
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500;">{label}</span>
                            </div>
                            <div style="color: var(--fx-color-text-secondary, #8c8c8c);">
                                {format!("Value: {:.1} ({:.1}%)", value, pct)}
                            </div>
                        </div>
                    }
                })
            }}

            // Legend
            {
            let chart_prefix_legend = chart_prefix.clone();
            config.show_legend.then(|| {
                let legend_items = move || {
                    data.get().iter().enumerate().map(|(i, s)| {
                        let color = s.color
                            .map(|c| match c {
                                ChartColor::Primary => palette.color_at(i).to_string(),
                                ChartColor::Success => "var(--fx-color-success, #52c41a)".to_string(),
                                ChartColor::Warning => "var(--fx-color-warning, #faad14)".to_string(),
                                ChartColor::Error => "var(--fx-color-error, #ff4d4f)".to_string(),
                                ChartColor::Info => "var(--fx-color-info, #1890ff)".to_string(),
                                ChartColor::Gray => "var(--fx-color-text-tertiary, #8c8c8c)".to_string(),
                            })
                            .unwrap_or_else(|| palette.color_at(i).to_string());
                        (i, s.label.clone(), color, s.value)
                    }).collect::<Vec<_>>()
                };

                view! {
                    <div
                        class=format!("{}-legend", chart_prefix_legend)
                        style="display: flex; flex-wrap: wrap; justify-content: center; gap: 12px; padding: 8px; font-size: 12px;"
                    >
                        {move || legend_items().into_iter().map(|(idx, name, color, value)| {
                            let is_hovered = move || hovered_slice.get() == Some(idx);
                            view! {
                                <span
                                    style=move || format!(
                                        "display: flex; align-items: center; gap: 6px; color: var(--fx-color-text-secondary, #8c8c8c); cursor: pointer; padding: 2px 6px; border-radius: 4px; transition: all 0.2s ease; {}",
                                        if is_hovered() { "background: var(--fx-color-bg-elevated, #1f1f1f);" } else { "" }
                                    )
                                    on:mouseenter=move |_| {
                                        hovered_slice.set(Some(idx));
                                    }
                                    on:mouseleave=move |_| {
                                        hovered_slice.set(None);
                                    }
                                >
                                    <span style=format!(
                                        "width: 10px; height: 10px; border-radius: 2px; background: {};",
                                        color
                                    ) />
                                    {name}
                                    <span style="color: var(--fx-color-text-tertiary, #595959);">
                                        {format!("({:.0})", value)}
                                    </span>
                                </span>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }
            })
            }

            {loading.then(|| view! {
                <div class=format!("{}-loading-overlay", chart_prefix)>
                    <span class=format!("{}-loading-spinner", chart_prefix)></span>
                </div>
            })}
        </div>
    }
}
