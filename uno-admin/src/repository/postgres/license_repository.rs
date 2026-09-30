//! PostgreSQL implementation of LicenseRepository

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::entity::{LicenseEntity, NewLicense, NewLicenseFromApi};
use crate::repository::traits::{LicenseRepositoryTrait, MarketplaceData, SplitData};

/// PostgreSQL license repository implementation
pub struct LicenseRepository {
    pool: Arc<PgPool>,
}

impl LicenseRepository {
    /// Create a new license repository with the given connection pool
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Convert DateTime<Utc> to RFC3339 string
    fn datetime_to_string(dt: Option<DateTime<Utc>>) -> Option<String> {
        dt.map(|d| d.to_rfc3339())
    }

    /// Get current time as RFC3339 string
    fn now_string() -> String {
        Utc::now().to_rfc3339()
    }
}

/// Row type for sqlx mapping
#[derive(Debug, sqlx::FromRow)]
struct LicenseRow {
    id: String,
    license_id: String,
    node_id: String,
    lease_code: Option<String>,
    alias: Option<String>,
    agent_id: Option<String>,
    ulo_name: Option<String>,
    uno_share: Option<f64>,
    agent_share: Option<f64>,
    ulo_share: Option<f64>,
    uptime: Option<f64>,
    is_online: Option<bool>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    owner_wallet_address: Option<String>,
    device_id: Option<String>,
    device_name: Option<String>,
    activation_start_at: Option<DateTime<Utc>>,
    activation_end_at: Option<DateTime<Utc>>,
    activation_by: Option<String>,
    activation_postponed_ms: Option<i64>,
    lease_user_id: Option<String>,
    lease_share_percentage: Option<f64>,
    lease_min_uptime_percentage: Option<f64>,
    lease_from: Option<DateTime<Utc>>,
    lease_to: Option<DateTime<Utc>>,
    validation_last_success_at: Option<DateTime<Utc>>,
    settings: Option<String>,
    synced_at: Option<DateTime<Utc>>,
    is_on_marketplace: Option<bool>,
    is_on_uno_marketplace: Option<bool>,
    is_published: Option<bool>,
    marketplace_status: Option<String>,
    marketplace_referral_code: Option<String>,
}

impl From<LicenseRow> for LicenseEntity {
    fn from(row: LicenseRow) -> Self {
        Self {
            id: row.id,
            license_id: row.license_id,
            node_id: row.node_id,
            lease_code: row.lease_code,
            alias: row.alias,
            agent_id: row.agent_id.unwrap_or_default(),
            ulo_name: row.ulo_name.unwrap_or_default(),
            uno_share: row.uno_share.unwrap_or(47.0),
            agent_share: row.agent_share.unwrap_or(3.0),
            ulo_share: row.ulo_share.unwrap_or(50.0),
            uptime: row.uptime.unwrap_or(0.0),
            is_online: row.is_online.unwrap_or(false),
            created_at: row.created_at.map(|d| d.to_rfc3339()).unwrap_or_else(|| Utc::now().to_rfc3339()),
            updated_at: row.updated_at.map(|d| d.to_rfc3339()).unwrap_or_else(|| Utc::now().to_rfc3339()),
            owner_wallet_address: row.owner_wallet_address,
            device_id: row.device_id,
            device_name: row.device_name,
            activation_start_at: row.activation_start_at.map(|d| d.to_rfc3339()),
            activation_end_at: row.activation_end_at.map(|d| d.to_rfc3339()),
            activation_by: row.activation_by,
            activation_postponed_ms: row.activation_postponed_ms,
            lease_user_id: row.lease_user_id,
            lease_share_percentage: row.lease_share_percentage.unwrap_or(0.0),
            lease_min_uptime_percentage: row.lease_min_uptime_percentage.unwrap_or(75.0),
            lease_from: row.lease_from.map(|d| d.to_rfc3339()),
            lease_to: row.lease_to.map(|d| d.to_rfc3339()),
            validation_last_success_at: row.validation_last_success_at.map(|d| d.to_rfc3339()),
            settings: row.settings,
            synced_at: row.synced_at.map(|d| d.to_rfc3339()),
            is_on_marketplace: row.is_on_marketplace.unwrap_or(false),
            is_on_uno_marketplace: row.is_on_uno_marketplace.unwrap_or(false),
            is_published: row.is_published.unwrap_or(false),
            marketplace_status: row.marketplace_status,
            marketplace_referral_code: row.marketplace_referral_code,
        }
    }
}

