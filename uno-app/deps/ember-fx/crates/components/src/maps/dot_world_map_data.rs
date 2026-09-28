//! DotWorldMap SVG data and network node definitions.
//!
//! Contains the SVG path data for the dot-based world map visualization,
//! network node types (Federation/Fleet), and demo data generators.

use std::collections::HashMap;

// Re-export ColorPalette from network_data_flow_types
pub use super::network_data_flow_types::ColorPalette;

/// Neutral state color (gray).
pub const NEUTRAL_COLOR: &str = "#4A4A5A";

/// Active state color (indigo).
pub const ACTIVE_COLOR: &str = "#6366F1";

/// Hover state color (light indigo).
pub const HOVER_COLOR: &str = "#818CF8";

/// SVG viewBox dimensions.
pub const VIEWBOX: &str = "-1 -1 128 72.15";

/// ViewBox width.
pub const VIEWBOX_WIDTH: f64 = 130.0;

/// ViewBox height.
pub const VIEWBOX_HEIGHT: f64 = 74.15;

/// The SVG path data for the world map dots.
/// This is extracted from the SVG files at compile time.
pub const WORLD_DOT_PATH: &str = include_str!("../../../../crates/styles/svg/world_neutral.svg");

/// Extract just the path `d` attribute from the full SVG.
pub fn get_path_data() -> &'static str {
    // The SVG format is: <svg ...><path d="..." fill="..."></path></svg>
    // We need to extract just the path data between d=" and the next "
    let svg = WORLD_DOT_PATH;
    if let Some(start) = svg.find("d=\"") {
        let start = start + 3; // Skip past 'd="'
        if let Some(end) = svg[start..].find('"') {
            return &svg[start..start + end];
        }
    }
    ""
}

// ============================================================================
// Network Node Types
// ============================================================================

/// Node type for network visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DotNodeType {
    /// Federation node - rounded rectangle container.
    Federation,
    /// Fleet node - small circle inside a Federation.
    #[default]
    Fleet,
}

/// Federation visual dimensions.
pub const FEDERATION_SIZE: f64 = 6.0;      // Width/height of the rounded rect
pub const FEDERATION_RADIUS: f64 = 1.5;    // Corner radius
pub const FLEET_RADIUS: f64 = 1.2;         // Fleet circle radius
pub const FLEET_SPACING: f64 = 2.8;        // Space between fleet circles

impl DotNodeType {
    /// Get the radius for this node type (for circles or corner radius).
    pub fn radius(&self) -> f64 {
        match self {
            DotNodeType::Federation => FEDERATION_SIZE / 2.0,
            DotNodeType::Fleet => FLEET_RADIUS,
        }
    }

    /// Get the label for this node type.
    pub fn label(&self) -> &'static str {
        match self {
            DotNodeType::Federation => "Federation",
            DotNodeType::Fleet => "Fleet",
        }
    }
}

/// Cursor mode for map interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DotCursorMode {
    /// Explore mode - hover and click interactions.
    #[default]
    Explore,
    /// Drag mode - pan the map.
    Drag,
}

/// Network node on the dot world map.
#[derive(Debug, Clone)]
pub struct DotNetworkNode {
    /// Unique node identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Node type (Federation or Fleet).
    pub node_type: DotNodeType,
    /// Country code (ISO 3166-1 alpha-2).
    pub country_code: &'static str,
    /// Country name.
    pub country_name: &'static str,
    /// X coordinate in viewbox space.
    pub x: f64,
    /// Y coordinate in viewbox space.
    pub y: f64,
    /// Parent Federation ID (for Fleet nodes).
    pub parent_id: Option<String>,
}

/// Connection between two network nodes.
#[derive(Debug, Clone)]
pub struct DotNetworkConnection {
    /// Source node ID.
    pub source_id: String,
    /// Target node ID.
    pub target_id: String,
}

/// Data particle for animated flow visualization.
#[derive(Debug, Clone)]
pub struct DotDataParticle {
    /// Unique particle identifier.
    pub id: u32,
    /// Source node ID.
    pub source_id: String,
    /// Target node ID.
    pub target_id: String,
    /// Progress along the path (0.0 to 1.0).
    pub progress: f64,
    /// Movement speed per frame.
    pub speed: f64,
}

/// Country node count summary.
#[derive(Debug, Clone)]
pub struct CountryNodeCount {
    /// Country code.
    pub code: &'static str,
    /// Country name.
    pub name: &'static str,
    /// Number of Federation nodes.
    pub federation_count: usize,
    /// Number of Fleet nodes.
    pub fleet_count: usize,
}

impl CountryNodeCount {
    /// Get total node count.
    pub fn total(&self) -> usize {
        self.federation_count + self.fleet_count
    }
}

