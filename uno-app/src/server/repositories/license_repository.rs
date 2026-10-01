//! License repository implementing uno-api LicenseRepository trait

use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

use uno_api::error::DbError;
use uno_api::models::{License, LicenseSummary, Paginated, PaginationParams, SplitType};
use uno_api::traits::{LicenseFilters, LicenseRepository};

use crate::server::db::ConnectionPool;

/// Dynamic type alias for LicenseRepository trait object
pub type DynLicenseRepository = Arc<dyn LicenseRepository + Send + Sync>;

/// PostgreSQL implementation of LicenseRepository
pub struct PostgresLicenseRepository {
    pool: ConnectionPool,
}

impl PostgresLicenseRepository {
    /// Create a new PostgreSQL license repository.
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LicenseRepository for PostgresLicenseRepository {
    async fn insert(&self, license: &License) -> Result<(), DbError> {
        // R5-06: Include exact share percentages and source provenance
        sqlx::query(
            r#"
            INSERT INTO licenses (
                id, lease_code, valid_from, valid_to, split_type, claimed, bound_to_device,
                device_id, claimed_at, created_at,
                uno_share_pct, ulo_share_pct, agent_share_pct,
                source_system, source_version, source_record_id, source_imported_at
            )
            VALUES (
                $1, $2, $3, $4, $5::split_type, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17
            )
            "#,
        )
        .bind(&license.id)
        .bind(&license.lease_code)
        .bind(license.valid_from)
        .bind(license.valid_to)
        .bind(license.split_type.as_db_str())
        .bind(license.claimed)
        .bind(license.bound_to_device)
        .bind(&license.device_id)
        .bind(license.claimed_at)
        .bind(license.created_at)
        // R5-06: Exact shares
        .bind(license.uno_share_pct)
        .bind(license.ulo_share_pct)
        .bind(license.agent_share_pct)
        // R5-06: Source provenance
        .bind(&license.source_system)
        .bind(&license.source_version)
        .bind(&license.source_record_id)
        .bind(if license.source_system.is_some() { Some(chrono::Utc::now()) } else { None })
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
                DbError::DuplicateEntry(format!("License with lease_code {} already exists", license.lease_code))
            } else {
                DbError::QueryError(e.to_string())
            }
        })?;

        Ok(())
    }

    async fn insert_batch(&self, licenses: &[License]) -> Result<usize, DbError> {
        if licenses.is_empty() {
            return Ok(0);
        }

        let mut count = 0;
        for license in licenses {
            match self.insert(license).await {
                Ok(()) => count += 1,
                Err(_) => continue, // Skip individual failures in batch
            }
        }

        Ok(count)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<License>, DbError> {
        // R5-06: Include exact shares and provenance columns
        let license = sqlx::query_as::<_, LicenseRow>(
            r#"SELECT id, lease_code, valid_from, valid_to, split_type::text, claimed, bound_to_device, device_id, claimed_at, created_at,
                      uno_share_pct, ulo_share_pct, agent_share_pct, source_system, source_version, source_record_id
               FROM licenses WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(license.map(License::from))
    }

    async fn get_by_lease_code(&self, lease_code: &str) -> Result<Option<License>, DbError> {
        // R5-06: Include exact shares and provenance columns
        let license = sqlx::query_as::<_, LicenseRow>(
            r#"SELECT id, lease_code, valid_from, valid_to, split_type::text, claimed, bound_to_device, device_id, claimed_at, created_at,
                      uno_share_pct, ulo_share_pct, agent_share_pct, source_system, source_version, source_record_id
               FROM licenses WHERE lease_code = $1"#,
        )
        .bind(lease_code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(license.map(License::from))
    }

    async fn get_first_unclaimed(&self, split_type: SplitType) -> Result<Option<License>, DbError> {
        let now = Utc::now();
        // R5-06: Include exact shares and provenance columns
        let license = sqlx::query_as::<_, LicenseRow>(
            r#"
            SELECT id, lease_code, valid_from, valid_to, split_type::text, claimed, bound_to_device, device_id, claimed_at, created_at,
                   uno_share_pct, ulo_share_pct, agent_share_pct, source_system, source_version, source_record_id
            FROM licenses
            WHERE split_type = $1::split_type
            AND claimed = false
            AND valid_from <= $2
            AND valid_to > $2
            ORDER BY created_at ASC
            LIMIT 1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(split_type.as_db_str())
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(license.map(License::from))
    }

    async fn get_first_unclaimed_any(&self) -> Result<Option<License>, DbError> {
        let now = Utc::now();
        // R5-06: Include exact shares and provenance columns
        let license = sqlx::query_as::<_, LicenseRow>(
            r#"
            SELECT id, lease_code, valid_from, valid_to, split_type::text, claimed, bound_to_device, device_id, claimed_at, created_at,
                   uno_share_pct, ulo_share_pct, agent_share_pct, source_system, source_version, source_record_id
            FROM licenses
            WHERE claimed = false
            AND valid_from <= $1
            AND valid_to > $1
            ORDER BY created_at ASC
            LIMIT 1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(license.map(License::from))
    }

    async fn get_by_split_type(
        &self,
        split_type: SplitType,
        pagination: &PaginationParams,
    ) -> Result<Paginated<License>, DbError> {
        let offset = pagination.offset() as i64;
        let limit = pagination.limit() as i64;

        // R5-06: Include exact shares and provenance columns
        let licenses = sqlx::query_as::<_, LicenseRow>(
            r#"
            SELECT id, lease_code, valid_from, valid_to, split_type::text, claimed, bound_to_device, device_id, claimed_at, created_at,
                   uno_share_pct, ulo_share_pct, agent_share_pct, source_system, source_version, source_record_id
            FROM licenses
            WHERE split_type = $1::split_type
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(split_type.as_db_str())
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM licenses WHERE split_type = $1::split_type",
        )
        .bind(split_type.as_db_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(Paginated::new(
            licenses.into_iter().map(License::from).collect(),
            total.0,
            pagination,
        ))
    }

    async fn get_unclaimed(&self, pagination: &PaginationParams) -> Result<Paginated<License>, DbError> {
        let offset = pagination.offset() as i64;
        let limit = pagination.limit() as i64;
        let now = Utc::now();

        // R5-06: Include exact shares and provenance columns
        let licenses = sqlx::query_as::<_, LicenseRow>(
            r#"
            SELECT id, lease_code, valid_from, valid_to, split_type::text, claimed, bound_to_device, device_id, claimed_at, created_at,
                   uno_share_pct, ulo_share_pct, agent_share_pct, source_system, source_version, source_record_id
            FROM licenses
            WHERE claimed = false AND valid_from <= $1 AND valid_to > $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(now)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM licenses WHERE claimed = false AND valid_from <= $1 AND valid_to > $1",
        )
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(Paginated::new(
            licenses.into_iter().map(License::from).collect(),
            total.0,
            pagination,
        ))
    }

    async fn count_total(&self) -> Result<i64, DbError> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM licenses")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(count.0)
    }

    async fn count_unclaimed(&self) -> Result<i64, DbError> {
        let now = Utc::now();
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM licenses WHERE claimed = false AND valid_from <= $1 AND valid_to > $1",
        )
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(count.0)
    }

    async fn count_unclaimed_by_split(&self, split_type: SplitType) -> Result<i64, DbError> {
        let now = Utc::now();
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM licenses WHERE split_type = $1::split_type AND claimed = false AND valid_from <= $2 AND valid_to > $2",
        )
        .bind(split_type.as_db_str())
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(count.0)
    }

    async fn claim(&self, id: &str, device_id: Option<String>) -> Result<License, DbError> {
        let now = Utc::now();
        let bound_to_device = device_id.is_some();

        // R5-06: Include exact shares and provenance columns in RETURNING
        let license = sqlx::query_as::<_, LicenseRow>(
            r#"
            UPDATE licenses
            SET claimed = true, bound_to_device = $2, device_id = $3, claimed_at = $4
            WHERE id = $1 AND claimed = false
            RETURNING id, lease_code, valid_from, valid_to, split_type::text, claimed, bound_to_device, device_id, claimed_at, created_at,
                      uno_share_pct, ulo_share_pct, agent_share_pct, source_system, source_version, source_record_id
            "#,
        )
        .bind(id)
        .bind(bound_to_device)
        .bind(&device_id)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        match license {
            Some(row) => Ok(License::from(row)),
            None => Err(DbError::NotFound(format!("License {} not found or already claimed", id))),
        }
    }

    async fn lease_code_exists(&self, lease_code: &str) -> Result<bool, DbError> {
        let exists: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM licenses WHERE lease_code = $1)",
        )
        .bind(lease_code)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(exists.0)
    }

    async fn delete(&self, id: &str) -> Result<(), DbError> {
        let result = sqlx::query("DELETE FROM licenses WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!("License {} not found", id)));
        }

        Ok(())
    }

    async fn delete_batch(&self, ids: &[String]) -> Result<usize, DbError> {
        if ids.is_empty() {
            return Ok(0);
        }

        let result = sqlx::query("DELETE FROM licenses WHERE id = ANY($1)")
            .bind(ids)
            .execute(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(result.rows_affected() as usize)
    }

    async fn get_summary(&self) -> Result<LicenseSummary, DbError> {
        let now = Utc::now();

        // Get total counts
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM licenses")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        let claimed: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM licenses WHERE claimed = true")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        let expired: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM licenses WHERE valid_to <= $1")
            .bind(now)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        // Get counts by split type
        let by_split: Vec<SplitTypeCounts> = sqlx::query_as(
            r#"
            SELECT
                split_type::text,
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE claimed = true) as claimed,
                COUNT(*) FILTER (WHERE claimed = false AND valid_from <= $1 AND valid_to > $1) as unclaimed
            FROM licenses
            GROUP BY split_type
            "#,
        )
        .bind(now)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;

        use uno_api::models::SplitTypeSummary;

        Ok(LicenseSummary {
            total: total.0,
            claimed: claimed.0,
            available: total.0 - claimed.0 - expired.0,
            expired: expired.0,
            by_split_type: by_split
                .into_iter()
                .filter_map(|s| {
                    SplitType::from_str(&s.split_type).map(|st| SplitTypeSummary {
                        split_type: st,
                        total: s.total,
                        claimed: s.claimed,
                        available: s.unclaimed, // SQL column 'unclaimed' maps to uno-api's 'available'
                    })
                })
                .collect(),
        })
    }

    async fn search(
        &self,
        filters: &LicenseFilters,
        pagination: &PaginationParams,
    ) -> Result<Paginated<License>, DbError> {
        let offset = pagination.offset() as i64;
        let limit = pagination.limit() as i64;
        let now = Utc::now();

        // Build dynamic query
        let mut conditions = Vec::new();
        let mut param_idx = 1;

        if filters.split_type.is_some() {
            conditions.push(format!("split_type = ${}::split_type", param_idx));
            param_idx += 1;
        }

        if let Some(claimed) = filters.claimed {
            conditions.push(format!("claimed = ${}", param_idx));
            param_idx += 1;
            let _ = claimed; // Used in bind
        }

        if let Some(bound) = filters.bound_to_device {
            conditions.push(format!("bound_to_device = ${}", param_idx));
            param_idx += 1;
            let _ = bound; // Used in bind
        }

        if filters.lease_code_search.is_some() {
            conditions.push(format!("lease_code ILIKE ${}", param_idx));
            param_idx += 1;
        }

        if filters.include_expired != Some(true) {
            conditions.push(format!("valid_to > ${}", param_idx));
            param_idx += 1;
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // R5-06: Include exact shares and provenance columns
        let query = format!(
            r#"
            SELECT id, lease_code, valid_from, valid_to, split_type::text, claimed, bound_to_device, device_id, claimed_at, created_at,
                   uno_share_pct, ulo_share_pct, agent_share_pct, source_system, source_version, source_record_id
            FROM licenses
            {}
            ORDER BY created_at DESC
            LIMIT ${} OFFSET ${}
            "#,
            where_clause,
            param_idx,
            param_idx + 1
        );

        let count_query = format!("SELECT COUNT(*) FROM licenses {}", where_clause);

        // Build and execute query with dynamic bindings
        let mut query_builder = sqlx::query_as::<_, LicenseRow>(&query);

        if let Some(ref split_type) = filters.split_type {
            query_builder = query_builder.bind(split_type.as_db_str());
        }
        if let Some(claimed) = filters.claimed {
            query_builder = query_builder.bind(claimed);
        }
        if let Some(bound) = filters.bound_to_device {
            query_builder = query_builder.bind(bound);
        }
        if let Some(ref search) = filters.lease_code_search {
            query_builder = query_builder.bind(format!("%{}%", search));
        }
        if filters.include_expired != Some(true) {
            query_builder = query_builder.bind(now);
        }

        query_builder = query_builder.bind(limit).bind(offset);

        let licenses = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        // Count query
        let mut count_builder = sqlx::query_as::<_, (i64,)>(&count_query);

        if let Some(ref split_type) = filters.split_type {
            count_builder = count_builder.bind(split_type.as_db_str());
        }
        if let Some(claimed) = filters.claimed {
            count_builder = count_builder.bind(claimed);
        }
        if let Some(bound) = filters.bound_to_device {
            count_builder = count_builder.bind(bound);
        }
        if let Some(ref search) = filters.lease_code_search {
            count_builder = count_builder.bind(format!("%{}%", search));
        }
        if filters.include_expired != Some(true) {
            count_builder = count_builder.bind(now);
        }

        let total = count_builder
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))?;

        Ok(Paginated::new(
            licenses.into_iter().map(License::from).collect(),
            total.0,
            pagination,
        ))
    }
}