#[async_trait]
impl LicenseRepositoryTrait for LicenseRepository {
    async fn save_license(&self, new_license: NewLicense) -> Result<LicenseEntity, String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let row = sqlx::query_as::<_, LicenseRow>(r#"
            INSERT INTO licenses (
                id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                uno_share, agent_share, ulo_share, uptime, is_online,
                is_active, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 0, false, true, $11, $11)
            RETURNING id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                      uno_share, agent_share, ulo_share, uptime, is_online,
                      created_at, updated_at, owner_wallet_address, device_id,
                      device_name, activation_start_at, activation_end_at, activation_by,
                      activation_postponed_ms, lease_user_id, lease_share_percentage,
                      lease_min_uptime_percentage, lease_from, lease_to,
                      validation_last_success_at, settings, synced_at,
                      is_on_marketplace, is_on_uno_marketplace, is_published,
                      marketplace_status, marketplace_referral_code
        "#)
            .bind(&id)
            .bind(&new_license.license_id)
            .bind(&new_license.node_id)
            .bind(&new_license.lease_code)
            .bind(&new_license.alias)
            .bind(&new_license.agent_id)
            .bind(&new_license.ulo_name)
            .bind(new_license.uno_share)
            .bind(new_license.agent_share)
            .bind(new_license.ulo_share)
            .bind(now)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to save license: {}", e))?;

        Ok(LicenseEntity::from(row))
    }

