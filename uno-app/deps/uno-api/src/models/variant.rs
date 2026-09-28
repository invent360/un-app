//! Variant models and DTOs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Variant status for filtering and display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum VariantStatus {
    /// All statuses.
    #[default]
    All,
    /// Active variants.
    Active,
    /// Inactive variants.
    Inactive,
}

/// Variant entity representing a license variant/tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantDto {
    /// Unique variant identifier.
    pub id: String,
    /// Variant name.
    pub name: String,
    /// Description of the variant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Price in cents.
    #[serde(default)]
    pub price_cents: i64,
    /// Number of licenses available for this variant.
    #[serde(default)]
    pub available_licenses: i64,
    /// Whether this variant is active.
    #[serde(default = "default_true")]
    pub active: bool,
    /// Creation timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    /// Last update timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

fn default_true() -> bool {
    true
}

/// Input for creating or updating a variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantInput {
    /// Variant ID (optional for create, required for update).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Variant name.
    pub name: String,
    /// Description of the variant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Price in cents.
    #[serde(default)]
    pub price_cents: i64,
    /// Whether this variant is active.
    #[serde(default = "default_true")]
    pub active: bool,
}

/// Summary statistics for variants.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VariantSummary {
    /// Total number of variants.
    pub total_variants: i64,
    /// Number of active variants.
    pub active_variants: i64,
    /// Total licenses across all variants.
    pub total_licenses: i64,
    /// Total available licenses.
    pub available_licenses: i64,
}
