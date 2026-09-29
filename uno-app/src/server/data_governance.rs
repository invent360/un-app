//! Data Governance Module
//!
//! Implements privacy controls and data governance policies for the application:
//!
//! - **Credential Redaction (GOV-01)**: Redact sensitive data from logs and error messages
//! - **Data Retention (GOV-02)**: Define and enforce data retention policies
//! - **IP Privacy (GOV-03)**: Anonymize or hash IP addresses for privacy
//! - **External Identity (GOV-04, GOV-05)**: Reference external KYC providers, don't store
//!
//! # Usage
//!
//! ```rust,ignore
//! use crate::server::data_governance::{redact_credential, anonymize_ip, RetentionPolicy};
//!
//! // Redact lease codes in error messages
//! let safe_error = redact_credential("Invalid lease code: ABC-123-XYZ");
//!
//! // Anonymize IP addresses
//! let anonymized = anonymize_ip("192.168.1.100");
//! ```

use std::fmt;
use sha2::{Sha256, Digest};

// =============================================================================
// GOV-01: Credential Redaction
// =============================================================================

/// Patterns that indicate sensitive credentials to redact
const CREDENTIAL_PATTERNS: &[&str] = &[
    "lease_code",
    "lease code",
    "license_key",
    "license key",
    "api_key",
    "api key",
    "token",
    "password",
    "secret",
    "credential",
    "device_id",
];

/// Redact a credential value, showing only first and last 2 characters
///
/// Example: "ABC-123-XYZ" becomes "AB***YZ"
pub fn redact_value(value: &str) -> String {
    if value.len() <= 4 {
        return "*".repeat(value.len());
    }
    let first = &value[..2];
    let last = &value[value.len() - 2..];
    format!("{}***{}", first, last)
}

/// Redact credentials from an error message or log line
///
/// Looks for patterns like "lease_code: ABC123" or "token=xyz789" and redacts the value.
pub fn redact_credential(message: &str) -> String {
    let mut result = message.to_string();

    for pattern in CREDENTIAL_PATTERNS {
        // Handle "pattern: value" format
        if let Some(idx) = result.to_lowercase().find(&format!("{}: ", pattern)) {
            let start = idx + pattern.len() + 2;
            let remaining = &result[start..];
            // Find end of value (whitespace, delimiter, or end of string)
            let end = remaining
                .find(|c: char| c.is_whitespace() || c == ',' || c == ')' || c == '"')
                .unwrap_or(remaining.len());
            if end > 0 {
                let value = &result[start..start + end];
                let redacted = redact_value(value);
                result = format!("{}{}{}", &result[..start], redacted, &result[start + end..]);
            }
        }

        // Handle "pattern=value" format
        if let Some(idx) = result.to_lowercase().find(&format!("{}=", pattern)) {
            let start = idx + pattern.len() + 1;
            let remaining = &result[start..];
            // Find end of value (whitespace, delimiter, or end of string)
            let end = remaining
                .find(|c: char| c.is_whitespace() || c == '&' || c == ')' || c == '"')
                .unwrap_or(remaining.len());
            if end > 0 {
                let value = &result[start..start + end];
                let redacted = redact_value(value);
                result = format!("{}{}{}", &result[..start], redacted, &result[start + end..]);
            }
        }
    }

    result
}

/// A credential wrapper that automatically redacts in Display/Debug
#[derive(Clone)]
pub struct RedactedCredential {
    value: String,
    credential_type: String,
}

impl RedactedCredential {
    pub fn new(credential_type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            credential_type: credential_type.into(),
        }
    }

    /// Get the actual value (use sparingly)
    pub fn expose(&self) -> &str {
        &self.value
    }

    /// Get a redacted version for display
    pub fn redacted(&self) -> String {
        redact_value(&self.value)
    }
}

impl fmt::Debug for RedactedCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:[REDACTED]", self.credential_type)
    }
}

impl fmt::Display for RedactedCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.credential_type, self.redacted())
    }
}

// =============================================================================
// GOV-03: IP Address Privacy
// =============================================================================

/// IP anonymization strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpAnonymizationStrategy {
    /// Store full IP (legacy mode, not recommended)
    None,
    /// Zero the last octet (IPv4) or last 80 bits (IPv6)
    Truncate,
    /// One-way hash the IP with salt
    Hash,
    /// Don't store IP at all
    Drop,
}

impl Default for IpAnonymizationStrategy {
    fn default() -> Self {
        Self::Truncate
    }
}

/// Anonymize an IP address according to the specified strategy
///
/// - `Truncate`: 192.168.1.100 -> 192.168.1.0, 2001:db8::1 -> 2001:db8::
/// - `Hash`: Returns SHA-256 hash prefix (first 16 chars)
/// - `Drop`: Returns empty string
/// - `None`: Returns unchanged
pub fn anonymize_ip(ip: &str, strategy: IpAnonymizationStrategy) -> String {
    match strategy {
        IpAnonymizationStrategy::None => ip.to_string(),
        IpAnonymizationStrategy::Drop => String::new(),
        IpAnonymizationStrategy::Hash => hash_ip(ip),
        IpAnonymizationStrategy::Truncate => truncate_ip(ip),
    }
}

