//! Marketplace service for license distribution to uno-app.
//!
//! Handles publishing licenses to marketplace and polling for claimed licenses.

use chrono::{Utc, Duration};
use tracing::{debug, error, info, warn};
use uno_api::models::{LicenseInput, PublishLicensesRequest, SplitType};

use crate::api::uno_client::{UnoLicenseClient, ClaimedLicenseDto};
use crate::db::DbPool;
use crate::repository::traits::LicenseRepositoryTrait;

use crate::repository::postgres::PgLicenseRepository as LicenseRepository;

/// Service for marketplace operations.
pub struct MarketplaceService {
    license_repo: LicenseRepository,
    uno_client: UnoLicenseClient,
}

impl MarketplaceService {
    /// Create a new MarketplaceService.
    pub fn new(pool: DbPool) -> Result<Self, String> {
        Ok(Self {
            license_repo: LicenseRepository::new(pool),
            uno_client: UnoLicenseClient::from_env()?,
        })
    }

    /// Create with explicit client configuration.
    pub fn with_client(pool: DbPool, uno_client: UnoLicenseClient) -> Self {
        Self {
            license_repo: LicenseRepository::new(pool),
            uno_client,
        }
    }

    /// Publish selected licenses to uno-app marketplace.
    pub async fn publish_to_marketplace(&self, license_ids: Vec<String>) -> Result<PublishResult, String> {
        if license_ids.is_empty() {
            return Ok(PublishResult {
                published: 0,
                failed: 0,
                errors: vec![],
            });
        }

        info!("Publishing {} licenses to marketplace", license_ids.len());

        // 1. Fetch license details from database
        let mut license_inputs = Vec::new();
        let mut errors = Vec::new();
        let mut successful_ids = Vec::new();

        for license_id in &license_ids {
            match self.license_repo.get_license_by_license_id(license_id).await {
                Ok(Some(license)) => {
                    // R3-06: Lease codes are required - no fabrication
                    let lease_code = match &license.lease_code {
                        Some(code) if !code.is_empty() => code.clone(),
                        _ => {
                            // Missing lease code is an error, not auto-generated
                            warn!("R5-06: License {} missing required lease_code - quarantining", license_id);
                            errors.push(format!("License {} missing required lease_code (quarantined)", license_id));
                            continue;
                        }
                    };

                    // R5-06: Use truthful validity dates from source, not fabricated
                    let valid_from = match &license.lease_from {
                        Some(date_str) if !date_str.is_empty() => {
                            match chrono::DateTime::parse_from_rfc3339(date_str) {
                                Ok(dt) => dt.with_timezone(&Utc),
                                Err(_) => {
                                    warn!("R5-06: License {} has invalid lease_from date '{}' - quarantining", license_id, date_str);
                                    errors.push(format!("License {} has invalid lease_from date (quarantined)", license_id));
                                    continue;
                                }
                            }
                        }
                        _ => {
                            // R5-06: Missing validity dates should quarantine, not fabricate
                            warn!("R5-06: License {} missing lease_from date - quarantining", license_id);
                            errors.push(format!("License {} missing validity dates (quarantined)", license_id));
                            continue;
                        }
                    };

                    let valid_to = match &license.lease_to {
                        Some(date_str) if !date_str.is_empty() => {
                            match chrono::DateTime::parse_from_rfc3339(date_str) {
                                Ok(dt) => dt.with_timezone(&Utc),
                                Err(_) => {
                                    warn!("R5-06: License {} has invalid lease_to date '{}' - quarantining", license_id, date_str);
                                    errors.push(format!("License {} has invalid lease_to date (quarantined)", license_id));
                                    continue;
                                }
                            }
                        }
                        _ => {
                            warn!("R5-06: License {} missing lease_to date - quarantining", license_id);
                            errors.push(format!("License {} missing validity dates (quarantined)", license_id));
                            continue;
                        }
                    };

                    // R5-06: Determine categorical split type from exact shares
                    // Use closest match, but preserve exact percentages
                    let split_type = if license.uno_share >= 57.5 {
                        SplitType::Split6040
                    } else if license.uno_share >= 52.5 {
                        SplitType::Split5545
                    } else {
                        SplitType::Split5050
                    };

                    // R5-06: Build input with exact shares and provenance
                    let input = LicenseInput::new(lease_code, valid_from, valid_to, split_type)
                        .with_id(license.license_id.clone())
                        .with_exact_shares(license.uno_share, license.ulo_share, license.agent_share)
                        .with_provenance(
                            "unetwork",
                            license.synced_at.clone(),
                            Some(license.license_id.clone()),
                        );

                    license_inputs.push(input);
                    successful_ids.push(license_id.clone());
                }
                Ok(None) => {
                    warn!("License not found: {}", license_id);
                    errors.push(format!("License not found: {}", license_id));
                }
                Err(e) => {
                    error!("Failed to fetch license {}: {}", license_id, e);
                    errors.push(format!("Failed to fetch {}: {}", license_id, e));
                }
            }
        }

        if license_inputs.is_empty() {
            return Ok(PublishResult {
                published: 0,
                failed: license_ids.len() as i32,
                errors,
            });
        }

        // 2. Call uno-app to publish licenses
        let request = PublishLicensesRequest::new(license_inputs);
        match self.uno_client.publish_licenses(request).await {
            Ok(result) => {
                info!("Published {} licenses to uno-app (failed: {})", result.created, result.failed);

                // 3. Update is_published = true for successful licenses
                for id in &successful_ids {
                    if let Err(e) = self.license_repo.set_is_published(id, true).await {
                        warn!("Failed to update is_published for {}: {}", id, e);
                    }
                }

                // Collect errors from publish result
                for err in result.errors {
                    errors.push(err.message);
                }

                Ok(PublishResult {
                    published: result.created as i32,
                    failed: result.failed as i32,
                    errors,
                })
            }
            Err(e) => {
                error!("Failed to publish to uno-app: {}", e);
                Err(format!("Failed to publish to uno-app: {}", e))
            }
        }
    }

