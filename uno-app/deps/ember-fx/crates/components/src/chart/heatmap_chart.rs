//! HeatmapChart Leptos component.

use leptos::prelude::*;
use super::types::{HeatmapCell, HeatmapChartConfig, ColorScale};
use super::utils::{min_max, interpolate_color, Margins, ChartArea};
use crate::try_use_theme;

/// HeatmapChart component.
///
/// Interactive matrix heatmap visualization for showing value density with hover effects and tooltips.
///
/// # Props
///
/// - `data` - Cell data
/// - `x_categories` - X axis categories (optional, derived from data)
/// - `y_categories` - Y axis categories (optional, derived from data)
/// - `config` - Chart configuration
/// - `on_cell_click` - Click handler (x_category, y_category)
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{HeatmapChart, HeatmapCell};
///
/// let data = vec![
///     HeatmapCell::new("Mon", "9am", 10.0),
///     HeatmapCell::new("Mon", "10am", 25.0),
///     HeatmapCell::new("Tue", "9am", 15.0),
/// ];
///
/// view! {
///     <HeatmapChart data=Signal::derive(move || data.clone()) />
/// }
/// ```
#[component]
pub fn HeatmapChart(
    /// Cell data.
    data: Signal<Vec<HeatmapCell>>,
    /// X categories (optional, derived from data if not provided).
    #[prop(optional)]
    x_categories: Option<Signal<Vec<String>>>,
    /// Y categories (optional, derived from data if not provided).
    #[prop(optional)]
    y_categories: Option<Signal<Vec<String>>>,
    /// Chart configuration.
    #[prop(optional)]
    config: Option<HeatmapChartConfig>,
    /// Click handler (x_category, y_category).
    #[prop(optional, into)]
    on_cell_click: Option<Callback<(String, String)>>,
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
    let hovered_cell = RwSignal::new(None::<(String, String)>); // (x_category, y_category)
    let tooltip_data = RwSignal::new(None::<(String, String, f64, f64, f64, String)>); // (x_cat, y_cat, value, screen_x, screen_y, color)
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    let chart_prefix = format!("fx-chart-{}", design_system);
    let prefix_svg = format!("{}-svg", chart_prefix);
    let prefix_cell = format!("{}-cell", chart_prefix);
    let combined_class = {
        let mut parts = vec![chart_prefix.clone(), format!("{}-heatmap", chart_prefix)];
        if loading {
            parts.push(format!("{}-loading", chart_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let chart_width = 600.0;
    let margins = Margins::new(20.0, 80.0, 50.0, 80.0);
    let area = ChartArea::from_size(chart_width, chart_height as f64, margins);

    // Derive categories from data
    let resolved_x_categories = move || {
        if let Some(ref cats) = x_categories {
            cats.get()
        } else {
            let mut cats: Vec<String> = data.get().iter().map(|c| c.x.clone()).collect();
            cats.sort();
            cats.dedup();
            cats
        }
    };

    let resolved_y_categories = move || {
        if let Some(ref cats) = y_categories {
            cats.get()
        } else {
            let mut cats: Vec<String> = data.get().iter().map(|c| c.y.clone()).collect();
            cats.sort();
            cats.dedup();
            cats
        }
    };

    // Value bounds
    let value_bounds = move || {
        let values: Vec<f64> = data.get().iter().map(|c| c.value).collect();
        min_max(&values)
    };

    // Calculate cells with full data for interactivity
    let cells = move || {
        let cells_data = data.get();
        let x_cats = resolved_x_categories();
        let y_cats = resolved_y_categories();
        let (v_min, v_max) = value_bounds();
        let cell_radius = config.cell_radius as f64;
        let show_values = config.show_values;

        if x_cats.is_empty() || y_cats.is_empty() {
            return vec![];
        }

        let cell_width = area.width / x_cats.len() as f64;
        let cell_height = area.height / y_cats.len() as f64;

        let (color_low, color_high) = match config.color_scale {
            ColorScale::Sequential => ("#1f1f1f", "#1890ff"),
            ColorScale::Diverging => ("#ff4d4f", "#52c41a"),
            ColorScale::Categorical => ("#1890ff", "#1890ff"),
        };

        let mut result = vec![];

        for cell in cells_data.iter() {
            let x_idx = x_cats.iter().position(|c| c == &cell.x);
            let y_idx = y_cats.iter().position(|c| c == &cell.y);

            if let (Some(xi), Some(yi)) = (x_idx, y_idx) {
                let x = area.x + xi as f64 * cell_width;
                let y = area.y + yi as f64 * cell_height;

                // Calculate color based on value
                let t = if (v_max - v_min).abs() > f64::EPSILON {
                    (cell.value - v_min) / (v_max - v_min)
                } else {
                    0.5
                };

                let color = interpolate_color(color_low, color_high, t);

                // Include x_cat, y_cat, and tooltip position (center of cell)
                let tooltip_x = x + cell_width / 2.0;
                let tooltip_y = y + cell_height / 2.0;

                result.push((
                    cell.x.clone(),     // x_cat
                    cell.y.clone(),     // y_cat
                    x,                  // rect x
                    y,                  // rect y
                    cell_width - 2.0,   // width
                    cell_height - 2.0,  // height
                    color,              // fill color
                    cell_radius,        // corner radius
                    cell.value,         // value
                    show_values,        // show value text
                    tooltip_x,          // tooltip x position
                    tooltip_y,          // tooltip y position
                ));
            }
        }

        result
    };

    // X-axis labels
    let x_labels = move || {
        let x_cats = resolved_x_categories();
        let cell_width = area.width / x_cats.len().max(1) as f64;

        x_cats.into_iter().enumerate().map(|(i, label)| {
            let x = area.x + (i as f64 + 0.5) * cell_width;
            (label, x, area.bottom() + 16.0)
        }).collect::<Vec<_>>()
    };

    // Y-axis labels
    let y_labels = move || {
        let y_cats = resolved_y_categories();
        let cell_height = area.height / y_cats.len().max(1) as f64;

        y_cats.into_iter().enumerate().map(|(i, label)| {
            let y = area.y + (i as f64 + 0.5) * cell_height;
            (label, area.x - 8.0, y)
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
                // Cells with interactivity
                {
                    let prefix = prefix_cell.clone();
                    let on_click = on_cell_click.clone();
                    move || cells().into_iter().map({
                        let prefix = prefix.clone();
                        let on_click = on_click.clone();
                        move |(x_cat, y_cat, x, y, w, h, color, radius, value, show_val, tip_x, tip_y)| {
                            let x_cat_clone = x_cat.clone();
                            let y_cat_clone = y_cat.clone();
                            let x_cat_click = x_cat.clone();
                            let y_cat_click = y_cat.clone();
                            let color_clone = color.clone();
                            let on_click = on_click.clone();

                            // Check if this cell is hovered
                            let is_hovered = {
                                let x_cat = x_cat.clone();
                                let y_cat = y_cat.clone();
                                move || {
                                    hovered_cell.get().as_ref()
                                        .map(|(hx, hy)| hx == &x_cat && hy == &y_cat)
                                        .unwrap_or(false)
                                }
                            };

                            // Opacity dims non-hovered cells when any cell is hovered
                            let opacity = {
                                let x_cat = x_cat.clone();
                                let y_cat = y_cat.clone();
                                move || {
                                    match hovered_cell.get().as_ref() {
                                        Some((hx, hy)) if hx != &x_cat || hy != &y_cat => 0.7,
                                        _ => 1.0,
                                    }
                                }
                            };

                            // Stroke attributes for hover highlight
                            let x_cat_stroke = x_cat.clone();
                            let y_cat_stroke = y_cat.clone();
                            let x_cat_sw = x_cat.clone();
                            let y_cat_sw = y_cat.clone();
                            let stroke = move || {
                                let is_hovered = hovered_cell.get().as_ref()
                                    .map(|(hx, hy)| hx == &x_cat_stroke && hy == &y_cat_stroke)
                                    .unwrap_or(false);
                                if is_hovered { "white" } else { "none" }
                            };
                            let stroke_width = move || {
                                let is_hovered = hovered_cell.get().as_ref()
                                    .map(|(hx, hy)| hx == &x_cat_sw && hy == &y_cat_sw)
                                    .unwrap_or(false);
                                if is_hovered { "2" } else { "0" }
                            };

                            view! {
                                <g>
                                    <rect
                                        x=x + 1.0
                                        y=y + 1.0
                                        width=w
                                        height=h
                                        fill=color.clone()
                                        rx=radius
                                        ry=radius
                                        stroke=stroke
                                        stroke-width=stroke_width
                                        class=prefix.clone()
                                        style=move || format!(
                                            "opacity: {}; transition: all 0.2s ease; cursor: pointer;",
                                            opacity()
                                        )
                                        on:mouseenter=move |ev| {
                                            hovered_cell.set(Some((x_cat_clone.clone(), y_cat_clone.clone())));
                                            tooltip_data.set(Some((
                                                x_cat_clone.clone(),
                                                y_cat_clone.clone(),
                                                value,
                                                tip_x,
                                                tip_y,
                                                color_clone.clone(),
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
                                            hovered_cell.set(None);
                                            tooltip_data.set(None);
                                        }
                                        on:click=move |_| {
                                            if let Some(ref callback) = on_click {
                                                callback.run((x_cat_click.clone(), y_cat_click.clone()));
                                            }
                                        }
                                    />
                                    {show_val.then(|| view! {
                                        <text
                                            x=x + w / 2.0 + 1.0
                                            y=y + h / 2.0 + 1.0
                                            fill="var(--fx-color-text, #ffffff)"
                                            font-size="10"
                                            text-anchor="middle"
                                            dominant-baseline="middle"
                                            style="pointer-events: none;"
                                        >
                                            {format!("{:.0}", value)}
                                        </text>
                                    })}
                                </g>
                            }
                        }
                    }).collect::<Vec<_>>()
                }

                // X-axis labels
                {move || x_labels().into_iter().map(|(label, x, y)| {
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
                {move || y_labels().into_iter().map(|(label, x, y)| {
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
            </svg>

            // Tooltip
            {
            let chart_prefix_tooltip = chart_prefix.clone();
            move || {
                tooltip_data.get().map(|(x_cat, y_cat, value, _data_tip_x, _data_tip_y, color)| {
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
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500;">
                                    {format!("{} / {}", x_cat, y_cat)}
                                </span>
                            </div>
                            <div style="color: var(--fx-color-text-secondary, #8c8c8c);">
                                {format!("Value: {:.1}", value)}
                            </div>
                        </div>
                    }
                })
            }}

            // Color scale legend
            {
            let chart_prefix_legend = chart_prefix.clone();
            config.show_legend.then(|| {
                let (v_min, v_max) = value_bounds();
                view! {
                    <div
                        class=format!("{}-legend", chart_prefix_legend)
                        style="display: flex; align-items: center; justify-content: center; gap: 8px; padding: 8px; font-size: 11px;"
                    >
                        <span style="color: var(--fx-color-text-secondary, #8c8c8c);">
                            {format!("{:.0}", v_min)}
                        </span>
                        <div style="width: 100px; height: 12px; border-radius: 2px; background: linear-gradient(to right, #1f1f1f, #1890ff);" />
                        <span style="color: var(--fx-color-text-secondary, #8c8c8c);">
                            {format!("{:.0}", v_max)}
                        </span>
                    </div>
                }
            })}

            {loading.then(|| view! {
                <div class=format!("{}-loading-overlay", chart_prefix)>
                    <span class=format!("{}-loading-spinner", chart_prefix)></span>
                </div>
            })}
        </div>
    }
}
