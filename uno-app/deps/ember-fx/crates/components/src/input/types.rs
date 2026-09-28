//! Input type definitions.
//!
//! Shared types for all input components (TextInput, PasswordInput, TextArea, InputGroup).

use std::fmt;

/// Input visual variants.
///
/// Controls the visual style of input fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum InputVariant {
    /// Standard input with border (default)
    #[default]
    Outlined,
    /// Filled background, subtle border
    Filled,
    /// Borderless/transparent input
    Borderless,
}

impl InputVariant {
    /// Get the CSS suffix for this variant.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Outlined => "outlined",
            Self::Filled => "filled",
            Self::Borderless => "borderless",
        }
    }

    /// Get the full CSS class for this variant.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "outlined" | "outline" | "default" | "standard" => Some(Self::Outlined),
            "filled" | "fill" => Some(Self::Filled),
            "borderless" | "none" | "transparent" => Some(Self::Borderless),
            _ => None,
        }
    }

    /// Check if this variant has a visible border.
    pub fn has_border(&self) -> bool {
        matches!(self, Self::Outlined)
    }

    /// Check if this variant has a filled background.
    pub fn is_filled(&self) -> bool {
        matches!(self, Self::Filled)
    }
}

impl fmt::Display for InputVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Input size variants.
///
/// Controls the size of input fields (padding, font size, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum InputSize {
    /// Small input
    Sm,
    /// Medium input (default)
    #[default]
    Md,
    /// Large input
    Lg,
}

impl InputSize {
    /// Get the CSS suffix for this size.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }

    /// Get the full CSS class for this size.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sm" | "small" => Some(Self::Sm),
            "md" | "medium" | "default" => Some(Self::Md),
            "lg" | "large" => Some(Self::Lg),
            _ => None,
        }
    }

    /// Convert to the core ComponentSize.
    #[cfg(feature = "core")]
    pub fn to_component_size(&self) -> crate::core::ComponentSize {
        match self {
            Self::Sm => crate::core::ComponentSize::Sm,
            Self::Md => crate::core::ComponentSize::Md,
            Self::Lg => crate::core::ComponentSize::Lg,
        }
    }
}

impl fmt::Display for InputSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Validation state for inputs.
///
/// Controls visual feedback for validation status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ValidationState {
    /// No validation state (default)
    #[default]
    None,
    /// Valid/success state
    Success,
    /// Warning state
    Warning,
    /// Error/invalid state
    Error,
}

impl ValidationState {
    /// Get the CSS suffix for this state.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::None => "default",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Get the full CSS class for this state.
    pub fn class(&self, prefix: &str) -> String {
        match self {
            Self::None => String::new(),
            _ => format!("{}-{}", prefix, self.as_suffix()),
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "none" | "default" | "" => Some(Self::None),
            "success" | "valid" | "ok" => Some(Self::Success),
            "warning" | "warn" => Some(Self::Warning),
            "error" | "invalid" | "danger" => Some(Self::Error),
            _ => None,
        }
    }

    /// Check if this is an error state.
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    /// Check if this is a success state.
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    /// Check if this is a warning state.
    pub fn is_warning(&self) -> bool {
        matches!(self, Self::Warning)
    }

    /// Check if any validation state is set.
    pub fn has_state(&self) -> bool {
        !matches!(self, Self::None)
    }
}

impl fmt::Display for ValidationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Input type attribute for HTML input elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum InputType {
    /// Standard text input
    #[default]
    Text,
    /// Password input (masked)
    Password,
    /// Email input
    Email,
    /// Number input
    Number,
    /// Telephone input
    Tel,
    /// URL input
    Url,
    /// Search input
    Search,
}

impl InputType {
    /// Get the HTML input type attribute value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Email => "email",
            Self::Number => "number",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Search => "search",
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "text" => Some(Self::Text),
            "password" => Some(Self::Password),
            "email" => Some(Self::Email),
            "number" => Some(Self::Number),
            "tel" | "telephone" | "phone" => Some(Self::Tel),
            "url" => Some(Self::Url),
            "search" => Some(Self::Search),
            _ => None,
        }
    }
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Stepper button position for InputNumber.
///
/// Controls where the increment/decrement buttons appear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum StepperPosition {
    /// Both buttons on the right (default, stacked vertically)
    #[default]
    Right,
    /// Buttons on left and right sides of the input
    Sides,
    /// No visible buttons (keyboard/scroll only)
    None,
}

impl StepperPosition {
    /// Get the CSS suffix for this position.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Right => "right",
            Self::Sides => "sides",
            Self::None => "none",
        }
    }

    /// Get the full CSS class for this position.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-stepper-{}", prefix, self.as_suffix())
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "right" | "default" => Some(Self::Right),
            "sides" | "both" | "split" => Some(Self::Sides),
            "none" | "hidden" => Some(Self::None),
            _ => None,
        }
    }
}

impl fmt::Display for StepperPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// OTP input length.
///
/// Number of digit boxes for OTP input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum OtpLength {
    /// 4 digits
    Four,
    /// 6 digits (default)
    #[default]
    Six,
    /// 8 digits
    Eight,
}

impl OtpLength {
    /// Get the number of digits.
    pub fn count(&self) -> usize {
        match self {
            Self::Four => 4,
            Self::Six => 6,
            Self::Eight => 8,
        }
    }

    /// Parse from string or number.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "4" | "four" => Some(Self::Four),
            "6" | "six" => Some(Self::Six),
            "8" | "eight" => Some(Self::Eight),
            _ => None,
        }
    }

    /// Create from a number.
    pub fn from_count(n: usize) -> Option<Self> {
        match n {
            4 => Some(Self::Four),
            6 => Some(Self::Six),
            8 => Some(Self::Eight),
            _ => None,
        }
    }
}

