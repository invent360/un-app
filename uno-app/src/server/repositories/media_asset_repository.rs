//! Media asset repository
//!
//! Provides:
//! - Asset tracking with state machine
//! - Quota management
//! - Access grants
//! - Orphan/missing detection

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// ENTITY STRUCTS
// ============================================

/// Asset state enum matching database
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetState {
    Uploading,
    Ready,
    Quarantined,
    Missing,
    Deleted,
}

/// R4-05: Asset visibility enum for access control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AssetVisibility {
    /// Anyone can access
    Public,
    /// Only owner and granted users can access
    Private,
    /// Only owner can access (default for new uploads)
    #[default]
    Draft,
}

impl AssetVisibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Draft => "draft",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "public" => Some(Self::Public),
            "private" => Some(Self::Private),
            "draft" => Some(Self::Draft),
            _ => None,
        }
    }

    /// R4-05: Check if anonymous access is allowed
    pub fn allows_anonymous(&self) -> bool {
        matches!(self, Self::Public)
    }
}

impl AssetState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Uploading => "uploading",
            Self::Ready => "ready",
            Self::Quarantined => "quarantined",
            Self::Missing => "missing",
            Self::Deleted => "deleted",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "uploading" => Some(Self::Uploading),
            "ready" => Some(Self::Ready),
            "quarantined" => Some(Self::Quarantined),
            "missing" => Some(Self::Missing),
            "deleted" => Some(Self::Deleted),
            _ => None,
        }
    }
}

/// Media asset entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaAsset {
    pub id: Uuid,
    pub resource_id: String,
    pub filename: String,
    pub original_filename: Option<String>,
    pub storage_url: String,
    pub display_url: String,
    pub mime_type: String,
    pub file_size: i64,
    /// R4-05: SHA256 of stored bytes (post-transformation)
    pub sha256_hash: Option<String>,
    /// R4-05: SHA256 of original uploaded bytes (pre-transformation)
    #[sqlx(default)]
    pub original_hash: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub state: String,
    pub state_reason: Option<String>,
    pub state_changed_at: DateTime<Utc>,
    /// R4-05: Visibility for access control
    #[sqlx(default)]
    pub visibility: Option<String>,
    pub owner_id: Option<String>,
    pub owner_type: Option<String>,
    pub category: Option<String>,
    pub alt_text: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}

impl MediaAsset {
    pub fn state_enum(&self) -> Option<AssetState> {
        AssetState::from_str(&self.state)
    }

    /// R4-05: Get visibility as enum
    pub fn visibility_enum(&self) -> AssetVisibility {
        self.visibility
            .as_deref()
            .and_then(AssetVisibility::from_str)
            .unwrap_or(AssetVisibility::Draft)
    }

    /// R4-05: Check if accessor can view this asset
    pub fn is_accessible_by(&self, accessor_id: Option<&str>, is_owner: bool) -> bool {
        let visibility = self.visibility_enum();

        // Public assets are always accessible
        if visibility == AssetVisibility::Public {
            return true;
        }

        // Draft assets only accessible by owner
        if visibility == AssetVisibility::Draft {
            return is_owner || self.owner_id.as_deref() == accessor_id;
        }

        // Private assets: owner gets access, others need grants (checked separately)
        is_owner || self.owner_id.as_deref() == accessor_id
    }
}

/// Media asset grant entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaAssetGrant {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub grantee_id: String,
    pub grantee_type: String,
    pub permission: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

/// Media quota entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaQuota {
    pub owner_id: String,
    pub owner_type: String,
    pub max_total_bytes: i64,
    pub max_file_size: i64,
    pub max_file_count: i32,
    pub max_inodes: i32,
    pub current_bytes: i64,
    pub current_count: i32,
    pub updated_at: DateTime<Utc>,
}