/// Tooltip data for display.
#[derive(Debug, Clone)]
pub enum DotTooltipData {
    /// Country tooltip with node counts.
    Country {
        name: String,
        federation: usize,
        fleet: usize,
    },
    /// Node tooltip with details.
    Node {
        name: String,
        node_type: DotNodeType,
        country: String,
    },
}

// ============================================================================
// Region Definitions for Node Placement
// ============================================================================

/// Federation location with exact coordinates.
/// Coordinates based on the dot world map SVG viewbox "-1 -1 128 72.15".
/// Measured from actual dot positions in the map.
pub struct FederationDef {
    pub name: &'static str,
    pub x: f64,
    pub y: f64,
    pub fleets: usize,
    pub country_code: &'static str,
    pub country_name: &'static str,
}

/// Pre-defined Federation locations with coordinates matching the dot map.
/// Coordinates aligned with unitynodes.io reference - viewBox: "-1 -1 128 72.15"
/// X: 0 = far west (Pacific), 128 = far east (Pacific)
/// Y: 0 = north pole, 72 = south pole
pub const FEDERATION_LOCATIONS: &[FederationDef] = &[
    // North America
    // US West Coast - California centered around x:16, y:26
    FederationDef { name: "San Francisco", x: 16.0, y: 26.0, fleets: 2, country_code: "us", country_name: "United States" },
    // US East Coast - NYC area around x:26, y:24
    FederationDef { name: "New York", x: 26.0, y: 24.0, fleets: 3, country_code: "us", country_name: "United States" },

    // South America
    // Brazil - São Paulo around x:35, y:50
    FederationDef { name: "São Paulo", x: 35.0, y: 50.0, fleets: 2, country_code: "br", country_name: "Brazil" },
    // Argentina - Buenos Aires around x:33, y:54
    FederationDef { name: "Buenos Aires", x: 33.0, y: 54.0, fleets: 1, country_code: "ar", country_name: "Argentina" },

    // Europe - cluster between x:48-56, y:18-22
    // UK - London around x:49, y:19
    FederationDef { name: "London", x: 49.0, y: 19.0, fleets: 2, country_code: "gb", country_name: "United Kingdom" },
    // Germany - Frankfurt around x:52, y: 20
    FederationDef { name: "Frankfurt", x: 52.0, y: 20.0, fleets: 2, country_code: "de", country_name: "Germany" },
    // Netherlands - Amsterdam around x:51, y:18
    FederationDef { name: "Amsterdam", x: 51.0, y: 18.0, fleets: 1, country_code: "nl", country_name: "Netherlands" },

    // Africa
    // Nigeria - Lagos around x:52, y:36
    FederationDef { name: "Lagos", x: 52.0, y: 36.0, fleets: 2, country_code: "ng", country_name: "Nigeria" },
    // South Africa - Johannesburg around x:60, y:52
    FederationDef { name: "Johannesburg", x: 60.0, y: 52.0, fleets: 2, country_code: "za", country_name: "South Africa" },

    // Middle East / South Asia
    // UAE - Dubai around x:66, y:28
    FederationDef { name: "Dubai", x: 66.0, y: 28.0, fleets: 1, country_code: "ae", country_name: "United Arab Emirates" },
    // India - Mumbai around x:72, y:30
    FederationDef { name: "Mumbai", x: 72.0, y: 30.0, fleets: 2, country_code: "in", country_name: "India" },
    // India - Bangalore around x:74, y:34
    FederationDef { name: "Bangalore", x: 74.0, y: 34.0, fleets: 1, country_code: "in", country_name: "India" },

    // Southeast Asia
    // Thailand - Bangkok around x:82, y:34
    FederationDef { name: "Bangkok", x: 82.0, y: 34.0, fleets: 1, country_code: "th", country_name: "Thailand" },
    // Singapore around x:84, y:38
    FederationDef { name: "Singapore", x: 84.0, y: 38.0, fleets: 2, country_code: "sg", country_name: "Singapore" },
    // Indonesia - Jakarta around x:86, y:40
    FederationDef { name: "Jakarta", x: 86.0, y: 40.0, fleets: 1, country_code: "id", country_name: "Indonesia" },

    // East Asia
    // China - Shanghai around x:98, y:26
    FederationDef { name: "Shanghai", x: 98.0, y: 26.0, fleets: 2, country_code: "cn", country_name: "China" },
    // South Korea - Seoul around x:100, y:24
    FederationDef { name: "Seoul", x: 100.0, y: 24.0, fleets: 1, country_code: "kr", country_name: "South Korea" },
    // Japan - Tokyo around x:106, y:24
    FederationDef { name: "Tokyo", x: 106.0, y: 24.0, fleets: 2, country_code: "jp", country_name: "Japan" },

    // Oceania
    // Australia - Sydney around x:114, y:54
    FederationDef { name: "Sydney", x: 114.0, y: 54.0, fleets: 2, country_code: "au", country_name: "Australia" },
    // New Zealand - Auckland around x:121, y:56
    FederationDef { name: "Auckland", x: 121.0, y: 56.0, fleets: 1, country_code: "nz", country_name: "New Zealand" },
];

