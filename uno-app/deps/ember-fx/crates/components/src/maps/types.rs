//! Shared types for dashboard components.
//!
//! This module defines common data structures used across the dashboard
//! visualization components.

use std::collections::HashMap;
use std::f64::consts::PI;

// Re-export ChartColor from chart module for convenience
pub use crate::chart::ChartColor;

// ============================================================================
// SVG World Map Types
// ============================================================================

/// Static country path data.
///
/// Contains the SVG path data for a country, extracted from the source SVG.
#[derive(Debug, Clone, Copy)]
pub struct CountryData {
    /// ISO 3166-1 alpha-2 code (e.g., "us", "fr", "cn").
    pub code: &'static str,
    /// Display name (e.g., "United States").
    pub name: &'static str,
    /// Region/continent (e.g., "North America").
    pub region: &'static str,
    /// Bounding box (min_x, min_y, width, height) in viewBox coordinates.
    pub bbox: (f64, f64, f64, f64),
    /// SVG path data strings (multiple paths for countries with islands).
    pub paths: &'static [&'static str],
}

/// Runtime country state with dynamic values.
#[derive(Debug, Clone)]
pub struct CountryState {
    /// Country code (ISO 3166-1 alpha-2).
    pub code: String,
    /// Value for heatmap coloring.
    pub value: f64,
    /// Whether this country is selected.
    pub selected: bool,
    /// Additional metadata.
    pub metadata: std::collections::HashMap<String, String>,
}

