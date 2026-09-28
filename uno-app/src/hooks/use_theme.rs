//! Theme hook for managing app themes
//!
//! This module provides reactive theme management with Dark, Light, and DarkBlue themes.
//! It integrates with ember-fx ThemeProvider for component theming while maintaining
//! UNO-specific theme variations.

use leptos::prelude::*;

// Re-export ember-fx theme types for convenience when features are enabled
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use ember_fx_components::{
    DesignSystem,
    ThemeContext as EmberThemeContext,
    use_theme as use_ember_theme,
    try_use_theme as try_use_ember_theme,
    ThemeProvider as EmberThemeProvider,
    MinimalThemeProvider as EmberMinimalThemeProvider,
};

// Fallback DesignSystem type when ember-fx is not available
#[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DesignSystem {
    #[default]
    Ant,
    Material,
}

/// Available themes for the UNO app
///
/// These map to ember-fx theme names with UNO-specific variations.
/// - `DarkBlue`: UNO's signature dark blue theme (maps to ember-fx "dark-blue")
/// - `Dark`: Pure dark theme (maps to ember-fx "dark")
/// - `Light`: Light theme (maps to ember-fx "light")
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Theme {
    /// Pure dark theme
    Dark,
    /// Light theme
    Light,
    /// UNO's signature dark blue theme (default)
    #[default]
    DarkBlue,
}

impl Theme {
    /// Get the theme's data-theme attribute value
    ///
    /// This is the value used for the `data-theme` attribute on the HTML element.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
            Theme::DarkBlue => "dark-blue",
        }
    }

    /// Get the display name for the theme
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::DarkBlue => "Dark Blue",
        }
    }

    /// Get icon for the theme
    #[must_use]
    pub const fn icon(&self) -> &'static str {
        match self {
            Theme::Dark => "\u{1F319}",    // Moon
            Theme::Light => "\u{2600}",    // Sun
            Theme::DarkBlue => "\u{1F4A7}", // Water drop
        }
    }

    /// Parse from string code
    #[must_use]
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "dark" => Some(Theme::Dark),
            "light" => Some(Theme::Light),
            "dark-blue" => Some(Theme::DarkBlue),
            _ => None,
        }
    }

    /// Get all available themes
    #[must_use]
    pub const fn all() -> &'static [Theme] {
        &[Theme::DarkBlue, Theme::Dark, Theme::Light]
    }

    /// Check if theme is a dark variant (for system preference fallback)
    #[must_use]
    pub const fn is_dark(&self) -> bool {
        matches!(self, Theme::Dark | Theme::DarkBlue)
    }

    /// Get the next theme in cycle
    #[must_use]
    pub const fn next(&self) -> Theme {
        match self {
            Theme::DarkBlue => Theme::Dark,
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::DarkBlue,
        }
    }
}

/// UNO theme context for app-specific theme state
///
/// This provides a thin wrapper around ember-fx's ThemeContext
/// with UNO-specific functionality.
#[derive(Clone, Copy)]
pub struct UnoThemeContext {
    /// Signal tracking the current UNO theme
    theme: RwSignal<Theme>,
}

impl UnoThemeContext {
    /// Check if current theme is a dark variant
    #[must_use]
    pub fn is_dark(&self) -> bool {
        self.theme.get().is_dark()
    }

    /// Get current theme
    #[must_use]
    pub fn theme(&self) -> Theme {
        self.theme.get()
    }

    /// Get theme signal for reactive updates
    #[must_use]
    pub fn theme_signal(&self) -> RwSignal<Theme> {
        self.theme
    }
}

/// Provide UNO theme context to the app
///
/// This should be called once at app initialization, typically in the App component.
/// It creates and provides the UNO-specific theme context.
///
/// Note: ember-fx ThemeProvider should be used in the view hierarchy to manage
/// the actual CSS variable injection.
pub fn provide_theme_context() {
    let theme = RwSignal::new(Theme::default());
    provide_context(UnoThemeContext { theme });
}

/// Use the UNO theme context
///
/// # Panics
/// Panics if `provide_theme_context()` was not called.
#[must_use]
pub fn use_theme() -> UnoThemeContext {
    expect_context::<UnoThemeContext>()
}

/// Try to use the UNO theme context (returns None if not available)
#[must_use]
pub fn try_use_theme() -> Option<UnoThemeContext> {
    use_context::<UnoThemeContext>()
}

/// Get the current theme
#[must_use]
pub fn current_theme() -> Theme {
    use_theme().theme.get()
}

/// Set the current theme
///
/// This updates both the UNO theme context and syncs with ember-fx's ThemeContext.
pub fn set_theme(new_theme: Theme) {
    // Update UNO theme context
    let ctx = use_theme();
    ctx.theme.set(new_theme);

    // Sync with ember-fx ThemeContext
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    if let Some(ember_ctx) = try_use_ember_theme() {
        ember_ctx.set_theme(new_theme.code());
    }

    // Update the document's data-theme attribute and persist to localStorage
    #[cfg(any(feature = "hydrate", feature = "csr"))]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute("data-theme", new_theme.code());
                }
            }
            // Persist to localStorage
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("fx-theme", new_theme.code());
            }
        }
    }
}

/// Cycle to the next theme
pub fn toggle_theme() {
    let current = current_theme();
    set_theme(current.next());
}

/// Initialize theme from localStorage or use default
///
/// This should be called after `provide_theme_context()` to restore
/// the user's saved theme preference.
pub fn initialize_theme() {
    #[cfg(any(feature = "hydrate", feature = "csr"))]
    {
        if let Some(window) = web_sys::window() {
            // Try localStorage first
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved_theme)) = storage.get_item("fx-theme") {
                    if let Some(theme) = Theme::from_code(&saved_theme) {
                        set_theme(theme);
                        return;
                    }
                }
            }

            // Default to DarkBlue theme
            set_theme(Theme::DarkBlue);
        }
    }
}

/// Get the current design system from ember-fx
#[must_use]
pub fn current_design_system() -> DesignSystem {
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    {
        try_use_ember_theme()
            .map(|ctx| ctx.design_system())
            .unwrap_or(DesignSystem::Ant)
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
    {
        DesignSystem::Ant
    }
}

/// Set the design system (Ant or Material)
pub fn set_design_system(system: DesignSystem) {
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    if let Some(ember_ctx) = try_use_ember_theme() {
        ember_ctx.set_design_system(system);
    }

    // Suppress unused warning
    let _ = system;
}

/// Toggle between Ant and Material design systems
pub fn toggle_design_system() {
    let current = current_design_system();
    let new_system = match current {
        DesignSystem::Ant => DesignSystem::Material,
        DesignSystem::Material => DesignSystem::Ant,
        #[allow(unreachable_patterns)]
        _ => DesignSystem::Ant,
    };
    set_design_system(new_system);
}
