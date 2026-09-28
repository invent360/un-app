//! Pre-computed world map hexagon data.
//!
//! This module contains hexagon coordinate data for rendering a world map
//! using a dot-matrix style similar to the ORION dashboard.

use super::types::HexOrientation;
use std::f64::consts::PI;

/// Simplified continent polygon data.
/// Each continent is defined by a series of (latitude, longitude) points.
/// These are simplified outlines for visual representation.
const NORTH_AMERICA: &[(f64, f64)] = &[
    // Alaska
    (70.0, -170.0), (70.0, -141.0), (68.0, -141.0), (60.0, -141.0),
    (60.0, -135.0), (56.0, -130.0), (54.0, -130.0), (48.0, -125.0),
    // West coast
    (40.0, -125.0), (32.0, -117.0), (25.0, -110.0), (22.0, -105.0),
    // Mexico
    (20.0, -105.0), (18.0, -95.0), (20.0, -90.0), (22.0, -88.0),
    (25.0, -80.0),
    // Florida
    (25.0, -80.0), (30.0, -82.0), (30.0, -85.0),
    // Gulf coast
    (30.0, -90.0), (30.0, -95.0),
    // East coast
    (35.0, -77.0), (40.0, -74.0), (42.0, -70.0), (45.0, -67.0),
    (47.0, -68.0), (50.0, -65.0), (52.0, -56.0), (60.0, -64.0),
    // Labrador
    (60.0, -64.0), (55.0, -77.0), (55.0, -95.0),
    // Hudson Bay return
    (60.0, -95.0), (65.0, -90.0), (70.0, -100.0), (75.0, -95.0),
    // Arctic Canada
    (75.0, -115.0), (70.0, -130.0), (70.0, -140.0),
];

const SOUTH_AMERICA: &[(f64, f64)] = &[
    // Colombia/Venezuela
    (12.0, -72.0), (10.0, -75.0), (5.0, -77.0), (0.0, -80.0),
    // Ecuador/Peru
    (-5.0, -81.0), (-10.0, -78.0), (-15.0, -75.0), (-20.0, -70.0),
    // Chile
    (-25.0, -70.0), (-30.0, -71.0), (-35.0, -71.0), (-40.0, -73.0),
    (-45.0, -74.0), (-50.0, -74.0), (-55.0, -68.0),
    // Tierra del Fuego
    (-55.0, -67.0),
    // Argentina/Uruguay/Brazil east coast
    (-50.0, -65.0), (-40.0, -62.0), (-35.0, -57.0), (-30.0, -51.0),
    (-25.0, -48.0), (-20.0, -40.0), (-15.0, -39.0), (-10.0, -36.0),
    (-5.0, -35.0), (0.0, -50.0), (5.0, -52.0), (10.0, -62.0),
];

const EUROPE: &[(f64, f64)] = &[
    // Iberia
    (36.0, -10.0), (37.0, -7.0), (42.0, -9.0), (44.0, -8.0),
    // France
    (48.0, -5.0), (50.0, 2.0), (51.0, 4.0),
    // North Sea coast
    (54.0, 8.0), (56.0, 8.0), (58.0, 10.0),
    // Scandinavia
    (63.0, 10.0), (68.0, 16.0), (70.0, 25.0), (70.0, 30.0),
    // Finland/Russia border
    (65.0, 30.0), (60.0, 30.0), (55.0, 20.0), (54.0, 14.0),
    // Baltic
    (55.0, 14.0), (54.0, 10.0),
    // Germany/Poland
    (52.0, 6.0), (50.0, 5.0),
    // Back to Iberia
    (45.0, 0.0), (43.0, -2.0), (36.0, -6.0),
];

