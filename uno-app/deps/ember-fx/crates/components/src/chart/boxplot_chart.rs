use leptos::prelude::*;

use super::types::{BoxPlotChartConfig, BoxPlotPoint, ColorPalette};
use super::utils::{chart_prefix, try_use_theme};

/// Box plot tooltip data containing all statistical values.
#[derive(Debug, Clone, PartialEq)]
pub struct BoxPlotTooltipData {
    /// Category/label for this box plot.
    pub label: String,
    /// Minimum value.
    pub min: f64,
    /// First quartile (Q1, 25th percentile).
    pub q1: f64,
    /// Median (Q2, 50th percentile).
    pub median: f64,
    /// Third quartile (Q3, 75th percentile).
    pub q3: f64,
    /// Maximum value.
    pub max: f64,
    /// Interquartile range (Q3 - Q1).
    pub iqr: f64,
    /// Box color.
    pub color: String,
    /// X position for tooltip.
    pub x: f64,
    /// Y position for tooltip.
    pub y: f64,
}

impl BoxPlotTooltipData {
    /// Create tooltip data from a BoxPlotPoint.
    pub fn from_point(point: &BoxPlotPoint, color: String, x: f64, y: f64) -> Self {
        Self {
            label: point.category.clone(),
            min: point.min,
            q1: point.q1,
            median: point.median,
            q3: point.q3,
            max: point.max,
            iqr: point.q3 - point.q1,
            color,
            x,
            y,
        }
    }
}

