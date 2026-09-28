//! Chart utilities for SVG path generation, scaling, and layout calculations.

use std::f64::consts::PI;
use crate::try_use_theme as crate_try_use_theme;
use crate::ThemeContext;

// =============================================================================
// Theme and Prefix Helpers
// =============================================================================

/// Generate a CSS class prefix for chart components.
pub fn chart_prefix(chart_type: &str) -> String {
    format!("fx-chart-{}", chart_type)
}

/// Try to get the current theme context.
/// Returns Some if a theme provider is available.
pub fn try_use_theme() -> Option<ThemeContext> {
    crate_try_use_theme()
}

// =============================================================================
// SVG Path Builders
// =============================================================================

/// Generate a line path from points.
pub fn line_path(points: &[(f64, f64)]) -> String {
    if points.is_empty() {
        return String::new();
    }

    let mut path = String::with_capacity(points.len() * 20);
    for (i, (x, y)) in points.iter().enumerate() {
        if i == 0 {
            path.push_str(&format!("M{:.2},{:.2}", x, y));
        } else {
            path.push_str(&format!(" L{:.2},{:.2}", x, y));
        }
    }
    path
}

/// Generate a smooth bezier curve path through points.
pub fn smooth_path(points: &[(f64, f64)]) -> String {
    if points.len() < 2 {
        return line_path(points);
    }

    let mut path = String::with_capacity(points.len() * 40);
    path.push_str(&format!("M{:.2},{:.2}", points[0].0, points[0].1));

    for i in 0..points.len() - 1 {
        let p0 = if i > 0 { points[i - 1] } else { points[i] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i + 2 < points.len() { points[i + 2] } else { p2 };

        // Catmull-Rom to Bezier conversion
        let tension = 0.5;
        let cp1x = p1.0 + (p2.0 - p0.0) * tension / 3.0;
        let cp1y = p1.1 + (p2.1 - p0.1) * tension / 3.0;
        let cp2x = p2.0 - (p3.0 - p1.0) * tension / 3.0;
        let cp2y = p2.1 - (p3.1 - p1.1) * tension / 3.0;

        path.push_str(&format!(
            " C{:.2},{:.2} {:.2},{:.2} {:.2},{:.2}",
            cp1x, cp1y, cp2x, cp2y, p2.0, p2.1
        ));
    }
    path
}

/// Step type for step charts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepType {
    /// Step before the point.
    Before,
    /// Step after the point.
    #[default]
    After,
    /// Step at midpoint.
    Middle,
}

/// Generate a stepped line path.
pub fn step_path(points: &[(f64, f64)], step_type: StepType) -> String {
    if points.is_empty() {
        return String::new();
    }

    let mut path = String::with_capacity(points.len() * 30);
    path.push_str(&format!("M{:.2},{:.2}", points[0].0, points[0].1));

    for i in 1..points.len() {
        let (x1, y1) = points[i - 1];
        let (x2, y2) = points[i];

        match step_type {
            StepType::Before => {
                path.push_str(&format!(" V{:.2} H{:.2}", y2, x2));
            }
            StepType::After => {
                path.push_str(&format!(" H{:.2} V{:.2}", x2, y2));
            }
            StepType::Middle => {
                let mid_x = (x1 + x2) / 2.0;
                path.push_str(&format!(" H{:.2} V{:.2} H{:.2}", mid_x, y2, x2));
            }
        }
    }
    path
}

/// Generate an area path (line with fill back to baseline).
pub fn area_path(points: &[(f64, f64)], baseline: f64) -> String {
    if points.is_empty() {
        return String::new();
    }

    let mut path = line_path(points);
    if let Some((last_x, _)) = points.last() {
        path.push_str(&format!(" L{:.2},{:.2}", last_x, baseline));
    }
    if let Some((first_x, _)) = points.first() {
        path.push_str(&format!(" L{:.2},{:.2}", first_x, baseline));
    }
    path.push_str(" Z");
    path
}

/// Generate a smooth area path.
pub fn smooth_area_path(points: &[(f64, f64)], baseline: f64) -> String {
    if points.is_empty() {
        return String::new();
    }

    let mut path = smooth_path(points);
    if let Some((last_x, _)) = points.last() {
        path.push_str(&format!(" L{:.2},{:.2}", last_x, baseline));
    }
    if let Some((first_x, _)) = points.first() {
        path.push_str(&format!(" L{:.2},{:.2}", first_x, baseline));
    }
    path.push_str(" Z");
    path
}

