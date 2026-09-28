use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};
#[cfg(target_arch = "wasm32")]
use web_sys::window;

const THEME_KEY: &str = "unity_dashboard_theme";

/// Theme variants
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    DarkBlue,
}

impl Theme {
    pub fn css_class(&self) -> &'static str {
        match self {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
            Theme::DarkBlue => "theme-dark-blue",
        }
    }

    /// Returns the ember-fx data-theme attribute value (Prime themes for uno-admin)
    pub fn ember_fx_theme(&self) -> &'static str {
        match self {
            Theme::Light => "lara-light",
            Theme::Dark => "lara-dark",
            Theme::DarkBlue => "aura-dark",
        }
    }

    pub fn next(&self) -> Theme {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::DarkBlue,
            Theme::DarkBlue => Theme::Light,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::DarkBlue => "Unity Blue",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            "dark-blue" => Theme::DarkBlue,
            _ => Theme::Light,
        }
    }

    pub fn to_storage_string(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::DarkBlue => "dark-blue",
        }
    }

    pub fn all() -> [Theme; 3] {
        [Theme::Light, Theme::Dark, Theme::DarkBlue]
    }
}

/// Apply theme class to the HTML element (client-side only)
#[cfg(target_arch = "wasm32")]
fn apply_theme_to_dom(theme: Theme) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                // Remove all theme classes
                let class_list = html.class_list();
                let _ = class_list.remove_1("theme-light");
                let _ = class_list.remove_1("theme-dark");
                let _ = class_list.remove_1("theme-dark-blue");
                let _ = class_list.remove_1("dark"); // Tailwind dark mode class

                // Add new theme class
                let _ = class_list.add_1(theme.css_class());

                // Add Tailwind 'dark' class for dark themes
                if matches!(theme, Theme::Dark | Theme::DarkBlue) {
                    let _ = class_list.add_1("dark");
                }

                // Set ember-fx data-theme attribute for ember-fx-styles CSS variables
                let _ = html.set_attribute("data-theme", theme.ember_fx_theme());
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_theme_to_dom(_theme: Theme) {
    // No-op on server
}

/// Save theme to LocalStorage (client-side only)
#[cfg(target_arch = "wasm32")]
fn save_theme_to_storage(theme: Theme) {
    let _ = LocalStorage::set(THEME_KEY, theme.to_storage_string());
}

#[cfg(not(target_arch = "wasm32"))]
fn save_theme_to_storage(_theme: Theme) {
    // No-op on server
}

/// Load theme from LocalStorage (client-side only)
#[cfg(target_arch = "wasm32")]
fn load_theme_from_storage() -> Theme {
    let stored: String = LocalStorage::get(THEME_KEY).unwrap_or_else(|_| "light".to_string());
    Theme::from_str(&stored)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_theme_from_storage() -> Theme {
    Theme::default()
}

/// Theme context holding the reactive theme state
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: RwSignal<Theme>,
}

impl ThemeContext {
    /// Get current theme
    pub fn current(&self) -> Theme {
        self.theme.get()
    }

    /// Check if current theme is dark (either dark or dark-blue)
    pub fn is_dark(&self) -> bool {
        matches!(self.theme.get(), Theme::Dark | Theme::DarkBlue)
    }

    /// Cycle to next theme
    pub fn toggle(&self) {
        let new_theme = self.theme.get().next();
        self.set_theme(new_theme);
    }

    /// Set a specific theme
    pub fn set_theme(&self, theme: Theme) {
        self.theme.set(theme);
        apply_theme_to_dom(theme);
        save_theme_to_storage(theme);
    }
}

/// Hook to access theme context
pub fn use_theme() -> ThemeContext {
    expect_context::<ThemeContext>()
}

/// Theme context provider component
#[component]
pub fn ThemeContextProvider(children: Children) -> impl IntoView {
    // Always start with default theme for SSR/hydration consistency
    let theme_signal = RwSignal::new(Theme::default());

    // Provide context first so children can access it
    provide_context(ThemeContext { theme: theme_signal });

    // Load actual theme from localStorage AFTER hydration (client-side only)
    Effect::new(move |_| {
        let stored_theme = load_theme_from_storage();
        if stored_theme != Theme::default() {
            theme_signal.set(stored_theme);
            apply_theme_to_dom(stored_theme);
        } else {
            // Apply default theme to DOM
            apply_theme_to_dom(Theme::default());
        }
    });

    children()
}