/// Statistical box plot chart component with interactive hover and click support.
///
/// # Props
///
/// - `data` - Box plot data points
/// - `config` - Chart configuration
/// - `colors` - Optional color palette
/// - `on_box_click` - Click handler (box index)
/// - `loading` - Loading state
/// - `class` - Additional CSS classes
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{BoxPlotChart, BoxPlotPoint};
///
/// let data = vec![
///     BoxPlotPoint::new("Q1", 10.0, 25.0, 50.0, 75.0, 90.0),
///     BoxPlotPoint::new("Q2", 15.0, 30.0, 55.0, 70.0, 85.0),
/// ];
///
/// view! {
///     <BoxPlotChart
///         data=Signal::derive(move || data.clone())
///         on_box_click=move |idx| log::info!("Clicked box {}", idx)
///     />
/// }
/// ```
#[component]
pub fn BoxPlotChart(
    /// Box plot data points
    #[prop(into)]
    data: Signal<Vec<BoxPlotPoint>>,
    /// Configuration for the box plot chart
    #[prop(optional)]
    config: Option<BoxPlotChartConfig>,
    /// Optional color palette
    #[prop(optional, into)]
    colors: Option<Signal<ColorPalette>>,
    /// Click handler for box plots (receives box index)
    #[prop(optional, into)]
    on_box_click: Option<Callback<usize>>,
    /// Whether the chart is in a loading state
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme = try_use_theme();
    let prefix = chart_prefix("boxplot");

    let config = config.unwrap_or_default();

    // Interactive state signals
    let hovered_box = RwSignal::new(None::<usize>);
    let tooltip_data = RwSignal::new(None::<BoxPlotTooltipData>);
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    // Pre-clone prefix for use in closures
    let prefix_class = prefix.clone();
    let prefix_svg = prefix.clone();
    let prefix_box = prefix.clone();
    let prefix_loading = prefix.clone();
    let prefix_spinner = prefix.clone();
    let prefix_tooltip = prefix.clone();

    let default_colors = ColorPalette::default();
    let colors = colors.unwrap_or_else(|| Signal::stored(default_colors));

    // Chart dimensions
    let width = 600.0;
    let height = 400.0;
    let padding_left = 60.0;
    let padding_right = 40.0;
    let padding_top = 40.0;
    let padding_bottom = 60.0;
    let chart_width = width - padding_left - padding_right;
    let chart_height = height - padding_top - padding_bottom;

    // Store config value for closures
    let show_outliers = config.show_outliers;

    // Calculate box plot layout - extended to include raw point data for tooltips
    let boxplot_data = Memo::new(move |_| {
        let points = data.get();
        let palette = colors.get();

        if points.is_empty() {
            return (Vec::new(), 0.0, 0.0, Vec::new(), Vec::new());
        }

        // Find overall min/max
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;
        for point in &points {
            min_val = min_val.min(point.min);
            max_val = max_val.max(point.max);
            for &o in &point.outliers {
                min_val = min_val.min(o);
                max_val = max_val.max(o);
            }
        }

        // Add padding to range
        let range = max_val - min_val;
        let value_padding = range * 0.1;
        min_val -= value_padding;
        max_val += value_padding;

        let box_width = (chart_width / points.len() as f64) * 0.6;
        let gap = (chart_width / points.len() as f64) * 0.4;

        // Extended tuple: (x, y_min, y_max, y_q1, y_q3, y_median, box_width, label, color, outlier_ys, raw_point)
        let boxes: Vec<(f64, f64, f64, f64, f64, f64, f64, String, String, Vec<f64>, BoxPlotPoint)> = points
            .iter()
            .enumerate()
            .map(|(i, point)| {
                let x = padding_left + (i as f64 * (box_width + gap)) + gap / 2.0;

                let y_min = padding_top + chart_height * (1.0 - (point.min - min_val) / (max_val - min_val));
                let y_max = padding_top + chart_height * (1.0 - (point.max - min_val) / (max_val - min_val));
                let y_q1 = padding_top + chart_height * (1.0 - (point.q1 - min_val) / (max_val - min_val));
                let y_q3 = padding_top + chart_height * (1.0 - (point.q3 - min_val) / (max_val - min_val));
                let y_median = padding_top + chart_height * (1.0 - (point.median - min_val) / (max_val - min_val));

                let color = palette.color_at(i).to_string();

                let outlier_ys: Vec<f64> = point.outliers.iter().map(|&o| {
                    padding_top + chart_height * (1.0 - (o - min_val) / (max_val - min_val))
                }).collect();

                (x, y_min, y_max, y_q1, y_q3, y_median, box_width, point.category.clone(), color, outlier_ys, point.clone())
            })
            .collect();

        // Y-axis labels
        let num_ticks = 5;
        let y_labels: Vec<(f64, String)> = (0..=num_ticks)
            .map(|i| {
                let value = min_val + (max_val - min_val) * (i as f64 / num_ticks as f64);
                let y = padding_top + chart_height * (1.0 - i as f64 / num_ticks as f64);
                (y, format!("{:.1}", value))
            })
            .collect();

        // Collect raw points for tooltip access
        let raw_points: Vec<(String, BoxPlotPoint)> = points
            .iter()
            .enumerate()
            .map(|(i, p)| (palette.color_at(i).to_string(), p.clone()))
            .collect();

        (boxes, min_val, max_val, y_labels, raw_points)
    });

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
                // Y-axis
                <line
                    x1=padding_left
                    y1=padding_top
                    x2=padding_left
                    y2=height - padding_bottom
                    stroke=move || if theme.is_some() { "var(--chart-axis)" } else { "#9ca3af" }
                    stroke-width="1"
                />

                // X-axis
                <line
                    x1=padding_left
                    y1=height - padding_bottom
                    x2=width - padding_right
                    y2=height - padding_bottom
                    stroke=move || if theme.is_some() { "var(--chart-axis)" } else { "#9ca3af" }
                    stroke-width="1"
                />

                // Grid lines and Y-axis labels
                {move || {
                    let (_, _, _, y_labels, _) = boxplot_data.get();
                    y_labels.into_iter().map(|(y, label)| {
                        view! {
                            <g>
                                <line
                                    x1=padding_left
                                    y1=y
                                    x2=width - padding_right
                                    y2=y
                                    stroke=move || if theme.is_some() { "var(--chart-grid)" } else { "#e5e7eb" }
                                    stroke-width="1"
                                    stroke-dasharray="4,4"
                                />
                                <text
                                    x=padding_left - 10.0
                                    y=y
                                    text-anchor="end"
                                    dominant-baseline="middle"
                                    fill=move || if theme.is_some() { "var(--chart-text)" } else { "#6b7280" }
                                    font-size="12"
                                >
                                    {label}
                                </text>
                            </g>
                        }
                    }).collect_view()
                }}

                // Box plots with interactivity
                {move || {
                    let (boxes, _, _, _, _) = boxplot_data.get();
                    let prefix_box = prefix_box.clone();
                    let on_click = on_box_click.clone();

                    boxes.into_iter().enumerate().map({
                        let prefix_box = prefix_box.clone();
                        let on_click = on_click.clone();
                        move |(idx, (x, y_min, y_max, y_q1, y_q3, y_median, box_w, label, color, outlier_ys, raw_point))| {
                            let center_x = x + box_w / 2.0;
                            let whisker_width = box_w * 0.5;

                            // Hover state computations
                            let is_hovered = move || hovered_box.get() == Some(idx);
                            let opacity = move || {
                                match hovered_box.get() {
                                    Some(h) if h != idx => 0.4,
                                    _ => 1.0,
                                }
                            };
                            let fill_opacity = move || {
                                if is_hovered() { 0.5 } else { 0.3 }
                            };
                            let stroke_width = move || {
                                if is_hovered() { 2.5 } else { 1.5 }
                            };

                            // Clone values for event handlers
                            let color_for_tooltip = color.clone();
                            let point_for_tooltip = raw_point.clone();
                            let on_click = on_click.clone();

                            view! {
                                <g
                                    class=format!("{}-box", prefix_box)
                                    style=move || format!(
                                        "opacity: {}; cursor: pointer; transition: opacity 0.2s ease;",
                                        opacity()
                                    )
                                    on:mouseenter={
                                        let color = color_for_tooltip.clone();
                                        let point = point_for_tooltip.clone();
                                        move |ev| {
                                            hovered_box.set(Some(idx));
                                            tooltip_data.set(Some(BoxPlotTooltipData::from_point(&point, color.clone(), center_x, y_median)));
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
                                        hovered_box.set(None);
                                        tooltip_data.set(None);
                                    }
                                    on:click={
                                        let on_click = on_click.clone();
                                        move |_| {
                                            if let Some(ref callback) = on_click {
                                                callback.run(idx);
                                            }
                                        }
                                    }
                                >
                                    // Invisible hit area for easier hovering
                                    <rect
                                        x=x - 5.0
                                        y=y_max - 5.0
                                        width=box_w + 10.0
                                        height=(y_min - y_max) + 10.0
                                        fill="transparent"
                                    />

                                    // Upper whisker (max to q3)
                                    <line
                                        x1=center_x
                                        y1=y_max
                                        x2=center_x
                                        y2=y_q3
                                        stroke=color.clone()
                                        stroke-width="1"
                                        style="transition: stroke-width 0.2s ease;"
                                    />
                                    // Upper whisker cap
                                    <line
                                        x1=center_x - whisker_width / 2.0
                                        y1=y_max
                                        x2=center_x + whisker_width / 2.0
                                        y2=y_max
                                        stroke=color.clone()
                                        stroke-width="1"
                                        style="transition: stroke-width 0.2s ease;"
                                    />

                                    // Lower whisker (q1 to min)
                                    <line
                                        x1=center_x
                                        y1=y_q1
                                        x2=center_x
                                        y2=y_min
                                        stroke=color.clone()
                                        stroke-width="1"
                                        style="transition: stroke-width 0.2s ease;"
                                    />
                                    // Lower whisker cap
                                    <line
                                        x1=center_x - whisker_width / 2.0
                                        y1=y_min
                                        x2=center_x + whisker_width / 2.0
                                        y2=y_min
                                        stroke=color.clone()
                                        stroke-width="1"
                                        style="transition: stroke-width 0.2s ease;"
                                    />

                                    // Box (q1 to q3) with hover effect
                                    <rect
                                        x=x
                                        y=y_q3
                                        width=box_w
                                        height=y_q1 - y_q3
                                        fill=color.clone()
                                        fill-opacity=fill_opacity
                                        stroke=color.clone()
                                        stroke-width=stroke_width
                                        style="transition: fill-opacity 0.2s ease, stroke-width 0.2s ease;"
                                    />

                                    // Highlight border on hover
                                    {move || is_hovered().then(|| {
                                        view! {
                                            <rect
                                                x=x - 2.0
                                                y=y_q3 - 2.0
                                                width=box_w + 4.0
                                                height=(y_q1 - y_q3) + 4.0
                                                fill="none"
                                                stroke="var(--fx-color-primary, #1890ff)"
                                                stroke-width="1"
                                                stroke-opacity="0.6"
                                                rx="2"
                                                ry="2"
                                            />
                                        }
                                    })}

                                    // Median line
                                    <line
                                        x1=x
                                        y1=y_median
                                        x2=x + box_w
                                        y2=y_median
                                        stroke=color.clone()
                                        stroke-width="2"
                                        style="transition: stroke-width 0.2s ease;"
                                    />

                                    // Outliers
                                    {if show_outliers {
                                        Some(outlier_ys.into_iter().map(|oy| {
                                            view! {
                                                <circle
                                                    cx=center_x
                                                    cy=oy
                                                    r="4"
                                                    fill="none"
                                                    stroke=color.clone()
                                                    stroke-width="1.5"
                                                    style="transition: r 0.2s ease;"
                                                />
                                            }
                                        }).collect_view())
                                    } else {
                                        None
                                    }}

                                    // X-axis label
                                    <text
                                        x=center_x
                                        y=height - padding_bottom + 20.0
                                        text-anchor="middle"
                                        fill=move || if theme.is_some() { "var(--chart-text)" } else { "#6b7280" }
                                        font-size="12"
                                        style="pointer-events: none;"
                                    >
                                        {label}
                                    </text>
                                </g>
                            }
                        }
                    }).collect_view()
                }}
            </svg>

            // Tooltip showing all statistical values
            {move || {
                tooltip_data.get().map(|data| {
                    // Use actual mouse position for tooltip placement
                    let (mx, my) = mouse_pos.get();
                    let tooltip_x = mx + 15.0;
                    let tooltip_y = my;
                    view! {
                        <div
                            class=format!("{}-tooltip", prefix_tooltip)
                            style=format!(
                                "position: absolute; left: {}px; top: {}px; background: var(--fx-color-bg-elevated, #1f1f1f); border: 1px solid var(--fx-color-border, #303030); border-radius: 6px; padding: 12px 16px; font-size: 12px; pointer-events: none; z-index: 10; box-shadow: 0 4px 12px rgba(0,0,0,0.3); min-width: 160px; transform: translateY(-50%);",
                                tooltip_x, tooltip_y
                            )
                        >
                            // Header with color indicator and label
                            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 8px; padding-bottom: 8px; border-bottom: 1px solid var(--fx-color-border, #303030);">
                                <span style=format!(
                                    "width: 12px; height: 12px; border-radius: 2px; background: {};",
                                    data.color
                                ) />
                                <span style="color: var(--fx-color-text, #fff); font-weight: 600; font-size: 13px;">
                                    {data.label.clone()}
                                </span>
                            </div>

                            // Statistical values
                            <div style="display: grid; grid-template-columns: auto auto; gap: 4px 16px;">
                                <span style="color: var(--fx-color-text-secondary, #8c8c8c);">"Max"</span>
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500; text-align: right;">
                                    {format!("{:.2}", data.max)}
                                </span>

                                <span style="color: var(--fx-color-text-secondary, #8c8c8c);">"Q3"</span>
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500; text-align: right;">
                                    {format!("{:.2}", data.q3)}
                                </span>

                                <span style="color: var(--fx-color-text-secondary, #8c8c8c);">"Median"</span>
                                <span style="color: var(--fx-color-primary, #1890ff); font-weight: 600; text-align: right;">
                                    {format!("{:.2}", data.median)}
                                </span>

                                <span style="color: var(--fx-color-text-secondary, #8c8c8c);">"Q1"</span>
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500; text-align: right;">
                                    {format!("{:.2}", data.q1)}
                                </span>

                                <span style="color: var(--fx-color-text-secondary, #8c8c8c);">"Min"</span>
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500; text-align: right;">
                                    {format!("{:.2}", data.min)}
                                </span>
                            </div>

                            // IQR in separate section
                            <div style="margin-top: 8px; padding-top: 8px; border-top: 1px solid var(--fx-color-border, #303030); display: flex; justify-content: space-between;">
                                <span style="color: var(--fx-color-text-tertiary, #595959);">"IQR (Q3-Q1)"</span>
                                <span style="color: var(--fx-color-success, #52c41a); font-weight: 500;">
                                    {format!("{:.2}", data.iqr)}
                                </span>
                            </div>
                        </div>
                    }
                })
            }}

            // Loading overlay
            {move || {
                if loading {
                    Some(view! {
                        <div class=format!("{}-loading", prefix_loading)>
                            <div class=format!("{}-spinner", prefix_spinner)></div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
