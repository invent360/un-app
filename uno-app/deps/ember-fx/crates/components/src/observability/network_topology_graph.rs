//! NetworkTopologyGraph Leptos component.
//!
//! A visual network topology graph showing node connections.

use leptos::prelude::*;
use crate::try_use_theme;

/// Node type in the graph topology.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GraphNodeType {
    /// Current/self node.
    #[default]
    Self_,
    /// Bootstrap node.
    Bootstrap,
    /// Relay node.
    Relay,
    /// Storage node.
    Storage,
    /// Compute node.
    Compute,
    /// Regular peer.
    Peer,
}

impl GraphNodeType {
    /// Returns the color.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Self_ => "var(--fx-color-primary, #1890ff)",
            Self::Bootstrap => "var(--fx-color-purple, #722ed1)",
            Self::Relay => "var(--fx-color-cyan, #13c2c2)",
            Self::Storage => "var(--fx-color-orange, #fa8c16)",
            Self::Compute => "var(--fx-color-green, #52c41a)",
            Self::Peer => "var(--fx-color-text-secondary, #8c8c8c)",
        }
    }

    /// Returns the icon.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::Self_ => "●",
            Self::Bootstrap => "◆",
            Self::Relay => "◈",
            Self::Storage => "▣",
            Self::Compute => "⬡",
            Self::Peer => "○",
        }
    }

    /// Returns the label.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Self_ => "You",
            Self::Bootstrap => "Bootstrap",
            Self::Relay => "Relay",
            Self::Storage => "Storage",
            Self::Compute => "Compute",
            Self::Peer => "Peer",
        }
    }
}

/// A node in the graph topology.
#[derive(Debug, Clone)]
pub struct GraphNode {
    /// Node ID.
    pub id: String,
    /// Node label (optional).
    pub label: Option<String>,
    /// Node type.
    pub node_type: GraphNodeType,
    /// Latency to this node in ms.
    pub latency_ms: Option<u32>,
    /// Is connected directly.
    pub is_direct: bool,
    /// Connection quality (0-100).
    pub quality: Option<f64>,
}

impl GraphNode {
    /// Create a new graph node.
    pub fn new(id: impl Into<String>, node_type: GraphNodeType) -> Self {
        Self {
            id: id.into(),
            label: None,
            node_type,
            latency_ms: None,
            is_direct: true,
            quality: None,
        }
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set latency.
    pub fn latency(mut self, ms: u32) -> Self {
        self.latency_ms = Some(ms);
        self
    }

    /// Set direct connection flag.
    pub fn direct(mut self, is_direct: bool) -> Self {
        self.is_direct = is_direct;
        self
    }

    /// Set connection quality.
    pub fn quality(mut self, q: f64) -> Self {
        self.quality = Some(q);
        self
    }
}

/// NetworkTopologyGraph component.
///
/// Displays a radial network topology visualization.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{NetworkTopologyGraph, GraphNode, GraphNodeType};
///
/// let nodes = vec![
///     GraphNode::new("boot1", GraphNodeType::Bootstrap).latency(25),
///     GraphNode::new("relay1", GraphNodeType::Relay).latency(45),
///     GraphNode::new("peer1", GraphNodeType::Peer).latency(80),
/// ];
///
/// view! {
///     <NetworkTopologyGraph
///         nodes=Signal::derive(move || nodes.clone())
///     />
/// }
/// ```
#[component]
pub fn NetworkTopologyGraph(
    /// Connected nodes.
    #[prop(into)]
    nodes: Signal<Vec<GraphNode>>,
    /// Show legend.
    #[prop(optional)]
    show_legend: Option<bool>,
    /// Show node labels.
    #[prop(optional)]
    show_labels: Option<bool>,
    /// Graph title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Graph size (px).
    #[prop(optional)]
    size: Option<u32>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let title = title.unwrap_or_else(|| "Network Topology".to_string());
    let show_legend = show_legend.unwrap_or(true);
    let show_labels = show_labels.unwrap_or(false);
    let size = size.unwrap_or(300);

    let prefix = format!("fx-topology-{}", design_system);

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![prefix.clone()];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // SVG parameters
    let view_size = size as f64;
    let center = view_size / 2.0;
    let inner_radius = 20.0; // Self node
    let outer_radius = (view_size / 2.0) - 30.0;

    // Calculate node positions (radial layout)
    let node_positions = move || {
        let n = nodes.get();
        let count = n.len();
        if count == 0 {
            return vec![];
        }

        n.iter()
            .enumerate()
            .map(|(i, node)| {
                let angle = (2.0 * std::f64::consts::PI * i as f64 / count as f64) - std::f64::consts::FRAC_PI_2;

                // Distance based on latency (closer = lower latency)
                let distance = if let Some(lat) = node.latency_ms {
                    let normalized = (lat as f64 / 200.0).min(1.0);
                    inner_radius + 20.0 + (normalized * (outer_radius - inner_radius - 20.0))
                } else {
                    (inner_radius + outer_radius) / 2.0
                };

                let x = center + distance * angle.cos();
                let y = center + distance * angle.sin();

                (node.clone(), x, y)
            })
            .collect::<Vec<_>>()
    };

    // Line color based on quality
    let line_color = |quality: Option<f64>| -> &'static str {
        match quality {
            Some(q) if q >= 80.0 => "var(--fx-color-success, #52c41a)",
            Some(q) if q >= 50.0 => "var(--fx-color-warning, #faad14)",
            Some(_) => "var(--fx-color-error, #ff4d4f)",
            None => "var(--fx-color-border, #303030)",
        }
    };

