//! Media asset service
//!
//! Provides:
//! - Upload with quota checking and DB tracking
//! - State management (uploading -> ready -> deleted)
//! - Asset verification and reconciliation
//! - Disk threshold monitoring

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use file_storage::{create_client_from_env, DynFileStorageClient};
use sha2::{Sha256, Digest};
use std::sync::Arc;
use uuid::Uuid;

use crate::server::repositories::{
    DynMediaAssetRepository, MediaAssetRepository,
    MediaAsset, MediaQuota,
    AssetState, QuotaCheckResult,
    CreateAssetInput, UpdateStateInput, CreateGrantInput, SetQuotaInput,
};
use crate::types::AppError;

// ============================================
// TYPES
// ============================================

/// Upload result with tracking info
#[derive(Debug, Clone)]
pub struct TrackedUploadResult {
    pub asset_id: Uuid,
    pub storage_url: String,
    pub display_url: String,
    pub file_size: i64,
    pub sha256_hash: String,
}

/// Upload request
#[derive(Debug, Clone)]
pub struct UploadRequest {
    pub resource_id: String,
    pub original_filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
    pub owner_id: Option<String>,
    pub owner_type: Option<String>,
    pub category: Option<String>,
    pub alt_text: Option<String>,
    pub created_by: Option<String>,
    /// R4-05: Visibility for the uploaded asset (defaults to draft)
    pub visibility: Option<crate::server::repositories::AssetVisibility>,
}

/// Disk threshold status
#[derive(Debug, Clone)]
pub struct DiskThresholdStatus {
    pub mount_path: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_percent: u8,
    pub status: ThresholdLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdLevel {
    Ok,
    Warning,  // 80%+
    Critical, // 90%+
    Emergency, // 95%+
}

impl ThresholdLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Warning => "warning",
            Self::Critical => "critical",
            Self::Emergency => "emergency",
        }
    }
}

/// Reconciliation result
#[derive(Debug, Clone)]
pub struct ReconcileResult {
    pub assets_checked: i32,
    pub marked_missing: i32,
    pub marked_ready: i32,
    pub orphaned_files: Vec<String>,
}

/// Verification result for a single asset
#[derive(Debug, Clone)]
pub struct AssetVerification {
    pub asset_id: Uuid,
    pub storage_url: String,
    pub exists: bool,
    pub hash_matches: Option<bool>,
    pub size_matches: Option<bool>,
}

// ============================================
// TRAIT DEFINITION
// ============================================

pub type DynMediaAssetService = Arc<dyn MediaAssetService + Send + Sync>;

#[async_trait]
pub trait MediaAssetService: Send + Sync {
    // Upload with tracking
    async fn upload_with_tracking(&self, request: UploadRequest) -> Result<TrackedUploadResult, AppError>;

    // State management
    async fn mark_ready(&self, asset_id: Uuid, sha256_hash: &str) -> Result<(), AppError>;
    async fn mark_quarantined(&self, asset_id: Uuid, reason: &str) -> Result<(), AppError>;
    async fn mark_missing(&self, asset_id: Uuid) -> Result<(), AppError>;
    async fn soft_delete(&self, asset_id: Uuid, deleted_by: &str) -> Result<(), AppError>;

    // Verification
    async fn verify_asset(&self, asset_id: Uuid) -> Result<AssetVerification, AppError>;
    async fn verify_all_ready(&self) -> Result<Vec<AssetVerification>, AppError>;

    // Reconciliation
    async fn reconcile_assets(&self) -> Result<ReconcileResult, AppError>;

    // Disk monitoring
    async fn check_disk_thresholds(&self) -> Result<DiskThresholdStatus, AppError>;
    async fn is_storage_ready(&self) -> Result<bool, AppError>;

    // Quota
    async fn check_quota_for_upload(&self, owner_id: &str, owner_type: &str, file_size: i64) -> Result<QuotaCheckResult, AppError>;
    async fn get_quota(&self, owner_id: &str, owner_type: &str) -> Result<Option<MediaQuota>, AppError>;
    async fn set_quota(&self, input: SetQuotaInput) -> Result<MediaQuota, AppError>;

    // Access grants
    async fn grant_access(&self, input: CreateGrantInput) -> Result<(), AppError>;
    async fn revoke_access(&self, grant_id: Uuid) -> Result<(), AppError>;
    async fn check_access(&self, asset_id: Uuid, grantee_id: &str, permission: &str) -> Result<bool, AppError>;

