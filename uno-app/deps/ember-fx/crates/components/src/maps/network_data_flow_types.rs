//! Types for NetworkDataFlow component.
//!
//! This module provides data structures for configuring and rendering
//! the network data flow visualization.

/// Color palette options for the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorPalette {
    /// Default blue theme
    #[default]
    Blue,
    /// Purple theme
    Purple,
    /// Pink theme
    Pink,
    /// Orange-red theme
    OrangeRed,
    /// Amber/Orange theme
    Amber,
    /// Cyan/Teal theme
    Cyan,
    /// Lime green theme
    Green,
    /// Glossy blue ocean theme
    OceanBlue,
    /// Clean arctic white theme
    ArcticWhite,
    /// Natural forest green theme
    ForestGreen,
    /// Warm sunset orange theme
    SunsetOrange,
    /// Deep midnight purple theme
    MidnightPurple,
    /// Grayscale monochrome theme
    Monochrome,
}

impl ColorPalette {
    /// Get the fill color for countries.
    pub fn country_fill(&self) -> &'static str {
        match self {
            Self::Blue => "#3b82f6",
            Self::Purple => "#8b5cf6",
            Self::Pink => "#ec4899",
            Self::OrangeRed => "#f97316",
            Self::Amber => "#f59e0b",
            Self::Cyan => "#14b8a6",
            Self::Green => "#84cc16",
            Self::OceanBlue => "#2563eb",
            Self::ArcticWhite => "#e5e7eb",
            Self::ForestGreen => "#059669",
            Self::SunsetOrange => "#f97316",
            Self::MidnightPurple => "#7c3aed",
            Self::Monochrome => "#4b5563",
        }
    }

    /// Get the stroke color for country borders.
    pub fn country_stroke(&self) -> &'static str {
        match self {
            Self::Blue => "#2563eb",
            Self::Purple => "#7c3aed",
            Self::Pink => "#db2777",
            Self::OrangeRed => "#ea580c",
            Self::Amber => "#d97706",
            Self::Cyan => "#0d9488",
            Self::Green => "#65a30d",
            Self::OceanBlue => "#1d4ed8",
            Self::ArcticWhite => "#9ca3af",
            Self::ForestGreen => "#047857",
            Self::SunsetOrange => "#ea580c",
            Self::MidnightPurple => "#6d28d9",
            Self::Monochrome => "#374151",
        }
    }

    /// Get node colors as (federation_color, fleet_color).
    pub fn node_colors(&self) -> (&'static str, &'static str) {
        match self {
            Self::Blue => ("#f59e0b", "#10b981"),
            Self::Purple => ("#f59e0b", "#06b6d4"),
            Self::Pink => ("#fbbf24", "#22d3ee"),
            Self::OrangeRed => ("#6366f1", "#14b8a6"),
            Self::Amber => ("#8b5cf6", "#06b6d4"),
            Self::Cyan => ("#f97316", "#fbbf24"),
            Self::Green => ("#f97316", "#06b6d4"),
            Self::OceanBlue => ("#f59e0b", "#10b981"),
            Self::ArcticWhite => ("#3b82f6", "#8b5cf6"),
            Self::ForestGreen => ("#f97316", "#06b6d4"),
            Self::SunsetOrange => ("#6366f1", "#14b8a6"),
            Self::MidnightPurple => ("#fbbf24", "#22d3ee"),
            Self::Monochrome => ("#ffffff", "#9ca3af"),
        }
    }

    /// Get particle color for data flow animation.
    pub fn particle_color(&self) -> &'static str {
        match self {
            Self::Blue => "#93c5fd",
            Self::Purple => "#c4b5fd",
            Self::Pink => "#f9a8d4",
            Self::OrangeRed => "#fdba74",
            Self::Amber => "#fcd34d",
            Self::Cyan => "#5eead4",
            Self::Green => "#bef264",
            Self::OceanBlue => "#60a5fa",
            Self::ArcticWhite => "#3b82f6",
            Self::ForestGreen => "#34d399",
            Self::SunsetOrange => "#fbbf24",
            Self::MidnightPurple => "#c4b5fd",
            Self::Monochrome => "#d1d5db",
        }
    }

    /// Get connection line color.
    pub fn connection_color(&self) -> &'static str {
        match self {
            Self::Blue => "#60a5fa",
            Self::Purple => "#a78bfa",
            Self::Pink => "#f472b6",
            Self::OrangeRed => "#fb923c",
            Self::Amber => "#fbbf24",
            Self::Cyan => "#2dd4bf",
            Self::Green => "#a3e635",
            Self::OceanBlue => "#93c5fd",
            Self::ArcticWhite => "#6b7280",
            Self::ForestGreen => "#6ee7b7",
            Self::SunsetOrange => "#fdba74",
            Self::MidnightPurple => "#a78bfa",
            Self::Monochrome => "#9ca3af",
        }
    }

    /// Get background color.
    pub fn background_color(&self) -> &'static str {
        match self {
            Self::Blue => "#0f172a",
            Self::Purple => "#1e1033",
            Self::Pink => "#1f1318",
            Self::OrangeRed => "#1c1917",
            Self::Amber => "#1c1917",
            Self::Cyan => "#042f2e",
            Self::Green => "#14230a",
            Self::OceanBlue => "#0f172a",
            Self::ArcticWhite => "#1f2937",
            Self::ForestGreen => "#022c22",
            Self::SunsetOrange => "#1c1917",
            Self::MidnightPurple => "#0f0326",
            Self::Monochrome => "#111827",
        }
    }

    /// Get all available palettes.
    pub fn all() -> &'static [ColorPalette] {
        &[
            Self::Blue,
            Self::Purple,
            Self::Pink,
            Self::OrangeRed,
            Self::Amber,
            Self::Cyan,
            Self::Green,
            Self::OceanBlue,
            Self::ArcticWhite,
            Self::ForestGreen,
            Self::SunsetOrange,
            Self::MidnightPurple,
            Self::Monochrome,
        ]
    }

    /// Get human-readable label for the palette.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Blue => "Blue",
            Self::Purple => "Purple",
            Self::Pink => "Pink",
            Self::OrangeRed => "Orange",
            Self::Amber => "Amber",
            Self::Cyan => "Cyan",
            Self::Green => "Green",
            Self::OceanBlue => "Ocean Blue",
            Self::ArcticWhite => "Arctic White",
            Self::ForestGreen => "Forest Green",
            Self::SunsetOrange => "Sunset Orange",
            Self::MidnightPurple => "Midnight Purple",
            Self::Monochrome => "Monochrome",
        }
    }
}