impl MediaQuota {
    /// Check if adding a file would exceed quota
    pub fn would_exceed(&self, file_size: i64) -> QuotaCheckResult {
        if file_size > self.max_file_size {
            return QuotaCheckResult::ExceedsFileSize {
                max: self.max_file_size,
                requested: file_size,
            };
        }

        if self.current_bytes + file_size > self.max_total_bytes {
            return QuotaCheckResult::ExceedsTotalBytes {
                max: self.max_total_bytes,
                current: self.current_bytes,
                requested: file_size,
            };
        }

        if self.current_count >= self.max_file_count {
            return QuotaCheckResult::ExceedsFileCount {
                max: self.max_file_count,
                current: self.current_count,
            };
        }

        QuotaCheckResult::Ok
    }
}

/// Quota check result
#[derive(Debug, Clone)]
pub enum QuotaCheckResult {
    Ok,
    ExceedsFileSize { max: i64, requested: i64 },
    ExceedsTotalBytes { max: i64, current: i64, requested: i64 },
    ExceedsFileCount { max: i32, current: i32 },
}

impl QuotaCheckResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok)
    }
}

/// R4-05: Integrity check result
#[derive(Debug, Clone)]
pub struct IntegrityCheckResult {
    pub verified: bool,
    pub expected_hash: Option<String>,
    pub actual_hash: String,
    pub mismatch_type: Option<String>,
}

impl IntegrityCheckResult {
    pub fn ok(expected_hash: String, actual_hash: String) -> Self {
        Self {
            verified: true,
            expected_hash: Some(expected_hash),
            actual_hash,
            mismatch_type: None,
        }
    }

    pub fn mismatch(expected_hash: Option<String>, actual_hash: String, mismatch_type: &str) -> Self {
        Self {
            verified: false,
            expected_hash,
            actual_hash,
            mismatch_type: Some(mismatch_type.to_string()),
        }
    }
}

/// Asset version entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MediaAssetVersion {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub version_number: i32,
    pub storage_url: String,
    pub display_url: String,
    pub file_size: i64,
    pub sha256_hash: String,
    pub change_summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

// ============================================
// INPUT STRUCTS
// ============================================

/// Input for creating a media asset
#[derive(Debug, Clone)]
pub struct CreateAssetInput {
    pub resource_id: String,
    pub filename: String,
    pub original_filename: Option<String>,
    pub storage_url: String,
    pub display_url: String,
    pub mime_type: String,
    pub file_size: i64,
    /// R4-05: SHA256 of stored bytes (post-transformation)
    pub sha256_hash: Option<String>,
    /// R4-05: SHA256 of original uploaded bytes (pre-transformation)
    pub original_hash: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    /// R4-05: Visibility for access control (defaults to draft)
    pub visibility: Option<AssetVisibility>,
    pub owner_id: Option<String>,
    pub owner_type: Option<String>,
    pub category: Option<String>,
    pub alt_text: Option<String>,
    pub created_by: Option<String>,
}

/// Input for updating asset state
#[derive(Debug, Clone)]
pub struct UpdateStateInput {
    pub asset_id: Uuid,
    pub state: AssetState,
    pub reason: Option<String>,
}

/// Input for creating a grant
#[derive(Debug, Clone)]
pub struct CreateGrantInput {
    pub asset_id: Uuid,
    pub grantee_id: String,
    pub grantee_type: String,
    pub permission: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
}

/// Input for setting quota
#[derive(Debug, Clone)]
pub struct SetQuotaInput {
    pub owner_id: String,
    pub owner_type: String,
    pub max_total_bytes: Option<i64>,
    pub max_file_size: Option<i64>,
    pub max_file_count: Option<i32>,
    pub max_inodes: Option<i32>,
}

// ============================================
// TRAIT DEFINITION
// ============================================

pub type DynMediaAssetRepository = Arc<dyn MediaAssetRepository + Send + Sync>;

#[async_trait]
pub trait MediaAssetRepository: Send + Sync {
    // Asset CRUD
    async fn create_asset(&self, input: CreateAssetInput) -> Result<MediaAsset, AppError>;
    async fn get_asset(&self, id: Uuid) -> Result<Option<MediaAsset>, AppError>;
    async fn get_asset_by_storage_url(&self, storage_url: &str) -> Result<Option<MediaAsset>, AppError>;
    async fn update_state(&self, input: UpdateStateInput) -> Result<(), AppError>;
    /// B4 FIX: Update storage URL after restoration (storage location may change)
    async fn update_storage_url(&self, id: Uuid, storage_url: &str) -> Result<(), AppError>;
    async fn soft_delete(&self, id: Uuid, deleted_by: &str) -> Result<(), AppError>;
    async fn list_by_owner(&self, owner_id: &str, owner_type: &str) -> Result<Vec<MediaAsset>, AppError>;
    async fn list_by_resource(&self, resource_id: &str) -> Result<Vec<MediaAsset>, AppError>;

