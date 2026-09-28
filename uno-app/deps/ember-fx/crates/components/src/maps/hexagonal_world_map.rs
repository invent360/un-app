//! HexagonalWorldMap component for geographic visualization.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::try_use_theme;
use super::types::{
    HexCell, MapLocation, HexWorldMapConfig, HexOrientation, ColorScale,
    hexagon_path, lat_lng_to_pixel, format_number,
};
use super::world_data::generate_world_hexagons;

/// HexagonalWorldMap component.
///
/// Displays geographic data using hexagonal tessellation with heat map coloring.
/// Supports location markers, interactive hover/click, and color legends.
///
/// # Example
///
/// ```ignore
/// let cells = Signal::derive(|| vec![
///     HexCell::new(40.7, -74.0, 85.0).region("North America"),
///     HexCell::new(51.5, -0.1, 62.0).region("Europe"),
/// ]);
///
/// let locations = Signal::derive(|| vec![
///     MapLocation::new("nyc", "New York", 40.7, -74.0, 8_336_817.0)
///         .formatted("8,336,817"),
/// ]);
///
/// view! {
///     <HexagonalWorldMap
///         cells=cells
///         locations=locations
///     />
/// }
/// ```
#[component]
pub fn HexagonalWorldMap(
    /// Hexagonal cells data.
    cells: Signal<Vec<HexCell>>,
    /// City/location markers.
    #[prop(optional)]
    locations: Option<Signal<Vec<MapLocation>>>,
    /// Configuration options.
    #[prop(optional)]
    config: Option<HexWorldMapConfig>,
    /// Callback when cell is clicked.
    #[prop(optional, into)]
    on_cell_click: Option<Callback<HexCell>>,
    /// Callback when location is clicked.
    #[prop(optional, into)]
    on_location_click: Option<Callback<MapLocation>>,
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
    let prefix = format!("fx-hexmap-{}", design_system);

    // Extract config values
    let width = config.width;
    let height = config.height;
    let hex_size = config.hex_size;
    let hex_orientation = config.hex_orientation;
    let color_scale = config.color_scale.clone();
    let color_scale_for_legend = color_scale.clone();
    let show_grid = config.show_grid;
    let show_labels = config.show_labels;
    let show_legend = config.show_legend;
    let interactive = config.interactive;
    let min_value = config.min_value;
    let max_value = config.max_value;
    let show_base_map = config.show_base_map;
    let base_hex_size = config.base_hex_size.unwrap_or(hex_size);

    // Hover state
    let hovered_cell = RwSignal::new(None::<String>);
    let hovered_location = RwSignal::new(None::<String>);

    // Tooltip position and content
    let tooltip_pos = RwSignal::new((0.0_f64, 0.0_f64));
    let tooltip_content = RwSignal::new(None::<(String, String)>);

    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    // Generate world outline paths for reference (simplified continents)
    let world_outlines = generate_world_outlines(width as f64, height as f64);

    // Generate base world hexagons (only if show_base_map is true)
    let base_hexagons: Vec<(f64, f64, String)> = if show_base_map {
        generate_world_hexagons(
            width as f64,
            height as f64,
            base_hex_size,
            hex_orientation,
        ).into_iter().map(|(x, y, r)| (x, y, r.to_string())).collect()
    } else {
        Vec::new()
    };

    view! {
        <div
            class=combined_class
            style=format!(
                "position: relative; width: {}px; height: {}px; background: var(--fx-color-bg, #141414);",
                width, height
            )
        >
            // Loading overlay
            {move || {
                loading.map(|l| l.get()).unwrap_or(false).then(|| view! {
                    <div style="position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.5); z-index: 100;">
                        <div style="color: var(--fx-color-text); font-size: 14px;">Loading...</div>
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
                    fill="var(--fx-color-bg, #141414)"
                />

                // World outline (subtle reference) - hidden since we have hex base layer
                <g class="world-outlines" opacity="0.0">
                    {world_outlines.into_iter().map(|path| view! {
                        <path
                            d=path
                            fill="none"
                            stroke="var(--fx-color-border, #434343)"
                            stroke-width="1"
                        />
                    }).collect_view()}
                </g>

                // Base world hexagons (land masses in muted color)
                <g class="base-hex-layer">
                    {base_hexagons.into_iter().map(|(px, py, _region)| {
                        let hex_path = hexagon_path(px, py, base_hex_size, hex_orientation);
                        view! {
                            <path
                                d=hex_path
                                fill="var(--fx-hex-base-color, #3a5a8f)"
                                stroke="var(--fx-hex-base-stroke, #2a4070)"
                                stroke-width="0.3"
                                opacity="0.85"
                            />
                        }
                    }).collect_view()}
                </g>

                // Hexagonal grid (data layer)
                <g class="hex-cells">
                    {move || {
                        let cell_list = cells.get();
                        let color_scale_clone = color_scale.clone();

                        cell_list.into_iter().map(|cell| {
                            let cell_id = cell.id.clone();
                            let cell_id_for_opacity = cell_id.clone();
                            let cell_id_for_stroke_width = cell_id.clone();
                            let cell_id_for_stroke = cell_id.clone();
                            let cell_id_hover = cell_id.clone();
                            let cell_clone = cell.clone();
                            let cell_for_click = cell.clone();
                            let callback = on_cell_click.clone();

                            // Convert lat/lng to pixel coordinates
                            let (px, py) = lat_lng_to_pixel(
                                cell.lat,
                                cell.lng,
                                width as f64,
                                height as f64,
                            );

                            // Generate hexagon path
                            let hex_path = hexagon_path(px, py, hex_size, hex_orientation);

                            // Calculate color based on value
                            let t = if max_value > min_value {
                                ((cell.value - min_value) / (max_value - min_value)).clamp(0.0, 1.0)
                            } else {
                                0.5
                            };
                            let fill_color = color_scale_clone.color_at(t);

                            // Hover state closures - each captures its own copy
                            let opacity = move || {
                                if hovered_cell.get().as_ref() == Some(&cell_id_for_opacity) { 1.0 } else { 0.8 }
                            };
                            let stroke_width = move || {
                                if hovered_cell.get().as_ref() == Some(&cell_id_for_stroke_width) { 1.5 } else { 0.5 }
                            };
                            let stroke_color = move || {
                                if hovered_cell.get().as_ref() == Some(&cell_id_for_stroke) {
                                    "var(--fx-color-text, #fff)"
                                } else {
                                    "transparent"
                                }
                            };

                            let has_click = callback.is_some();
                            let cursor = if has_click && interactive { "pointer" } else { "default" };

                            view! {
                                <path
                                    d=hex_path
                                    fill=fill_color.clone()
                                    stroke=stroke_color
                                    stroke-width=stroke_width
                                    opacity=opacity
                                    style=format!("cursor: {}; transition: opacity 0.2s, stroke-width 0.2s;", cursor)
                                    on:mouseenter=move |e| {
                                        if interactive {
                                            hovered_cell.set(Some(cell_id_hover.clone()));
                                            // Update tooltip
                                            let region = cell_clone.region.clone().unwrap_or_else(|| "Unknown".to_string());
                                            tooltip_content.set(Some((region, format!("{:.1}", cell_clone.value))));
                                            // Get mouse position relative to SVG
                                            let rect = e.target().unwrap().dyn_into::<web_sys::Element>().ok()
                                                .and_then(|el| el.closest("svg").ok().flatten())
                                                .map(|svg| svg.get_bounding_client_rect());
                                            if let Some(r) = rect {
                                                tooltip_pos.set((
                                                    e.client_x() as f64 - r.left() + 10.0,
                                                    e.client_y() as f64 - r.top() - 30.0,
                                                ));
                                            }
                                        }
                                    }
                                    on:mouseleave=move |_| {
                                        hovered_cell.set(None);
                                        tooltip_content.set(None);
                                    }
                                    on:click=move |_| {
                                        if let Some(ref cb) = callback {
                                            cb.run(cell_for_click.clone());
                                        }
                                    }
                                />
                            }
                        }).collect_view()
                    }}
                </g>

                // Grid lines (optional)
                {show_grid.then(|| view! {
                    <g class="hex-grid" opacity="0.1">
                        // Would render grid pattern here
                    </g>
                })}

                // Location markers
                {locations.map(|locs| view! {
                    <g class="location-markers">
                        {move || {
                            locs.get().into_iter().map(|loc| {
                                let loc_id = loc.id.clone();
                                let loc_id_hover = loc_id.clone();
                                let loc_clone = loc.clone();
                                let loc_for_click = loc.clone();
                                let callback = on_location_click.clone();

                                // Convert to pixel coordinates
                                let (px, py) = lat_lng_to_pixel(
                                    loc.lat,
                                    loc.lng,
                                    width as f64,
                                    height as f64,
                                );

                                // Get color
                                let marker_color = loc.color
                                    .map(|c| c.as_css().to_string())
                                    .unwrap_or_else(|| "var(--fx-color-primary, #1890ff)".to_string());

                                // Hover state
                                let is_hovered = move || hovered_location.get().as_ref() == Some(&loc_id);
                                let scale = move || if is_hovered() { 1.2 } else { 1.0 };

                                let has_click = callback.is_some();
                                let cursor = if has_click { "pointer" } else { "default" };

                                let icon_path = loc.icon.svg_path();
                                let formatted = loc_clone.formatted_value.clone()
                                    .unwrap_or_else(|| format_number(loc_clone.value));

                                view! {
                                    <g
                                        transform=move || format!("translate({}, {}) scale({})", px, py, scale())
                                        style=format!("cursor: {}; transition: transform 0.2s;", cursor)
                                        on:mouseenter=move |_| {
                                            hovered_location.set(Some(loc_id_hover.clone()));
                                        }
                                        on:mouseleave=move |_| {
                                            hovered_location.set(None);
                                        }
                                        on:click=move |_| {
                                            if let Some(ref cb) = callback {
                                                cb.run(loc_for_click.clone());
                                            }
                                        }
                                    >
                                        // Marker circle background
                                        <circle
                                            cx="0"
                                            cy="0"
                                            r="12"
                                            fill=marker_color.clone()
                                            stroke="var(--fx-color-bg-elevated, #1f1f1f)"
                                            stroke-width="2"
                                        />

                                        // Icon inside circle
                                        <g transform="translate(-8, -8) scale(0.67)">
                                            <path
                                                d=icon_path
                                                fill="white"
                                            />
                                        </g>

                                        // Label (when showing labels)
                                        {show_labels.then(|| view! {
                                            <g transform="translate(16, 4)">
                                                // Label background
                                                <rect
                                                    x="-2"
                                                    y="-12"
                                                    width="80"
                                                    height="24"
                                                    rx="4"
                                                    fill="var(--fx-color-bg-elevated, #1f1f1f)"
                                                    stroke="var(--fx-color-border, #434343)"
                                                    stroke-width="1"
                                                />
                                                // Name
                                                <text
                                                    x="4"
                                                    y="-2"
                                                    style="font-size: 10px; fill: var(--fx-color-text-secondary, #888);"
                                                >
                                                    {loc.name.clone()}
                                                </text>
                                                // Value
                                                <text
                                                    x="4"
                                                    y="8"
                                                    style="font-size: 11px; font-weight: 600; fill: var(--fx-color-text, #fff);"
                                                >
                                                    {formatted}
                                                </text>
                                            </g>
                                        })}
                                    </g>
                                }
                            }).collect_view()
                        }}
                    </g>
                })}
            </svg>

            // Tooltip
            {move || {
                tooltip_content.get().map(|(title, value)| {
                    let (tx, ty) = tooltip_pos.get();
                    view! {
                        <div style=format!(
                            "position: absolute; left: {}px; top: {}px; \
                             background: var(--fx-color-bg-elevated, #1f1f1f); \
                             border: 1px solid var(--fx-color-border, #434343); \
                             border-radius: 4px; padding: 8px 12px; \
                             pointer-events: none; z-index: 50; \
                             box-shadow: 0 2px 8px rgba(0,0,0,0.3);",
                            tx, ty
                        )>
                            <div style="font-size: 12px; color: var(--fx-color-text-secondary, #888);">
                                {title}
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

/// Generate simplified world outline paths for visual reference.
/// These are approximate continent shapes projected to the given dimensions.
fn generate_world_outlines(width: f64, height: f64) -> Vec<String> {
    // Simplified continent outlines (approximate lat/lng points converted to path)
    // This is a very simplified representation for visual reference

    let continents = vec![
        // North America (simplified)
        vec![
            (70.0, -170.0), (70.0, -60.0), (50.0, -55.0), (25.0, -80.0),
            (25.0, -100.0), (30.0, -115.0), (50.0, -125.0), (60.0, -145.0),
            (70.0, -170.0),
        ],
        // South America (simplified)
        vec![
            (10.0, -75.0), (5.0, -50.0), (-5.0, -35.0), (-23.0, -43.0),
            (-55.0, -70.0), (-45.0, -75.0), (-23.0, -70.0), (-5.0, -80.0),
            (10.0, -75.0),
        ],
        // Europe (simplified)
        vec![
            (70.0, 30.0), (60.0, 50.0), (45.0, 40.0), (35.0, 25.0),
            (35.0, -10.0), (45.0, -10.0), (55.0, 10.0), (70.0, 30.0),
        ],
        // Africa (simplified)
        vec![
            (35.0, -10.0), (35.0, 35.0), (10.0, 50.0), (-35.0, 20.0),
            (-35.0, 15.0), (5.0, -10.0), (35.0, -10.0),
        ],
        // Asia (simplified)
        vec![
            (70.0, 30.0), (70.0, 180.0), (50.0, 140.0), (35.0, 140.0),
            (10.0, 105.0), (5.0, 80.0), (25.0, 65.0), (35.0, 35.0),
            (45.0, 40.0), (60.0, 50.0), (70.0, 30.0),
        ],
        // Australia (simplified)
        vec![
            (-10.0, 115.0), (-10.0, 150.0), (-25.0, 155.0), (-40.0, 150.0),
            (-35.0, 115.0), (-20.0, 115.0), (-10.0, 115.0),
        ],
    ];

    continents.into_iter().map(|points| {
        let path_points: Vec<String> = points.iter().map(|(lat, lng)| {
            let (x, y) = lat_lng_to_pixel(*lat, *lng, width, height);
            format!("{:.1},{:.1}", x, y)
        }).collect();

        format!("M{}", path_points.join(" L"))
    }).collect()
}

/// HexagonalWorldMapLegend component for standalone legend display.
#[component]
pub fn HexagonalWorldMapLegend(
    /// Color scale to display.
    color_scale: ColorScale,
    /// Minimum value.
    #[prop(optional, default = 0.0)]
    min_value: f64,
    /// Maximum value.
    #[prop(optional, default = 100.0)]
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

    let prefix = format!("fx-hexmap-legend-{}", design_system);

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
                    {format!("{:.0}", min_value)}
                </span>
                <span style="font-size: 11px; color: var(--fx-color-text-secondary, #888);">
                    {format!("{:.0}", max_value)}
                </span>
            </div>
        </div>
    }
}
