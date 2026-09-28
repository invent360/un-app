use leptos::prelude::*;

use super::types::{ColorPalette, GaugeChartConfig};
use super::utils::{chart_prefix, try_use_theme};

/// Tooltip data for the gauge chart.
#[derive(Debug, Clone)]
pub struct GaugeTooltipData {
    /// Current value.
    pub value: f64,
    /// Minimum value.
    pub min: f64,
    /// Maximum value.
    pub max: f64,
    /// Percentage of range (0-100).
    pub percentage: f64,
    /// Value color.
    pub color: String,
    /// X position for tooltip.
    pub x: f64,
    /// Y position for tooltip.
    pub y: f64,
}

/// Gauge/speedometer chart component
///
/// Interactive gauge chart with hover effects and optional click callback.
///
/// # Props
///
/// - `value` - Current value to display
/// - `config` - Chart configuration (min, max, angles, etc.)
/// - `colors` - Color palette
/// - `on_value_click` - Optional callback triggered when gauge arc is clicked
/// - `loading` - Loading state
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::GaugeChart;
///
/// view! {
///     <GaugeChart
///         value=Signal::derive(move || 75.0)
///         on_value_click=Callback::new(|v| log::info!("Clicked value: {}", v))
///     />
/// }
/// ```
#[component]
pub fn GaugeChart(
    /// The current value to display on the gauge
    #[prop(into)]
    value: Signal<f64>,
    /// Configuration for the gauge chart
    #[prop(optional)]
    config: Option<GaugeChartConfig>,
    /// Optional color palette
    #[prop(optional, into)]
    colors: Option<Signal<ColorPalette>>,
    /// Callback triggered when the gauge arc is clicked
    #[prop(optional, into)]
    on_value_click: Option<Callback<f64>>,
    /// Whether the chart is in a loading state
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme = try_use_theme();
    let chart_prefix_str = chart_prefix("gauge");
    let prefix_class = chart_prefix_str.clone();
    let prefix_svg = format!("{}-svg", chart_prefix_str);
    let prefix_value_arc = format!("{}-value-arc", chart_prefix_str);
    let prefix_value_text = format!("{}-value-text", chart_prefix_str);
    let prefix_loading = format!("{}-loading", chart_prefix_str);
    let prefix_spinner = format!("{}-spinner", chart_prefix_str);
    let prefix_tooltip = format!("{}-tooltip", chart_prefix_str);

    let config = config.unwrap_or_default();

    let default_colors = ColorPalette::default();
    let colors = colors.unwrap_or_else(|| Signal::stored(default_colors));

    // Interactive state signals
    let is_hovered = RwSignal::new(false);
    let tooltip_data = RwSignal::new(None::<GaugeTooltipData>);
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    // Store config values for closures
    let cfg_min = config.min;
    let cfg_max = config.max;
    let cfg_start_angle = config.start_angle as f64;
    let cfg_end_angle = config.end_angle as f64;
    let cfg_show_value = config.show_value;

    // Calculate gauge geometry
    let gauge_geometry = Memo::new(move |_| {
        let val = value.get();
        let palette = colors.get();

        let center_x = 150.0;
        let center_y = 150.0;
        let radius = 100.0;
        let stroke_width = 20.0;

        // Normalize value to 0-1 range
        let normalized = ((val - cfg_min) / (cfg_max - cfg_min)).clamp(0.0, 1.0);
        let percentage = normalized * 100.0;

        // Calculate angles
        let start_angle = cfg_start_angle;
        let end_angle = cfg_end_angle;
        let angle_range = end_angle - start_angle;
        let current_angle = start_angle + (normalized * angle_range);

        // Convert to radians
        let start_rad = start_angle.to_radians();
        let end_rad = end_angle.to_radians();
        let current_rad = current_angle.to_radians();

        // Calculate arc paths
        let start_x = center_x + radius * start_rad.cos();
        let start_y = center_y + radius * start_rad.sin();
        let end_x = center_x + radius * end_rad.cos();
        let end_y = center_y + radius * end_rad.sin();
        let current_x = center_x + radius * current_rad.cos();
        let current_y = center_y + radius * current_rad.sin();

        let large_arc_bg = if angle_range.abs() > 180.0 { 1 } else { 0 };
        let large_arc_value = if (current_angle - start_angle).abs() > 180.0 { 1 } else { 0 };

        // Background arc path
        let bg_path = format!(
            "M {} {} A {} {} 0 {} 1 {} {}",
            start_x, start_y, radius, radius, large_arc_bg, end_x, end_y
        );

        // Value arc path
        let value_path = format!(
            "M {} {} A {} {} 0 {} 1 {} {}",
            start_x, start_y, radius, radius, large_arc_value, current_x, current_y
        );

        // Get color from palette
        let value_color = palette.color_at(0).to_string();

        (bg_path, value_path, value_color, center_x, center_y, stroke_width, val, cfg_show_value, percentage, cfg_min, cfg_max)
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
            <svg viewBox="0 0 300 200" class=prefix_svg>
                // Background track
                <path
                    d=move || gauge_geometry.get().0.clone()
                    fill="none"
                    stroke=move || {
                        if theme.is_some() { "var(--chart-grid)" } else { "#e5e7eb" }
                    }
                    stroke-width=move || gauge_geometry.get().5
                    stroke-linecap="round"
                />

                // Value arc with interactivity
                {
                    let on_click = on_value_click.clone();
                    let value_arc_class = prefix_value_arc.clone();
                    move || {
                        let geom = gauge_geometry.get();
                        let value_path = geom.1.clone();
                        let value_color = geom.2.clone();
                        let stroke_width = geom.5;
                        let current_value = geom.6;
                        let percentage = geom.8;
                        let min_val = geom.9;
                        let max_val = geom.10;
                        let color_for_tooltip = value_color.clone();
                        let on_click = on_click.clone();

                        // Compute hover-dependent styles
                        let arc_opacity = move || {
                            if is_hovered.get() { 1.0 } else { 0.9 }
                        };
                        let arc_filter = move || {
                            if is_hovered.get() {
                                "drop-shadow(0 0 8px rgba(255,255,255,0.4))"
                            } else {
                                "none"
                            }
                        };
                        let arc_transform = move || {
                            if is_hovered.get() {
                                "scale(1.02)"
                            } else {
                                "scale(1)"
                            }
                        };

                        view! {
                            <path
                                d=value_path
                                fill="none"
                                stroke=value_color
                                stroke-width=stroke_width
                                stroke-linecap="round"
                                class=value_arc_class.clone()
                                style=move || format!(
                                    "transform-origin: 150px 150px; transform: {}; opacity: {}; filter: {}; transition: all 0.2s ease; cursor: pointer;",
                                    arc_transform(), arc_opacity(), arc_filter()
                                )
                                on:mouseenter=move |ev| {
                                    is_hovered.set(true);
                                    tooltip_data.set(Some(GaugeTooltipData {
                                        value: current_value,
                                        min: min_val,
                                        max: max_val,
                                        percentage,
                                        color: color_for_tooltip.clone(),
                                        x: 150.0,
                                        y: 100.0,
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
                                    is_hovered.set(false);
                                    tooltip_data.set(None);
                                }
                                on:click=move |_| {
                                    if let Some(ref callback) = on_click {
                                        callback.run(current_value);
                                    }
                                }
                            />
                        }
                    }
                }

                // Center text (value)
                {
                    let text_class = prefix_value_text.clone();
                    move || {
                        let geom = gauge_geometry.get();
                        if geom.7 {
                            let value_text = format!("{:.1}", geom.6);
                            // Text style changes based on hover
                            let text_opacity = move || {
                                if is_hovered.get() { 1.0 } else { 0.85 }
                            };
                            Some(view! {
                                <text
                                    x=geom.3
                                    y=geom.4
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    class=text_class.clone()
                                    fill=move || {
                                        if theme.is_some() { "var(--chart-text)" } else { "#1f2937" }
                                    }
                                    font-size="24"
                                    font-weight="bold"
                                    style=move || format!(
                                        "opacity: {}; transition: opacity 0.2s ease;",
                                        text_opacity()
                                    )
                                >
                                    {value_text}
                                </text>
                            })
                        } else {
                            None
                        }
                    }
                }
            </svg>

            // Tooltip
            {
                let tooltip_class = prefix_tooltip.clone();
                move || {
                    tooltip_data.get().map(|data| {
                        // Use actual mouse position for tooltip placement
                        let (mx, my) = mouse_pos.get();
                        let tooltip_x = mx + 15.0;
                        let tooltip_y = my;
                        view! {
                            <div
                                class=tooltip_class.clone()
                                style=format!(
                                    "position: absolute; left: {}px; top: {}px; background: var(--fx-color-bg-elevated, #1f1f1f); border: 1px solid var(--fx-color-border, #303030); border-radius: 6px; padding: 8px 12px; font-size: 12px; pointer-events: none; z-index: 10; box-shadow: 0 2px 8px rgba(0,0,0,0.3); min-width: 140px; transform: translateY(-50%);",
                                    tooltip_x, tooltip_y
                                )
                            >
                                <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 6px;">
                                    <span style=format!("width: 10px; height: 10px; border-radius: 2px; background: {};", data.color) />
                                    <span style="color: var(--fx-color-text, #fff); font-weight: 500;">
                                        "Value"
                                    </span>
                                </div>
                                <div style="color: var(--fx-color-text-secondary, #8c8c8c); margin-bottom: 4px;">
                                    {format!("Current: {:.2}", data.value)}
                                </div>
                                <div style="color: var(--fx-color-text-secondary, #8c8c8c); margin-bottom: 4px;">
                                    {format!("Range: {:.0} - {:.0}", data.min, data.max)}
                                </div>
                                <div style="color: var(--fx-color-text, #fff); font-weight: 500;">
                                    {format!("{:.1}% of range", data.percentage)}
                                </div>
                            </div>
                        }
                    })
                }
            }

            // Loading overlay
            {
                let loading_class = prefix_loading.clone();
                let spinner_class = prefix_spinner.clone();
                move || {
                    if loading {
                        Some(view! {
                            <div class=loading_class.clone()>
                                <div class=spinner_class.clone()></div>
                            </div>
                        })
                    } else {
                        None
                    }
                }
            }
        </div>
    }
}