    // State queries
    async fn find_by_state(&self, state: AssetState, limit: i32) -> Result<Vec<MediaAsset>, AppError>;
    async fn find_missing(&self) -> Result<Vec<MediaAsset>, AppError>;
    async fn find_orphaned(&self) -> Result<Vec<String>, AppError>;

    // Grants
    async fn create_grant(&self, input: CreateGrantInput) -> Result<MediaAssetGrant, AppError>;
    async fn revoke_grant(&self, id: Uuid) -> Result<(), AppError>;
    async fn check_access(&self, asset_id: Uuid, grantee_id: &str, permission: &str) -> Result<bool, AppError>;
    async fn list_grants(&self, asset_id: Uuid) -> Result<Vec<MediaAssetGrant>, AppError>;
    async fn cleanup_expired_grants(&self) -> Result<i32, AppError>;

    // Quotas
    async fn get_quota(&self, owner_id: &str, owner_type: &str) -> Result<Option<MediaQuota>, AppError>;
    async fn set_quota(&self, input: SetQuotaInput) -> Result<MediaQuota, AppError>;
    async fn check_quota(&self, owner_id: &str, owner_type: &str, file_size: i64) -> Result<QuotaCheckResult, AppError>;

    // Versions
    async fn create_version(&self, asset_id: Uuid, storage_url: &str, display_url: &str, file_size: i64, sha256_hash: &str, change_summary: Option<&str>, created_by: Option<&str>) -> Result<MediaAssetVersion, AppError>;
    async fn get_versions(&self, asset_id: Uuid) -> Result<Vec<MediaAssetVersion>, AppError>;
    async fn get_latest_version(&self, asset_id: Uuid) -> Result<Option<MediaAssetVersion>, AppError>;

    // Statistics
    async fn count_by_state(&self) -> Result<Vec<(String, i64)>, AppError>;
    async fn total_storage_used(&self) -> Result<i64, AppError>;

    // R4-05: Visibility and integrity
    /// Check if accessor can view the asset based on visibility policy
    async fn check_visibility(&self, asset_id: Uuid, accessor_id: Option<&str>, is_owner: bool) -> Result<bool, AppError>;
    /// Update asset visibility (only owner can do this)
    async fn update_visibility(&self, asset_id: Uuid, visibility: AssetVisibility, actor_id: &str) -> Result<bool, AppError>;
    /// Verify stored bytes match expected hash
    async fn verify_integrity(&self, asset_id: Uuid, computed_hash: &str) -> Result<IntegrityCheckResult, AppError>;
    /// Get asset for visibility-checked read
    async fn get_asset_with_visibility_check(&self, id: Uuid, accessor_id: Option<&str>, is_owner: bool) -> Result<Option<MediaAsset>, AppError>;
}

// ============================================
// POSTGRES IMPLEMENTATION
// ============================================

pub struct MediaAssetRepositoryImpl {
    pool: ConnectionPool,
}

