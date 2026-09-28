//! Design system trait definitions.

use ember_fx_common::DesignSystem;

/// Design tokens for a design system.
#[derive(Debug, Clone, Default)]
pub struct DesignTokens {
    /// Color palette
    pub colors: ColorPalette,
    /// Spacing scale
    pub spacing: Spacing,
    /// Typography settings
    pub typography: Typography,
    /// Border radius values
    pub radius: RadiusScale,
    /// Shadow definitions
    pub shadows: Shadows,
}

/// Color palette for a design system.
#[derive(Debug, Clone, Default)]
pub struct ColorPalette {
    /// Primary brand color
    pub primary: &'static str,
    /// Primary color for text on primary backgrounds
    pub primary_content: &'static str,
    /// Secondary brand color
    pub secondary: &'static str,
    /// Secondary content color
    pub secondary_content: &'static str,
    /// Accent color for highlights
    pub accent: &'static str,
    /// Accent content color
    pub accent_content: &'static str,
    /// Neutral/gray color
    pub neutral: &'static str,
    /// Neutral content color
    pub neutral_content: &'static str,
    /// Background colors
    pub base_100: &'static str,
    pub base_200: &'static str,
    pub base_300: &'static str,
    pub base_content: &'static str,
    /// Semantic colors
    pub info: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
}

/// Spacing scale for a design system.
#[derive(Debug, Clone, Default)]
pub struct Spacing {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
    pub xxl: &'static str,
}

/// Typography settings for a design system.
#[derive(Debug, Clone, Default)]
pub struct Typography {
    pub font_family: &'static str,
    pub font_size_xs: &'static str,
    pub font_size_sm: &'static str,
    pub font_size_md: &'static str,
    pub font_size_lg: &'static str,
    pub font_size_xl: &'static str,
    pub line_height: &'static str,
}

/// Border radius scale.
#[derive(Debug, Clone, Default)]
pub struct RadiusScale {
    pub none: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
    pub full: &'static str,
}

/// Shadow definitions.
#[derive(Debug, Clone, Default)]
pub struct Shadows {
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

/// Trait for design system implementations.
///
/// Each design system provides its own visual language, tokens, and defaults.
pub trait DesignSystemImpl {
    /// Get the design system enum variant.
    fn design_system(&self) -> DesignSystem;

    /// Get the human-readable name of this design system.
    fn name(&self) -> &'static str;

    /// Get the CSS class prefix for this design system.
    fn class_prefix(&self) -> &'static str;

    /// Get the default theme name for this design system.
    fn default_theme(&self) -> &'static str;

    /// Get the design tokens for this system.
    fn tokens(&self) -> &DesignTokens;

    /// Get the minimum touch target size in pixels.
    fn min_touch_target(&self) -> u32 {
        44
    }

    /// Whether this design system uses heavily rounded corners.
    fn uses_rounded_corners(&self) -> bool {
        true
    }

    /// Get component-specific class name.
    fn component_class(&self, component: &str) -> String {
        format!("fx-{}-{}", component, self.class_prefix())
    }

    /// Get component variant class name.
    fn variant_class(&self, component: &str, variant: &str) -> String {
        format!("fx-{}-{}-{}", component, self.class_prefix(), variant)
    }
}
