//! Size traits for component sizing.
//!
//! Provides a consistent sizing system across all components.

use std::fmt;

/// Standard component sizes.
///
/// These map to CSS class suffixes and design system tokens.
///
/// # Example
///
/// ```ignore
/// use ember_fx::core::ComponentSize;
///
/// let size = ComponentSize::Lg;
/// assert_eq!(size.as_str(), "lg");
/// assert_eq!(size.class_suffix(), "lg");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ComponentSize {
    /// Extra small (typically 24px height)
    Xs,
    /// Small (typically 28px height)
    Sm,
    /// Medium - default size (typically 32px height)
    #[default]
    Md,
    /// Large (typically 40px height)
    Lg,
    /// Extra large (typically 48px height)
    Xl,
    /// 2x extra large (typically 56px height)
    Xxl,
}

impl ComponentSize {
    /// Get the string representation for CSS classes.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
            Self::Xxl => "xxl",
        }
    }

    /// Get the CSS class suffix for this size.
    ///
    /// Same as `as_str()` but semantically indicates CSS usage.
    pub fn class_suffix(&self) -> &'static str {
        self.as_str()
    }

    /// Get the approximate height in pixels (for reference).
    ///
    /// Actual height depends on the design system.
    pub fn approx_height_px(&self) -> u32 {
        match self {
            Self::Xs => 24,
            Self::Sm => 28,
            Self::Md => 32,
            Self::Lg => 40,
            Self::Xl => 48,
            Self::Xxl => 56,
        }
    }

    /// Get all available sizes.
    pub fn all() -> &'static [ComponentSize] {
        &[
            Self::Xs,
            Self::Sm,
            Self::Md,
            Self::Lg,
            Self::Xl,
            Self::Xxl,
        ]
    }

    /// Parse from string (case-insensitive).
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "xs" | "extra-small" | "extrasmall" => Some(Self::Xs),
            "sm" | "small" => Some(Self::Sm),
            "md" | "medium" | "default" => Some(Self::Md),
            "lg" | "large" => Some(Self::Lg),
            "xl" | "extra-large" | "extralarge" => Some(Self::Xl),
            "xxl" | "2xl" => Some(Self::Xxl),
            _ => None,
        }
    }
}

impl fmt::Display for ComponentSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Trait for components that support standard sizing.
///
/// Most interactive components should implement this trait.
///
/// # Example
///
/// ```ignore
/// use ember_fx::core::{Sizable, ComponentSize};
///
/// struct Button {
///     size: ComponentSize,
/// }
///
/// impl Sizable for Button {
///     fn size(&self) -> ComponentSize {
///         self.size
///     }
/// }
///
/// let btn = Button { size: ComponentSize::Lg };
/// assert_eq!(btn.size_class("fx-button-ant"), "fx-button-ant-lg");
/// ```
pub trait Sizable {
    /// Get the current size.
    fn size(&self) -> ComponentSize;

    /// Generate the size-specific CSS class.
    ///
    /// Format: `{base-class}-{size}`
    fn size_class(&self, base_class: &str) -> String {
        format!("{}-{}", base_class, self.size().class_suffix())
    }

    /// Check if using the default size.
    fn is_default_size(&self) -> bool {
        self.size() == ComponentSize::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_as_str() {
        assert_eq!(ComponentSize::Xs.as_str(), "xs");
        assert_eq!(ComponentSize::Md.as_str(), "md");
        assert_eq!(ComponentSize::Xxl.as_str(), "xxl");
    }

    #[test]
    fn test_size_from_str() {
        assert_eq!(ComponentSize::from_str("lg"), Some(ComponentSize::Lg));
        assert_eq!(ComponentSize::from_str("LARGE"), Some(ComponentSize::Lg));
        assert_eq!(ComponentSize::from_str("invalid"), None);
    }

    #[test]
    fn test_default_size() {
        assert_eq!(ComponentSize::default(), ComponentSize::Md);
    }
}