impl MediaAssetRepositoryImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MediaAssetRepository for MediaAssetRepositoryImpl {
    async fn create_asset(&self, input: CreateAssetInput) -> Result<MediaAsset, AppError> {
        // R4-05: Default to draft visibility for new uploads
        let visibility = input.visibility.unwrap_or(AssetVisibility::Draft);

        let asset = sqlx::query_as::<_, MediaAsset>(
            r#"
            INSERT INTO media_assets (
                resource_id, filename, original_filename, storage_url, display_url,
                mime_type, file_size, sha256_hash, original_hash, width, height,
                visibility, owner_id, owner_type, category, alt_text, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12::asset_visibility, $13, $14, $15, $16, $17)
            RETURNING id, resource_id, filename, original_filename, storage_url, display_url,
                      mime_type, file_size, sha256_hash, original_hash, width, height,
                      state, state_reason, state_changed_at, visibility::TEXT,
                      owner_id, owner_type, category, alt_text,
                      created_at, created_by, updated_at, deleted_at, deleted_by
            "#,
        )
        .bind(&input.resource_id)
        .bind(&input.filename)
        .bind(&input.original_filename)
        .bind(&input.storage_url)
        .bind(&input.display_url)
        .bind(&input.mime_type)
        .bind(input.file_size)
        .bind(&input.sha256_hash)
        .bind(&input.original_hash)
        .bind(input.width)
        .bind(input.height)
        .bind(visibility.as_str())
        .bind(&input.owner_id)
        .bind(&input.owner_type)
        .bind(&input.category)
        .bind(&input.alt_text)
        .bind(&input.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            asset_id = %asset.id,
            resource_id = %asset.resource_id,
            mime_type = %asset.mime_type,
            file_size = asset.file_size,
            visibility = %visibility.as_str(),
            "Media asset created"
        );

        Ok(asset)
    }

    async fn get_asset(&self, id: Uuid) -> Result<Option<MediaAsset>, AppError> {
        let result = sqlx::query_as::<_, MediaAsset>(
            r#"
            SELECT id, resource_id, filename, original_filename, storage_url, display_url,
                   mime_type, file_size, sha256_hash, original_hash, width, height,
                   state, state_reason, state_changed_at, visibility::TEXT,
                   owner_id, owner_type, category, alt_text,
                   created_at, created_by, updated_at, deleted_at, deleted_by
            FROM media_assets
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn get_asset_by_storage_url(&self, storage_url: &str) -> Result<Option<MediaAsset>, AppError> {
        let result = sqlx::query_as::<_, MediaAsset>(
            r#"
            SELECT id, resource_id, filename, original_filename, storage_url, display_url,
                   mime_type, file_size, sha256_hash, original_hash, width, height,
                   state, state_reason, state_changed_at, visibility::TEXT,
                   owner_id, owner_type, category, alt_text,
                   created_at, created_by, updated_at, deleted_at, deleted_by
            FROM media_assets
            WHERE storage_url = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(storage_url)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn update_state(&self, input: UpdateStateInput) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE media_assets
            SET state = $2, state_reason = $3
            WHERE id = $1
            "#,
        )
        .bind(input.asset_id)
        .bind(input.state.as_str())
        .bind(&input.reason)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            asset_id = %input.asset_id,
            new_state = %input.state.as_str(),
            "Media asset state updated"
        );

        Ok(())
    }

    /// B4 FIX: Update storage URL after restoration
    async fn update_storage_url(&self, id: Uuid, storage_url: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE media_assets
            SET storage_url = $2, updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .bind(storage_url)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            asset_id = %id,
            storage_url = %storage_url,
            "Media asset storage URL updated"
        );

        Ok(())
    }

    async fn soft_delete(&self, id: Uuid, deleted_by: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE media_assets
            SET deleted_at = NOW(), deleted_by = $2, state = 'deleted'
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .bind(deleted_by)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(asset_id = %id, deleted_by = %deleted_by, "Media asset soft deleted");

        Ok(())
    }

