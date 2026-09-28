//! ColumnChart Leptos component.

use leptos::prelude::*;
use super::types::{CategoryPoint, BarChartConfig, ChartColor, Series, ColorPalette};
use super::utils::{scale_linear, scale_band, min_max, nice_domain, Margins, ChartArea};
use crate::try_use_theme;

/// ColumnChart component.
///
/// Vertical column chart for categorical data comparison.
///
/// # Props
///
/// - `data` - Series of categorical data points
/// - `categories` - Optional category labels
/// - `config` - Chart configuration (uses BarChartConfig)
/// - `on_column_click` - Click handler
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{ColumnChart, CategoryPoint, Series};
///
/// let data = vec![
///     Series::new("Revenue", vec![
///         CategoryPoint::new("Jan", 100.0),
///         CategoryPoint::new("Feb", 150.0),
///         CategoryPoint::new("Mar", 120.0),
///     ]),
/// ];
///
/// view! {
///     <ColumnChart data=Signal::derive(move || data.clone()) />
/// }
/// ```
#[component]
pub fn ColumnChart(
    /// Series data.
    data: Signal<Vec<Series<CategoryPoint>>>,
    /// Optional category labels.
    #[prop(optional)]
    categories: Option<Signal<Vec<String>>>,
    /// Chart configuration.
    #[prop(optional)]
    config: Option<BarChartConfig>,
    /// Click handler (series_index, category_index).
    #[prop(optional, into)]
    on_column_click: Option<Callback<(usize, usize)>>,
    /// Loading state.
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve config
    let config = config.unwrap_or_default();
    let chart_height = config.height;

    // Interactive state - (series_index, category_index)
    let hovered_column = RwSignal::new(None::<(usize, usize)>);
    // Tooltip data: (series_name, category, value, x, y)
    let tooltip_data = RwSignal::new(None::<(String, String, f64, f64, f64)>);
    // Hovered legend series index
    let hovered_legend = RwSignal::new(None::<usize>);
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    let chart_prefix = format!("fx-chart-{}", design_system);
    let prefix_svg = format!("{}-svg", chart_prefix);
    let prefix_column = format!("{}-column-rect", chart_prefix);
    let combined_class = {
        let mut parts = vec![chart_prefix.clone(), format!("{}-column", chart_prefix)];
        if loading {
            parts.push(format!("{}-loading", chart_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let chart_width = 600.0;
    let margins = Margins::new(20.0, 20.0, 50.0, 50.0);
    let area = ChartArea::from_size(chart_width, chart_height as f64, margins);

    let palette = ColorPalette::Default;

    let resolved_categories = move || {
        if let Some(ref cats) = categories {
            cats.get()
        } else {
            let series_data = data.get();
            if let Some(first_series) = series_data.first() {
                first_series.data.iter().map(|p| p.category.clone()).collect()
            } else {
                vec![]
            }
        }
    };

    let value_bounds = move || {
        let series = data.get();
        let stacked = config.stacked;

        if stacked {
            let cats = resolved_categories();
            let mut max_sum = 0.0_f64;
            let mut min_sum = 0.0_f64;

            for (i, _cat) in cats.iter().enumerate() {
                let sum: f64 = series.iter()
                    .filter_map(|s| s.data.get(i).map(|p| p.value))
                    .sum();
                max_sum = max_sum.max(sum);
                if sum < 0.0 {
                    min_sum = min_sum.min(sum);
                }
            }
            nice_domain(min_sum.min(0.0), max_sum)
        } else {
            let values: Vec<f64> = series.iter()
                .flat_map(|s| s.data.iter().map(|p| p.value))
                .collect();
            let (min, max) = min_max(&values);
            nice_domain(min.min(0.0), max)
        }
    };

    let grid_lines = move || {
        if !config.show_grid {
            return vec![];
        }

        let (v_min, v_max) = value_bounds();
        let mut lines = vec![];

        for i in 0..=4 {
            let t = i as f64 / 4.0;
            let v = v_min + t * (v_max - v_min);
            let y = scale_linear((v_min, v_max), (area.bottom(), area.y), v);
            lines.push((area.x, y, area.right(), y));
        }
        lines
    };

    let columns = move || {
        let series = data.get();
        let cats = resolved_categories();
        let cat_count = cats.len();
        let series_count = series.len();
        let (v_min, v_max) = value_bounds();
        let stacked = config.stacked;
        let bar_ratio = config.bar_width as f64;
        let border_radius = config.border_radius;

        if cat_count == 0 || series_count == 0 {
            return vec![];
        }

        let mut result = vec![];
        let zero_y = scale_linear((v_min, v_max), (area.bottom(), area.y), 0.0);

        for (cat_idx, cat_name) in cats.iter().enumerate() {
            let (band_start, band_width) = scale_band(
                cat_count,
                (area.x, area.right()),
                cat_idx,
                1.0 - bar_ratio as f32,
            );

            if stacked {
                // Stacked columns
                let mut pos_offset = zero_y;
                let mut neg_offset = zero_y;

                for (series_idx, s) in series.iter().enumerate() {
                    if let Some(point) = s.data.iter().find(|p| &p.category == cat_name) {
                        let value = point.value;
                        let bar_height = ((value / (v_max - v_min)) * area.height).abs();

                        let (bar_y, _) = if value >= 0.0 {
                            pos_offset -= bar_height;
                            (pos_offset, bar_height)
                        } else {
                            let y = neg_offset;
                            neg_offset += bar_height;
                            (y, bar_height)
                        };

                        let color = match s.color {
                            ChartColor::Primary => palette.color_at(series_idx),
                            ChartColor::Success => "var(--fx-color-success, #52c41a)",
                            ChartColor::Warning => "var(--fx-color-warning, #faad14)",
                            ChartColor::Error => "var(--fx-color-error, #ff4d4f)",
                            ChartColor::Info => "var(--fx-color-info, #1890ff)",
                            ChartColor::Gray => "var(--fx-color-text-tertiary, #8c8c8c)",
                        };

                        result.push((
                            series_idx,
                            cat_idx,
                            band_start,
                            bar_y,
                            band_width,
                            bar_height,
                            color.to_string(),
                            value,
                            border_radius,
                            s.name.clone(),
                            cat_name.clone(),
                        ));
                    }
                }
            } else {
                // Grouped columns
                let group_width = band_width / series_count as f64;

                for (series_idx, s) in series.iter().enumerate() {
                    if let Some(point) = s.data.iter().find(|p| &p.category == cat_name) {
                        let value = point.value;
                        let bar_end_y = scale_linear((v_min, v_max), (area.bottom(), area.y), value);

                        let (bar_y, bar_h) = if value >= 0.0 {
                            (bar_end_y, zero_y - bar_end_y)
                        } else {
                            (zero_y, bar_end_y - zero_y)
                        };

                        let bar_x = band_start + group_width * series_idx as f64;

                        let color = match s.color {
                            ChartColor::Primary => palette.color_at(series_idx),
                            ChartColor::Success => "var(--fx-color-success, #52c41a)",
                            ChartColor::Warning => "var(--fx-color-warning, #faad14)",
                            ChartColor::Error => "var(--fx-color-error, #ff4d4f)",
                            ChartColor::Info => "var(--fx-color-info, #1890ff)",
                            ChartColor::Gray => "var(--fx-color-text-tertiary, #8c8c8c)",
                        };

                        result.push((
                            series_idx,
                            cat_idx,
                            bar_x,
                            bar_y,
                            group_width * 0.9,
                            bar_h,
                            color.to_string(),
                            value,
                            border_radius,
                            s.name.clone(),
                            cat_name.clone(),
                        ));
                    }
                }
            }
        }

        result
    };

    let category_labels = move || {
        let cats = resolved_categories();
        let cat_count = cats.len();

        cats.into_iter().enumerate().map(move |(i, label)| {
            let (band_start, band_width) = scale_band(
                cat_count,
                (area.x, area.right()),
                i,
                1.0 - config.bar_width as f32,
            );
            let x = band_start + band_width / 2.0;
            (label, x, area.bottom() + 16.0)
        }).collect::<Vec<_>>()
    };

    let y_axis_labels = move || {
        let (v_min, v_max) = value_bounds();
        (0..=4).map(|i| {
            let t = i as f64 / 4.0;
            let v = v_min + t * (v_max - v_min);
            let y = scale_linear((v_min, v_max), (area.bottom(), area.y), v);
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

                // Columns with interactivity
                {
                    let prefix = prefix_column.clone();
                    let on_click = on_column_click.clone();
                    move || columns().into_iter().map({
                        let prefix = prefix.clone();
                        let on_click = on_click.clone();
                        move |(series_idx, cat_idx, x, y, w, h, color, value, radius, series_name, cat_name)| {
                            let is_hovered = move || {
                                hovered_column.get() == Some((series_idx, cat_idx)) ||
                                hovered_legend.get() == Some(series_idx)
                            };
                            let is_dimmed = move || {
                                let col_hover = hovered_column.get();
                                let legend_hover = hovered_legend.get();
                                match (col_hover, legend_hover) {
                                    (Some((s, c)), _) => s != series_idx || c != cat_idx,
                                    (_, Some(s)) => s != series_idx,
                                    _ => false,
                                }
                            };

                            let series_name_clone = series_name.clone();
                            let cat_name_clone = cat_name.clone();
                            let on_click = on_click.clone();

                            view! {
                                <rect
                                    x=x
                                    y=y
                                    width=w
                                    height=h.max(0.0)
                                    fill=color
                                    rx=radius
                                    ry=radius
                                    class=prefix.clone()
                                    style=move || format!(
                                        "cursor: pointer; transition: all 0.2s ease; filter: {}; opacity: {};",
                                        if is_hovered() { "brightness(1.2)" } else { "brightness(1.0)" },
                                        if is_dimmed() { 0.6 } else { 1.0 }
                                    )
                                    on:mouseenter=move |ev| {
                                        hovered_column.set(Some((series_idx, cat_idx)));
                                        tooltip_data.set(Some((
                                            series_name_clone.clone(),
                                            cat_name_clone.clone(),
                                            value,
                                            x + w / 2.0,
                                            y,
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
                                        hovered_column.set(None);
                                        tooltip_data.set(None);
                                    }
                                    on:click=move |_| {
                                        if let Some(ref callback) = on_click {
                                            callback.run((series_idx, cat_idx));
                                        }
                                    }
                                />
                            }
                        }
                    }).collect::<Vec<_>>()
                }

                // Data labels
                {move || {
                    if !config.show_data_labels {
                        return vec![];
                    }
                    columns().into_iter().map(|(_, _, x, y, w, _, _, value, _, _, _)| {
                        view! {
                            <text
                                x=x + w / 2.0
                                y=y - 4.0
                                fill="var(--fx-color-text-secondary, #8c8c8c)"
                                font-size="10"
                                text-anchor="middle"
                                style="pointer-events: none;"
                            >
                                {format!("{:.0}", value)}
                            </text>
                        }
                    }).collect::<Vec<_>>()
                }}

                // Category labels
                {move || category_labels().into_iter().map(|(label, x, y)| {
                    view! {
                        <text
                            x=x
                            y=y
                            fill="var(--fx-color-text-secondary, #8c8c8c)"
                            font-size="11"
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

                // Zero line
                {move || {
                    let (v_min, v_max) = value_bounds();
                    if v_min < 0.0 && v_max > 0.0 {
                        let zero_y = scale_linear((v_min, v_max), (area.bottom(), area.y), 0.0);
                        Some(view! {
                            <line
                                x1=area.x
                                y1=zero_y
                                x2=area.right()
                                y2=zero_y
                                stroke="var(--fx-color-text-tertiary, #595959)"
                                stroke-width="1"
                            />
                        })
                    } else {
                        None
                    }
                }}
            </svg>

            // Tooltip
            {
            let chart_prefix_tooltip = chart_prefix.clone();
            move || {
                tooltip_data.get().map(|(series_name, category, value, _data_x, _data_y)| {
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
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500;">{series_name}</span>
                            </div>
                            <div style="color: var(--fx-color-text-secondary, #8c8c8c);">
                                {format!("{}: {:.1}", category, value)}
                            </div>
                        </div>
                    }
                })
            }}

            // Legend with interactivity
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
                        (i, s.name.clone(), color.to_string())
                    }).collect::<Vec<_>>()
                };

                view! {
                    <div
                        class=format!("{}-legend", chart_prefix_legend)
                        style="display: flex; justify-content: center; gap: 16px; padding: 8px; font-size: 12px;"
                    >
                        {move || legend_items().into_iter().map(|(idx, name, color)| {
                            let is_hovered = move || hovered_legend.get() == Some(idx);
                            view! {
                                <span
                                    style=move || format!(
                                        "display: flex; align-items: center; gap: 6px; color: var(--fx-color-text-secondary, #8c8c8c); cursor: pointer; padding: 2px 6px; border-radius: 4px; transition: all 0.2s ease; {}",
                                        if is_hovered() { "background: var(--fx-color-bg-elevated, #1f1f1f);" } else { "" }
                                    )
                                    on:mouseenter=move |_| {
                                        hovered_legend.set(Some(idx));
                                    }
                                    on:mouseleave=move |_| {
                                        hovered_legend.set(None);
                                    }
                                >
                                    <span style=format!(
                                        "width: 10px; height: 10px; border-radius: 2px; background: {};",
                                        color
                                    ) />
                                    {name}
                                </span>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }
            })
            }

            // Loading overlay
            {loading.then(|| view! {
                <div class=format!("{}-loading-overlay", chart_prefix)>
                    <span class=format!("{}-loading-spinner", chart_prefix)></span>
                </div>
            })}
        </div>
    }
}