impl CountryState {
    /// Create a new country state.
    pub fn new(code: impl Into<String>, value: f64) -> Self {
        Self {
            code: code.into(),
            value,
            selected: false,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set selected state.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Add metadata.
    pub fn meta(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// Configuration for SvgWorldMap component.
#[derive(Debug, Clone)]
pub struct SvgWorldMapConfig {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// ViewBox width (from source SVG).
    pub viewbox_width: u32,
    /// ViewBox height (from source SVG).
    pub viewbox_height: u32,
    /// Color scale for value-based coloring.
    pub color_scale: ColorScale,
    /// Default fill color for countries without values.
    pub default_fill: String,
    /// Stroke color for country borders.
    pub stroke_color: String,
    /// Stroke width for country borders.
    pub stroke_width: f64,
    /// Enable interactive hover/click.
    pub interactive: bool,
    /// Show country labels on hover.
    pub show_tooltip: bool,
    /// Show color legend.
    pub show_legend: bool,
    /// Minimum value for color scaling.
    pub min_value: f64,
    /// Maximum value for color scaling.
    pub max_value: f64,
    /// Enable multi-selection mode.
    pub multi_select: bool,
    /// Selection color override.
    pub selection_color: Option<String>,
    /// Hover color override.
    pub hover_color: Option<String>,
    /// Countries to exclude from rendering.
    pub exclude_countries: Vec<String>,
    /// Enable zoom on country click.
    pub enable_zoom: bool,
    /// Padding around country when zoomed (in viewBox units).
    pub zoom_padding: f64,
    /// Show back button when zoomed in.
    pub show_zoom_controls: bool,
}

impl Default for SvgWorldMapConfig {
    fn default() -> Self {
        Self {
            width: 900,
            height: 506, // Maintains 1920:1080 aspect ratio
            viewbox_width: 1920,
            viewbox_height: 1080,
            color_scale: ColorScale::PurpleToOrange,
            default_fill: "#3a5a8f".to_string(),
            stroke_color: "#2a4070".to_string(),
            stroke_width: 0.5,
            interactive: true,
            show_tooltip: true,
            show_legend: true,
            min_value: 0.0,
            max_value: 100.0,
            multi_select: false,
            selection_color: None,
            hover_color: None,
            exclude_countries: Vec::new(),
            enable_zoom: false,
            zoom_padding: 50.0,
            show_zoom_controls: true,
        }
    }
}

// ============================================================================
// Hexagonal World Map Types
// ============================================================================

/// A hexagonal cell in the world map.
#[derive(Debug, Clone)]
pub struct HexCell {
    /// Unique identifier for this cell.
    pub id: String,
    /// Latitude coordinate.
    pub lat: f64,
    /// Longitude coordinate.
    pub lng: f64,
    /// Value for color scaling.
    pub value: f64,
    /// Optional region name (e.g., "Europe", "Asia").
    pub region: Option<String>,
    /// Additional metadata.
    pub metadata: HashMap<String, String>,
}

impl HexCell {
    /// Create a new hex cell at the given coordinates.
    pub fn new(lat: f64, lng: f64, value: f64) -> Self {
        Self {
            id: format!("{:.2}_{:.2}", lat, lng),
            lat,
            lng,
            value,
            region: None,
            metadata: HashMap::new(),
        }
    }

    /// Create with custom ID.
    pub fn with_id(id: impl Into<String>, lat: f64, lng: f64, value: f64) -> Self {
        Self {
            id: id.into(),
            lat,
            lng,
            value,
            region: None,
            metadata: HashMap::new(),
        }
    }

    /// Set the region.
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Add metadata.
    pub fn meta(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// City/location marker on the map.
#[derive(Debug, Clone)]
pub struct MapLocation {
    /// Unique identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Latitude coordinate.
    pub lat: f64,
    /// Longitude coordinate.
    pub lng: f64,
    /// Value to display (e.g., population, revenue).
    pub value: f64,
    /// Formatted value string (e.g., "98,320,300").
    pub formatted_value: Option<String>,
    /// Icon type.
    pub icon: LocationIcon,
    /// Color for the marker.
    pub color: Option<ChartColor>,
}

impl MapLocation {
    /// Create a new map location.
    pub fn new(id: impl Into<String>, name: impl Into<String>, lat: f64, lng: f64, value: f64) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            lat,
            lng,
            value,
            formatted_value: None,
            icon: LocationIcon::Building,
            color: None,
        }
    }

    /// Set formatted value string.
    pub fn formatted(mut self, formatted: impl Into<String>) -> Self {
        self.formatted_value = Some(formatted.into());
        self
    }

    /// Set icon type.
    pub fn icon(mut self, icon: LocationIcon) -> Self {
        self.icon = icon;
        self
    }

    /// Set color.
    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = Some(color);
        self
    }
}

/// Icon types for location markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LocationIcon {
    #[default]
    Building,
    City,
    Server,
    User,
    Flag,
    Pin,
    Dot,
}

impl LocationIcon {
    /// Get SVG path for the icon.
    pub fn svg_path(&self) -> &'static str {
        match self {
            Self::Building => "M3 21h18v-2H3v2zm0-4h18v-2H3v2zm0-4h18v-2H3v2zm0-4h18V7H3v2zm0-6v2h18V3H3z",
            Self::City => "M15 11V5l-3-3-3 3v2H3v14h18V11h-6zm-8 8H5v-2h2v2zm0-4H5v-2h2v2zm0-4H5V9h2v2zm6 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V9h2v2zm0-4h-2V5h2v2zm6 12h-2v-2h2v2zm0-4h-2v-2h2v2z",
            Self::Server => "M20 13H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-6c0-.55-.45-1-1-1zM7 19c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zM20 3H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1zM7 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            Self::User => "M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z",
            Self::Flag => "M14.4 6L14 4H5v17h2v-7h5.6l.4 2h7V6z",
            Self::Pin => "M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z",
            Self::Dot => "M12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4z",
        }
    }
}

/// Hexagon orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HexOrientation {
    /// Pointy top hexagon (vertex at top).
    #[default]
    PointyTop,
    /// Flat top hexagon (edge at top).
    FlatTop,
}