    async fn list_by_owner(&self, owner_id: &str, owner_type: &str) -> Result<Vec<MediaAsset>, AppError> {
        let results = sqlx::query_as::<_, MediaAsset>(
            r#"
            SELECT id, resource_id, filename, original_filename, storage_url, display_url,
                   mime_type, file_size, sha256_hash, original_hash, width, height,
                   state, state_reason, state_changed_at, visibility::TEXT,
                   owner_id, owner_type, category, alt_text,
                   created_at, created_by, updated_at, deleted_at, deleted_by
            FROM media_assets
            WHERE owner_id = $1 AND owner_type = $2 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .bind(owner_id)
        .bind(owner_type)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn list_by_resource(&self, resource_id: &str) -> Result<Vec<MediaAsset>, AppError> {
        let results = sqlx::query_as::<_, MediaAsset>(
            r#"
            SELECT id, resource_id, filename, original_filename, storage_url, display_url,
                   mime_type, file_size, sha256_hash, original_hash, width, height,
                   state, state_reason, state_changed_at, visibility::TEXT,
                   owner_id, owner_type, category, alt_text,
                   created_at, created_by, updated_at, deleted_at, deleted_by
            FROM media_assets
            WHERE resource_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .bind(resource_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn find_by_state(&self, state: AssetState, limit: i32) -> Result<Vec<MediaAsset>, AppError> {
        let results = sqlx::query_as::<_, MediaAsset>(
            r#"
            SELECT id, resource_id, filename, original_filename, storage_url, display_url,
                   mime_type, file_size, sha256_hash, original_hash, width, height,
                   state, state_reason, state_changed_at, visibility::TEXT,
                   owner_id, owner_type, category, alt_text,
                   created_at, created_by, updated_at, deleted_at, deleted_by
            FROM media_assets
            WHERE state = $1 AND deleted_at IS NULL
            ORDER BY state_changed_at ASC
            LIMIT $2
            "#,
        )
        .bind(state.as_str())
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn find_missing(&self) -> Result<Vec<MediaAsset>, AppError> {
        self.find_by_state(AssetState::Missing, 1000).await
    }

    async fn find_orphaned(&self) -> Result<Vec<String>, AppError> {
        // This would typically be called after comparing with filesystem
        // Returns storage_urls that exist in DB but file doesn't exist on disk
        // For now, return assets marked as missing
        let results: Vec<(String,)> = sqlx::query_as(
            r#"
            SELECT storage_url FROM media_assets
            WHERE state = 'missing' AND deleted_at IS NULL
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results.into_iter().map(|(url,)| url).collect())
    }

    async fn create_grant(&self, input: CreateGrantInput) -> Result<MediaAssetGrant, AppError> {
        let grant = sqlx::query_as::<_, MediaAssetGrant>(
            r#"
            INSERT INTO media_asset_grants (asset_id, grantee_id, grantee_type, permission, expires_at, created_by)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (asset_id, grantee_id, grantee_type, permission) DO UPDATE
            SET expires_at = EXCLUDED.expires_at
            RETURNING id, asset_id, grantee_id, grantee_type, permission, expires_at, created_at, created_by
            "#,
        )
        .bind(input.asset_id)
        .bind(&input.grantee_id)
        .bind(&input.grantee_type)
        .bind(&input.permission)
        .bind(input.expires_at)
        .bind(&input.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            grant_id = %grant.id,
            asset_id = %grant.asset_id,
            grantee = %grant.grantee_id,
            permission = %grant.permission,
            "Media asset grant created"
        );

        Ok(grant)
    }

    async fn revoke_grant(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM media_asset_grants WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn check_access(&self, asset_id: Uuid, grantee_id: &str, permission: &str) -> Result<bool, AppError> {
        let result: Option<(i64,)> = sqlx::query_as(
            r#"
            SELECT 1 FROM media_asset_grants
            WHERE asset_id = $1 AND grantee_id = $2 AND permission = $3
              AND (expires_at IS NULL OR expires_at > NOW())
            "#,
        )
        .bind(asset_id)
        .bind(grantee_id)
        .bind(permission)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.is_some())
    }

    async fn list_grants(&self, asset_id: Uuid) -> Result<Vec<MediaAssetGrant>, AppError> {
        let results = sqlx::query_as::<_, MediaAssetGrant>(
            r#"
            SELECT id, asset_id, grantee_id, grantee_type, permission, expires_at, created_at, created_by
            FROM media_asset_grants
            WHERE asset_id = $1 AND (expires_at IS NULL OR expires_at > NOW())
            ORDER BY created_at DESC
            "#,
        )
        .bind(asset_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn cleanup_expired_grants(&self) -> Result<i32, AppError> {
        let result = sqlx::query(
            "DELETE FROM media_asset_grants WHERE expires_at IS NOT NULL AND expires_at <= NOW()"
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let count = result.rows_affected() as i32;
        if count > 0 {
            tracing::info!(count = count, "Expired media grants cleaned up");
        }
        Ok(count)
    }

    async fn get_quota(&self, owner_id: &str, owner_type: &str) -> Result<Option<MediaQuota>, AppError> {
        let result = sqlx::query_as::<_, MediaQuota>(
            r#"
            SELECT owner_id, owner_type, max_total_bytes, max_file_size, max_file_count,
                   max_inodes, current_bytes, current_count, updated_at
            FROM media_quotas
            WHERE owner_id = $1 AND owner_type = $2
            "#,
        )
        .bind(owner_id)
        .bind(owner_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn set_quota(&self, input: SetQuotaInput) -> Result<MediaQuota, AppError> {
        let quota = sqlx::query_as::<_, MediaQuota>(
            r#"
            INSERT INTO media_quotas (owner_id, owner_type, max_total_bytes, max_file_size, max_file_count, max_inodes)
            VALUES ($1, $2, COALESCE($3, 104857600), COALESCE($4, 10485760), COALESCE($5, 50), COALESCE($6, 100))
            ON CONFLICT (owner_id, owner_type) DO UPDATE
            SET max_total_bytes = COALESCE($3, media_quotas.max_total_bytes),
                max_file_size = COALESCE($4, media_quotas.max_file_size),
                max_file_count = COALESCE($5, media_quotas.max_file_count),
                max_inodes = COALESCE($6, media_quotas.max_inodes),
                updated_at = NOW()
            RETURNING owner_id, owner_type, max_total_bytes, max_file_size, max_file_count,
                      max_inodes, current_bytes, current_count, updated_at
            "#,
        )
        .bind(&input.owner_id)
        .bind(&input.owner_type)
        .bind(input.max_total_bytes)
        .bind(input.max_file_size)
        .bind(input.max_file_count)
        .bind(input.max_inodes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(quota)
    }

    async fn check_quota(&self, owner_id: &str, owner_type: &str, file_size: i64) -> Result<QuotaCheckResult, AppError> {
        let quota = self.get_quota(owner_id, owner_type).await?;
        match quota {
            Some(q) => Ok(q.would_exceed(file_size)),
            None => Ok(QuotaCheckResult::Ok), // No quota set = no limit
        }
    }

    async fn create_version(&self, asset_id: Uuid, storage_url: &str, display_url: &str, file_size: i64, sha256_hash: &str, change_summary: Option<&str>, created_by: Option<&str>) -> Result<MediaAssetVersion, AppError> {
        let version = sqlx::query_as::<_, MediaAssetVersion>(
            r#"
            INSERT INTO media_asset_versions (asset_id, version_number, storage_url, display_url, file_size, sha256_hash, change_summary, created_by)
            VALUES (
                $1,
                COALESCE((SELECT MAX(version_number) + 1 FROM media_asset_versions WHERE asset_id = $1), 1),
                $2, $3, $4, $5, $6, $7
            )
            RETURNING id, asset_id, version_number, storage_url, display_url, file_size, sha256_hash, change_summary, created_at, created_by
            "#,
        )
        .bind(asset_id)
        .bind(storage_url)
        .bind(display_url)
        .bind(file_size)
        .bind(sha256_hash)
        .bind(change_summary)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            asset_id = %asset_id,
            version = version.version_number,
            "Media asset version created"
        );

        Ok(version)
    }

    async fn get_versions(&self, asset_id: Uuid) -> Result<Vec<MediaAssetVersion>, AppError> {
        let results = sqlx::query_as::<_, MediaAssetVersion>(
            r#"
            SELECT id, asset_id, version_number, storage_url, display_url, file_size, sha256_hash, change_summary, created_at, created_by
            FROM media_asset_versions
            WHERE asset_id = $1
            ORDER BY version_number DESC
            "#,
        )
        .bind(asset_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn get_latest_version(&self, asset_id: Uuid) -> Result<Option<MediaAssetVersion>, AppError> {
        let result = sqlx::query_as::<_, MediaAssetVersion>(
            r#"
            SELECT id, asset_id, version_number, storage_url, display_url, file_size, sha256_hash, change_summary, created_at, created_by
            FROM media_asset_versions
            WHERE asset_id = $1
            ORDER BY version_number DESC
            LIMIT 1
            "#,
        )
        .bind(asset_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    async fn count_by_state(&self) -> Result<Vec<(String, i64)>, AppError> {
        let results: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT state, COUNT(*) as count
            FROM media_assets
            WHERE deleted_at IS NULL
            GROUP BY state
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }

    async fn total_storage_used(&self) -> Result<i64, AppError> {
        let result: (Option<i64>,) = sqlx::query_as(
            "SELECT COALESCE(SUM(file_size), 0) FROM media_assets WHERE state = 'ready' AND deleted_at IS NULL"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.0.unwrap_or(0))
    }

    /// R4-05: Check if accessor can view the asset based on visibility policy
    async fn check_visibility(&self, asset_id: Uuid, accessor_id: Option<&str>, is_owner: bool) -> Result<bool, AppError> {
        // Use the PostgreSQL function for consistent visibility checks
        let result: (bool,) = sqlx::query_as(
            "SELECT check_media_visibility($1, $2, $3)"
        )
        .bind(asset_id)
        .bind(accessor_id)
        .bind(is_owner)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.0)
    }

    /// R4-05: Update asset visibility (only owner can do this)
    async fn update_visibility(&self, asset_id: Uuid, visibility: AssetVisibility, actor_id: &str) -> Result<bool, AppError> {
        // Use the PostgreSQL function which enforces ownership check
        let result: (bool,) = sqlx::query_as(
            "SELECT update_media_visibility($1, $2::asset_visibility, $3)"
        )
        .bind(asset_id)
        .bind(visibility.as_str())
        .bind(actor_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.0 {
            tracing::info!(
                asset_id = %asset_id,
                visibility = %visibility.as_str(),
                actor_id = %actor_id,
                "Media asset visibility updated"
            );
        }

        Ok(result.0)
    }

    /// R4-05: Verify stored bytes match expected hash
    async fn verify_integrity(&self, asset_id: Uuid, computed_hash: &str) -> Result<IntegrityCheckResult, AppError> {
        // Use the PostgreSQL function which also quarantines on mismatch
        let result: (bool, Option<String>, String, Option<String>) = sqlx::query_as(
            "SELECT verified, expected_hash, actual_hash, mismatch_type FROM verify_media_integrity($1, $2)"
        )
        .bind(asset_id)
        .bind(computed_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !result.0 {
            tracing::warn!(
                asset_id = %asset_id,
                expected_hash = ?result.1,
                actual_hash = %result.2,
                mismatch_type = ?result.3,
                "Media asset integrity verification failed"
            );
        }

        Ok(IntegrityCheckResult {
            verified: result.0,
            expected_hash: result.1,
            actual_hash: result.2,
            mismatch_type: result.3,
        })
    }

    /// R4-05: Get asset for visibility-checked read
    async fn get_asset_with_visibility_check(&self, id: Uuid, accessor_id: Option<&str>, is_owner: bool) -> Result<Option<MediaAsset>, AppError> {
        // First get the asset
        let asset = self.get_asset(id).await?;

        let Some(asset) = asset else {
            return Ok(None);
        };

        // Check visibility
        let is_accessible = self.check_visibility(id, accessor_id, is_owner).await?;

        if is_accessible {
            Ok(Some(asset))
        } else {
            // Asset exists but not accessible - return None to avoid leaking existence
            tracing::debug!(
                asset_id = %id,
                accessor_id = ?accessor_id,
                is_owner = is_owner,
                visibility = ?asset.visibility,
                "Asset access denied due to visibility policy"
            );
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_state() {
        assert_eq!(AssetState::Ready.as_str(), "ready");
        assert_eq!(AssetState::from_str("ready"), Some(AssetState::Ready));
        assert_eq!(AssetState::from_str("invalid"), None);
    }

    #[test]
    fn test_quota_check() {
        let quota = MediaQuota {
            owner_id: "user1".to_string(),
            owner_type: "user".to_string(),
            max_total_bytes: 100,
            max_file_size: 50,
            max_file_count: 10,
            max_inodes: 20,
            current_bytes: 80,
            current_count: 5,
            updated_at: Utc::now(),
        };

        // File too large
        assert!(!quota.would_exceed(60).is_ok());

        // Would exceed total
        assert!(!quota.would_exceed(25).is_ok());

        // OK
        assert!(quota.would_exceed(10).is_ok());
    }
}