/// Generate an arc path segment.
pub fn arc_path(
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

    let large_arc = if (end_angle - start_angle).abs() > 180.0 { 1 } else { 0 };
    let sweep = if end_angle > start_angle { 1 } else { 0 };

    format!(
        "M{:.2},{:.2} A{:.2},{:.2} 0 {} {} {:.2},{:.2}",
        x1, y1, radius, radius, large_arc, sweep, x2, y2
    )
}

/// Generate a pie slice path.
pub fn pie_slice_path(
    cx: f64,
    cy: f64,
    outer_radius: f64,
    inner_radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> String {
    let start_rad = start_angle * PI / 180.0;
    let end_rad = end_angle * PI / 180.0;

    // Outer arc points
    let outer_x1 = cx + outer_radius * start_rad.cos();
    let outer_y1 = cy + outer_radius * start_rad.sin();
    let outer_x2 = cx + outer_radius * end_rad.cos();
    let outer_y2 = cy + outer_radius * end_rad.sin();

    let large_arc = if (end_angle - start_angle).abs() > 180.0 { 1 } else { 0 };

    if inner_radius > 0.0 {
        // Donut slice
        let inner_x1 = cx + inner_radius * start_rad.cos();
        let inner_y1 = cy + inner_radius * start_rad.sin();
        let inner_x2 = cx + inner_radius * end_rad.cos();
        let inner_y2 = cy + inner_radius * end_rad.sin();

        format!(
            "M{:.2},{:.2} A{:.2},{:.2} 0 {} 1 {:.2},{:.2} L{:.2},{:.2} A{:.2},{:.2} 0 {} 0 {:.2},{:.2} Z",
            outer_x1, outer_y1,
            outer_radius, outer_radius, large_arc,
            outer_x2, outer_y2,
            inner_x2, inner_y2,
            inner_radius, inner_radius, large_arc,
            inner_x1, inner_y1
        )
    } else {
        // Full pie slice
        format!(
            "M{:.2},{:.2} L{:.2},{:.2} A{:.2},{:.2} 0 {} 1 {:.2},{:.2} Z",
            cx, cy,
            outer_x1, outer_y1,
            outer_radius, outer_radius, large_arc,
            outer_x2, outer_y2
        )
    }
}

/// Generate a polygon path (for radar charts).
pub fn polygon_path(points: &[(f64, f64)], close: bool) -> String {
    if points.is_empty() {
        return String::new();
    }

    let mut path = line_path(points);
    if close {
        path.push_str(" Z");
    }
    path
}

// =============================================================================
// Scaling Functions
// =============================================================================

/// Linear scale: maps a value from domain to range.
pub fn scale_linear(domain: (f64, f64), range: (f64, f64), value: f64) -> f64 {
    let (d_min, d_max) = domain;
    let (r_min, r_max) = range;

    if (d_max - d_min).abs() < f64::EPSILON {
        return r_min;
    }

    let t = (value - d_min) / (d_max - d_min);
    r_min + t * (r_max - r_min)
}

/// Time scale: maps a timestamp to range.
pub fn scale_time(domain: (i64, i64), range: (f64, f64), value: i64) -> f64 {
    scale_linear(
        (domain.0 as f64, domain.1 as f64),
        range,
        value as f64,
    )
}

/// Band scale for categories (returns start and width).
pub fn scale_band(
    category_count: usize,
    range: (f64, f64),
    index: usize,
    padding: f32,
) -> (f64, f64) {
    if category_count == 0 {
        return (range.0, 0.0);
    }

    let total_width = range.1 - range.0;
    let padding_ratio = padding as f64;

    // Calculate band width with padding
    let step = total_width / category_count as f64;
    let band_width = step * (1.0 - padding_ratio);
    let offset = step * padding_ratio / 2.0;

    let x = range.0 + step * index as f64 + offset;
    (x, band_width)
}

/// Clamp value to range.
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

// =============================================================================
// Axis Helpers
// =============================================================================

/// Generate nice tick values for an axis.
pub fn generate_ticks(min: f64, max: f64, count: usize) -> Vec<f64> {
    if count == 0 || min >= max {
        return vec![];
    }

    let range = max - min;
    let rough_step = range / count as f64;

    // Find a "nice" step value (1, 2, 5, 10, 20, 50, etc.)
    let magnitude = 10_f64.powf(rough_step.log10().floor());
    let residual = rough_step / magnitude;

    let nice_step = if residual <= 1.0 {
        magnitude
    } else if residual <= 2.0 {
        2.0 * magnitude
    } else if residual <= 5.0 {
        5.0 * magnitude
    } else {
        10.0 * magnitude
    };

    // Generate ticks
    let mut ticks = Vec::new();
    let start = (min / nice_step).ceil() * nice_step;
    let mut tick = start;

    while tick <= max + nice_step * 0.001 {
        ticks.push(tick);
        tick += nice_step;
    }

    ticks
}

/// Tick format types.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TickFormat {
    /// Default numeric format.
    #[default]
    Number,
    /// Percentage format.
    Percent,
    /// Compact format (K, M, B).
    Compact,
    /// Currency format.
    Currency,
}

