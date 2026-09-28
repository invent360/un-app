//! Button type definitions.
//!
//! This module provides comprehensive button types following Ant Design patterns
//! with support for both legacy `type` prop and modern `color` + `variant` system.

use std::fmt;

// ============================================================================
// BUTTON COLOR
// ============================================================================

/// Button color variants.
///
/// Controls the color scheme of the button. Works in combination with
/// `ButtonStyleVariant` to create different visual styles.
///
/// # Ant Design Mapping
/// - `Default` - Neutral color (gray borders, dark text)
/// - `Primary` - Brand primary color (blue by default)
/// - `Danger` - Error/destructive color (red)
/// - Preset colors for custom styling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ButtonColor {
    /// Default neutral color
    #[default]
    Default,
    /// Primary brand color
    Primary,
    /// Danger/error color for destructive actions
    Danger,
    // Preset colors
    /// Pink preset color
    Pink,
    /// Purple preset color
    Purple,
    /// Cyan preset color
    Cyan,
    /// Blue preset color
    Blue,
    /// Green preset color
    Green,
    /// Orange preset color
    Orange,
    /// Red preset color
    Red,
    /// Yellow preset color
    Yellow,
    /// Lime preset color
    Lime,
    /// Magenta preset color
    Magenta,
    /// Volcano preset color
    Volcano,
    /// Gold preset color
    Gold,
}

impl ButtonColor {
    /// Get the CSS suffix for this color.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Primary => "primary",
            Self::Danger => "danger",
            Self::Pink => "pink",
            Self::Purple => "purple",
            Self::Cyan => "cyan",
            Self::Blue => "blue",
            Self::Green => "green",
            Self::Orange => "orange",
            Self::Red => "red",
            Self::Yellow => "yellow",
            Self::Lime => "lime",
            Self::Magenta => "magenta",
            Self::Volcano => "volcano",
            Self::Gold => "gold",
        }
    }

    /// Get the full CSS class for this color.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-color-{}", prefix, self.as_suffix())
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" => Some(Self::Default),
            "primary" => Some(Self::Primary),
            "danger" | "error" => Some(Self::Danger),
            "pink" => Some(Self::Pink),
            "purple" => Some(Self::Purple),
            "cyan" => Some(Self::Cyan),
            "blue" => Some(Self::Blue),
            "green" | "success" => Some(Self::Green),
            "orange" => Some(Self::Orange),
            "red" => Some(Self::Red),
            "yellow" | "warning" => Some(Self::Yellow),
            "lime" => Some(Self::Lime),
            "magenta" => Some(Self::Magenta),
            "volcano" => Some(Self::Volcano),
            "gold" => Some(Self::Gold),
            _ => None,
        }
    }

    /// Check if this is a preset color (not default/primary/danger).
    pub fn is_preset(&self) -> bool {
        !matches!(self, Self::Default | Self::Primary | Self::Danger)
    }
}

impl fmt::Display for ButtonColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

// ============================================================================
// BUTTON STYLE VARIANT
// ============================================================================

/// Button style variants (modern Ant Design 5.x system).
///
/// Controls the visual style of the button. Works in combination with
/// `ButtonColor` to create different appearances.
///
/// # Variants
/// - `Solid` - Filled background (most prominent)
/// - `Outlined` - Border only, transparent background (default)
/// - `Dashed` - Dashed border, transparent background
/// - `Filled` - Light background fill
/// - `Text` - No border or background, text only
/// - `Link` - Link-styled button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ButtonStyleVariant {
    /// Solid filled background (most prominent)
    Solid,
    /// Bordered with transparent background (default)
    #[default]
    Outlined,
    /// Dashed border with transparent background
    Dashed,
    /// Light background fill
    Filled,
    /// No border or background, text only
    Text,
    /// Link-styled appearance
    Link,
}

