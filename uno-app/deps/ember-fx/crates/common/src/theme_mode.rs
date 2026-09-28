//! Theme mode (light/dark/system).

use serde::{Deserialize, Serialize};

/// Theme color mode preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    /// Light color scheme.
    Light,
    /// Dark color scheme.
    #[default]
    Dark,
    /// Follow system preference via `prefers-color-scheme`.
    System,
}

impl ThemeMode {
    /// Get the `data-theme` attribute value.
    ///
    /// Note: For `System`, this returns the effective value based on
    /// system preference detection done elsewhere.
    #[must_use]
    pub const fn as_data_theme(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::System => "system",
        }
    }

    /// Check if this is a dark theme.
    #[must_use]
    pub const fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }

    /// Check if this is a light theme.
    #[must_use]
    pub const fn is_light(&self) -> bool {
        matches!(self, Self::Light)
    }

    /// Check if this follows system preference.
    #[must_use]
    pub const fn is_system(&self) -> bool {
        matches!(self, Self::System)
    }

    /// Toggle between light and dark modes.
    /// System mode toggles to light.
    #[must_use]
    pub const fn toggle(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
            Self::System => Self::Light,
        }
    }
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_data_theme())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_dark() {
        assert_eq!(ThemeMode::default(), ThemeMode::Dark);
    }

    #[test]
    fn test_toggle() {
        assert_eq!(ThemeMode::Light.toggle(), ThemeMode::Dark);
        assert_eq!(ThemeMode::Dark.toggle(), ThemeMode::Light);
        assert_eq!(ThemeMode::System.toggle(), ThemeMode::Light);
    }
}