/// Cursor interaction mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorMode {
    /// Explore mode: hover/click on nodes and countries.
    #[default]
    Explore,
    /// Drag mode: pan the map by click-dragging.
    Drag,
}

impl CursorMode {
    /// Get label for the mode.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Explore => "Explore",
            Self::Drag => "Drag",
        }
    }

    /// Get icon for the mode.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Explore => "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
            Self::Drag => "M7 11.5V14m0-2.5v-6a1.5 1.5 0 113 0m-3 6a1.5 1.5 0 00-3 0v2a7.5 7.5 0 0015 0v-5a1.5 1.5 0 00-3 0m-6-3V11m0-5.5v-1a1.5 1.5 0 013 0v1m0 0V11m0-5.5a1.5 1.5 0 013 0v3m0 0V11",
        }
    }
}

/// Type of network node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkNodeType {
    /// Federation: larger central node.
    Federation,
    /// Fleet: smaller worker node.
    Fleet,
}

impl NetworkNodeType {
    /// Get the radius for this node type.
    pub fn radius(&self) -> f64 {
        match self {
            Self::Federation => 12.0,
            Self::Fleet => 8.0,
        }
    }

    /// Get the label for this node type.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Federation => "Federation",
            Self::Fleet => "Fleet",
        }
    }
}

/// A network node representing a Federation or Fleet.
#[derive(Debug, Clone)]
pub struct NetworkNode {
    /// Unique identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Node type (Federation or Fleet).
    pub node_type: NetworkNodeType,
    /// Region/continent name.
    pub region: &'static str,
    /// X position in SVG viewBox coordinates.
    pub x: f64,
    /// Y position in SVG viewBox coordinates.
    pub y: f64,
}