/// Map projection type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MapProjection {
    /// Mercator projection (most common for web maps).
    #[default]
    Mercator,
    /// Robinson projection (less distortion at poles).
    Robinson,
    /// Equirectangular projection (simple, linear).
    Equirectangular,
}

/// Color scale for heat maps.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorScale {
    /// Blue (cold) to Red (hot).
    BlueToRed,
    /// Purple to Orange (like ORION dashboard).
    PurpleToOrange,
    /// Single color with varying opacity.
    Monochrome(String),
    /// Custom color stops.
    Custom(Vec<String>),
}

impl Default for ColorScale {
    fn default() -> Self {
        Self::PurpleToOrange
    }
}

impl ColorScale {
    /// Get color for a normalized value (0.0 to 1.0).
    pub fn color_at(&self, t: f64) -> String {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::BlueToRed => {
                // Interpolate from blue (#1890ff) to red (#ff4d4f)
                let r = (24.0 + (255.0 - 24.0) * t) as u8;
                let g = (144.0 + (77.0 - 144.0) * t) as u8;
                let b = (255.0 + (79.0 - 255.0) * t) as u8;
                format!("#{:02x}{:02x}{:02x}", r, g, b)
            }
            Self::PurpleToOrange => {
                // Purple (#722ed1) to Orange (#fa8c16)
                let r = (114.0 + (250.0 - 114.0) * t) as u8;
                let g = (46.0 + (140.0 - 46.0) * t) as u8;
                let b = (209.0 + (22.0 - 209.0) * t) as u8;
                format!("#{:02x}{:02x}{:02x}", r, g, b)
            }
            Self::Monochrome(color) => {
                format!("{}; opacity: {:.2}", color, 0.2 + t * 0.8)
            }
            Self::Custom(colors) => {
                if colors.is_empty() {
                    return "#888888".to_string();
                }
                let idx = ((colors.len() - 1) as f64 * t) as usize;
                colors.get(idx).cloned().unwrap_or_else(|| "#888888".to_string())
            }
        }
    }

    /// Get all color stops for gradient display.
    pub fn gradient_stops(&self) -> Vec<(f64, String)> {
        match self {
            Self::BlueToRed => vec![
                (0.0, "#1890ff".to_string()),
                (0.5, "#faad14".to_string()),
                (1.0, "#ff4d4f".to_string()),
            ],
            Self::PurpleToOrange => vec![
                (0.0, "#722ed1".to_string()),
                (0.5, "#eb2f96".to_string()),
                (1.0, "#fa8c16".to_string()),
            ],
            Self::Monochrome(color) => vec![
                (0.0, format!("{}20", color)),
                (1.0, color.clone()),
            ],
            Self::Custom(colors) => {
                colors.iter().enumerate().map(|(i, c)| {
                    (i as f64 / (colors.len() - 1).max(1) as f64, c.clone())
                }).collect()
            }
        }
    }
}

/// Configuration for HexagonalWorldMap.
#[derive(Debug, Clone)]
pub struct HexWorldMapConfig {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Hexagon radius.
    pub hex_size: f64,
    /// Hexagon orientation.
    pub hex_orientation: HexOrientation,
    /// Color scale for values.
    pub color_scale: ColorScale,
    /// Show hexagon grid lines.
    pub show_grid: bool,
    /// Show location labels.
    pub show_labels: bool,
    /// Show color legend.
    pub show_legend: bool,
    /// Map projection type.
    pub projection: MapProjection,
    /// Enable interactive hover/click.
    pub interactive: bool,
    /// Minimum value for color scaling.
    pub min_value: f64,
    /// Maximum value for color scaling.
    pub max_value: f64,
    /// Show base world map layer.
    pub show_base_map: bool,
    /// Base hex size (defaults to hex_size if None).
    pub base_hex_size: Option<f64>,
}

