//! License-related type definitions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Split type for license revenue sharing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "split_type"))]
pub enum SplitType {
    #[serde(rename = "50:50")]
    #[cfg_attr(feature = "ssr", sqlx(rename = "50:50"))]
    Split5050,
    #[serde(rename = "55:45")]
    #[cfg_attr(feature = "ssr", sqlx(rename = "55:45"))]
    Split5545,
    #[serde(rename = "60:40")]
    #[cfg_attr(feature = "ssr", sqlx(rename = "60:40"))]
    Split6040,
}

impl SplitType {
    /// Get user share percentage.
    pub fn user_share(&self) -> i32 {
        match self {
            SplitType::Split5050 => 50,
            SplitType::Split5545 => 55,
            SplitType::Split6040 => 60,
        }
    }

    /// Get operator share percentage.
    pub fn operator_share(&self) -> i32 {
        100 - self.user_share()
    }

    /// Display format (e.g., "50:50").
    pub fn display(&self) -> &'static str {
        match self {
            SplitType::Split5050 => "50:50",
            SplitType::Split5545 => "55:45",
            SplitType::Split6040 => "60:40",
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "50:50" | "5050" => Some(SplitType::Split5050),
            "55:45" | "5545" => Some(SplitType::Split5545),
            "60:40" | "6040" => Some(SplitType::Split6040),
            _ => None,
        }
    }

    /// All available split types.
    pub fn all() -> [SplitType; 3] {
        [SplitType::Split5050, SplitType::Split5545, SplitType::Split6040]
    }
}

impl std::fmt::Display for SplitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

/// Availability status for license variants.
/// Used to determine what UI state to show in the claim wizard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AvailabilityStatus {
    /// Licenses are available to claim
    #[default]
    Available,
    /// Licenses exist but all have been claimed
    AllClaimed,
    /// Licenses exist but all have expired
    AllExpired,
    /// No licenses exist in the system for this split type
    NoneInSystem,
}

impl AvailabilityStatus {
    /// Check if any licenses can be claimed
    pub fn can_claim(&self) -> bool {
        matches!(self, Self::Available)
    }

    /// Check if licenses exist (even if not claimable)
    pub fn has_licenses(&self) -> bool {
        matches!(self, Self::Available | Self::AllClaimed | Self::AllExpired)
    }

    /// Get a user-friendly message for this status
    pub fn message(&self) -> &'static str {
        match self {
            Self::Available => "Licenses available",
            Self::AllClaimed => "All licenses have been claimed",
            Self::AllExpired => "All licenses have expired",
            Self::NoneInSystem => "Coming soon",
        }
    }
}

/// License model for claim service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct License {
    pub id: uuid::Uuid,
    pub lease_code: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub split_type: SplitType,
    pub claimed: bool,
    pub bound_to_device: bool,
    pub device_id: Option<String>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl License {
    /// Check if license is currently valid (within date range and not expired).
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        now >= self.valid_from && now < self.valid_to
    }

    /// Check if license has expired.
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.valid_to
    }

    /// Check if license is available for claiming.
    pub fn is_available(&self) -> bool {
        !self.claimed && self.is_valid()
    }

    /// Get user share percentage.
    pub fn user_share(&self) -> i32 {
        self.split_type.user_share()
    }

    /// Get operator share percentage.
    pub fn operator_share(&self) -> i32 {
        self.split_type.operator_share()
    }
}

/// License DTO for API responses (minimal info for claiming).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseDto {
    pub id: uuid::Uuid,
    pub lease_code: String,
    pub split_type: SplitType,
    pub user_share: i32,
    pub operator_share: i32,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub claimed: bool,
}

impl From<License> for LicenseDto {
    fn from(license: License) -> Self {
        Self {
            id: license.id,
            lease_code: license.lease_code.clone(),
            split_type: license.split_type,
            user_share: license.split_type.user_share(),
            operator_share: license.split_type.operator_share(),
            valid_from: license.valid_from,
            valid_to: license.valid_to,
            claimed: license.claimed,
        }
    }
}

/// Request to claim a license by lease code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimRequest {
    pub lease_code: String,
    #[serde(default)]
    pub device_id: Option<String>,
}

/// Response after claiming a license.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimResponse {
    pub success: bool,
    pub license: Option<LicenseDto>,
    /// The license ID (UUID as string)
    pub license_id: Option<String>,
    /// The license key (same as lease_code)
    pub license_key: Option<String>,
    pub error: Option<String>,
    /// Optional message (for errors or info)
    #[serde(default)]
    pub message: Option<String>,
    /// When the license was claimed
    #[serde(default)]
    pub claimed_at: Option<DateTime<Utc>>,
}

impl ClaimResponse {
    /// Create a successful claim response.
    pub fn success(license: License) -> Self {
        let license_id = license.id.to_string();
        let license_key = license.lease_code.clone();
        let claimed_at = license.claimed_at;
        Self {
            success: true,
            license: Some(LicenseDto::from(license)),
            license_id: Some(license_id),
            license_key: Some(license_key),
            error: None,
            message: None,
            claimed_at,
        }
    }

