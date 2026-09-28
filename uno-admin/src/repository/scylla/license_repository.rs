//! ScyllaDB implementation of LicenseRepositoryTrait

use async_trait::async_trait;
use chrono::Utc;
use scylla::frame::value::CqlTimestamp;
use scylla::Session;
use std::sync::Arc;
use tracing::{debug, error, info};
use uuid::Uuid;

use std::collections::HashMap;
use crate::db::DbPool;
use crate::models::entity::{LicenseEntity, NewLicense, NewLicenseFromApi};
use crate::repository::traits::{LicenseRepositoryTrait, MarketplaceData, SplitData};

// Core license row (14 columns) - ScyllaDB has a limit on tuple size
type LicenseRow = (String, String, String, Option<String>, Option<String>, String, String, f64, f64, f64, f64, bool, CqlTimestamp, CqlTimestamp);

// Extended fields row (separate query for additional fields)
type ExtendedFieldsRow = (Option<f64>, Option<f64>, Option<CqlTimestamp>);  // lease_share_percentage, lease_min_uptime_percentage, synced_at

/// ScyllaDB implementation of the License repository
pub struct LicenseRepository {
    session: Arc<Session>,
}

impl LicenseRepository {
    /// Create a new LicenseRepository with the given session
    pub fn new(session: DbPool) -> Self {
        Self { session }
    }

    fn timestamp_to_datetime(ts: CqlTimestamp) -> String {
        chrono::DateTime::from_timestamp_millis(ts.0)
            .map(|d| d.to_rfc3339())
            .unwrap_or_else(|| Utc::now().to_rfc3339())
    }

    fn optional_timestamp_to_datetime(ts: Option<CqlTimestamp>) -> Option<String> {
        ts.and_then(|t| {
            chrono::DateTime::from_timestamp_millis(t.0).map(|d| d.to_rfc3339())
        })
    }

    fn datetime_to_timestamp(dt: &str) -> Option<CqlTimestamp> {
        chrono::DateTime::parse_from_rfc3339(dt)
            .ok()
            .map(|d| CqlTimestamp(d.timestamp_millis()))
    }

    /// Get is_published status for a license
    async fn get_is_published(&self, id: &str) -> Result<bool, String> {
        let query = "SELECT is_published FROM licenses WHERE id = ?";
        let result = self
            .session
            .query_unpaged(query, (id,))
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = result.into_rows_result().map_err(|e| e.to_string())?;
        if let Some(row) = rows_result.rows::<(Option<bool>,)>().map_err(|e| e.to_string())?.next() {
            if let Ok((is_published,)) = row {
                return Ok(is_published.unwrap_or(false));
            }
        }
        Ok(false)
    }

    /// Get marketplace status fields for a license (status, referral_code)
    async fn get_marketplace_fields(&self, id: &str) -> Result<(Option<String>, Option<String>), String> {
        let query = "SELECT marketplace_status, marketplace_referral_code FROM licenses WHERE id = ?";
        let result = self
            .session
            .query_unpaged(query, (id,))
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = result.into_rows_result().map_err(|e| e.to_string())?;
        if let Some(row) = rows_result.rows::<(Option<String>, Option<String>)>().map_err(|e| e.to_string())?.next() {
            if let Ok((status, referral_code)) = row {
                return Ok((status, referral_code));
            }
        }
        Ok((None, None))
    }

    fn parse_single_license(result: scylla::QueryResult) -> Result<Option<LicenseEntity>, String> {
        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        let rows = match rows_result.rows::<LicenseRow>() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        for row_result in rows {
            if let Ok((id, license_id, node_id, lease_code, alias, agent_id, ulo_name, uno_share, agent_share, ulo_share, uptime, is_online, created_at, updated_at)) = row_result {
                return Ok(Some(LicenseEntity {
                    id,
                    license_id,
                    node_id,
                    lease_code,
                    alias,
                    agent_id,
                    ulo_name,
                    uno_share,
                    agent_share,
                    ulo_share,
                    uptime,
                    is_online,
                    created_at: Self::timestamp_to_datetime(created_at),
                    updated_at: Self::timestamp_to_datetime(updated_at),
                    // Extended fields - default to None/0.0 for basic queries
                    // These will be populated when fetching via extended query
                    owner_wallet_address: None,
                    device_id: None,
                    device_name: None,
                    activation_start_at: None,
                    activation_end_at: None,
                    activation_by: None,
                    activation_postponed_ms: None,
                    lease_user_id: None,
                    lease_share_percentage: 0.0,
                    lease_min_uptime_percentage: 0.0,
                    lease_from: None,
                    lease_to: None,
                    validation_last_success_at: None,
                    settings: None,
                    synced_at: None,
                    // Marketplace fields - default to false/None
                    is_on_marketplace: false,
                    is_on_uno_marketplace: false,
                    is_published: false,
                    marketplace_status: None,
                    marketplace_referral_code: None,
                }));
            }
        }

        Ok(None)
    }