impl Default for HexWorldMapConfig {
    fn default() -> Self {
        Self {
            width: 900,
            height: 500,
            hex_size: 8.0,
            hex_orientation: HexOrientation::PointyTop,
            color_scale: ColorScale::PurpleToOrange,
            show_grid: false,
            show_labels: true,
            show_legend: true,
            projection: MapProjection::Mercator,
            interactive: true,
            min_value: 0.0,
            max_value: 100.0,
            show_base_map: true,
            base_hex_size: None,
        }
    }
}

// ============================================================================
// Progress Ring Types
// ============================================================================

/// Progress ring data.
#[derive(Debug, Clone)]
pub struct ProgressRingData {
    /// Current value.
    pub value: f64,
    /// Maximum value (default 100).
    pub max: f64,
    /// Label text shown below percentage.
    pub label: String,
    /// Color for the ring.
    pub color: Option<ChartColor>,
    /// Sublabel text.
    pub sublabel: Option<String>,
}

impl ProgressRingData {
    /// Create a new progress ring data.
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            max: 100.0,
            label: label.into(),
            color: None,
            sublabel: None,
        }
    }

    /// Set maximum value.
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// Set color.
    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Set sublabel.
    pub fn sublabel(mut self, sublabel: impl Into<String>) -> Self {
        self.sublabel = Some(sublabel.into());
        self
    }

    /// Get percentage (0.0 to 1.0).
    pub fn percentage(&self) -> f64 {
        if self.max == 0.0 {
            0.0
        } else {
            (self.value / self.max).clamp(0.0, 1.0)
        }
    }
}

/// Configuration for ProgressRing.
#[derive(Debug, Clone)]
pub struct ProgressRingConfig {
    /// Diameter in pixels.
    pub size: u32,
    /// Ring thickness.
    pub stroke_width: f64,
    /// Show percentage in center.
    pub show_percentage: bool,
    /// Show label below percentage.
    pub show_label: bool,
    /// Animate on value change.
    pub animate: bool,
    /// Start angle in degrees (-90 = top).
    pub start_angle: f64,
    /// Background ring opacity.
    pub background_opacity: f64,
}

impl Default for ProgressRingConfig {
    fn default() -> Self {
        Self {
            size: 120,
            stroke_width: 10.0,
            show_percentage: true,
            show_label: true,
            animate: true,
            start_angle: -90.0,
            background_opacity: 0.2,
        }
    }
}

// ============================================================================
// Arc Flow Lines Types
// ============================================================================

/// A point for arc connections.
#[derive(Debug, Clone)]
pub struct ArcPoint {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Optional label.
    pub label: Option<String>,
}

impl ArcPoint {
    /// Create a new arc point.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, label: None }
    }

    /// Create with label.
    pub fn with_label(x: f64, y: f64, label: impl Into<String>) -> Self {
        Self {
            x,
            y,
            label: Some(label.into()),
        }
    }
}

/// Arc connection between two points.
#[derive(Debug, Clone)]
pub struct ArcConnection {
    /// Unique identifier.
    pub id: String,
    /// Source point.
    pub source: ArcPoint,
    /// Target point.
    pub target: ArcPoint,
    /// Optional value for line thickness.
    pub value: Option<f64>,
    /// Optional label.
    pub label: Option<String>,
    /// Line color.
    pub color: Option<ChartColor>,
    /// Line style.
    pub style: ArcStyle,
}

impl ArcConnection {
    /// Create a new arc connection.
    pub fn new(
        id: impl Into<String>,
        source: ArcPoint,
        target: ArcPoint,
    ) -> Self {
        Self {
            id: id.into(),
            source,
            target,
            value: None,
            label: None,
            color: None,
            style: ArcStyle::Solid,
        }
    }

    /// Set value.
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set color.
    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Set style.
    pub fn style(mut self, style: ArcStyle) -> Self {
        self.style = style;
        self
    }
}

/// Arc line style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArcStyle {
    /// Solid line.
    #[default]
    Solid,
    /// Dashed line.
    Dashed,
    /// Animated flowing dashes.
    Animated,
    /// Pulsing opacity.
    Pulse,
}

