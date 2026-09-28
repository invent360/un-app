//! Types for Network Topology visualization.
//!
//! Provides data structures for representing network nodes, edges,
//! and configuration for the NetworkTopologyMap component.

use std::collections::HashMap;

/// Node in the topology graph.
#[derive(Debug, Clone, PartialEq)]
pub struct TopologyNode {
    /// Unique node identifier (peer ID).
    pub id: String,
    /// Display label for the node.
    pub label: String,
    /// Type of node in the network.
    pub node_type: TopologyNodeType,
    /// Current health/connection status.
    pub status: TopologyNodeStatus,
    /// Optional fixed position (x, y). If None, layout algorithm positions it.
    pub position: Option<(f64, f64)>,
    /// Additional metadata as key-value pairs.
    pub metadata: Vec<(String, String)>,
}

impl TopologyNode {
    /// Create a new topology node.
    pub fn new(id: impl Into<String>, label: impl Into<String>, node_type: TopologyNodeType) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            node_type,
            status: TopologyNodeStatus::Healthy,
            position: None,
            metadata: Vec::new(),
        }
    }

    /// Set the node status.
    pub fn status(mut self, status: TopologyNodeStatus) -> Self {
        self.status = status;
        self
    }

    /// Set a fixed position for the node.
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Some((x, y));
        self
    }

    /// Add metadata to the node.
    pub fn meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push((key.into(), value.into()));
        self
    }

    /// Get color class based on status.
    pub fn status_color(&self) -> &'static str {
        self.status.as_color()
    }

    /// Get CSS class suffix for node type.
    pub fn type_class(&self) -> &'static str {
        self.node_type.as_class()
    }
}

/// Type of node in the network topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TopologyNodeType {
    /// The local node (this instance).
    LocalNode,
    /// A connected peer node.
    #[default]
    Peer,
    /// A relay server for NAT traversal.
    Relay,
    /// A bootstrap node.
    Bootstrap,
    /// A federation hub node.
    Federation,
    /// A fleet coordinator node.
    Fleet,
    /// A service provider node.
    Service,
}

impl TopologyNodeType {
    /// Get display label for the node type.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::LocalNode => "Local",
            Self::Peer => "Peer",
            Self::Relay => "Relay",
            Self::Bootstrap => "Bootstrap",
            Self::Federation => "Federation",
            Self::Fleet => "Fleet",
            Self::Service => "Service",
        }
    }

    /// Get CSS class suffix for styling.
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::LocalNode => "local",
            Self::Peer => "peer",
            Self::Relay => "relay",
            Self::Bootstrap => "bootstrap",
            Self::Federation => "federation",
            Self::Fleet => "fleet",
            Self::Service => "service",
        }
    }

    /// Get icon character for the node type.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::LocalNode => "●",
            Self::Peer => "○",
            Self::Relay => "◇",
            Self::Bootstrap => "◆",
            Self::Federation => "★",
            Self::Fleet => "☆",
            Self::Service => "□",
        }
    }
}

/// Health/connection status of a topology node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TopologyNodeStatus {
    /// Node is healthy and connected.
    #[default]
    Healthy,
    /// Node is experiencing issues but still connected.
    Degraded,
    /// Node is unhealthy or failing.
    Unhealthy,
    /// Node is in the process of connecting.
    Connecting,
    /// Node is disconnected or unreachable.
    Disconnected,
    /// Node status is unknown.
    Unknown,
}

impl TopologyNodeStatus {
    /// Get display label for the status.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Healthy => "Healthy",
            Self::Degraded => "Degraded",
            Self::Unhealthy => "Unhealthy",
            Self::Connecting => "Connecting",
            Self::Disconnected => "Disconnected",
            Self::Unknown => "Unknown",
        }
    }

    /// Get CSS color class suffix.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Healthy => "success",
            Self::Degraded => "warning",
            Self::Unhealthy => "error",
            Self::Connecting => "info",
            Self::Disconnected => "muted",
            Self::Unknown => "muted",
        }
    }
}

