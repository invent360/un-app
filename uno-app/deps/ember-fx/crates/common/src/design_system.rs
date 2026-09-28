//! Design system definitions and utilities.
//!
//! Each design system provides a distinct visual language:
//! - **Ant**: Ant Design (default for web/desktop)
//! - **Material**: Material Design 3 (Android)
//! - **Cupertino**: Apple Human Interface Guidelines (iOS)
//! - **DaisyUI**: Web-friendly component library
//! - **Prime**: PrimeReact-inspired design
//! - **Flutter**: Flutter Material-like design

use serde::{Deserialize, Serialize};

/// Available design systems.
///
/// Design systems define the visual language and component styling.
/// Each system has its own CSS classes prefixed accordingly:
/// - Ant: `fx-btn-ant`, `fx-card-ant`, etc.
/// - Material: `fx-btn-material`, `fx-card-material`, etc.
/// - Cupertino: `fx-btn-cupertino`, `fx-card-cupertino`, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DesignSystem {
    /// Ant Design - Default for web and desktop platforms.
    /// Enterprise-grade design with comprehensive component library.
    #[default]
    Ant,

    /// Material Design 3 - Default for Android.
    /// Google's design system with dynamic color and motion.
    Material,

    /// Cupertino - Default for iOS.
    /// Apple Human Interface Guidelines inspired design.
    Cupertino,

    /// DaisyUI - Alternative web design system.
    /// Tailwind CSS-based component library.
    DaisyUI,

    /// PrimeReact-inspired design.
    /// Rich component library with 56+ themes.
    Prime,

    /// Flutter Material-like design.
    /// Cross-platform consistency with Flutter aesthetics.
    Flutter,

    /// Auto-detect based on platform.
    /// Will resolve to appropriate design system at runtime.
    Auto,
}

impl DesignSystem {
    /// Get the CSS class prefix for this design system.
    ///
    /// # Example
    /// ```
    /// use ember_fx_common::DesignSystem;
    /// assert_eq!(DesignSystem::Ant.class_prefix(), "ant");
    /// assert_eq!(DesignSystem::Material.class_prefix(), "material");
    /// ```
    #[must_use]
    pub const fn class_prefix(&self) -> &'static str {
        match self {
            Self::Ant => "ant",
            Self::Material => "material",
            Self::Cupertino => "cupertino",
            Self::DaisyUI => "daisy",
            Self::Prime => "prime",
            Self::Flutter => "flutter",
            Self::Auto => "ant", // Default to Ant when auto
        }
    }

    /// Get a human-readable display name.
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::Ant => "Ant Design",
            Self::Material => "Material Design",
            Self::Cupertino => "Cupertino",
            Self::DaisyUI => "DaisyUI",
            Self::Prime => "PrimeReact",
            Self::Flutter => "Flutter",
            Self::Auto => "Auto",
        }
    }

    /// Get all available design systems (excluding Auto).
    #[must_use]
    pub const fn all() -> &'static [DesignSystem] {
        &[
            Self::Ant,
            Self::Material,
            Self::Cupertino,
            Self::DaisyUI,
            Self::Prime,
            Self::Flutter,
        ]
    }

    /// Parse from a string (case-insensitive).
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ant" | "antdesign" | "ant-design" => Some(Self::Ant),
            "material" | "md" | "material3" => Some(Self::Material),
            "cupertino" | "ios" | "apple" => Some(Self::Cupertino),
            "daisyui" | "daisy" => Some(Self::DaisyUI),
            "prime" | "primereact" => Some(Self::Prime),
            "flutter" => Some(Self::Flutter),
            "auto" => Some(Self::Auto),
            _ => None,
        }
    }

    /// Convert to a string identifier.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Ant => "ant",
            Self::Material => "material",
            Self::Cupertino => "cupertino",
            Self::DaisyUI => "daisyui",
            Self::Prime => "prime",
            Self::Flutter => "flutter",
            Self::Auto => "auto",
        }
    }

    /// Get minimum touch target size for this design system.
    ///
    /// Returns the minimum recommended touch target size in pixels.
    /// Mobile-focused design systems have larger touch targets.
    #[must_use]
    pub const fn min_touch_target(&self) -> u32 {
        match self {
            Self::Cupertino => 44, // Apple HIG: 44x44 points
            Self::Material => 48,  // Material: 48x48 dp
            Self::Flutter => 48,   // Flutter Material: 48x48
            _ => 44,               // Default web: 44px
        }
    }

    /// Check if this design system uses rounded corners heavily.
    #[must_use]
    pub const fn uses_rounded_corners(&self) -> bool {
        match self {
            Self::Cupertino => true, // iOS loves rounded corners
            Self::Material => true,  // Material 3 uses rounded corners
            Self::Flutter => true,   // Flutter Material uses rounded
            Self::DaisyUI => true,   // DaisyUI has rounded by default
            Self::Ant => false,      // Ant uses subtle rounding
            Self::Prime => false,    // Prime is more rectangular
            Self::Auto => true,      // Default to rounded
        }
    }
}

impl std::fmt::Display for DesignSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_ant() {
        assert_eq!(DesignSystem::default(), DesignSystem::Ant);
    }

    #[test]
    fn test_class_prefix() {
        assert_eq!(DesignSystem::Ant.class_prefix(), "ant");
        assert_eq!(DesignSystem::Material.class_prefix(), "material");
        assert_eq!(DesignSystem::Cupertino.class_prefix(), "cupertino");
    }

    #[test]
    fn test_parse() {
        assert_eq!(DesignSystem::parse("ant"), Some(DesignSystem::Ant));
        assert_eq!(DesignSystem::parse("MATERIAL"), Some(DesignSystem::Material));
        assert_eq!(DesignSystem::parse("ios"), Some(DesignSystem::Cupertino));
        assert_eq!(DesignSystem::parse("invalid"), None);
    }

    #[test]
    fn test_touch_targets() {
        assert_eq!(DesignSystem::Cupertino.min_touch_target(), 44);
        assert_eq!(DesignSystem::Material.min_touch_target(), 48);
    }
}
