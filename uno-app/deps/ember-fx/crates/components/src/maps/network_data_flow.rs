//! NetworkDataFlow component for network visualization.
//!
//! Displays an interactive world map with Federation and Fleet nodes,
//! animated data flow particles, and various controls.

use leptos::prelude::*;
use std::time::Duration;
use crate::try_use_theme;
use super::svg_country_data::COUNTRIES;
use super::network_data_flow_types::{
    ColorPalette, CursorMode, NetworkNode, NetworkNodeType,
    NetworkConnection, DataParticle, NetworkDataFlowConfig,
    generate_demo_nodes, generate_demo_connections,
};

/// NetworkDataFlow component.
///
/// Displays an interactive world map with network nodes representing
/// Federations and Fleets. Features include:
/// - Color palette selection for the globe
/// - Drag/pan mode for moving the map
/// - Zoom controls (+/-)
/// - Animated data flow particles between nodes
///
/// # Example
///
/// ```ignore
/// view! {
///     <NetworkDataFlow
///         config=NetworkDataFlowConfig {
///             width: 900,
///             height: 506,
///             show_zoom_controls: true,
///             show_palette_selector: true,
///             ..Default::default()
///         }
///     />
/// }
/// ```
#[component]
pub fn NetworkDataFlow(
    /// Configuration options.
    #[prop(optional)]
    config: Option<NetworkDataFlowConfig>,
    /// Custom nodes (overrides demo nodes).
    #[prop(optional)]
    nodes: Option<Signal<Vec<NetworkNode>>>,
    /// Custom connections (overrides demo connections).
    #[prop(optional)]
    connections: Option<Signal<Vec<NetworkConnection>>>,
    /// Callback when a node is clicked.
    #[prop(optional, into)]
    on_node_click: Option<Callback<NetworkNode>>,
    /// Callback when a node is hovered.
    #[prop(optional, into)]
    on_node_hover: Option<Callback<Option<String>>>,
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
    let prefix = format!("fx-netflow-{}", design_system);

    // Extract config values
    let width = config.width;
    let height = config.height;
    let viewbox_width = config.viewbox_width;
    let viewbox_height = config.viewbox_height;
    let show_zoom_controls = config.show_zoom_controls;
    let show_palette_selector = config.show_palette_selector;
    let show_mode_toggle = config.show_mode_toggle;
    let show_flow_toggle = config.show_flow_toggle;
    let initial_zoom = config.initial_zoom;
    let min_zoom = config.min_zoom;
    let max_zoom = config.max_zoom;
    let zoom_step = config.zoom_step;
    let particle_speed = config.particle_speed;
    let max_particles = config.max_particles;
    let show_flow_by_default = config.show_flow_by_default;
    let default_palette = config.default_palette;
    let default_cursor_mode = config.default_cursor_mode;

    // Generate demo data if not provided
    let demo_nodes = generate_demo_nodes();
    let demo_connections = generate_demo_connections(&demo_nodes);

    let internal_nodes = RwSignal::new(demo_nodes);
    let internal_connections = RwSignal::new(demo_connections);

    let nodes_signal = nodes.unwrap_or_else(|| Signal::derive(move || internal_nodes.get()));
    let connections_signal = connections.unwrap_or_else(|| Signal::derive(move || internal_connections.get()));

    // Reactive state
    let zoom_level = RwSignal::new(initial_zoom);
    let pan_offset = RwSignal::new((0.0_f64, 0.0_f64));
    let cursor_mode = RwSignal::new(default_cursor_mode);
    let selected_palette = RwSignal::new(default_palette);
    let palette_open = RwSignal::new(false);
    let hovered_node = RwSignal::new(None::<String>);
    let hovered_country = RwSignal::new(None::<String>);
    let tooltip_pos = RwSignal::new((0.0_f64, 0.0_f64));
    let tooltip_content = RwSignal::new(None::<(String, String, String)>);
    let show_flow = RwSignal::new(show_flow_by_default);
    let particles = RwSignal::new(Vec::<DataParticle>::new());
    let particle_counter = RwSignal::new(0u32);

    // Country zoom state
    let zoomed_country = RwSignal::new(None::<String>);
    let zoomed_country_name = RwSignal::new(None::<String>);

    // Drag state
    let is_dragging = RwSignal::new(false);
    let last_mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));

    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    // Zoom padding for country zoom
    let zoom_padding = 50.0_f64;

    // Compute viewBox based on zoom, pan, and country zoom
    let viewbox = move || {
        // If zoomed to a country, show that country's bounding box
        if let Some(code) = zoomed_country.get() {
            if let Some(country) = COUNTRIES.iter().find(|c| c.code == code) {
                let (min_x, min_y, bbox_width, bbox_height) = country.bbox;

                // Add padding
                let padded_x = (min_x - zoom_padding).max(0.0);
                let padded_y = (min_y - zoom_padding).max(0.0);
                let padded_w = bbox_width + zoom_padding * 2.0;
                let padded_h = bbox_height + zoom_padding * 2.0;

                // Maintain aspect ratio
                let target_aspect = viewbox_width as f64 / viewbox_height as f64;
                let bbox_aspect = padded_w / padded_h;

                let (final_x, final_y, final_w, final_h) = if bbox_aspect > target_aspect {
                    let new_h = padded_w / target_aspect;
                    let y_offset = (new_h - padded_h) / 2.0;
                    (padded_x, padded_y - y_offset, padded_w, new_h)
                } else {
                    let new_w = padded_h * target_aspect;
                    let x_offset = (new_w - padded_w) / 2.0;
                    (padded_x - x_offset, padded_y, new_w, padded_h)
                };

                return format!("{:.1} {:.1} {:.1} {:.1}", final_x, final_y, final_w, final_h);
            }
        }

        // Otherwise use zoom/pan
        let zoom = zoom_level.get();
        let (px, py) = pan_offset.get();
        let vw = viewbox_width as f64 / zoom;
        let vh = viewbox_height as f64 / zoom;
        format!("{:.1} {:.1} {:.1} {:.1}", px, py, vw, vh)
    };

    // Particle animation effect
    Effect::new(move |_| {
        if !show_flow.get() {
            particles.set(Vec::new());
            return;
        }

        let handle = set_interval_with_handle(
            move || {
                let conns = connections_signal.get();
                let current_nodes = nodes_signal.get();

                // Update existing particles
                particles.update(|p| {
                    p.retain_mut(|particle| {
                        particle.progress += particle.speed;
                        particle.progress < 1.0
                    });
                });

                // Spawn new particles randomly
                let current_count = particles.get().len();
                if current_count < max_particles && !conns.is_empty() {
                    // Simple deterministic random based on counter
                    let counter = particle_counter.get();
                    particle_counter.set(counter.wrapping_add(1));

                    // Spawn a new particle every few frames
                    if counter % 3 == 0 {
                        let conn_idx = (counter as usize) % conns.len();
                        let conn = &conns[conn_idx];

                        // Verify connection has valid nodes
                        let has_source = current_nodes.iter().any(|n| n.id == conn.source_id);
                        let has_target = current_nodes.iter().any(|n| n.id == conn.target_id);

                        if has_source && has_target {
                            particles.update(|p| {
                                p.push(DataParticle::new(
                                    counter,
                                    conn.source_id.clone(),
                                    conn.target_id.clone(),
                                    particle_speed,
                                ));
                            });
                        }
                    }
                }
            },
            Duration::from_millis(50),
        );

        on_cleanup(move || {
            if let Ok(h) = handle {
                h.clear();
            }
        });
    });

    // Mouse handlers for drag mode
    let on_mousedown = move |e: web_sys::MouseEvent| {
        if cursor_mode.get() == CursorMode::Drag {
            is_dragging.set(true);
            last_mouse_pos.set((e.client_x() as f64, e.client_y() as f64));
            e.prevent_default();
        }
    };

    let on_mousemove = move |e: web_sys::MouseEvent| {
        if is_dragging.get() && cursor_mode.get() == CursorMode::Drag {
            let (lx, ly) = last_mouse_pos.get();
            let zoom = zoom_level.get();
            // Scale movement by zoom level and map it to viewBox coordinates
            let scale = viewbox_width as f64 / (width as f64 * zoom);
            let dx = (e.client_x() as f64 - lx) * scale;
            let dy = (e.client_y() as f64 - ly) * scale;

            pan_offset.update(|(px, py)| {
                *px -= dx;
                *py -= dy;
            });
            last_mouse_pos.set((e.client_x() as f64, e.client_y() as f64));
        }
    };

    let on_mouseup = move |_: web_sys::MouseEvent| {
        is_dragging.set(false);
    };

    let on_mouseleave = move |_: web_sys::MouseEvent| {
        is_dragging.set(false);
    };

    // Zoom handlers
    let zoom_in = move |_| {
        zoom_level.update(|z| *z = (*z + zoom_step).min(max_zoom));
    };

    let zoom_out = move |_| {
        zoom_level.update(|z| *z = (*z - zoom_step).max(min_zoom));
    };

    // Toggle cursor mode
    let toggle_cursor_mode = move |_| {
        cursor_mode.update(|m| {
            *m = match m {
                CursorMode::Explore => CursorMode::Drag,
                CursorMode::Drag => CursorMode::Explore,
            };
        });
    };

    // Toggle data flow
    let toggle_flow = move |_| {
        show_flow.update(|v| *v = !*v);
    };

    view! {
        <div
            class=combined_class
            style=move || format!(
                "position: relative; width: {}px; height: {}px; background: {}; overflow: hidden; border-radius: 8px; user-select: none;",
                width, height, selected_palette.get().background_color()
            )
            on:mousedown=on_mousedown
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
            on:mouseleave=on_mouseleave
        >
            // Loading overlay
            {move || {
                loading.map(|l| l.get()).unwrap_or(false).then(|| view! {
                    <div style="position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.5); z-index: 100;">
                        <div style="color: var(--fx-color-text, #fff); font-size: 14px;">{"Loading..."}</div>
                    </div>
                })
            }}

            <svg
                viewBox=viewbox
                style=move || format!(
                    "width: 100%; height: 100%; cursor: {};",
                    if cursor_mode.get() == CursorMode::Drag {
                        if is_dragging.get() { "grabbing" } else { "grab" }
                    } else {
                        "default"
                    }
                )
                preserveAspectRatio="xMidYMid meet"
            >
                // Background
                <rect
                    x="0"
                    y="0"
                    width=viewbox_width
                    height=viewbox_height
                    fill=move || selected_palette.get().background_color()
                />

                // Countries - clickable with zoom
                <g class="countries">
                    {move || {
                        let palette = selected_palette.get();
                        let fill = palette.country_fill();
                        let stroke = palette.country_stroke();
                        let current_cursor = cursor_mode.get();
                        let current_hovered = hovered_country.get();
                        let current_zoomed = zoomed_country.get();

                        COUNTRIES.iter().map(|country| {
                            let code = country.code.to_string();
                            let name = country.name.to_string();
                            let code_for_hover = code.clone();
                            let code_for_click = code.clone();
                            let name_for_click = name.clone();
                            let is_hovered = current_hovered.as_ref() == Some(&code);
                            let is_zoomed = current_zoomed.as_ref() == Some(&code);

                            country.paths.iter().map(|path_d| {
                                let code_hover = code_for_hover.clone();
                                let code_click = code_for_click.clone();
                                let name_click = name_for_click.clone();

                                view! {
                                    <path
                                        d=*path_d
                                        fill=fill
                                        stroke=stroke
                                        stroke-width=if is_hovered || is_zoomed { "1.5" } else { "0.5" }
                                        opacity=if is_hovered || is_zoomed { "1" } else { "0.85" }
                                        style=move || format!(
                                            "cursor: {}; transition: opacity 0.2s, stroke-width 0.2s;",
                                            if current_cursor == CursorMode::Explore { "pointer" } else { "grab" }
                                        )
                                        on:mouseenter=move |_| {
                                            if cursor_mode.get() == CursorMode::Explore {
                                                hovered_country.set(Some(code_hover.clone()));
                                            }
                                        }
                                        on:mouseleave=move |_| {
                                            hovered_country.set(None);
                                        }
                                        on:click=move |e: web_sys::MouseEvent| {
                                            if cursor_mode.get() == CursorMode::Explore {
                                                // Toggle zoom
                                                if zoomed_country.get().as_ref() == Some(&code_click) {
                                                    zoomed_country.set(None);
                                                    zoomed_country_name.set(None);
                                                } else {
                                                    zoomed_country.set(Some(code_click.clone()));
                                                    zoomed_country_name.set(Some(name_click.clone()));
                                                }
                                                e.stop_propagation();
                                            }
                                        }
                                    />
                                }
                            }).collect_view()
                        }).collect_view()
                    }}
                </g>

                // Connection lines
                <g class="connections">
                    {move || {
                        let current_nodes = nodes_signal.get();
                        let conns = connections_signal.get();
                        let palette = selected_palette.get();
                        let conn_color = palette.connection_color();

                        conns.iter().filter_map(|conn| {
                            let source = current_nodes.iter().find(|n| n.id == conn.source_id)?;
                            let target = current_nodes.iter().find(|n| n.id == conn.target_id)?;

                            // Calculate control point for curved line
                            let mid_x = (source.x + target.x) / 2.0;
                            let mid_y = (source.y + target.y) / 2.0;
                            let dx = target.x - source.x;
                            let dy = target.y - source.y;
                            let dist = (dx * dx + dy * dy).sqrt();
                            // Perpendicular offset for curve
                            let offset = dist * 0.2;
                            let ctrl_x = mid_x - dy * offset / dist;
                            let ctrl_y = mid_y + dx * offset / dist;

                            let path_d = format!(
                                "M {} {} Q {} {} {} {}",
                                source.x, source.y,
                                ctrl_x, ctrl_y,
                                target.x, target.y
                            );

                            Some(view! {
                                <path
                                    d=path_d
                                    fill="none"
                                    stroke=conn_color
                                    stroke-width="1.5"
                                    opacity="0.4"
                                    stroke-linecap="round"
                                />
                            })
                        }).collect_view()
                    }}
                </g>

                // Animated particles
                <g class="particles">
                    {move || {
                        let current_particles = particles.get();
                        let current_nodes = nodes_signal.get();
                        let palette = selected_palette.get();
                        let particle_color = palette.particle_color();

                        current_particles.iter().filter_map(|particle| {
                            let source = current_nodes.iter().find(|n| n.id == particle.source_id)?;
                            let target = current_nodes.iter().find(|n| n.id == particle.target_id)?;

                            // Calculate position along curved path
                            let t = particle.progress;
                            let mid_x = (source.x + target.x) / 2.0;
                            let mid_y = (source.y + target.y) / 2.0;
                            let dx = target.x - source.x;
                            let dy = target.y - source.y;
                            let dist = (dx * dx + dy * dy).sqrt();
                            let offset = dist * 0.2;
                            let ctrl_x = mid_x - dy * offset / dist.max(1.0);
                            let ctrl_y = mid_y + dx * offset / dist.max(1.0);

                            // Quadratic bezier interpolation
                            let t1 = 1.0 - t;
                            let px = t1 * t1 * source.x + 2.0 * t1 * t * ctrl_x + t * t * target.x;
                            let py = t1 * t1 * source.y + 2.0 * t1 * t * ctrl_y + t * t * target.y;

                            Some(view! {
                                <circle
                                    cx=px
                                    cy=py
                                    r="4"
                                    fill=particle_color
                                    opacity="0.9"
                                >
                                    <animate
                                        attributeName="r"
                                        values="3;5;3"
                                        dur="0.5s"
                                        repeatCount="indefinite"
                                    />
                                </circle>
                            })
                        }).collect_view()
                    }}
                </g>

                // Network nodes
                <g class="nodes">
                    {move || {
                        let current_nodes = nodes_signal.get();
                        let palette = selected_palette.get();
                        let (fed_color, fleet_color) = palette.node_colors();
                        let hovered = hovered_node.get();
                        let on_node_click = on_node_click.clone();
                        let on_node_hover = on_node_hover.clone();
                        let cursor = cursor_mode.get();

                        current_nodes.iter().map(|node| {
                            let node_id = node.id.clone();
                            let node_name = node.name.clone();
                            let node_region = node.region;
                            let node_type = node.node_type;
                            let node_x = node.x;
                            let node_y = node.y;
                            let node_clone = node.clone();
                            let on_node_click = on_node_click.clone();
                            let on_node_hover = on_node_hover.clone();

                            let color = match node_type {
                                NetworkNodeType::Federation => fed_color,
                                NetworkNodeType::Fleet => fleet_color,
                            };
                            let radius = node_type.radius();
                            let is_hovered = hovered.as_ref() == Some(&node_id);
                            let display_radius = if is_hovered { radius * 1.3 } else { radius };

                            view! {
                                <g
                                    style=move || format!(
                                        "cursor: {}; transition: transform 0.2s;",
                                        if cursor == CursorMode::Explore { "pointer" } else { "grab" }
                                    )
                                    on:mouseenter={
                                        let node_id = node_id.clone();
                                        let node_name = node_name.clone();
                                        let node_region = node_region;
                                        let on_node_hover = on_node_hover.clone();
                                        move |e: web_sys::MouseEvent| {
                                            if cursor_mode.get() == CursorMode::Explore {
                                                hovered_node.set(Some(node_id.clone()));
                                                if let Some(ref cb) = on_node_hover {
                                                    cb.run(Some(node_id.clone()));
                                                }
                                                tooltip_content.set(Some((
                                                    node_name.clone(),
                                                    node_type.label().to_string(),
                                                    node_region.to_string(),
                                                )));
                                                tooltip_pos.set((
                                                    e.client_x() as f64 + 15.0,
                                                    e.client_y() as f64 + 15.0,
                                                ));
                                            }
                                        }
                                    }
                                    on:mousemove={
                                        move |e: web_sys::MouseEvent| {
                                            if hovered_node.get().is_some() {
                                                tooltip_pos.set((
                                                    e.client_x() as f64 + 15.0,
                                                    e.client_y() as f64 + 15.0,
                                                ));
                                            }
                                        }
                                    }
                                    on:mouseleave={
                                        let on_node_hover = on_node_hover.clone();
                                        move |_| {
                                            hovered_node.set(None);
                                            tooltip_content.set(None);
                                            if let Some(ref cb) = on_node_hover {
                                                cb.run(None);
                                            }
                                        }
                                    }
                                    on:click={
                                        let node_clone = node_clone.clone();
                                        let on_node_click = on_node_click.clone();
                                        move |e: web_sys::MouseEvent| {
                                            if cursor_mode.get() == CursorMode::Explore {
                                                if let Some(ref cb) = on_node_click {
                                                    cb.run(node_clone.clone());
                                                }
                                                e.stop_propagation();
                                            }
                                        }
                                    }
                                >
                                    // Outer glow
                                    <circle
                                        cx=node_x
                                        cy=node_y
                                        r=display_radius + 4.0
                                        fill=color
                                        opacity="0.3"
                                    />
                                    // Main node
                                    <circle
                                        cx=node_x
                                        cy=node_y
                                        r=display_radius
                                        fill=color
                                        stroke="#ffffff"
                                        stroke-width="2"
                                        opacity="1"
                                    />
                                    // Inner highlight
                                    <circle
                                        cx=node_x - display_radius * 0.3
                                        cy=node_y - display_radius * 0.3
                                        r=display_radius * 0.3
                                        fill="#ffffff"
                                        opacity="0.4"
                                    />
                                </g>
                            }
                        }).collect_view()
                    }}
                </g>
            </svg>

            // Color palette selector (top-left)
            {move || show_palette_selector.then(|| {
                let palette = selected_palette.get();
                let is_open = palette_open.get();

                view! {
                    <div style="position: absolute; top: 16px; left: 16px; z-index: 50;">
                        // Selected palette button
                        <button
                            style=format!(
                                "background: var(--fx-color-bg-elevated, #1f1f1f); \
                                 border: 1px solid var(--fx-color-border, #434343); \
                                 border-radius: 8px; padding: 8px 12px; \
                                 color: var(--fx-color-text, #fff); \
                                 font-size: 12px; cursor: pointer; \
                                 display: flex; align-items: center; gap: 8px; \
                                 transition: background 0.2s; min-width: 100px;"
                            )
                            on:click=move |_| palette_open.update(|v| *v = !*v)
                        >
                            // Color swatch
                            <span style=format!(
                                "width: 16px; height: 16px; border-radius: 4px; background: {}; border: 1px solid rgba(255,255,255,0.2);",
                                palette.country_fill()
                            ) />
                            {palette.label()}
                            // Dropdown arrow
                            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-left: auto;">
                                <path d="M6 9l6 6 6-6"/>
                            </svg>
                        </button>

                        // Dropdown menu
                        {is_open.then(|| view! {
                            <div style="position: absolute; top: 100%; left: 0; margin-top: 4px; \
                                        background: var(--fx-color-bg-elevated, #1f1f1f); \
                                        border: 1px solid var(--fx-color-border, #434343); \
                                        border-radius: 8px; overflow: hidden; \
                                        box-shadow: 0 4px 12px rgba(0,0,0,0.4); min-width: 120px;">
                                {ColorPalette::all().iter().map(|p| {
                                    let p = *p;
                                    let is_selected = palette == p;
                                    view! {
                                        <button
                                            style=format!(
                                                "width: 100%; padding: 8px 12px; border: none; \
                                                 background: {}; color: var(--fx-color-text, #fff); \
                                                 font-size: 12px; cursor: pointer; \
                                                 display: flex; align-items: center; gap: 8px; \
                                                 text-align: left; transition: background 0.2s;",
                                                if is_selected { "rgba(255,255,255,0.1)" } else { "transparent" }
                                            )
                                            on:click=move |_| {
                                                selected_palette.set(p);
                                                palette_open.set(false);
                                            }
                                        >
                                            <span style=format!(
                                                "width: 14px; height: 14px; border-radius: 4px; background: {}; border: 1px solid rgba(255,255,255,0.2);",
                                                p.country_fill()
                                            ) />
                                            {p.label()}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        })}
                    </div>
                }
            })}

            // Cursor mode toggle and flow toggle (top-right)
            <div style="position: absolute; top: 16px; right: 16px; display: flex; gap: 8px; z-index: 50;">
                // Data flow toggle
                {move || show_flow_toggle.then(|| {
                    let is_on = show_flow.get();
                    view! {
                        <button
                            style=format!(
                                "background: {}; \
                                 border: 1px solid var(--fx-color-border, #434343); \
                                 border-radius: 8px; padding: 8px 12px; \
                                 color: var(--fx-color-text, #fff); \
                                 font-size: 12px; cursor: pointer; \
                                 display: flex; align-items: center; gap: 6px; \
                                 transition: background 0.2s;",
                                if is_on { "rgba(34, 197, 94, 0.2)" } else { "var(--fx-color-bg-elevated, #1f1f1f)" }
                            )
                            on:click=toggle_flow
                        >
                            // Flow icon
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M22 12h-4l-3 9L9 3l-3 9H2"/>
                            </svg>
                            {if is_on { "Flow On" } else { "Flow Off" }}
                        </button>
                    }
                })}

                // Cursor mode toggle
                {move || show_mode_toggle.then(|| {
                    let mode = cursor_mode.get();
                    view! {
                        <button
                            style="background: var(--fx-color-bg-elevated, #1f1f1f); \
                                   border: 1px solid var(--fx-color-border, #434343); \
                                   border-radius: 8px; padding: 8px 12px; \
                                   color: var(--fx-color-text, #fff); \
                                   font-size: 12px; cursor: pointer; \
                                   display: flex; align-items: center; gap: 6px; \
                                   transition: background 0.2s;"
                            on:click=toggle_cursor_mode
                        >
                            // Mode icon
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d=mode.icon() />
                            </svg>
                            {mode.label()}
                        </button>
                    }
                })}
            </div>

            // Zoom controls (bottom-right)
            {move || show_zoom_controls.then(|| {
                view! {
                    <div style="position: absolute; bottom: 16px; right: 16px; \
                                display: flex; flex-direction: column; gap: 4px; z-index: 50;">
                        // Zoom in
                        <button
                            style="background: var(--fx-color-bg-elevated, #1f1f1f); \
                                   border: 1px solid var(--fx-color-border, #434343); \
                                   border-radius: 6px; width: 32px; height: 32px; \
                                   color: var(--fx-color-text, #fff); \
                                   font-size: 18px; cursor: pointer; \
                                   display: flex; align-items: center; justify-content: center; \
                                   transition: background 0.2s;"
                            on:click=zoom_in
                        >
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 5v14M5 12h14"/>
                            </svg>
                        </button>
                        // Zoom level indicator
                        <div style="background: var(--fx-color-bg-elevated, #1f1f1f); \
                                    border: 1px solid var(--fx-color-border, #434343); \
                                    border-radius: 6px; padding: 4px 8px; \
                                    color: var(--fx-color-text-secondary, #888); \
                                    font-size: 10px; text-align: center;">
                            {move || format!("{:.0}%", zoom_level.get() * 100.0)}
                        </div>
                        // Zoom out
                        <button
                            style="background: var(--fx-color-bg-elevated, #1f1f1f); \
                                   border: 1px solid var(--fx-color-border, #434343); \
                                   border-radius: 6px; width: 32px; height: 32px; \
                                   color: var(--fx-color-text, #fff); \
                                   font-size: 18px; cursor: pointer; \
                                   display: flex; align-items: center; justify-content: center; \
                                   transition: background 0.2s;"
                            on:click=zoom_out
                        >
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M5 12h14"/>
                            </svg>
                        </button>
                    </div>
                }
            })}

            // Back to World button (when zoomed to country)
            {move || {
                zoomed_country.get().map(|_| {
                    let country_name = zoomed_country_name.get().unwrap_or_default();
                    view! {
                        <div style="position: absolute; top: 16px; left: 50%; transform: translateX(-50%); \
                                    display: flex; flex-direction: column; align-items: center; gap: 8px; z-index: 60;">
                            <button
                                style="background: var(--fx-color-bg-elevated, #1f1f1f); \
                                       border: 1px solid var(--fx-color-border, #434343); \
                                       border-radius: 6px; padding: 8px 16px; \
                                       color: var(--fx-color-text, #fff); \
                                       font-size: 13px; cursor: pointer; \
                                       display: flex; align-items: center; gap: 6px; \
                                       transition: background 0.2s;"
                                on:click=move |_| {
                                    zoomed_country.set(None);
                                    zoomed_country_name.set(None);
                                }
                            >
                                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M19 12H5M12 19l-7-7 7-7"/>
                                </svg>
                                {"Back to World"}
                            </button>
                            <div style="background: var(--fx-color-primary, #1890ff); \
                                        border-radius: 6px; padding: 6px 12px; \
                                        color: #fff; font-size: 12px; font-weight: 600;">
                                {country_name}
                            </div>
                        </div>
                    }
                })
            }}

            // Node tooltip
            {move || {
                tooltip_content.get().map(|(name, node_type, region)| {
                    let (tx, ty) = tooltip_pos.get();
                    // Clamp tooltip position to stay within viewport
                    let clamped_y = if ty < 10.0 { 10.0 } else { ty };
                    view! {
                        <div style=format!(
                            "position: fixed; left: {}px; top: {}px; \
                             background: var(--fx-color-bg-elevated, #1f1f1f); \
                             border: 1px solid var(--fx-color-border, #434343); \
                             border-radius: 6px; padding: 10px 14px; \
                             pointer-events: none; z-index: 200; \
                             box-shadow: 0 4px 12px rgba(0,0,0,0.4); \
                             min-width: 140px;",
                            tx, clamped_y
                        )>
                            <div style="font-size: 13px; font-weight: 600; color: var(--fx-color-text, #fff); margin-bottom: 4px;">
                                {name}
                            </div>
                            <div style="font-size: 11px; color: var(--fx-color-text-secondary, #888); margin-bottom: 2px;">
                                {node_type}
                            </div>
                            <div style="font-size: 11px; color: var(--fx-color-primary, #1890ff);">
                                {region}
                            </div>
                        </div>
                    }
                })
            }}

            // Legend (bottom-left)
            <div style="position: absolute; bottom: 16px; left: 16px; \
                        background: var(--fx-color-bg-elevated, #1f1f1f); \
                        border: 1px solid var(--fx-color-border, #434343); \
                        border-radius: 6px; padding: 10px 14px; z-index: 50;">
                <div style="font-size: 10px; color: var(--fx-color-text-secondary, #888); margin-bottom: 8px; text-transform: uppercase; letter-spacing: 0.5px;">
                    {"Legend"}
                </div>
                <div style="display: flex; flex-direction: column; gap: 6px;">
                    // Federation
                    {move || {
                        let palette = selected_palette.get();
                        let (fed_color, fleet_color) = palette.node_colors();
                        view! {
                            <>
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <span style=format!(
                                        "width: 12px; height: 12px; border-radius: 50%; background: {}; border: 1px solid #fff;",
                                        fed_color
                                    ) />
                                    <span style="font-size: 11px; color: var(--fx-color-text, #fff);">{"Federation"}</span>
                                </div>
                                // Fleet
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <span style=format!(
                                        "width: 10px; height: 10px; border-radius: 50%; background: {}; border: 1px solid #fff;",
                                        fleet_color
                                    ) />
                                    <span style="font-size: 11px; color: var(--fx-color-text, #fff);">{"Fleet"}</span>
                                </div>
                            </>
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