const AFRICA: &[(f64, f64)] = &[
    // Morocco
    (36.0, -6.0), (35.0, -1.0), (37.0, 10.0),
    // Tunisia/Libya
    (37.0, 10.0), (33.0, 12.0), (32.0, 25.0), (31.0, 35.0),
    // Egypt
    (30.0, 35.0), (22.0, 37.0),
    // East coast
    (15.0, 43.0), (10.0, 51.0), (0.0, 42.0), (-5.0, 40.0),
    (-12.0, 44.0), (-25.0, 35.0), (-34.0, 26.0),
    // South Africa
    (-35.0, 20.0), (-34.0, 18.0),
    // West coast
    (-30.0, 16.0), (-20.0, 13.0), (-5.0, 10.0), (5.0, 0.0),
    (5.0, -5.0), (10.0, -15.0), (15.0, -17.0), (20.0, -17.0),
    (28.0, -13.0), (36.0, -6.0),
];

const ASIA: &[(f64, f64)] = &[
    // Russia west
    (70.0, 30.0), (72.0, 55.0), (72.0, 80.0), (75.0, 100.0),
    (72.0, 130.0), (70.0, 170.0), (65.0, -170.0), (65.0, -168.0),
    // Russia east/Kamchatka
    (60.0, 165.0), (55.0, 163.0), (50.0, 155.0), (45.0, 145.0),
    // Japan area (skipped - islands)
    (40.0, 130.0), (35.0, 120.0), (25.0, 120.0), (22.0, 115.0),
    // SE Asia
    (20.0, 110.0), (15.0, 108.0), (10.0, 105.0), (8.0, 98.0),
    // India
    (8.0, 77.0), (10.0, 78.0), (15.0, 80.0), (20.0, 72.0),
    (25.0, 68.0), (28.0, 65.0),
    // Middle East
    (25.0, 57.0), (28.0, 50.0), (30.0, 48.0), (37.0, 40.0),
    // Turkey
    (42.0, 45.0), (42.0, 30.0), (36.0, 35.0), (32.0, 35.0),
    (30.0, 35.0),
    // Back through Russia
    (45.0, 40.0), (55.0, 40.0), (60.0, 30.0), (70.0, 30.0),
];

const AUSTRALIA: &[(f64, f64)] = &[
    // North coast
    (-12.0, 130.0), (-12.0, 135.0), (-15.0, 140.0), (-20.0, 149.0),
    // East coast
    (-25.0, 153.0), (-30.0, 153.0), (-35.0, 151.0), (-38.0, 147.0),
    // South coast
    (-38.0, 140.0), (-35.0, 135.0), (-35.0, 130.0), (-32.0, 125.0),
    // West coast
    (-25.0, 113.0), (-22.0, 114.0), (-18.0, 122.0), (-15.0, 130.0),
    (-12.0, 130.0),
];

/// Point-in-polygon test using ray casting algorithm.
fn point_in_polygon(x: f64, y: f64, polygon: &[(f64, f64)]) -> bool {
    let n = polygon.len();
    if n < 3 {
        return false;
    }

    let mut inside = false;
    let mut j = n - 1;

    for i in 0..n {
        let (yi, xi) = polygon[i]; // Note: polygon is (lat, lng), we treat lat as y, lng as x
        let (yj, xj) = polygon[j];

        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }

    inside
}

/// Check if a latitude/longitude point is on land.
fn is_land(lat: f64, lng: f64) -> bool {
    // Check each continent
    point_in_polygon(lng, lat, NORTH_AMERICA)
        || point_in_polygon(lng, lat, SOUTH_AMERICA)
        || point_in_polygon(lng, lat, EUROPE)
        || point_in_polygon(lng, lat, AFRICA)
        || point_in_polygon(lng, lat, ASIA)
        || point_in_polygon(lng, lat, AUSTRALIA)
}

/// Get region name based on coordinates.
fn get_region_for_point(lat: f64, lng: f64) -> &'static str {
    if point_in_polygon(lng, lat, NORTH_AMERICA) {
        return "north_america";
    }
    if point_in_polygon(lng, lat, SOUTH_AMERICA) {
        return "south_america";
    }
    if point_in_polygon(lng, lat, EUROPE) {
        return "europe";
    }
    if point_in_polygon(lng, lat, AFRICA) {
        return "africa";
    }
    if point_in_polygon(lng, lat, ASIA) {
        return "asia";
    }
    if point_in_polygon(lng, lat, AUSTRALIA) {
        return "australia";
    }
    "unknown"
}

