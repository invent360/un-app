//! Core wizard types and traits.
//!
//! Defines the fundamental types used throughout the wizard system.

use std::fmt::Debug;
use std::hash::Hash;

/// Trait that must be implemented by wizard stage enums.
///
/// This trait defines the contract for wizard stages, enabling type-safe
/// navigation and state management.
///
/// # Example
///
/// ```rust
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// enum OnboardingStage {
///     Welcome,
///     PersonalInfo,
///     Preferences,
///     Review,
/// }
///
/// impl WizardStage for OnboardingStage {
///     fn display_name(&self) -> &'static str {
///         match self {
///             Self::Welcome => "Welcome",
///             Self::PersonalInfo => "Personal Info",
///             Self::Preferences => "Preferences",
///             Self::Review => "Review",
///         }
///     }
///
///     fn all_stages() -> Vec<Self> {
///         vec![Self::Welcome, Self::PersonalInfo, Self::Preferences, Self::Review]
///     }
/// }
/// ```
pub trait WizardStage: Debug + Clone + PartialEq + Eq + Hash + Send + Sync + 'static {
    /// Returns the display name for this stage.
    fn display_name(&self) -> &'static str;

    /// Returns all possible stages in order.
    fn all_stages() -> Vec<Self>;

    /// Returns whether this stage is optional (can be skipped).
    fn is_optional(&self) -> bool {
        false
    }

    /// Returns the index of this stage in the ordered list.
    fn index(&self) -> usize {
        Self::all_stages()
            .iter()
            .position(|s| s == self)
            .unwrap_or(0)
    }
}

/// Wizard layout variants for responsive design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WizardLayout {
    /// Vertical layout (mobile-first, stacked).
    Vertical,
    /// Horizontal layout with sidebar (desktop).
    Horizontal,
    /// Auto-adapt based on screen width.
    #[default]
    Auto,
}

impl WizardLayout {
    /// Returns the CSS class suffix for this layout.
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Auto => "auto",
        }
    }
}

/// Stepper orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepperOrientation {
    /// Horizontal stepper (steps side by side).
    #[default]
    Horizontal,
    /// Vertical stepper (steps stacked).
    Vertical,
}

impl StepperOrientation {
    /// Returns the CSS class suffix for this orientation.
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Navigation button alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavigationAlignment {
    /// Buttons aligned to the left.
    Left,
    /// Buttons aligned to the right.
    Right,
    /// Buttons centered.
    Center,
    /// Buttons spread with space between.
    #[default]
    SpaceBetween,
}

impl NavigationAlignment {
    /// Returns the CSS class suffix for this alignment.
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Center => "center",
            Self::SpaceBetween => "space-between",
        }
    }
}

/// Error display style variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErrorDisplayStyle {
    /// Toast notification (floating, auto-dismiss).
    Toast,
    /// Alert banner (inline, persistent).
    #[default]
    Alert,
    /// Inline error (minimal, field-level).
    Inline,
}

impl ErrorDisplayStyle {
    /// Returns the CSS class suffix for this style.
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Self::Toast => "toast",
            Self::Alert => "alert",
            Self::Inline => "inline",
        }
    }
}

/// Progress display variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressStyle {
    /// Linear progress bar.
    #[default]
    Bar,
    /// Circular progress indicator.
    Circle,
    /// Step counter text only.
    Counter,
}

impl ProgressStyle {
    /// Returns the CSS class suffix for this style.
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Self::Bar => "bar",
            Self::Circle => "circle",
            Self::Counter => "counter",
        }
    }
}