    view! {
        <div class=combined_class>
            // Header
            <div class=format!("{}-header", prefix)>
                <span class=format!("{}-title", prefix)>{title}</span>
                <span class=format!("{}-count", prefix)>
                    {move || format!("{} nodes", nodes.get().len())}
                </span>
            </div>

            // Graph container
            <div
                class=format!("{}-container", prefix)
                style=format!("width: {}px; height: {}px;", size, size)
            >
                <svg
                    viewBox=format!("0 0 {} {}", view_size, view_size)
                    class=format!("{}-svg", prefix)
                >
                    // Concentric circles (distance guides)
                    <circle
                        cx=center
                        cy=center
                        r=inner_radius + 40.0
                        fill="none"
                        stroke="var(--fx-color-border, #303030)"
                        stroke-width="0.5"
                        stroke-dasharray="4 4"
                    />
                    <circle
                        cx=center
                        cy=center
                        r=inner_radius + 80.0
                        fill="none"
                        stroke="var(--fx-color-border, #303030)"
                        stroke-width="0.5"
                        stroke-dasharray="4 4"
                    />

                    // Connection lines
                    {move || {
                        node_positions().into_iter().map(|(node, x, y)| {
                            let color = line_color(node.quality);
                            let stroke_width = if node.is_direct { "1.5" } else { "0.75" };
                            let dash = if node.is_direct { "" } else { "4 2" };
                            view! {
                                <line
                                    x1=center
                                    y1=center
                                    x2=x
                                    y2=y
                                    stroke=color
                                    stroke-width=stroke_width
                                    stroke-dasharray=dash
                                    opacity="0.6"
                                />
                            }
                        }).collect_view()
                    }}

                    // Self node (center)
                    <circle
                        cx=center
                        cy=center
                        r=inner_radius
                        fill=GraphNodeType::Self_.as_color()
                        stroke="var(--fx-color-bg, #141414)"
                        stroke-width="3"
                    />
                    <text
                        x=center
                        y=center
                        text-anchor="middle"
                        dominant-baseline="middle"
                        fill="white"
                        font-size="10"
                    >
                        "You"
                    </text>

                    // Peer nodes
                    {move || {
                        node_positions().into_iter().map(|(node, x, y)| {
                            let node_id = node.id.clone();
                            let node_label = node.label.clone().unwrap_or_else(|| {
                                if node_id.len() > 6 {
                                    format!("{}...", &node_id[..6])
                                } else {
                                    node_id.clone()
                                }
                            });

                            view! {
                                <g>
                                    <circle
                                        cx=x
                                        cy=y
                                        r="12"
                                        fill=node.node_type.as_color()
                                        stroke="var(--fx-color-bg, #141414)"
                                        stroke-width="2"
                                    >
                                        <title>
                                            {format!(
                                                "{} ({}) - {}ms",
                                                node_label,
                                                node.node_type.as_label(),
                                                node.latency_ms.unwrap_or(0)
                                            )}
                                        </title>
                                    </circle>
                                    {if show_labels {
                                        Some(view! {
                                            <text
                                                x=x
                                                y=y + 22.0
                                                text-anchor="middle"
                                                fill="var(--fx-color-text-secondary, #8c8c8c)"
                                                font-size="8"
                                            >
                                                {node_label}
                                            </text>
                                        })
                                    } else {
                                        None
                                    }}
                                </g>
                            }
                        }).collect_view()
                    }}
                </svg>
            </div>

            // Legend
            {if show_legend {
                Some(view! {
                    <div class=format!("{}-legend", prefix)>
                        {[
                            GraphNodeType::Bootstrap,
                            GraphNodeType::Relay,
                            GraphNodeType::Storage,
                            GraphNodeType::Compute,
                            GraphNodeType::Peer,
                        ].into_iter().map(|nt| {
                            view! {
                                <div class=format!("{}-legend-item", prefix)>
                                    <span
                                        class=format!("{}-legend-dot", prefix)
                                        style=format!("background: {};", nt.as_color())
                                    />
                                    <span class=format!("{}-legend-label", prefix)>
                                        {nt.as_label()}
                                    </span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}
