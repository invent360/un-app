//! DotWorldMap component for geographic visualization.
//!
//! An interactive world map using dot/circle patterns with network nodes,
//! data flow particles, drag/pan mode, and zoom controls.

use leptos::prelude::*;
use std::collections::HashMap;
use crate::try_use_theme;
use super::dot_world_map_data::{
    NEUTRAL_COLOR, ACTIVE_COLOR, HOVER_COLOR,
    VIEWBOX, VIEWBOX_WIDTH, VIEWBOX_HEIGHT,
    FEDERATION_SIZE, FEDERATION_RADIUS, FLEET_RADIUS,
    get_path_data, ColorPalette, DotNodeType, DotCursorMode,
    DotNetworkNode, DotNetworkConnection, DotDataParticle, DotTooltipData,
    generate_demo_nodes, generate_demo_connections,
};

/// Map interaction state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DotWorldMapState {
    /// Default gray state.
    #[default]
    Neutral,
    /// Hovering over the map.
    Hover,
    /// Map has been clicked/activated.
    Active,
}

/// Configuration for DotWorldMap.
#[derive(Debug, Clone)]
pub struct DotWorldMapConfig {
    // Display
    pub width: u32,
    pub height: u32,
    pub background_color: String,

    // Zoom
    pub enable_zoom: bool,
    pub show_zoom_controls: bool,
    pub min_zoom: f64,
    pub max_zoom: f64,
    pub zoom_step: f64,

    // Interaction
    pub interactive: bool,
    pub enable_drag: bool,
    pub show_mode_toggle: bool,
    pub default_cursor_mode: DotCursorMode,

    // Color palette
    pub default_palette: ColorPalette,
    pub show_palette_selector: bool,

    // Network visualization
    pub show_network_nodes: bool,
    pub show_connections: bool,
    pub show_data_flow: bool,
    pub show_flow_toggle: bool,
    pub show_flow_by_default: bool,

    // Tooltips
    pub show_country_tooltips: bool,
    pub show_node_tooltips: bool,

    // Animation
    pub particle_speed: f64,
    pub max_particles: usize,

    // Legend
    pub show_legend: bool,
}

impl Default for DotWorldMapConfig {
    fn default() -> Self {
        Self {
            width: 1000,
            height: 550,
            background_color: "var(--fx-color-bg, #141414)".to_string(),
            enable_zoom: true,
            show_zoom_controls: true,
            min_zoom: 0.5,
            max_zoom: 4.0,
            zoom_step: 0.25,
            interactive: true,
            enable_drag: true,
            show_mode_toggle: true,
            default_cursor_mode: DotCursorMode::Explore,
            default_palette: ColorPalette::Blue,
            show_palette_selector: true,
            show_network_nodes: true,
            show_connections: true,
            show_data_flow: true,
            show_flow_toggle: true,
            show_flow_by_default: true,
            show_country_tooltips: true,
            show_node_tooltips: true,
            particle_speed: 0.02,
            max_particles: 50,
            show_legend: true,
        }
    }
}