    async fn get_license_by_id(&self, id: &str) -> Result<Option<LicenseEntity>, String> {
        let row = sqlx::query_as::<_, LicenseRow>(r#"
            SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                   uno_share, agent_share, ulo_share, uptime, is_online,
                   created_at, updated_at, owner_wallet_address, device_id,
                   device_name, activation_start_at, activation_end_at, activation_by,
                   activation_postponed_ms, lease_user_id, lease_share_percentage,
                   lease_min_uptime_percentage, lease_from, lease_to,
                   validation_last_success_at, settings, synced_at,
                   is_on_marketplace, is_on_uno_marketplace, is_published,
                   marketplace_status, marketplace_referral_code
            FROM licenses
            WHERE id = $1 AND is_active = true
        "#)
            .bind(id)
            .fetch_optional(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to get license by id: {}", e))?;

        Ok(row.map(LicenseEntity::from))
    }

    async fn get_license_by_license_id(&self, license_id: &str) -> Result<Option<LicenseEntity>, String> {
        let row = sqlx::query_as::<_, LicenseRow>(r#"
            SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                   uno_share, agent_share, ulo_share, uptime, is_online,
                   created_at, updated_at, owner_wallet_address, device_id,
                   device_name, activation_start_at, activation_end_at, activation_by,
                   activation_postponed_ms, lease_user_id, lease_share_percentage,
                   lease_min_uptime_percentage, lease_from, lease_to,
                   validation_last_success_at, settings, synced_at,
                   is_on_marketplace, is_on_uno_marketplace, is_published,
                   marketplace_status, marketplace_referral_code
            FROM licenses
            WHERE license_id = $1 AND is_active = true
        "#)
            .bind(license_id)
            .fetch_optional(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to get license by license_id: {}", e))?;

        Ok(row.map(LicenseEntity::from))
    }

    async fn get_licenses_by_agent_id(&self, agent_id: &str) -> Result<Vec<LicenseEntity>, String> {
        let rows = sqlx::query_as::<_, LicenseRow>(r#"
            SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                   uno_share, agent_share, ulo_share, uptime, is_online,
                   created_at, updated_at, owner_wallet_address, device_id,
                   device_name, activation_start_at, activation_end_at, activation_by,
                   activation_postponed_ms, lease_user_id, lease_share_percentage,
                   lease_min_uptime_percentage, lease_from, lease_to,
                   validation_last_success_at, settings, synced_at,
                   is_on_marketplace, is_on_uno_marketplace, is_published,
                   marketplace_status, marketplace_referral_code
            FROM licenses
            WHERE agent_id = $1 AND is_active = true
            ORDER BY created_at DESC
        "#)
            .bind(agent_id)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to get licenses by agent_id: {}", e))?;

        Ok(rows.into_iter().map(LicenseEntity::from).collect())
    }

    async fn get_licenses_by_node_id(&self, node_id: &str) -> Result<Vec<LicenseEntity>, String> {
        let rows = sqlx::query_as::<_, LicenseRow>(r#"
            SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                   uno_share, agent_share, ulo_share, uptime, is_online,
                   created_at, updated_at, owner_wallet_address, device_id,
                   device_name, activation_start_at, activation_end_at, activation_by,
                   activation_postponed_ms, lease_user_id, lease_share_percentage,
                   lease_min_uptime_percentage, lease_from, lease_to,
                   validation_last_success_at, settings, synced_at,
                   is_on_marketplace, is_on_uno_marketplace, is_published,
                   marketplace_status, marketplace_referral_code
            FROM licenses
            WHERE node_id = $1 AND is_active = true
            ORDER BY created_at DESC
        "#)
            .bind(node_id)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to get licenses by node_id: {}", e))?;

        Ok(rows.into_iter().map(LicenseEntity::from).collect())
    }

    async fn list_licenses(&self) -> Result<Vec<LicenseEntity>, String> {
        let rows = sqlx::query_as::<_, LicenseRow>(r#"
            SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                   uno_share, agent_share, ulo_share, uptime, is_online,
                   created_at, updated_at, owner_wallet_address, device_id,
                   device_name, activation_start_at, activation_end_at, activation_by,
                   activation_postponed_ms, lease_user_id, lease_share_percentage,
                   lease_min_uptime_percentage, lease_from, lease_to,
                   validation_last_success_at, settings, synced_at,
                   is_on_marketplace, is_on_uno_marketplace, is_published,
                   marketplace_status, marketplace_referral_code
            FROM licenses
            WHERE is_active = true
            ORDER BY created_at DESC
        "#)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to list licenses: {}", e))?;

        Ok(rows.into_iter().map(LicenseEntity::from).collect())
    }

    async fn bulk_save_licenses(&self, licenses: Vec<NewLicense>) -> Result<usize, String> {
        let mut count = 0;
        let now = Utc::now();

        for license in licenses {
            let id = Uuid::new_v4().to_string();

            let result = sqlx::query(r#"
                INSERT INTO licenses (
                    id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                    uno_share, agent_share, ulo_share, uptime, is_online,
                    is_active, created_at, updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 0, false, true, $11, $11)
                ON CONFLICT (id) DO NOTHING
            "#)
                .bind(&id)
                .bind(&license.license_id)
                .bind(&license.node_id)
                .bind(&license.lease_code)
                .bind(&license.alias)
                .bind(&license.agent_id)
                .bind(&license.ulo_name)
                .bind(license.uno_share)
                .bind(license.agent_share)
                .bind(license.ulo_share)
                .bind(now)
                .execute(self.pool.as_ref())
                .await
                .map_err(|e| format!("Failed to bulk save license: {}", e))?;

            if result.rows_affected() > 0 {
                count += 1;
            }
        }

        Ok(count)
    }

    async fn update_license(&self, license: LicenseEntity) -> Result<LicenseEntity, String> {
        let now = Utc::now();

        let row = sqlx::query_as::<_, LicenseRow>(r#"
            UPDATE licenses SET
                node_id = $2,
                lease_code = $3,
                alias = $4,
                agent_id = $5,
                ulo_name = $6,
                uno_share = $7,
                agent_share = $8,
                ulo_share = $9,
                uptime = $10,
                is_online = $11,
                updated_at = $12,
                owner_wallet_address = $13,
                device_id = $14,
                device_name = $15,
                activation_start_at = $16,
                activation_end_at = $17,
                activation_by = $18,
                activation_postponed_ms = $19,
                lease_user_id = $20,
                lease_share_percentage = $21,
                lease_min_uptime_percentage = $22,
                lease_from = $23,
                lease_to = $24,
                validation_last_success_at = $25,
                settings = $26,
                synced_at = $27,
                is_on_marketplace = $28,
                is_on_uno_marketplace = $29,
                is_published = $30,
                marketplace_status = $31,
                marketplace_referral_code = $32
            WHERE id = $1
            RETURNING id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                      uno_share, agent_share, ulo_share, uptime, is_online,
                      created_at, updated_at, owner_wallet_address, device_id,
                      device_name, activation_start_at, activation_end_at, activation_by,
                      activation_postponed_ms, lease_user_id, lease_share_percentage,
                      lease_min_uptime_percentage, lease_from, lease_to,
                      validation_last_success_at, settings, synced_at,
                      is_on_marketplace, is_on_uno_marketplace, is_published,
                      marketplace_status, marketplace_referral_code
        "#)
            .bind(&license.id)
            .bind(&license.node_id)
            .bind(&license.lease_code)
            .bind(&license.alias)
            .bind(&license.agent_id)
            .bind(&license.ulo_name)
            .bind(license.uno_share)
            .bind(license.agent_share)
            .bind(license.ulo_share)
            .bind(license.uptime)
            .bind(license.is_online)
            .bind(now)
            .bind(&license.owner_wallet_address)
            .bind(&license.device_id)
            .bind(&license.device_name)
            .bind(license.activation_start_at.as_ref().and_then(|s| DateTime::parse_from_rfc3339(s).ok()).map(|d| d.with_timezone(&Utc)))
            .bind(license.activation_end_at.as_ref().and_then(|s| DateTime::parse_from_rfc3339(s).ok()).map(|d| d.with_timezone(&Utc)))
            .bind(&license.activation_by)
            .bind(license.activation_postponed_ms)
            .bind(&license.lease_user_id)
            .bind(license.lease_share_percentage)
            .bind(license.lease_min_uptime_percentage)
            .bind(license.lease_from.as_ref().and_then(|s| DateTime::parse_from_rfc3339(s).ok()).map(|d| d.with_timezone(&Utc)))
            .bind(license.lease_to.as_ref().and_then(|s| DateTime::parse_from_rfc3339(s).ok()).map(|d| d.with_timezone(&Utc)))
            .bind(license.validation_last_success_at.as_ref().and_then(|s| DateTime::parse_from_rfc3339(s).ok()).map(|d| d.with_timezone(&Utc)))
            .bind(&license.settings)
            .bind(license.synced_at.as_ref().and_then(|s| DateTime::parse_from_rfc3339(s).ok()).map(|d| d.with_timezone(&Utc)))
            .bind(license.is_on_marketplace)
            .bind(license.is_on_uno_marketplace)
            .bind(license.is_published)
            .bind(&license.marketplace_status)
            .bind(&license.marketplace_referral_code)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to update license: {}", e))?;

        Ok(LicenseEntity::from(row))
    }

    async fn delete_license(&self, id: &str) -> Result<bool, String> {
        let now = Utc::now();

        let result = sqlx::query(r#"
            UPDATE licenses
            SET is_active = false, updated_at = $2
            WHERE id = $1 AND is_active = true
        "#)
            .bind(id)
            .bind(now)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to delete license: {}", e))?;

        Ok(result.rows_affected() > 0)
    }

    async fn count_licenses(&self) -> Result<i64, String> {
        let result: (i64,) = sqlx::query_as(r#"
            SELECT COUNT(*) FROM licenses WHERE is_active = true
        "#)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to count licenses: {}", e))?;

        Ok(result.0)
    }

    async fn count_online_licenses(&self) -> Result<i64, String> {
        let result: (i64,) = sqlx::query_as(r#"
            SELECT COUNT(*) FROM licenses WHERE is_active = true AND is_online = true
        "#)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to count online licenses: {}", e))?;

        Ok(result.0)
    }

    async fn get_last_synced(&self) -> Result<Option<String>, String> {
        let result: Option<(DateTime<Utc>,)> = sqlx::query_as(r#"
            SELECT MAX(synced_at) FROM licenses WHERE is_active = true AND synced_at IS NOT NULL
        "#)
            .fetch_optional(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to get last synced: {}", e))?;

        Ok(result.and_then(|(dt,)| Some(dt.to_rfc3339())))
    }

    async fn upsert_license_from_api(&self, license: NewLicenseFromApi) -> Result<LicenseEntity, String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Parse datetime strings
        let activation_start_at = license.activation_start_at.as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&Utc));
        let activation_end_at = license.activation_end_at.as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&Utc));
        let lease_from = license.lease_from.as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&Utc));
        let lease_to = license.lease_to.as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&Utc));
        let validation_last_success_at = license.validation_last_success_at.as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&Utc));

        let row = sqlx::query_as::<_, LicenseRow>(r#"
            INSERT INTO licenses (
                id, license_id, node_id, alias, owner_wallet_address,
                device_id, device_name, activation_start_at, activation_end_at,
                activation_by, activation_postponed_ms, lease_user_id,
                lease_share_percentage, lease_min_uptime_percentage,
                lease_from, lease_to, validation_last_success_at,
                uptime, settings, is_online, synced_at,
                is_active, created_at, updated_at,
                agent_id, ulo_name, uno_share, agent_share, ulo_share
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21, true, $22, $22,
                '', '', 47.0, 3.0, 50.0
            )
            ON CONFLICT (id) DO UPDATE SET
                node_id = EXCLUDED.node_id,
                alias = EXCLUDED.alias,
                owner_wallet_address = EXCLUDED.owner_wallet_address,
                device_id = EXCLUDED.device_id,
                device_name = EXCLUDED.device_name,
                activation_start_at = EXCLUDED.activation_start_at,
                activation_end_at = EXCLUDED.activation_end_at,
                activation_by = EXCLUDED.activation_by,
                activation_postponed_ms = EXCLUDED.activation_postponed_ms,
                lease_user_id = EXCLUDED.lease_user_id,
                lease_share_percentage = EXCLUDED.lease_share_percentage,
                lease_min_uptime_percentage = EXCLUDED.lease_min_uptime_percentage,
                lease_from = EXCLUDED.lease_from,
                lease_to = EXCLUDED.lease_to,
                validation_last_success_at = EXCLUDED.validation_last_success_at,
                uptime = EXCLUDED.uptime,
                settings = EXCLUDED.settings,
                is_online = EXCLUDED.is_online,
                synced_at = EXCLUDED.synced_at,
                updated_at = EXCLUDED.updated_at
            RETURNING id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                      uno_share, agent_share, ulo_share, uptime, is_online,
                      created_at, updated_at, owner_wallet_address, device_id,
                      device_name, activation_start_at, activation_end_at, activation_by,
                      activation_postponed_ms, lease_user_id, lease_share_percentage,
                      lease_min_uptime_percentage, lease_from, lease_to,
                      validation_last_success_at, settings, synced_at,
                      is_on_marketplace, is_on_uno_marketplace, is_published,
                      marketplace_status, marketplace_referral_code
        "#)
            .bind(&id)
            .bind(&license.license_id)
            .bind(&license.node_id)
            .bind(&license.alias)
            .bind(&license.owner_wallet_address)
            .bind(&license.device_id)
            .bind(&license.device_name)
            .bind(activation_start_at)
            .bind(activation_end_at)
            .bind(&license.activation_by)
            .bind(license.activation_postponed_ms)
            .bind(&license.lease_user_id)
            .bind(license.lease_share_percentage)
            .bind(license.lease_min_uptime_percentage)
            .bind(lease_from)
            .bind(lease_to)
            .bind(validation_last_success_at)
            .bind(license.uptime)
            .bind(&license.settings)
            .bind(license.is_online)
            .bind(now)
            .bind(now)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to upsert license from API: {}", e))?;

        Ok(LicenseEntity::from(row))
    }

    async fn upsert_licenses_from_api(&self, licenses: Vec<NewLicenseFromApi>) -> Result<usize, String> {
        let mut count = 0;

        for license in licenses {
            match self.upsert_license_from_api(license).await {
                Ok(_) => count += 1,
                Err(e) => {
                    tracing::warn!("Failed to upsert license: {}", e);
                }
            }
        }

        Ok(count)
    }

    async fn get_marketplace_licenses(&self) -> Result<Vec<LicenseEntity>, String> {
        let rows = sqlx::query_as::<_, LicenseRow>(r#"
            SELECT id, license_id, node_id, lease_code, alias, agent_id, ulo_name,
                   uno_share, agent_share, ulo_share, uptime, is_online,
                   created_at, updated_at, owner_wallet_address, device_id,
                   device_name, activation_start_at, activation_end_at, activation_by,
                   activation_postponed_ms, lease_user_id, lease_share_percentage,
                   lease_min_uptime_percentage, lease_from, lease_to,
                   validation_last_success_at, settings, synced_at,
                   is_on_marketplace, is_on_uno_marketplace, is_published,
                   marketplace_status, marketplace_referral_code
            FROM licenses
            WHERE is_active = true AND is_on_marketplace = true
            ORDER BY created_at DESC
        "#)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to get marketplace licenses: {}", e))?;

        Ok(rows.into_iter().map(LicenseEntity::from).collect())
    }

    async fn update_marketplace_status(
        &self,
        license_id: &str,
        is_on_marketplace: bool,
        status: Option<&str>,
        referral_code: Option<&str>,
    ) -> Result<(), String> {
        let now = Utc::now();

        sqlx::query(r#"
            UPDATE licenses SET
                is_on_marketplace = $2,
                marketplace_status = $3,
                marketplace_referral_code = $4,
                updated_at = $5
            WHERE license_id = $1 AND is_active = true
        "#)
            .bind(license_id)
            .bind(is_on_marketplace)
            .bind(status)
            .bind(referral_code)
            .bind(now)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to update marketplace status: {}", e))?;

        Ok(())
    }

    async fn publish_to_marketplace(&self, license_ids: Vec<String>) -> Result<usize, String> {
        if license_ids.is_empty() {
            return Ok(0);
        }

        let now = Utc::now();
        let mut count = 0;

        for license_id in license_ids {
            let result = sqlx::query(r#"
                UPDATE licenses SET
                    is_on_marketplace = true,
                    marketplace_status = 'unclaimed',
                    updated_at = $2
                WHERE license_id = $1 AND is_active = true
            "#)
                .bind(&license_id)
                .bind(now)
                .execute(self.pool.as_ref())
                .await
                .map_err(|e| format!("Failed to publish license: {}", e))?;

            if result.rows_affected() > 0 {
                count += 1;
            }
        }

        Ok(count)
    }

    async fn unpublish_from_marketplace(&self, license_ids: Vec<String>) -> Result<usize, String> {
        if license_ids.is_empty() {
            return Ok(0);
        }

        let now = Utc::now();
        let mut count = 0;

        for license_id in license_ids {
            let result = sqlx::query(r#"
                UPDATE licenses SET
                    is_on_marketplace = false,
                    updated_at = $2
                WHERE license_id = $1 AND is_active = true
            "#)
                .bind(&license_id)
                .bind(now)
                .execute(self.pool.as_ref())
                .await
                .map_err(|e| format!("Failed to unpublish license: {}", e))?;

            if result.rows_affected() > 0 {
                count += 1;
            }
        }

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
        let now = Utc::now();

        sqlx::query(r#"
            UPDATE licenses SET
                is_on_marketplace = $2,
                is_on_uno_marketplace = $3,
                marketplace_status = $4,
                marketplace_referral_code = $5,
                updated_at = $6
            WHERE license_id = $1 AND is_active = true
        "#)
            .bind(license_id)
            .bind(is_on_marketplace)
            .bind(is_on_uno_marketplace)
            .bind(status)
            .bind(referral_code)
            .bind(now)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to update marketplace status extended: {}", e))?;

        Ok(())
    }

    async fn set_is_published(&self, license_id: &str, is_published: bool) -> Result<(), String> {
        let now = Utc::now();

        sqlx::query(r#"
            UPDATE licenses SET
                is_published = $2,
                updated_at = $3
            WHERE license_id = $1 AND is_active = true
        "#)
            .bind(license_id)
            .bind(is_published)
            .bind(now)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to set is_published: {}", e))?;

        Ok(())
    }

    async fn get_marketplace_data_batch(&self, license_ids: &[String]) -> Result<HashMap<String, MarketplaceData>, String> {
        if license_ids.is_empty() {
            return Ok(HashMap::new());
        }

        #[derive(sqlx::FromRow)]
        struct MarketplaceRow {
            license_id: String,
            is_on_marketplace: Option<bool>,
            is_on_uno_marketplace: Option<bool>,
        }

        // Build parameterized query for batch lookup
        let placeholders: Vec<String> = (1..=license_ids.len())
            .map(|i| format!("${}", i))
            .collect();
        let query = format!(
            r#"
            SELECT license_id, is_on_marketplace, is_on_uno_marketplace
            FROM licenses
            WHERE license_id IN ({}) AND is_active = true
            "#,
            placeholders.join(", ")
        );

        let mut query_builder = sqlx::query_as::<_, MarketplaceRow>(&query);
        for id in license_ids {
            query_builder = query_builder.bind(id);
        }

        let rows = query_builder
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to get marketplace data batch: {}", e))?;

        let mut result = HashMap::new();
        for row in rows {
            result.insert(row.license_id, MarketplaceData {
                is_on_marketplace: row.is_on_marketplace.unwrap_or(false),
                is_on_uno_marketplace: row.is_on_uno_marketplace.unwrap_or(false),
            });
        }

        Ok(result)
    }

    async fn get_split_data_batch(&self, license_ids: &[String]) -> Result<HashMap<String, SplitData>, String> {
        if license_ids.is_empty() {
            return Ok(HashMap::new());
        }

        #[derive(sqlx::FromRow)]
        struct SplitRow {
            license_id: String,
            uno_share: Option<f64>,
            ulo_share: Option<f64>,
            agent_share: Option<f64>,
        }

        // Build parameterized query for batch lookup
        let placeholders: Vec<String> = (1..=license_ids.len())
            .map(|i| format!("${}", i))
            .collect();
        let query = format!(
            r#"
            SELECT license_id, uno_share, ulo_share, agent_share
            FROM licenses
            WHERE license_id IN ({}) AND is_active = true
            "#,
            placeholders.join(", ")
        );

        let mut query_builder = sqlx::query_as::<_, SplitRow>(&query);
        for id in license_ids {
            query_builder = query_builder.bind(id);
        }

        let rows = query_builder
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| format!("Failed to get split data batch: {}", e))?;

        let mut result = HashMap::new();
        for row in rows {
            result.insert(row.license_id, SplitData {
                uno_share: row.uno_share.unwrap_or(47.0),
                ulo_share: row.ulo_share.unwrap_or(50.0),
                agent_share: row.agent_share.unwrap_or(3.0),
            });
        }

        Ok(result)
    }
}
