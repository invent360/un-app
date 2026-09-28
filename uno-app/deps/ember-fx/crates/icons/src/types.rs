//! Shared types for the icon system.

/// Icon variant - outline or filled.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum IconVariant {
    /// Outline/stroke-based icon (default).
    #[default]
    Outline,
    /// Filled/solid icon.
    Filled,
}

impl IconVariant {
    /// Returns the variant as a string suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Outline => "outline",
            Self::Filled => "filled",
        }
    }
}

/// Icon size presets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum IconSize {
    /// Extra small (12px).
    Xs,
    /// Small (16px).
    Sm,
    /// Medium (20px) - default.
    #[default]
    Md,
    /// Large (24px).
    Lg,
    /// Extra large (32px).
    Xl,
    /// Double extra large (48px).
    Xxl,
}

impl IconSize {
    /// Returns the CSS class for this size.
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Xs => "fx-icon-xs",
            Self::Sm => "fx-icon-sm",
            Self::Md => "fx-icon-md",
            Self::Lg => "fx-icon-lg",
            Self::Xl => "fx-icon-xl",
            Self::Xxl => "fx-icon-xxl",
        }
    }

    /// Returns the pixel size.
    pub fn to_px(&self) -> u32 {
        match self {
            Self::Xs => 12,
            Self::Sm => 16,
            Self::Md => 20,
            Self::Lg => 24,
            Self::Xl => 32,
            Self::Xxl => 48,
        }
    }

    /// Returns the Tailwind-compatible class.
    pub fn to_tailwind(&self) -> &'static str {
        match self {
            Self::Xs => "w-3 h-3",
            Self::Sm => "w-4 h-4",
            Self::Md => "w-5 h-5",
            Self::Lg => "w-6 h-6",
            Self::Xl => "w-8 h-8",
            Self::Xxl => "w-12 h-12",
        }
    }
}

/// Icon color semantic presets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum IconColor {
    /// Inherit from parent (currentColor) - default.
    #[default]
    Inherit,
    /// Primary theme color.
    Primary,
    /// Secondary/muted color.
    Secondary,
    /// Success/positive color.
    Success,
    /// Warning color.
    Warning,
    /// Error/danger color.
    Error,
    /// Info color.
    Info,
}

impl IconColor {
    /// Returns the CSS class for this color.
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Inherit => "",
            Self::Primary => "fx-icon-primary",
            Self::Secondary => "fx-icon-secondary",
            Self::Success => "fx-icon-success",
            Self::Warning => "fx-icon-warning",
            Self::Error => "fx-icon-error",
            Self::Info => "fx-icon-info",
        }
    }
}