/// Format a tick label.
pub fn format_tick_label(value: f64, format: TickFormat) -> String {
    match format {
        TickFormat::Number => {
            if value.abs() < 0.01 || value.abs() >= 10000.0 {
                format!("{:.2e}", value)
            } else if value.fract().abs() < 0.001 {
                format!("{:.0}", value)
            } else {
                format!("{:.2}", value)
            }
        }
        TickFormat::Percent => format!("{:.0}%", value),
        TickFormat::Compact => {
            let abs = value.abs();
            if abs >= 1_000_000_000.0 {
                format!("{:.1}B", value / 1_000_000_000.0)
            } else if abs >= 1_000_000.0 {
                format!("{:.1}M", value / 1_000_000.0)
            } else if abs >= 1_000.0 {
                format!("{:.1}K", value / 1_000.0)
            } else {
                format!("{:.0}", value)
            }
        }
        TickFormat::Currency => {
            let abs = value.abs();
            if abs >= 1_000_000.0 {
                format!("${:.2}M", value / 1_000_000.0)
            } else if abs >= 1_000.0 {
                format!("${:.2}K", value / 1_000.0)
            } else {
                format!("${:.2}", value)
            }
        }
    }
}

/// Time format types.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TimeFormat {
    /// Time only (HH:MM).
    Time,
    /// Date only (MM/DD).
    Date,
    /// Full datetime.
    #[default]
    DateTime,
    /// Month and year.
    MonthYear,
}

/// Format a timestamp label.
pub fn format_time_label(timestamp: i64, format: TimeFormat) -> String {
    // Convert milliseconds to seconds for date calculation
    let secs = timestamp / 1000;

    // Simple date formatting (in production, use chrono or similar)
    // This is a basic implementation for demo purposes
    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;

    // Approximate month/day (simplified, not accounting for leap years etc)
    let year = 1970 + (days_since_epoch / 365);
    let day_of_year = days_since_epoch % 365;
    let month = (day_of_year / 30) + 1;
    let day = (day_of_year % 30) + 1;

    match format {
        TimeFormat::Time => format!("{:02}:{:02}", hours, minutes),
        TimeFormat::Date => format!("{:02}/{:02}", month, day),
        TimeFormat::DateTime => format!("{:02}/{:02} {:02}:{:02}", month, day, hours, minutes),
        TimeFormat::MonthYear => {
            let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                         "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
            let m = ((month - 1) % 12) as usize;
            format!("{} {}", months[m], year)
        }
    }
}

// =============================================================================
// Layout Calculations
// =============================================================================

/// Chart margins.
#[derive(Debug, Clone, Copy, Default)]
pub struct Margins {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl Margins {
    /// Create uniform margins.
    pub fn uniform(value: f64) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create asymmetric margins.
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self { top, right, bottom, left }
    }

    /// Default margins for charts with axis.
    pub fn with_axis() -> Self {
        Self {
            top: 20.0,
            right: 20.0,
            bottom: 40.0,
            left: 50.0,
        }
    }
}

/// Calculated chart drawing area.
#[derive(Debug, Clone, Copy)]
pub struct ChartArea {
    /// Inner X start.
    pub x: f64,
    /// Inner Y start.
    pub y: f64,
    /// Inner width.
    pub width: f64,
    /// Inner height.
    pub height: f64,
}

impl ChartArea {
    /// Calculate chart area from total dimensions and margins.
    pub fn from_size(width: f64, height: f64, margins: Margins) -> Self {
        Self {
            x: margins.left,
            y: margins.top,
            width: (width - margins.left - margins.right).max(0.0),
            height: (height - margins.top - margins.bottom).max(0.0),
        }
    }

