//! Status enums for type-safe status handling
//!
//! This module replaces magic strings with strongly-typed enums for:
//! - License variant status (active, inactive, deprecated)
//! - License claim status (pending, claimed, expired, cancelled)
//!
//! # Example
//!
//! ```ignore
//! use crate::types::{VariantStatus, ClaimStatus};
//!
//! // Type-safe comparisons
//! if variant.status == VariantStatus::Active {
//!     // Process active variant
//! }
//!
//! // Serializes to lowercase strings for API/database compatibility
//! let json = serde_json::to_string(&VariantStatus::Active)?; // "active"
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Status of a license variant
///
/// Variants can be:
/// - `Active`: Available for claiming
/// - `Inactive`: Temporarily disabled
/// - `Deprecated`: No longer available, kept for historical records
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "varchar", rename_all = "lowercase"))]
pub enum VariantStatus {
    /// Variant is active and available for claiming
    #[default]
    Active,
    /// Variant is temporarily inactive
    Inactive,
    /// Variant is deprecated and no longer available
    Deprecated,
}

impl VariantStatus {
    /// All possible variant status values
    pub const ALL: &'static [VariantStatus] = &[
        VariantStatus::Active,
        VariantStatus::Inactive,
        VariantStatus::Deprecated,
    ];

    /// Check if the variant is claimable
    pub fn is_claimable(&self) -> bool {
        matches!(self, VariantStatus::Active)
    }

    /// Get the status as a static string slice
    pub fn as_str(&self) -> &'static str {
        match self {
            VariantStatus::Active => "active",
            VariantStatus::Inactive => "inactive",
            VariantStatus::Deprecated => "deprecated",
        }
    }
}

impl fmt::Display for VariantStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for VariantStatus {
    type Err = StatusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(VariantStatus::Active),
            "inactive" => Ok(VariantStatus::Inactive),
            "deprecated" => Ok(VariantStatus::Deprecated),
            _ => Err(StatusParseError::InvalidVariantStatus(s.to_string())),
        }
    }
}

impl Serialize for VariantStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for VariantStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        VariantStatus::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Status of a license claim
///
/// Claims progress through:
/// - `Pending`: Claim initiated but not completed
/// - `Claimed`: Successfully claimed
/// - `Expired`: Claim expired without completion
/// - `Cancelled`: Claim was cancelled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "varchar", rename_all = "lowercase"))]
pub enum ClaimStatus {
    /// Claim is pending completion
    #[default]
    Pending,
    /// License has been successfully claimed
    Claimed,
    /// Claim expired without completion
    Expired,
    /// Claim was cancelled
    Cancelled,
}

impl ClaimStatus {
    /// All possible claim status values
    pub const ALL: &'static [ClaimStatus] = &[
        ClaimStatus::Pending,
        ClaimStatus::Claimed,
        ClaimStatus::Expired,
        ClaimStatus::Cancelled,
    ];

    /// Check if the claim is finalized (cannot change)
    pub fn is_final(&self) -> bool {
        matches!(self, ClaimStatus::Claimed | ClaimStatus::Expired | ClaimStatus::Cancelled)
    }

    /// Check if the claim was successful
    pub fn is_successful(&self) -> bool {
        matches!(self, ClaimStatus::Claimed)
    }

    /// Get the status as a static string slice
    pub fn as_str(&self) -> &'static str {
        match self {
            ClaimStatus::Pending => "pending",
            ClaimStatus::Claimed => "claimed",
            ClaimStatus::Expired => "expired",
            ClaimStatus::Cancelled => "cancelled",
        }
    }
}

impl fmt::Display for ClaimStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ClaimStatus {
    type Err = StatusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(ClaimStatus::Pending),
            "claimed" => Ok(ClaimStatus::Claimed),
            "expired" => Ok(ClaimStatus::Expired),
            "cancelled" | "canceled" => Ok(ClaimStatus::Cancelled),
            _ => Err(StatusParseError::InvalidClaimStatus(s.to_string())),
        }
    }
}

impl Serialize for ClaimStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ClaimStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ClaimStatus::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Error type for status parsing failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusParseError {
    /// Invalid variant status string
    InvalidVariantStatus(String),
    /// Invalid claim status string
    InvalidClaimStatus(String),
}

