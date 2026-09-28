//! Referral-related type definitions
//!
//! This module defines types for the referral system where users can apply
//! to become referrals (agents) and distribute their referral codes to
//! potential license operators.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Status of a referral application
///
/// Referrals progress through:
/// - `Pending`: Application submitted, awaiting review
/// - `Active`: Approved and able to distribute referral code
/// - `Suspended`: Temporarily suspended (can be reactivated)
/// - `Rejected`: Application rejected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "referral_status", rename_all = "lowercase"))]
pub enum ReferralStatus {
    /// Application is pending review
    #[default]
    Pending,
    /// Referral is active and can distribute codes
    Active,
    /// Referral is temporarily suspended
    Suspended,
    /// Application was rejected
    Rejected,
}

impl ReferralStatus {
    /// All possible referral status values
    pub const ALL: &'static [ReferralStatus] = &[
        ReferralStatus::Pending,
        ReferralStatus::Active,
        ReferralStatus::Suspended,
        ReferralStatus::Rejected,
    ];

    /// Check if the referral is active
    pub fn is_active(&self) -> bool {
        matches!(self, ReferralStatus::Active)
    }

    /// Check if the referral can earn commissions
    pub fn can_earn(&self) -> bool {
        matches!(self, ReferralStatus::Active)
    }

    /// Check if the status is final (cannot change)
    pub fn is_final(&self) -> bool {
        matches!(self, ReferralStatus::Rejected)
    }

    /// Get the status as a static string slice
    pub fn as_str(&self) -> &'static str {
        match self {
            ReferralStatus::Pending => "pending",
            ReferralStatus::Active => "active",
            ReferralStatus::Suspended => "suspended",
            ReferralStatus::Rejected => "rejected",
        }
    }
}

impl fmt::Display for ReferralStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ReferralStatus {
    type Err = ReferralStatusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(ReferralStatus::Pending),
            "active" => Ok(ReferralStatus::Active),
            "suspended" => Ok(ReferralStatus::Suspended),
            "rejected" => Ok(ReferralStatus::Rejected),
            _ => Err(ReferralStatusParseError(s.to_string())),
        }
    }
}

impl Serialize for ReferralStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ReferralStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ReferralStatus::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Error type for referral status parsing failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferralStatusParseError(pub String);

impl fmt::Display for ReferralStatusParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid referral status: '{}'. Expected one of: pending, active, suspended, rejected",
            self.0
        )
    }
}

impl std::error::Error for ReferralStatusParseError {}

/// Referral model representing an agent who can distribute referral codes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Referral {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub country_code: String,
    pub referral_code: String,
    pub status: ReferralStatus,
    pub joined_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Referral {
    /// Check if the referral is active and can earn commissions
    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }

    /// Check if the referral code is valid for use
    pub fn is_valid_for_use(&self) -> bool {
        self.status.is_active()
    }
}

/// Response for referral code validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateReferralResponse {
    /// Whether the referral code is valid
    pub valid: bool,
    /// Optional message (e.g., error reason)
    pub message: Option<String>,
    /// The referral's username (if valid, for display purposes)
    pub referral_name: Option<String>,
}

impl ValidateReferralResponse {
    /// Create a valid response
    pub fn valid(referral_name: String) -> Self {
        Self {
            valid: true,
            message: None,
            referral_name: Some(referral_name),
        }
    }

    /// Create an invalid response
    pub fn invalid(message: impl Into<String>) -> Self {
        Self {
            valid: false,
            message: Some(message.into()),
            referral_name: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_referral_status_default() {
        assert_eq!(ReferralStatus::default(), ReferralStatus::Pending);
    }

    #[test]
    fn test_referral_status_display() {
        assert_eq!(ReferralStatus::Pending.to_string(), "pending");
        assert_eq!(ReferralStatus::Active.to_string(), "active");
        assert_eq!(ReferralStatus::Suspended.to_string(), "suspended");
        assert_eq!(ReferralStatus::Rejected.to_string(), "rejected");
    }

    #[test]
    fn test_referral_status_from_str() {
        assert_eq!(ReferralStatus::from_str("pending").unwrap(), ReferralStatus::Pending);
        assert_eq!(ReferralStatus::from_str("ACTIVE").unwrap(), ReferralStatus::Active);
        assert_eq!(ReferralStatus::from_str("suspended").unwrap(), ReferralStatus::Suspended);
        assert_eq!(ReferralStatus::from_str("rejected").unwrap(), ReferralStatus::Rejected);
        assert!(ReferralStatus::from_str("invalid").is_err());
    }

    #[test]
    fn test_referral_status_serialize() {
        let json = serde_json::to_string(&ReferralStatus::Active).unwrap();
        assert_eq!(json, "\"active\"");
    }

    #[test]
    fn test_referral_status_is_active() {
        assert!(!ReferralStatus::Pending.is_active());
        assert!(ReferralStatus::Active.is_active());
        assert!(!ReferralStatus::Suspended.is_active());
        assert!(!ReferralStatus::Rejected.is_active());
    }

    #[test]
    fn test_validate_response_valid() {
        let resp = ValidateReferralResponse::valid("TestUser".to_string());
        assert!(resp.valid);
        assert!(resp.message.is_none());
        assert_eq!(resp.referral_name, Some("TestUser".to_string()));
    }

    #[test]
    fn test_validate_response_invalid() {
        let resp = ValidateReferralResponse::invalid("Code not found");
        assert!(!resp.valid);
        assert_eq!(resp.message, Some("Code not found".to_string()));
        assert!(resp.referral_name.is_none());
    }
}
