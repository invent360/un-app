//! Text and layout direction.

use serde::{Deserialize, Serialize};

/// Layout direction for internationalization support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    /// Left-to-right (default for most Western languages).
    #[default]
    Ltr,
    /// Right-to-left (for Arabic, Hebrew, etc.).
    Rtl,
}

impl Direction {
    /// Get the HTML `dir` attribute value.
    #[must_use]
    pub const fn as_html_dir(&self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }

    /// Check if this is right-to-left.
    #[must_use]
    pub const fn is_rtl(&self) -> bool {
        matches!(self, Self::Rtl)
    }

    /// Check if this is left-to-right.
    #[must_use]
    pub const fn is_ltr(&self) -> bool {
        matches!(self, Self::Ltr)
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_html_dir())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_ltr() {
        assert_eq!(Direction::default(), Direction::Ltr);
    }

    #[test]
    fn test_is_rtl() {
        assert!(!Direction::Ltr.is_rtl());
        assert!(Direction::Rtl.is_rtl());
    }
}