    fn parse_licenses(result: scylla::QueryResult) -> Result<Vec<LicenseEntity>, String> {
        let mut licenses = Vec::new();

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(licenses),
        };

        let rows = match rows_result.rows::<LicenseRow>() {
            Ok(r) => r,
            Err(_) => return Ok(licenses),
        };

        for row_result in rows {
            if let Ok((id, license_id, node_id, lease_code, alias, agent_id, ulo_name, uno_share, agent_share, ulo_share, uptime, is_online, created_at, updated_at)) = row_result {
                licenses.push(LicenseEntity {
                    id,
                    license_id,
                    node_id,
                    lease_code,
                    alias,
                    agent_id,
                    ulo_name,
                    uno_share,
                    agent_share,
                    ulo_share,
                    uptime,
                    is_online,
                    created_at: Self::timestamp_to_datetime(created_at),
                    updated_at: Self::timestamp_to_datetime(updated_at),
                    // Extended fields - default to None/0.0 for basic queries
                    owner_wallet_address: None,
                    device_id: None,
                    device_name: None,
                    activation_start_at: None,
                    activation_end_at: None,
                    activation_by: None,
                    activation_postponed_ms: None,
                    lease_user_id: None,
                    lease_share_percentage: 0.0,
                    lease_min_uptime_percentage: 0.0,
                    lease_from: None,
                    lease_to: None,
                    validation_last_success_at: None,
                    settings: None,
                    synced_at: None,
                    // Marketplace fields - default to false/None
                    is_on_marketplace: false,
                    is_on_uno_marketplace: false,
                    is_published: false,
                    marketplace_status: None,
                    marketplace_referral_code: None,
                });
            }
        }

        Ok(licenses)
    }

    /// Fetch extended fields for a license and merge into the entity
    async fn get_extended_fields(&self, license_id: &str) -> Result<(f64, f64, Option<String>), String> {
        let query = "SELECT lease_share_percentage, lease_min_uptime_percentage, synced_at FROM licenses WHERE license_id = ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (license_id,))
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok((0.0, 0.0, None)),
        };

        let rows = match rows_result.rows::<ExtendedFieldsRow>() {
            Ok(r) => r,
            Err(_) => return Ok((0.0, 0.0, None)),
        };

        for row_result in rows {
            if let Ok((lease_share, lease_min_uptime, synced_at)) = row_result {
                return Ok((
                    lease_share.unwrap_or(0.0),
                    lease_min_uptime.unwrap_or(0.0),
                    Self::optional_timestamp_to_datetime(synced_at),
                ));
            }
        }

        Ok((0.0, 0.0, None))
    }
}

