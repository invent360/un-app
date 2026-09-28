//! Theme registry for managing embedded themes.
//!
//! Themes are compiled at build time and embedded into the binary
//! via `include_str!()`. This module provides access to those themes.

use std::collections::HashMap;

/// A compiled theme with metadata.
#[derive(Debug, Clone)]
pub struct CompiledTheme {
    /// Theme name (e.g., "dark", "light")
    pub name: &'static str,
    /// Display name for UI
    pub display_name: &'static str,
    /// Design system this theme belongs to
    pub design_system: &'static str,
    /// Whether this is a dark theme
    pub is_dark: bool,
    /// Compiled CSS content
    pub css: &'static str,
}

/// Theme registry for accessing embedded themes.
pub struct ThemeRegistry;

impl ThemeRegistry {
    /// Load theme CSS for a given design system and theme name.
    ///
    /// Returns `None` if the theme is not found.
    #[must_use]
    pub fn load_theme_css(design_system: &str, theme_name: &str) -> Option<String> {
        Self::get_embedded_themes()
            .iter()
            .find(|t| t.design_system == design_system && t.name == theme_name)
            .map(|t| t.css.to_string())
    }

    /// Get all available themes for a design system.
    #[must_use]
    pub fn themes_for_system(design_system: &str) -> Vec<&'static CompiledTheme> {
        Self::get_embedded_themes()
            .iter()
            .filter(|t| t.design_system == design_system)
            .collect()
    }

    /// Get all available themes grouped by design system.
    #[must_use]
    pub fn all_themes() -> HashMap<&'static str, Vec<&'static CompiledTheme>> {
        let mut map: HashMap<&'static str, Vec<&'static CompiledTheme>> = HashMap::new();

        for theme in Self::get_embedded_themes() {
            map.entry(theme.design_system)
                .or_default()
                .push(theme);
        }

        map
    }

    /// Get available design systems.
    #[must_use]
    pub fn available_design_systems() -> Vec<&'static str> {
        let mut systems: Vec<&'static str> = Self::get_embedded_themes()
            .iter()
            .map(|t| t.design_system)
            .collect();

        systems.sort();
        systems.dedup();
        systems
    }

    /// Get the default theme for a design system.
    #[must_use]
    pub fn default_theme(design_system: &str) -> Option<&'static CompiledTheme> {
        let themes = Self::themes_for_system(design_system);

        // Design system specific defaults
        match design_system {
            "prime" => {
                // Prime defaults to lara-dark
                themes.iter().find(|t| t.name == "lara-dark").copied()
                    .or_else(|| themes.first().copied())
            }
            _ => {
                // Others default to "dark"
                themes.iter().find(|t| t.name == "dark").copied()
                    .or_else(|| themes.first().copied())
            }
        }
    }

    /// Check if a theme exists.
    #[must_use]
    pub fn has_theme(design_system: &str, theme_name: &str) -> bool {
        Self::get_embedded_themes()
            .iter()
            .any(|t| t.design_system == design_system && t.name == theme_name)
    }

    /// Get embedded themes based on feature flags.
    ///
    /// This is where themes are actually embedded via `include_str!()`.
    fn get_embedded_themes() -> &'static [CompiledTheme] {
        static ALL_THEMES: &[CompiledTheme] = &[
            // Ant Design themes
            #[cfg(feature = "ant")]
            CompiledTheme {
                name: "light",
                display_name: "Light",
                design_system: "ant",
                is_dark: false,
                css: include_str!("compiled/ant/light.css"),
            },
            #[cfg(feature = "ant")]
            CompiledTheme {
                name: "dark",
                display_name: "Dark",
                design_system: "ant",
                is_dark: true,
                css: include_str!("compiled/ant/dark.css"),
            },
            #[cfg(feature = "ant")]
            CompiledTheme {
                name: "glass",
                display_name: "Glass",
                design_system: "ant",
                is_dark: false,
                css: include_str!("compiled/ant/glass.css"),
            },
            #[cfg(feature = "ant")]
            CompiledTheme {
                name: "skeumorph",
                display_name: "Skeumorph",
                design_system: "ant",
                is_dark: false,
                css: include_str!("compiled/ant/skeumorph.css"),
            },
            #[cfg(feature = "ant")]
            CompiledTheme {
                name: "shad",
                display_name: "Shad",
                design_system: "ant",
                is_dark: false,
                css: include_str!("compiled/ant/shad.css"),
            },
            #[cfg(feature = "ant")]
            CompiledTheme {
                name: "mui",
                display_name: "MUI",
                design_system: "ant",
                is_dark: false,
                css: include_str!("compiled/ant/mui.css"),
            },
            #[cfg(feature = "ant")]
            CompiledTheme {
                name: "sky",
                display_name: "Sky",
                design_system: "ant",
                is_dark: true,
                css: include_str!("compiled/ant/sky.css"),
            },
            #[cfg(feature = "ant")]
            CompiledTheme {
                name: "unity",
                display_name: "Unity",
                design_system: "ant",
                is_dark: true,
                css: include_str!("compiled/ant/unity.css"),
            },

            // Prime themes
            #[cfg(feature = "prime")]
            CompiledTheme {
                name: "lara-dark",
                display_name: "Lara Dark",
                design_system: "prime",
                is_dark: true,
                css: include_str!("compiled/prime/lara-dark.css"),
            },
            #[cfg(feature = "prime")]
            CompiledTheme {
                name: "lara-light",
                display_name: "Lara Light",
                design_system: "prime",
                is_dark: false,
                css: include_str!("compiled/prime/lara-light.css"),
            },
            #[cfg(feature = "prime")]
            CompiledTheme {
                name: "aura-dark",
                display_name: "Aura Dark",
                design_system: "prime",
                is_dark: true,
                css: include_str!("compiled/prime/aura-dark.css"),
            },
            #[cfg(feature = "prime")]
            CompiledTheme {
                name: "aura-light",
                display_name: "Aura Light",
                design_system: "prime",
                is_dark: false,
                css: include_str!("compiled/prime/aura-light.css"),
            },
            #[cfg(feature = "prime")]
            CompiledTheme {
                name: "material-dark",
                display_name: "Material Dark",
                design_system: "prime",
                is_dark: true,
                css: include_str!("compiled/prime/material-dark.css"),
            },
            #[cfg(feature = "prime")]
            CompiledTheme {
                name: "material-light",
                display_name: "Material Light",
                design_system: "prime",
                is_dark: false,
                css: include_str!("compiled/prime/material-light.css"),
            },
        ];

        ALL_THEMES
    }
}

/// Get component CSS for a design system.
///
/// Returns the compiled component CSS or empty string if not available.
#[must_use]
pub fn get_component_css(design_system: &str) -> &'static str {
    match design_system {
        #[cfg(feature = "ant")]
        "ant" => include_str!("compiled/ant/components.css"),

        #[cfg(feature = "material")]
        "material" => include_str!("compiled/material/components.css"),

        #[cfg(feature = "cupertino")]
        "cupertino" => include_str!("compiled/cupertino/components.css"),

        #[cfg(feature = "daisyui")]
        "daisyui" => include_str!("compiled/daisyui/components.css"),

        #[cfg(feature = "prime")]
        "prime" => include_str!("compiled/prime/components.css"),

        #[cfg(feature = "flutter")]
        "flutter" => include_str!("compiled/flutter/components.css"),

        _ => "",
    }
}
