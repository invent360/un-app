//! RadialBarChart Leptos component.

use leptos::prelude::*;
use std::f64::consts::PI;
use super::types::{RadialBarChartConfig, ChartColor, ColorPalette};
use crate::try_use_theme;

/// Radial bar data point.
#[derive(Debug, Clone)]
pub struct RadialBarData {
    /// Label.
    pub label: String,
    /// Value (0-100 or max defined in config).
    pub value: f64,
    /// Color override.
    pub color: Option<ChartColor>,
}

impl RadialBarData {
    /// Create a new radial bar data point.
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Set color.
    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = Some(color);
        self
    }
}

/// RadialBarChart component.
///
/// Interactive circular progress bars for showing multiple metrics with hover effects and tooltips.
///
/// # Props
///
/// - `data` - Radial bar data
/// - `max` - Maximum value (100 by default)
/// - `config` - Chart configuration
/// - `on_bar_click` - Click handler (bar index)
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{RadialBarChart, RadialBarData};
///
/// let data = vec![
///     RadialBarData::new("CPU", 75.0),
///     RadialBarData::new("Memory", 60.0),
///     RadialBarData::new("Disk", 45.0),
/// ];
///
/// view! {
///     <RadialBarChart data=Signal::derive(move || data.clone()) />
/// }
/// ```
#[component]
pub fn RadialBarChart(
    /// Bar data.
    data: Signal<Vec<RadialBarData>>,
    /// Max value (100 by default).
    #[prop(optional)]
    max: Option<f64>,
    /// Chart configuration.
    #[prop(optional)]
    config: Option<RadialBarChartConfig>,
    /// Click handler (bar index).
    #[prop(optional, into)]
    on_bar_click: Option<Callback<usize>>,
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
    let max_value = max.unwrap_or(100.0);

    // Interactive state
    let hovered_bar = RwSignal::new(None::<usize>);
    // Tooltip: (label, value, max_value, percentage, color, x, y)
    let tooltip_data = RwSignal::new(None::<(String, f64, f64, f64, String, f64, f64)>);
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    let chart_prefix = format!("fx-chart-{}", design_system);
    let prefix_svg = format!("{}-svg", chart_prefix);
    let prefix_legend = format!("{}-legend", chart_prefix);
    let prefix_loading_overlay = format!("{}-loading-overlay", chart_prefix);
    let prefix_loading_spinner = format!("{}-loading-spinner", chart_prefix);
    let combined_class = {
        let mut parts = vec![chart_prefix.clone(), format!("{}-radialbar", chart_prefix)];
        if loading {
            parts.push(format!("{}-loading", chart_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let palette = ColorPalette::Default;

    let cx = chart_size as f64 / 2.0;
    let cy = chart_size as f64 / 2.0;
    let max_radius = (chart_size as f64 / 2.0) - 30.0;
    let hollow_ratio = config.hollow_size as f64;
    let min_radius = max_radius * hollow_ratio;

    // Generate arc path
    let arc_path = move |radius: f64, start_angle: f64, end_angle: f64| -> String {
        let start_rad = start_angle * PI / 180.0;
        let end_rad = end_angle * PI / 180.0;

        let x1 = cx + radius * start_rad.cos();
        let y1 = cy + radius * start_rad.sin();
        let x2 = cx + radius * end_rad.cos();
        let y2 = cy + radius * end_rad.sin();

        let large_arc = if (end_angle - start_angle).abs() > 180.0 { 1 } else { 0 };

        format!(
            "M{:.2},{:.2} A{:.2},{:.2} 0 {} 1 {:.2},{:.2}",
            x1, y1, radius, radius, large_arc, x2, y2
        )
    };

    // Calculate bars
    let bars = move || {
        let bars_data = data.get();
        let count = bars_data.len();

        if count == 0 {
            return vec![];
        }

        let ring_width = (max_radius - min_radius) / count as f64;
        let start_angle = config.start_angle as f64;
        let total_angle = config.end_angle as f64 - start_angle;
        let track_opacity = config.track_opacity;

        bars_data.iter().enumerate().map(|(i, bar)| {
            let radius = min_radius + ring_width * (count - 1 - i) as f64 + ring_width / 2.0;
            let stroke_width = ring_width * 0.7;

            // Track (background) path
            let track_path = arc_path(radius, start_angle, start_angle + total_angle);

            // Value path
            let value_angle = (bar.value / max_value).min(1.0) * total_angle;
            let value_path = arc_path(radius, start_angle, start_angle + value_angle);

            // Percentage
            let percentage = (bar.value / max_value) * 100.0;

            // Color
            let color = bar.color
                .map(|c| match c {
                    ChartColor::Primary => palette.color_at(i).to_string(),
                    ChartColor::Success => "var(--fx-color-success, #52c41a)".to_string(),
                    ChartColor::Warning => "var(--fx-color-warning, #faad14)".to_string(),
                    ChartColor::Error => "var(--fx-color-error, #ff4d4f)".to_string(),
                    ChartColor::Info => "var(--fx-color-info, #1890ff)".to_string(),
                    ChartColor::Gray => "var(--fx-color-text-tertiary, #8c8c8c)".to_string(),
                })
                .unwrap_or_else(|| palette.color_at(i).to_string());

            (
                i,                  // index
                bar.label.clone(),  // label
                track_path,         // track path
                value_path,         // value path
                stroke_width,       // stroke width
                color,              // color
                track_opacity,      // track opacity
                bar.value,          // value
                percentage,         // percentage
            )
        }).collect::<Vec<_>>()
    };

    view! {
        <div class=combined_class style="position: relative;" node_ref=container_ref>
            <svg
                width="100%"
                height=chart_size
                viewBox=format!("0 0 {} {}", chart_size, chart_size)
                preserveAspectRatio="xMidYMid meet"
                class=format!("{}-svg", chart_prefix)
            >
                // Bars with interactivity
                {
                    let on_click = on_bar_click.clone();
                    move || bars().into_iter().map({
                        let on_click = on_click.clone();
                        move |(idx, label, track, value, sw, color, track_op, val, pct)| {
                            let is_hovered = move || hovered_bar.get() == Some(idx);
                            let opacity = move || {
                                match hovered_bar.get() {
                                    Some(h) if h != idx => 0.6,
                                    _ => 1.0,
                                }
                            };
                            let glow_filter = move || {
                                if is_hovered() {
                                    "drop-shadow(0 0 4px rgba(255,255,255,0.3))"
                                } else {
                                    "none"
                                }
                            };

                            let label_clone = label.clone();
                            let color_clone = color.clone();
                            let on_click = on_click.clone();
                            // Calculate tooltip position at the end of the value arc
                            let start_angle = config.start_angle as f64;
                            let total_angle = config.end_angle as f64 - start_angle;
                            let value_angle_rad = (start_angle + (val / max_value).min(1.0) * total_angle).to_radians();
                            let radius = min_radius + (sw / 2.0) + (idx as f64 * sw * 1.43);
                            let tip_x = cx + radius * value_angle_rad.cos();
                            let tip_y = cy + radius * value_angle_rad.sin();

                            view! {
                                <g>
                                    // Track
                                    <path
                                        d=track
                                        fill="none"
                                        stroke="var(--fx-color-border, #303030)"
                                        stroke-width=sw
                                        stroke-linecap="round"
                                        opacity=track_op
                                    />
                                    // Value bar with interactivity
                                    <path
                                        d=value
                                        fill="none"
                                        stroke=color.clone()
                                        stroke-width=sw
                                        stroke-linecap="round"
                                        style=move || format!(
                                            "opacity: {}; filter: {}; transition: all 0.2s ease; cursor: pointer;",
                                            opacity(), glow_filter()
                                        )
                                        on:mouseenter=move |ev| {
                                            hovered_bar.set(Some(idx));
                                            tooltip_data.set(Some((label_clone.clone(), val, max_value, pct, color_clone.clone(), tip_x, tip_y)));
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
                                            hovered_bar.set(None);
                                            tooltip_data.set(None);
                                        }
                                        on:click=move |_| {
                                            if let Some(ref callback) = on_click {
                                                callback.run(idx);
                                            }
                                        }
                                    />
                                </g>
                            }
                        }
                    }).collect::<Vec<_>>()
                }

                // Center labels
                {config.show_labels.then(|| {
                    let total = move || {
                        let bars_data = data.get();
                        if bars_data.is_empty() {
                            return 0.0;
                        }
                        let avg = bars_data.iter().map(|b| b.value).sum::<f64>() / bars_data.len() as f64;
                        (avg / max_value) * 100.0
                    };

                    view! {
                        <text
                            x=cx
                            y=cy - 8.0
                            fill="var(--fx-color-text-secondary, #8c8c8c)"
                            font-size="12"
                            text-anchor="middle"
                        >
                            "Average"
                        </text>
                        <text
                            x=cx
                            y=cy + 12.0
                            fill="var(--fx-color-text, #ffffff)"
                            font-size="24"
                            font-weight="600"
                            text-anchor="middle"
                        >
                            {move || format!("{:.0}%", total())}
                        </text>
                    }
                })}
            </svg>

            // Tooltip
            {move || {
                tooltip_data.get().map(|(label, value, max_val, pct, color, _data_x, _data_y)| {
                    // Use actual mouse position for tooltip placement
                    let (mx, my) = mouse_pos.get();
                    let tooltip_x = mx + 15.0;
                    let tooltip_y = my;
                    view! {
                        <div
                            class=format!("{}-tooltip", chart_prefix)
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
                                {format!("Value: {:.1} / {:.0}", value, max_val)}
                            </div>
                            <div style="color: var(--fx-color-text-secondary, #8c8c8c);">
                                {format!("Progress: {:.1}%", pct)}
                            </div>
                        </div>
                    }
                })
            }}

            // Interactive Legend
            {
                let legend_class = prefix_legend.clone();
                move || {
                    let legend_items = bars();
                    if legend_items.is_empty() {
                        return view! {}.into_any();
                    }

                    let legend_class = legend_class.clone();
                    view! {
                        <div
                            class=legend_class
                            style="display: flex; flex-wrap: wrap; justify-content: center; gap: 12px; padding: 8px; font-size: 12px;"
                        >
                            {legend_items.into_iter().map(|(idx, label, _, _, _, color, _, value, pct)| {
                                let is_hovered = move || hovered_bar.get() == Some(idx);
                                view! {
                                    <span
                                        style=move || format!(
                                            "display: flex; align-items: center; gap: 6px; color: var(--fx-color-text-secondary, #8c8c8c); cursor: pointer; padding: 2px 6px; border-radius: 4px; transition: all 0.2s ease; {}",
                                            if is_hovered() { "background: var(--fx-color-bg-elevated, #1f1f1f);" } else { "" }
                                        )
                                        on:mouseenter=move |_| {
                                            hovered_bar.set(Some(idx));
                                        }
                                        on:mouseleave=move |_| {
                                            hovered_bar.set(None);
                                        }
                                    >
                                        <span style=format!(
                                            "width: 10px; height: 10px; border-radius: 2px; background: {};",
                                            color
                                        ) />
                                        {label}
                                        <span style="color: var(--fx-color-text, #ffffff); font-weight: 500;">
                                            {format!("{:.1}%", pct)}
                                        </span>
                                    </span>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }
            }

            {
                let overlay_class = prefix_loading_overlay.clone();
                let spinner_class = prefix_loading_spinner.clone();
                loading.then(move || view! {
                    <div class=overlay_class.clone()>
                        <span class=spinner_class.clone()></span>
                    </div>
                })
            }
        </div>
    }
}