impl NetworkNode {
    /// Create a new network node.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        node_type: NetworkNodeType,
        region: &'static str,
        x: f64,
        y: f64,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            node_type,
            region,
            x,
            y,
        }
    }
}

/// A connection between two nodes.
#[derive(Debug, Clone)]
pub struct NetworkConnection {
    /// Source node ID.
    pub source_id: String,
    /// Target node ID.
    pub target_id: String,
}

impl NetworkConnection {
    /// Create a new connection.
    pub fn new(source_id: impl Into<String>, target_id: impl Into<String>) -> Self {
        Self {
            source_id: source_id.into(),
            target_id: target_id.into(),
        }
    }
}

/// An animated particle flowing between nodes.
#[derive(Debug, Clone)]
pub struct DataParticle {
    /// Unique identifier.
    pub id: u32,
    /// Source node ID.
    pub source_id: String,
    /// Target node ID.
    pub target_id: String,
    /// Progress along the path (0.0 to 1.0).
    pub progress: f64,
    /// Speed of particle movement per frame.
    pub speed: f64,
}

impl DataParticle {
    /// Create a new particle.
    pub fn new(
        id: u32,
        source_id: impl Into<String>,
        target_id: impl Into<String>,
        speed: f64,
    ) -> Self {
        Self {
            id,
            source_id: source_id.into(),
            target_id: target_id.into(),
            progress: 0.0,
            speed,
        }
    }
}

/// Configuration for the NetworkDataFlow component.
#[derive(Debug, Clone)]
pub struct NetworkDataFlowConfig {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// SVG viewBox width.
    pub viewbox_width: u32,
    /// SVG viewBox height.
    pub viewbox_height: u32,
    /// Default color palette.
    pub default_palette: ColorPalette,
    /// Default cursor mode.
    pub default_cursor_mode: CursorMode,
    /// Show zoom +/- controls.
    pub show_zoom_controls: bool,
    /// Show color palette dropdown.
    pub show_palette_selector: bool,
    /// Show cursor mode toggle.
    pub show_mode_toggle: bool,
    /// Show data flow animation toggle.
    pub show_flow_toggle: bool,
    /// Initial zoom level (1.0 = default).
    pub initial_zoom: f64,
    /// Minimum zoom level.
    pub min_zoom: f64,
    /// Maximum zoom level.
    pub max_zoom: f64,
    /// Zoom step per button click.
    pub zoom_step: f64,
    /// Particle movement speed per frame.
    pub particle_speed: f64,
    /// Maximum number of particles at once.
    pub max_particles: usize,
    /// Enable data flow animation by default.
    pub show_flow_by_default: bool,
}

impl Default for NetworkDataFlowConfig {
    fn default() -> Self {
        Self {
            width: 900,
            height: 506,
            viewbox_width: 1920,
            viewbox_height: 1080,
            default_palette: ColorPalette::default(),
            default_cursor_mode: CursorMode::default(),
            show_zoom_controls: true,
            show_palette_selector: true,
            show_mode_toggle: true,
            show_flow_toggle: true,
            initial_zoom: 1.0,
            min_zoom: 0.5,
            max_zoom: 4.0,
            zoom_step: 0.25,
            particle_speed: 0.02,
            max_particles: 50,
            show_flow_by_default: true,
        }
    }
}

/// Region definition for node placement.
#[derive(Debug, Clone, Copy)]
pub struct RegionBounds {
    /// Region name.
    pub name: &'static str,
    /// Minimum X coordinate.
    pub min_x: f64,
    /// Minimum Y coordinate.
    pub min_y: f64,
    /// Maximum X coordinate.
    pub max_x: f64,
    /// Maximum Y coordinate.
    pub max_y: f64,
    /// Number of federations to place.
    pub federations: usize,
    /// Number of fleets to place.
    pub fleets: usize,
}