impl fmt::Display for StatusParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusParseError::InvalidVariantStatus(s) => {
                write!(f, "Invalid variant status: '{}'. Expected one of: active, inactive, deprecated", s)
            }
            StatusParseError::InvalidClaimStatus(s) => {
                write!(f, "Invalid claim status: '{}'. Expected one of: pending, claimed, expired, cancelled", s)
            }
        }
    }
}

impl std::error::Error for StatusParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    // VariantStatus tests
    #[test]
    fn test_variant_status_default() {
        assert_eq!(VariantStatus::default(), VariantStatus::Active);
    }

    #[test]
    fn test_variant_status_display() {
        assert_eq!(VariantStatus::Active.to_string(), "active");
        assert_eq!(VariantStatus::Inactive.to_string(), "inactive");
        assert_eq!(VariantStatus::Deprecated.to_string(), "deprecated");
    }

    #[test]
    fn test_variant_status_from_str() {
        assert_eq!(VariantStatus::from_str("active").unwrap(), VariantStatus::Active);
        assert_eq!(VariantStatus::from_str("ACTIVE").unwrap(), VariantStatus::Active);
        assert_eq!(VariantStatus::from_str("inactive").unwrap(), VariantStatus::Inactive);
        assert_eq!(VariantStatus::from_str("deprecated").unwrap(), VariantStatus::Deprecated);
        assert!(VariantStatus::from_str("invalid").is_err());
    }

    #[test]
    fn test_variant_status_serialize() {
        let json = serde_json::to_string(&VariantStatus::Active).unwrap();
        assert_eq!(json, "\"active\"");
    }

    #[test]
    fn test_variant_status_deserialize() {
        let status: VariantStatus = serde_json::from_str("\"active\"").unwrap();
        assert_eq!(status, VariantStatus::Active);

        let status: VariantStatus = serde_json::from_str("\"INACTIVE\"").unwrap();
        assert_eq!(status, VariantStatus::Inactive);
    }

    #[test]
    fn test_variant_status_is_claimable() {
        assert!(VariantStatus::Active.is_claimable());
        assert!(!VariantStatus::Inactive.is_claimable());
        assert!(!VariantStatus::Deprecated.is_claimable());
    }

    // ClaimStatus tests
    #[test]
    fn test_claim_status_default() {
        assert_eq!(ClaimStatus::default(), ClaimStatus::Pending);
    }

    #[test]
    fn test_claim_status_display() {
        assert_eq!(ClaimStatus::Pending.to_string(), "pending");
        assert_eq!(ClaimStatus::Claimed.to_string(), "claimed");
        assert_eq!(ClaimStatus::Expired.to_string(), "expired");
        assert_eq!(ClaimStatus::Cancelled.to_string(), "cancelled");
    }

    #[test]
    fn test_claim_status_from_str() {
        assert_eq!(ClaimStatus::from_str("pending").unwrap(), ClaimStatus::Pending);
        assert_eq!(ClaimStatus::from_str("claimed").unwrap(), ClaimStatus::Claimed);
        assert_eq!(ClaimStatus::from_str("cancelled").unwrap(), ClaimStatus::Cancelled);
        assert_eq!(ClaimStatus::from_str("canceled").unwrap(), ClaimStatus::Cancelled); // US spelling
        assert!(ClaimStatus::from_str("invalid").is_err());
    }

    #[test]
    fn test_claim_status_serialize() {
        let json = serde_json::to_string(&ClaimStatus::Claimed).unwrap();
        assert_eq!(json, "\"claimed\"");
    }

    #[test]
    fn test_claim_status_is_final() {
        assert!(!ClaimStatus::Pending.is_final());
        assert!(ClaimStatus::Claimed.is_final());
        assert!(ClaimStatus::Expired.is_final());
        assert!(ClaimStatus::Cancelled.is_final());
    }

    #[test]
    fn test_claim_status_is_successful() {
        assert!(!ClaimStatus::Pending.is_successful());
        assert!(ClaimStatus::Claimed.is_successful());
        assert!(!ClaimStatus::Expired.is_successful());
        assert!(!ClaimStatus::Cancelled.is_successful());
    }

    // StatusParseError tests
    #[test]
    fn test_status_parse_error_display() {
        let err = StatusParseError::InvalidVariantStatus("bad".to_string());
        assert!(err.to_string().contains("bad"));
        assert!(err.to_string().contains("active"));

        let err = StatusParseError::InvalidClaimStatus("bad".to_string());
        assert!(err.to_string().contains("bad"));
        assert!(err.to_string().contains("claimed"));
    }
}
