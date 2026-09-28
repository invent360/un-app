//! Animal icons.
//!
//! This module provides SVG icons for animals.
//! Enable the `animals` feature to use these icons.

use leptos::prelude::*;

/// Animal icon identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimalIcon {
    // Pets
    /// Dog
    Dog,
    /// Cat
    Cat,
    /// Fish
    Fish,
    /// Bird
    Bird,
    /// Rabbit
    Rabbit,
    /// Hamster
    Hamster,

    // Farm animals
    /// Horse
    Horse,
    /// Cow
    Cow,
    /// Pig
    Pig,
    /// Sheep
    Sheep,
    /// Chicken
    Chicken,
    /// Duck
    Duck,

    // Wild animals
    /// Bear
    Bear,
    /// Lion
    Lion,
    /// Elephant
    Elephant,
    /// Wolf
    Wolf,
    /// Fox
    Fox,
    /// Deer
    Deer,

    // Sea creatures
    /// Whale
    Whale,
    /// Dolphin
    Dolphin,
    /// Turtle
    Turtle,
    /// Crab
    Crab,

    // Insects
    /// Bee
    Bee,
    /// Butterfly
    Butterfly,
    /// Bug
    Bug,
}

impl AnimalIcon {
    /// Returns the icon name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dog => "Dog",
            Self::Cat => "Cat",
            Self::Fish => "Fish",
            Self::Bird => "Bird",
            Self::Rabbit => "Rabbit",
            Self::Hamster => "Hamster",
            Self::Horse => "Horse",
            Self::Cow => "Cow",
            Self::Pig => "Pig",
            Self::Sheep => "Sheep",
            Self::Chicken => "Chicken",
            Self::Duck => "Duck",
            Self::Bear => "Bear",
            Self::Lion => "Lion",
            Self::Elephant => "Elephant",
            Self::Wolf => "Wolf",
            Self::Fox => "Fox",
            Self::Deer => "Deer",
            Self::Whale => "Whale",
            Self::Dolphin => "Dolphin",
            Self::Turtle => "Turtle",
            Self::Crab => "Crab",
            Self::Bee => "Bee",
            Self::Butterfly => "Butterfly",
            Self::Bug => "Bug",
        }
    }

    /// Parse an animal icon from its name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "dog" => Some(Self::Dog),
            "cat" => Some(Self::Cat),
            "fish" => Some(Self::Fish),
            "bird" => Some(Self::Bird),
            "rabbit" => Some(Self::Rabbit),
            "hamster" => Some(Self::Hamster),
            "horse" => Some(Self::Horse),
            "cow" => Some(Self::Cow),
            "pig" => Some(Self::Pig),
            "sheep" => Some(Self::Sheep),
            "chicken" => Some(Self::Chicken),
            "duck" => Some(Self::Duck),
            "bear" => Some(Self::Bear),
            "lion" => Some(Self::Lion),
            "elephant" => Some(Self::Elephant),
            "wolf" => Some(Self::Wolf),
            "fox" => Some(Self::Fox),
            "deer" => Some(Self::Deer),
            "whale" => Some(Self::Whale),
            "dolphin" => Some(Self::Dolphin),
            "turtle" => Some(Self::Turtle),
            "crab" => Some(Self::Crab),
            "bee" => Some(Self::Bee),
            "butterfly" => Some(Self::Butterfly),
            "bug" => Some(Self::Bug),
            _ => None,
        }
    }

    /// Returns all available animal icons.
    pub fn all() -> &'static [AnimalIcon] {
        &[
            Self::Dog, Self::Cat, Self::Fish, Self::Bird, Self::Rabbit, Self::Hamster,
            Self::Horse, Self::Cow, Self::Pig, Self::Sheep, Self::Chicken, Self::Duck,
            Self::Bear, Self::Lion, Self::Elephant, Self::Wolf, Self::Fox, Self::Deer,
            Self::Whale, Self::Dolphin, Self::Turtle, Self::Crab,
            Self::Bee, Self::Butterfly, Self::Bug,
        ]
    }
}