/// Default region bounds for demo node generation.
pub const DEMO_REGIONS: &[RegionBounds] = &[
    RegionBounds { name: "North America", min_x: 250.0, min_y: 180.0, max_x: 520.0, max_y: 380.0, federations: 2, fleets: 4 },
    RegionBounds { name: "South America", min_x: 420.0, min_y: 580.0, max_x: 620.0, max_y: 820.0, federations: 1, fleets: 3 },
    RegionBounds { name: "Europe", min_x: 920.0, min_y: 220.0, max_x: 1080.0, max_y: 360.0, federations: 2, fleets: 4 },
    RegionBounds { name: "Africa", min_x: 960.0, min_y: 450.0, max_x: 1140.0, max_y: 680.0, federations: 1, fleets: 3 },
    RegionBounds { name: "Asia", min_x: 1200.0, min_y: 240.0, max_x: 1600.0, max_y: 480.0, federations: 3, fleets: 5 },
    RegionBounds { name: "Oceania", min_x: 1520.0, min_y: 620.0, max_x: 1720.0, max_y: 800.0, federations: 1, fleets: 2 },
];

/// Generate demo nodes distributed across regions.
pub fn generate_demo_nodes() -> Vec<NetworkNode> {
    let mut nodes = Vec::new();
    let mut node_id = 0;

    // Simple deterministic pseudo-random generator
    let mut seed: u64 = 12345;
    let mut next_rand = || {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((seed >> 16) & 0x7fff) as f64 / 32767.0
    };

    for region in DEMO_REGIONS {
        let width = region.max_x - region.min_x;
        let height = region.max_y - region.min_y;

        // Place federations
        for i in 0..region.federations {
            let x = region.min_x + width * (0.2 + 0.6 * next_rand());
            let y = region.min_y + height * (0.2 + 0.6 * next_rand());
            nodes.push(NetworkNode::new(
                format!("fed-{}", node_id),
                format!("{} Federation {}", region.name, i + 1),
                NetworkNodeType::Federation,
                region.name,
                x,
                y,
            ));
            node_id += 1;
        }

        // Place fleets
        for i in 0..region.fleets {
            let x = region.min_x + width * (0.1 + 0.8 * next_rand());
            let y = region.min_y + height * (0.1 + 0.8 * next_rand());
            nodes.push(NetworkNode::new(
                format!("fleet-{}", node_id),
                format!("{} Fleet {}", region.name, i + 1),
                NetworkNodeType::Fleet,
                region.name,
                x,
                y,
            ));
            node_id += 1;
        }
    }

    nodes
}

/// Generate connections between nodes.
/// Creates connections between federations and fleets in the same region,
/// and some inter-region federation connections.
pub fn generate_demo_connections(nodes: &[NetworkNode]) -> Vec<NetworkConnection> {
    let mut connections = Vec::new();

    // Group nodes by region
    let mut regions: std::collections::HashMap<&str, Vec<&NetworkNode>> = std::collections::HashMap::new();
    for node in nodes {
        regions.entry(node.region).or_default().push(node);
    }

    // Connect nodes within each region
    for (_region, region_nodes) in &regions {
        let federations: Vec<_> = region_nodes.iter()
            .filter(|n| n.node_type == NetworkNodeType::Federation)
            .collect();
        let fleets: Vec<_> = region_nodes.iter()
            .filter(|n| n.node_type == NetworkNodeType::Fleet)
            .collect();

        // Connect each fleet to nearest federation
        for fleet in &fleets {
            if let Some(fed) = federations.first() {
                connections.push(NetworkConnection::new(&fed.id, &fleet.id));
            }
        }

        // Connect federations within region
        for i in 0..federations.len() {
            for j in (i + 1)..federations.len() {
                connections.push(NetworkConnection::new(&federations[i].id, &federations[j].id));
            }
        }
    }

    // Connect some federations across regions
    let all_federations: Vec<_> = nodes.iter()
        .filter(|n| n.node_type == NetworkNodeType::Federation)
        .collect();

    if all_federations.len() >= 2 {
        // Connect first federation to others across regions
        let regions_seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
        let mut connected_regions = regions_seen;
        connected_regions.insert(all_federations[0].region);

        for fed in all_federations.iter().skip(1) {
            if !connected_regions.contains(fed.region) {
                connections.push(NetworkConnection::new(&all_federations[0].id, &fed.id));
                connected_regions.insert(fed.region);
            }
        }
    }

    connections
}
