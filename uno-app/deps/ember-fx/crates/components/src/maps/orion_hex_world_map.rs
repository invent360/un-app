//! OrionHexWorldMap component for hexagonal world visualization.
//!
//! This component renders a pre-computed hexagonal world map from the Orion UI Kit
//! design, with support for value-based coloring, hover/click interactions, and legends.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::try_use_theme;
use super::types::{ColorScale, hexagon_path, HexOrientation};
use super::orion_hex_data::{HEX_MAP_CELLS, HexMapCell};

/// Configuration for OrionHexWorldMap component.
#[derive(Debug, Clone)]
pub struct OrionHexWorldMapConfig {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Hexagon size (radius).
    pub hex_size: f64,
    /// Color scale for heatmap coloring.
    pub color_scale: ColorScale,
    /// Ocean color (for cells with is_land = false).
    pub ocean_color: String,
    /// Enable interactive hover/click.
    pub interactive: bool,
    /// Show tooltip on hover.
    pub show_tooltip: bool,
    /// Show color legend.
    pub show_legend: bool,
    /// Minimum value for color scaling.
    pub min_value: f64,
    /// Maximum value for color scaling.
    pub max_value: f64,
    /// Show only land cells (hide ocean).
    pub land_only: bool,
    /// Show only ocean cells (hide land).
    pub ocean_only: bool,
    /// Stroke color for hexagons.
    pub stroke_color: String,
    /// Stroke width for hexagons.
    pub stroke_width: f64,
    /// Background color.
    pub background_color: String,
}

impl Default for OrionHexWorldMapConfig {
    fn default() -> Self {
        Self {
            width: 900,
            height: 500,
            hex_size: 4.5,
            color_scale: ColorScale::PurpleToOrange,
            ocean_color: "#1F1F43".to_string(),
            interactive: true,
            show_tooltip: true,
            show_legend: true,
            min_value: 0.0,
            max_value: 1.0,
            land_only: false,
            ocean_only: false,
            stroke_color: "#0a0a15".to_string(),
            stroke_width: 0.3,
            background_color: "#0D0D10".to_string(),
        }
    }
}

/// Runtime cell state with custom value.
#[derive(Debug, Clone)]
pub struct OrionHexCellState {
    /// Cell index (matches position in HEX_MAP_CELLS).
    pub index: usize,
    /// Custom value for coloring (overrides default).
    pub value: f64,
    /// Whether this cell is selected.
    pub selected: bool,
}

impl OrionHexCellState {
    /// Create a new cell state.
    pub fn new(index: usize, value: f64) -> Self {
        Self {
            index,
            value,
            selected: false,
        }
    }