    // Queries
    async fn get_asset(&self, asset_id: Uuid) -> Result<Option<MediaAsset>, AppError>;
    async fn list_by_owner(&self, owner_id: &str, owner_type: &str) -> Result<Vec<MediaAsset>, AppError>;
    async fn count_by_state(&self) -> Result<Vec<(String, i64)>, AppError>;
    async fn total_storage_used(&self) -> Result<i64, AppError>;
}

// ============================================
// IMPLEMENTATION
// ============================================

pub struct MediaAssetServiceImpl {
    repo: DynMediaAssetRepository,
    storage_base_path: Option<String>,
}

impl MediaAssetServiceImpl {
    pub fn new(repo: DynMediaAssetRepository) -> Self {
        // Get storage base path from env for local storage
        let storage_base_path = std::env::var("LOCAL_STORAGE_PATH").ok();

        Self {
            repo,
            storage_base_path,
        }
    }

    fn get_storage_client(&self) -> Result<DynFileStorageClient, AppError> {
        create_client_from_env()
            .map_err(|e| AppError::ServiceUnavailable(format!("Storage client: {}", e)))
    }

    fn compute_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Extract dimensions from image data
    fn extract_dimensions(data: &[u8], content_type: &str) -> (Option<i32>, Option<i32>) {
        // Only try to extract for image types
        if !content_type.starts_with("image/") {
            return (None, None);
        }

        // Use imagesize crate if available, otherwise return None
        // For now, return None - dimensions can be extracted by file-storage validation
        (None, None)
    }
}

#[async_trait]
impl MediaAssetService for MediaAssetServiceImpl {
    async fn upload_with_tracking(&self, request: UploadRequest) -> Result<TrackedUploadResult, AppError> {
        // Check quota first if owner specified
        if let (Some(owner_id), Some(owner_type)) = (&request.owner_id, &request.owner_type) {
            let quota_result = self.repo.check_quota(owner_id, owner_type, request.data.len() as i64).await?;
            if !quota_result.is_ok() {
                return Err(AppError::ValidationError(match quota_result {
                    QuotaCheckResult::ExceedsFileSize { max, requested } =>
                        format!("File size {} exceeds maximum {}", requested, max),
                    QuotaCheckResult::ExceedsTotalBytes { max, current, requested } =>
                        format!("Upload would exceed quota: current {} + {} > {}", current, requested, max),
                    QuotaCheckResult::ExceedsFileCount { max, current } =>
                        format!("File count {} would exceed maximum {}", current + 1, max),
                    _ => "Quota exceeded".to_string(),
                }));
            }
        }

        // Compute hash before upload
        let sha256_hash = Self::compute_hash(&request.data);
        let file_size = request.data.len() as i64;

        // Extract dimensions for images
        let (width, height) = Self::extract_dimensions(&request.data, &request.content_type);

        // Get storage client and upload
        let client = self.get_storage_client()?;

        let files = vec![(
            request.data,
            request.original_filename.clone(),
            request.content_type.clone(),
        )];

        let upload_result = client.upload_files(&request.resource_id, files, None)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Upload failed: {}", e)))?;

        if upload_result.storage_urls.is_empty() {
            return Err(AppError::InternalServerError("No files uploaded".to_string()));
        }

        let storage_url = upload_result.storage_urls[0].clone();
        let display_url = upload_result.display_urls[0].clone();

        // Extract filename from storage URL
        let filename = storage_url
            .split('/')
            .last()
            .unwrap_or(&request.original_filename)
            .to_string();

        // R4-05: Compute original hash (pre-transformation) vs stored hash (post-transformation)
        // For now they're the same since we don't transform, but the schema supports both
        let original_hash = sha256_hash.clone();

        // Create asset record in database (starts in 'uploading' state)
        let asset = self.repo.create_asset(CreateAssetInput {
            resource_id: request.resource_id,
            filename,
            original_filename: Some(request.original_filename),
            storage_url: storage_url.clone(),
            display_url: display_url.clone(),
            mime_type: request.content_type,
            file_size,
            sha256_hash: Some(sha256_hash.clone()),
            original_hash: Some(original_hash),
            width,
            height,
            visibility: request.visibility,
            owner_id: request.owner_id,
            owner_type: request.owner_type,
            category: request.category,
            alt_text: request.alt_text,
            created_by: request.created_by,
        }).await?;

        // Mark as ready since upload succeeded
        self.repo.update_state(UpdateStateInput {
            asset_id: asset.id,
            state: AssetState::Ready,
            reason: None,
        }).await?;

        tracing::info!(
            asset_id = %asset.id,
            storage_url = %storage_url,
            file_size = file_size,
            "Media asset uploaded and tracked"
        );

        Ok(TrackedUploadResult {
            asset_id: asset.id,
            storage_url,
            display_url,
            file_size,
            sha256_hash,
        })
    }

