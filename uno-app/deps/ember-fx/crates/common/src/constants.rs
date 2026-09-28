//! Shared constants for ember-fx.

/// The localStorage key used for persisting theme preference.
pub const THEME_STORAGE_KEY: &str = "ember_theme";

/// The localStorage key used for persisting design system preference.
pub const DESIGN_SYSTEM_STORAGE_KEY: &str = "ember_design_system";

/// The ID prefix for injected style elements.
pub const STYLE_ID_PREFIX: &str = "ember-";

/// The default theme name.
pub const DEFAULT_THEME: &str = "dark";

/// The attribute name used on the HTML element for theme switching.
pub const THEME_ATTRIBUTE: &str = "data-theme";

/// The attribute name used on the HTML element for design system.
pub const DESIGN_SYSTEM_ATTRIBUTE: &str = "data-design-system";

/// CSS class prefix for all ember-fx components.
pub const CLASS_PREFIX: &str = "fx-";