/// DotWorldMap component with network visualization.
#[component]
pub fn DotWorldMap(
    #[prop(optional)]
    config: Option<DotWorldMapConfig>,
    #[prop(optional)]
    nodes: Option<Signal<Vec<DotNetworkNode>>>,
    #[prop(optional)]
    connections: Option<Signal<Vec<DotNetworkConnection>>>,
    #[prop(optional, into)]
    on_node_click: Option<Callback<DotNetworkNode>>,
    #[prop(optional, into)]
    on_node_hover: Option<Callback<Option<String>>>,
    #[prop(optional, into)]
    on_zoom_change: Option<Callback<f64>>,
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let prefix = format!("fx-dot-world-map-{}", design_system);

    // Extract config values
    let width = config.width;
    let height = config.height;
    let background_color = config.background_color.clone();
    let enable_zoom = config.enable_zoom;
    let show_zoom_controls = config.show_zoom_controls;
    let min_zoom = config.min_zoom;
    let max_zoom = config.max_zoom;
    let zoom_step = config.zoom_step;
    let interactive = config.interactive;
    let enable_drag = config.enable_drag;
    let show_mode_toggle = config.show_mode_toggle;
    let default_cursor_mode = config.default_cursor_mode;
    let default_palette = config.default_palette;
    let show_palette_selector = config.show_palette_selector;
    let show_network_nodes = config.show_network_nodes;
    let show_connections = config.show_connections;
    let show_data_flow = config.show_data_flow;
    let show_flow_toggle = config.show_flow_toggle;
    let show_flow_by_default = config.show_flow_by_default;
    let _show_country_tooltips = config.show_country_tooltips;
    let show_node_tooltips = config.show_node_tooltips;
    let particle_speed = config.particle_speed;
    let max_particles = config.max_particles;
    let show_legend = config.show_legend;

    // Get path data
    let path_data = get_path_data();
    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    // ========================================================================
    // Reactive State
    // ========================================================================

    // Map state
    let map_state = RwSignal::new(DotWorldMapState::Neutral);
    let is_hovering = RwSignal::new(false);

    // Zoom and pan
    let zoom_level = RwSignal::new(1.0_f64);
    let pan_offset = RwSignal::new((0.0_f64, 0.0_f64));
    let is_dragging = RwSignal::new(false);
    let last_mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));

    // UI state
    let cursor_mode = RwSignal::new(default_cursor_mode);
    let selected_palette = RwSignal::new(default_palette);
    let palette_open = RwSignal::new(false);
    let show_flow = RwSignal::new(show_flow_by_default);

    // Network state - use provided or generate demo
    let internal_nodes = RwSignal::new(generate_demo_nodes());
    let nodes_signal = nodes.unwrap_or_else(|| Signal::derive(move || internal_nodes.get()));

    let internal_connections = RwSignal::new(Vec::<DotNetworkConnection>::new());
    let connections_signal = connections.unwrap_or_else(|| Signal::derive(move || internal_connections.get()));

    // Generate connections from nodes if not provided
    Effect::new(move || {
        if connections.is_none() {
            let nodes = nodes_signal.get();
            let conns = generate_demo_connections(&nodes);
            internal_connections.set(conns);
        }
    });

    // Particles for data flow animation
    let particles = RwSignal::new(Vec::<DotDataParticle>::new());
    let particle_counter = RwSignal::new(0u32);
    let frame_counter = RwSignal::new(0u32);

    // Tooltip state
    let tooltip_content = RwSignal::new(None::<DotTooltipData>);
    let tooltip_pos = RwSignal::new((0.0_f64, 0.0_f64));
    let hovered_node_id = RwSignal::new(None::<String>);

    // Country node counts (memoized)
    let country_counts = Memo::new(move |_| {
        let nodes = nodes_signal.get();
        let mut counts: HashMap<&str, (usize, usize, &str)> = HashMap::new();
        for node in &nodes {
            let entry = counts.entry(node.country_code).or_insert((0, 0, node.country_name));
            match node.node_type {
                DotNodeType::Federation => entry.0 += 1,
                DotNodeType::Fleet => entry.1 += 1,
            }
        }
        counts
    });

    // ========================================================================
    // Animation Loop
    // ========================================================================

    // Particle animation (every 50ms)
    Effect::new(move || {
        if !show_flow.get() || !show_data_flow {
            return;
        }

        let handle = set_interval_with_handle(
            move || {
                frame_counter.update(|f| *f = f.wrapping_add(1));

                // Update particle positions
                particles.update(|ps| {
                    for p in ps.iter_mut() {
                        p.progress += p.speed;
                    }
                    ps.retain(|p| p.progress < 1.0);
                });

                // Spawn new particles
                let conns = connections_signal.get();
                let nodes = nodes_signal.get();
                if !conns.is_empty() && particles.get().len() < max_particles {
                    let frame = frame_counter.get();
                    // Spawn every 3 frames
                    if frame % 3 == 0 {
                        let conn_idx = (frame as usize / 3) % conns.len();
                        let conn = &conns[conn_idx];

                        // Verify both nodes exist
                        let source_exists = nodes.iter().any(|n| n.id == conn.source_id);
                        let target_exists = nodes.iter().any(|n| n.id == conn.target_id);

                        if source_exists && target_exists {
                            let id = particle_counter.get();
                            particle_counter.update(|c| *c = c.wrapping_add(1));

                            particles.update(|ps| {
                                ps.push(DotDataParticle {
                                    id,
                                    source_id: conn.source_id.clone(),
                                    target_id: conn.target_id.clone(),
                                    progress: 0.0,
                                    speed: particle_speed,
                                });
                            });
                        }
                    }
                }
            },
            std::time::Duration::from_millis(50),
        );

        on_cleanup(move || {
            if let Ok(h) = handle {
                h.clear();
            }
        });
    });

    // ========================================================================
    // ViewBox Computation
    // ========================================================================

    let viewbox = move || {
        let zoom = zoom_level.get();
        let (px, py) = pan_offset.get();
        let vw = VIEWBOX_WIDTH / zoom;
        let vh = VIEWBOX_HEIGHT / zoom;
        // Center the viewbox with pan offset
        let x = px - 1.0; // Account for original viewbox offset
        let y = py - 1.0;
        format!("{:.2} {:.2} {:.2} {:.2}", x, y, vw, vh)
    };

    // ========================================================================
    // Event Handlers
    // ========================================================================

    // Zoom handlers
    let zoom_in = move |_: web_sys::MouseEvent| {
        if enable_zoom {
            zoom_level.update(|z| *z = (*z + zoom_step).min(max_zoom));
            if let Some(ref cb) = on_zoom_change {
                cb.run(zoom_level.get());
            }
        }
    };

    let zoom_out = move |_: web_sys::MouseEvent| {
        if enable_zoom {
            zoom_level.update(|z| *z = (*z - zoom_step).max(min_zoom));
            if let Some(ref cb) = on_zoom_change {
                cb.run(zoom_level.get());
            }
        }
    };

    let zoom_reset = move |_: web_sys::MouseEvent| {
        if enable_zoom {
            zoom_level.set(1.0);
            pan_offset.set((0.0, 0.0));
            if let Some(ref cb) = on_zoom_change {
                cb.run(1.0);
            }
        }
    };

    // Drag handlers
    let handle_mousedown = move |e: web_sys::MouseEvent| {
        if enable_drag && cursor_mode.get() == DotCursorMode::Drag {
            is_dragging.set(true);
            last_mouse_pos.set((e.client_x() as f64, e.client_y() as f64));
            e.prevent_default();
        }
    };

    let handle_mousemove = move |e: web_sys::MouseEvent| {
        if is_dragging.get() && cursor_mode.get() == DotCursorMode::Drag {
            let (lx, ly) = last_mouse_pos.get();
            let zoom = zoom_level.get();
            // Scale factor: viewbox units per screen pixel
            let scale = VIEWBOX_WIDTH / (width as f64 * zoom);

            let dx = (e.client_x() as f64 - lx) * scale;
            let dy = (e.client_y() as f64 - ly) * scale;

            pan_offset.update(|(px, py)| {
                *px -= dx;
                *py -= dy;
            });

            last_mouse_pos.set((e.client_x() as f64, e.client_y() as f64));
        }
    };

    let handle_mouseup = move |_: web_sys::MouseEvent| {
        is_dragging.set(false);
    };

    let handle_mouseleave = move |_: web_sys::MouseEvent| {
        is_dragging.set(false);
        is_hovering.set(false);
    };

    // Map hover
    let handle_map_enter = move |_: web_sys::MouseEvent| {
        if interactive && cursor_mode.get() == DotCursorMode::Explore {
            is_hovering.set(true);
        }
    };

    // Map click
    let handle_map_click = move |_: web_sys::MouseEvent| {
        if interactive && cursor_mode.get() == DotCursorMode::Explore {
            map_state.update(|state| {
                *state = match *state {
                    DotWorldMapState::Neutral | DotWorldMapState::Hover => DotWorldMapState::Active,
                    DotWorldMapState::Active => DotWorldMapState::Neutral,
                };
            });
        }
    };

    // Toggle functions
    let toggle_flow = move |_: web_sys::MouseEvent| {
        show_flow.update(|v| *v = !*v);
    };

    let toggle_mode = move |_: web_sys::MouseEvent| {
        cursor_mode.update(|m| {
            *m = match *m {
                DotCursorMode::Explore => DotCursorMode::Drag,
                DotCursorMode::Drag => DotCursorMode::Explore,
            };
        });
    };

    // ========================================================================
    // Render
    // ========================================================================

    // Compute map fill color based on palette
    let map_fill = move || {
        let palette = selected_palette.get();
        if is_hovering.get() {
            // Slightly brighter on hover
            match palette {
                ColorPalette::Blue => "#2563eb",
                ColorPalette::Purple => "#7c3aed",
                ColorPalette::Pink => "#db2777",
                ColorPalette::Cyan => "#06b6d4",
                ColorPalette::Green => "#10b981",
                ColorPalette::Amber => "#f59e0b",
                _ => palette.country_fill(),
            }
        } else {
            palette.country_fill()
        }
    };

    view! {
        <div
            class=combined_class
            style=format!(
                "position: relative; width: {}px; height: {}px; background: {}; border-radius: 8px; overflow: hidden;",
                width, height, background_color
            )
        >
            <svg
                viewBox=viewbox
                style=move || format!(
                    "width: 100%; height: 100%; cursor: {};",
                    if cursor_mode.get() == DotCursorMode::Drag {
                        if is_dragging.get() { "grabbing" } else { "grab" }
                    } else {
                        "default"
                    }
                )
                preserveAspectRatio="xMidYMid meet"
                on:mousedown=handle_mousedown
                on:mousemove=handle_mousemove
                on:mouseup=handle_mouseup
                on:mouseleave=handle_mouseleave
                on:mouseenter=handle_map_enter
                on:click=handle_map_click
            >
                // World dot map
                <path
                    d=path_data
                    fill=map_fill
                    style="transition: fill 0.3s ease;"
                />

                // Connections layer
                {move || {
                    if !show_connections { return None; }

                    let nodes = nodes_signal.get();
                    let conns = connections_signal.get();
                    let palette = selected_palette.get();

                    Some(view! {
                        <g class="connections">
                            {conns.iter().filter_map(|conn| {
                                let source = nodes.iter().find(|n| n.id == conn.source_id)?;
                                let target = nodes.iter().find(|n| n.id == conn.target_id)?;

                                // Quadratic bezier control point (curved line)
                                let mid_x = (source.x + target.x) / 2.0;
                                let mid_y = (source.y + target.y) / 2.0;
                                let dx = target.x - source.x;
                                let dy = target.y - source.y;
                                let ctrl_x = mid_x - dy * 0.2;
                                let ctrl_y = mid_y + dx * 0.2;

                                let path_d = format!(
                                    "M{:.1},{:.1} Q{:.1},{:.1} {:.1},{:.1}",
                                    source.x, source.y, ctrl_x, ctrl_y, target.x, target.y
                                );

                                Some(view! {
                                    <path
                                        d=path_d
                                        fill="none"
                                        stroke=palette.connection_color()
                                        stroke-width="0.3"
                                        opacity="0.4"
                                    />
                                })
                            }).collect_view()}
                        </g>
                    })
                }}

                // Data flow particles
                {move || {
                    if !show_flow.get() || !show_data_flow { return None; }

                    let nodes = nodes_signal.get();
                    let ps = particles.get();
                    let palette = selected_palette.get();

                    Some(view! {
                        <g class="particles">
                            {ps.iter().filter_map(|p| {
                                let source = nodes.iter().find(|n| n.id == p.source_id)?;
                                let target = nodes.iter().find(|n| n.id == p.target_id)?;

                                // Quadratic bezier interpolation
                                let t = p.progress;
                                let t1 = 1.0 - t;
                                let mid_x = (source.x + target.x) / 2.0;
                                let mid_y = (source.y + target.y) / 2.0;
                                let dx = target.x - source.x;
                                let dy = target.y - source.y;
                                let ctrl_x = mid_x - dy * 0.2;
                                let ctrl_y = mid_y + dx * 0.2;

                                let px = t1 * t1 * source.x + 2.0 * t1 * t * ctrl_x + t * t * target.x;
                                let py = t1 * t1 * source.y + 2.0 * t1 * t * ctrl_y + t * t * target.y;

                                Some(view! {
                                    <circle
                                        cx=format!("{:.2}", px)
                                        cy=format!("{:.2}", py)
                                        r="0.8"
                                        fill=palette.particle_color()
                                        opacity=format!("{:.2}", 0.8 - t * 0.5)
                                    >
                                        <animate
                                            attributeName="r"
                                            values="0.6;1.0;0.6"
                                            dur="0.5s"
                                            repeatCount="indefinite"
                                        />
                                    </circle>
                                })
                            }).collect_view()}
                        </g>
                    })
                }}

                // Network nodes - Federations as rounded rects, Fleets as circles inside
                {move || {
                    if !show_network_nodes { return None; }

                    let nodes: Vec<DotNetworkNode> = nodes_signal.get();
                    let palette = selected_palette.get();
                    let (fed_color, fleet_color) = palette.node_colors();

                    // Separate federations and fleets
                    let federations: Vec<_> = nodes.iter()
                        .filter(|n| n.node_type == DotNodeType::Federation)
                        .cloned()
                        .collect();
                    let fleets: Vec<_> = nodes.iter()
                        .filter(|n| n.node_type == DotNodeType::Fleet)
                        .cloned()
                        .collect();

                    Some(view! {
                        <g class="network-nodes">
                            // Render Federation rounded rectangles first (background)
                            {federations.iter().map(|fed| {
                                let fed_id = fed.id.clone();
                                let fed_id_hover = fed.id.clone();
                                let fed_clone = fed.clone();
                                let fed_for_tooltip = fed.clone();
                                let fed_for_click = fed.clone();
                                let on_node_click = on_node_click.clone();
                                let on_node_hover = on_node_hover.clone();

                                // Count fleets for this federation
                                let fleet_count = fleets.iter()
                                    .filter(|f| f.parent_id.as_ref() == Some(&fed_id))
                                    .count();

                                // Calculate rect width based on fleet count
                                let rect_width = if fleet_count <= 1 {
                                    FEDERATION_SIZE
                                } else {
                                    FEDERATION_SIZE + (fleet_count as f64 - 1.0) * 2.5
                                };
                                let rect_height = FEDERATION_SIZE;
                                let rect_x = fed.x - rect_width / 2.0;
                                let rect_y = fed.y - rect_height / 2.0;

                                view! {
                                    <g class="federation-group">
                                        // Glow effect on hover
                                        {move || {
                                            let is_hovered = hovered_node_id.get().as_ref() == Some(&fed_id_hover);
                                            is_hovered.then(|| view! {
                                                <rect
                                                    x=format!("{:.2}", rect_x - 1.5)
                                                    y=format!("{:.2}", rect_y - 1.5)
                                                    width=format!("{:.2}", rect_width + 3.0)
                                                    height=format!("{:.2}", rect_height + 3.0)
                                                    rx=format!("{:.2}", FEDERATION_RADIUS + 0.5)
                                                    ry=format!("{:.2}", FEDERATION_RADIUS + 0.5)
                                                    fill=fed_color
                                                    opacity="0.3"
                                                />
                                            })
                                        }}
                                        // Federation rounded rectangle
                                        <rect
                                            x=format!("{:.2}", rect_x)
                                            y=format!("{:.2}", rect_y)
                                            width=format!("{:.2}", rect_width)
                                            height=format!("{:.2}", rect_height)
                                            rx=format!("{:.2}", FEDERATION_RADIUS)
                                            ry=format!("{:.2}", FEDERATION_RADIUS)
                                            fill=fed_color
                                            stroke="rgba(255,255,255,0.3)"
                                            stroke-width="0.3"
                                            style="cursor: pointer;"
                                            on:mouseenter=move |e: web_sys::MouseEvent| {
                                                if show_node_tooltips && cursor_mode.get() == DotCursorMode::Explore {
                                                    hovered_node_id.set(Some(fed_clone.id.clone()));
                                                    tooltip_content.set(Some(DotTooltipData::Node {
                                                        name: fed_for_tooltip.name.clone(),
                                                        node_type: DotNodeType::Federation,
                                                        country: fed_for_tooltip.country_name.to_string(),
                                                    }));
                                                    tooltip_pos.set((e.client_x() as f64 + 15.0, e.client_y() as f64 + 15.0));
                                                    if let Some(ref cb) = on_node_hover {
                                                        cb.run(Some(fed_clone.id.clone()));
                                                    }
                                                }
                                            }
                                            on:mousemove=move |e: web_sys::MouseEvent| {
                                                if tooltip_content.get().is_some() {
                                                    tooltip_pos.set((e.client_x() as f64 + 15.0, e.client_y() as f64 + 15.0));
                                                }
                                            }
                                            on:mouseleave=move |_: web_sys::MouseEvent| {
                                                hovered_node_id.set(None);
                                                tooltip_content.set(None);
                                            }
                                            on:click=move |e: web_sys::MouseEvent| {
                                                e.stop_propagation();
                                                if let Some(ref cb) = on_node_click {
                                                    cb.run(fed_for_click.clone());
                                                }
                                            }
                                        />
                                    </g>
                                }
                            }).collect_view()}

                            // Render Fleet circles on top (inside Federation rects)
                            {fleets.iter().map(|fleet| {
                                let fleet_clone = fleet.clone();
                                let fleet_for_tooltip = fleet.clone();
                                let fleet_for_click = fleet.clone();
                                let on_node_click = on_node_click.clone();
                                let on_node_hover = on_node_hover.clone();

                                view! {
                                    <circle
                                        cx=format!("{:.2}", fleet.x)
                                        cy=format!("{:.2}", fleet.y)
                                        r=format!("{:.2}", FLEET_RADIUS)
                                        fill=fleet_color
                                        stroke="rgba(255,255,255,0.4)"
                                        stroke-width="0.2"
                                        style="cursor: pointer;"
                                        on:mouseenter=move |e: web_sys::MouseEvent| {
                                            if show_node_tooltips && cursor_mode.get() == DotCursorMode::Explore {
                                                hovered_node_id.set(Some(fleet_clone.id.clone()));
                                                tooltip_content.set(Some(DotTooltipData::Node {
                                                    name: fleet_for_tooltip.name.clone(),
                                                    node_type: DotNodeType::Fleet,
                                                    country: fleet_for_tooltip.country_name.to_string(),
                                                }));
                                                tooltip_pos.set((e.client_x() as f64 + 15.0, e.client_y() as f64 + 15.0));
                                                if let Some(ref cb) = on_node_hover {
                                                    cb.run(Some(fleet_clone.id.clone()));
                                                }
                                            }
                                        }
                                        on:mousemove=move |e: web_sys::MouseEvent| {
                                            if tooltip_content.get().is_some() {
                                                tooltip_pos.set((e.client_x() as f64 + 15.0, e.client_y() as f64 + 15.0));
                                            }
                                        }
                                        on:mouseleave=move |_: web_sys::MouseEvent| {
                                            hovered_node_id.set(None);
                                            tooltip_content.set(None);
                                        }
                                        on:click=move |e: web_sys::MouseEvent| {
                                            e.stop_propagation();
                                            if let Some(ref cb) = on_node_click {
                                                cb.run(fleet_for_click.clone());
                                            }
                                        }
                                    />
                                }
                            }).collect_view()}
                        </g>
                    })
                }}
            </svg>

            // ================================================================
            // UI Controls
            // ================================================================

            // Palette selector (top-left)
            {move || {
                if !show_palette_selector { return None; }

                let current_palette = selected_palette.get();
                let is_open = palette_open.get();

                Some(view! {
                    <div style="position: absolute; top: 16px; left: 16px; z-index: 100;">
                        <button
                            style=format!(
                                "display: flex; align-items: center; gap: 8px; \
                                 background: var(--fx-color-bg-elevated, #1f1f1f); \
                                 border: 1px solid var(--fx-color-border, #434343); \
                                 border-radius: 6px; padding: 8px 12px; \
                                 color: var(--fx-color-text, #fff); font-size: 12px; \
                                 cursor: pointer;"
                            )
                            on:click=move |_: web_sys::MouseEvent| palette_open.update(|v| *v = !*v)
                        >
                            <span style=format!(
                                "width: 12px; height: 12px; border-radius: 3px; background: {};",
                                current_palette.country_fill()
                            )></span>
                            {current_palette.label()}
                            <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M7 10l5 5 5-5z"/>
                            </svg>
                        </button>

                        {is_open.then(|| view! {
                            <div style="position: absolute; top: 100%; left: 0; margin-top: 4px; \
                                        background: var(--fx-color-bg-elevated, #1f1f1f); \
                                        border: 1px solid var(--fx-color-border, #434343); \
                                        border-radius: 6px; padding: 4px; min-width: 140px; \
                                        box-shadow: 0 4px 12px rgba(0,0,0,0.4);">
                                {ColorPalette::all().iter().map(|p| {
                                    let p_copy = *p;
                                    let is_selected = current_palette == p_copy;
                                    view! {
                                        <button
                                            style=format!(
                                                "display: flex; align-items: center; gap: 8px; width: 100%; \
                                                 background: {}; border: none; border-radius: 4px; \
                                                 padding: 8px; color: var(--fx-color-text, #fff); \
                                                 font-size: 12px; cursor: pointer; text-align: left;",
                                                if is_selected { "rgba(255,255,255,0.1)" } else { "transparent" }
                                            )
                                            on:click=move |_: web_sys::MouseEvent| {
                                                selected_palette.set(p_copy);
                                                palette_open.set(false);
                                            }
                                        >
                                            <span style=format!(
                                                "width: 12px; height: 12px; border-radius: 3px; background: {};",
                                                p_copy.country_fill()
                                            )></span>
                                            {p_copy.label()}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        })}
                    </div>
                })
            }}

            // Flow toggle and Mode toggle (top-right)
            <div style="position: absolute; top: 16px; right: 16px; display: flex; gap: 8px; z-index: 50;">
                // Flow toggle
                {move || {
                    if !show_flow_toggle || !show_data_flow { return None; }

                    let flow_on = show_flow.get();
                    Some(view! {
                        <button
                            style=format!(
                                "display: flex; align-items: center; gap: 6px; \
                                 background: {}; \
                                 border: 1px solid var(--fx-color-border, #434343); \
                                 border-radius: 6px; padding: 8px 12px; \
                                 color: var(--fx-color-text, #fff); font-size: 12px; \
                                 cursor: pointer;",
                                if flow_on { "rgba(34, 197, 94, 0.2)" } else { "var(--fx-color-bg-elevated, #1f1f1f)" }
                            )
                            on:click=toggle_flow
                        >
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M22 12h-4l-3 9L9 3l-3 9H2"/>
                            </svg>
                            {if flow_on { "Flow On" } else { "Flow Off" }}
                        </button>
                    })
                }}

                // Mode toggle
                {move || {
                    if !show_mode_toggle || !enable_drag { return None; }

                    let mode = cursor_mode.get();
                    Some(view! {
                        <button
                            style=format!(
                                "display: flex; align-items: center; gap: 6px; \
                                 background: {}; \
                                 border: 1px solid var(--fx-color-border, #434343); \
                                 border-radius: 6px; padding: 8px 12px; \
                                 color: var(--fx-color-text, #fff); font-size: 12px; \
                                 cursor: pointer;",
                                if mode == DotCursorMode::Drag { "rgba(99, 102, 241, 0.2)" } else { "var(--fx-color-bg-elevated, #1f1f1f)" }
                            )
                            on:click=toggle_mode
                        >
                            {match mode {
                                DotCursorMode::Explore => view! {
                                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/>
                                    </svg>
                                }.into_any(),
                                DotCursorMode::Drag => view! {
                                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M5 9l-3 3 3 3M9 5l3-3 3 3M15 19l-3 3-3-3M19 9l3 3-3 3M2 12h20M12 2v20"/>
                                    </svg>
                                }.into_any(),
                            }}
                            {match mode {
                                DotCursorMode::Explore => "Explore",
                                DotCursorMode::Drag => "Pan",
                            }}
                        </button>
                    })
                }}
            </div>

            // Zoom controls (bottom-right)
            {move || {
                if !enable_zoom || !show_zoom_controls { return None; }

                let current_zoom = zoom_level.get();
                let can_reset = current_zoom != 1.0 || pan_offset.get() != (0.0, 0.0);

                Some(view! {
                    <div style="position: absolute; bottom: 16px; right: 16px; \
                                display: flex; flex-direction: column; gap: 4px;">
                        <button
                            style="width: 32px; height: 32px; \
                                   background: var(--fx-color-bg-elevated, #1f1f1f); \
                                   border: 1px solid var(--fx-color-border, #434343); \
                                   border-radius: 6px; color: var(--fx-color-text, #fff); \
                                   font-size: 18px; font-weight: 600; cursor: pointer; \
                                   display: flex; align-items: center; justify-content: center;"
                            disabled=move || zoom_level.get() >= max_zoom
                            on:click=zoom_in
                        >
                            {"+"}
                        </button>

                        <div style="width: 32px; height: 24px; \
                                    background: var(--fx-color-bg-elevated, #1f1f1f); \
                                    border: 1px solid var(--fx-color-border, #434343); \
                                    border-radius: 4px; color: var(--fx-color-text-secondary, #888); \
                                    font-size: 10px; display: flex; align-items: center; justify-content: center;">
                            {format!("{:.0}%", current_zoom * 100.0)}
                        </div>

                        <button
                            style="width: 32px; height: 32px; \
                                   background: var(--fx-color-bg-elevated, #1f1f1f); \
                                   border: 1px solid var(--fx-color-border, #434343); \
                                   border-radius: 6px; color: var(--fx-color-text, #fff); \
                                   font-size: 18px; font-weight: 600; cursor: pointer; \
                                   display: flex; align-items: center; justify-content: center;"
                            disabled=move || zoom_level.get() <= min_zoom
                            on:click=zoom_out
                        >
                            {"-"}
                        </button>

                        {can_reset.then(|| view! {
                            <button
                                style="width: 32px; height: 32px; \
                                       background: var(--fx-color-bg-elevated, #1f1f1f); \
                                       border: 1px solid var(--fx-color-border, #434343); \
                                       border-radius: 6px; color: var(--fx-color-text, #fff); \
                                       font-size: 9px; cursor: pointer; margin-top: 4px; \
                                       display: flex; align-items: center; justify-content: center;"
                                on:click=zoom_reset
                            >
                                {"Reset"}
                            </button>
                        })}
                    </div>
                })
            }}

            // Legend (bottom-left)
            {move || {
                if !show_legend || !show_network_nodes { return None; }

                let palette = selected_palette.get();
                let (fed_color, fleet_color) = palette.node_colors();
                let counts = country_counts.get();
                let total_fed: usize = counts.values().map(|(f, _, _)| f).sum();
                let total_fleet: usize = counts.values().map(|(_, f, _)| f).sum();

                Some(view! {
                    <div style="position: absolute; bottom: 16px; left: 16px; \
                                background: var(--fx-color-bg-elevated, #1f1f1f); \
                                border: 1px solid var(--fx-color-border, #434343); \
                                border-radius: 6px; padding: 12px;">
                        <div style="font-size: 11px; color: var(--fx-color-text-secondary, #888); margin-bottom: 8px;">
                            {"Network Nodes"}
                        </div>
                        <div style="display: flex; flex-direction: column; gap: 6px;">
                            <div style="display: flex; align-items: center; gap: 8px;">
                                // Rounded rectangle for Federation
                                <span style=format!(
                                    "width: 14px; height: 10px; border-radius: 3px; background: {};",
                                    fed_color
                                )></span>
                                <span style="font-size: 12px; color: var(--fx-color-text, #fff);">
                                    {format!("Federation ({})", total_fed)}
                                </span>
                            </div>
                            <div style="display: flex; align-items: center; gap: 8px;">
                                // Circle for Fleet
                                <span style=format!(
                                    "width: 8px; height: 8px; border-radius: 50%; background: {}; margin-left: 3px;",
                                    fleet_color
                                )></span>
                                <span style="font-size: 12px; color: var(--fx-color-text, #fff);">
                                    {format!("Fleet ({})", total_fleet)}
                                </span>
                            </div>
                        </div>
                    </div>
                })
            }}

            // Tooltip
            {move || {
                tooltip_content.get().map(|content| {
                    let (tx, ty) = tooltip_pos.get();
                    view! {
                        <div style=format!(
                            "position: fixed; left: {}px; top: {}px; \
                             background: var(--fx-color-bg-elevated, #1f1f1f); \
                             border: 1px solid var(--fx-color-border, #434343); \
                             border-radius: 6px; padding: 10px 14px; \
                             pointer-events: none; z-index: 1000; \
                             box-shadow: 0 4px 12px rgba(0,0,0,0.4); \
                             min-width: 140px;",
                            tx, ty
                        )>
                            {match content {
                                DotTooltipData::Node { name, node_type, country } => view! {
                                    <div>
                                        <div style="font-size: 13px; font-weight: 600; color: var(--fx-color-text, #fff); margin-bottom: 4px;">
                                            {name}
                                        </div>
                                        <div style="font-size: 11px; color: var(--fx-color-text-secondary, #888);">
                                            {format!("{} • {}", node_type.label(), country)}
                                        </div>
                                    </div>
                                }.into_any(),
                                DotTooltipData::Country { name, federation, fleet } => view! {
                                    <div>
                                        <div style="font-size: 13px; font-weight: 600; color: var(--fx-color-text, #fff); margin-bottom: 4px;">
                                            {name}
                                        </div>
                                        <div style="font-size: 11px; color: var(--fx-color-text-secondary, #888);">
                                            {format!("{} Federation, {} Fleet", federation, fleet)}
                                        </div>
                                        <div style="font-size: 12px; font-weight: 600; color: var(--fx-color-primary, #1890ff); margin-top: 4px;">
                                            {format!("{} Total Nodes", federation + fleet)}
                                        </div>
                                    </div>
                                }.into_any(),
                            }}
                        </div>
                    }
                })
            }}
        </div>
    }
}

/// Standalone variant display component.
#[component]
pub fn DotWorldMapVariant(
    state: DotWorldMapState,
    #[prop(optional, default = 400)]
    width: u32,
    #[prop(optional, default = 225)]
    height: u32,
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-dot-world-map-variant-{}", design_system);
    let path_data = get_path_data();

    let fill_color = match state {
        DotWorldMapState::Neutral => NEUTRAL_COLOR,
        DotWorldMapState::Hover => HOVER_COLOR,
        DotWorldMapState::Active => ACTIVE_COLOR,
    };

    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    view! {
        <div
            class=combined_class
            style=format!(
                "position: relative; width: {}px; height: {}px; \
                 background: var(--fx-color-bg, #141414); border-radius: 8px; overflow: hidden;",
                width, height
            )
        >
            <svg
                viewBox=VIEWBOX
                style="width: 100%; height: 100%;"
                preserveAspectRatio="xMidYMid meet"
            >
                <path
                    d=path_data
                    fill=fill_color
                />
            </svg>

            <div style="position: absolute; bottom: 8px; left: 8px; \
                        background: rgba(0,0,0,0.6); \
                        border-radius: 4px; padding: 4px 8px;">
                <span style=format!(
                    "font-size: 11px; font-weight: 500; color: {};",
                    fill_color
                )>
                    {format!("{:?}", state)}
                </span>
            </div>
        </div>
    }
}