// Animal SVG constants - minimal, clean designs
const DOG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M10 5.172C10 3.782 8.423 2.679 6.5 3c-2.823.47-4.113 6.006-4 7 .08.703 1.725 1.722 3.656 1 1.261-.472 1.96-1.45 2.344-2.5"/><path d="M14.267 5.172c0-1.39 1.577-2.493 3.5-2.172 2.823.47 4.113 6.006 4 7-.08.703-1.725 1.722-3.656 1-1.261-.472-1.855-1.45-2.239-2.5"/><path d="M8 14v.5"/><path d="M16 14v.5"/><path d="M11.25 16.25h1.5L12 17l-.75-.75z"/><ellipse cx="12" cy="13" rx="7" ry="6"/><path d="M5.173 15.5A6.971 6.971 0 0 0 7 19c1.5 1.5 3.5 2 5 2s3.5-.5 5-2a6.971 6.971 0 0 0 1.827-3.5"/></svg>"##;

const CAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5c.67 0 1.35.09 2 .26 1.78-2 5.03-2.84 6.42-2.26 1.4.58-.42 7-.42 7 .57 1.07 1 2.24 1 3.44C21 17.9 16.97 21 12 21s-9-3.1-9-7.56c0-1.25.5-2.4 1-3.44 0 0-1.89-6.42-.5-7 1.39-.58 4.72.23 6.5 2.23A9.04 9.04 0 0 1 12 5z"/><path d="M8 14v.5"/><path d="M16 14v.5"/><path d="M11.25 16.25h1.5L12 17l-.75-.75z"/></svg>"##;

const FISH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M6.5 12c.94-3.46 4.94-6 8.5-6 3.56 0 6.06 2.54 7 6-.94 3.46-3.44 6-7 6-3.56 0-7.56-2.54-8.5-6z"/><path d="M18 12h.01"/><path d="M2 12l4-4v8l-4-4z"/><path d="M22 12c-1.5 0-3-1.5-3-3s1.5-3 3-3"/><path d="M22 12c-1.5 0-3 1.5-3 3s1.5 3 3 3"/></svg>"##;

const BIRD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M16 7h.01"/><path d="M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 20"/><path d="m20 7 2 .5-2 .5"/><path d="M10 18v3"/><path d="M14 17.75V21"/><path d="M7 18a6 6 0 0 0 3.84-10.61"/></svg>"##;

const RABBIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M13 16a3 3 0 0 1 2.24 5"/><path d="M18 12h.01"/><path d="M18 21h-8a4 4 0 0 1-4-4 7 7 0 0 1 7-7h.2L9.6 6.4a1.93 1.93 0 1 1 2.8-2.8L15.8 7h.2c3.3 0 6 2.7 6 6v1a2 2 0 0 1-2 2h-1a3 3 0 0 0-3 3"/><path d="M20 8.54V4a2 2 0 1 0-4 0v3"/><path d="M7.612 12.524a3 3 0 1 0-1.6 4.3"/></svg>"##;

const HAMSTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="13" r="7"/><path d="M9 10h.01"/><path d="M15 10h.01"/><path d="M10 16c.5.3 1.2.5 2 .5s1.5-.2 2-.5"/><path d="M5 6c0-1.5 1-3 3-3s3 1.5 3 3"/><path d="M13 3c0-1.5 1-3 3-3s3 1.5 3 3"/><ellipse cx="12" cy="14" rx="2" ry="1"/></svg>"##;

const HORSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M19 5h-2l-1.5-2.5a1 1 0 0 0-.83-.5H13a1 1 0 0 0-1 1v3l-2 3v5h3l1 4h2l-1-4h4a3 3 0 0 0 3-3v-3a3 3 0 0 0-3-3z"/><path d="M5 18h2l1-5h3"/><path d="M2 8l3 1 2-2"/><circle cx="18" cy="8" r="1"/></svg>"##;

const COW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="12" cy="13" rx="8" ry="6"/><path d="M4 7c0-1.5 1-3 3-3"/><path d="M20 7c0-1.5-1-3-3-3"/><path d="M9 10h.01"/><path d="M15 10h.01"/><ellipse cx="12" cy="15" rx="2" ry="1.5"/><path d="M8 19v2"/><path d="M16 19v2"/></svg>"##;

const PIG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="12" cy="12" rx="8" ry="6"/><path d="M4 12c-1.5 0-2-.5-2-1.5S3 9 4 9"/><path d="M20 12c1.5 0 2-.5 2-1.5S21 9 20 9"/><circle cx="9" cy="10" r="1"/><circle cx="15" cy="10" r="1"/><ellipse cx="12" cy="13" rx="2" ry="1.5"/><path d="M11 13.5v.5"/><path d="M13 13.5v.5"/><path d="M8 17v2"/><path d="M16 17v2"/></svg>"##;

