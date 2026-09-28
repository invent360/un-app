//! Input validation rules and utilities.
//!
//! This module provides a flexible validation system for form inputs.
//!
//! # Example
//!
//! ```ignore
//! use ember_fx_common::validation::{ValidationRule, validate_value};
//!
//! let rules = vec![
//!     ValidationRule::required("Email is required"),
//!     ValidationRule::email("Invalid email format"),
//! ];
//!
//! let result = validate_value("test@example.com", &rules);
//! assert!(result.is_valid());
//! ```

use std::fmt;

/// When to trigger validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidateOn {
    /// Validate on blur (when input loses focus)
    #[default]
    Blur,
    /// Validate on every change
    Change,
    /// Only validate on form submit
    Submit,
}

impl ValidateOn {
    /// Parse from string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "blur" => Some(Self::Blur),
            "change" | "input" => Some(Self::Change),
            "submit" => Some(Self::Submit),
            _ => None,
        }
    }
}

/// A validation rule for input fields.
#[derive(Clone)]
pub enum ValidationRule {
    /// Field is required (non-empty).
    Required(String),
    /// Minimum length.
    MinLength(usize, String),
    /// Maximum length.
    MaxLength(usize, String),
    /// Exact length.
    Length(usize, String),
    /// Email format validation.
    Email(String),
    /// URL format validation.
    Url(String),
    /// Numeric value validation.
    Numeric(String),
    /// Integer validation.
    Integer(String),
    /// Minimum numeric value.
    Min(f64, String),
    /// Maximum numeric value.
    Max(f64, String),
    /// Range of numeric values.
    Range(f64, f64, String),
    /// Regex pattern validation.
    Pattern(String, String),
    /// Custom validation function.
    Custom(CustomValidator),
}

impl fmt::Debug for ValidationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Required(msg) => write!(f, "Required({:?})", msg),
            Self::MinLength(len, msg) => write!(f, "MinLength({}, {:?})", len, msg),
            Self::MaxLength(len, msg) => write!(f, "MaxLength({}, {:?})", len, msg),
            Self::Length(len, msg) => write!(f, "Length({}, {:?})", len, msg),
            Self::Email(msg) => write!(f, "Email({:?})", msg),
            Self::Url(msg) => write!(f, "Url({:?})", msg),
            Self::Numeric(msg) => write!(f, "Numeric({:?})", msg),
            Self::Integer(msg) => write!(f, "Integer({:?})", msg),
            Self::Min(val, msg) => write!(f, "Min({}, {:?})", val, msg),
            Self::Max(val, msg) => write!(f, "Max({}, {:?})", val, msg),
            Self::Range(min, max, msg) => write!(f, "Range({}, {}, {:?})", min, max, msg),
            Self::Pattern(pat, msg) => write!(f, "Pattern({:?}, {:?})", pat, msg),
            Self::Custom(_) => write!(f, "Custom(...)"),
        }
    }
}

/// Custom validator function wrapper.
#[derive(Clone)]
pub struct CustomValidator {
    /// Validation function that takes a value and returns an optional error message.
    pub validate: fn(&str) -> Option<String>,
}

impl CustomValidator {
    /// Create a new custom validator.
    pub fn new(validate: fn(&str) -> Option<String>) -> Self {
        Self { validate }
    }
}

impl ValidationRule {
    /// Create a required rule.
    pub fn required(message: impl Into<String>) -> Self {
        Self::Required(message.into())
    }

    /// Create a minimum length rule.
    pub fn min_length(length: usize, message: impl Into<String>) -> Self {
        Self::MinLength(length, message.into())
    }

    /// Create a maximum length rule.
    pub fn max_length(length: usize, message: impl Into<String>) -> Self {
        Self::MaxLength(length, message.into())
    }

    /// Create an exact length rule.
    pub fn length(length: usize, message: impl Into<String>) -> Self {
        Self::Length(length, message.into())
    }

    /// Create an email validation rule.
    pub fn email(message: impl Into<String>) -> Self {
        Self::Email(message.into())
    }

    /// Create a URL validation rule.
    pub fn url(message: impl Into<String>) -> Self {
        Self::Url(message.into())
    }

    /// Create a numeric validation rule.
    pub fn numeric(message: impl Into<String>) -> Self {
        Self::Numeric(message.into())
    }

    /// Create an integer validation rule.
    pub fn integer(message: impl Into<String>) -> Self {
        Self::Integer(message.into())
    }

