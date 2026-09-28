//! Size variants for components.

use serde::{Deserialize, Serialize};

/// Common size variants used across components.
///
/// These correspond to standard sizing scales that map to CSS classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Size {
    /// Extra small size.
    Xs,
    /// Small size.
    Sm,
    /// Medium size (default).
    #[default]
    Md,
    /// Large size.
    Lg,
    /// Extra large size.
    Xl,
}

impl Size {
    /// Convert to a CSS class suffix.
    #[must_use]
    pub const fn as_class_suffix(&self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
        }
    }

    /// Get a human-readable display name.
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::Xs => "Extra Small",
            Self::Sm => "Small",
            Self::Md => "Medium",
            Self::Lg => "Large",
            Self::Xl => "Extra Large",
        }
    }

    /// Get all sizes in order from smallest to largest.
    #[must_use]
    pub const fn all() -> &'static [Size] {
        &[Self::Xs, Self::Sm, Self::Md, Self::Lg, Self::Xl]
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_md() {
        assert_eq!(Size::default(), Size::Md);
    }

    #[test]
    fn test_class_suffix() {
        assert_eq!(Size::Xs.as_class_suffix(), "xs");
        assert_eq!(Size::Md.as_class_suffix(), "md");
    }
}
