#!/usr/bin/env python3
"""Extract ocean map region data from SVG and generate Rust code."""

import re
from pathlib import Path

# Color to temperature value mapping (0.0 = coldest blue, 1.0 = warmest cyan)
# These colors form a gradient from deep blue to cyan
COLOR_VALUES = {
    # Deep blue (coldest)
    '#0184F1': 0.0,
    '#0187ED': 0.05,
    '#0189EC': 0.08,
    '#018AEA': 0.10,
    '#018DEB': 0.12,
    '#0191E9': 0.15,
    '#0192E8': 0.18,
    '#0194E7': 0.20,
    '#0196E6': 0.22,
    '#0198E5': 0.25,
    '#019AE4': 0.28,
    '#019CE3': 0.30,
    '#01A0E1': 0.35,
    '#01A1E0': 0.38,
    '#01A2DB': 0.40,
    '#01A4DE': 0.42,
    '#01A6DF': 0.45,
    '#01A7DE': 0.48,
    '#01A9DB': 0.50,
    '#01AADD': 0.52,
    '#01ABDB': 0.55,
    '#01ACDB': 0.58,
    '#01ADDA': 0.60,
    '#01B0D9': 0.65,
    '#01B2D7': 0.70,
    '#01B3D7': 0.72,
    '#01B6D7': 0.75,
    # Light cyan (warmest)
    '#8EE7F1': 1.0,
}

def get_color_value(fill_color):
    """Get normalized value for a fill color (0.0-1.0)."""
    color_upper = fill_color.upper()
    if color_upper in COLOR_VALUES:
        return COLOR_VALUES[color_upper]

    # Try to match by finding closest color
    for color, value in COLOR_VALUES.items():
        if color.upper() == color_upper:
            return value

    # Parse hex and estimate value based on color components
    if fill_color.startswith('#') and len(fill_color) == 7:
        try:
            r = int(fill_color[1:3], 16)
            g = int(fill_color[3:5], 16)
            b = int(fill_color[5:7], 16)
            # Blue channel dominant = cold, cyan (g+b high) = warm
            # Estimate: higher green relative to blue = warmer
            if b > 0:
                warmth = g / 255.0
                return min(1.0, warmth)
        except ValueError:
            pass

    return 0.5  # Default middle value

def parse_path_bounds(path_d):
    """Parse path data to extract bounding box center.

    The ocean map uses complex multi-line paths. We'll extract
    all coordinate pairs and compute the centroid.
    """
    # Clean up the path data (remove newlines and extra spaces)
    path_d = ' '.join(path_d.split())

    # Find all coordinate pairs (M, L commands have x,y pairs)
    # Pattern matches numbers that could be coordinates
    coords = re.findall(r'([0-9]+\.?[0-9]*),([0-9]+\.?[0-9]*)', path_d)

    if not coords:
        # Try space-separated format
        coords = re.findall(r'([0-9]+\.?[0-9]*)\s+([0-9]+\.?[0-9]*)', path_d)

    if len(coords) < 3:
        return None

    xs = [float(c[0]) for c in coords]
    ys = [float(c[1]) for c in coords]

    # Calculate centroid
    cx = sum(xs) / len(xs)
    cy = sum(ys) / len(ys)

    # Calculate approximate area (bounding box)
    width = max(xs) - min(xs)
    height = max(ys) - min(ys)
    area = width * height

    return {
        'cx': cx,
        'cy': cy,
        'min_x': min(xs),
        'max_x': max(xs),
        'min_y': min(ys),
        'max_y': max(ys),
        'area': area,
        'point_count': len(coords)
    }