    /// Get the right edge x coordinate.
    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    /// Get the bottom edge y coordinate.
    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }
}

/// Legend position.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LegendPosition {
    /// Top of chart.
    Top,
    /// Right of chart.
    Right,
    /// Bottom of chart.
    #[default]
    Bottom,
    /// Left of chart.
    Left,
}

/// Legend item.
#[derive(Debug, Clone)]
pub struct LegendItem {
    /// Label text.
    pub label: String,
    /// Item color.
    pub color: String,
    /// Is visible.
    pub visible: bool,
}

impl LegendItem {
    /// Create a new legend item.
    pub fn new(label: impl Into<String>, color: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            color: color.into(),
            visible: true,
        }
    }
}

/// Calculate legend layout dimensions.
pub fn calculate_legend_dimensions(
    items: &[LegendItem],
    position: LegendPosition,
    container_width: f64,
) -> (f64, f64) {
    let item_height = 20.0;
    let item_min_width = 80.0;
    let padding = 10.0;

    match position {
        LegendPosition::Top | LegendPosition::Bottom => {
            // Horizontal layout
            let items_per_row = (container_width / item_min_width).floor() as usize;
            let rows = (items.len() + items_per_row - 1) / items_per_row.max(1);
            (container_width, rows as f64 * item_height + padding * 2.0)
        }
        LegendPosition::Left | LegendPosition::Right => {
            // Vertical layout
            let max_label_len = items.iter()
                .map(|i| i.label.len())
                .max()
                .unwrap_or(10);
            let width = max_label_len as f64 * 8.0 + 30.0; // Approximate width
            let height = items.len() as f64 * item_height + padding * 2.0;
            (width, height)
        }
    }
}

// =============================================================================
// Data Helpers
// =============================================================================

/// Calculate min and max from a series of values.
pub fn min_max(values: &[f64]) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 1.0);
    }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    (min, max)
}

/// Extend domain to include zero if close.
pub fn nice_domain(min: f64, max: f64) -> (f64, f64) {
    let range = max - min;

    // Include zero if it's close
    let nice_min = if min > 0.0 && min < range * 0.2 {
        0.0
    } else {
        min
    };

    let nice_max = if max < 0.0 && max > -range * 0.2 {
        0.0
    } else {
        max
    };

    (nice_min, nice_max)
}

/// Calculate percentage of total.
pub fn percentage(value: f64, total: f64) -> f64 {
    if total.abs() < f64::EPSILON {
        0.0
    } else {
        (value / total) * 100.0
    }
}

/// Interpolate between two colors (hex format).
pub fn interpolate_color(color1: &str, color2: &str, t: f64) -> String {
    // Parse hex colors
    let parse_hex = |s: &str| -> Option<(u8, u8, u8)> {
        let s = s.trim_start_matches('#');
        if s.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some((r, g, b))
    };

    let (r1, g1, b1) = parse_hex(color1).unwrap_or((24, 144, 255));
    let (r2, g2, b2) = parse_hex(color2).unwrap_or((255, 77, 79));

    let t = clamp(t, 0.0, 1.0);
    let r = (r1 as f64 + t * (r2 as f64 - r1 as f64)) as u8;
    let g = (g1 as f64 + t * (g2 as f64 - g1 as f64)) as u8;
    let b = (b1 as f64 + t * (b2 as f64 - b1 as f64)) as u8;

    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_path() {
        let points = vec![(0.0, 0.0), (10.0, 20.0), (20.0, 10.0)];
        let path = line_path(&points);
        assert!(path.starts_with("M0.00,0.00"));
        assert!(path.contains("L10.00,20.00"));
    }

    #[test]
    fn test_scale_linear() {
        assert!((scale_linear((0.0, 100.0), (0.0, 1.0), 50.0) - 0.5).abs() < 0.001);
        assert!((scale_linear((0.0, 100.0), (100.0, 200.0), 50.0) - 150.0).abs() < 0.001);
    }

    #[test]
    fn test_generate_ticks() {
        let ticks = generate_ticks(0.0, 100.0, 5);
        assert!(!ticks.is_empty());
        assert!(ticks.iter().all(|&t| t >= 0.0 && t <= 100.0));
    }

    #[test]
    fn test_format_compact() {
        assert_eq!(format_tick_label(1_500_000.0, TickFormat::Compact), "1.5M");
        assert_eq!(format_tick_label(2_500.0, TickFormat::Compact), "2.5K");
    }
}
