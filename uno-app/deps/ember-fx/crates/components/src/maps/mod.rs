//! Maps and geographic visualization components.
//!
//! This module provides components for building analytics dashboards
//! with geographic visualizations, progress indicators, and activity lists.
//!
//! ## Components
//!
//! - [`HexagonalWorldMap`] - Geographic visualization using hexagonal tessellation
//! - [`SvgWorldMap`] - Country-level world map with SVG paths
//! - [`OrionHexWorldMap`] - Pre-computed hexagonal world map from Orion UI Kit
//! - [`NetworkDataFlow`] - Network visualization with nodes and animated data flow
//! - [`OceanMap`] - Ocean temperature visualization
//! - [`WindMap`] - Wind pattern visualization
//! - [`ProgressRing`] - Circular percentage indicators
//! - [`ArcFlowLines`] - Connection visualization between points
//! - [`GradientProgressBar`] - Linear progress bar with gradient coloring
//! - [`CityMarker`] - Tooltip-style callouts for map locations
//! - [`ActivityList`] - Country/region breakdown tables
//!
//! ## Example
//!
//! ```ignore
//! use ember_fx_components::maps::{
//!     ProgressRing, ProgressRingData, ProgressRingConfig,
//! };
//!
//! let data = Signal::derive(|| ProgressRingData::new(75.0, "Active Users"));
//!
//! view! {
//!     <ProgressRing data=data />
//! }
//! ```

mod types;
mod world_data;
mod progress_ring;
mod gradient_progress_bar;
mod activity_list;
mod city_marker;
mod arc_flow_lines;
mod hexagonal_world_map;
mod svg_country_data;
mod svg_world_map;
mod orion_hex_data;
mod orion_hex_world_map;
mod ocean_map_data;
mod ocean_map;
mod wind_map_data;
mod wind_map;
mod network_data_flow;
mod network_data_flow_types;
mod dot_world_map;
pub mod dot_world_map_data;

// Re-export types
pub use types::*;

// Re-export components
pub use progress_ring::ProgressRing;
pub use gradient_progress_bar::GradientProgressBar;
pub use activity_list::ActivityList;
pub use city_marker::CityMarker;
pub use arc_flow_lines::ArcFlowLines;
pub use hexagonal_world_map::{HexagonalWorldMap, HexagonalWorldMapLegend};
pub use city_marker::CityMarkerGroup;
pub use svg_world_map::{SvgWorldMap, SvgWorldMapLegend};
pub use svg_country_data::{COUNTRIES, get_country, get_countries_by_region, get_all_codes};
pub use orion_hex_world_map::{OrionHexWorldMap, OrionHexWorldMapLegend, OrionHexWorldMapConfig, OrionHexCellState};
pub use orion_hex_data::{HEX_MAP_CELLS, HexMapCell, HEX_COUNT, LAND_COUNT, OCEAN_COUNT, land_cells, ocean_cells};
pub use ocean_map::{OceanMap, OceanMapLegend, OceanMapConfig, OceanColorScale};
pub use ocean_map_data::{OCEAN_MAP_REGIONS, OceanMapRegion, REGION_COUNT};
pub use wind_map::{WindMap, WindMapInfo, WindMapConfig};
pub use wind_map_data::{WIND_MAP_DOTS, WindMapDot, DOT_COUNT};
pub use network_data_flow::NetworkDataFlow;
pub use network_data_flow_types::{
    ColorPalette, CursorMode, NetworkNode, NetworkNodeType,
    NetworkConnection, DataParticle, NetworkDataFlowConfig,
    RegionBounds, DEMO_REGIONS, generate_demo_nodes, generate_demo_connections,
};
pub use dot_world_map::{DotWorldMap, DotWorldMapVariant, DotWorldMapConfig, DotWorldMapState};
pub use dot_world_map_data::{NEUTRAL_COLOR, ACTIVE_COLOR, HOVER_COLOR};