/// Convert latitude/longitude to pixel coordinates using Mercator projection.
fn lat_lng_to_pixel(lat: f64, lng: f64, width: f64, height: f64) -> (f64, f64) {
    // Normalize longitude to 0-1 range
    let x = (lng + 180.0) / 360.0 * width;

    // Mercator projection for latitude
    let lat_rad = lat * PI / 180.0;
    let merc_n = (PI / 4.0 + lat_rad / 2.0).tan().ln();
    let y = (height / 2.0) - (height * merc_n / (2.0 * PI));

    // Clamp to bounds
    (x.clamp(0.0, width), y.clamp(0.0, height))
}

/// Generate hexagon center positions for the world map base layer.
/// Returns Vec of (x, y, region_hint) for each hexagon.
pub fn generate_world_hexagons(
    width: f64,
    height: f64,
    hex_size: f64,
    orientation: HexOrientation,
) -> Vec<(f64, f64, &'static str)> {
    let mut hexagons = Vec::new();

    // Calculate hex spacing for proper tessellation
    let (h_step, v_step) = match orientation {
        HexOrientation::PointyTop => (hex_size * 3.0_f64.sqrt(), hex_size * 1.5),
        HexOrientation::FlatTop => (hex_size * 1.5, hex_size * 3.0_f64.sqrt()),
    };

    // Generate a hex grid
    let hex_cols = (width / h_step).ceil() as i32 + 1;
    let hex_rows = (height / v_step).ceil() as i32 + 1;

    for row in 0..hex_rows {
        for col in 0..hex_cols {
            // Calculate hex center position with offset for odd rows
            let (px, py) = match orientation {
                HexOrientation::PointyTop => {
                    let offset = if row % 2 == 1 { h_step * 0.5 } else { 0.0 };
                    (col as f64 * h_step + offset, row as f64 * v_step)
                }
                HexOrientation::FlatTop => {
                    let offset = if col % 2 == 1 { v_step * 0.5 } else { 0.0 };
                    (col as f64 * h_step, row as f64 * v_step + offset)
                }
            };

            // Skip if outside bounds
            if px < 0.0 || px >= width || py < 0.0 || py >= height {
                continue;
            }

            // Convert pixel to lat/lng (inverse Mercator)
            let lng = (px / width) * 360.0 - 180.0;
            let merc_y = (height / 2.0 - py) * 2.0 * PI / height;
            let lat = (2.0 * merc_y.exp().atan() - PI / 2.0) * 180.0 / PI;

            // Check if this position is land
            if is_land(lat, lng) {
                let region = get_region_for_point(lat, lng);
                hexagons.push((px, py, region));
            }
        }
    }

    hexagons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_land() {
        // New York should be land
        assert!(is_land(40.7, -74.0), "New York should be land");
        // London should be land
        assert!(is_land(51.5, -0.1), "London should be land");
        // Middle of Atlantic should be ocean
        assert!(!is_land(40.0, -40.0), "Mid-Atlantic should be ocean");
        // Middle of Pacific should be ocean
        assert!(!is_land(0.0, -150.0), "Mid-Pacific should be ocean");
        // Sydney should be land
        assert!(is_land(-34.0, 151.0), "Sydney should be land");
    }

    #[test]
    fn test_generate_hexagons() {
        let hexagons = generate_world_hexagons(900.0, 450.0, 5.0, HexOrientation::PointyTop);
        assert!(!hexagons.is_empty(), "Should generate hexagons");
        // Verify all hexagons are within bounds
        for (x, y, _) in &hexagons {
            assert!(*x >= 0.0 && *x < 900.0, "X out of bounds");
            assert!(*y >= 0.0 && *y < 450.0, "Y out of bounds");
        }
    }
}