#[async_trait]
impl LicenseRepositoryTrait for LicenseRepository {
    async fn save_license(&self, new_license: NewLicense) -> Result<LicenseEntity, String> {
        let now = Utc::now();
        let now_ts = CqlTimestamp(now.timestamp_millis());
        let id = Uuid::new_v4().to_string();

        let query = "INSERT INTO licenses (id, license_id, node_id, lease_code, alias, agent_id, ulo_name, uno_share, agent_share, ulo_share, uptime, is_online, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        self.session
            .query_unpaged(
                query,
                (
                    &id,
                    &new_license.license_id,
                    &new_license.node_id,
                    &new_license.lease_code,
                    &new_license.alias,
                    &new_license.agent_id,
                    &new_license.ulo_name,
                    new_license.uno_share,
                    new_license.agent_share,
                    new_license.ulo_share,
                    0.0f64,
                    false,
                    true,
                    now_ts,
                    now_ts,
                ),
            )
            .await
            .map_err(|e| {
                error!("Failed to save license: {}", e);
                e.to_string()
            })?;

        debug!("Saved license: {}", id);

        Ok(LicenseEntity {
            id,
            license_id: new_license.license_id,
            node_id: new_license.node_id,
            lease_code: new_license.lease_code,
            alias: new_license.alias,
            agent_id: new_license.agent_id,
            ulo_name: new_license.ulo_name,
            uno_share: new_license.uno_share,
            agent_share: new_license.agent_share,
            ulo_share: new_license.ulo_share,
            uptime: 0.0,
            is_online: false,
            created_at: now.to_rfc3339(),
            updated_at: now.to_rfc3339(),
            // Extended fields default to None/0.0
            owner_wallet_address: None,
            device_id: None,
            device_name: None,
            activation_start_at: None,
            activation_end_at: None,
            activation_by: None,
            activation_postponed_ms: None,
            lease_user_id: None,
            lease_share_percentage: 0.0,
            lease_min_uptime_percentage: 0.0,
            lease_from: None,
            lease_to: None,
            validation_last_success_at: None,
            settings: None,
            synced_at: None,
            // Marketplace fields default to false/None
            is_on_marketplace: false,
            is_on_uno_marketplace: false,
            is_published: false,
            marketplace_status: None,
            marketplace_referral_code: None,
        })
    }

    async fn get_license_by_id(&self, id: &str) -> Result<Option<LicenseEntity>, String> {
        let query = "SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name, uno_share, agent_share, ulo_share, uptime, is_online, created_at, updated_at FROM licenses WHERE id = ?";

        let result = self
            .session
            .query_unpaged(query, (id,))
            .await
            .map_err(|e| e.to_string())?;

        let mut license = Self::parse_single_license(result)?;

        // Fetch extended fields separately
        if let Some(ref mut lic) = license {
            let (lease_share, lease_min_uptime, synced_at) = self.get_extended_fields(&lic.license_id).await?;
            lic.lease_share_percentage = lease_share;
            lic.lease_min_uptime_percentage = lease_min_uptime;
            lic.synced_at = synced_at;
        }

        Ok(license)
    }

    async fn get_license_by_license_id(&self, license_id: &str) -> Result<Option<LicenseEntity>, String> {
        let query = "SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name, uno_share, agent_share, ulo_share, uptime, is_online, created_at, updated_at FROM licenses WHERE license_id = ? ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (license_id,))
            .await
            .map_err(|e| e.to_string())?;

        let mut license = Self::parse_single_license(result)?;

        // Fetch extended fields separately
        if let Some(ref mut lic) = license {
            let (lease_share, lease_min_uptime, synced_at) = self.get_extended_fields(license_id).await?;
            lic.lease_share_percentage = lease_share;
            lic.lease_min_uptime_percentage = lease_min_uptime;
            lic.synced_at = synced_at;
        }

        Ok(license)
    }

    async fn get_licenses_by_agent_id(&self, agent_id: &str) -> Result<Vec<LicenseEntity>, String> {
        let query = "SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name, uno_share, agent_share, ulo_share, uptime, is_online, created_at, updated_at FROM licenses WHERE agent_id = ? AND is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (agent_id,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_licenses(result)
    }

    async fn get_licenses_by_node_id(&self, node_id: &str) -> Result<Vec<LicenseEntity>, String> {
        let query = "SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name, uno_share, agent_share, ulo_share, uptime, is_online, created_at, updated_at FROM licenses WHERE node_id = ? AND is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, (node_id,))
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_licenses(result)
    }

    async fn list_licenses(&self) -> Result<Vec<LicenseEntity>, String> {
        let query = "SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name, uno_share, agent_share, ulo_share, uptime, is_online, created_at, updated_at FROM licenses WHERE is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        Self::parse_licenses(result)
    }

