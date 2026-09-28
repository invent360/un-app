//! Ant Design system implementation.

use crate::traits::{
    ColorPalette, DesignSystemImpl, DesignTokens, RadiusScale, Shadows, Spacing, Typography,
};
use ember_fx_common::DesignSystem;

/// Ant Design system implementation.
///
/// Enterprise-grade design with comprehensive component library.
/// Default for web and desktop platforms.
#[derive(Debug, Clone, Default)]
pub struct AntDesignSystem {
    tokens: DesignTokens,
}

impl AntDesignSystem {
    /// Create a new Ant Design system with default tokens.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tokens: Self::default_tokens(),
        }
    }

    /// Get default design tokens for Ant Design.
    #[must_use]
    pub fn default_tokens() -> DesignTokens {
        DesignTokens {
            colors: ColorPalette {
                primary: "#1677ff",
                primary_content: "#ffffff",
                secondary: "#722ed1",
                secondary_content: "#ffffff",
                accent: "#13c2c2",
                accent_content: "#ffffff",
                neutral: "#141414",
                neutral_content: "#ffffff",
                base_100: "#ffffff",
                base_200: "#f5f5f5",
                base_300: "#d9d9d9",
                base_content: "#000000e0",
                info: "#1677ff",
                success: "#52c41a",
                warning: "#faad14",
                error: "#ff4d4f",
            },
            spacing: Spacing {
                xs: "4px",
                sm: "8px",
                md: "16px",
                lg: "24px",
                xl: "32px",
                xxl: "48px",
            },
            typography: Typography {
                font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif",
                font_size_xs: "12px",
                font_size_sm: "14px",
                font_size_md: "16px",
                font_size_lg: "18px",
                font_size_xl: "20px",
                line_height: "1.5715",
            },
            radius: RadiusScale {
                none: "0",
                sm: "2px",
                md: "6px",
                lg: "8px",
                xl: "12px",
                full: "9999px",
            },
            shadows: Shadows {
                sm: "0 1px 2px 0 rgba(0, 0, 0, 0.03), 0 1px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px 0 rgba(0, 0, 0, 0.02)",
                md: "0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 9px 28px 8px rgba(0, 0, 0, 0.05)",
                lg: "0 6px 16px -8px rgba(0, 0, 0, 0.08), 0 9px 28px 0 rgba(0, 0, 0, 0.05), 0 12px 48px 16px rgba(0, 0, 0, 0.03)",
                xl: "0 12px 24px -12px rgba(0, 0, 0, 0.12), 0 16px 48px 0 rgba(0, 0, 0, 0.08)",
            },
        }
    }
}

impl DesignSystemImpl for AntDesignSystem {
    fn design_system(&self) -> DesignSystem {
        DesignSystem::Ant
    }

    fn name(&self) -> &'static str {
        "Ant Design"
    }

    fn class_prefix(&self) -> &'static str {
        "ant"
    }

    fn default_theme(&self) -> &'static str {
        "dark"
    }

    fn tokens(&self) -> &DesignTokens {
        &self.tokens
    }

    fn min_touch_target(&self) -> u32 {
        44
    }

    fn uses_rounded_corners(&self) -> bool {
        false // Ant uses subtle rounding
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ant_design_system() {
        let system = AntDesignSystem::new();
        assert_eq!(system.name(), "Ant Design");
        assert_eq!(system.class_prefix(), "ant");
        assert_eq!(system.default_theme(), "dark");
    }

    #[test]
    fn test_component_class() {
        let system = AntDesignSystem::new();
        assert_eq!(system.component_class("btn"), "fx-btn-ant");
        assert_eq!(system.variant_class("btn", "primary"), "fx-btn-ant-primary");
    }
}