/// Truncate IP address (zero last octet for IPv4, last 80 bits for IPv6)
fn truncate_ip(ip: &str) -> String {
    if ip.contains(':') {
        // IPv6: truncate to /48 equivalent
        let parts: Vec<&str> = ip.split(':').collect();
        if parts.len() >= 3 {
            format!("{}:{}:{}::", parts[0], parts[1], parts[2])
        } else {
            ip.to_string()
        }
    } else if ip.contains('.') {
        // IPv4: zero last octet
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() == 4 {
            format!("{}.{}.{}.0", parts[0], parts[1], parts[2])
        } else {
            ip.to_string()
        }
    } else {
        ip.to_string()
    }
}

/// Hash IP address with SHA-256 (returns first 16 hex chars)
fn hash_ip(ip: &str) -> String {
    // Add a static salt to prevent rainbow table attacks
    // In production, this should come from configuration
    const IP_HASH_SALT: &str = "uno-app-ip-privacy-salt-v1";

    let mut hasher = Sha256::new();
    hasher.update(IP_HASH_SALT.as_bytes());
    hasher.update(ip.as_bytes());
    let result = hasher.finalize();

    // Return first 16 hex characters (64 bits)
    hex::encode(&result[..8])
}

// =============================================================================
// GOV-02: Data Retention Policies
// =============================================================================

/// Data retention policy for a table or data category
#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    /// Name of the data category
    pub name: String,
    /// Retention period in days (None = indefinite)
    pub retention_days: Option<u32>,
    /// Whether to archive before deletion
    pub archive_before_delete: bool,
    /// SQL condition for deletion (e.g., "created_at < NOW() - INTERVAL '90 days'")
    pub deletion_condition: String,
}

impl RetentionPolicy {
    pub fn new(name: impl Into<String>, retention_days: Option<u32>) -> Self {
        let name = name.into();
        let deletion_condition = match retention_days {
            Some(days) => format!("created_at < NOW() - INTERVAL '{} days'", days),
            None => String::new(),
        };

        Self {
            name,
            retention_days,
            archive_before_delete: false,
            deletion_condition,
        }
    }

    pub fn with_archive(mut self) -> Self {
        self.archive_before_delete = true;
        self
    }

    pub fn with_custom_condition(mut self, condition: impl Into<String>) -> Self {
        self.deletion_condition = condition.into();
        self
    }
}

/// Standard retention policies for the application
pub fn standard_retention_policies() -> Vec<RetentionPolicy> {
    vec![
        // Visitor analytics: 90 days
        RetentionPolicy::new("visitors", Some(90))
            .with_custom_condition("last_seen_at < NOW() - INTERVAL '90 days'"),

        // Health checks: keep last 1000 or 7 days
        RetentionPolicy::new("health_checks", Some(7))
            .with_custom_condition("checked_at < NOW() - INTERVAL '7 days' AND id NOT IN (SELECT id FROM health_checks ORDER BY checked_at DESC LIMIT 1000)"),

        // Audit logs: 2 years (regulatory requirement)
        RetentionPolicy::new("audit_logs", Some(730))
            .with_archive(),

        // Job queue completed jobs: 30 days
        RetentionPolicy::new("job_queue", Some(30))
            .with_custom_condition("status IN ('completed', 'failed', 'dead_letter') AND completed_at < NOW() - INTERVAL '30 days'"),

        // License reservations (expired): 7 days
        RetentionPolicy::new("license_reservations", Some(7))
            .with_custom_condition("status = 'expired' AND expires_at < NOW() - INTERVAL '7 days'"),

        // Allocation ledger: indefinite (immutable audit trail)
        RetentionPolicy::new("allocation_ledger", None),
    ]
}

// =============================================================================
// GOV-04/05: External Identity Reference
// =============================================================================

/// Reference to an external KYC/identity provider
///
/// We never store actual identity documents or PII beyond what's necessary.
/// Instead, we store references to external providers who handle KYC.
#[derive(Debug, Clone)]
pub struct ExternalIdentityRef {
    /// The KYC provider (e.g., "jumio", "onfido", "manual_verification")
    pub provider: String,
    /// The external reference ID from the provider
    pub external_id: String,
    /// Verification status
    pub status: VerificationStatus,
    /// When verification was completed
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Verification status from external provider
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStatus {
    /// Verification not started
    Pending,
    /// Verification in progress
    InProgress,
    /// Successfully verified
    Verified,
    /// Verification failed
    Failed,
    /// Verification expired (needs re-verification)
    Expired,
}

impl ExternalIdentityRef {
    pub fn new(provider: impl Into<String>, external_id: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            external_id: external_id.into(),
            status: VerificationStatus::Pending,
            verified_at: None,
        }
    }