    /// Create a minimum value rule.
    pub fn min(value: f64, message: impl Into<String>) -> Self {
        Self::Min(value, message.into())
    }

    /// Create a maximum value rule.
    pub fn max(value: f64, message: impl Into<String>) -> Self {
        Self::Max(value, message.into())
    }

    /// Create a range rule.
    pub fn range(min: f64, max: f64, message: impl Into<String>) -> Self {
        Self::Range(min, max, message.into())
    }

    /// Create a pattern validation rule.
    pub fn pattern(regex: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Pattern(regex.into(), message.into())
    }

    /// Create a custom validation rule.
    pub fn custom(validate: fn(&str) -> Option<String>) -> Self {
        Self::Custom(CustomValidator::new(validate))
    }

    /// Validate a value against this rule.
    /// Returns `None` if valid, or `Some(error_message)` if invalid.
    pub fn validate(&self, value: &str) -> Option<String> {
        match self {
            Self::Required(msg) => {
                if value.trim().is_empty() {
                    Some(msg.clone())
                } else {
                    None
                }
            }
            Self::MinLength(min, msg) => {
                if value.len() < *min {
                    Some(msg.clone())
                } else {
                    None
                }
            }
            Self::MaxLength(max, msg) => {
                if value.len() > *max {
                    Some(msg.clone())
                } else {
                    None
                }
            }
            Self::Length(len, msg) => {
                if value.len() != *len {
                    Some(msg.clone())
                } else {
                    None
                }
            }
            Self::Email(msg) => {
                if is_valid_email(value) {
                    None
                } else {
                    Some(msg.clone())
                }
            }
            Self::Url(msg) => {
                if is_valid_url(value) {
                    None
                } else {
                    Some(msg.clone())
                }
            }
            Self::Numeric(msg) => {
                if value.parse::<f64>().is_ok() {
                    None
                } else {
                    Some(msg.clone())
                }
            }
            Self::Integer(msg) => {
                if value.parse::<i64>().is_ok() {
                    None
                } else {
                    Some(msg.clone())
                }
            }
            Self::Min(min, msg) => {
                if let Ok(num) = value.parse::<f64>() {
                    if num >= *min {
                        None
                    } else {
                        Some(msg.clone())
                    }
                } else {
                    Some(msg.clone())
                }
            }
            Self::Max(max, msg) => {
                if let Ok(num) = value.parse::<f64>() {
                    if num <= *max {
                        None
                    } else {
                        Some(msg.clone())
                    }
                } else {
                    Some(msg.clone())
                }
            }
            Self::Range(min, max, msg) => {
                if let Ok(num) = value.parse::<f64>() {
                    if num >= *min && num <= *max {
                        None
                    } else {
                        Some(msg.clone())
                    }
                } else {
                    Some(msg.clone())
                }
            }
            Self::Pattern(pattern, msg) => {
                if matches_pattern(value, pattern) {
                    None
                } else {
                    Some(msg.clone())
                }
            }
            Self::Custom(validator) => (validator.validate)(value),
        }
    }
}

/// Result of validating a value against multiple rules.
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    /// List of error messages (empty if valid).
    pub errors: Vec<String>,
}

impl ValidationResult {
    /// Create a valid result.
    pub fn valid() -> Self {
        Self { errors: Vec::new() }
    }

    /// Create an invalid result with errors.
    pub fn invalid(errors: Vec<String>) -> Self {
        Self { errors }
    }

    /// Check if the result is valid.
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the first error message, if any.
    pub fn first_error(&self) -> Option<&String> {
        self.errors.first()
    }

    /// Get all error messages joined by a separator.
    pub fn all_errors(&self, separator: &str) -> String {
        self.errors.join(separator)
    }
}

/// Validate a value against a list of rules.
///
/// Returns a `ValidationResult` containing any error messages.
/// Validation stops at the first error (fail-fast).
pub fn validate_value(value: &str, rules: &[ValidationRule]) -> ValidationResult {
    for rule in rules {
        if let Some(error) = rule.validate(value) {
            return ValidationResult::invalid(vec![error]);
        }
    }
    ValidationResult::valid()
}

/// Validate a value against a list of rules, collecting all errors.
///
/// Unlike `validate_value`, this function collects all validation errors
/// instead of stopping at the first one.
pub fn validate_all(value: &str, rules: &[ValidationRule]) -> ValidationResult {
    let errors: Vec<String> = rules
        .iter()
        .filter_map(|rule| rule.validate(value))
        .collect();

    if errors.is_empty() {
        ValidationResult::valid()
    } else {
        ValidationResult::invalid(errors)
    }
}