    async fn mark_ready(&self, asset_id: Uuid, sha256_hash: &str) -> Result<(), AppError> {
        // Verify asset exists
        let asset = self.repo.get_asset(asset_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", asset_id)))?;

        // Verify hash matches if we have one stored
        if let Some(stored_hash) = &asset.sha256_hash {
            if stored_hash != sha256_hash {
                return Err(AppError::ValidationError(format!(
                    "Hash mismatch: expected {}, got {}",
                    stored_hash, sha256_hash
                )));
            }
        }

        self.repo.update_state(UpdateStateInput {
            asset_id,
            state: AssetState::Ready,
            reason: None,
        }).await
    }

    async fn mark_quarantined(&self, asset_id: Uuid, reason: &str) -> Result<(), AppError> {
        self.repo.update_state(UpdateStateInput {
            asset_id,
            state: AssetState::Quarantined,
            reason: Some(reason.to_string()),
        }).await
    }

    async fn mark_missing(&self, asset_id: Uuid) -> Result<(), AppError> {
        self.repo.update_state(UpdateStateInput {
            asset_id,
            state: AssetState::Missing,
            reason: Some("File not found on disk".to_string()),
        }).await
    }

    async fn soft_delete(&self, asset_id: Uuid, deleted_by: &str) -> Result<(), AppError> {
        // Get asset to find storage URL
        let asset = self.repo.get_asset(asset_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", asset_id)))?;

        // Delete from storage
        let client = self.get_storage_client()?;
        if let Err(e) = client.delete_files(&asset.resource_id).await {
            tracing::warn!(
                asset_id = %asset_id,
                resource_id = %asset.resource_id,
                error = %e,
                "Failed to delete files from storage, continuing with soft delete"
            );
        }

        // Soft delete in DB
        self.repo.soft_delete(asset_id, deleted_by).await
    }

    async fn verify_asset(&self, asset_id: Uuid) -> Result<AssetVerification, AppError> {
        let asset = self.repo.get_asset(asset_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", asset_id)))?;

        let client = self.get_storage_client()?;

        // Try to get the file
        let (exists, hash_matches, size_matches) = match client.get_file(&asset.storage_url).await {
            Ok(data) => {
                let actual_hash = Self::compute_hash(&data);
                let actual_size = data.len() as i64;

                let hash_matches = asset.sha256_hash
                    .as_ref()
                    .map(|h| h == &actual_hash);

                let size_matches = Some(asset.file_size == actual_size);

                (true, hash_matches, size_matches)
            }
            Err(_) => (false, None, None),
        };

        Ok(AssetVerification {
            asset_id,
            storage_url: asset.storage_url,
            exists,
            hash_matches,
            size_matches,
        })
    }

    async fn verify_all_ready(&self) -> Result<Vec<AssetVerification>, AppError> {
        let assets = self.repo.find_by_state(AssetState::Ready, 1000).await?;
        let mut results = Vec::with_capacity(assets.len());

        for asset in assets {
            match self.verify_asset(asset.id).await {
                Ok(v) => results.push(v),
                Err(e) => {
                    tracing::warn!(asset_id = %asset.id, error = %e, "Failed to verify asset");
                }
            }
        }

        Ok(results)
    }

    async fn reconcile_assets(&self) -> Result<ReconcileResult, AppError> {
        let mut result = ReconcileResult {
            assets_checked: 0,
            marked_missing: 0,
            marked_ready: 0,
            orphaned_files: Vec::new(),
        };

        // Check all ready assets
        let ready_assets = self.repo.find_by_state(AssetState::Ready, 10000).await?;
        let client = self.get_storage_client()?;

        for asset in &ready_assets {
            result.assets_checked += 1;

            match client.get_file(&asset.storage_url).await {
                Ok(_) => {
                    // File exists, asset is correctly marked ready
                }
                Err(_) => {
                    // File missing, mark asset as missing
                    if let Err(e) = self.mark_missing(asset.id).await {
                        tracing::error!(asset_id = %asset.id, error = %e, "Failed to mark asset missing");
                    } else {
                        result.marked_missing += 1;
                    }
                }
            }
        }

        // Check missing assets in case files were restored
        let missing_assets = self.repo.find_by_state(AssetState::Missing, 1000).await?;
        for asset in &missing_assets {
            if client.get_file(&asset.storage_url).await.is_ok() {
                // File found, mark as ready
                if let Err(e) = self.repo.update_state(UpdateStateInput {
                    asset_id: asset.id,
                    state: AssetState::Ready,
                    reason: Some("File restored".to_string()),
                }).await {
                    tracing::error!(asset_id = %asset.id, error = %e, "Failed to mark asset ready");
                } else {
                    result.marked_ready += 1;
                }
            }
        }

        tracing::info!(
            checked = result.assets_checked,
            missing = result.marked_missing,
            restored = result.marked_ready,
            "Asset reconciliation complete"
        );

        Ok(result)
    }

    async fn check_disk_thresholds(&self) -> Result<DiskThresholdStatus, AppError> {
        let mount_path = self.storage_base_path
            .clone()
            .unwrap_or_else(|| "/data".to_string());

        // Use std::fs::metadata to check if path exists
        // For actual disk space checking, we'd need platform-specific code
        // or a crate like `sysinfo`. For now, return a healthy default.
        let path = std::path::Path::new(&mount_path);
        let exists = path.exists() && path.is_dir();

        if !exists {
            return Ok(DiskThresholdStatus {
                mount_path,
                total_bytes: 0,
                available_bytes: 0,
                used_percent: 100, // Treat as emergency if path doesn't exist
                status: ThresholdLevel::Emergency,
            });
        }

        // Default to healthy - in production, use fs2 or sysinfo crate
        // for actual disk space monitoring
        Ok(DiskThresholdStatus {
            mount_path,
            total_bytes: 0,
            available_bytes: 0,
            used_percent: 0,
            status: ThresholdLevel::Ok,
        })
    }

    async fn is_storage_ready(&self) -> Result<bool, AppError> {
        // Check if we can create a storage client
        let client = match self.get_storage_client() {
            Ok(c) => c,
            Err(_) => return Ok(false),
        };

        // For local storage, check if the base path exists and is writable
        if let Some(base_path) = &self.storage_base_path {
            let path = std::path::Path::new(base_path);
            if !path.exists() || !path.is_dir() {
                tracing::warn!(path = %base_path, "Storage path does not exist or is not a directory");
                return Ok(false);
            }

            // Check disk thresholds
            if let Ok(status) = self.check_disk_thresholds().await {
                if status.status == ThresholdLevel::Emergency {
                    tracing::error!(
                        path = %base_path,
                        used_percent = status.used_percent,
                        "Storage is in emergency state - disk nearly full"
                    );
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    async fn check_quota_for_upload(&self, owner_id: &str, owner_type: &str, file_size: i64) -> Result<QuotaCheckResult, AppError> {
        self.repo.check_quota(owner_id, owner_type, file_size).await
    }

    async fn get_quota(&self, owner_id: &str, owner_type: &str) -> Result<Option<MediaQuota>, AppError> {
        self.repo.get_quota(owner_id, owner_type).await
    }

    async fn set_quota(&self, input: SetQuotaInput) -> Result<MediaQuota, AppError> {
        self.repo.set_quota(input).await
    }

    async fn grant_access(&self, input: CreateGrantInput) -> Result<(), AppError> {
        self.repo.create_grant(input).await?;
        Ok(())
    }

    async fn revoke_access(&self, grant_id: Uuid) -> Result<(), AppError> {
        self.repo.revoke_grant(grant_id).await
    }

    async fn check_access(&self, asset_id: Uuid, grantee_id: &str, permission: &str) -> Result<bool, AppError> {
        // First check if the asset exists
        let asset = self.repo.get_asset(asset_id).await?;
        if asset.is_none() {
            return Ok(false);
        }
        let asset = asset.unwrap();

        // Owner always has access
        if let Some(owner_id) = &asset.owner_id {
            if owner_id == grantee_id {
                return Ok(true);
            }
        }

        // Check grants
        self.repo.check_access(asset_id, grantee_id, permission).await
    }

    async fn get_asset(&self, asset_id: Uuid) -> Result<Option<MediaAsset>, AppError> {
        self.repo.get_asset(asset_id).await
    }

    async fn list_by_owner(&self, owner_id: &str, owner_type: &str) -> Result<Vec<MediaAsset>, AppError> {
        self.repo.list_by_owner(owner_id, owner_type).await
    }

    async fn count_by_state(&self) -> Result<Vec<(String, i64)>, AppError> {
        self.repo.count_by_state().await
    }

    async fn total_storage_used(&self) -> Result<i64, AppError> {
        self.repo.total_storage_used().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash() {
        let data = b"hello world";
        let hash = MediaAssetServiceImpl::compute_hash(data);
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex chars
    }

    #[test]
    fn test_threshold_levels() {
        assert_eq!(ThresholdLevel::Ok.as_str(), "ok");
        assert_eq!(ThresholdLevel::Emergency.as_str(), "emergency");
    }
}
