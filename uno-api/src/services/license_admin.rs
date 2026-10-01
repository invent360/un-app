//! License administration service.

use std::sync::Arc;

use crate::error::ApiError;
use crate::models::*;
use crate::privacy::redact_credential;
use crate::traits::{LicenseFilters, LicenseRepository};

use super::csv_import::{parse_csv, ParsedCsv};

/// License administration service.
///
/// This service handles all license management operations including
/// publishing, importing, claiming, and revoking licenses.
pub struct LicenseAdminService {
    license_repo: Arc<dyn LicenseRepository>,
}

impl LicenseAdminService {
    /// Create a new license admin service.
    pub fn new(license_repo: Arc<dyn LicenseRepository>) -> Self {
        Self { license_repo }
    }

    /// Publish a batch of licenses.
    pub async fn publish_licenses(
        &self,
        request: PublishLicensesRequest,
    ) -> Result<PublishResult, ApiError> {
        let mut created = 0;
        let mut failed = 0;
        let mut errors = Vec::new();
        let mut licenses_to_insert = Vec::new();

        // Validate and prepare all licenses
        for (idx, input) in request.licenses.iter().enumerate() {
            match self.prepare_license(input).await {
                Ok(license) => {
                    licenses_to_insert.push(license);
                }
                Err(e) => {
                    failed += 1;
                    errors.push(PublishError::new(idx, e.to_string()));
                }
            }
        }

        // Batch insert licenses
        if !licenses_to_insert.is_empty() {
            match self.license_repo.insert_batch(&licenses_to_insert).await {
                Ok(count) => {
                    created = count;
                }
                Err(_batch_err) => {
                    // If batch insert fails, try individual inserts
                    for (idx, license) in licenses_to_insert.iter().enumerate() {
                        match self.license_repo.insert(license).await {
                            Ok(()) => created += 1,
                            Err(e) => {
                                failed += 1;
                                errors.push(PublishError::new(idx, e.to_string()));
                            }
                        }
                    }
                }
            }
        }

        Ok(PublishResult::partial(created, failed, errors))
    }

    /// Prepare a single license from input.
    async fn prepare_license(&self, input: &LicenseInput) -> Result<License, ApiError> {
        // Validate date range
        if input.valid_to <= input.valid_from {
            return Err(ApiError::validation("valid_to must be after valid_from"));
        }

        // Validate lease code is not empty
        if input.lease_code.is_empty() {
            return Err(ApiError::validation("lease_code is required"));
        }

        // Check if lease code already exists
        // GOV-01: Redact lease code in error messages to prevent credential exposure
        if self.license_repo.lease_code_exists(&input.lease_code).await? {
            let redacted = redact_credential(&input.lease_code);
            return Err(ApiError::conflict(format!(
                "Lease code already exists: {}",
                redacted
            )));
        }

        // Create license with custom ID if provided, otherwise auto-generate
        let mut license = if let Some(ref custom_id) = input.id {
            License::with_custom_id(
                custom_id,
                input.lease_code.clone(),
                input.valid_from,
                input.valid_to,
                input.split_type,
            )
        } else {
            License::new(
                input.lease_code.clone(),
                input.valid_from,
                input.valid_to,
                input.split_type,
            )
        };

        // R5-06: Copy exact shares and provenance from input
        if let (Some(uno), Some(ulo), Some(agent)) = (input.uno_share_pct, input.ulo_share_pct, input.agent_share_pct) {
            license = license.with_exact_shares(uno, ulo, agent);
        }
        if let Some(ref system) = input.source_system {
            license = license.with_provenance(
                system.clone(),
                input.source_version.clone(),
                input.source_record_id.clone(),
            );
        }

        Ok(license)
    }

    /// Import licenses from CSV data.
    pub async fn import_csv(&self, request: CsvImportRequest) -> Result<ImportResult, ApiError> {
        let ParsedCsv {
            licenses,
            errors: parse_errors,
            total_rows,
        } = parse_csv(&request.csv_data, request.has_header)?;

        // Convert to publish request
        let publish_request = PublishLicensesRequest {
            licenses,
            idempotency_key: None,
        };

        let publish_result = self.publish_licenses(publish_request).await?;

        // Combine errors
        let mut all_errors: Vec<ImportError> = parse_errors;
        for err in publish_result.errors {
            all_errors.push(ImportError::new(err.index + 1, err.message));
        }

        Ok(ImportResult {
            rows_parsed: total_rows,
            licenses_created: publish_result.created,
            rows_failed: all_errors.len(),
            errors: all_errors,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Get a license by ID.
    pub async fn get_license(&self, id: &str) -> Result<Option<LicenseDto>, ApiError> {
        let license = self.license_repo.get_by_id(id).await?;
        Ok(license.map(LicenseDto::from))
    }

    /// Get a license by lease code.
    pub async fn get_license_by_code(&self, code: &str) -> Result<Option<LicenseDto>, ApiError> {
        let license = self.license_repo.get_by_lease_code(code).await?;
        Ok(license.map(LicenseDto::from))
    }

    /// Claim a license by lease code.
    pub async fn claim_license(&self, request: ClaimRequest) -> Result<ClaimResponse, ApiError> {
        // Find the license by lease code
        let license = self
            .license_repo
            .get_by_lease_code(&request.lease_code)
            .await?;

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
        let claimed_license = self
            .license_repo
            .claim(&license.id, request.device_id)
            .await?;

        Ok(ClaimResponse::success(claimed_license))
    }

    /// Search licenses with filters.
    pub async fn search_licenses(
        &self,
        filters: LicenseFilters,
        pagination: PaginationParams,
    ) -> Result<Paginated<LicenseDto>, ApiError> {
        let result = self.license_repo.search(&filters, &pagination).await?;
        Ok(Paginated::new(
            result.items.into_iter().map(LicenseDto::from).collect(),
            result.total,
            &pagination,
        ))
    }

    /// Revoke licenses.
    pub async fn revoke_licenses(&self, request: RevokeRequest) -> Result<RevokeResult, ApiError> {
        use crate::error::DbError;

        let mut revoked = 0;
        let mut failed = 0;
        let mut errors = Vec::new();

        for id_str in &request.license_ids {
            // Validate ID is not empty
            if id_str.is_empty() {
                failed += 1;
                errors.push(RevokeError {
                    license_id: id_str.clone(),
                    message: "Empty license ID".to_string(),
                });
                continue;
            }

            match self.license_repo.delete(id_str).await {
                Ok(()) => revoked += 1,
                Err(DbError::NotFound(_)) => {
                    failed += 1;
                    errors.push(RevokeError {
                        license_id: id_str.clone(),
                        message: "License not found".to_string(),
                    });
                }
                Err(e) => {
                    failed += 1;
                    errors.push(RevokeError {
                        license_id: id_str.clone(),
                        message: e.to_string(),
                    });
                }
            }
        }

        Ok(RevokeResult {
            revoked,
            failed,
            errors,
        })
    }

    /// Get license summary statistics.
    pub async fn get_summary(&self) -> Result<LicenseSummary, ApiError> {
        let summary = self.license_repo.get_summary().await?;
        Ok(summary)
    }

    /// Get available (unclaimed) licenses by split type.
    pub async fn get_available_by_split(
        &self,
        split_type: SplitType,
    ) -> Result<i64, ApiError> {
        let count = self.license_repo.count_unclaimed_by_split(split_type).await?;
        Ok(count)
    }
}
