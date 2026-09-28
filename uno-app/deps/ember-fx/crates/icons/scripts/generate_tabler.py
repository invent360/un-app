#!/usr/bin/env python3
"""
Generate Rust modules from Tabler Icons SVG files.

Usage:
    python3 scripts/generate_tabler.py /path/to/tabler-icons

This script reads SVG files from the tabler-icons repository and generates
Rust modules with icon enums and SVG constants.
"""

import os
import sys
import re
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Tuple, Optional

# Category mappings from tabler categories to our module names
# We map all tabler categories to our curated module set
CATEGORY_MAP = {
    # Direct mappings
    "Arrows": "arrows",
    "System": "system",
    "Communication": "communication",
    "Media": "media",
    "Document": "document",
    "E-commerce": "commerce",
    "Map": "map",
    "Charts": "charts",
    "Brand": "brand",
    "Design": "design",
    "Devices": "devices",

    # Merged into system (core UI/actions)
    "Buildings": "system",
    "Development": "system",
    "Database": "system",
    "Math": "system",
    "Health": "system",
    "Logic": "system",
    "Symbols": "system",

    # Merged into document (text/files)
    "Text": "document",
    "Letters": "document",
    "Numbers": "document",

    # Merged into design (creative/visual)
    "Shapes": "design",
    "Photography": "design",
    "Badges": "design",
    "Mood": "design",
    "Gender": "design",
    "Gestures": "design",

    # Merged into media (entertainment)
    "Sport": "media",
    "Games": "media",

    # Merged into commerce (shopping/money)
    "Food": "commerce",
    "Currencies": "commerce",
    "Laundry": "commerce",

    # Merged into map (location/travel)
    "Weather": "map",
    "Nature": "map",
    "Vehicles": "map",
    "Zodiac": "map",
    "Animals": "map",

    # Merged into devices (hardware)
    "Computers": "devices",
    "Electrical": "devices",
    "Extensions": "devices",
    "Version control": "devices",
}

# Icons to skip (problematic or not useful)
SKIP_ICONS = set()

def kebab_to_pascal(name: str) -> str:
    """Convert kebab-case to PascalCase."""
    return ''.join(word.capitalize() for word in name.split('-'))

def kebab_to_snake(name: str) -> str:
    """Convert kebab-case to snake_case."""
    return name.replace('-', '_')

def extract_category(content: str) -> Optional[str]:
    """Extract category from SVG front-matter."""
    match = re.search(r'category:\s*([^\n]+)', content)
    if match:
        return match.group(1).strip()
    return None

def extract_svg_content(content: str) -> str:
    """Extract just the SVG element, stripping front-matter comments."""
    # Remove HTML comment front-matter
    content = re.sub(r'<!--[\s\S]*?-->', '', content).strip()
    return content

def clean_svg_for_rust(svg: str) -> str:
    """Clean SVG for embedding in Rust string."""
    # Remove newlines and extra whitespace
    svg = ' '.join(svg.split())
    # Escape any quotes if needed (though tabler uses double quotes)
    svg = svg.replace('"', r'"')
    return svg

def parse_icon_file(filepath: Path) -> Tuple[str, str, Optional[str]]:
    """Parse an icon SVG file and return (name, svg_content, category)."""
    name = filepath.stem
    content = filepath.read_text()
    category = extract_category(content)
    svg = extract_svg_content(content)
    svg = clean_svg_for_rust(svg)
    return name, svg, category

def generate_module(module_name: str, icons: List[Tuple[str, str, Optional[str]]]) -> str:
    """Generate Rust module code for a set of icons."""
    enum_name = kebab_to_pascal(module_name) + "Icon"

    # Sort icons by name
    icons = sorted(icons, key=lambda x: x[0])

    # Generate enum variants
    variants = []
    for name, _, _ in icons:
        variant = kebab_to_pascal(name)
        variants.append(f"    {variant},")

    # Generate SVG constants
    svg_consts = []
    for name, svg, _ in icons:
        const_name = kebab_to_snake(name).upper()
        svg_consts.append(f'const {const_name}_SVG: &str = r##"{svg}"##;')

    # Generate match arms for outline_svg
    outline_arms = []
    for name, _, _ in icons:
        variant = kebab_to_pascal(name)
        const_name = kebab_to_snake(name).upper()
        outline_arms.append(f"            Self::{variant} => {const_name}_SVG,")

    # Generate name() match arms
    name_arms = []
    for name, _, _ in icons:
        variant = kebab_to_pascal(name)
        name_arms.append(f'            Self::{variant} => "{name}",')

    # Generate from_name match arms
    from_name_arms = []
    for name, _, _ in icons:
        variant = kebab_to_pascal(name)
        from_name_arms.append(f'            "{name}" => Some(Self::{variant}),')

    # Generate all() array
    all_items = ", ".join(f"Self::{kebab_to_pascal(name)}" for name, _, _ in icons)

    code = f'''//! {module_name.replace('_', ' ').title()} icons from Tabler Icons.
//!
//! This module contains {len(icons)} icons.

use crate::tabler::TablerIconData;

// SVG Constants
{chr(10).join(svg_consts)}

/// {module_name.replace('_', ' ').title()} icon variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum {enum_name} {{
{chr(10).join(variants)}
}}

impl {enum_name} {{
    /// Returns all available icons in this category.
    pub fn all() -> &'static [Self] {{
        &[{all_items}]
    }}

    /// Returns the icon count.
    pub fn count() -> usize {{
        {len(icons)}
    }}

    /// Creates an icon from its kebab-case name.
    pub fn from_name(name: &str) -> Option<Self> {{
        match name {{
{chr(10).join(from_name_arms)}
            _ => None,
        }}
    }}
}}

impl TablerIconData for {enum_name} {{
    fn name(&self) -> &'static str {{
        match self {{
{chr(10).join(name_arms)}
        }}
    }}

    fn outline_svg(&self) -> &'static str {{
        match self {{
{chr(10).join(outline_arms)}
        }}
    }}

    fn filled_svg(&self) -> Option<&'static str> {{
        // Filled variants would be added here
        None
    }}
}}
'''
    return code

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 generate_tabler.py /path/to/tabler-icons")
        print("\nExample:")
        print("  python3 scripts/generate_tabler.py /Users/admin/Dev-x/themes/tabler-icons")
        sys.exit(1)

    tabler_path = Path(sys.argv[1])
    icons_path = tabler_path / "icons" / "outline"

    if not icons_path.exists():
        print(f"Error: Icons directory not found at {icons_path}")
        sys.exit(1)

    output_path = Path(__file__).parent.parent / "src" / "tabler"

    # Collect icons by category
    categories: Dict[str, List[Tuple[str, str, Optional[str]]]] = defaultdict(list)

    for svg_file in sorted(icons_path.glob("*.svg")):
        if svg_file.stem in SKIP_ICONS:
            continue

        try:
            name, svg, category = parse_icon_file(svg_file)

            if category and category in CATEGORY_MAP:
                module = CATEGORY_MAP[category]
                categories[module].append((name, svg, category))
            else:
                # Default to system for uncategorized
                categories["system"].append((name, svg, category))
        except Exception as e:
            print(f"Warning: Failed to parse {svg_file}: {e}")

    # Generate modules
    for module_name, icons in categories.items():
        if not icons:
            continue

        module_file = output_path / f"{module_name}.rs"
        code = generate_module(module_name, icons)

        module_file.write_text(code)
        print(f"Generated {module_file.name} with {len(icons)} icons")

    print(f"\nTotal: {sum(len(icons) for icons in categories.values())} icons across {len(categories)} categories")

if __name__ == "__main__":
    main()
