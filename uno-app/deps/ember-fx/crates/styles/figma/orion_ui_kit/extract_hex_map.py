#!/usr/bin/env python3
"""Extract hexagon map data from Figma SVG and generate Rust code."""

import re
from pathlib import Path

# Color categories based on the SVG analysis
COLOR_CATEGORIES = {
    # Ocean colors
    '#1F1F43': 'ocean',      # Background/ocean (1510 hexagons)
    '#13132B': 'ocean_dark', # Darker ocean
    '#191932': 'ocean_dark', # Darker ocean variant

    # Land base colors (lowest value land)
    '#25245D': 'land_base',
    '#252563': 'land_base',
    '#252667': 'land_base',
    '#262766': 'land_base',
    '#262767': 'land_base',
    '#26286D': 'land_base',
    '#26296D': 'land_base',

    # Land low (slightly higher value)
    '#323864': 'land_low',   # Low value land (72 hexagons)

    # Land medium (medium value)
    '#632757': 'land_med',
    '#6B264F': 'land_med',
    '#6D264D': 'land_med',
    '#71254A': 'land_med',   # (60 hexagons)

    # Land high (higher value)
    '#772443': 'land_high',
    '#A7285A': 'land_high',

    # Hotspot colors (highest value)
    '#EC223B': 'hotspot',    # Hotspot/highlight (34 hexagons)
    '#CB2449': 'hotspot',
    '#CD2549': 'hotspot',

    # Blue accents
    '#353ABF': 'accent_blue',
    '#383DCA': 'accent_blue',

    # Bright accents
    '#176BF8': 'accent_bright',
    '#1963E9': 'accent_bright',
    '#1A61E5': 'accent_bright',

    # Special accents
    '#FFA63F': 'accent_orange',
    '#FFC700': 'accent_yellow',
}

# Skip these fill colors (UI elements, not hexagons)
SKIP_FILLS = {'white', 'black', 'none', '#05050F', '#0D0D10'}

# Map categories to numeric values for heatmap
CATEGORY_VALUES = {
    'ocean': 0.0,
    'ocean_dark': 0.0,
    'land_base': 0.15,
    'land_low': 0.3,
    'land_med': 0.5,
    'land_high': 0.7,
    'hotspot': 1.0,
    'accent_blue': 0.6,
    'accent_bright': 0.8,
    'accent_orange': 0.9,
    'accent_yellow': 0.95,
}

def parse_hex_path(path_d):
    """Parse a hexagon path to extract center coordinates.

    Handles SVG path commands: M (move), L (line), V (vertical line), H (horizontal line), Z (close).
    """
    vertices = []
    current_x, current_y = 0, 0

    # Split path into commands
    # Pattern matches command letter followed by numbers
    commands = re.findall(r'([MLHVZ])([0-9.\s-]*)', path_d, re.IGNORECASE)

    for cmd, args in commands:
        args = args.strip()
        cmd = cmd.upper()

        if cmd == 'M':  # Move to
            coords = re.findall(r'([0-9.-]+)', args)
            if len(coords) >= 2:
                current_x, current_y = float(coords[0]), float(coords[1])
                vertices.append((current_x, current_y))
        elif cmd == 'L':  # Line to
            coords = re.findall(r'([0-9.-]+)', args)
            if len(coords) >= 2:
                current_x, current_y = float(coords[0]), float(coords[1])
                vertices.append((current_x, current_y))
        elif cmd == 'V':  # Vertical line to (only y changes)
            coords = re.findall(r'([0-9.-]+)', args)
            if len(coords) >= 1:
                current_y = float(coords[0])
                vertices.append((current_x, current_y))
        elif cmd == 'H':  # Horizontal line to (only x changes)
            coords = re.findall(r'([0-9.-]+)', args)
            if len(coords) >= 1:
                current_x = float(coords[0])
                vertices.append((current_x, current_y))
        elif cmd == 'Z':  # Close path
            pass

    if len(vertices) >= 6:
        # Calculate center from vertices
        xs = [v[0] for v in vertices[:6]]
        ys = [v[1] for v in vertices[:6]]
        cx = sum(xs) / len(xs)
        cy = sum(ys) / len(ys)
        return (cx, cy)
    return None

def extract_hexagons(svg_content):
    """Extract all hexagon paths with their colors."""
    hexagons = []

    # Pattern to match path elements with fill
    path_pattern = re.compile(r'<path d="([^"]+)" fill="([^"]+)"')

    for match in path_pattern.finditer(svg_content):
        path_d = match.group(1)
        fill = match.group(2)

        # Skip UI element fills
        if fill.lower() in SKIP_FILLS:
            continue

        # Only process hexagon-shaped paths (have L or V commands for vertices)
        if path_d.count('L') >= 2 or (path_d.count('L') >= 1 and path_d.count('V') >= 1):
            center = parse_hex_path(path_d)
            if center:
                hexagons.append({
                    'cx': center[0],
                    'cy': center[1],
                    'fill': fill,
                    'path': path_d
                })

    return hexagons

