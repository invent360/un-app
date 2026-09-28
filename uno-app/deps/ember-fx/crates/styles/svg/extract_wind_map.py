#!/usr/bin/env python3
"""Extract wind map dot data from SVG and generate Rust code."""

import re
from pathlib import Path

def parse_circle_path(path_d):
    """Parse a circle path to extract center coordinates.

    The circles are drawn using cubic bezier curves (C commands).
    We can extract the center from the M (moveto) command.

    Path format: M cx+r cy C ... (draws a circle)
    We need to find the center by looking at the M coordinate minus the radius.
    """
    # Find the first M command coordinates
    m_match = re.search(r'M\s*([0-9.-]+)\s+([0-9.-]+)', path_d)
    if not m_match:
        return None

    start_x = float(m_match.group(1))
    start_y = float(m_match.group(2))

    # The circle is drawn starting from the rightmost point
    # The first C command gives us info about the circle
    # For a circle of radius r: M (cx+r) cy C ...
    # We need to find r from the bezier control points

    # Look for C commands to estimate radius
    c_matches = re.findall(r'C\s*([0-9.-]+)\s+([0-9.-]+)\s+([0-9.-]+)\s+([0-9.-]+)\s+([0-9.-]+)\s+([0-9.-]+)', path_d)

    if c_matches:
        # The first C command ends at a point on the circle
        # For a standard circle, the endpoint of first C is at (cx, cy+r) or (cx, cy-r)
        first_c = c_matches[0]
        end_x = float(first_c[4])
        end_y = float(first_c[5])

        # The center x is the same as the end_x of first C (for a circle starting from right)
        # The radius can be estimated from start_x - end_x
        cx = end_x
        # For circles starting at right point: start_x = cx + r
        r = abs(start_x - cx)
        cy = start_y  # The start y is at the same level as center

        return (cx, cy, r)

    return None

def extract_dots(svg_content):
    """Extract all dot paths from SVG."""
    dots = []

    # Pattern to match path elements with white fill
    # The paths can span multiple lines, so we need a more flexible pattern
    path_pattern = re.compile(r'<path[^>]*fill="rgb\(100%, 100%, 100%\)"[^>]*d="([^"]+)"', re.DOTALL)

    for match in path_pattern.finditer(svg_content):
        path_d = match.group(1)

        # Parse the circle path
        result = parse_circle_path(path_d)
        if result:
            cx, cy, r = result
            # Only include dots with reasonable radius (filter out non-circles)
            if 3 < r < 15:
                dots.append({
                    'x': cx,
                    'y': cy,
                    'radius': r
                })

    return dots

def normalize_coordinates(dots, target_width=900, target_height=450):
    """Normalize coordinates to a standard viewport."""
    if not dots:
        return dots

    # Find bounds
    min_x = min(d['x'] for d in dots)
    max_x = max(d['x'] for d in dots)
    min_y = min(d['y'] for d in dots)
    max_y = max(d['y'] for d in dots)

    # Calculate scale and offset
    src_width = max_x - min_x
    src_height = max_y - min_y

    scale_x = target_width / src_width if src_width > 0 else 1
    scale_y = target_height / src_height if src_height > 0 else 1
    scale = min(scale_x, scale_y) * 0.95  # Leave some margin

    offset_x = (target_width - src_width * scale) / 2
    offset_y = (target_height - src_height * scale) / 2

    for d in dots:
        d['norm_x'] = (d['x'] - min_x) * scale + offset_x
        d['norm_y'] = (d['y'] - min_y) * scale + offset_y
        d['norm_r'] = d['radius'] * scale

    return dots

def generate_rust_code(dots):
    """Generate Rust code for the wind map data."""
    lines = [
        '//! Pre-computed wind map dot data.',
        '//!',
        '//! This module contains dot positions extracted from the wind map SVG.',
        '//! Each dot represents a wind measurement point.',
        '',
        '/// A dot in the wind map.',
        '#[derive(Debug, Clone, Copy)]',
        'pub struct WindMapDot {',
        '    /// X coordinate (normalized 0-900).',
        '    pub x: f64,',
        '    /// Y coordinate (normalized 0-450).',
        '    pub y: f64,',
        '    /// Radius of the dot.',
        '    pub radius: f64,',
        '}',
        '',
        '/// All dots in the wind map.',
        'pub static WIND_MAP_DOTS: &[WindMapDot] = &[',
    ]

    for d in dots:
        lines.append(f'    WindMapDot {{ x: {d["norm_x"]:.2f}, y: {d["norm_y"]:.2f}, radius: {d["norm_r"]:.2f} }},')

    lines.append('];')
    lines.append('')
    lines.append(f'/// Total number of dots.')
    lines.append(f'pub const DOT_COUNT: usize = {len(dots)};')
    lines.append('')

    return '\n'.join(lines)

def main():
    svg_path = Path(__file__).parent / 'wind_map.svg'
    output_path = Path(__file__).parent.parent.parent / 'components' / 'src' / 'maps' / 'wind_map_data.rs'

    print(f"Reading SVG from: {svg_path}")
    content = svg_path.read_text(encoding='utf-8')

    print("Extracting dots...")
    dots = extract_dots(content)
    print(f"Found {len(dots)} dots")

    if not dots:
        print("No dots found! Checking path format...")
        # Debug: show a sample path
        sample = re.search(r'<path[^>]*d="([^"]{100})', content)
        if sample:
            print(f"Sample path: {sample.group(1)}...")
        return

    print("Normalizing coordinates...")
    dots = normalize_coordinates(dots)

    # Show bounds
    if dots:
        print(f"\nBounds after normalization:")
        print(f"  X: {min(d['norm_x'] for d in dots):.2f} - {max(d['norm_x'] for d in dots):.2f}")
        print(f"  Y: {min(d['norm_y'] for d in dots):.2f} - {max(d['norm_y'] for d in dots):.2f}")

    print("\nGenerating Rust code...")
    rust_code = generate_rust_code(dots)

    print(f"Writing to: {output_path}")
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(rust_code, encoding='utf-8')
    print(f"Done! Generated {len(rust_code)} bytes")

if __name__ == '__main__':
    main()
