use leptos::prelude::*;

use super::types::{ColorPalette, PolarAreaChartConfig, PolarAreaSegment};
use super::utils::{chart_prefix, try_use_theme};

/// Polar area chart component
///
/// An interactive polar area chart where each segment has equal angles but varying radii
/// based on data values. Features hover effects, tooltips, and click callbacks.
///
/// # Props
///
/// - `data` - Polar area segment data
/// - `config` - Chart configuration
/// - `colors` - Optional color palette
/// - `on_segment_click` - Click handler (segment index)
/// - `loading` - Loading state
/// - `class` - Additional CSS classes
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{PolarAreaChart, PolarAreaSegment};
///
/// let data = vec![
///     PolarAreaSegment::new("Category A", 75.0),
///     PolarAreaSegment::new("Category B", 50.0),
///     PolarAreaSegment::new("Category C", 100.0),
/// ];
///
/// view! {
///     <PolarAreaChart
///         data=Signal::derive(move || data.clone())
///         on_segment_click=Callback::new(|idx| log::info!("Clicked segment {}", idx))
///     />
/// }
/// ```
#[component]
pub fn PolarAreaChart(
    /// Polar area segment data
    #[prop(into)]
    data: Signal<Vec<PolarAreaSegment>>,
    /// Configuration for the polar area chart
    #[prop(optional)]
    config: Option<PolarAreaChartConfig>,
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
    let prefix = chart_prefix("polararea");

    let config = config.unwrap_or_default();

    // Pre-clone prefixes for use in closures
    let prefix_svg = format!("{}-svg", prefix);
    let prefix_sector = format!("{}-sector", prefix);
    let prefix_path = format!("{}-path", prefix);
    let prefix_legend = format!("{}-legend", prefix);
    let prefix_legend_item = format!("{}-legend-item", prefix);
    let prefix_legend_color = format!("{}-legend-color", prefix);
    let prefix_legend_label = format!("{}-legend-label", prefix);
    let prefix_loading = format!("{}-loading", prefix);
    let prefix_spinner = format!("{}-spinner", prefix);
    let prefix_tooltip = format!("{}-tooltip", prefix);

    let default_colors = ColorPalette::default();
    let colors = colors.unwrap_or_else(|| Signal::stored(default_colors));

    // Interactive state
    let hovered_segment = RwSignal::new(None::<usize>);
    let tooltip_data = RwSignal::new(None::<(String, f64, f64, String, f64, f64)>); // (label, value, percentage, color, x, y)
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    // Chart dimensions
    let size = 400.0;
    let center = size / 2.0;
    let max_radius = 150.0;

    // Store config values for closures
    let show_labels = config.show_labels;
    let show_legend = config.show_legend;
    let start_angle_offset = (config.start_angle as f64).to_radians();

    // Calculate polar area geometry
    let polar_data = Memo::new(move |_| {
        let segments = data.get();
        let palette = colors.get();

        if segments.is_empty() {
            return (Vec::new(), Vec::new(), 0.0_f64);
        }

        let max_value = segments.iter().map(|s| s.value).fold(0.0_f64, f64::max);
        let total_value: f64 = segments.iter().map(|s| s.value).sum();
        let angle_per_segment = 2.0 * std::f64::consts::PI / segments.len() as f64;

        let sector_data: Vec<(usize, String, String, f64, f64, String, f64, f64)> = segments
            .iter()
            .enumerate()
            .map(|(i, segment)| {
                let start_angle = start_angle_offset + (i as f64 * angle_per_segment);
                let end_angle = start_angle + angle_per_segment;
                let radius = max_radius * (segment.value / max_value);

                // Create sector path
                let x1 = center + radius * start_angle.cos();
                let y1 = center + radius * start_angle.sin();
                let x2 = center + radius * end_angle.cos();
                let y2 = center + radius * end_angle.sin();

                let large_arc = if angle_per_segment > std::f64::consts::PI { 1 } else { 0 };

                let path = format!(
                    "M {} {} L {} {} A {} {} 0 {} 1 {} {} Z",
                    center, center,
                    x1, y1,
                    radius, radius,
                    large_arc,
                    x2, y2
                );

                let color = segment.color.clone().unwrap_or_else(|| {
                    palette.color_at(i).to_string()
                });

                // Label position (middle of segment)
                let mid_angle = (start_angle + end_angle) / 2.0;
                let label_radius = radius * 0.6;
                let label_x = center + label_radius * mid_angle.cos();
                let label_y = center + label_radius * mid_angle.sin();

                // Calculate percentage
                let percentage = if total_value > 0.0 {
                    (segment.value / total_value) * 100.0
                } else {
                    0.0
                };

                (i, segment.label.clone(), path, label_x, label_y, color, segment.value, percentage)
            })
            .collect();

        // Grid circles (default 4 rings)
        let num_rings = 4;
        let rings: Vec<f64> = (1..=num_rings)
            .map(|i| max_radius * (i as f64 / num_rings as f64))
            .collect();

        (sector_data, rings, total_value)
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
            <svg viewBox=format!("0 0 {} {}", size, size) class=prefix_svg.clone()>
                // Grid circles
                {move || {
                    let (_, rings, _) = polar_data.get();
                    rings.into_iter().map(|radius| {
                        view! {
                            <circle
                                cx=center
                                cy=center
                                r=radius
                                fill="none"
                                stroke=move || if theme.is_some() { "var(--chart-grid)" } else { "#e5e7eb" }
                                stroke-width="1"
                                stroke-dasharray="4,4"
                            />
                        }
                    }).collect_view()
                }}

                // Sectors with interactivity
                {
                    let prefix_sector = prefix_sector.clone();
                    let prefix_path = prefix_path.clone();
                    let on_click = on_segment_click.clone();
                    move || {
                        let (sector_data, _, _) = polar_data.get();
                        let prefix_sector = prefix_sector.clone();
                        let prefix_path = prefix_path.clone();
                        let on_click = on_click.clone();

                        sector_data.into_iter().map({
                            let prefix_sector = prefix_sector.clone();
                            let prefix_path = prefix_path.clone();
                            let on_click = on_click.clone();
                            move |(idx, label, path, label_x, label_y, color, value, percentage)| {
                                let is_hovered = move || hovered_segment.get() == Some(idx);

                                // Scale outward when hovered
                                let transform = move || {
                                    if is_hovered() {
                                        format!("scale(1.05) translate({}, {})",
                                            -center * 0.025, -center * 0.025)
                                    } else {
                                        String::new()
                                    }
                                };

                                // Dim non-hovered segments
                                let opacity = move || {
                                    match hovered_segment.get() {
                                        Some(h) if h != idx => 0.5,
                                        _ => 0.8,
                                    }
                                };

                                let label_clone = label.clone();
                                let color_clone = color.clone();
                                let on_click = on_click.clone();
                                let tooltip_x = label_x;
                                let tooltip_y = label_y;

                                view! {
                                    <g class=prefix_sector.clone()>
                                        <path
                                            d=path
                                            fill=color.clone()
                                            fill-opacity=move || opacity()
                                            stroke=move || if theme.is_some() { "var(--chart-background)" } else { "#ffffff" }
                                            stroke-width="2"
                                            class=prefix_path.clone()
                                            style=move || format!(
                                                "transform-origin: {}px {}px; transform: {}; transition: all 0.2s ease; cursor: pointer;",
                                                center, center, transform()
                                            )
                                            on:mouseenter=move |ev| {
                                                hovered_segment.set(Some(idx));
                                                tooltip_data.set(Some((label_clone.clone(), value, percentage, color_clone.clone(), tooltip_x, tooltip_y)));
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
                                        />

                                        {if show_labels {
                                            let is_hovered_label = move || hovered_segment.get() == Some(idx);
                                            Some(view! {
                                                <text
                                                    x=label_x
                                                    y=label_y
                                                    text-anchor="middle"
                                                    dominant-baseline="middle"
                                                    fill="white"
                                                    font-size="11"
                                                    font-weight="600"
                                                    style=move || format!(
                                                        "opacity: {}; transition: opacity 0.2s ease; pointer-events: none;",
                                                        if is_hovered_label() { 1.0 } else { 0.9 }
                                                    )
                                                >
                                                    {label}
                                                </text>
                                            })
                                        } else {
                                            None
                                        }}
                                    </g>
                                }
                            }
                        }).collect_view()
                    }
                }
            </svg>

            // Tooltip
            {
                let prefix_tooltip = prefix_tooltip.clone();
                move || {
                    tooltip_data.get().map(|(label, value, percentage, color, _data_x, _data_y)| {
                        // Use actual mouse position for tooltip placement
                        let (mx, my) = mouse_pos.get();
                        let tooltip_x = mx + 15.0;
                        let tooltip_y = my;
                        view! {
                            <div
                                class=prefix_tooltip.clone()
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
                                    {format!("Value: {:.1}", value)}
                                </div>
                                <div style="color: var(--fx-color-text-secondary, #8c8c8c);">
                                    {format!("Percentage: {:.1}%", percentage)}
                                </div>
                            </div>
                        }
                    })
                }
            }

            // Legend with interactive highlighting
            {
                let prefix_legend = prefix_legend.clone();
                let prefix_legend_item = prefix_legend_item.clone();
                let prefix_legend_color = prefix_legend_color.clone();
                let prefix_legend_label = prefix_legend_label.clone();
                move || {
                    if show_legend {
                        let segments = data.get();
                        let palette = colors.get();
                        let prefix_legend = prefix_legend.clone();
                        let prefix_legend_item = prefix_legend_item.clone();
                        let prefix_legend_color = prefix_legend_color.clone();
                        let prefix_legend_label = prefix_legend_label.clone();

                        Some(view! {
                            <div
                                class=prefix_legend
                                style="display: flex; flex-wrap: wrap; justify-content: center; gap: 12px; padding: 8px; font-size: 12px;"
                            >
                                {segments.into_iter().enumerate().map({
                                    let prefix_legend_item = prefix_legend_item.clone();
                                    let prefix_legend_color = prefix_legend_color.clone();
                                    let prefix_legend_label = prefix_legend_label.clone();
                                    move |(idx, segment)| {
                                        let color = segment.color.clone().unwrap_or_else(|| {
                                            palette.color_at(idx).to_string()
                                        });
                                        let is_hovered = move || hovered_segment.get() == Some(idx);

                                        view! {
                                            <div
                                                class=prefix_legend_item.clone()
                                                style=move || format!(
                                                    "display: flex; align-items: center; gap: 6px; cursor: pointer; padding: 4px 8px; border-radius: 4px; transition: all 0.2s ease; {}",
                                                    if is_hovered() { "background: var(--fx-color-bg-elevated, #1f1f1f);" } else { "" }
                                                )
                                                on:mouseenter=move |_| {
                                                    hovered_segment.set(Some(idx));
                                                }
                                                on:mouseleave=move |_| {
                                                    hovered_segment.set(None);
                                                }
                                            >
                                                <span
                                                    class=prefix_legend_color.clone()
                                                    style=format!("width: 10px; height: 10px; border-radius: 2px; background: {};", color)
                                                ></span>
                                                <span
                                                    class=prefix_legend_label.clone()
                                                    style="color: var(--fx-color-text-secondary, #8c8c8c);"
                                                >
                                                    {segment.label.clone()}
                                                </span>
                                                <span style="color: var(--fx-color-text-tertiary, #595959);">
                                                    {format!("({:.1})", segment.value)}
                                                </span>
                                            </div>
                                        }
                                    }
                                }).collect_view()}
                            </div>
                        })
                    } else {
                        None
                    }
                }
            }

            // Loading overlay
            {
                let prefix_loading = prefix_loading.clone();
                let prefix_spinner = prefix_spinner.clone();
                move || {
                    if loading {
                        Some(view! {
                            <div class=prefix_loading.clone()>
                                <div class=prefix_spinner.clone()></div>
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
