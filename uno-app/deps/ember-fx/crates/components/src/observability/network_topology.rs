//! NetworkTopologyMap component for P2P network visualization.
//!
//! Provides an interactive SVG-based visualization of network nodes
//! and their connections with support for multiple layout algorithms.

use std::collections::{HashMap, HashSet};
use leptos::prelude::*;
use leptos::html::Div;
use crate::try_use_theme;
use super::topology_types::*;

/// NetworkTopologyMap component.
///
/// An interactive network topology visualization showing nodes and their
/// connections with status-based coloring and click/hover interactions.
///
/// # Example
///
/// ```ignore
/// let nodes = vec![
///     TopologyNode::new("local", "Local Node", TopologyNodeType::LocalNode)
///         .position(400.0, 300.0),
///     TopologyNode::new("peer-1", "Peer 1", TopologyNodeType::Peer),
/// ];
///
/// let edges = vec![
///     TopologyEdge::new("local", "peer-1", TopologyEdgeType::Direct),
/// ];
///
/// view! {
///     <NetworkTopologyMap
///         nodes=Signal::derive(move || nodes.clone())
///         edges=Signal::derive(move || edges.clone())
///         on_node_click=|id| log!("Clicked: {}", id)
///     />
/// }
/// ```
#[component]
pub fn NetworkTopologyMap(
    /// Network nodes to display.
    nodes: Signal<Vec<TopologyNode>>,
    /// Connections between nodes.
    edges: Signal<Vec<TopologyEdge>>,
    /// Optional configuration.
    #[prop(optional)]
    config: Option<TopologyConfig>,
    /// Callback when a node is clicked.
    #[prop(optional, into)]
    on_node_click: Option<Callback<String>>,
    /// Callback when an edge is clicked.
    #[prop(optional, into)]
    on_edge_click: Option<Callback<(String, String)>>,
    /// Currently selected node ID (empty string means none selected).
    #[prop(optional, into)]
    selected_node: String,
    /// Loading state.
    #[prop(optional)]
    loading: bool,
    /// Whether to show data flow animation (demo mode).
    #[prop(optional, into)]
    show_data_flow: Signal<bool>,
    /// Enabled traffic types for data flow (if None, all enabled).
    #[prop(optional)]
    enabled_traffic_types: Option<RwSignal<HashSet<TrafficType>>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-topology-{}", design_system);
    let config = config.unwrap_or_default();

    // Extract config values before moving config into closure
    let width = config.width;
    let height = config.height;
    let node_radius = config.node_radius;
    let edge_width = config.edge_width;
    let show_labels = config.show_labels;
    let show_legend = config.show_legend;
    let highlight_local = config.highlight_local;
    let layout = config.layout;

    // Local state
    let hovered_node = RwSignal::new(None::<String>);
    let tooltip_data = RwSignal::new(None::<(String, f64, f64, TopologyNode)>);

    // Drag state
    let container_ref = NodeRef::<Div>::new();
    let dragging_node = RwSignal::new(None::<String>);
    let drag_offset = RwSignal::new((0.0, 0.0));
    let dragged_positions = RwSignal::new(HashMap::<String, (f64, f64)>::new());

    // Data flow animation state
    let demo_particles = RwSignal::new(Vec::<DataFlowParticle>::new());
    let particle_id_counter = RwSignal::new(0u32);

    // Store config for positions calculation
    let config_for_layout = config.clone();

    // Calculate positions based on layout - use Memo for caching
    let layout_positions = Memo::new(move |_| {
        let node_list = nodes.get();
        let edge_list = edges.get();

        match layout {
            TopologyLayout::Circular => calculate_circular_layout(&node_list, &config_for_layout),
            TopologyLayout::ForceDirected => {
                calculate_force_layout(&node_list, &edge_list, &config_for_layout, 50)
            }
            _ => calculate_circular_layout(&node_list, &config_for_layout),
        }
    });

    // Merge layout positions with user-dragged positions
    let positions = Memo::new(move |_| {
        let mut result = layout_positions.get();
        let dragged = dragged_positions.get();

        for (id, (x, y)) in dragged {
            if let Some(pos) = result.get_mut(&id) {
                pos.x = x;
                pos.y = y;
            }
        }
        result
    });

    // Coordinate conversion helper (client to SVG coordinates)
    let client_to_svg = move |client_x: f64, client_y: f64| -> Option<(f64, f64)> {
        container_ref.get().and_then(|container| {
            let rect = container.get_bounding_client_rect();
            if rect.width() > 0.0 && rect.height() > 0.0 {
                let scale_x = width as f64 / rect.width();
                let scale_y = height as f64 / rect.height();
                let svg_x = (client_x - rect.left()) * scale_x;
                let svg_y = (client_y - rect.top()) * scale_y;
                Some((svg_x, svg_y))
            } else {
                None
            }
        })
    };

    // Data flow animation effect (demo mode)
    let edges_for_anim = edges.clone();
    Effect::new(move |_| {
        if !show_data_flow.get() {
            demo_particles.set(Vec::new());
            return;
        }

        let handle = set_interval_with_handle(
            move || {
                // Update existing particles
                demo_particles.update(|particles| {
                    particles.retain_mut(|p| {
                        p.progress += p.speed;
                        p.progress < 1.0
                    });
                });

                // Spawn new particles randomly on edges
                let edge_list = edges_for_anim.get();
                if edge_list.is_empty() {
                    return;
                }

                // Random chance to spawn (roughly 30% per tick)
                let spawn_roll = particle_id_counter.get_untracked() % 10;
                if spawn_roll < 3 {
                    let edge_idx = (particle_id_counter.get_untracked() as usize) % edge_list.len();
                    let edge = &edge_list[edge_idx];
                    let traffic_types = TrafficType::all();
                    let traffic_idx = (particle_id_counter.get_untracked() as usize) % traffic_types.len();
                    let traffic = traffic_types[traffic_idx];

                    let id = particle_id_counter.get_untracked();
                    particle_id_counter.set(id.wrapping_add(1));

                    demo_particles.update(|p| {
                        // Limit total particles for performance
                        if p.len() < 50 {
                            p.push(DataFlowParticle::new(id, edge.source.clone(), edge.target.clone(), traffic));
                        }
                    });
                } else {
                    // Still increment counter for randomness
                    particle_id_counter.update(|c| *c = c.wrapping_add(1));
                }
            },
            std::time::Duration::from_millis(50), // 20 FPS
        );

        on_cleanup(move || {
            if let Ok(h) = handle {
                h.clear();
            }
        });
    });

    let prefix_container = prefix.clone();
    let prefix_svg = prefix.clone();
    let prefix_edge = prefix.clone();
    let prefix_node = prefix.clone();
    let _prefix_tooltip = prefix.clone();
    let prefix_legend = prefix.clone();
    let prefix_loading = prefix.clone();

    view! {
        <div
            node_ref=container_ref
            class=format!("{} {}", prefix_container, class.clone().unwrap_or_default())
            style="position: relative; overflow: hidden; width: 100%; height: 100%;"
        >
            <svg
                class=format!("{}-svg", prefix_svg)
                viewBox=format!("0 0 {} {}", width, height)
                preserveAspectRatio="xMidYMid meet"
                style="width: 100%; height: 100%; display: block;"
                on:mousemove=move |ev: web_sys::MouseEvent| {
                    if let Some(node_id) = dragging_node.get_untracked() {
                        if let Some((svg_x, svg_y)) = client_to_svg(ev.client_x() as f64, ev.client_y() as f64) {
                            let (ox, oy) = drag_offset.get_untracked();
                            let margin = node_radius * 2.0;
                            let new_x = (svg_x + ox).clamp(margin, width as f64 - margin);
                            let new_y = (svg_y + oy).clamp(margin, height as f64 - margin);
                            dragged_positions.update(|map| {
                                map.insert(node_id.clone(), (new_x, new_y));
                            });
                        }
                    }
                }
                on:mouseup=move |_| {
                    dragging_node.set(None);
                }
                on:mouseleave=move |_| {
                    dragging_node.set(None);
                }
            >
                // Defs for markers and gradients
                <defs>
                    <marker
                        id="arrow-direct"
                        viewBox="0 0 10 10"
                        refX="9"
                        refY="5"
                        markerWidth="6"
                        markerHeight="6"
                        orient="auto-start-reverse"
                    >
                        <path d="M 0 0 L 10 5 L 0 10 z" fill="#888888" />
                    </marker>
                    <marker
                        id="arrow-relayed"
                        viewBox="0 0 10 10"
                        refX="9"
                        refY="5"
                        markerWidth="6"
                        markerHeight="6"
                        orient="auto-start-reverse"
                    >
                        <path d="M 0 0 L 10 5 L 0 10 z" fill="#faad14" />
                    </marker>
                    <marker
                        id="arrow-federation"
                        viewBox="0 0 10 10"
                        refX="9"
                        refY="5"
                        markerWidth="6"
                        markerHeight="6"
                        orient="auto-start-reverse"
                    >
                        <path d="M 0 0 L 10 5 L 0 10 z" fill="#1890ff" />
                    </marker>
                    <marker
                        id="arrow-discovery"
                        viewBox="0 0 10 10"
                        refX="9"
                        refY="5"
                        markerWidth="6"
                        markerHeight="6"
                        orient="auto-start-reverse"
                    >
                        <path d="M 0 0 L 10 5 L 0 10 z" fill="#13c2c2" />
                    </marker>
                    <marker
                        id="arrow-cluster"
                        viewBox="0 0 10 10"
                        refX="9"
                        refY="5"
                        markerWidth="6"
                        markerHeight="6"
                        orient="auto-start-reverse"
                    >
                        <path d="M 0 0 L 10 5 L 0 10 z" fill="#52c41a" />
                    </marker>
                </defs>

                // Render edges
                {move || {
                    let pos = positions.get();
                    let edge_list = edges.get();
                    let prefix = prefix_edge.clone();

                    edge_list.into_iter().filter_map(|edge| {
                        let src_pos = pos.get(&edge.source)?;
                        let tgt_pos = pos.get(&edge.target)?;

                        // Use explicit hex colors for SVG compatibility
                        let stroke_color = match edge.edge_type {
                            TopologyEdgeType::Direct => "#888888",      // gray
                            TopologyEdgeType::Relayed => "#faad14",     // warning/orange
                            TopologyEdgeType::Federation => "#1890ff",  // primary/blue
                            TopologyEdgeType::Discovery => "#13c2c2",   // info/cyan
                            TopologyEdgeType::Cluster => "#52c41a",     // success/green
                        };

                        let dash_array = edge.edge_type.as_dash_array();
                        let marker_end = match edge.edge_type {
                            TopologyEdgeType::Direct => "url(#arrow-direct)",
                            TopologyEdgeType::Relayed => "url(#arrow-relayed)",
                            TopologyEdgeType::Federation => "url(#arrow-federation)",
                            TopologyEdgeType::Discovery => "url(#arrow-discovery)",
                            TopologyEdgeType::Cluster => "url(#arrow-cluster)",
                        };

                        let edge_class = format!("{}-edge {}-edge-{}", prefix, prefix, edge.type_class());
                        let source = edge.source.clone();
                        let target = edge.target.clone();

                        Some(view! {
                            <line
                                class=edge_class
                                x1=src_pos.x
                                y1=src_pos.y
                                x2=tgt_pos.x
                                y2=tgt_pos.y
                                stroke=stroke_color
                                stroke-width=edge_width
                                stroke-dasharray=dash_array
                                marker-end=marker_end
                                style="cursor: pointer; transition: stroke-width 0.15s ease;"
                                on:click={
                                    let src = source.clone();
                                    let tgt = target.clone();
                                    let callback = on_edge_click.clone();
                                    move |_| {
                                        if let Some(ref cb) = callback {
                                            cb.run((src.clone(), tgt.clone()));
                                        }
                                    }
                                }
                            />
                        })
                    }).collect_view()
                }}

                // Render data flow particles
                {move || {
                    let pos = positions.get();
                    let particles = demo_particles.get();

                    if !show_data_flow.get() || particles.is_empty() {
                        return None::<Vec<_>>.into_iter().flatten().collect_view();
                    }

                    let enabled = enabled_traffic_types
                        .map(|s| s.get())
                        .unwrap_or_else(|| TrafficType::all().iter().copied().collect());

                    particles.into_iter().filter_map(|particle| {
                        // Filter by enabled traffic type
                        if !enabled.contains(&particle.traffic_type) {
                            return None;
                        }

                        let source_pos = pos.get(&particle.source)?;
                        let target_pos = pos.get(&particle.target)?;

                        // Linear interpolation
                        let x = source_pos.x + (target_pos.x - source_pos.x) * particle.progress;
                        let y = source_pos.y + (target_pos.y - source_pos.y) * particle.progress;

                        // Fade out as particle approaches destination
                        let opacity = if particle.progress > 0.8 {
                            1.0 - (particle.progress - 0.8) * 5.0
                        } else {
                            1.0
                        };

                        let color = particle.traffic_type.as_color();

                        Some(view! {
                            <circle
                                cx=x
                                cy=y
                                r=4.0
                                fill=color
                                opacity=opacity
                                style=format!(
                                    "pointer-events: none; filter: drop-shadow(0 0 4px {});",
                                    color
                                )
                            />
                        })
                    }).collect_view()
                }}

                // Render nodes
                {move || {
                    let pos = positions.get();
                    let node_list = nodes.get();
                    let prefix = prefix_node.clone();
                    let selected = selected_node.clone();

                    node_list.into_iter().filter_map(|node| {
                        let node_pos = pos.get(&node.id)?;
                        let node_id = node.id.clone();
                        let node_id_click = node_id.clone();
                        let node_id_hover = node_id.clone();
                        let node_id_drag = node_id.clone();
                        let node_clone = node.clone();

                        let is_local = node.node_type == TopologyNodeType::LocalNode;
                        let is_selected = !selected.is_empty() && selected == node_id;
                        let is_hovered = move || hovered_node.get().as_ref() == Some(&node_id);

                        let radius = if is_local && highlight_local {
                            node_radius * 1.5
                        } else {
                            node_radius
                        };

                        // Use explicit hex colors for SVG compatibility
                        let fill_color = match node.status {
                            TopologyNodeStatus::Healthy => "#52c41a",      // green
                            TopologyNodeStatus::Degraded => "#faad14",     // orange/warning
                            TopologyNodeStatus::Unhealthy => "#ff4d4f",    // red/error
                            TopologyNodeStatus::Connecting => "#1890ff",   // blue/info
                            TopologyNodeStatus::Disconnected => "#8c8c8c", // gray/muted
                            TopologyNodeStatus::Unknown => "#8c8c8c",      // gray/muted
                        };

                        let stroke_color = if is_local {
                            "#1890ff"  // primary blue
                        } else {
                            "#434343"  // border gray
                        };

                        let stroke_width = if is_selected {
                            3.0
                        } else if is_local {
                            2.5
                        } else {
                            1.5
                        };

                        let node_class = format!(
                            "{}-node {}-node-{} {}-node-{}",
                            prefix, prefix, node.type_class(), prefix, node.status_color()
                        );

                        let x = node_pos.x;
                        let y = node_pos.y;
                        let label = node.label.clone();

                        Some(view! {
                            <g class=node_class>
                                // Node circle
                                <circle
                                    cx=x
                                    cy=y
                                    r=move || if is_hovered() { radius * 1.15 } else { radius }
                                    fill=fill_color
                                    stroke=stroke_color
                                    stroke-width=stroke_width
                                    style=move || {
                                        let is_dragging = dragging_node.get().is_some();
                                        let cursor = if is_dragging { "grabbing" } else { "grab" };
                                        format!("cursor: {}; transition: r 0.15s ease, stroke-width 0.15s ease;", cursor)
                                    }
                                    on:mousedown={
                                        let node_id = node_id_drag.clone();
                                        move |ev: web_sys::MouseEvent| {
                                            ev.prevent_default();
                                            dragging_node.set(Some(node_id.clone()));
                                            // Calculate offset from mouse to node center
                                            if let Some((svg_x, svg_y)) = client_to_svg(ev.client_x() as f64, ev.client_y() as f64) {
                                                drag_offset.set((x - svg_x, y - svg_y));
                                            }
                                            // Clear tooltip while dragging
                                            tooltip_data.set(None);
                                        }
                                    }
                                    on:mouseenter={
                                        let node_id = node_id_hover.clone();
                                        let node = node_clone.clone();
                                        move |_| {
                                            // Don't show tooltip while dragging
                                            if dragging_node.get_untracked().is_none() {
                                                hovered_node.set(Some(node_id.clone()));
                                                tooltip_data.set(Some((node_id.clone(), x, y, node.clone())));
                                            }
                                        }
                                    }
                                    on:mouseleave=move |_| {
                                        if dragging_node.get_untracked().is_none() {
                                            hovered_node.set(None);
                                            tooltip_data.set(None);
                                        }
                                    }
                                    on:click={
                                        let id = node_id_click.clone();
                                        let callback = on_node_click.clone();
                                        move |_| {
                                            // Only trigger click if not dragging
                                            if dragging_node.get_untracked().is_none() {
                                                if let Some(ref cb) = callback {
                                                    cb.run(id.clone());
                                                }
                                            }
                                        }
                                    }
                                />

                                // Node label
                                {show_labels.then(|| view! {
                                    <text
                                        x=x
                                        y=y + radius + 15.0
                                        text-anchor="middle"
                                        class=format!("{}-label", prefix)
                                        style="font-size: 12px; fill: #bfbfbf; pointer-events: none;"
                                    >
                                        {label}
                                    </text>
                                })}

                                // Node type icon (for local node)
                                {is_local.then(|| view! {
                                    <text
                                        x=x
                                        y=y + 5.0
                                        text-anchor="middle"
                                        style="font-size: 16px; fill: white; pointer-events: none; font-weight: bold;"
                                    >
                                        "★"
                                    </text>
                                })}
                            </g>
                        })
                    }).collect_view()
                }}
                // Tooltip inside SVG using foreignObject for proper scaling
                // Always render a single foreignObject, control visibility via style
                <foreignObject
                    x=move || {
                        tooltip_data.get().map(|(_, x, _y, node)| {
                            let tooltip_width = 140.0;
                            let margin = 10.0;
                            let node_r = node_radius * if node.node_type == TopologyNodeType::LocalNode { 1.5 } else { 1.0 };
                            let tooltip_x = if x + node_r + margin + tooltip_width < width as f64 {
                                x + node_r + margin
                            } else if x - node_r - margin - tooltip_width > 0.0 {
                                x - node_r - margin - tooltip_width
                            } else {
                                x - tooltip_width / 2.0
                            };
                            tooltip_x.max(5.0).min(width as f64 - tooltip_width - 5.0)
                        }).unwrap_or(-200.0)
                    }
                    y=move || {
                        tooltip_data.get().map(|(_, x, y, node)| {
                            let tooltip_width = 140.0;
                            let tooltip_height = 90.0;
                            let margin = 10.0;
                            let node_r = node_radius * if node.node_type == TopologyNodeType::LocalNode { 1.5 } else { 1.0 };
                            let tooltip_y = if x + node_r + margin + tooltip_width < width as f64 {
                                y - tooltip_height / 2.0
                            } else if x - node_r - margin - tooltip_width > 0.0 {
                                y - tooltip_height / 2.0
                            } else if y + node_r + margin + tooltip_height < height as f64 {
                                y + node_r + margin
                            } else {
                                y - node_r - margin - tooltip_height
                            };
                            tooltip_y.max(5.0).min(height as f64 - tooltip_height - 5.0)
                        }).unwrap_or(-200.0)
                    }
                    width=140.0
                    height=90.0
                    style=move || {
                        let visible = tooltip_data.get().is_some();
                        format!(
                            "overflow: visible; pointer-events: none; opacity: {}; transition: opacity 0.1s;",
                            if visible { "1" } else { "0" }
                        )
                    }
                >
                    {move || {
                        tooltip_data.get().map(|(id, _, _, node)| {
                            view! {
                                <div
                                    style="background: #262626; border: 1px solid #434343; \
                                           border-radius: 6px; padding: 8px 10px; \
                                           box-shadow: 0 4px 12px rgba(0,0,0,0.3); \
                                           color: #ffffff; font-size: 11px;"
                                >
                                    <div style="font-weight: 600; margin-bottom: 4px; font-size: 12px;">
                                        {node.label.clone()}
                                    </div>
                                    <div style="color: #bfbfbf;">
                                        <div>"Type: "{node.node_type.as_label()}</div>
                                        <div>"Status: "{node.status.as_label()}</div>
                                        <div style="font-family: monospace; font-size: 9px; margin-top: 4px; word-break: break-all; opacity: 0.7;">
                                            {if id.len() > 12 { format!("{}...", &id[..10]) } else { id }}
                                        </div>
                                    </div>
                                </div>
                            }
                        })
                    }}
                </foreignObject>
            </svg>

            // Legend
            {show_legend.then(|| {
                let prefix = prefix_legend.clone();
                view! {
                    <div
                        class=format!("{}-legend", prefix)
                        style="position: absolute; bottom: 10px; left: 10px; \
                               background: #262626; \
                               border: 1px solid #434343; \
                               border-radius: 6px; padding: 8px 12px; \
                               font-size: 11px; color: #ffffff;"
                    >
                        <div style="font-weight: 600; margin-bottom: 6px;">"Legend"</div>
                        // Status colors
                        <div style="display: flex; gap: 12px; flex-wrap: wrap;">
                            <span style="display: flex; align-items: center; gap: 4px;">
                                <span style="width: 10px; height: 10px; border-radius: 50%; background: #52c41a;"></span>
                                "Healthy"
                            </span>
                            <span style="display: flex; align-items: center; gap: 4px;">
                                <span style="width: 10px; height: 10px; border-radius: 50%; background: #faad14;"></span>
                                "Degraded"
                            </span>
                            <span style="display: flex; align-items: center; gap: 4px;">
                                <span style="width: 10px; height: 10px; border-radius: 50%; background: #ff4d4f;"></span>
                                "Unhealthy"
                            </span>
                            <span style="display: flex; align-items: center; gap: 4px;">
                                <span style="width: 10px; height: 10px; border-radius: 50%; background: #8c8c8c;"></span>
                                "Disconnected"
                            </span>
                        </div>
                        // Edge types
                        <div style="margin-top: 6px; display: flex; gap: 12px; flex-wrap: wrap;">
                            <span style="display: flex; align-items: center; gap: 4px;">
                                <span style="width: 20px; height: 2px; background: #888888;"></span>
                                "Direct"
                            </span>
                            <span style="display: flex; align-items: center; gap: 4px;">
                                <span style="width: 20px; height: 2px; background: #faad14; border-style: dashed;"></span>
                                "Relayed"
                            </span>
                        </div>

                        // Traffic type toggles (only shown when data flow is enabled)
                        {move || show_data_flow.get().then(|| {
                            view! {
                                <div style="margin-top: 8px; border-top: 1px solid #434343; padding-top: 8px;">
                                    <div style="font-weight: 600; margin-bottom: 4px;">"Data Flow"</div>
                                    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                                        {TrafficType::all().iter().map(|&tt| {
                                            let is_enabled = move || {
                                                enabled_traffic_types
                                                    .map(|s| s.get().contains(&tt))
                                                    .unwrap_or(true)
                                            };
                                            let color = tt.as_color();
                                            let label = tt.as_label();
                                            view! {
                                                <label style="display: flex; align-items: center; gap: 4px; cursor: pointer;">
                                                    <input
                                                        type="checkbox"
                                                        prop:checked=is_enabled
                                                        on:change=move |_| {
                                                            if let Some(enabled) = enabled_traffic_types {
                                                                enabled.update(|set| {
                                                                    if set.contains(&tt) {
                                                                        set.remove(&tt);
                                                                    } else {
                                                                        set.insert(tt);
                                                                    }
                                                                });
                                                            }
                                                        }
                                                        style="accent-color: currentColor;"
                                                    />
                                                    <span style=format!("display: flex; align-items: center; gap: 4px; color: {};", color)>
                                                        <span style=format!(
                                                            "width: 8px; height: 8px; border-radius: 50%; background: {}; box-shadow: 0 0 4px {};",
                                                            color, color
                                                        )></span>
                                                        {label}
                                                    </span>
                                                </label>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                }
            })}

            // Loading overlay
            {loading.then(|| {
                let prefix = prefix_loading.clone();
                view! {
                    <div
                        class=format!("{}-loading", prefix)
                        style="position: absolute; top: 0; left: 0; right: 0; bottom: 0; \
                               display: flex; align-items: center; justify-content: center; \
                               background: rgba(0,0,0,0.5); border-radius: 8px;"
                    >
                        <div style="background: #262626; padding: 16px 24px; \
                                    border-radius: 8px; font-weight: 500; color: #ffffff;">
                            "Loading topology..."
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