    pub fn mark_verified(mut self) -> Self {
        self.status = VerificationStatus::Verified;
        self.verified_at = Some(chrono::Utc::now());
        self
    }
}

// =============================================================================
// Audit Log Sanitization
// =============================================================================

/// Sanitize a value for audit logging
///
/// Removes or redacts sensitive fields from JSONB values before storage.
pub fn sanitize_for_audit<T: serde::Serialize>(value: &T) -> serde_json::Value {
    let json = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
    sanitize_json_value(json)
}

fn sanitize_json_value(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(mut map) => {
            // Redact sensitive fields
            for key in CREDENTIAL_PATTERNS {
                if map.contains_key(*key) {
                    if let Some(serde_json::Value::String(s)) = map.get(*key) {
                        map.insert(key.to_string(), serde_json::Value::String(redact_value(s)));
                    }
                }
            }

            // Also redact common sensitive field names
            for sensitive_key in ["lease_code", "license_key", "api_key", "password", "secret", "token", "device_id"] {
                if let Some(serde_json::Value::String(s)) = map.get(sensitive_key) {
                    map.insert(sensitive_key.to_string(), serde_json::Value::String(redact_value(s)));
                }
            }

            // Recursively sanitize nested objects
            let sanitized: serde_json::Map<String, serde_json::Value> = map
                .into_iter()
                .map(|(k, v)| (k, sanitize_json_value(v)))
                .collect();

            serde_json::Value::Object(sanitized)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(sanitize_json_value).collect())
        }
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_value_short() {
        assert_eq!(redact_value("AB"), "**");
        assert_eq!(redact_value("ABCD"), "****");
    }

    #[test]
    fn test_redact_value_normal() {
        assert_eq!(redact_value("ABC-123-XYZ"), "AB***YZ");
        assert_eq!(redact_value("secret_key_12345"), "se***45");
    }

    #[test]
    fn test_redact_credential_in_message() {
        let msg = "Invalid lease_code: ABC-123-XYZ";
        let redacted = redact_credential(msg);
        assert!(redacted.contains("AB***YZ"));
        assert!(!redacted.contains("ABC-123-XYZ"));
    }

    #[test]
    fn test_truncate_ipv4() {
        assert_eq!(truncate_ip("192.168.1.100"), "192.168.1.0");
        assert_eq!(truncate_ip("10.0.0.1"), "10.0.0.0");
    }

    #[test]
    fn test_truncate_ipv6() {
        assert_eq!(truncate_ip("2001:db8:85a3:8d3:1319:8a2e:370:7348"), "2001:db8:85a3::");
    }

    #[test]
    fn test_hash_ip() {
        let hash1 = hash_ip("192.168.1.1");
        let hash2 = hash_ip("192.168.1.2");

        // Different IPs should produce different hashes
        assert_ne!(hash1, hash2);

        // Same IP should produce same hash
        assert_eq!(hash_ip("192.168.1.1"), hash1);

        // Hash should be 16 hex characters
        assert_eq!(hash1.len(), 16);
    }

    #[test]
    fn test_anonymize_ip_strategies() {
        let ip = "192.168.1.100";

        assert_eq!(anonymize_ip(ip, IpAnonymizationStrategy::None), ip);
        assert_eq!(anonymize_ip(ip, IpAnonymizationStrategy::Drop), "");
        assert_eq!(anonymize_ip(ip, IpAnonymizationStrategy::Truncate), "192.168.1.0");
        assert_eq!(anonymize_ip(ip, IpAnonymizationStrategy::Hash).len(), 16);
    }

    #[test]
    fn test_standard_retention_policies() {
        let policies = standard_retention_policies();

        // Check we have policies for key tables
        assert!(policies.iter().any(|p| p.name == "visitors"));
        assert!(policies.iter().any(|p| p.name == "audit_logs"));
        assert!(policies.iter().any(|p| p.name == "health_checks"));

        // Allocation ledger should be indefinite
        let allocation = policies.iter().find(|p| p.name == "allocation_ledger").unwrap();
        assert!(allocation.retention_days.is_none());

        // Audit logs should be archived
        let audit = policies.iter().find(|p| p.name == "audit_logs").unwrap();
        assert!(audit.archive_before_delete);
    }

    #[test]
    fn test_sanitize_for_audit() {
        #[derive(serde::Serialize)]
        struct TestData {
            user_id: i32,
            lease_code: String,
            action: String,
        }

        let data = TestData {
            user_id: 123,
            lease_code: "ABC-123-XYZ".to_string(),
            action: "claim".to_string(),
        };

        let sanitized = sanitize_for_audit(&data);
        let obj = sanitized.as_object().unwrap();

        // lease_code should be redacted
        assert!(obj.get("lease_code").unwrap().as_str().unwrap().contains("***"));

        // Other fields should be unchanged
        assert_eq!(obj.get("user_id").unwrap().as_i64().unwrap(), 123);
        assert_eq!(obj.get("action").unwrap().as_str().unwrap(), "claim");
    }
}
