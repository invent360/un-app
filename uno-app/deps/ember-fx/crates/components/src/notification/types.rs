//! Feedback component type definitions.
//!
//! Shared types for notification components (Alert, Toast, Badge, Tag, Progress, etc.).

use std::fmt;

/// Alert/notification severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum AlertType {
    /// Informational message
    #[default]
    Info,
    /// Success message
    Success,
    /// Warning message
    Warning,
    /// Error message
    Error,
}

impl AlertType {
    /// Get the CSS suffix for this alert type.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Get the full CSS class for this alert type.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }

    /// Get the default icon for this alert type.
    pub fn default_icon(&self) -> &'static str {
        match self {
            Self::Info => "ℹ",
            Self::Success => "✓",
            Self::Warning => "⚠",
            Self::Error => "✕",
        }
    }
}

impl fmt::Display for AlertType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Badge status/color variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum BadgeStatus {
    /// Default badge
    #[default]
    Default,
    /// Processing/active state
    Processing,
    /// Success state
    Success,
    /// Warning state
    Warning,
    /// Error state
    Error,
}

impl BadgeStatus {
    /// Get the CSS suffix for this status.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Processing => "processing",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Get the full CSS class for this status.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl fmt::Display for BadgeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Tag color variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TagColor {
    /// Default tag color
    #[default]
    Default,
    /// Primary color
    Primary,
    /// Success color
    Success,
    /// Warning color
    Warning,
    /// Error color
    Error,
    /// Processing/info color
    Processing,
    /// Magenta
    Magenta,
    /// Red
    Red,
    /// Volcano
    Volcano,
    /// Orange
    Orange,
    /// Gold
    Gold,
    /// Lime
    Lime,
    /// Green
    Green,
    /// Cyan
    Cyan,
    /// Blue
    Blue,
    /// Geekblue
    Geekblue,
    /// Purple
    Purple,
}

impl TagColor {
    /// Get the CSS suffix for this color.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Primary => "primary",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Processing => "processing",
            Self::Magenta => "magenta",
            Self::Red => "red",
            Self::Volcano => "volcano",
            Self::Orange => "orange",
            Self::Gold => "gold",
            Self::Lime => "lime",
            Self::Green => "green",
            Self::Cyan => "cyan",
            Self::Blue => "blue",
            Self::Geekblue => "geekblue",
            Self::Purple => "purple",
        }
    }

    /// Get the full CSS class for this color.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl fmt::Display for TagColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Progress bar type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ProgressType {
    /// Linear progress bar
    #[default]
    Line,
    /// Circular progress
    Circle,
    /// Dashboard style (semi-circle)
    Dashboard,
}

impl ProgressType {
    /// Get the CSS suffix for this type.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Circle => "circle",
            Self::Dashboard => "dashboard",
        }
    }

    /// Get the full CSS class for this type.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl fmt::Display for ProgressType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Progress bar status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ProgressStatus {
    /// Normal/active progress
    #[default]
    Normal,
    /// Success state
    Success,
    /// Exception/error state
    Exception,
    /// Active/animated state
    Active,
}

impl ProgressStatus {
    /// Get the CSS suffix for this status.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Success => "success",
            Self::Exception => "exception",
            Self::Active => "active",
        }
    }

    /// Get the full CSS class for this status.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl fmt::Display for ProgressStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Spinner size variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum SpinnerSize {
    /// Small spinner
    Small,
    /// Default/medium spinner
    #[default]
    Default,
    /// Large spinner
    Large,
}

impl SpinnerSize {
    /// Get the CSS suffix for this size.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "default",
            Self::Large => "lg",
        }
    }

    /// Get the full CSS class for this size.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl fmt::Display for SpinnerSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Toast/notification placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ToastPlacement {
    /// Top left corner
    TopLeft,
    /// Top center
    TopCenter,
    /// Top right corner (default)
    #[default]
    TopRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom center
    BottomCenter,
    /// Bottom right corner
    BottomRight,
}

impl ToastPlacement {
    /// Get the CSS suffix for this placement.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::TopLeft => "top-left",
            Self::TopCenter => "top-center",
            Self::TopRight => "top-right",
            Self::BottomLeft => "bottom-left",
            Self::BottomCenter => "bottom-center",
            Self::BottomRight => "bottom-right",
        }
    }

    /// Get the full CSS class for this placement.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl fmt::Display for ToastPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_type() {
        assert_eq!(AlertType::default(), AlertType::Info);
        assert_eq!(AlertType::Success.as_suffix(), "success");
        assert_eq!(AlertType::Error.class("fx-alert"), "fx-alert-error");
    }

    #[test]
    fn test_badge_status() {
        assert_eq!(BadgeStatus::default(), BadgeStatus::Default);
        assert_eq!(BadgeStatus::Processing.as_suffix(), "processing");
    }

    #[test]
    fn test_tag_color() {
        assert_eq!(TagColor::default(), TagColor::Default);
        assert_eq!(TagColor::Cyan.as_suffix(), "cyan");
    }

    #[test]
    fn test_progress_type() {
        assert_eq!(ProgressType::default(), ProgressType::Line);
        assert_eq!(ProgressType::Circle.as_suffix(), "circle");
    }

    #[test]
    fn test_spinner_size() {
        assert_eq!(SpinnerSize::default(), SpinnerSize::Default);
        assert_eq!(SpinnerSize::Large.as_suffix(), "lg");
    }
}