impl ButtonStyleVariant {
    /// Get the CSS suffix for this variant.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Outlined => "outlined",
            Self::Dashed => "dashed",
            Self::Filled => "filled",
            Self::Text => "text",
            Self::Link => "link",
        }
    }

    /// Get the full CSS class for this variant.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-variant-{}", prefix, self.as_suffix())
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "solid" | "contained" | "filled-solid" => Some(Self::Solid),
            "outlined" | "outline" | "bordered" => Some(Self::Outlined),
            "dashed" => Some(Self::Dashed),
            "filled" | "tonal" => Some(Self::Filled),
            "text" | "ghost" => Some(Self::Text),
            "link" => Some(Self::Link),
            _ => None,
        }
    }

    /// Check if this variant should have a border.
    pub fn has_border(&self) -> bool {
        matches!(self, Self::Solid | Self::Outlined | Self::Dashed)
    }

    /// Check if this variant should have a filled background.
    pub fn has_background(&self) -> bool {
        matches!(self, Self::Solid | Self::Filled)
    }
}

impl fmt::Display for ButtonStyleVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

// ============================================================================
// LEGACY BUTTON VARIANT (for backwards compatibility)
// ============================================================================

/// Button visual variants (legacy API).
///
/// This is the simplified variant system for common use cases.
/// For full control, use `ButtonColor` + `ButtonStyleVariant` combination.
///
/// # Mapping to Modern System
/// - `Primary` → color: Primary + variant: Solid
/// - `Secondary` → color: Default + variant: Outlined
/// - `Outline` → color: Default + variant: Outlined
/// - `Ghost` → color: Default + variant: Text
/// - `Danger` → color: Danger + variant: Solid
/// - `Link` → color: Primary + variant: Link
/// - `Blue` → color: Blue + variant: Solid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ButtonVariant {
    /// Primary action button (filled, prominent)
    #[default]
    Primary,
    /// Secondary action button (less prominent)
    Secondary,
    /// Outline button (bordered, transparent background)
    Outline,
    /// Ghost button (minimal, no background or border)
    Ghost,
    /// Danger/destructive action button
    Danger,
    /// Link-styled button (looks like a link)
    Link,
    /// Blue action button (bright blue, high contrast)
    Blue,
    /// Dashed border button
    Dashed,
    /// Text-only button (no border or background)
    Text,
}

impl ButtonVariant {
    /// Get the CSS suffix for this variant.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
            Self::Danger => "danger",
            Self::Link => "link",
            Self::Blue => "blue",
            Self::Dashed => "dashed",
            Self::Text => "text",
        }
    }

    /// Get the full CSS class for this variant.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }

    /// Convert to modern color + style variant combination.
    pub fn to_modern(&self) -> (ButtonColor, ButtonStyleVariant) {
        match self {
            Self::Primary => (ButtonColor::Primary, ButtonStyleVariant::Solid),
            Self::Secondary => (ButtonColor::Default, ButtonStyleVariant::Outlined),
            Self::Outline => (ButtonColor::Default, ButtonStyleVariant::Outlined),
            Self::Ghost => (ButtonColor::Default, ButtonStyleVariant::Text),
            Self::Danger => (ButtonColor::Danger, ButtonStyleVariant::Solid),
            Self::Link => (ButtonColor::Primary, ButtonStyleVariant::Link),
            Self::Blue => (ButtonColor::Blue, ButtonStyleVariant::Solid),
            Self::Dashed => (ButtonColor::Default, ButtonStyleVariant::Dashed),
            Self::Text => (ButtonColor::Default, ButtonStyleVariant::Text),
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "primary" | "default" => Some(Self::Primary),
            "secondary" => Some(Self::Secondary),
            "outline" | "bordered" => Some(Self::Outline),
            "ghost" => Some(Self::Ghost),
            "danger" | "destructive" | "error" => Some(Self::Danger),
            "link" => Some(Self::Link),
            "blue" => Some(Self::Blue),
            "dashed" => Some(Self::Dashed),
            "text" => Some(Self::Text),
            _ => None,
        }
    }

    /// Check if this variant has a filled background.
    pub fn is_filled(&self) -> bool {
        matches!(self, Self::Primary | Self::Secondary | Self::Danger | Self::Blue)
    }

    /// Check if this variant should have reduced visual weight.
    pub fn is_subtle(&self) -> bool {
        matches!(self, Self::Ghost | Self::Link | Self::Text)
    }
}

impl fmt::Display for ButtonVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

// ============================================================================
// BUTTON SIZE
// ============================================================================

/// Button size variants.
///
/// Controls the size of the button (padding, font size, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ButtonSize {
    /// Extra small button
    Xs,
    /// Small button
    Sm,
    /// Medium button (default)
    #[default]
    Md,
    /// Large button
    Lg,
    /// Extra large button
    Xl,
}