const SHEEP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="6" cy="8" r="2"/><circle cx="10" cy="6" r="2"/><circle cx="14" cy="6" r="2"/><circle cx="18" cy="8" r="2"/><circle cx="5" cy="12" r="2"/><circle cx="19" cy="12" r="2"/><ellipse cx="12" cy="12" rx="6" ry="4"/><path d="M8 16v3"/><path d="M16 16v3"/><circle cx="10" cy="12" r="1"/><circle cx="14" cy="12" r="1"/></svg>"##;

const CHICKEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="12" cy="14" rx="6" ry="5"/><path d="M12 9V6a3 3 0 0 1 3-3c1 0 2 .5 2 1.5S16 6 15 6h-3"/><path d="M10 12h.01"/><path d="M14 12h.01"/><path d="M10 15l2 1 2-1"/><path d="M12 16v3"/><path d="M8 19h2"/><path d="M14 19h2"/></svg>"##;

const DUCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="11" cy="14" rx="7" ry="5"/><circle cx="8" cy="9" r="4"/><path d="M12 9h4a1 1 0 0 1 1 1v1a1 1 0 0 1-1 1h-2"/><path d="M6 8h.01"/><path d="M7 19v2"/><path d="M15 19v2"/></svg>"##;

const BEAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="7" cy="6" r="2"/><circle cx="17" cy="6" r="2"/><ellipse cx="12" cy="13" rx="7" ry="6"/><circle cx="9" cy="11" r="1"/><circle cx="15" cy="11" r="1"/><ellipse cx="12" cy="15" rx="2" ry="1.5"/></svg>"##;

const LION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="9"/><circle cx="12" cy="13" r="5"/><circle cx="10" cy="11" r="1"/><circle cx="14" cy="11" r="1"/><path d="M10 15l2 1 2-1"/><path d="M12 3v2"/><path d="M4.93 4.93l1.41 1.41"/><path d="M3 12h2"/><path d="M4.93 19.07l1.41-1.41"/><path d="M19.07 4.93l-1.41 1.41"/><path d="M21 12h-2"/><path d="M19.07 19.07l-1.41-1.41"/></svg>"##;

const ELEPHANT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="14" cy="12" rx="7" ry="6"/><path d="M7 12c-2 0-4-1-4-3s1-4 3-4 3 1 3 3"/><path d="M5 16v4"/><path d="M19 16v4"/><circle cx="16" cy="10" r="1"/><path d="M9 5c1-1 3-2 5-2s4 1 5 2"/></svg>"##;

const WOLF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M6 4l2 4"/><path d="M18 4l-2 4"/><ellipse cx="12" cy="13" rx="7" ry="6"/><circle cx="9" cy="11" r="1"/><circle cx="15" cy="11" r="1"/><path d="M9 16l3 2 3-2"/></svg>"##;

const FOX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M6 3l3 5"/><path d="M18 3l-3 5"/><ellipse cx="12" cy="13" rx="7" ry="6"/><circle cx="9" cy="11" r="1"/><circle cx="15" cy="11" r="1"/><path d="M12 14v2"/><path d="M9 17l3-1 3 1"/></svg>"##;

const DEER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M6 2l2 3-1 2"/><path d="M18 2l-2 3 1 2"/><ellipse cx="12" cy="13" rx="6" ry="5"/><circle cx="10" cy="11" r="1"/><circle cx="14" cy="11" r="1"/><ellipse cx="12" cy="15" rx="1.5" ry="1"/><path d="M9 18v3"/><path d="M15 18v3"/></svg>"##;

const WHALE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M19 10c0-4-3-7-7-7S5 6 5 10c0 3 2 6 5 7v4l3-2 3 2v-4c3-1 5-4 5-7z"/><path d="M2 10c0-2 1-4 3-5"/><circle cx="10" cy="9" r="1"/><path d="M12 3v2"/></svg>"##;

const DOLPHIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 11c-1 3-4 6-8 6s-7-2-9-5c-1-2 0-4 2-5 1-.5 3-.5 4 0l2 1h3c2 0 4-1 5-3l1 2v4z"/><circle cx="17" cy="9" r="1"/><path d="M6 12l-3 3 3 1"/></svg>"##;

const TURTLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="12" cy="13" rx="7" ry="4"/><path d="M12 9v-3a1 1 0 0 1 1-1h1"/><circle cx="12" cy="6" r="1"/><path d="M6 13l-2 3"/><path d="M18 13l2 3"/><path d="M8 17l-1 2"/><path d="M16 17l1 2"/><path d="M9 13v2"/><path d="M12 13v2"/><path d="M15 13v2"/></svg>"##;

const CRAB_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="12" cy="14" rx="6" ry="4"/><circle cx="10" cy="13" r="1"/><circle cx="14" cy="13" r="1"/><path d="M5 10c-2-1-3-3-2-4s3 0 4 2"/><path d="M19 10c2-1 3-3 2-4s-3 0-4 2"/><path d="M7 18l-2 2"/><path d="M17 18l2 2"/><path d="M9 18v2"/><path d="M15 18v2"/></svg>"##;

const BEE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="12" cy="13" rx="5" ry="6"/><path d="M9 10h6"/><path d="M9 13h6"/><path d="M9 16h6"/><circle cx="10" cy="8" r="1"/><circle cx="14" cy="8" r="1"/><path d="M10 7l-1-3"/><path d="M14 7l1-3"/><path d="M7 11c-2-1-3-2-3-3"/><path d="M17 11c2-1 3-2 3-3"/></svg>"##;

const BUTTERFLY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 6v12"/><path d="M5 9c-2-3-1-6 1-7s5 1 6 4c1-3 4-5 6-4s3 4 1 7c-2 4-7 6-7 9 0-3-5-5-7-9z"/><circle cx="7" cy="9" r="1"/><circle cx="17" cy="9" r="1"/><path d="M10 4l2 2 2-2"/></svg>"##;

const BUG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="12" cy="14" rx="5" ry="6"/><path d="M12 8v-4"/><path d="M10 4l2 2 2-2"/><circle cx="10" cy="12" r="1"/><circle cx="14" cy="12" r="1"/><path d="M12 14v3"/><path d="M7 10l-3-2"/><path d="M17 10l3-2"/><path d="M7 14l-3 1"/><path d="M17 14l3 1"/><path d="M7 18l-2 2"/><path d="M17 18l2 2"/></svg>"##;

/// Get the SVG content for an animal icon.
#[must_use]
pub fn get_animal_svg(icon: AnimalIcon) -> &'static str {
    match icon {
        AnimalIcon::Dog => DOG_SVG,
        AnimalIcon::Cat => CAT_SVG,
        AnimalIcon::Fish => FISH_SVG,
        AnimalIcon::Bird => BIRD_SVG,
        AnimalIcon::Rabbit => RABBIT_SVG,
        AnimalIcon::Hamster => HAMSTER_SVG,
        AnimalIcon::Horse => HORSE_SVG,
        AnimalIcon::Cow => COW_SVG,
        AnimalIcon::Pig => PIG_SVG,
        AnimalIcon::Sheep => SHEEP_SVG,
        AnimalIcon::Chicken => CHICKEN_SVG,
        AnimalIcon::Duck => DUCK_SVG,
        AnimalIcon::Bear => BEAR_SVG,
        AnimalIcon::Lion => LION_SVG,
        AnimalIcon::Elephant => ELEPHANT_SVG,
        AnimalIcon::Wolf => WOLF_SVG,
        AnimalIcon::Fox => FOX_SVG,
        AnimalIcon::Deer => DEER_SVG,
        AnimalIcon::Whale => WHALE_SVG,
        AnimalIcon::Dolphin => DOLPHIN_SVG,
        AnimalIcon::Turtle => TURTLE_SVG,
        AnimalIcon::Crab => CRAB_SVG,
        AnimalIcon::Bee => BEE_SVG,
        AnimalIcon::Butterfly => BUTTERFLY_SVG,
        AnimalIcon::Bug => BUG_SVG,
    }
}

/// Animal icon component.
#[component]
pub fn Animal(
    /// The animal icon to display.
    icon: AnimalIcon,
    /// Optional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
    /// Optional aria-label for accessibility.
    #[prop(optional, into)]
    aria_label: Option<String>,
) -> impl IntoView {
    let svg = get_animal_svg(icon);
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
