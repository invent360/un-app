//! Nature icons.
//!
//! This module provides SVG icons for nature, plants, farming, and weather.
//! Enable the `nature` feature to use these icons.

use leptos::prelude::*;

/// Nature icon identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NatureIcon {
    // Plants
    /// Tree
    Tree,
    /// Pine tree
    Pine,
    /// Palm tree
    Palm,
    /// Flower
    Flower,
    /// Leaf
    Leaf,
    /// Clover
    Clover,
    /// Cactus
    Cactus,
    /// Sprout/Seedling
    Sprout,

    // Farming
    /// Barn
    Barn,
    /// Tractor
    Tractor,
    /// Wheat/Grain
    Wheat,
    /// Corn
    Corn,
    /// Carrot
    Carrot,
    /// Apple
    Apple,
    /// Grapes
    Grapes,
    /// Fence
    Fence,

    // Weather
    /// Sun
    Sun,
    /// Moon
    Moon,
    /// Cloud
    Cloud,
    /// Rain
    Rain,
    /// Snow
    Snow,
    /// Wind
    Wind,
    /// Thunder/Lightning
    Thunder,
    /// Rainbow
    Rainbow,

    // Landscape
    /// Mountain
    Mountain,
    /// Water/Wave
    Wave,
    /// River
    River,
    /// Sunrise
    Sunrise,
    /// Sunset
    Sunset,
}

impl NatureIcon {
    /// Returns the icon name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Tree => "Tree",
            Self::Pine => "Pine",
            Self::Palm => "Palm",
            Self::Flower => "Flower",
            Self::Leaf => "Leaf",
            Self::Clover => "Clover",
            Self::Cactus => "Cactus",
            Self::Sprout => "Sprout",
            Self::Barn => "Barn",
            Self::Tractor => "Tractor",
            Self::Wheat => "Wheat",
            Self::Corn => "Corn",
            Self::Carrot => "Carrot",
            Self::Apple => "Apple",
            Self::Grapes => "Grapes",
            Self::Fence => "Fence",
            Self::Sun => "Sun",
            Self::Moon => "Moon",
            Self::Cloud => "Cloud",
            Self::Rain => "Rain",
            Self::Snow => "Snow",
            Self::Wind => "Wind",
            Self::Thunder => "Thunder",
            Self::Rainbow => "Rainbow",
            Self::Mountain => "Mountain",
            Self::Wave => "Wave",
            Self::River => "River",
            Self::Sunrise => "Sunrise",
            Self::Sunset => "Sunset",
        }
    }

    /// Parse a nature icon from its name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "tree" => Some(Self::Tree),
            "pine" => Some(Self::Pine),
            "palm" => Some(Self::Palm),
            "flower" => Some(Self::Flower),
            "leaf" => Some(Self::Leaf),
            "clover" => Some(Self::Clover),
            "cactus" => Some(Self::Cactus),
            "sprout" | "seedling" => Some(Self::Sprout),
            "barn" => Some(Self::Barn),
            "tractor" => Some(Self::Tractor),
            "wheat" | "grain" => Some(Self::Wheat),
            "corn" => Some(Self::Corn),
            "carrot" => Some(Self::Carrot),
            "apple" => Some(Self::Apple),
            "grapes" => Some(Self::Grapes),
            "fence" => Some(Self::Fence),
            "sun" => Some(Self::Sun),
            "moon" => Some(Self::Moon),
            "cloud" => Some(Self::Cloud),
            "rain" => Some(Self::Rain),
            "snow" => Some(Self::Snow),
            "wind" => Some(Self::Wind),
            "thunder" | "lightning" => Some(Self::Thunder),
            "rainbow" => Some(Self::Rainbow),
            "mountain" => Some(Self::Mountain),
            "wave" | "water" => Some(Self::Wave),
            "river" => Some(Self::River),
            "sunrise" => Some(Self::Sunrise),
            "sunset" => Some(Self::Sunset),
            _ => None,
        }
    }

    /// Returns all available nature icons.
    pub fn all() -> &'static [NatureIcon] {
        &[
            Self::Tree, Self::Pine, Self::Palm, Self::Flower, Self::Leaf, Self::Clover, Self::Cactus, Self::Sprout,
            Self::Barn, Self::Tractor, Self::Wheat, Self::Corn, Self::Carrot, Self::Apple, Self::Grapes, Self::Fence,
            Self::Sun, Self::Moon, Self::Cloud, Self::Rain, Self::Snow, Self::Wind, Self::Thunder, Self::Rainbow,
            Self::Mountain, Self::Wave, Self::River, Self::Sunrise, Self::Sunset,
        ]
    }
}

// Nature SVG constants
const TREE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22v-7"/><path d="M9 22h6"/><path d="M12 15l-6-7h4l-4-5h12l-4 5h4l-6 7z"/></svg>"##;

const PINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22v-6"/><path d="M9 22h6"/><path d="M12 3l-6 8h3l-4 5h14l-4-5h3l-6-8z"/></svg>"##;