impl ButtonSize {
    /// Get the CSS suffix for this size.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
        }
    }

    /// Get the full CSS class for this size.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "xs" | "extra-small" | "tiny" => Some(Self::Xs),
            "sm" | "small" => Some(Self::Sm),
            "md" | "medium" | "default" | "middle" => Some(Self::Md),
            "lg" | "large" => Some(Self::Lg),
            "xl" | "extra-large" => Some(Self::Xl),
            _ => None,
        }
    }

    /// Convert to the core ComponentSize.
    #[cfg(feature = "core")]
    pub fn to_component_size(&self) -> crate::core::ComponentSize {
        match self {
            Self::Xs => crate::core::ComponentSize::Xs,
            Self::Sm => crate::core::ComponentSize::Sm,
            Self::Md => crate::core::ComponentSize::Md,
            Self::Lg => crate::core::ComponentSize::Lg,
            Self::Xl => crate::core::ComponentSize::Xl,
        }
    }
}

impl fmt::Display for ButtonSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

// ============================================================================
// BUTTON SHAPE
// ============================================================================

/// Button shape variants.
///
/// Controls the shape of the button (rectangular, circular, pill).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ButtonShape {
    /// Default rectangular with border-radius
    #[default]
    Default,
    /// Circular button (for icons)
    Circle,
    /// Pill/rounded button (fully rounded ends)
    Round,
}

impl ButtonShape {
    /// Get the CSS suffix for this shape.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Circle => "circle",
            Self::Round => "round",
        }
    }

    /// Get the full CSS class for this shape.
    /// Returns empty string for Default (no extra class needed).
    pub fn class(&self, prefix: &str) -> String {
        if *self == Self::Default {
            String::new()
        } else {
            format!("{}-{}", prefix, self.as_suffix())
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" | "rectangle" | "rect" => Some(Self::Default),
            "circle" | "circular" => Some(Self::Circle),
            "round" | "pill" | "rounded" => Some(Self::Round),
            _ => None,
        }
    }
}

impl fmt::Display for ButtonShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

// ============================================================================
// ICON POSITION
// ============================================================================

/// Icon position within a button.
///
/// Controls where the icon is placed relative to the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum IconPosition {
    /// Icon at the start (left in LTR, right in RTL)
    #[default]
    Start,
    /// Icon at the end (right in LTR, left in RTL)
    End,
}

impl IconPosition {
    /// Get the CSS suffix for this position.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }

    /// Get the full CSS class for this position.
    /// Returns empty string for Start (default, no extra class).
    pub fn class(&self, prefix: &str) -> String {
        if *self == Self::Start {
            String::new()
        } else {
            format!("{}-icon-{}", prefix, self.as_suffix())
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "start" | "left" | "before" => Some(Self::Start),
            "end" | "right" | "after" => Some(Self::End),
            _ => None,
        }
    }
}

impl fmt::Display for IconPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

// ============================================================================
// HTML BUTTON TYPE
// ============================================================================

/// HTML button type attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum HtmlButtonType {
    /// Standard button (default)
    #[default]
    Button,
    /// Form submit button
    Submit,
    /// Form reset button
    Reset,
}

impl HtmlButtonType {
    /// Get the HTML attribute value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "button" => Some(Self::Button),
            "submit" => Some(Self::Submit),
            "reset" => Some(Self::Reset),
            _ => None,
        }
    }
}

impl fmt::Display for HtmlButtonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// FLOAT BUTTON TYPES
// ============================================================================

/// Float button placement positions.
///
/// Controls where the floating button is positioned on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum FloatButtonPlacement {
    /// Bottom right corner (default)
    #[default]
    BottomRight,
    /// Bottom left corner
    BottomLeft,
    /// Top right corner
    TopRight,
    /// Top left corner
    TopLeft,
}

impl FloatButtonPlacement {
    /// Get the CSS suffix for this placement.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::BottomRight => "bottom-right",
            Self::BottomLeft => "bottom-left",
            Self::TopRight => "top-right",
            Self::TopLeft => "top-left",
        }
    }

    /// Get the full CSS class for this placement.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().replace('_', "-").as_str() {
            "bottom-right" | "br" => Some(Self::BottomRight),
            "bottom-left" | "bl" => Some(Self::BottomLeft),
            "top-right" | "tr" => Some(Self::TopRight),
            "top-left" | "tl" => Some(Self::TopLeft),
            _ => None,
        }
    }
}