def normalize_coordinates(hexagons, target_width=900, target_height=500):
    """Normalize coordinates to a standard viewport."""
    if not hexagons:
        return hexagons

    # Find bounds
    min_x = min(h['cx'] for h in hexagons)
    max_x = max(h['cx'] for h in hexagons)
    min_y = min(h['cy'] for h in hexagons)
    max_y = max(h['cy'] for h in hexagons)

    # Calculate scale and offset
    src_width = max_x - min_x
    src_height = max_y - min_y

    scale_x = target_width / src_width if src_width > 0 else 1
    scale_y = target_height / src_height if src_height > 0 else 1
    scale = min(scale_x, scale_y) * 0.95  # Leave some margin

    offset_x = (target_width - src_width * scale) / 2
    offset_y = (target_height - src_height * scale) / 2

    for h in hexagons:
        h['norm_x'] = (h['cx'] - min_x) * scale + offset_x
        h['norm_y'] = (h['cy'] - min_y) * scale + offset_y

    return hexagons

def categorize_hexagon(fill_color):
    """Categorize a hexagon by its fill color."""
    color_upper = fill_color.upper()
    for color, category in COLOR_CATEGORIES.items():
        if color.upper() == color_upper:
            return category, CATEGORY_VALUES.get(category, 0.0)
    return 'unknown', 0.0

def generate_rust_code(hexagons):
    """Generate Rust code for the hexagon data."""
    lines = [
        '//! Pre-computed hexagonal world map data from Orion UI Kit.',
        '//!',
        '//! This module contains hexagon positions and values extracted from',
        '//! the Figma design file.',
        '',
        '/// A hexagon cell in the pre-computed map.',
        '#[derive(Debug, Clone, Copy)]',
        'pub struct HexMapCell {',
        '    /// X coordinate (normalized 0-900).',
        '    pub x: f64,',
        '    /// Y coordinate (normalized 0-500).',
        '    pub y: f64,',
        '    /// Value for coloring (0.0-1.0).',
        '    pub value: f64,',
        '    /// Whether this is a land cell.',
        '    pub is_land: bool,',
        '}',
        '',
        '/// All hexagon cells in the map.',
        'pub static HEX_MAP_CELLS: &[HexMapCell] = &[',
    ]

    land_count = 0
    ocean_count = 0

    for h in hexagons:
        category, value = categorize_hexagon(h['fill'])
        is_land = category not in ('ocean', 'ocean_dark')

        if is_land:
            land_count += 1
        else:
            ocean_count += 1

        lines.append(f'    HexMapCell {{ x: {h["norm_x"]:.2f}, y: {h["norm_y"]:.2f}, value: {value:.2f}, is_land: {str(is_land).lower()} }},')

    lines.append('];')
    lines.append('')
    lines.append(f'/// Total number of hexagons.')
    lines.append(f'pub const HEX_COUNT: usize = {len(hexagons)};')
    lines.append('')
    lines.append(f'/// Number of land hexagons.')
    lines.append(f'pub const LAND_COUNT: usize = {land_count};')
    lines.append('')
    lines.append(f'/// Number of ocean hexagons.')
    lines.append(f'pub const OCEAN_COUNT: usize = {ocean_count};')
    lines.append('')
    lines.append('/// Get only land cells.')
    lines.append('pub fn land_cells() -> impl Iterator<Item = &\'static HexMapCell> {')
    lines.append('    HEX_MAP_CELLS.iter().filter(|c| c.is_land)')
    lines.append('}')
    lines.append('')
    lines.append('/// Get only ocean cells.')
    lines.append('pub fn ocean_cells() -> impl Iterator<Item = &\'static HexMapCell> {')
    lines.append('    HEX_MAP_CELLS.iter().filter(|c| !c.is_land)')
    lines.append('}')
    lines.append('')

    return '\n'.join(lines)

def main():
    svg_path = Path(__file__).parent / 'svgs' / 'secondary_view_2.svg'
    output_path = Path(__file__).parent.parent.parent.parent / 'components' / 'src' / 'maps' / 'orion_hex_data.rs'

    print(f"Reading SVG from: {svg_path}")
    content = svg_path.read_text(encoding='utf-8')

    print("Extracting hexagons...")
    hexagons = extract_hexagons(content)
    print(f"Found {len(hexagons)} hexagons")

    print("Normalizing coordinates...")
    hexagons = normalize_coordinates(hexagons)

    # Analyze colors
    colors = {}
    for h in hexagons:
        fill = h['fill']
        colors[fill] = colors.get(fill, 0) + 1

    print("\nColor distribution:")
    for color, count in sorted(colors.items(), key=lambda x: -x[1])[:10]:
        category, _ = categorize_hexagon(color)
        print(f"  {color}: {count} ({category})")

    print("\nGenerating Rust code...")
    rust_code = generate_rust_code(hexagons)

    print(f"Writing to: {output_path}")
    output_path.write_text(rust_code, encoding='utf-8')
    print(f"Done! Generated {len(rust_code)} bytes")

if __name__ == '__main__':
    main()
