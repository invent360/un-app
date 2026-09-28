//! Visual traits for component appearance.
//!
//! These traits control the visual styling of components including
//! theming, colors, borders, and corner radius.

use std::fmt;

/// Semantic color variants for components.
///
/// Maps to design system color tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ComponentColor {
    /// Primary brand color
    #[default]
    Primary,
    /// Secondary/muted color
    Secondary,
    /// Accent/highlight color
    Accent,
    /// Neutral/gray color
    Neutral,
    /// Informational (blue)
    Info,
    /// Success/positive (green)
    Success,
    /// Warning/caution (yellow/orange)
    Warning,
    /// Error/danger (red)
    Error,
    /// Ghost/transparent
    Ghost,
}

impl ComponentColor {
    /// Get the string representation for CSS classes.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Accent => "accent",
            Self::Neutral => "neutral",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Ghost => "ghost",
        }
    }

    /// Get CSS class suffix.
    pub fn class_suffix(&self) -> &'static str {
        self.as_str()
    }

    /// Check if this is a semantic status color.
    pub fn is_status(&self) -> bool {
        matches!(self, Self::Info | Self::Success | Self::Warning | Self::Error)
    }

    /// Parse from string (case-insensitive).
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "primary" => Some(Self::Primary),
            "secondary" => Some(Self::Secondary),
            "accent" => Some(Self::Accent),
            "neutral" => Some(Self::Neutral),
            "info" => Some(Self::Info),
            "success" => Some(Self::Success),
            "warning" => Some(Self::Warning),
            "error" | "danger" => Some(Self::Error),
            "ghost" | "transparent" => Some(Self::Ghost),
            _ => None,
        }
    }
}

impl fmt::Display for ComponentColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Border radius presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum BorderRadius {
    /// No rounding (0)
    None,
    /// Small rounding (0.25rem)
    Sm,
    /// Medium rounding (0.5rem) - default
    #[default]
    Md,
    /// Large rounding (1rem)
    Lg,
    /// Extra large rounding (1.5rem)
    Xl,
    /// Fully rounded (9999px)
    Full,
}

impl BorderRadius {
    /// Get the string representation for CSS classes.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
            Self::Full => "full",
        }
    }

    /// Get the CSS class suffix.
    pub fn class_suffix(&self) -> &'static str {
        self.as_str()
    }

    /// Get the CSS value for this radius.
    pub fn css_value(&self) -> &'static str {
        match self {
            Self::None => "0",
            Self::Sm => "var(--fx-radius-sm, 0.25rem)",
            Self::Md => "var(--fx-radius-md, 0.5rem)",
            Self::Lg => "var(--fx-radius-lg, 1rem)",
            Self::Xl => "var(--fx-radius-xl, 1.5rem)",
            Self::Full => "var(--fx-radius-full, 9999px)",
        }
    }
}

impl fmt::Display for BorderRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Trait for components that support theming.
///
/// Components implementing this trait can adapt their appearance
/// based on the current theme context.
pub trait Themed {
    /// Check if the component should use dark mode styling.
    fn is_dark_mode(&self) -> bool {
        false
    }

    /// Get the theme-specific CSS class modifier.
    fn theme_class(&self, base_class: &str) -> Option<String> {
        if self.is_dark_mode() {
            Some(format!("{}-dark", base_class))
        } else {
            None
        }
    }
}

/// Trait for components that support color variants.
pub trait Colored {
    /// Get the current color variant.
    fn color(&self) -> ComponentColor;

    /// Generate the color-specific CSS class.
    fn color_class(&self, base_class: &str) -> String {
        format!("{}-{}", base_class, self.color().class_suffix())
    }
}

/// Trait for components with configurable border radius.
pub trait Rounded {
    /// Get the current border radius.
    fn radius(&self) -> BorderRadius;

    /// Generate the radius-specific CSS class.
    fn radius_class(&self, base_class: &str) -> String {
        format!("{}-rounded-{}", base_class, self.radius().class_suffix())
    }

    /// Check if the component is fully rounded (pill shape).
    fn is_pill(&self) -> bool {
        self.radius() == BorderRadius::Full
    }
}

/// Trait for components with configurable borders.
pub trait Bordered {
    /// Check if the component has a visible border.
    fn has_border(&self) -> bool;

    /// Get the border width (0 = no border).
    fn border_width(&self) -> u8 {
        if self.has_border() { 1 } else { 0 }
    }

    /// Generate the border CSS class.
    fn border_class(&self, base_class: &str) -> Option<String> {
        if self.has_border() {
            Some(format!("{}-bordered", base_class))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_variants() {
        assert_eq!(ComponentColor::Primary.as_str(), "primary");
        assert_eq!(ComponentColor::Error.as_str(), "error");
        assert!(ComponentColor::Success.is_status());
        assert!(!ComponentColor::Primary.is_status());
    }

    #[test]
    fn test_border_radius() {
        assert_eq!(BorderRadius::default(), BorderRadius::Md);
        assert_eq!(BorderRadius::Full.as_str(), "full");
    }

    #[test]
    fn test_color_from_str() {
        assert_eq!(ComponentColor::from_str("danger"), Some(ComponentColor::Error));
        assert_eq!(ComponentColor::from_str("PRIMARY"), Some(ComponentColor::Primary));
    }
}