def extract_regions(svg_content):
    """Extract all ocean region paths with their colors."""
    regions = []

    # The SVG has multi-line path data with "d=" spanning multiple lines
    # Pattern: <path fill="#XXXXXX" ... d="\nM ... Z\n">
    # We need to match the fill color and then capture the d attribute

    # First, let's find all path elements with fill
    path_pattern = re.compile(
        r'<path\s+fill="([^"]+)"[^>]*\s+d="\s*\n?(M[^"]+)"',
        re.DOTALL
    )

    for match in path_pattern.finditer(svg_content):
        fill = match.group(1)
        path_d = match.group(2)

        # Skip background/non-ocean colors
        if fill.upper() == '#0A162D':  # Background
            continue

        bounds = parse_path_bounds(path_d)
        if bounds and bounds['area'] > 100:  # Filter tiny regions
            value = get_color_value(fill)
            regions.append({
                'cx': bounds['cx'],
                'cy': bounds['cy'],
                'fill': fill,
                'value': value,
                'area': bounds['area'],
                'point_count': bounds['point_count']
            })

    return regions

def normalize_coordinates(regions, target_width=900, target_height=540):
    """Normalize coordinates to a standard viewport."""
    if not regions:
        return regions

    # Find bounds
    min_x = min(r['cx'] for r in regions)
    max_x = max(r['cx'] for r in regions)
    min_y = min(r['cy'] for r in regions)
    max_y = max(r['cy'] for r in regions)

    # Calculate scale and offset
    src_width = max_x - min_x
    src_height = max_y - min_y

    scale_x = target_width / src_width if src_width > 0 else 1
    scale_y = target_height / src_height if src_height > 0 else 1
    scale = min(scale_x, scale_y) * 0.95  # Leave some margin

    offset_x = (target_width - src_width * scale) / 2
    offset_y = (target_height - src_height * scale) / 2

    for r in regions:
        r['norm_x'] = (r['cx'] - min_x) * scale + offset_x
        r['norm_y'] = (r['cy'] - min_y) * scale + offset_y

    return regions

def generate_rust_code(regions):
    """Generate Rust code for the ocean map data."""
    lines = [
        '//! Pre-computed ocean map region data.',
        '//!',
        '//! This module contains ocean region positions and temperature values',
        '//! extracted from the ocean map SVG.',
        '',
        '/// An ocean region point in the map.',
        '#[derive(Debug, Clone, Copy)]',
        'pub struct OceanMapRegion {',
        '    /// X coordinate (normalized 0-900).',
        '    pub x: f64,',
        '    /// Y coordinate (normalized 0-540).',
        '    pub y: f64,',
        '    /// Temperature value (0.0 = cold/blue, 1.0 = warm/cyan).',
        '    pub value: f64,',
        '}',
        '',
        '/// All ocean regions in the map.',
        'pub static OCEAN_MAP_REGIONS: &[OceanMapRegion] = &[',
    ]

    for r in regions:
        lines.append(f'    OceanMapRegion {{ x: {r["norm_x"]:.2f}, y: {r["norm_y"]:.2f}, value: {r["value"]:.3f} }},')

    lines.append('];')
    lines.append('')
    lines.append(f'/// Total number of regions.')
    lines.append(f'pub const REGION_COUNT: usize = {len(regions)};')
    lines.append('')

    return '\n'.join(lines)

def main():
    svg_path = Path(__file__).parent / 'ocean_map.svg'
    output_path = Path(__file__).parent.parent.parent / 'components' / 'src' / 'maps' / 'ocean_map_data.rs'

    print(f"Reading SVG from: {svg_path}")
    content = svg_path.read_text(encoding='utf-8')

    print("Extracting ocean regions...")
    regions = extract_regions(content)
    print(f"Found {len(regions)} regions")

    if not regions:
        print("No regions found! Debugging...")
        # Show sample of SVG structure
        sample = content[1000:2000]
        print(f"Sample content:\n{sample}")
        return

    print("Normalizing coordinates...")
    regions = normalize_coordinates(regions)

    # Analyze values
    values = [r['value'] for r in regions]
    print(f"\nValue distribution:")
    print(f"  Min: {min(values):.3f}")
    print(f"  Max: {max(values):.3f}")
    print(f"  Avg: {sum(values)/len(values):.3f}")

    print("\nGenerating Rust code...")
    rust_code = generate_rust_code(regions)

    print(f"Writing to: {output_path}")
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(rust_code, encoding='utf-8')
    print(f"Done! Generated {len(rust_code)} bytes")

if __name__ == '__main__':
    main()