    /// Poll uno-app for claimed licenses and update local records.
    pub async fn poll_claimed_licenses(&self, since: Option<&str>) -> Result<PollResult, String> {
        info!("Polling uno-app for claimed licenses (since: {:?})", since);

        // Fetch claimed licenses from uno-app
        let response = self.uno_client
            .get_claimed_licenses(since, Some(100))
            .await
            .map_err(|e| {
                error!("Failed to fetch claimed licenses: {}", e);
                e.to_string()
            })?;

        info!("Received {} claimed licenses from uno-app", response.licenses.len());

        // Update local records
        let updated = self.update_claim_status(&response.licenses).await?;

        Ok(PollResult {
            fetched: response.licenses.len(),
            updated,
        })
    }

    /// Update local license records with claim status.
    async fn update_claim_status(&self, claimed: &[ClaimedLicenseDto]) -> Result<usize, String> {
        let mut updated = 0;

        for claim in claimed {
            match self.license_repo.update_marketplace_status(
                &claim.license_id,
                true, // still on marketplace
                Some("claimed"),
                claim.referral_code.as_deref(),
            ).await {
                Ok(_) => {
                    updated += 1;
                    debug!("Updated claim status for license {}", claim.license_id);
                }
                Err(e) => {
                    warn!("Failed to update claim status for {}: {}", claim.license_id, e);
                }
            }
        }

        info!("Updated {} license claim statuses", updated);
        Ok(updated)
    }

    /// Full sync: poll and update.
    pub async fn poll_and_sync(&self) -> Result<PollResult, String> {
        // For now, we poll all claimed licenses
        // In the future, we could track last sync time for incremental updates
        self.poll_claimed_licenses(None).await
    }

    /// Get marketplace licenses (from local DB only).
    pub async fn get_marketplace_licenses(&self) -> Result<Vec<crate::models::entity::LicenseEntity>, String> {
        self.license_repo.get_marketplace_licenses().await
    }