impl ArcStyle {
    /// Get CSS class suffix.
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Animated => "animated",
            Self::Pulse => "pulse",
        }
    }

    /// Get stroke-dasharray value.
    pub fn dash_array(&self) -> &'static str {
        match self {
            Self::Solid => "",
            Self::Dashed | Self::Animated => "8 4",
            Self::Pulse => "",
        }
    }
}

/// Configuration for ArcFlowLines.
#[derive(Debug, Clone)]
pub struct ArcFlowConfig {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Line thickness.
    pub stroke_width: f64,
    /// Curvature amount (0.0 = straight, 1.0 = very curved).
    pub curvature: f64,
    /// Show endpoint markers.
    pub show_endpoints: bool,
    /// Animation duration in ms.
    pub animate_duration: u32,
    /// Show labels on arcs.
    pub show_labels: bool,
}

impl Default for ArcFlowConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 400,
            stroke_width: 2.0,
            curvature: 0.5,
            show_endpoints: true,
            animate_duration: 1000,
            show_labels: false,
        }
    }
}

// ============================================================================
// Gradient Progress Bar Types
// ============================================================================

/// Progress bar segment.
#[derive(Debug, Clone)]
pub struct ProgressSegment {
    /// Segment value.
    pub value: f64,
    /// Segment color (CSS color string).
    pub color: Option<String>,
    /// Segment label.
    pub label: Option<String>,
}

impl ProgressSegment {
    /// Create a new segment.
    pub fn new(value: f64) -> Self {
        Self {
            value,
            color: None,
            label: None,
        }
    }

    /// Set color.
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Configuration for GradientProgressBar.
#[derive(Debug, Clone)]
pub struct GradientProgressBarConfig {
    /// Height in pixels.
    pub height: u32,
    /// Show segment labels.
    pub show_labels: bool,
    /// Show current value.
    pub show_value: bool,
    /// Gradient colors (for single-value mode).
    pub gradient_colors: Vec<String>,
    /// Border radius.
    pub border_radius: f64,
    /// Animate on value change.
    pub animate: bool,
}

impl Default for GradientProgressBarConfig {
    fn default() -> Self {
        Self {
            height: 8,
            show_labels: false,
            show_value: true,
            gradient_colors: vec![
                "#52c41a".to_string(),
                "#1890ff".to_string(),
                "#722ed1".to_string(),
            ],
            border_radius: 4.0,
            animate: true,
        }
    }
}

// ============================================================================
// City Marker Types
// ============================================================================

/// City marker data.
#[derive(Debug, Clone)]
pub struct CityMarkerData {
    /// Unique identifier.
    pub id: String,
    /// City/location name.
    pub name: String,
    /// Display value (formatted string).
    pub value: String,
    /// Icon type.
    pub icon: MarkerIcon,
    /// Marker color.
    pub color: Option<ChartColor>,
    /// Position type.
    pub position: MarkerPosition,
}

impl CityMarkerData {
    /// Create a new city marker.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        position: MarkerPosition,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            value: value.into(),
            icon: MarkerIcon::Building,
            color: None,
            position,
        }
    }

    /// Set icon.
    pub fn icon(mut self, icon: MarkerIcon) -> Self {
        self.icon = icon;
        self
    }

    /// Set color.
    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = Some(color);
        self
    }
}

/// Marker icon type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkerIcon {
    #[default]
    Building,
    Flag,
    Pin,
    Dot,
}

impl MarkerIcon {
    /// Get SVG path.
    pub fn svg_path(&self) -> &'static str {
        match self {
            Self::Building => "M3 21h18v-2H3v2zm0-4h18v-2H3v2zm0-4h18v-2H3v2zm0-4h18V7H3v2zm0-6v2h18V3H3z",
            Self::Flag => "M14.4 6L14 4H5v17h2v-7h5.6l.4 2h7V6z",
            Self::Pin => "M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z",
            Self::Dot => "M12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4z",
        }
    }
}