// Internal row type for SQLx mapping
#[derive(Debug, sqlx::FromRow)]
struct LicenseRow {
    id: String,
    lease_code: String,
    valid_from: chrono::DateTime<Utc>,
    valid_to: chrono::DateTime<Utc>,
    split_type: String,
    claimed: bool,
    bound_to_device: bool,
    device_id: Option<String>,
    claimed_at: Option<chrono::DateTime<Utc>>,
    created_at: chrono::DateTime<Utc>,
    // R5-06: Exact share percentages
    #[sqlx(default)]
    uno_share_pct: Option<f64>,
    #[sqlx(default)]
    ulo_share_pct: Option<f64>,
    #[sqlx(default)]
    agent_share_pct: Option<f64>,
    // R5-06: Source provenance
    #[sqlx(default)]
    source_system: Option<String>,
    #[sqlx(default)]
    source_version: Option<String>,
    #[sqlx(default)]
    source_record_id: Option<String>,
}

impl From<LicenseRow> for License {
    fn from(row: LicenseRow) -> Self {
        License {
            id: row.id,
            lease_code: row.lease_code,
            valid_from: row.valid_from,
            valid_to: row.valid_to,
            split_type: SplitType::from_str(&row.split_type).unwrap_or(SplitType::Split5050),
            claimed: row.claimed,
            bound_to_device: row.bound_to_device,
            device_id: row.device_id,
            claimed_at: row.claimed_at,
            created_at: row.created_at,
            // R5-06: Exact shares
            uno_share_pct: row.uno_share_pct,
            ulo_share_pct: row.ulo_share_pct,
            agent_share_pct: row.agent_share_pct,
            // R5-06: Source provenance
            source_system: row.source_system,
            source_version: row.source_version,
            source_record_id: row.source_record_id,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct SplitTypeCounts {
    split_type: String,
    total: i64,
    claimed: i64,
    unclaimed: i64,
}

/// Extension trait for SplitType to get database string representation
trait SplitTypeDbExt {
    fn as_db_str(&self) -> &'static str;
}

impl SplitTypeDbExt for SplitType {
    fn as_db_str(&self) -> &'static str {
        // Match PostgreSQL split_type enum values: '5050', '5545', '6040'
        match self {
            SplitType::Split5050 => "5050",
            SplitType::Split5545 => "5545",
            SplitType::Split6040 => "6040",
        }
    }
}