    /// Create an error claim response.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            license: None,
            license_id: None,
            license_key: None,
            error: Some(message.into()),
            message: None,
            claimed_at: None,
        }
    }
}

/// Summary statistics for licenses.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LicenseSummary {
    pub total: i64,
    pub claimed: i64,
    pub unclaimed: i64,
    pub expired: i64,
    pub by_split_type: Vec<SplitTypeSummary>,
}

/// Summary for a specific split type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitTypeSummary {
    pub split_type: SplitType,
    pub total: i64,
    pub claimed: i64,
    pub unclaimed: i64,
}

/// License split option for UI display (replaces LicenseVariant).
/// This is a UI-only type representing available split options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseVariant {
    pub id: i32,
    pub user_share_percentage: i32,
    pub operator_share_percentage: i32,
    pub lease_duration_months: i32,
    pub total_quantity: i32,
    pub claimed_count: i32,
    pub min_monthly_earnings: Option<f64>,
    pub max_monthly_earnings: Option<f64>,
    pub display_name: Option<String>,
    pub display_order: i32,
    pub is_featured: bool,
    /// Availability status for this variant
    #[serde(default)]
    pub availability_status: AvailabilityStatus,
}

impl LicenseVariant {
    /// Create a variant from SplitType with availability and claimed counts.
    pub fn from_split_with_counts(split_type: SplitType, available: i64, claimed: i64, total: i64) -> Self {
        let id = match split_type {
            SplitType::Split5050 => 1,
            SplitType::Split5545 => 2,
            SplitType::Split6040 => 3,
        };

        // Determine availability status based on counts
        let availability_status = if total == 0 {
            AvailabilityStatus::NoneInSystem
        } else if available > 0 {
            AvailabilityStatus::Available
        } else if claimed == total {
            AvailabilityStatus::AllClaimed
        } else {
            // total > 0, available == 0, but not all claimed = some must be expired
            AvailabilityStatus::AllExpired
        };

        Self {
            id,
            user_share_percentage: split_type.user_share(),
            operator_share_percentage: split_type.operator_share(),
            lease_duration_months: 12, // Default to 12 months
            total_quantity: total as i32,
            claimed_count: claimed as i32,
            min_monthly_earnings: None,
            max_monthly_earnings: None,
            display_name: Some(split_type.display().to_string()),
            display_order: id,
            is_featured: split_type == SplitType::Split6040,
            availability_status,
        }
    }

    /// Create a variant from SplitType with availability count (legacy, sets claimed to 0).
    pub fn from_split(split_type: SplitType, available: i64) -> Self {
        Self::from_split_with_counts(split_type, available, 0, available)
    }

    /// Calculate remaining available licenses.
    pub fn remaining(&self) -> i32 {
        self.total_quantity - self.claimed_count
    }

    /// Check if this variant is available for claiming.
    pub fn is_available(&self) -> bool {
        self.remaining() > 0
    }

    /// Calculate claim progress as percentage (0.0 - 100.0).
    pub fn progress(&self) -> f64 {
        if self.total_quantity == 0 {
            return 0.0;
        }
        (self.claimed_count as f64 / self.total_quantity as f64) * 100.0
    }

    /// Format split as display string (e.g., "60:40").
    pub fn split_display(&self) -> String {
        format!("{}:{}", self.user_share_percentage, self.operator_share_percentage)
    }

    /// Get the SplitType for this variant.
    pub fn split_type(&self) -> SplitType {
        match self.user_share_percentage {
            50 => SplitType::Split5050,
            55 => SplitType::Split5545,
            60 => SplitType::Split6040,
            _ => SplitType::Split5050, // Default fallback
        }
    }
}

/// Claim page data for UI display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimPageData {
    /// The license key (same as lease_code)
    pub license_key: String,
    /// Same as license_key, kept for backward compatibility
    pub claim_token: String,
    /// Lease code
    pub lease_code: String,
    /// User's revenue share percentage
    pub user_share: i32,
    /// Operator's revenue share percentage
    pub operator_share: i32,
    /// License valid from date
    pub valid_from: DateTime<Utc>,
    /// License valid to date
    pub valid_to: DateTime<Utc>,
    /// Minimum expected earnings (placeholder)
    pub min_earnings: Option<f64>,
    /// Maximum expected earnings (placeholder)
    pub max_earnings: Option<f64>,
}

impl From<License> for ClaimPageData {
    fn from(license: License) -> Self {
        Self {
            license_key: license.lease_code.clone(),
            claim_token: license.lease_code.clone(),
            lease_code: license.lease_code,
            user_share: license.split_type.user_share(),
            operator_share: license.split_type.operator_share(),
            valid_from: license.valid_from,
            valid_to: license.valid_to,
            min_earnings: None, // Can be calculated based on split type
            max_earnings: None, // Can be calculated based on split type
        }
    }
}