/// Edge/connection between nodes in the topology.
#[derive(Debug, Clone, PartialEq)]
pub struct TopologyEdge {
    /// Source node ID.
    pub source: String,
    /// Target node ID.
    pub target: String,
    /// Type of connection.
    pub edge_type: TopologyEdgeType,
    /// Weight for layout algorithm (e.g., bandwidth, inverse latency).
    pub weight: f64,
    /// Optional label to display on the edge.
    pub label: Option<String>,
}

impl TopologyEdge {
    /// Create a new topology edge.
    pub fn new(source: impl Into<String>, target: impl Into<String>, edge_type: TopologyEdgeType) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            edge_type,
            weight: 1.0,
            label: None,
        }
    }

    /// Set the edge weight.
    pub fn weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }

    /// Set a label for the edge.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get CSS class suffix for edge type.
    pub fn type_class(&self) -> &'static str {
        self.edge_type.as_class()
    }
}

/// Type of connection/edge in the topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TopologyEdgeType {
    /// Direct TCP/QUIC connection.
    #[default]
    Direct,
    /// Connection via relay (circuit relay v2).
    Relayed,
    /// Cross-federation link.
    Federation,
    /// Discovery link (DHT, mDNS, etc.).
    Discovery,
    /// Internal cluster link.
    Cluster,
}

impl TopologyEdgeType {
    /// Get display label for the edge type.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Direct => "Direct",
            Self::Relayed => "Relayed",
            Self::Federation => "Federation",
            Self::Discovery => "Discovery",
            Self::Cluster => "Cluster",
        }
    }

    /// Get CSS class suffix for styling.
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Relayed => "relayed",
            Self::Federation => "federation",
            Self::Discovery => "discovery",
            Self::Cluster => "cluster",
        }
    }

    /// Get stroke dash array for SVG.
    pub fn as_dash_array(&self) -> &'static str {
        match self {
            Self::Direct => "",            // Solid
            Self::Relayed => "5,5",        // Dashed
            Self::Federation => "10,5",    // Long dash
            Self::Discovery => "2,3",      // Dotted
            Self::Cluster => "8,3,2,3",    // Dash-dot
        }
    }
}

/// Configuration for the NetworkTopologyMap component.
#[derive(Debug, Clone)]
pub struct TopologyConfig {
    /// Width of the visualization area.
    pub width: u32,
    /// Height of the visualization area.
    pub height: u32,
    /// Base radius for node circles.
    pub node_radius: f64,
    /// Whether to show node labels.
    pub show_labels: bool,
    /// Whether to show the legend.
    pub show_legend: bool,
    /// Whether to animate layout changes.
    pub animate: bool,
    /// Layout algorithm to use.
    pub layout: TopologyLayout,
    /// Whether to highlight the local node.
    pub highlight_local: bool,
    /// Edge stroke width.
    pub edge_width: f64,
    /// Minimum distance between nodes.
    pub min_distance: f64,
}

impl Default for TopologyConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            node_radius: 20.0,
            show_labels: true,
            show_legend: true,
            animate: true,
            layout: TopologyLayout::ForceDirected,
            highlight_local: true,
            edge_width: 2.0,
            min_distance: 80.0,
        }
    }
}

/// Layout algorithm for positioning nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TopologyLayout {
    /// Force-directed layout with spring physics.
    #[default]
    ForceDirected,
    /// Hierarchical/tree layout.
    Hierarchical,
    /// Circular layout around center.
    Circular,
    /// Grid-based layout.
    Grid,
}

impl TopologyLayout {
    /// Get display label for the layout.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::ForceDirected => "Force-Directed",
            Self::Hierarchical => "Hierarchical",
            Self::Circular => "Circular",
            Self::Grid => "Grid",
        }
    }
}

/// Calculated position for a node after layout.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
    pub vx: f64,  // velocity x (for force-directed)
    pub vy: f64,  // velocity y
}

impl NodePosition {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, vx: 0.0, vy: 0.0 }
    }
}