/// Marker position type.
#[derive(Debug, Clone)]
pub enum MarkerPosition {
    /// Absolute pixel coordinates.
    Absolute { x: f64, y: f64 },
    /// Geographic coordinates.
    LatLng { lat: f64, lng: f64 },
}

impl MarkerPosition {
    /// Create absolute position.
    pub fn absolute(x: f64, y: f64) -> Self {
        Self::Absolute { x, y }
    }

    /// Create lat/lng position.
    pub fn lat_lng(lat: f64, lng: f64) -> Self {
        Self::LatLng { lat, lng }
    }
}

/// Marker size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkerSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl MarkerSize {
    /// Get CSS class suffix.
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
        }
    }
}

/// Configuration for CityMarker.
#[derive(Debug, Clone)]
pub struct CityMarkerConfig {
    /// Marker size.
    pub size: MarkerSize,
    /// Show connector line to point.
    pub show_connector: bool,
    /// Expand on hover.
    pub hover_expand: bool,
}

impl Default for CityMarkerConfig {
    fn default() -> Self {
        Self {
            size: MarkerSize::Medium,
            show_connector: false,
            hover_expand: true,
        }
    }
}

// ============================================================================
// Activity List Types
// ============================================================================

/// Activity list item.
#[derive(Debug, Clone)]
pub struct ActivityItem {
    /// Unique identifier.
    pub id: String,
    /// Item name (e.g., country name).
    pub name: String,
    /// Numeric value.
    pub value: f64,
    /// Percentage (0-100).
    pub percentage: f64,
    /// Icon or flag (emoji or icon name).
    pub icon: Option<String>,
    /// Item color.
    pub color: Option<ChartColor>,
    /// Trend direction.
    pub trend: Option<TrendDirection>,
}

impl ActivityItem {
    /// Create a new activity item.
    pub fn new(id: impl Into<String>, name: impl Into<String>, value: f64, percentage: f64) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            value,
            percentage,
            icon: None,
            color: None,
            trend: None,
        }
    }

    /// Set icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set color.
    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Set trend.
    pub fn trend(mut self, trend: TrendDirection) -> Self {
        self.trend = Some(trend);
        self
    }
}

/// Trend direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrendDirection {
    Up,
    Down,
    Neutral,
}

impl TrendDirection {
    /// Get CSS class suffix.
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Neutral => "neutral",
        }
    }

    /// Get arrow character.
    pub fn arrow(&self) -> &'static str {
        match self {
            Self::Up => "↑",
            Self::Down => "↓",
            Self::Neutral => "→",
        }
    }

    /// Get color.
    pub fn color(&self) -> &'static str {
        match self {
            Self::Up => "var(--fx-color-success, #52c41a)",
            Self::Down => "var(--fx-color-error, #ff4d4f)",
            Self::Neutral => "var(--fx-color-text-secondary)",
        }
    }
}

/// Sort order for activity list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActivitySortOrder {
    #[default]
    ValueDesc,
    ValueAsc,
    Alphabetical,
    PercentageDesc,
}

/// Configuration for ActivityList.
#[derive(Debug, Clone)]
pub struct ActivityListConfig {
    /// Show icons/flags.
    pub show_icons: bool,
    /// Show progress bars.
    pub show_progress_bars: bool,
    /// Show percentages.
    pub show_percentages: bool,
    /// Maximum items to display.
    pub max_items: Option<usize>,
    /// Enable sorting.
    pub sortable: bool,
    /// Default sort order.
    pub sort_order: ActivitySortOrder,
    /// Animate on update.
    pub animated: bool,
}

impl Default for ActivityListConfig {
    fn default() -> Self {
        Self {
            show_icons: true,
            show_progress_bars: true,
            show_percentages: true,
            max_items: None,
            sortable: false,
            sort_order: ActivitySortOrder::ValueDesc,
            animated: true,
        }
    }
}

