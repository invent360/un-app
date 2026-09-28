//! Theme context for reactive theme management.
//!
//! Provides a Leptos context for accessing and modifying theme state
//! across the application.

use crate::platform::Platform;
use ember_fx_common::DesignSystem;
#[cfg(target_arch = "wasm32")]
use ember_fx_common::{THEME_STORAGE_KEY, DESIGN_SYSTEM_STORAGE_KEY};
#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;

/// Theme context for managing application theming.
///
/// This context provides reactive access to:
/// - Current theme name (e.g., "dark", "light")
/// - Design system (e.g., Ant, Material, Cupertino)
/// - Platform detection
///
/// # Example
/// ```ignore
/// let theme_ctx = use_theme();
///
/// // Get current theme
/// let current = theme_ctx.theme();
///
/// // Change theme
/// theme_ctx.set_theme("light");
///
/// // Toggle dark/light
/// theme_ctx.toggle();
/// ```
#[derive(Clone, Copy)]
pub struct ThemeContext {
    theme: RwSignal<String>,
    design_system: RwSignal<DesignSystem>,
    platform: Platform,
}

impl ThemeContext {
    /// Create a new theme context.
    #[must_use]
    pub fn new(
        initial_theme: &str,
        design_system: DesignSystem,
        platform: Platform,
    ) -> Self {
        let theme = RwSignal::new(initial_theme.to_string());
        let resolved_system = if design_system == DesignSystem::Auto {
            platform.default_design_system()
        } else {
            design_system
        };
        let design_system = RwSignal::new(resolved_system);

        Self {
            theme,
            design_system,
            platform,
        }
    }

    /// Get the current theme name.
    #[must_use]
    pub fn theme(&self) -> String {
        self.theme.get()
    }

    /// Get the current theme name as a signal for reactive updates.
    #[must_use]
    pub fn theme_signal(&self) -> RwSignal<String> {
        self.theme
    }

    /// Set the current theme by name.
    ///
    /// This will:
    /// 1. Update the reactive signal
    /// 2. Persist to localStorage (browser only)
    /// 3. Update the `data-theme` attribute on `<html>`
    pub fn set_theme(&self, theme_name: &str) {
        self.theme.set(theme_name.to_string());
        #[cfg(target_arch = "wasm32")]
        {
            let _ = LocalStorage::set(THEME_STORAGE_KEY, theme_name.to_string());
        }
    }

    /// Check if the current theme is a dark theme.
    ///
    /// Checks common dark theme names like "dark", "night", "dracula", etc.
    #[must_use]
    pub fn is_dark(&self) -> bool {
        let theme = self.theme.get();
        let theme_lower = theme.to_lowercase();
        theme_lower.contains("dark")
            || theme_lower.contains("night")
            || theme_lower.contains("dracula")
            || theme_lower.contains("cyberpunk")
            || theme_lower.contains("charcoal")
            || theme_lower.contains("dim")
    }

    /// Toggle between dark and light themes.
    ///
    /// If currently dark, switches to "light".
    /// If currently light, switches to "dark".
    pub fn toggle(&self) {
        let new_theme = if self.is_dark() { "light" } else { "dark" };
        self.set_theme(new_theme);
    }

    /// Get the current design system.
    #[must_use]
    pub fn design_system(&self) -> DesignSystem {
        self.design_system.get()
    }

    /// Get the design system signal for reactive updates.
    #[must_use]
    pub fn design_system_signal(&self) -> RwSignal<DesignSystem> {
        self.design_system
    }

    /// Set the design system.
    ///
    /// If `Auto` is provided, it will be resolved to the platform default.
    pub fn set_design_system(&self, system: DesignSystem) {
        let resolved = if system == DesignSystem::Auto {
            self.platform.default_design_system()
        } else {
            system
        };
        self.design_system.set(resolved);
        #[cfg(target_arch = "wasm32")]
        {
            let _ = LocalStorage::set(DESIGN_SYSTEM_STORAGE_KEY, resolved.as_str().to_string());
        }
    }

    /// Get the detected platform.
    #[must_use]
    pub const fn platform(&self) -> Platform {
        self.platform
    }

    /// Check if running on a mobile platform.
    #[must_use]
    pub const fn is_mobile(&self) -> bool {
        self.platform.is_mobile()
    }

    /// Check if running on a desktop platform.
    #[must_use]
    pub const fn is_desktop(&self) -> bool {
        self.platform.is_desktop()
    }

    /// Get the CSS class prefix for the current design system.
    #[must_use]
    pub fn class_prefix(&self) -> &'static str {
        self.design_system.get().class_prefix()
    }

    /// Generate a component class name with the current design system prefix.
    ///
    /// # Example
    /// ```ignore
    /// let btn_class = theme_ctx.component_class("btn", "primary");
    /// // Returns "fx-btn-ant fx-btn-ant-primary" for Ant design
    /// ```
    #[must_use]
    pub fn component_class(&self, component: &str, variant: &str) -> String {
        let prefix = self.class_prefix();
        format!(
            "fx-{}-{} fx-{}-{}-{}",
            component, prefix, component, prefix, variant
        )
    }

    /// Get minimum touch target size for the current design system.
    #[must_use]
    pub fn min_touch_target(&self) -> u32 {
        self.design_system.get().min_touch_target()
    }
}

/// Use the theme context from anywhere in the component tree.
///
/// # Panics
/// Panics if `ThemeProvider` is not in the component tree above this call.
#[must_use]
#[allow(clippy::expect_used)] // Intentional panic - use try_use_theme() for fallible version
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect(
        "ThemeContext not found. Make sure ThemeProvider is in the component tree."
    )
}

/// Try to use the theme context, returning None if not available.
#[must_use]
pub fn try_use_theme() -> Option<ThemeContext> {
    use_context::<ThemeContext>()
}

/// Load the saved theme from localStorage, or return default.
/// Returns default during SSR (non-wasm target).
#[must_use]
pub fn load_saved_theme(default: &str) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        LocalStorage::get(THEME_STORAGE_KEY).unwrap_or_else(|_| default.to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        default.to_string()
    }
}

/// Load the saved design system from localStorage, or return default.
/// Returns default during SSR (non-wasm target).
#[must_use]
pub fn load_saved_design_system(default: DesignSystem) -> DesignSystem {
    #[cfg(target_arch = "wasm32")]
    {
        LocalStorage::get::<String>(DESIGN_SYSTEM_STORAGE_KEY)
            .ok()
            .and_then(|s| DesignSystem::parse(&s))
            .unwrap_or(default)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        default
    }
}
