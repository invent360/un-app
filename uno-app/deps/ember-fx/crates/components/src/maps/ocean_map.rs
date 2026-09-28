//! OceanMap component for ocean temperature visualization.
//!
//! This component renders pre-computed ocean temperature data as colored dots
//! representing ocean regions with temperature values.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::try_use_theme;
use super::types::ColorScale;
use super::ocean_map_data::{OCEAN_MAP_REGIONS, OceanMapRegion, REGION_COUNT};

/// Configuration for OceanMap component.
#[derive(Debug, Clone)]
pub struct OceanMapConfig {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Dot size multiplier.
    pub dot_size: f64,
    /// Color scale for temperature coloring.
    pub color_scale: ColorScale,
    /// Enable interactive hover.
    pub interactive: bool,
    /// Show tooltip on hover.
    pub show_tooltip: bool,
    /// Show color legend.
    pub show_legend: bool,
    /// Background color.
    pub background_color: String,
    /// Dot opacity.
    pub dot_opacity: f64,
    /// Minimum value for color scaling.
    pub min_value: f64,
    /// Maximum value for color scaling.
    pub max_value: f64,
}

impl Default for OceanMapConfig {
    fn default() -> Self {
        Self {
            width: 900,
            height: 540,
            dot_size: 3.0,
            color_scale: ColorScale::BlueToRed,
            interactive: true,
            show_tooltip: true,
            show_legend: true,
            background_color: "#0A162D".to_string(),
            dot_opacity: 0.85,
            min_value: 0.0,
            max_value: 1.0,
        }
    }
}

/// Custom color scale for ocean temperatures (blue to cyan).
#[derive(Debug, Clone)]
pub struct OceanColorScale;

