//! License models and DTOs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// License status for filtering and display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LicenseStatus {
    /// All statuses.
    #[default]
    All,
    /// Available for claiming.
    Available,
    /// Already claimed.
    Claimed,
    /// Expired.
    Expired,
    /// Revoked.
    Revoked,
}

/// Split type for license revenue sharing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SplitType {
    /// 50% user, 50% operator
    #[default]
    #[serde(rename = "50:50")]
    Split5050,
    /// 55% user, 45% operator
    #[serde(rename = "55:45")]
    Split5545,
    /// 60% user, 40% operator
    #[serde(rename = "60:40")]
    Split6040,
}

impl SplitType {
    /// Get the user share percentage.
    pub fn user_share(&self) -> i32 {
        match self {
            SplitType::Split5050 => 50,
            SplitType::Split5545 => 55,
            SplitType::Split6040 => 60,
        }
    }

    /// Get the operator share percentage.
    pub fn operator_share(&self) -> i32 {
        100 - self.user_share()
    }

    /// Get display string.
    pub fn display(&self) -> &'static str {
        match self {
            SplitType::Split5050 => "50:50",
            SplitType::Split5545 => "55:45",
            SplitType::Split6040 => "60:40",
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "50:50" | "5050" => Some(SplitType::Split5050),
            "55:45" | "5545" => Some(SplitType::Split5545),
            "60:40" | "6040" => Some(SplitType::Split6040),
            _ => None,
        }
    }

    /// Convert to database string (matches PostgreSQL split_type enum).
    pub fn to_db_str(&self) -> &'static str {
        match self {
            SplitType::Split5050 => "5050",
            SplitType::Split5545 => "5545",
            SplitType::Split6040 => "6040",
        }
    }
}

/// License entity (minimal database representation).
///
/// The `id` field stores the full upstream identifier (VARCHAR(66) in database).
/// This preserves blockchain addresses and other external IDs without truncation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    /// Unique license identifier (full upstream ID, not truncated).
    /// Format: blockchain hex address (e.g., "0x0111e1758d35...") or UUID string.
    pub id: String,
    /// Lease code (displayed to user for claiming).
    pub lease_code: String,
    /// Valid from date.
    pub valid_from: DateTime<Utc>,
    /// Valid until date.
    pub valid_to: DateTime<Utc>,
    /// Revenue split type.
    pub split_type: SplitType,
    /// Whether the license has been claimed.
    pub claimed: bool,
    /// Whether the license is bound to a device.
    pub bound_to_device: bool,
    /// Device ID if bound.
    pub device_id: Option<String>,
    /// Claimed at timestamp.
    pub claimed_at: Option<DateTime<Utc>>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

impl License {
    /// Create a new unclaimed license with auto-generated ID.
    pub fn new(
        lease_code: String,
        valid_from: DateTime<Utc>,
        valid_to: DateTime<Utc>,
        split_type: SplitType,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            lease_code,
            valid_from,
            valid_to,
            split_type,
            claimed: false,
            bound_to_device: false,
            device_id: None,
            claimed_at: None,
            created_at: Utc::now(),
        }
    }

    /// Create a new unclaimed license with a custom ID.
    ///
    /// The ID is preserved exactly as provided (no truncation).
    /// This supports blockchain hex addresses (e.g., "0x0111e1758d35...")
    /// and other external identifier formats up to 66 characters.
    pub fn with_custom_id(
        custom_id: &str,
        lease_code: String,
        valid_from: DateTime<Utc>,
        valid_to: DateTime<Utc>,
        split_type: SplitType,
    ) -> Self {
        Self {
            id: custom_id.to_string(),
            lease_code,
            valid_from,
            valid_to,
            split_type,
            claimed: false,
            bound_to_device: false,
            device_id: None,
            claimed_at: None,
            created_at: Utc::now(),
        }
    }

    /// Check if the license is currently valid (not expired).
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        now >= self.valid_from && now <= self.valid_to
    }

    /// Check if the license can be claimed.
    pub fn is_claimable(&self) -> bool {
        !self.claimed && self.is_valid()
    }

    /// Check if the license is expired.
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.valid_to
    }
}

/// Input for creating/publishing a new license.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInput {
    /// License ID (optional, will be generated if not provided).
    /// Example: "0x0111e1758d35de5306c4feec2e87db6fcf593d055b22a32a4d49e1c1d1cb9281"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Lease code (required).
    /// Example: "7e457a39-5f5d-41cb-9eda-737ab99923c6"
    pub lease_code: String,
    /// Valid from date.
    pub valid_from: DateTime<Utc>,
    /// Valid until date.
    pub valid_to: DateTime<Utc>,
    /// Revenue split type.
    pub split_type: SplitType,
}

impl LicenseInput {
    /// Create a new license input.
    pub fn new(lease_code: impl Into<String>, valid_from: DateTime<Utc>, valid_to: DateTime<Utc>, split_type: SplitType) -> Self {
        Self {
            id: None,
            lease_code: lease_code.into(),
            valid_from,
            valid_to,
            split_type,
        }
    }

    /// Set a custom ID.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set a custom lease code.
    pub fn with_lease_code(mut self, code: impl Into<String>) -> Self {
        self.lease_code = code.into();
        self
    }
}

/// License DTO for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseDto {
    pub id: String,
    pub lease_code: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub split_type: SplitType,
    pub claimed: bool,
    pub bound_to_device: bool,
    pub is_valid: bool,
    pub is_expired: bool,
    pub created_at: DateTime<Utc>,
}

impl From<License> for LicenseDto {
    fn from(license: License) -> Self {
        let is_valid = license.is_valid();
        let is_expired = license.is_expired();
        Self {
            id: license.id.to_string(),
            lease_code: license.lease_code,
            valid_from: license.valid_from,
            valid_to: license.valid_to,
            split_type: license.split_type,
            claimed: license.claimed,
            bound_to_device: license.bound_to_device,
            is_valid,
            is_expired,
            created_at: license.created_at,
        }
    }
}

/// Claim request from a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimRequest {
    /// The lease code to claim.
    pub lease_code: String,
    /// Optional device ID to bind to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

/// Claim response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponse {
    pub success: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseDto>,
    pub message: String,
}

impl ClaimResponse {
    pub fn success(license: License) -> Self {
        Self {
            success: true,
            license: Some(LicenseDto::from(license)),
            message: "License claimed successfully".to_string(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            license: None,
            message: message.into(),
        }
    }
}

/// Generate a unique lease code.
/// Format: XXXX-XXXX-XXXX (uppercase alphanumeric, no ambiguous chars)
pub fn generate_lease_code() -> String {
    use rand::Rng;
    // Exclude ambiguous characters: 0, O, I, L, 1
    const CHARSET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ23456789";
    let mut rng = rand::rng();

    let segments: Vec<String> = (0..3)
        .map(|_| {
            (0..4)
                .map(|_| {
                    let idx = rng.random_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect()
        })
        .collect();

    segments.join("-")
}

/// Summary of licenses by split type.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LicenseSummary {
    pub total: i64,
    pub claimed: i64,
    pub available: i64,
    pub expired: i64,
    pub by_split_type: Vec<SplitTypeSummary>,
}

/// Summary for a specific split type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitTypeSummary {
    pub split_type: SplitType,
    pub total: i64,
    pub claimed: i64,
    pub available: i64,
}