    /// Set selected state.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

/// OrionHexWorldMap component.
///
/// Displays a pre-computed hexagonal world map from the Orion UI Kit design
/// with interactive features and value-based coloring.
///
/// # Example
///
/// ```ignore
/// view! {
///     <OrionHexWorldMap />
/// }
///
/// // With custom values
/// let custom_values = Signal::derive(|| vec![
///     OrionHexCellState::new(0, 0.8),
///     OrionHexCellState::new(100, 0.5),
/// ]);
///
/// view! {
///     <OrionHexWorldMap
///         custom_values=custom_values
///         on_cell_click=|cell| log!("Clicked: {:?}", cell)
///     />
/// }
/// ```
#[component]
pub fn OrionHexWorldMap(
    /// Custom values for specific cells (overrides defaults).
    #[prop(optional)]
    custom_values: Option<Signal<Vec<OrionHexCellState>>>,
    /// Configuration options.
    #[prop(optional)]
    config: Option<OrionHexWorldMapConfig>,
    /// Callback when cell is clicked.
    #[prop(optional, into)]
    on_cell_click: Option<Callback<HexMapCell>>,
    /// Callback when cell is hovered.
    #[prop(optional, into)]
    on_cell_hover: Option<Callback<Option<HexMapCell>>>,
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
    let prefix = format!("fx-orion-hexmap-{}", design_system);

    // Extract config values
    let width = config.width;
    let height = config.height;
    let hex_size = config.hex_size;
    let color_scale = config.color_scale.clone();
    let ocean_color = config.ocean_color.clone();
    let interactive = config.interactive;
    let show_tooltip = config.show_tooltip;
    let show_legend = config.show_legend;
    let min_value = config.min_value;
    let max_value = config.max_value;
    let land_only = config.land_only;
    let ocean_only = config.ocean_only;
    let stroke_color = config.stroke_color.clone();
    let stroke_width = config.stroke_width;
    let background_color = config.background_color.clone();
    let color_scale_for_legend = color_scale.clone();

    // Reactive state
    let hovered_cell = RwSignal::new(None::<usize>);
    let tooltip_pos = RwSignal::new((0.0_f64, 0.0_f64));
    let tooltip_content = RwSignal::new(None::<(String, String)>);

    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    // Filter cells based on config
    let cells: Vec<(usize, &HexMapCell)> = HEX_MAP_CELLS
        .iter()
        .enumerate()
        .filter(|(_, cell)| {
            if land_only && !cell.is_land {
                return false;
            }
            if ocean_only && cell.is_land {
                return false;
            }
            true
        })
        .collect();

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

                // Hexagon cells
                <g class="hex-cells">
                    {cells.into_iter().map(|(idx, cell)| {
                        let cell_clone = *cell;
                        let cell_for_click = *cell;
                        let cell_for_hover = *cell;
                        let callback_click = on_cell_click.clone();
                        let callback_hover = on_cell_hover.clone();
                        let color_scale = color_scale.clone();
                        let ocean_color = ocean_color.clone();
                        let stroke_color = stroke_color.clone();

                        // Generate hexagon path at cell position
                        let hex_path = hexagon_path(cell.x, cell.y, hex_size, HexOrientation::PointyTop);

                        // Calculate fill color
                        let fill_color = {
                            let color_scale = color_scale.clone();
                            let ocean_color = ocean_color.clone();
                            move || {
                                // Check for custom value
                                let custom_value = custom_values
                                    .map(|cv| cv.get())
                                    .and_then(|values| values.iter().find(|s| s.index == idx).map(|s| s.value));

                                let value = custom_value.unwrap_or(cell_clone.value);

                                if !cell_clone.is_land {
                                    ocean_color.clone()
                                } else {
                                    let t = if max_value > min_value {
                                        ((value - min_value) / (max_value - min_value)).clamp(0.0, 1.0)
                                    } else {
                                        0.5
                                    };
                                    color_scale.color_at(t)
                                }
                            }
                        };

                        // Hover state
                        let opacity = move || {
                            let is_hovered = hovered_cell.get() == Some(idx);
                            if is_hovered { 1.0 } else { 0.85 }
                        };
                        let stroke_w = move || {
                            let is_hovered = hovered_cell.get() == Some(idx);
                            if is_hovered { stroke_width * 3.0 } else { stroke_width }
                        };
                        let stroke_c = {
                            let stroke_color = stroke_color.clone();
                            move || {
                                let is_hovered = hovered_cell.get() == Some(idx);
                                if is_hovered {
                                    "var(--fx-color-text, #fff)".to_string()
                                } else {
                                    stroke_color.clone()
                                }
                            }
                        };

                        view! {
                            <path
                                d=hex_path
                                fill=fill_color
                                stroke=stroke_c
                                stroke-width=stroke_w
                                opacity=opacity
                                style=format!(
                                    "cursor: {}; transition: opacity 0.2s, stroke-width 0.2s;",
                                    if interactive && cell_clone.is_land { "pointer" } else { "default" }
                                )
                                on:mouseenter={
                                    let callback_hover = callback_hover.clone();
                                    move |e| {
                                        if interactive {
                                            hovered_cell.set(Some(idx));
                                            if let Some(ref cb) = callback_hover {
                                                cb.run(Some(cell_for_hover));
                                            }
                                            if show_tooltip && cell_for_hover.is_land {
                                                let cell_type = if cell_for_hover.is_land { "Land" } else { "Ocean" };
                                                tooltip_content.set(Some((
                                                    cell_type.to_string(),
                                                    format!("Value: {:.2}", cell_for_hover.value),
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
                                        hovered_cell.set(None);
                                        tooltip_content.set(None);
                                        if let Some(ref cb) = callback_hover {
                                            cb.run(None);
                                        }
                                    }
                                }
                                on:click={
                                    let callback_click = callback_click.clone();
                                    move |_| {
                                        if interactive && cell_for_click.is_land {
                                            if let Some(ref cb) = callback_click {
                                                cb.run(cell_for_click);
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
                tooltip_content.get().map(|(cell_type, value)| {
                    let (tx, ty) = tooltip_pos.get();
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
                                {cell_type}
                            </div>
                            <div style="font-size: 14px; font-weight: 600; color: var(--fx-color-text, #fff);">
                                {value}
                            </div>
                        </div>
                    }
                })
            }}

            // Legend
            {show_legend.then(|| {
                let gradient_stops = color_scale_for_legend.gradient_stops();
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
                            {"Value Scale"}
                        </div>
                        <div style=format!(
                            "width: 120px; height: 8px; border-radius: 4px; \
                             background: linear-gradient(90deg, {});",
                            gradient_css
                        ) />
                        <div style="display: flex; justify-content: space-between; margin-top: 4px;">
                            <span style="font-size: 10px; color: var(--fx-color-text-secondary, #888);">
                                {format!("{:.0}", min_value)}
                            </span>
                            <span style="font-size: 10px; color: var(--fx-color-text-secondary, #888);">
                                {format!("{:.0}", max_value)}
                            </span>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}

/// Standalone legend component for OrionHexWorldMap.
#[component]
pub fn OrionHexWorldMapLegend(
    /// Color scale to display.
    color_scale: ColorScale,
    /// Minimum value.
    #[prop(optional, default = 0.0)]
    min_value: f64,
    /// Maximum value.
    #[prop(optional, default = 1.0)]
    max_value: f64,
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

    let prefix = format!("fx-orion-hexmap-legend-{}", design_system);

    let gradient_stops = color_scale.gradient_stops();
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
                    {format!("{:.2}", min_value)}
                </span>
                <span style="font-size: 11px; color: var(--fx-color-text-secondary, #888);">
                    {format!("{:.2}", max_value)}
                </span>
            </div>
        </div>
    }
}