// ============================================================================
// Demo Data Generators
// ============================================================================

/// Generate demo network nodes with Fleets inside Federation rectangles.
pub fn generate_demo_nodes() -> Vec<DotNetworkNode> {
    let mut nodes = Vec::new();
    let mut fed_counter = 0u32;
    let mut fleet_counter = 0u32;

    for fed_def in FEDERATION_LOCATIONS {
        let fed_id = format!("fed-{}", fed_counter);

        // Add Federation node at exact location
        nodes.push(DotNetworkNode {
            id: fed_id.clone(),
            name: format!("{} Federation", fed_def.name),
            node_type: DotNodeType::Federation,
            country_code: fed_def.country_code,
            country_name: fed_def.country_name,
            x: fed_def.x,
            y: fed_def.y,
            parent_id: None,
        });
        fed_counter += 1;

        // Add Fleet nodes positioned INSIDE the Federation rectangle
        let fleet_count = fed_def.fleets;

        for i in 0..fleet_count {
            // Position fleets in a row inside the federation box
            // For 2 fleets: positions at -1/4 and +1/4 of the box width
            // For 3 fleets: positions at -1/3, 0, +1/3 of the box width
            let offset = if fleet_count == 1 {
                0.0
            } else {
                let slot = i as f64 - (fleet_count as f64 - 1.0) / 2.0;
                slot * FLEET_SPACING
            };

            let fleet_x = fed_def.x + offset;
            let fleet_y = fed_def.y; // Center vertically in the federation

            nodes.push(DotNetworkNode {
                id: format!("fleet-{}", fleet_counter),
                name: format!("{} Fleet {}", fed_def.name, i + 1),
                node_type: DotNodeType::Fleet,
                country_code: fed_def.country_code,
                country_name: fed_def.country_name,
                x: fleet_x,
                y: fleet_y,
                parent_id: Some(fed_id.clone()),
            });
            fleet_counter += 1;
        }
    }

    nodes
}

/// Generate connections between Federation and Fleet nodes.
/// Each Fleet connects to its nearest Federation (its parent).
/// Federations also connect to nearby Federations for inter-region links.
pub fn generate_demo_connections(nodes: &[DotNetworkNode]) -> Vec<DotNetworkConnection> {
    let mut connections = Vec::new();

    let federations: Vec<_> = nodes.iter()
        .filter(|n| n.node_type == DotNodeType::Federation)
        .collect();

    let fleets: Vec<_> = nodes.iter()
        .filter(|n| n.node_type == DotNodeType::Fleet)
        .collect();

    // Connect each Fleet to its nearest Federation (parent)
    for fleet in &fleets {
        // Find the closest Federation to this Fleet
        let closest_fed = federations.iter()
            .min_by(|a, b| {
                let dist_a = (a.x - fleet.x).powi(2) + (a.y - fleet.y).powi(2);
                let dist_b = (b.x - fleet.x).powi(2) + (b.y - fleet.y).powi(2);
                dist_a.partial_cmp(&dist_b).unwrap()
            });

        if let Some(fed) = closest_fed {
            connections.push(DotNetworkConnection {
                source_id: fed.id.clone(),
                target_id: fleet.id.clone(),
            });
        }
    }

    // Connect nearby Federations to each other (inter-region backbone)
    for (i, fed) in federations.iter().enumerate() {
        for other_fed in federations.iter().skip(i + 1) {
            let dx = other_fed.x - fed.x;
            let dy = other_fed.y - fed.y;
            let dist = (dx * dx + dy * dy).sqrt();
            // Connect if within 30 units (regional neighbors)
            if dist < 30.0 {
                connections.push(DotNetworkConnection {
                    source_id: fed.id.clone(),
                    target_id: other_fed.id.clone(),
                });
            }
        }
    }

    connections
}

/// Get node counts per country from a list of nodes.
pub fn get_country_node_counts(nodes: &[DotNetworkNode]) -> Vec<CountryNodeCount> {
    let mut counts: HashMap<&'static str, (usize, usize, &'static str)> = HashMap::new();

    for node in nodes {
        let entry = counts.entry(node.country_code).or_insert((0, 0, node.country_name));
        match node.node_type {
            DotNodeType::Federation => entry.0 += 1,
            DotNodeType::Fleet => entry.1 += 1,
        }
    }

    counts.into_iter()
        .map(|(code, (fed, fleet, name))| CountryNodeCount {
            code,
            name,
            federation_count: fed,
            fleet_count: fleet,
        })
        .collect()
}
