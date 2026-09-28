//! BubbleChart Leptos component.

use leptos::prelude::*;
use super::types::{BubblePoint, BubbleChartConfig, ChartColor, Series, ColorPalette};
use super::utils::{scale_linear, min_max, nice_domain, Margins, ChartArea};
use crate::try_use_theme;

/// BubbleChart component.
///
/// Interactive bubble chart with three dimensions (x, y, size).
///
/// # Props
///
/// - `data` - Series of bubble points
/// - `config` - Chart configuration
/// - `on_bubble_click` - Click handler
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{BubbleChart, BubblePoint, Series};
///
/// let data = vec![
///     Series::new("Products", vec![
///         BubblePoint::new(100.0, 50.0, 25.0),
///         BubblePoint::new(150.0, 75.0, 40.0),
///         BubblePoint::new(80.0, 90.0, 15.0),
///     ]),
/// ];
///
/// view! {
///     <BubbleChart data=Signal::derive(move || data.clone()) />
/// }
/// ```
#[component]
pub fn BubbleChart(
    /// Series data.
    data: Signal<Vec<Series<BubblePoint>>>,
    /// Chart configuration.
    #[prop(optional)]
    config: Option<BubbleChartConfig>,
    /// Click handler (series_index, point_index).
    #[prop(optional, into)]
    on_bubble_click: Option<Callback<(usize, usize)>>,
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
    let chart_height = config.height;

    // Interactive state
    let hovered_bubble = RwSignal::new(None::<(usize, usize)>); // (series_index, point_index)
    let hovered_series = RwSignal::new(None::<usize>); // For legend hover
    // Tooltip: (series_name, x, y, z, cx, cy, color, label)
    let tooltip_data = RwSignal::new(None::<(String, f64, f64, f64, f64, f64, String, Option<String>)>);
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    let chart_prefix = format!("fx-chart-{}", design_system);
    let prefix_svg = format!("{}-svg", chart_prefix);
    let prefix_bubble = format!("{}-bubble-circle", chart_prefix);
    let combined_class = {
        let mut parts = vec![chart_prefix.clone(), format!("{}-bubble", chart_prefix)];
        if loading {
            parts.push(format!("{}-loading", chart_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Clone on_bubble_click for use in closures
    let on_click = on_bubble_click;

    let chart_width = 600.0;
    let margins = Margins::new(20.0, 40.0, 40.0, 50.0);
    let area = ChartArea::from_size(chart_width, chart_height as f64, margins);

    let palette = ColorPalette::Default;

    // Calculate bounds
    let x_bounds = move || {
        let series = data.get();
        let values: Vec<f64> = series.iter()
            .flat_map(|s| s.data.iter().map(|p| p.x))
            .collect();
        let (min, max) = min_max(&values);
        nice_domain(min, max)
    };

    let y_bounds = move || {
        let series = data.get();
        let values: Vec<f64> = series.iter()
            .flat_map(|s| s.data.iter().map(|p| p.y))
            .collect();
        let (min, max) = min_max(&values);
        nice_domain(min, max)
    };

    let z_bounds = move || {
        let series = data.get();
        let values: Vec<f64> = series.iter()
            .flat_map(|s| s.data.iter().map(|p| p.z))
            .collect();
        min_max(&values)
    };

    // Grid lines
    let grid_lines = move || {
        if !config.show_grid {
            return vec![];
        }

        let (x_min, x_max) = x_bounds();
        let (y_min, y_max) = y_bounds();
        let mut lines = vec![];

        for i in 0..=4 {
            let t = i as f64 / 4.0;
            let v = y_min + t * (y_max - y_min);
            let y = scale_linear((y_min, y_max), (area.bottom(), area.y), v);
            lines.push((area.x, y, area.right(), y));
        }

        for i in 0..=5 {
            let t = i as f64 / 5.0;
            let v = x_min + t * (x_max - x_min);
            let x = scale_linear((x_min, x_max), (area.x, area.right()), v);
            lines.push((x, area.y, x, area.bottom()));
        }

        lines
    };

    // Calculate bubbles
    let bubbles = move || {
        let series = data.get();
        let (x_min, x_max) = x_bounds();
        let (y_min, y_max) = y_bounds();
        let (z_min, z_max) = z_bounds();
        let min_r = config.min_radius as f64;
        let max_r = config.max_radius as f64;
        let opacity = config.opacity;

        let mut result = vec![];

        for (series_idx, s) in series.iter().enumerate() {
            let color = match s.color {
                ChartColor::Primary => palette.color_at(series_idx),
                ChartColor::Success => "var(--fx-color-success, #52c41a)",
                ChartColor::Warning => "var(--fx-color-warning, #faad14)",
                ChartColor::Error => "var(--fx-color-error, #ff4d4f)",
                ChartColor::Info => "var(--fx-color-info, #1890ff)",
                ChartColor::Gray => "var(--fx-color-text-tertiary, #8c8c8c)",
            };

            for (point_idx, point) in s.data.iter().enumerate() {
                let cx = scale_linear((x_min, x_max), (area.x, area.right()), point.x);
                let cy = scale_linear((y_min, y_max), (area.bottom(), area.y), point.y);

                // Scale Z to radius
                let r = if (z_max - z_min).abs() > f64::EPSILON {
                    let t = (point.z - z_min) / (z_max - z_min);
                    min_r + t * (max_r - min_r)
                } else {
                    (min_r + max_r) / 2.0
                };

                result.push((
                    series_idx,
                    point_idx,
                    cx,
                    cy,
                    r,
                    color.to_string(),
                    opacity,
                    point.label.clone(),
                    point.x,
                    point.y,
                    point.z,
                    s.name.clone(),
                ));
            }
        }

        // Sort by size (larger bubbles first, so smaller ones render on top)
        result.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal));
        result
    };

    // Axis labels
    let x_axis_labels = move || {
        let (x_min, x_max) = x_bounds();
        (0..=5).map(|i| {
            let t = i as f64 / 5.0;
            let v = x_min + t * (x_max - x_min);
            let x = scale_linear((x_min, x_max), (area.x, area.right()), v);
            (format!("{:.0}", v), x, area.bottom() + 16.0)
        }).collect::<Vec<_>>()
    };

    let y_axis_labels = move || {
        let (y_min, y_max) = y_bounds();
        (0..=4).map(|i| {
            let t = i as f64 / 4.0;
            let v = y_min + t * (y_max - y_min);
            let y = scale_linear((y_min, y_max), (area.bottom(), area.y), v);
            (format!("{:.0}", v), area.x - 8.0, y)
        }).collect::<Vec<_>>()
    };

    view! {
        <div class=combined_class style="position: relative;" node_ref=container_ref>
            <svg
                width="100%"
                height=chart_height
                viewBox=format!("0 0 {} {}", chart_width, chart_height)
                preserveAspectRatio="xMidYMid meet"
                class=prefix_svg.clone()
            >
                // Grid lines
                {move || grid_lines().into_iter().map(|(x1, y1, x2, y2)| {
                    view! {
                        <line
                            x1=x1
                            y1=y1
                            x2=x2
                            y2=y2
                            stroke="var(--fx-color-border, #303030)"
                            stroke-width="1"
                            stroke-dasharray="2,2"
                        />
                    }
                }).collect::<Vec<_>>()}

                // Bubbles with interactivity
                {
                    let prefix = prefix_bubble.clone();
                    let on_click = on_click.clone();
                    move || bubbles().into_iter().map({
                        let prefix = prefix.clone();
                        let on_click = on_click.clone();
                        move |(series_idx, point_idx, cx, cy, r, color, base_opacity, label, x_val, y_val, z_val, series_name)| {
                            let is_hovered = move || hovered_bubble.get() == Some((series_idx, point_idx));
                            let series_is_hovered = move || hovered_series.get() == Some(series_idx);

                            // Calculate radius: slight scale up on hover
                            let radius = move || {
                                if is_hovered() {
                                    r * 1.15
                                } else if series_is_hovered() {
                                    r * 1.08
                                } else {
                                    r
                                }
                            };

                            // Calculate opacity: dim non-hovered bubbles when any is hovered
                            let opacity = move || {
                                match (hovered_bubble.get(), hovered_series.get()) {
                                    (Some((h_series, h_point)), _) => {
                                        if h_series == series_idx && h_point == point_idx {
                                            base_opacity.min(1.0)
                                        } else {
                                            0.4
                                        }
                                    }
                                    (None, Some(h_series)) => {
                                        if h_series == series_idx {
                                            base_opacity.min(1.0)
                                        } else {
                                            0.4
                                        }
                                    }
                                    _ => base_opacity,
                                }
                            };

                            // Stroke for hovered bubble
                            let color_for_stroke = color.clone();
                            let stroke = move || {
                                if is_hovered() {
                                    "var(--fx-color-bg, #141414)".to_string()
                                } else {
                                    color_for_stroke.clone()
                                }
                            };

                            let stroke_width = move || {
                                if is_hovered() { 3.0 } else { 1.0 }
                            };

                            let stroke_opacity = move || {
                                if is_hovered() { 1.0 } else { 0.8 }
                            };

                            let series_name_clone = series_name.clone();
                            let color_clone = color.clone();
                            let label_clone = label.clone();
                            let on_click = on_click.clone();
                            let color_fill = color.clone();

                            view! {
                                <circle
                                    cx=cx
                                    cy=cy
                                    r=radius
                                    fill=color_fill
                                    stroke=stroke
                                    stroke-width=stroke_width
                                    stroke-opacity=stroke_opacity
                                    class=prefix.clone()
                                    style=move || format!(
                                        "opacity: {}; transition: all 0.2s ease; cursor: pointer;",
                                        opacity()
                                    )
                                    on:mouseenter=move |ev| {
                                        hovered_bubble.set(Some((series_idx, point_idx)));
                                        tooltip_data.set(Some((
                                            series_name_clone.clone(),
                                            x_val,
                                            y_val,
                                            z_val,
                                            cx,
                                            cy,
                                            color_clone.clone(),
                                            label_clone.clone(),
                                        )));
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
                                        hovered_bubble.set(None);
                                        tooltip_data.set(None);
                                    }
                                    on:click=move |_| {
                                        if let Some(ref callback) = on_click {
                                            callback.run((series_idx, point_idx));
                                        }
                                    }
                                />
                            }
                        }
                    }).collect::<Vec<_>>()
                }

                // X-axis labels
                {move || x_axis_labels().into_iter().map(|(label, x, y)| {
                    view! {
                        <text
                            x=x
                            y=y
                            fill="var(--fx-color-text-secondary, #8c8c8c)"
                            font-size="10"
                            text-anchor="middle"
                        >
                            {label}
                        </text>
                    }
                }).collect::<Vec<_>>()}

                // Y-axis labels
                {move || y_axis_labels().into_iter().map(|(label, x, y)| {
                    view! {
                        <text
                            x=x
                            y=y
                            fill="var(--fx-color-text-secondary, #8c8c8c)"
                            font-size="10"
                            text-anchor="end"
                            dominant-baseline="middle"
                        >
                            {label}
                        </text>
                    }
                }).collect::<Vec<_>>()}

                // Axes
                <line
                    x1=area.x
                    y1=area.bottom()
                    x2=area.right()
                    y2=area.bottom()
                    stroke="var(--fx-color-text-tertiary, #595959)"
                    stroke-width="1"
                />
                <line
                    x1=area.x
                    y1=area.y
                    x2=area.x
                    y2=area.bottom()
                    stroke="var(--fx-color-text-tertiary, #595959)"
                    stroke-width="1"
                />
            </svg>

            // Tooltip
            {
            let chart_prefix_tooltip = chart_prefix.clone();
            move || {
                tooltip_data.get().map(|(series_name, x_val, y_val, z_val, _data_cx, _data_cy, color, label)| {
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
                                <span style=format!("width: 10px; height: 10px; border-radius: 50%; background: {};", color) />
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500;">{series_name}</span>
                            </div>
                            <div style="color: var(--fx-color-text-secondary, #8c8c8c);">
                                {format!("X: {:.2}", x_val)}
                            </div>
                            <div style="color: var(--fx-color-text-secondary, #8c8c8c);">
                                {format!("Y: {:.2}", y_val)}
                            </div>
                            <div style="color: var(--fx-color-text-secondary, #8c8c8c);">
                                {format!("Size: {:.2}", z_val)}
                            </div>
                            {label.map(|l| view! {
                                <div style="color: var(--fx-color-text-tertiary, #595959); font-style: italic; margin-top: 4px;">
                                    {l}
                                </div>
                            })}
                        </div>
                    }
                })
            }}

            // Legend with hover interactivity
            {
            let chart_prefix_legend = chart_prefix.clone();
            config.show_legend.then(|| {
                let legend_items = move || {
                    data.get().iter().enumerate().map(|(i, s)| {
                        let color = match s.color {
                            ChartColor::Primary => palette.color_at(i),
                            ChartColor::Success => "var(--fx-color-success, #52c41a)",
                            ChartColor::Warning => "var(--fx-color-warning, #faad14)",
                            ChartColor::Error => "var(--fx-color-error, #ff4d4f)",
                            ChartColor::Info => "var(--fx-color-info, #1890ff)",
                            ChartColor::Gray => "var(--fx-color-text-tertiary, #8c8c8c)",
                        };
                        (i, s.name.clone(), color.to_string(), s.data.len())
                    }).collect::<Vec<_>>()
                };

                view! {
                    <div
                        class=format!("{}-legend", chart_prefix_legend)
                        style="display: flex; flex-wrap: wrap; justify-content: center; gap: 12px; padding: 8px; font-size: 12px;"
                    >
                        {move || legend_items().into_iter().map(|(idx, name, color, bubble_count)| {
                            let is_hovered = move || hovered_series.get() == Some(idx);
                            view! {
                                <span
                                    style=move || format!(
                                        "display: flex; align-items: center; gap: 6px; color: var(--fx-color-text-secondary, #8c8c8c); cursor: pointer; padding: 2px 6px; border-radius: 4px; transition: all 0.2s ease; {}",
                                        if is_hovered() { "background: var(--fx-color-bg-elevated, #1f1f1f);" } else { "" }
                                    )
                                    on:mouseenter=move |_| {
                                        hovered_series.set(Some(idx));
                                    }
                                    on:mouseleave=move |_| {
                                        hovered_series.set(None);
                                    }
                                >
                                    <span style=format!(
                                        "width: 12px; height: 12px; border-radius: 50%; background: {}; opacity: 0.7;",
                                        color
                                    ) />
                                    {name}
                                    <span style="color: var(--fx-color-text-tertiary, #595959);">
                                        {format!("({})", bubble_count)}
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