// ============================================================================
// Geometry Utilities
// ============================================================================

/// Generate hexagon SVG path.
pub fn hexagon_path(cx: f64, cy: f64, size: f64, orientation: HexOrientation) -> String {
    let angles: [f64; 6] = match orientation {
        HexOrientation::PointyTop => [30.0, 90.0, 150.0, 210.0, 270.0, 330.0],
        HexOrientation::FlatTop => [0.0, 60.0, 120.0, 180.0, 240.0, 300.0],
    };

    let points: Vec<String> = angles
        .iter()
        .map(|&angle| {
            let rad = angle * PI / 180.0;
            format!("{:.2},{:.2}", cx + size * rad.cos(), cy + size * rad.sin())
        })
        .collect();

    format!("M{} Z", points.join(" L"))
}

/// Convert lat/lng to pixel coordinates (Mercator projection).
pub fn lat_lng_to_pixel(lat: f64, lng: f64, width: f64, height: f64) -> (f64, f64) {
    let x = (lng + 180.0) * (width / 360.0);
    let lat_rad = lat * PI / 180.0;
    let merc_n = (PI / 4.0 + lat_rad / 2.0).tan().ln();
    let y = (height / 2.0) - (height * merc_n / (2.0 * PI));
    (x, y)
}

/// Convert hex grid coordinates to pixel coordinates.
pub fn hex_to_pixel(col: i32, row: i32, size: f64, orientation: HexOrientation) -> (f64, f64) {
    match orientation {
        HexOrientation::PointyTop => {
            let x = size * 3.0_f64.sqrt() * (col as f64 + 0.5 * (row & 1) as f64);
            let y = size * 1.5 * row as f64;
            (x, y)
        }
        HexOrientation::FlatTop => {
            let x = size * 1.5 * col as f64;
            let y = size * 3.0_f64.sqrt() * (row as f64 + 0.5 * (col & 1) as f64);
            (x, y)
        }
    }
}

/// Generate quadratic bezier arc path.
pub fn arc_connection_path(x1: f64, y1: f64, x2: f64, y2: f64, curvature: f64) -> String {
    // Calculate control point (perpendicular to midpoint)
    let mx = (x1 + x2) / 2.0;
    let my = (y1 + y2) / 2.0;

    let dx = x2 - x1;
    let dy = y2 - y1;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist < 0.001 {
        return format!("M{:.2},{:.2} L{:.2},{:.2}", x1, y1, x2, y2);
    }

    // Perpendicular offset for curve
    let offset = dist * curvature * 0.3;
    let cx = mx - dy / dist * offset;
    let cy = my + dx / dist * offset;

    format!("M{:.2},{:.2} Q{:.2},{:.2} {:.2},{:.2}", x1, y1, cx, cy, x2, y2)
}

/// Generate circular arc path for progress rings.
pub fn progress_arc_path(
    cx: f64,
    cy: f64,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> String {
    let start_rad = start_angle * PI / 180.0;
    let end_rad = end_angle * PI / 180.0;

    let x1 = cx + radius * start_rad.cos();
    let y1 = cy + radius * start_rad.sin();
    let x2 = cx + radius * end_rad.cos();
    let y2 = cy + radius * end_rad.sin();

    let angle_diff = (end_angle - start_angle).abs();
    let large_arc = if angle_diff > 180.0 { 1 } else { 0 };
    let sweep = if end_angle > start_angle { 1 } else { 0 };

    format!(
        "M{:.2},{:.2} A{:.2},{:.2} 0 {} {} {:.2},{:.2}",
        x1, y1, radius, radius, large_arc, sweep, x2, y2
    )
}

/// Format large numbers with commas.
pub fn format_number(value: f64) -> String {
    let s = format!("{:.0}", value);
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }
    result
}