    /// Normalize license ID for matching.
    ///
    /// R5-06: Use full license IDs, not substring-derived UUIDs.
    /// Both uno-admin and uno-app now store the full license ID.
    /// Normalization handles case differences for reliable matching.
    fn normalize_license_id(license_id: &str) -> String {
        license_id.to_lowercase()
    }

    /// Sync claim statuses from uno-app and return enriched licenses.
    ///
    /// Called on every table load for fresh data:
    /// 1. Get local marketplace licenses
    /// 2. Fetch claimed statuses from uno-app
    /// 3. Update local DB with fresh statuses
    /// 4. Return enriched licenses
    pub async fn sync_and_get_licenses(&self) -> Result<Vec<crate::models::entity::LicenseEntity>, String> {
        use std::collections::HashMap;

        // 1. Get local marketplace licenses
        let mut licenses = self.license_repo.get_marketplace_licenses().await?;
        info!("Syncing {} marketplace licenses from uno-app", licenses.len());

        // 2. Fetch claimed statuses from uno-app (fetch all, up to 1000)
        let claimed_response = match self.uno_client.get_claimed_licenses(None, Some(1000)).await {
            Ok(response) => {
                info!("Fetched {} claimed licenses from uno-app", response.licenses.len());
                // Log claimed license IDs for debugging
                for claimed in &response.licenses {
                    debug!("Claimed license from uno-app: {}", claimed.license_id);
                }
                response
            }
            Err(e) => {
                // If uno-app is unavailable, return licenses with current local status
                warn!("Failed to fetch claimed statuses from uno-app: {}. Using local data.", e);
                return Ok(licenses);
            }
        };

        // 3. Build lookup map: normalized license_id -> (status, referral_code)
        // R5-06: Use full license IDs, not substring-derived UUIDs
        let claimed_map: HashMap<String, (String, Option<String>)> = claimed_response.licenses
            .into_iter()
            .map(|c| (Self::normalize_license_id(&c.license_id), ("claimed".to_string(), c.referral_code)))
            .collect();

        // 4. Update each license status and persist to DB
        for license in &mut licenses {
            // R5-06: Use normalized full ID for matching (no truncation)
            let normalized_id = Self::normalize_license_id(&license.license_id);

            let is_claimed = claimed_map.get(&normalized_id).is_some();

            if is_claimed {
                let referral = claimed_map.get(&normalized_id)
                    .and_then(|(_, r)| r.clone());

                // License is claimed
                if license.marketplace_status.as_deref() != Some("claimed") {
                    // Status changed - update local DB
                    if let Err(e) = self.license_repo.update_marketplace_status(
                        &license.license_id,
                        true,
                        Some("claimed"),
                        referral.as_deref(),
                    ).await {
                        warn!("Failed to persist claimed status for {}: {}", license.license_id, e);
                    }
                }
                license.marketplace_status = Some("claimed".to_string());
                license.marketplace_referral_code = referral;
                debug!("R5-06: Matched claimed license: {}", license.license_id);
            } else {
                // Not in claimed list - set to unclaimed if currently not set
                if license.marketplace_status.is_none() {
                    license.marketplace_status = Some("unclaimed".to_string());
                }
            }
        }

        info!("Sync complete: {} licenses enriched", licenses.len());
        Ok(licenses)
    }

    /// Unpublish licenses from marketplace.
    pub async fn unpublish_from_marketplace(&self, license_ids: Vec<String>) -> Result<usize, String> {
        if license_ids.is_empty() {
            return Ok(0);
        }

        info!("Unpublishing {} licenses from marketplace", license_ids.len());
        self.license_repo.unpublish_from_marketplace(license_ids).await
    }
}

/// Result of publishing licenses.
#[derive(Debug, Clone)]
pub struct PublishResult {
    pub published: i32,
    pub failed: i32,
    pub errors: Vec<String>,
}

/// Result of polling claimed licenses.
#[derive(Debug, Clone)]
pub struct PollResult {
    pub fetched: usize,
    pub updated: usize,
}