impl fmt::Display for OtpLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.count())
    }
}

/// Input mask type for formatted inputs.
///
/// Predefined patterns for common input formats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaskType {
    /// Phone number: (XXX) XXX-XXXX
    Phone,
    /// Credit card: XXXX XXXX XXXX XXXX
    CreditCard,
    /// Date: XX/XX/XXXX
    Date,
    /// Time: XX:XX
    Time,
    /// Currency: $X,XXX.XX
    Currency,
    /// Custom pattern (# = digit, A = letter, * = any)
    Custom(String),
}

impl MaskType {
    /// Get the mask pattern string.
    pub fn pattern(&self) -> &str {
        match self {
            Self::Phone => "(###) ###-####",
            Self::CreditCard => "#### #### #### ####",
            Self::Date => "##/##/####",
            Self::Time => "##:##",
            Self::Currency => "$#,###.##",
            Self::Custom(p) => p,
        }
    }

    /// Get the CSS suffix for this mask type.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Phone => "phone",
            Self::CreditCard => "credit-card",
            Self::Date => "date",
            Self::Time => "time",
            Self::Currency => "currency",
            Self::Custom(_) => "custom",
        }
    }

    /// Get the full CSS class for this mask type.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-mask-{}", prefix, self.as_suffix())
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "phone" | "tel" | "telephone" => Some(Self::Phone),
            "credit-card" | "creditcard" | "card" => Some(Self::CreditCard),
            "date" => Some(Self::Date),
            "time" => Some(Self::Time),
            "currency" | "money" => Some(Self::Currency),
            _ => None,
        }
    }
}

impl fmt::Display for MaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// AutoComplete suggestion option.
#[derive(Debug, Clone, PartialEq)]
pub struct AutoCompleteOption {
    /// The actual value to be submitted.
    pub value: String,
    /// The display label.
    pub label: String,
    /// Whether this option is disabled.
    pub disabled: bool,
    /// Optional group name for grouping options.
    pub group: Option<String>,
}

impl AutoCompleteOption {
    /// Create a new option with value and label.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
            group: None,
        }
    }

    /// Create an option where value equals label.
    pub fn simple(value: impl Into<String>) -> Self {
        let v = value.into();
        Self {
            label: v.clone(),
            value: v,
            disabled: false,
            group: None,
        }
    }

    /// Add this option to a group.
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Mark this option as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_variant() {
        assert_eq!(InputVariant::default(), InputVariant::Outlined);
        assert_eq!(InputVariant::Filled.as_suffix(), "filled");
        assert_eq!(InputVariant::Borderless.class("fx-input"), "fx-input-borderless");
        assert!(InputVariant::Outlined.has_border());
        assert!(!InputVariant::Borderless.has_border());
    }

    #[test]
    fn test_input_variant_from_str() {
        assert_eq!(InputVariant::from_str("outlined"), Some(InputVariant::Outlined));
        assert_eq!(InputVariant::from_str("FILLED"), Some(InputVariant::Filled));
        assert_eq!(InputVariant::from_str("invalid"), None);
    }

    #[test]
    fn test_input_size() {
        assert_eq!(InputSize::default(), InputSize::Md);
        assert_eq!(InputSize::Lg.as_suffix(), "lg");
        assert_eq!(InputSize::Sm.class("fx-input"), "fx-input-sm");
    }

    #[test]
    fn test_validation_state() {
        assert_eq!(ValidationState::default(), ValidationState::None);
        assert!(ValidationState::Error.is_error());
        assert!(ValidationState::Success.is_success());
        assert!(ValidationState::Warning.is_warning());
        assert!(!ValidationState::None.has_state());
        assert!(ValidationState::Error.has_state());
    }

    #[test]
    fn test_validation_state_class() {
        assert_eq!(ValidationState::None.class("fx-input"), "");
        assert_eq!(ValidationState::Error.class("fx-input"), "fx-input-error");
    }

    #[test]
    fn test_input_type() {
        assert_eq!(InputType::default(), InputType::Text);
        assert_eq!(InputType::Password.as_str(), "password");
        assert_eq!(InputType::from_str("email"), Some(InputType::Email));
    }

    #[test]
    fn test_stepper_position() {
        assert_eq!(StepperPosition::default(), StepperPosition::Right);
        assert_eq!(StepperPosition::Sides.as_suffix(), "sides");
        assert_eq!(StepperPosition::Right.class("fx-input-number"), "fx-input-number-stepper-right");
    }

    #[test]
    fn test_otp_length() {
        assert_eq!(OtpLength::default(), OtpLength::Six);
        assert_eq!(OtpLength::Four.count(), 4);
        assert_eq!(OtpLength::Six.count(), 6);
        assert_eq!(OtpLength::Eight.count(), 8);
        assert_eq!(OtpLength::from_count(6), Some(OtpLength::Six));
    }

    #[test]
    fn test_mask_type() {
        assert_eq!(MaskType::Phone.pattern(), "(###) ###-####");
        assert_eq!(MaskType::CreditCard.as_suffix(), "credit-card");
        let custom = MaskType::Custom("##-##-##".to_string());
        assert_eq!(custom.pattern(), "##-##-##");
    }

    #[test]
    fn test_autocomplete_option() {
        let opt = AutoCompleteOption::new("us", "United States")
            .with_group("Countries");
        assert_eq!(opt.value, "us");
        assert_eq!(opt.label, "United States");
        assert_eq!(opt.group, Some("Countries".to_string()));
        assert!(!opt.disabled);
    }
}