const PALM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M13 22c0-6 1-10 1-10"/><path d="M10 22c0-5 3-9 3-9"/><path d="M14 13c4-1 6-4 6-7-4 1-6 4-6 7z"/><path d="M10 13c-4-1-6-4-6-7 4 1 6 4 6 7z"/><path d="M12 6c-2-3 0-6 0-6s2 3 0 6z"/></svg>"##;

const FLOWER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M12 2a3 3 0 0 0 0 6 3 3 0 0 0 0-6z"/><path d="M19 7a3 3 0 0 0-5.2 3 3 3 0 0 0 5.2-3z"/><path d="M20.5 14a3 3 0 0 0-5.2-3 3 3 0 0 0 5.2 3z"/><path d="M16.5 21a3 3 0 0 0-1.5-5.2 3 3 0 0 0 1.5 5.2z"/><path d="M7.5 21a3 3 0 0 1 1.5-5.2 3 3 0 0 1-1.5 5.2z"/><path d="M3.5 14a3 3 0 0 1 5.2-3 3 3 0 0 1-5.2 3z"/><path d="M5 7a3 3 0 0 1 5.2 3 3 3 0 0 1-5.2-3z"/></svg>"##;

const LEAF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M6 3c8 0 13 3 13 9-4 8-13 9-13 9s5-7 0-18z"/><path d="M6 12c3-1 6-3 8-6"/></svg>"##;

const CLOVER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 12a4 4 0 1 0-4-4 4 4 0 0 0 4 4z"/><path d="M16 12a4 4 0 1 0-4 4 4 4 0 0 0 4-4z"/><path d="M12 12a4 4 0 1 0 4-4 4 4 0 0 0-4 4z"/><path d="M8 12a4 4 0 1 0 4 4 4 4 0 0 0-4-4z"/><path d="M12 16v6"/></svg>"##;

const CACTUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M10 21V9a2 2 0 1 1 4 0v12"/><path d="M6 14v-2a2 2 0 1 1 4 0v2"/><path d="M14 11v-2a2 2 0 1 1 4 0v2"/><path d="M8 21h8"/></svg>"##;

const SPROUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22V10"/><path d="M12 10c0-4 4-6 8-6-1 4-4 6-8 6z"/><path d="M12 14c0-4-4-6-8-6 1 4 4 6 8 6z"/><path d="M9 22h6"/></svg>"##;

const BARN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4 11v10h16V11"/><path d="M2 11l10-8 10 8"/><path d="M12 3v5"/><path d="M9 21v-6h6v6"/><path d="M4 11h16"/></svg>"##;

const TRACTOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="7" cy="17" r="3"/><circle cx="18" cy="17" r="2"/><path d="M10 17h5"/><path d="M4 17V8h8l4 5v4"/><path d="M16 13h4v4"/><path d="M7 8V5h6"/></svg>"##;

const WHEAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22V10"/><path d="M12 10c2-2 4-4 4-7-3 0-5 2-4 7z"/><path d="M12 10c-2-2-4-4-4-7 3 0 5 2 4 7z"/><path d="M12 14c2-2 4-3 4-5-2 0-4 1-4 5z"/><path d="M12 14c-2-2-4-3-4-5 2 0 4 1 4 5z"/><path d="M9 22h6"/></svg>"##;

const CORN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22v-8"/><ellipse cx="12" cy="10" rx="4" ry="6"/><path d="M8 8h8"/><path d="M8 11h8"/><path d="M9 14h6"/><path d="M12 4l-2-2"/><path d="M12 4l2-2"/><path d="M12 4v2"/></svg>"##;

const CARROT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M8 8l8 12-4-2-4 2 8-12z"/><path d="M8 8c-2-2-2-5 0-6 2 1 2 4 0 6z"/><path d="M12 6c0-3 2-5 4-4-1 2-2 4-4 4z"/><path d="M10 11l2 3"/><path d="M12 15l1 2"/></svg>"##;

const APPLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 6c-3 0-6 2-6 6 0 5 3 9 6 9s6-4 6-9c0-4-3-6-6-6z"/><path d="M12 6V2"/><path d="M12 3c2 0 4 1 4 3"/><path d="M9 9c0-1 1-2 3-2"/></svg>"##;

const GRAPES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="9" cy="9" r="2"/><circle cx="15" cy="9" r="2"/><circle cx="6" cy="14" r="2"/><circle cx="12" cy="14" r="2"/><circle cx="18" cy="14" r="2"/><circle cx="9" cy="19" r="2"/><circle cx="15" cy="19" r="2"/><path d="M12 5V2"/><path d="M12 2c2 0 3 1 4 2"/></svg>"##;

const FENCE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4 6l2-3 2 3v15H4V6z"/><path d="M10 6l2-3 2 3v15h-4V6z"/><path d="M16 6l2-3 2 3v15h-4V6z"/><path d="M2 10h20"/><path d="M2 16h20"/></svg>"##;

