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
                            warn!("License {} missing required lease_code", license_id);
                            errors.push(format!("License {} missing required lease_code", license_id));
                            continue;
                        }
                    };

                    // Create LicenseInput for uno-app
                    let valid_from = Utc::now();
                    let valid_to = valid_from + Duration::days(365); // 1 year validity

                    // Determine split type based on uno_share
                    let split_type = if license.uno_share >= 60.0 {
                        SplitType::Split6040
                    } else if license.uno_share >= 55.0 {
                        SplitType::Split5545
                    } else {
                        SplitType::Split5050
                    };

                    let input = LicenseInput::new(lease_code, valid_from, valid_to, split_type)
                        .with_id(license.license_id.clone());

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

    /// Convert a hex license_id (0x...) to UUID format for matching with uno-app.
    ///
    /// uno-app stores licenses using a UUID derived from the first 32 hex chars of the license_id.
    /// Example: "0x0111e1758d35de5306c4feec2e87db6fcf593d055b22a32a4d49e1c1d1cb9281"
    ///       -> "0111e175-8d35-de53-06c4-feec2e87db6f"
    fn hex_to_uuid(hex_license_id: &str) -> Option<String> {
        // Strip "0x" prefix if present
        let hex_str = hex_license_id.strip_prefix("0x").unwrap_or(hex_license_id);

        // Need at least 32 hex chars to form a UUID
        if hex_str.len() >= 32 {
            let uuid_hex = &hex_str[..32];
            // Format as UUID: 8-4-4-4-12
            Some(format!(
                "{}-{}-{}-{}-{}",
                &uuid_hex[0..8],
                &uuid_hex[8..12],
                &uuid_hex[12..16],
                &uuid_hex[16..20],
                &uuid_hex[20..32]
            ))
        } else {
            None
        }
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

        // 3. Build lookup map: uuid_license_id -> (status, referral_code)
        // uno-app returns license_id in UUID format
        let claimed_map: HashMap<String, (String, Option<String>)> = claimed_response.licenses
            .into_iter()
            .map(|c| (c.license_id.to_lowercase(), ("claimed".to_string(), c.referral_code)))
            .collect();

        // 4. Update each license status and persist to DB
        for license in &mut licenses {
            // Convert hex license_id to UUID format for matching
            let uuid_key = Self::hex_to_uuid(&license.license_id)
                .map(|u| u.to_lowercase());

            let is_claimed = uuid_key
                .as_ref()
                .and_then(|key| claimed_map.get(key))
                .is_some();

            if is_claimed {
                let referral = uuid_key
                    .as_ref()
                    .and_then(|key| claimed_map.get(key))
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
                debug!("Matched claimed license: {} -> {:?}", license.license_id, uuid_key);
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