impl fmt::Display for FloatButtonPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

// ============================================================================
// SPEED DIAL TYPES
// ============================================================================

/// Speed dial expand direction.
///
/// Controls which direction the speed dial actions expand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum SpeedDialDirection {
    /// Expand upward (default)
    #[default]
    Up,
    /// Expand downward
    Down,
    /// Expand leftward
    Left,
    /// Expand rightward
    Right,
}

impl SpeedDialDirection {
    /// Get the CSS suffix for this direction.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    /// Get the full CSS class for this direction.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-direction-{}", prefix, self.as_suffix())
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "up" | "top" => Some(Self::Up),
            "down" | "bottom" => Some(Self::Down),
            "left" => Some(Self::Left),
            "right" => Some(Self::Right),
            _ => None,
        }
    }
}

impl fmt::Display for SpeedDialDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Speed dial action item.
///
/// Represents a single action in a speed dial menu.
#[derive(Debug, Clone, PartialEq)]
pub struct SpeedDialAction {
    /// Unique key for this action.
    pub key: String,
    /// Icon content (HTML/SVG string or emoji).
    pub icon: String,
    /// Optional label text.
    pub label: Option<String>,
    /// Optional tooltip text.
    pub tooltip: Option<String>,
    /// Whether this action is disabled.
    pub disabled: bool,
}

impl SpeedDialAction {
    /// Create a new speed dial action.
    pub fn new(key: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            icon: icon.into(),
            label: None,
            tooltip: None,
            disabled: false,
        }
    }

    /// Add a tooltip to this action.
    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Add a label to this action.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Mark this action as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

// ============================================================================
// LOADING CONFIG
// ============================================================================

/// Loading configuration for buttons.
///
/// Allows configuring loading state with delay and custom icon.
#[derive(Debug, Clone, Default)]
pub struct LoadingConfig {
    /// Whether loading is active.
    pub loading: bool,
    /// Delay in milliseconds before showing loading indicator.
    pub delay: Option<u32>,
}

impl LoadingConfig {
    /// Create a new loading config.
    pub fn new(loading: bool) -> Self {
        Self { loading, delay: None }
    }

    /// Set the delay before showing loading indicator.
    pub fn with_delay(mut self, delay_ms: u32) -> Self {
        self.delay = Some(delay_ms);
        self
    }
}