const SUN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="M4.93 4.93l1.41 1.41"/><path d="M17.66 17.66l1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="M6.34 17.66l-1.41 1.41"/><path d="M19.07 4.93l-1.41 1.41"/></svg>"##;

const MOON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9z"/></svg>"##;

const CLOUD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9z"/></svg>"##;

const RAIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M17.5 13H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9z"/><path d="M8 19v2"/><path d="M12 17v2"/><path d="M16 19v2"/></svg>"##;

const SNOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M17.5 13H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9z"/><path d="M8 18l.01.01"/><path d="M12 16l.01.01"/><path d="M16 18l.01.01"/><path d="M10 21l.01.01"/><path d="M14 21l.01.01"/></svg>"##;

const WIND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9.59 4.59A2 2 0 1 1 11 8H2"/><path d="M12.59 19.41A2 2 0 1 0 14 16H2"/><path d="M17.73 7.73A2.5 2.5 0 1 1 19.5 12H2"/></svg>"##;

const THUNDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M17.5 13H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9z"/><path d="M13 17l-4 4 1-4h-2l4-4-1 4h2z"/></svg>"##;

const RAINBOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M2 17a10 10 0 0 1 20 0"/><path d="M5 17a7 7 0 0 1 14 0"/><path d="M8 17a4 4 0 0 1 8 0"/></svg>"##;

const MOUNTAIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3l9 18H3L12 3z"/><path d="M12 3l3 6-3 2-3-2 3-6z"/></svg>"##;

const WAVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M2 12c2-2 4-2 6 0s4 2 6 0 4-2 6 0"/><path d="M2 17c2-2 4-2 6 0s4 2 6 0 4-2 6 0"/><path d="M2 7c2-2 4-2 6 0s4 2 6 0 4-2 6 0"/></svg>"##;

const RIVER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4 4c3 4 6 4 8 0s5-4 8 0"/><path d="M4 12c3 4 6 4 8 0s5-4 8 0"/><path d="M4 20c3 4 6 4 8 0s5-4 8 0"/></svg>"##;

const SUNRISE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2v4"/><path d="M4.93 10.93l2.83 2.83"/><path d="M2 18h2"/><path d="M20 18h2"/><path d="M19.07 10.93l-2.83 2.83"/><path d="M22 22H2"/><path d="M8 6l4-4 4 4"/><path d="M16 18a4 4 0 0 0-8 0"/></svg>"##;

const SUNSET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 10v-4"/><path d="M4.93 10.93l2.83 2.83"/><path d="M2 18h2"/><path d="M20 18h2"/><path d="M19.07 10.93l-2.83 2.83"/><path d="M22 22H2"/><path d="M8 2l4 4 4-4"/><path d="M16 18a4 4 0 0 0-8 0"/></svg>"##;

/// Get the SVG content for a nature icon.
#[must_use]
pub fn get_nature_svg(icon: NatureIcon) -> &'static str {
    match icon {
        NatureIcon::Tree => TREE_SVG,
        NatureIcon::Pine => PINE_SVG,
        NatureIcon::Palm => PALM_SVG,
        NatureIcon::Flower => FLOWER_SVG,
        NatureIcon::Leaf => LEAF_SVG,
        NatureIcon::Clover => CLOVER_SVG,
        NatureIcon::Cactus => CACTUS_SVG,
        NatureIcon::Sprout => SPROUT_SVG,
        NatureIcon::Barn => BARN_SVG,
        NatureIcon::Tractor => TRACTOR_SVG,
        NatureIcon::Wheat => WHEAT_SVG,
        NatureIcon::Corn => CORN_SVG,
        NatureIcon::Carrot => CARROT_SVG,
        NatureIcon::Apple => APPLE_SVG,
        NatureIcon::Grapes => GRAPES_SVG,
        NatureIcon::Fence => FENCE_SVG,
        NatureIcon::Sun => SUN_SVG,
        NatureIcon::Moon => MOON_SVG,
        NatureIcon::Cloud => CLOUD_SVG,
        NatureIcon::Rain => RAIN_SVG,
        NatureIcon::Snow => SNOW_SVG,
        NatureIcon::Wind => WIND_SVG,
        NatureIcon::Thunder => THUNDER_SVG,
        NatureIcon::Rainbow => RAINBOW_SVG,
        NatureIcon::Mountain => MOUNTAIN_SVG,
        NatureIcon::Wave => WAVE_SVG,
        NatureIcon::River => RIVER_SVG,
        NatureIcon::Sunrise => SUNRISE_SVG,
        NatureIcon::Sunset => SUNSET_SVG,
    }
}

/// Nature icon component.
#[component]
pub fn Nature(
    /// The nature icon to display.
    icon: NatureIcon,
    /// Optional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
    /// Optional aria-label for accessibility.
    #[prop(optional, into)]
    aria_label: Option<String>,
) -> impl IntoView {
    let svg = get_nature_svg(icon);
    let label = aria_label.unwrap_or_else(|| icon.name().to_string());

    view! {
        <span
            class=class
            role="img"
            aria-label=label
            inner_html=svg
        />
    }
}