impl OceanColorScale {
    /// Get color for a normalized value (0.0 = cold blue, 1.0 = warm cyan).
    pub fn color_at(t: f64) -> String {
        let t = t.clamp(0.0, 1.0);
        // Interpolate from deep blue (#0184F1) to cyan (#8EE7F1)
        let r = (1.0 + (142.0 - 1.0) * t) as u8;
        let g = (132.0 + (231.0 - 132.0) * t) as u8;
        let b = (241.0 + (241.0 - 241.0) * t) as u8;
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Get gradient stops for legend display.
    pub fn gradient_stops() -> Vec<(f64, String)> {
        vec![
            (0.0, "#0184F1".to_string()),
            (0.25, "#0194E7".to_string()),
            (0.5, "#01AADD".to_string()),
            (0.75, "#01B6D7".to_string()),
            (1.0, "#8EE7F1".to_string()),
        ]
    }
}

/// OceanMap component.
///
/// Displays ocean temperature data as colored dots.
///
/// # Example
///
/// ```ignore
/// view! {
///     <OceanMap />
/// }
/// ```
#[component]
pub fn OceanMap(
    /// Configuration options.
    #[prop(optional)]
    config: Option<OceanMapConfig>,
    /// Callback when region is clicked.
    #[prop(optional, into)]
    on_region_click: Option<Callback<OceanMapRegion>>,
    /// Callback when region is hovered.
    #[prop(optional, into)]
    on_region_hover: Option<Callback<Option<OceanMapRegion>>>,
    /// Loading state.
    #[prop(optional)]
    loading: Option<Signal<bool>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let prefix = format!("fx-ocean-map-{}", design_system);

    // Extract config values
    let width = config.width;
    let height = config.height;
    let dot_size = config.dot_size;
    let interactive = config.interactive;
    let show_tooltip = config.show_tooltip;
    let show_legend = config.show_legend;
    let background_color = config.background_color.clone();
    let dot_opacity = config.dot_opacity;
    let min_value = config.min_value;
    let max_value = config.max_value;

    // Reactive state
    let hovered_region = RwSignal::new(None::<usize>);
    let tooltip_pos = RwSignal::new((0.0_f64, 0.0_f64));
    let tooltip_content = RwSignal::new(None::<(f64, f64, f64)>);

    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    view! {
        <div
            class=combined_class
            style=format!(
                "position: relative; width: {}px; height: {}px; background: {};",
                width, height, background_color
            )
        >
            // Loading overlay
            {move || {
                loading.map(|l| l.get()).unwrap_or(false).then(|| view! {
                    <div style="position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.5); z-index: 100;">
                        <div style="color: var(--fx-color-text); font-size: 14px;">{"Loading..."}</div>
                    </div>
                })
            }}

            <svg
                viewBox=format!("0 0 {} {}", width, height)
                style="width: 100%; height: 100%;"
                preserveAspectRatio="xMidYMid meet"
            >
                // Background
                <rect
                    width=width
                    height=height
                    fill=background_color.clone()
                />

                // Ocean regions as dots
                <g class="ocean-regions">
                    {OCEAN_MAP_REGIONS.iter().enumerate().map(|(idx, region)| {
                        let region_copy = *region;
                        let region_for_click = *region;
                        let region_for_hover = *region;
                        let callback_click = on_region_click.clone();
                        let callback_hover = on_region_hover.clone();

                        // Calculate color based on temperature value
                        let t = if max_value > min_value {
                            ((region.value - min_value) / (max_value - min_value)).clamp(0.0, 1.0)
                        } else {
                            0.5
                        };
                        let fill_color = OceanColorScale::color_at(t);

                        // Hover state
                        let opacity = move || {
                            let is_hovered = hovered_region.get() == Some(idx);
                            if is_hovered { 1.0 } else { dot_opacity }
                        };
                        let radius = move || {
                            let is_hovered = hovered_region.get() == Some(idx);
                            if is_hovered { dot_size * 1.5 } else { dot_size }
                        };

                        view! {
                            <circle
                                cx=region.x
                                cy=region.y
                                r=radius
                                fill=fill_color
                                opacity=opacity
                                style=format!(
                                    "cursor: {}; transition: r 0.2s, opacity 0.2s;",
                                    if interactive { "pointer" } else { "default" }
                                )
                                on:mouseenter={
                                    let callback_hover = callback_hover.clone();
                                    move |e| {
                                        if interactive {
                                            hovered_region.set(Some(idx));
                                            if let Some(ref cb) = callback_hover {
                                                cb.run(Some(region_for_hover));
                                            }
                                            if show_tooltip {
                                                tooltip_content.set(Some((
                                                    region_copy.x,
                                                    region_copy.y,
                                                    region_copy.value,
                                                )));
                                                if let Some(rect) = e.target()
                                                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                                                    .and_then(|el| el.closest("svg").ok().flatten())
                                                    .map(|svg| svg.get_bounding_client_rect())
                                                {
                                                    tooltip_pos.set((
                                                        e.client_x() as f64 - rect.left() + 10.0,
                                                        e.client_y() as f64 - rect.top() - 40.0,
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                                on:mouseleave={
                                    let callback_hover = callback_hover.clone();
                                    move |_| {
                                        hovered_region.set(None);
                                        tooltip_content.set(None);
                                        if let Some(ref cb) = callback_hover {
                                            cb.run(None);
                                        }
                                    }
                                }
                                on:click={
                                    let callback_click = callback_click.clone();
                                    move |_| {
                                        if interactive {
                                            if let Some(ref cb) = callback_click {
                                                cb.run(region_for_click);
                                            }
                                        }
                                    }
                                }
                            />
                        }
                    }).collect_view()}
                </g>
            </svg>

            // Tooltip
            {move || {
                tooltip_content.get().map(|(x, y, value)| {
                    let (tx, ty) = tooltip_pos.get();
                    let temp_label = if value < 0.33 {
                        "Cold"
                    } else if value < 0.66 {
                        "Moderate"
                    } else {
                        "Warm"
                    };
                    view! {
                        <div style=format!(
                            "position: absolute; left: {}px; top: {}px; \
                             background: var(--fx-color-bg-elevated, #1f1f1f); \
                             border: 1px solid var(--fx-color-border, #434343); \
                             border-radius: 6px; padding: 8px 12px; \
                             pointer-events: none; z-index: 50; \
                             box-shadow: 0 4px 12px rgba(0,0,0,0.4);",
                            tx, ty
                        )>
                            <div style="font-size: 12px; color: var(--fx-color-text-secondary, #888);">
                                {"Ocean Temperature"}
                            </div>
                            <div style="font-size: 14px; font-weight: 600; color: var(--fx-color-text, #fff);">
                                {format!("{} ({:.0}%)", temp_label, value * 100.0)}
                            </div>
                        </div>
                    }
                })
            }}

            // Legend
            {show_legend.then(|| {
                let gradient_stops = OceanColorScale::gradient_stops();
                let gradient_css = gradient_stops.iter()
                    .map(|(pos, color)| format!("{} {:.0}%", color, pos * 100.0))
                    .collect::<Vec<_>>()
                    .join(", ");

                view! {
                    <div style="position: absolute; bottom: 16px; right: 16px; \
                                background: var(--fx-color-bg-elevated, #1f1f1f); \
                                border: 1px solid var(--fx-color-border, #434343); \
                                border-radius: 6px; padding: 12px;">
                        <div style="font-size: 11px; color: var(--fx-color-text-secondary, #888); margin-bottom: 8px;">
                            {"Temperature"}
                        </div>
                        <div style=format!(
                            "width: 120px; height: 8px; border-radius: 4px; \
                             background: linear-gradient(90deg, {});",
                            gradient_css
                        ) />
                        <div style="display: flex; justify-content: space-between; margin-top: 4px;">
                            <span style="font-size: 10px; color: var(--fx-color-text-secondary, #888);">
                                {"Cold"}
                            </span>
                            <span style="font-size: 10px; color: var(--fx-color-text-secondary, #888);">
                                {"Warm"}
                            </span>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}

/// Standalone legend component for OceanMap.
#[component]
pub fn OceanMapLegend(
    /// Legend title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-ocean-map-legend-{}", design_system);

    let gradient_stops = OceanColorScale::gradient_stops();
    let gradient_css = gradient_stops.iter()
        .map(|(pos, color)| format!("{} {:.0}%", color, pos * 100.0))
        .collect::<Vec<_>>()
        .join(", ");

    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    view! {
        <div
            class=combined_class
            style="background: var(--fx-color-bg-elevated, #1f1f1f); \
                   border: 1px solid var(--fx-color-border, #434343); \
                   border-radius: 6px; padding: 12px; display: inline-block;"
        >
            {title.map(|t| view! {
                <div style="font-size: 12px; font-weight: 500; color: var(--fx-color-text, #fff); margin-bottom: 8px;">
                    {t}
                </div>
            })}

            <div style=format!(
                "width: 150px; height: 10px; border-radius: 4px; \
                 background: linear-gradient(90deg, {});",
                gradient_css
            ) />

            <div style="display: flex; justify-content: space-between; margin-top: 6px;">
                <span style="font-size: 11px; color: var(--fx-color-text-secondary, #888);">
                    {"Cold"}
                </span>
                <span style="font-size: 11px; color: var(--fx-color-text-secondary, #888);">
                    {"Warm"}
                </span>
            </div>
        </div>
    }
}