impl From<bool> for LoadingConfig {
    fn from(loading: bool) -> Self {
        Self::new(loading)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_color() {
        assert_eq!(ButtonColor::default(), ButtonColor::Default);
        assert_eq!(ButtonColor::Primary.as_suffix(), "primary");
        assert_eq!(ButtonColor::Danger.class("fx-btn"), "fx-btn-color-danger");
        assert!(!ButtonColor::Primary.is_preset());
        assert!(ButtonColor::Pink.is_preset());
    }

    #[test]
    fn test_button_color_from_str() {
        assert_eq!(ButtonColor::from_str("primary"), Some(ButtonColor::Primary));
        assert_eq!(ButtonColor::from_str("error"), Some(ButtonColor::Danger));
        assert_eq!(ButtonColor::from_str("success"), Some(ButtonColor::Green));
        assert_eq!(ButtonColor::from_str("invalid"), None);
    }

    #[test]
    fn test_button_style_variant() {
        assert_eq!(ButtonStyleVariant::default(), ButtonStyleVariant::Outlined);
        assert_eq!(ButtonStyleVariant::Solid.as_suffix(), "solid");
        assert_eq!(ButtonStyleVariant::Dashed.class("fx-btn"), "fx-btn-variant-dashed");
        assert!(ButtonStyleVariant::Solid.has_border());
        assert!(ButtonStyleVariant::Solid.has_background());
        assert!(!ButtonStyleVariant::Text.has_border());
    }

    #[test]
    fn test_button_variant_to_modern() {
        let (color, variant) = ButtonVariant::Primary.to_modern();
        assert_eq!(color, ButtonColor::Primary);
        assert_eq!(variant, ButtonStyleVariant::Solid);

        let (color, variant) = ButtonVariant::Dashed.to_modern();
        assert_eq!(color, ButtonColor::Default);
        assert_eq!(variant, ButtonStyleVariant::Dashed);
    }

    #[test]
    fn test_button_variant() {
        assert_eq!(ButtonVariant::default(), ButtonVariant::Primary);
        assert_eq!(ButtonVariant::Primary.as_suffix(), "primary");
        assert_eq!(ButtonVariant::Outline.class("fx-btn"), "fx-btn-outline");
        assert!(ButtonVariant::Primary.is_filled());
        assert!(!ButtonVariant::Ghost.is_filled());
        assert!(ButtonVariant::Ghost.is_subtle());
    }

    #[test]
    fn test_button_variant_from_str() {
        assert_eq!(ButtonVariant::from_str("primary"), Some(ButtonVariant::Primary));
        assert_eq!(ButtonVariant::from_str("OUTLINE"), Some(ButtonVariant::Outline));
        assert_eq!(ButtonVariant::from_str("destructive"), Some(ButtonVariant::Danger));
        assert_eq!(ButtonVariant::from_str("invalid"), None);
    }

    #[test]
    fn test_button_size() {
        assert_eq!(ButtonSize::default(), ButtonSize::Md);
        assert_eq!(ButtonSize::Lg.as_suffix(), "lg");
        assert_eq!(ButtonSize::Sm.class("fx-btn"), "fx-btn-sm");
    }

    #[test]
    fn test_button_size_from_str() {
        assert_eq!(ButtonSize::from_str("sm"), Some(ButtonSize::Sm));
        assert_eq!(ButtonSize::from_str("extra-large"), Some(ButtonSize::Xl));
        assert_eq!(ButtonSize::from_str("middle"), Some(ButtonSize::Md));
        assert_eq!(ButtonSize::from_str("invalid"), None);
    }

    #[test]
    fn test_button_shape() {
        assert_eq!(ButtonShape::default(), ButtonShape::Default);
        assert_eq!(ButtonShape::Circle.as_suffix(), "circle");
        assert_eq!(ButtonShape::Default.class("fx-btn"), "");
        assert_eq!(ButtonShape::Circle.class("fx-btn"), "fx-btn-circle");
        assert_eq!(ButtonShape::Round.class("fx-btn"), "fx-btn-round");
    }

    #[test]
    fn test_icon_position() {
        assert_eq!(IconPosition::default(), IconPosition::Start);
        assert_eq!(IconPosition::End.as_suffix(), "end");
        assert_eq!(IconPosition::Start.class("fx-btn"), "");
        assert_eq!(IconPosition::End.class("fx-btn"), "fx-btn-icon-end");
    }

    #[test]
    fn test_html_button_type() {
        assert_eq!(HtmlButtonType::default(), HtmlButtonType::Button);
        assert_eq!(HtmlButtonType::Submit.as_str(), "submit");
        assert_eq!(HtmlButtonType::from_str("reset"), Some(HtmlButtonType::Reset));
    }

    #[test]
    fn test_float_button_placement() {
        assert_eq!(FloatButtonPlacement::default(), FloatButtonPlacement::BottomRight);
        assert_eq!(FloatButtonPlacement::TopLeft.as_suffix(), "top-left");
        assert_eq!(FloatButtonPlacement::BottomRight.class("fx-float-btn"), "fx-float-btn-bottom-right");
    }

    #[test]
    fn test_speed_dial_direction() {
        assert_eq!(SpeedDialDirection::default(), SpeedDialDirection::Up);
        assert_eq!(SpeedDialDirection::Left.as_suffix(), "left");
        assert_eq!(SpeedDialDirection::Up.class("fx-speed-dial"), "fx-speed-dial-direction-up");
    }

    #[test]
    fn test_speed_dial_action() {
        let action = SpeedDialAction::new("edit", "✏️")
            .with_tooltip("Edit item")
            .with_label("Edit");
        assert_eq!(action.key, "edit");
        assert_eq!(action.icon, "✏️");
        assert_eq!(action.tooltip, Some("Edit item".to_string()));
        assert_eq!(action.label, Some("Edit".to_string()));
        assert!(!action.disabled);
    }

    #[test]
    fn test_loading_config() {
        let config: LoadingConfig = true.into();
        assert!(config.loading);
        assert!(config.delay.is_none());

        let config = LoadingConfig::new(true).with_delay(1000);
        assert!(config.loading);
        assert_eq!(config.delay, Some(1000));
    }
}
