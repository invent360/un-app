//! Privacy utilities for uno-api
//!
//! Provides credential redaction for error messages and logs.
//! GOV-01: Treat lease codes as credentials (redact from logs)

/// Redact a credential value, showing only first and last 2 characters
///
/// Example: "ABC-123-XYZ" becomes "AB***YZ"
pub fn redact_credential(value: &str) -> String {
    if value.len() <= 4 {
        return "*".repeat(value.len());
    }
    let first = &value[..2];
    let last = &value[value.len() - 2..];
    format!("{}***{}", first, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_short_values() {
        assert_eq!(redact_credential("AB"), "**");
        assert_eq!(redact_credential("ABC"), "***");
        assert_eq!(redact_credential("ABCD"), "****");
    }

    #[test]
    fn test_redact_normal_values() {
        assert_eq!(redact_credential("ABC-123-XYZ"), "AB***YZ");
        assert_eq!(redact_credential("secret_key_12345"), "se***45");
        assert_eq!(redact_credential("lease-code-value"), "le***ue");
    }

    #[test]
    fn test_redact_empty_value() {
        assert_eq!(redact_credential(""), "");
    }
}