/// Simple email validation.
/// Checks for basic email format: something@something.something
fn is_valid_email(value: &str) -> bool {
    if value.is_empty() {
        return true; // Empty is valid (use Required rule for non-empty check)
    }

    let mut parts = value.split('@');
    let local = match parts.next() {
        Some(l) => l,
        None => return false,
    };
    let domain = match parts.next() {
        Some(d) => d,
        None => return false,
    };
    // Must have exactly 2 parts (no more @)
    if parts.next().is_some() {
        return false;
    }

    // Basic checks
    if local.is_empty() || domain.is_empty() {
        return false;
    }

    // Domain must have at least one dot
    if !domain.contains('.') {
        return false;
    }

    // Domain parts must not be empty
    let domain_parts: Vec<&str> = domain.split('.').collect();
    if domain_parts.iter().any(|p| p.is_empty()) {
        return false;
    }

    true
}

/// Simple URL validation.
fn is_valid_url(value: &str) -> bool {
    if value.is_empty() {
        return true; // Empty is valid
    }

    // Must start with http:// or https://
    if !value.starts_with("http://") && !value.starts_with("https://") {
        return false;
    }

    // Must have something after the protocol
    let after_protocol = if let Some(rest) = value.strip_prefix("https://") {
        rest
    } else if let Some(rest) = value.strip_prefix("http://") {
        rest
    } else {
        return false;
    };

    !after_protocol.is_empty() && after_protocol.contains('.')
}

/// Pattern matching (simple contains check, or regex if available).
fn matches_pattern(value: &str, pattern: &str) -> bool {
    if value.is_empty() {
        return true; // Empty is valid
    }

    // For now, use simple contains check
    // In a future version, this could use the regex crate
    value.contains(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_rule() {
        let rule = ValidationRule::required("Field is required");
        assert!(rule.validate("").is_some());
        assert!(rule.validate("   ").is_some());
        assert!(rule.validate("hello").is_none());
    }

    #[test]
    fn test_min_length_rule() {
        let rule = ValidationRule::min_length(5, "Too short");
        assert!(rule.validate("hi").is_some());
        assert!(rule.validate("hello").is_none());
        assert!(rule.validate("hello world").is_none());
    }

    #[test]
    fn test_max_length_rule() {
        let rule = ValidationRule::max_length(5, "Too long");
        assert!(rule.validate("hi").is_none());
        assert!(rule.validate("hello").is_none());
        assert!(rule.validate("hello world").is_some());
    }

    #[test]
    fn test_email_rule() {
        let rule = ValidationRule::email("Invalid email");
        assert!(rule.validate("").is_none()); // Empty is valid
        assert!(rule.validate("test@example.com").is_none());
        assert!(rule.validate("user.name@domain.co.uk").is_none());
        assert!(rule.validate("invalid").is_some());
        assert!(rule.validate("no@domain").is_some());
        assert!(rule.validate("@domain.com").is_some());
    }

    #[test]
    fn test_numeric_rule() {
        let rule = ValidationRule::numeric("Must be a number");
        assert!(rule.validate("123").is_none());
        assert!(rule.validate("12.34").is_none());
        assert!(rule.validate("-5").is_none());
        assert!(rule.validate("abc").is_some());
    }

    #[test]
    fn test_range_rule() {
        let rule = ValidationRule::range(1.0, 10.0, "Must be between 1 and 10");
        assert!(rule.validate("5").is_none());
        assert!(rule.validate("1").is_none());
        assert!(rule.validate("10").is_none());
        assert!(rule.validate("0").is_some());
        assert!(rule.validate("11").is_some());
    }

    #[test]
    fn test_validate_value() {
        let rules = vec![
            ValidationRule::required("Required"),
            ValidationRule::min_length(3, "Too short"),
        ];

        assert!(!validate_value("", &rules).is_valid());
        assert!(!validate_value("ab", &rules).is_valid());
        assert!(validate_value("abc", &rules).is_valid());
    }

    #[test]
    fn test_custom_validator() {
        fn no_spaces(value: &str) -> Option<String> {
            if value.contains(' ') {
                Some("No spaces allowed".to_string())
            } else {
                None
            }
        }

        let rule = ValidationRule::custom(no_spaces);
        assert!(rule.validate("hello").is_none());
        assert!(rule.validate("hello world").is_some());
    }
}