/// Simple force-directed layout calculator.
pub fn calculate_force_layout(
    nodes: &[TopologyNode],
    edges: &[TopologyEdge],
    config: &TopologyConfig,
    iterations: usize,
) -> HashMap<String, NodePosition> {
    let mut positions: HashMap<String, NodePosition> = HashMap::new();

    // Define usable area with margins
    let margin = config.node_radius * 3.0;
    let usable_width = config.width as f64 - margin * 2.0;
    let usable_height = config.height as f64 - margin * 2.0;
    let center_x = config.width as f64 / 2.0;
    let center_y = config.height as f64 / 2.0;

    // Find local node to position it at center
    let local_node_id = nodes.iter()
        .find(|n| n.node_type == TopologyNodeType::LocalNode)
        .map(|n| n.id.clone());

    // Initialize positions - local node at center, others in circle around it
    let initial_radius = usable_width.min(usable_height) / 4.0;
    let non_local_count = nodes.iter().filter(|n| n.node_type != TopologyNodeType::LocalNode).count();
    let mut non_local_idx = 0;

    for node in nodes.iter() {
        let pos = if node.node_type == TopologyNodeType::LocalNode {
            // Local node always at center
            NodePosition::new(center_x, center_y)
        } else if let Some((x, y)) = node.position {
            // Use fixed position if provided, but normalize to our coordinate space
            NodePosition::new(x, y)
        } else {
            // Arrange other nodes in a circle around center
            let angle = (non_local_idx as f64) * 2.0 * std::f64::consts::PI / (non_local_count.max(1) as f64);
            non_local_idx += 1;
            NodePosition::new(
                center_x + initial_radius * angle.cos(),
                center_y + initial_radius * angle.sin(),
            )
        };
        positions.insert(node.id.clone(), pos);
    }

    // Force-directed iterations
    let repulsion = 8000.0;
    let attraction = 0.02;
    let damping = 0.85;
    let center_gravity = 0.005;  // Pull toward center
    let min_dist = config.min_distance;

    for _ in 0..iterations {
        let current_positions: Vec<_> = positions.iter()
            .map(|(id, pos)| (id.clone(), *pos))
            .collect();

        for (id, mut pos) in current_positions {
            // Keep local node fixed at center
            if Some(&id) == local_node_id.as_ref() {
                continue;
            }

            // Skip other fixed position nodes
            if nodes.iter().find(|n| n.id == id).map_or(false, |n| n.position.is_some()) {
                continue;
            }

            let mut fx = 0.0;
            let mut fy = 0.0;

            // Repulsion from all other nodes
            for (other_id, other_pos) in positions.iter() {
                if *other_id == id {
                    continue;
                }

                let dx = pos.x - other_pos.x;
                let dy = pos.y - other_pos.y;
                let dist = (dx * dx + dy * dy).sqrt().max(1.0);

                // Always apply some repulsion, stronger when closer
                let force = repulsion / (dist * dist);
                fx += (dx / dist) * force;
                fy += (dy / dist) * force;
            }

            // Attraction along edges
            for edge in edges {
                let other_id = if edge.source == id {
                    &edge.target
                } else if edge.target == id {
                    &edge.source
                } else {
                    continue;
                };

                if let Some(other_pos) = positions.get(other_id) {
                    let dx = other_pos.x - pos.x;
                    let dy = other_pos.y - pos.y;
                    let dist = (dx * dx + dy * dy).sqrt().max(1.0);

                    // Only attract if beyond minimum distance
                    if dist > min_dist {
                        let force = attraction * (dist - min_dist) * edge.weight;
                        fx += (dx / dist) * force;
                        fy += (dy / dist) * force;
                    }
                }
            }

            // Center gravity - pull toward center
            let dx = center_x - pos.x;
            let dy = center_y - pos.y;
            fx += dx * center_gravity;
            fy += dy * center_gravity;

            // Update velocity and position
            pos.vx = (pos.vx + fx) * damping;
            pos.vy = (pos.vy + fy) * damping;
            pos.x += pos.vx;
            pos.y += pos.vy;

            // Constrain to bounds with margin
            pos.x = pos.x.clamp(margin, config.width as f64 - margin);
            pos.y = pos.y.clamp(margin, config.height as f64 - margin);

            positions.insert(id, pos);
        }
    }

    positions
}