    async fn bulk_save_licenses(&self, licenses: Vec<NewLicense>) -> Result<usize, String> {
        let mut count = 0;

        for license in licenses {
            match self.save_license(license).await {
                Ok(_) => count += 1,
                Err(e) => {
                    error!("Failed to save license in bulk: {}", e);
                }
            }
        }

        Ok(count)
    }

    async fn update_license(&self, license: LicenseEntity) -> Result<LicenseEntity, String> {
        let now = Utc::now();
        let now_ts = CqlTimestamp(now.timestamp_millis());

        let query = "UPDATE licenses SET license_id = ?, node_id = ?, lease_code = ?, alias = ?, agent_id = ?, ulo_name = ?, uno_share = ?, agent_share = ?, ulo_share = ?, uptime = ?, is_online = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(
                query,
                (
                    &license.license_id,
                    &license.node_id,
                    &license.lease_code,
                    &license.alias,
                    &license.agent_id,
                    &license.ulo_name,
                    license.uno_share,
                    license.agent_share,
                    license.ulo_share,
                    license.uptime,
                    license.is_online,
                    now_ts,
                    &license.id,
                ),
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(LicenseEntity {
            updated_at: now.to_rfc3339(),
            ..license
        })
    }

    async fn delete_license(&self, id: &str) -> Result<bool, String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        let query = "UPDATE licenses SET is_active = false, deleted_at = ? WHERE id = ?";

        self.session
            .query_unpaged(query, (now_ts, id))
            .await
            .map_err(|e| e.to_string())?;

