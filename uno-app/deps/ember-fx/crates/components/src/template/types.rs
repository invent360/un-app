//! Template type definitions.
//!
//! Each component type has its own template enum defining available
//! structural variants.

use std::fmt;

/// Input component structural templates.
///
/// These control how input fields are laid out, not how they look.
///
/// # Variants
///
/// - `Standard`: Label above, input below (most common)
/// - `Floating`: Label floats inside input, moves up on focus
/// - `Material`: Material Design with underline and floating label
/// - `Inline`: Label and input on same line
/// - `Minimal`: Input only, no wrapper elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum InputTemplate {
    /// Standard layout: label above input
    #[default]
    Standard,
    /// Floating label that moves up on focus/fill
    Floating,
    /// Material Design style with underline
    Material,
    /// Label and input inline (side by side)
    Inline,
    /// Minimal - input only, no wrapper
    Minimal,
}

impl InputTemplate {
    /// Get template identifier string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Floating => "floating",
            Self::Material => "material",
            Self::Inline => "inline",
            Self::Minimal => "minimal",
        }
    }

    /// Get CSS class for this template.
    pub fn class(&self) -> &'static str {
        match self {
            Self::Standard => "fx-template-standard",
            Self::Floating => "fx-template-floating",
            Self::Material => "fx-template-material",
            Self::Inline => "fx-template-inline",
            Self::Minimal => "fx-template-minimal",
        }
    }

    /// Check if this template uses a floating label.
    pub fn has_floating_label(&self) -> bool {
        matches!(self, Self::Floating | Self::Material)
    }

    /// Check if this template has a wrapper element.
    pub fn has_wrapper(&self) -> bool {
        !matches!(self, Self::Minimal)
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "standard" | "default" => Some(Self::Standard),
            "floating" | "float" => Some(Self::Floating),
            "material" | "md" | "mdc" => Some(Self::Material),
            "inline" | "horizontal" => Some(Self::Inline),
            "minimal" | "bare" | "none" => Some(Self::Minimal),
            _ => None,
        }
    }
}

impl fmt::Display for InputTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Button component structural templates.
///
/// Most buttons use the standard template, but some variants
/// have different structure (icon-only, split button, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ButtonTemplate {
    /// Standard button with optional icon + text
    #[default]
    Standard,
    /// Icon-only button (square, centered icon)
    Icon,
    /// Split button with main action + dropdown
    Split,
    /// Button group item (no rounded corners on sides)
    GroupItem,
}

impl ButtonTemplate {
    /// Get template identifier string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Icon => "icon",
            Self::Split => "split",
            Self::GroupItem => "group-item",
        }
    }

    /// Get CSS class for this template.
    pub fn class(&self) -> &'static str {
        match self {
            Self::Standard => "fx-btn-template-standard",
            Self::Icon => "fx-btn-template-icon",
            Self::Split => "fx-btn-template-split",
            Self::GroupItem => "fx-btn-template-group-item",
        }
    }

    /// Check if this is an icon-only button.
    pub fn is_icon_only(&self) -> bool {
        matches!(self, Self::Icon)
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "standard" | "default" | "text" => Some(Self::Standard),
            "icon" | "icon-only" | "square" => Some(Self::Icon),
            "split" | "dropdown" => Some(Self::Split),
            "group" | "group-item" => Some(Self::GroupItem),
            _ => None,
        }
    }
}

impl fmt::Display for ButtonTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Card component structural templates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum CardTemplate {
    /// Standard card with header, body, footer sections
    #[default]
    Standard,
    /// Compact card without header/footer separation
    Compact,
    /// Media card with image/video at top
    Media,
    /// Horizontal card with image on side
    Horizontal,
}

impl CardTemplate {
    /// Get template identifier string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Compact => "compact",
            Self::Media => "media",
            Self::Horizontal => "horizontal",
        }
    }

    /// Get CSS class for this template.
    pub fn class(&self) -> &'static str {
        match self {
            Self::Standard => "fx-card-template-standard",
            Self::Compact => "fx-card-template-compact",
            Self::Media => "fx-card-template-media",
            Self::Horizontal => "fx-card-template-horizontal",
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "standard" | "default" => Some(Self::Standard),
            "compact" | "simple" => Some(Self::Compact),
            "media" | "image" => Some(Self::Media),
            "horizontal" | "side" => Some(Self::Horizontal),
            _ => None,
        }
    }
}

impl fmt::Display for CardTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Generic template variant that can hold any template type.
///
/// Useful for APIs that need to work with multiple template types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateVariant {
    Input(InputTemplate),
    Button(ButtonTemplate),
    Card(CardTemplate),
}

impl TemplateVariant {
    /// Get the CSS class for this template variant.
    pub fn class(&self) -> &'static str {
        match self {
            Self::Input(t) => t.class(),
            Self::Button(t) => t.class(),
            Self::Card(t) => t.class(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_template() {
        assert_eq!(InputTemplate::default(), InputTemplate::Standard);
        assert_eq!(InputTemplate::Floating.as_str(), "floating");
        assert!(InputTemplate::Material.has_floating_label());
        assert!(!InputTemplate::Standard.has_floating_label());
    }

    #[test]
    fn test_input_template_from_str() {
        assert_eq!(InputTemplate::from_str("floating"), Some(InputTemplate::Floating));
        assert_eq!(InputTemplate::from_str("MD"), Some(InputTemplate::Material));
        assert_eq!(InputTemplate::from_str("invalid"), None);
    }

    #[test]
    fn test_button_template() {
        assert_eq!(ButtonTemplate::default(), ButtonTemplate::Standard);
        assert!(ButtonTemplate::Icon.is_icon_only());
        assert!(!ButtonTemplate::Standard.is_icon_only());
    }

    #[test]
    fn test_card_template() {
        assert_eq!(CardTemplate::default(), CardTemplate::Standard);
        assert_eq!(CardTemplate::Media.as_str(), "media");
    }
}
