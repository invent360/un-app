//! License claiming service for end users

use std::sync::Arc;
use crate::types::{ClaimRequest, ClaimResponse, ClaimPageData, License, LicenseDto, LicenseSummary, SplitType, AppError};
use uno_api::traits::LicenseRepository;

/// License service for claim operations
#[derive(Clone)]
pub struct LicenseService {
    license_repo: Arc<dyn LicenseRepository + Send + Sync>,
}

impl LicenseService {
    /// Create a new license service.
    pub fn new(license_repo: Arc<dyn LicenseRepository + Send + Sync>) -> Self {
        Self { license_repo }
    }

    /// Claim a license by split type (auto-assigns an available license).
    pub async fn claim_by_split_type(&self, split_type: SplitType, device_id: Option<String>) -> Result<ClaimResponse, AppError> {
        let api_split = convert_to_api_split(split_type);

        // Get the first unclaimed license of this type
        let license = self.license_repo
            .get_first_unclaimed(api_split)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let license = match license {
            Some(l) => l,
            None => {
                return Ok(ClaimResponse::error("No licenses available for this split type"));
            }
        };

        // Claim the license
        let claimed_license = self.license_repo
            .claim(license.id, device_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(ClaimResponse::success(convert_to_local_license(claimed_license)))
    }

    /// Reserve a license by split type (returns license info WITHOUT marking as claimed).
    /// This is phase 1 of the two-phase claim process.
    pub async fn reserve_by_split_type(&self, split_type: SplitType) -> Result<ClaimResponse, AppError> {
        let api_split = convert_to_api_split(split_type);

        // Get the first unclaimed license of this type (without claiming)
        let license = self.license_repo
            .get_first_unclaimed(api_split)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let license = match license {
            Some(l) => l,
            None => {
                return Ok(ClaimResponse::error("No licenses available for this split type"));
            }
        };

        // Return license info without marking as claimed
        Ok(ClaimResponse::success(convert_to_local_license(license)))
    }

    /// Reserve the next available license (any split type).
    /// Returns license info WITHOUT marking as claimed.
    /// This is phase 1 of the simplified two-phase claim process.
    pub async fn reserve_next_available(&self) -> Result<ClaimResponse, AppError> {
        // Get the first unclaimed license (any split type)
        let license = self.license_repo
            .get_first_unclaimed_any()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let license = match license {
            Some(l) => l,
            None => {
                return Ok(ClaimResponse::error("No licenses available"));
            }
        };

        // Return license info without marking as claimed
        Ok(ClaimResponse::success(convert_to_local_license(license)))
    }

    /// Check if any licenses are available for claiming.
    pub async fn has_available_licenses(&self) -> Result<bool, AppError> {
        let count = self.license_repo
            .count_unclaimed()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    /// Get the count of available (unclaimed) licenses.
    pub async fn get_available_count(&self) -> Result<i64, AppError> {
        self.license_repo
            .count_unclaimed()
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Confirm a license claim by ID (marks as claimed).
    /// This is phase 2 of the two-phase claim process - called when user copies the key.
    pub async fn confirm_claim(&self, license_id: &str, device_id: Option<String>) -> Result<ClaimResponse, AppError> {
        // Parse the license ID
        let id = uuid::Uuid::parse_str(license_id)
            .map_err(|e| AppError::ValidationError(format!("Invalid license ID: {}", e)))?;

        // Check if license exists and is not already claimed
        let license = self.license_repo
            .get_by_id(id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let license = match license {
            Some(l) => l,
            None => {
                return Ok(ClaimResponse::error("License not found"));
            }
        };

        // Check if already claimed
        if license.claimed {
            // If already claimed, just return success (idempotent)
            return Ok(ClaimResponse::success(convert_to_local_license(license)));
        }

        // Claim the license
        let claimed_license = self.license_repo
            .claim(id, device_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(ClaimResponse::success(convert_to_local_license(claimed_license)))
    }

    /// Claim a license by lease code.
    pub async fn claim_license(&self, request: ClaimRequest) -> Result<ClaimResponse, AppError> {
        // Find the license by lease code
        let license = self.license_repo
            .get_by_lease_code(&request.lease_code)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let license = match license {
            Some(l) => l,
            None => {
                return Ok(ClaimResponse::error("License not found"));
            }
        };

        // Check if already claimed
        if license.claimed {
            return Ok(ClaimResponse::error("License has already been claimed"));
        }

        // Check if valid
        if !license.is_valid() {
            if license.is_expired() {
                return Ok(ClaimResponse::error("License has expired"));
            } else {
                return Ok(ClaimResponse::error("License is not yet valid"));
            }
        }

        // Claim the license
        let claimed_license = self.license_repo
            .claim(license.id, request.device_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(ClaimResponse::success(convert_to_local_license(claimed_license)))
    }

    /// Get a license by lease code (for display).
    pub async fn get_license_by_code(&self, lease_code: &str) -> Result<Option<ClaimPageData>, AppError> {
        let license = self.license_repo
            .get_by_lease_code(lease_code)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(license.map(|l| ClaimPageData::from(convert_to_local_license(l))))
    }

    /// Get available license count by split type.
    pub async fn get_available_by_split(&self, split_type: SplitType) -> Result<i64, AppError> {
        let api_split = convert_to_api_split(split_type);
        self.license_repo
            .count_unclaimed_by_split(api_split)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Get license summary statistics.
    pub async fn get_summary(&self) -> Result<LicenseSummary, AppError> {
        let api_summary = self.license_repo
            .get_summary()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(convert_to_local_summary(api_summary))
    }

    /// Get available licenses by split type.
    pub async fn get_available_splits(&self) -> Result<Vec<SplitTypeAvailability>, AppError> {
        // Use summary to get both available and claimed counts
        let summary = self.license_repo
            .get_summary()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = summary.by_split_type
            .into_iter()
            .map(|s| SplitTypeAvailability {
                split_type: convert_to_local_split(s.split_type),
                available: s.available,
                claimed: s.claimed,
                total: s.total,
            })
            .collect();

        Ok(result)
    }
}

/// Availability information for a split type.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SplitTypeAvailability {
    pub split_type: SplitType,
    pub available: i64,
    pub claimed: i64,
    pub total: i64,
}

// Conversion functions between local and uno-api types

fn convert_to_local_license(api_license: uno_api::models::License) -> License {
    License {
        id: api_license.id,
        lease_code: api_license.lease_code,
        valid_from: api_license.valid_from,
        valid_to: api_license.valid_to,
        split_type: convert_to_local_split(api_license.split_type),
        claimed: api_license.claimed,
        bound_to_device: api_license.bound_to_device,
        device_id: api_license.device_id,
        claimed_at: api_license.claimed_at,
        created_at: api_license.created_at,
    }
}

fn convert_to_local_split(api_split: uno_api::models::SplitType) -> SplitType {
    match api_split {
        uno_api::models::SplitType::Split5050 => SplitType::Split5050,
        uno_api::models::SplitType::Split5545 => SplitType::Split5545,
        uno_api::models::SplitType::Split6040 => SplitType::Split6040,
    }
}

fn convert_to_api_split(local_split: SplitType) -> uno_api::models::SplitType {
    match local_split {
        SplitType::Split5050 => uno_api::models::SplitType::Split5050,
        SplitType::Split5545 => uno_api::models::SplitType::Split5545,
        SplitType::Split6040 => uno_api::models::SplitType::Split6040,
    }
}

fn convert_to_local_summary(api_summary: uno_api::models::LicenseSummary) -> LicenseSummary {
    use crate::types::SplitTypeSummary;

    LicenseSummary {
        total: api_summary.total,
        claimed: api_summary.claimed,
        unclaimed: api_summary.available,  // uno-api uses 'available', uno-app uses 'unclaimed'
        expired: api_summary.expired,
        by_split_type: api_summary
            .by_split_type
            .into_iter()
            .map(|s| SplitTypeSummary {
                split_type: convert_to_local_split(s.split_type),
                total: s.total,
                claimed: s.claimed,
                unclaimed: s.available,  // uno-api uses 'available', uno-app uses 'unclaimed'
            })
            .collect(),
    }
}