        Ok(true)
    }

    async fn count_licenses(&self) -> Result<i64, String> {
        let query = "SELECT COUNT(*) FROM licenses WHERE is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        let rows = match rows_result.rows::<(i64,)>() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        for row_result in rows {
            if let Ok((count,)) = row_result {
                return Ok(count);
            }
        }

        Ok(0)
    }

    async fn count_online_licenses(&self) -> Result<i64, String> {
        let query = "SELECT COUNT(*) FROM licenses WHERE is_active = true AND is_online = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        let rows = match rows_result.rows::<(i64,)>() {
            Ok(r) => r,
            Err(_) => return Ok(0),
        };

        for row_result in rows {
            if let Ok((count,)) = row_result {
                return Ok(count);
            }
        }

        Ok(0)
    }

    async fn get_last_synced(&self) -> Result<Option<String>, String> {
        // Get the most recent synced_at timestamp from all licenses
        let query = "SELECT synced_at FROM licenses WHERE is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        let rows = match rows_result.rows::<(Option<CqlTimestamp>,)>() {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };

        let mut max_synced: Option<CqlTimestamp> = None;

        for row_result in rows {
            if let Ok((synced_at,)) = row_result {
                if let Some(ts) = synced_at {
                    max_synced = Some(match max_synced {
                        Some(existing) if existing.0 > ts.0 => existing,
                        _ => ts,
                    });
                }
            }
        }

        Ok(Self::optional_timestamp_to_datetime(max_synced))
    }

    async fn upsert_license_from_api(&self, license: NewLicenseFromApi) -> Result<LicenseEntity, String> {
        let now = Utc::now();
        let now_ts = CqlTimestamp(now.timestamp_millis());

        // Check if license already exists
        let existing = self.get_license_by_license_id(&license.license_id).await?;

        let id = existing.as_ref().map(|e| e.id.clone()).unwrap_or_else(|| Uuid::new_v4().to_string());
        let agent_id = existing.as_ref().map(|e| e.agent_id.clone()).unwrap_or_else(|| "unknown".to_string());
        let ulo_name = existing.as_ref().map(|e| e.ulo_name.clone()).unwrap_or_else(|| "unknown".to_string());
        let uno_share = existing.as_ref().map(|e| e.uno_share).unwrap_or(47.0);
        let agent_share = existing.as_ref().map(|e| e.agent_share).unwrap_or(3.0);
        let ulo_share = existing.as_ref().map(|e| e.ulo_share).unwrap_or(50.0);
        let created_at = existing.as_ref().and_then(|e| Self::datetime_to_timestamp(&e.created_at)).unwrap_or(now_ts);

        // Step 1: Insert/update core fields (15 params max for ScyllaDB tuple limit)
        let core_query = r#"
            INSERT INTO licenses (
                id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                uno_share, agent_share, ulo_share, uptime, is_online, is_active,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        self.session
            .query_unpaged(
                core_query,
                (
                    &id,
                    &license.license_id,
                    &license.node_id,
                    &None::<String>, // lease_code
                    &license.alias,
                    &agent_id,
                    &ulo_name,
                    uno_share,
                    agent_share,
                    ulo_share,
                    license.uptime,
                    license.is_online,
                    true,
                    created_at,
                    now_ts,
                ),
            )
            .await
            .map_err(|e| {
                error!("Failed to insert core license fields: {}", e);
                e.to_string()
            })?;

        // Step 2: Update extended fields in separate queries to avoid tuple limit
        // Update API-specific fields
        let ext_query1 = r#"
            UPDATE licenses SET
                owner_wallet_address = ?,
                device_id = ?,
                device_name = ?,
                activation_by = ?,
                activation_postponed_ms = ?,
                lease_user_id = ?,
                settings = ?,
                synced_at = ?
            WHERE id = ?
        "#;

        self.session
            .query_unpaged(
                ext_query1,
                (
                    &license.owner_wallet_address,
                    &license.device_id,
                    &license.device_name,
                    &license.activation_by,
                    license.activation_postponed_ms,
                    &license.lease_user_id,
                    &license.settings,
                    now_ts,
                    &id,
                ),
            )
            .await
            .map_err(|e| {
                error!("Failed to update extended fields (part 1): {}", e);
                e.to_string()
            })?;

        // Update timestamp and numeric fields
        let activation_start_at = license.activation_start_at.as_ref().and_then(|d| Self::datetime_to_timestamp(d));
        let activation_end_at = license.activation_end_at.as_ref().and_then(|d| Self::datetime_to_timestamp(d));
        let lease_from = license.lease_from.as_ref().and_then(|d| Self::datetime_to_timestamp(d));
        let lease_to = license.lease_to.as_ref().and_then(|d| Self::datetime_to_timestamp(d));
        let validation_last_success_at = license.validation_last_success_at.as_ref().and_then(|d| Self::datetime_to_timestamp(d));

        let ext_query2 = r#"
            UPDATE licenses SET
                activation_start_at = ?,
                activation_end_at = ?,
                lease_share_percentage = ?,
                lease_min_uptime_percentage = ?,
                lease_from = ?,
                lease_to = ?,
                validation_last_success_at = ?
            WHERE id = ?
        "#;

        self.session
            .query_unpaged(
                ext_query2,
                (
                    activation_start_at,
                    activation_end_at,
                    license.lease_share_percentage,
                    license.lease_min_uptime_percentage,
                    lease_from,
                    lease_to,
                    validation_last_success_at,
                    &id,
                ),
            )
            .await
            .map_err(|e| {
                error!("Failed to update extended fields (part 2): {}", e);
                e.to_string()
            })?;

        debug!("Upserted license from API: {}", license.license_id);

        // Preserve marketplace fields from existing license if they exist
        let is_on_marketplace = existing.as_ref().map(|e| e.is_on_marketplace).unwrap_or(false);
        let is_on_uno_marketplace = existing.as_ref().map(|e| e.is_on_uno_marketplace).unwrap_or(false);
        let is_published = existing.as_ref().map(|e| e.is_published).unwrap_or(false);
        let marketplace_status = existing.as_ref().and_then(|e| e.marketplace_status.clone());
        let marketplace_referral_code = existing.as_ref().and_then(|e| e.marketplace_referral_code.clone());

        Ok(LicenseEntity {
            id,
            license_id: license.license_id,
            node_id: license.node_id,
            lease_code: None,
            alias: license.alias,
            agent_id,
            ulo_name,
            uno_share,
            agent_share,
            ulo_share,
            uptime: license.uptime,
            is_online: license.is_online,
            created_at: Self::timestamp_to_datetime(created_at),
            updated_at: now.to_rfc3339(),
            owner_wallet_address: license.owner_wallet_address,
            device_id: license.device_id,
            device_name: license.device_name,
            activation_start_at: license.activation_start_at,
            activation_end_at: license.activation_end_at,
            activation_by: license.activation_by,
            activation_postponed_ms: license.activation_postponed_ms,
            lease_user_id: license.lease_user_id,
            lease_share_percentage: license.lease_share_percentage,
            lease_min_uptime_percentage: license.lease_min_uptime_percentage,
            lease_from: license.lease_from,
            lease_to: license.lease_to,
            validation_last_success_at: license.validation_last_success_at,
            settings: license.settings,
            synced_at: Some(now.to_rfc3339()),
            // Preserve marketplace fields from existing license
            is_on_marketplace,
            is_on_uno_marketplace,
            is_published,
            marketplace_status,
            marketplace_referral_code,
        })
    }

    async fn upsert_licenses_from_api(&self, licenses: Vec<NewLicenseFromApi>) -> Result<usize, String> {
        let mut count = 0;

        for license in licenses {
            match self.upsert_license_from_api(license).await {
                Ok(_) => count += 1,
                Err(e) => {
                    error!("Failed to upsert license in batch: {}", e);
                }
            }
        }

        info!("Upserted {} licenses from API", count);
        Ok(count)
    }

    async fn get_marketplace_licenses(&self) -> Result<Vec<LicenseEntity>, String> {
        // Query licenses where is_on_uno_marketplace = true
        let query = "SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name, uno_share, agent_share, ulo_share, uptime, is_online, created_at, updated_at FROM licenses WHERE is_on_uno_marketplace = true AND is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        // Parse base licenses
        let mut licenses = Self::parse_licenses(result)?;

        // Set marketplace flags and fetch is_published + marketplace status for each license
        for license in &mut licenses {
            license.is_on_uno_marketplace = true;
            // Fetch is_published status
            if let Ok(published) = self.get_is_published(&license.id).await {
                license.is_published = published;
            }
            // Fetch marketplace status fields
            if let Ok((status, referral_code)) = self.get_marketplace_fields(&license.id).await {
                license.marketplace_status = status;
                license.marketplace_referral_code = referral_code;
            }
        }

        Ok(licenses)
    }

    async fn update_marketplace_status(
        &self,
        license_id: &str,
        is_on_marketplace: bool,
        status: Option<&str>,
        referral_code: Option<&str>,
    ) -> Result<(), String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        // First get the license to get its id
        let license = self.get_license_by_license_id(license_id).await?
            .ok_or_else(|| format!("License not found: {}", license_id))?;

        let query = "UPDATE licenses SET is_on_marketplace = ?, marketplace_status = ?, marketplace_referral_code = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(
                query,
                (
                    is_on_marketplace,
                    status,
                    referral_code,
                    now_ts,
                    &license.id,
                ),
            )
            .await
            .map_err(|e| {
                error!("Failed to update marketplace status for {}: {}", license_id, e);
                e.to_string()
            })?;

        debug!("Updated marketplace status for {}: on_marketplace={}, status={:?}", license_id, is_on_marketplace, status);
        Ok(())
    }

    async fn publish_to_marketplace(&self, license_ids: Vec<String>) -> Result<usize, String> {
        let mut count = 0;

        for license_id in license_ids {
            match self.update_marketplace_status(&license_id, true, Some("unclaimed"), None).await {
                Ok(_) => count += 1,
                Err(e) => {
                    error!("Failed to publish license {}: {}", license_id, e);
                }
            }
        }

        info!("Published {} licenses to marketplace", count);
        Ok(count)
    }

    async fn unpublish_from_marketplace(&self, license_ids: Vec<String>) -> Result<usize, String> {
        let mut count = 0;

        for license_id in license_ids {
            match self.update_marketplace_status(&license_id, false, None, None).await {
                Ok(_) => count += 1,
                Err(e) => {
                    error!("Failed to unpublish license {}: {}", license_id, e);
                }
            }
        }

        info!("Unpublished {} licenses from marketplace", count);
        Ok(count)
    }

    async fn update_marketplace_status_extended(
        &self,
        license_id: &str,
        is_on_marketplace: bool,
        is_on_uno_marketplace: bool,
        status: Option<&str>,
        referral_code: Option<&str>,
    ) -> Result<(), String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        // First get the license to get its id
        let license = self.get_license_by_license_id(license_id).await?
            .ok_or_else(|| format!("License not found: {}", license_id))?;

        let query = "UPDATE licenses SET is_on_marketplace = ?, is_on_uno_marketplace = ?, marketplace_status = ?, marketplace_referral_code = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(
                query,
                (
                    is_on_marketplace,
                    is_on_uno_marketplace,
                    status,
                    referral_code,
                    now_ts,
                    &license.id,
                ),
            )
            .await
            .map_err(|e| {
                error!("Failed to update marketplace status extended for {}: {}", license_id, e);
                e.to_string()
            })?;

        debug!(
            "Updated marketplace status extended for {}: on_marketplace={}, on_uno_marketplace={}, status={:?}",
            license_id, is_on_marketplace, is_on_uno_marketplace, status
        );
        Ok(())
    }

    async fn set_is_published(&self, license_id: &str, is_published: bool) -> Result<(), String> {
        let now_ts = CqlTimestamp(Utc::now().timestamp_millis());

        // First get the license to get its id
        let license = self.get_license_by_license_id(license_id).await?
            .ok_or_else(|| format!("License not found: {}", license_id))?;

        let query = "UPDATE licenses SET is_published = ?, updated_at = ? WHERE id = ?";

        self.session
            .query_unpaged(query, (is_published, now_ts, &license.id))
            .await
            .map_err(|e| {
                error!("Failed to update is_published for {}: {}", license_id, e);
                e.to_string()
            })?;

        info!("Set is_published={} for license {}", is_published, license_id);
        Ok(())
    }

    async fn get_marketplace_data_batch(&self, license_ids: &[String]) -> Result<HashMap<String, MarketplaceData>, String> {
        let mut result_map = HashMap::new();

        if license_ids.is_empty() {
            return Ok(result_map);
        }

        // Query all licenses and filter in memory (ScyllaDB doesn't support IN queries well for non-partition keys)
        let query = "SELECT license_id, is_on_marketplace, is_on_uno_marketplace FROM licenses WHERE is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(result_map),
        };

        let rows = match rows_result.rows::<(String, Option<bool>, Option<bool>)>() {
            Ok(r) => r,
            Err(_) => return Ok(result_map),
        };

        // Create a set for quick lookup
        let license_id_set: std::collections::HashSet<&String> = license_ids.iter().collect();

        for row_result in rows {
            if let Ok((license_id, is_on_marketplace, is_on_uno_marketplace)) = row_result {
                if license_id_set.contains(&license_id) {
                    result_map.insert(license_id, MarketplaceData {
                        is_on_marketplace: is_on_marketplace.unwrap_or(false),
                        is_on_uno_marketplace: is_on_uno_marketplace.unwrap_or(false),
                    });
                }
            }
        }

        Ok(result_map)
    }

    async fn get_split_data_batch(&self, license_ids: &[String]) -> Result<HashMap<String, SplitData>, String> {
        let mut result_map = HashMap::new();

        if license_ids.is_empty() {
            return Ok(result_map);
        }

        // Query all licenses and filter in memory (ScyllaDB doesn't support IN queries well for non-partition keys)
        let query = "SELECT license_id, uno_share, ulo_share, agent_share FROM licenses WHERE is_active = true ALLOW FILTERING";

        let result = self
            .session
            .query_unpaged(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        let rows_result = match result.into_rows_result() {
            Ok(r) => r,
            Err(_) => return Ok(result_map),
        };

        let rows = match rows_result.rows::<(String, Option<f64>, Option<f64>, Option<f64>)>() {
            Ok(r) => r,
            Err(_) => return Ok(result_map),
        };

        // Create a set for quick lookup
        let license_id_set: std::collections::HashSet<&String> = license_ids.iter().collect();

        for row_result in rows {
            if let Ok((license_id, uno_share, ulo_share, agent_share)) = row_result {
                if license_id_set.contains(&license_id) {
                    result_map.insert(license_id, SplitData {
                        uno_share: uno_share.unwrap_or(47.0),
                        ulo_share: ulo_share.unwrap_or(50.0),
                        agent_share: agent_share.unwrap_or(3.0),
                    });
                }
            }
        }

        Ok(result_map)
    }
}