/// Calculate circular layout.
pub fn calculate_circular_layout(
    nodes: &[TopologyNode],
    config: &TopologyConfig,
) -> HashMap<String, NodePosition> {
    let mut positions = HashMap::new();

    let center_x = config.width as f64 / 2.0;
    let center_y = config.height as f64 / 2.0;

    // Use a smaller radius to ensure nodes stay well within bounds
    // Account for node radius and labels below nodes
    let margin = config.node_radius * 4.0;
    let max_radius = (config.width.min(config.height) as f64 / 2.0) - margin;
    let radius = max_radius * 0.7;  // Use 70% of available space

    // Put local node in center
    let (local_nodes, other_nodes): (Vec<_>, Vec<_>) = nodes
        .iter()
        .partition(|n| n.node_type == TopologyNodeType::LocalNode);

    for node in local_nodes {
        positions.insert(node.id.clone(), NodePosition::new(center_x, center_y));
    }

    // Start angle at -90 degrees (top) so first node is at top
    let start_angle = -std::f64::consts::PI / 2.0;

    for (i, node) in other_nodes.iter().enumerate() {
        if let Some((x, y)) = node.position {
            positions.insert(node.id.clone(), NodePosition::new(x, y));
        } else {
            let angle = start_angle + (i as f64) * 2.0 * std::f64::consts::PI / (other_nodes.len().max(1) as f64);
            positions.insert(
                node.id.clone(),
                NodePosition::new(
                    center_x + radius * angle.cos(),
                    center_y + radius * angle.sin(),
                ),
            );
        }
    }

    positions
}

/// Type of network traffic for data flow visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TrafficType {
    /// HTTP/REST API traffic.
    #[default]
    Http,
    /// GossipSub pub/sub messages.
    GossipSub,
    /// Email or chat messages.
    EmailChat,
    /// WebSocket streaming data.
    WebSocket,
}

impl TrafficType {
    /// Get display label for the traffic type.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Http => "HTTP",
            Self::GossipSub => "GossipSub",
            Self::EmailChat => "Email/Chat",
            Self::WebSocket => "WebSocket",
        }
    }

    /// Get color for the traffic type.
    pub fn as_color(&self) -> &'static str {
        match self {
            Self::Http => "#1890ff",      // Blue
            Self::GossipSub => "#52c41a", // Green
            Self::EmailChat => "#eb2f96", // Magenta/Pink
            Self::WebSocket => "#faad14", // Orange
        }
    }

    /// Get all traffic types.
    pub fn all() -> &'static [TrafficType] {
        &[Self::Http, Self::GossipSub, Self::EmailChat, Self::WebSocket]
    }
}

/// A data flow particle traveling along an edge.
#[derive(Debug, Clone)]
pub struct DataFlowParticle {
    /// Unique particle identifier.
    pub id: u32,
    /// Source node ID.
    pub source: String,
    /// Target node ID.
    pub target: String,
    /// Type of traffic this particle represents.
    pub traffic_type: TrafficType,
    /// Progress along the edge (0.0 to 1.0).
    pub progress: f64,
    /// Speed of the particle (progress per tick).
    pub speed: f64,
}

impl DataFlowParticle {
    /// Create a new data flow particle.
    pub fn new(id: u32, source: impl Into<String>, target: impl Into<String>, traffic_type: TrafficType) -> Self {
        Self {
            id,
            source: source.into(),
            target: target.into(),
            traffic_type,
            progress: 0.0,
            speed: 0.015 + (id as f64 % 5.0) * 0.005, // Vary speed slightly
        }
    }

    /// Check if the particle has completed its journey.
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }

    /// Update the particle progress and return whether it's still active.
    pub fn tick(&mut self) -> bool {
        self.progress += self.speed;
        !self.is_complete()
    }
}
