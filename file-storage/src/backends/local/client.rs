//! Local filesystem storage client implementation

use async_trait::async_trait;
use std::path::PathBuf;
use tracing::{debug, info, warn};

use crate::config::LocalConfig;
use crate::error::{Result, StorageError};
use crate::traits::FileStorageClient;
use crate::types::{FileRetrievalResult, FileUploadResult, UploadError, UploadOptions};

/// Local filesystem storage client
pub struct LocalStorageClient {
    config: LocalConfig,
}

impl LocalStorageClient {
    /// Create a new local storage client
    pub fn new(config: LocalConfig) -> Result<Self> {
        // Ensure base path exists
        let base_path = PathBuf::from(&config.base_path);
        if !base_path.exists() {
            std::fs::create_dir_all(&base_path).map_err(|e| {
                StorageError::ConfigError(format!(
                    "Failed to create storage directory '{}': {}",
                    config.base_path, e
                ))
            })?;
        }

        Ok(Self { config })
    }

    /// Generate storage URL for local file
    fn storage_url(&self, relative_path: &str) -> String {
        format!("file://{}/{}", self.config.base_path, relative_path)
    }

    /// Generate display URL for local file
    fn display_url(&self, relative_path: &str) -> String {
        format!("{}/{}", self.config.base_url, relative_path)
    }

    /// Get absolute path for a relative path
    fn absolute_path(&self, relative_path: &str) -> PathBuf {
        PathBuf::from(&self.config.base_path).join(relative_path)
    }

    /// Generate a unique filename
    fn generate_filename(&self, resource_id: &str, original_name: &str, index: usize) -> String {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let extension = std::path::Path::new(original_name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("bin");

        format!("{}/{}_{}.{}", resource_id, index, timestamp, extension)
    }

    /// Validate file against configuration
    fn validate_file(
        &self,
        data: &[u8],
        mime_type: &str,
        options: &Option<UploadOptions>,
    ) -> Result<()> {
        let max_size = options
            .as_ref()
            .and_then(|o| o.max_size)
            .unwrap_or(self.config.max_file_size);

        if data.len() as u64 > max_size {
            return Err(StorageError::FileTooLarge {
                size: data.len() as u64,
                limit: max_size,
            });
        }

        // Check allowed types
        let allowed_types = if let Some(opts) = options {
            if !opts.allowed_types.is_empty() {
                &opts.allowed_types
            } else {
                &self.config.allowed_mime_types
            }
        } else {
            &self.config.allowed_mime_types
        };

        if !allowed_types.is_empty() {
            let mime_lower = mime_type.to_lowercase();
            let is_allowed = allowed_types.iter().any(|t| {
                let t_lower = t.to_lowercase();
                if t_lower.ends_with("/*") {
                    let prefix = &t_lower[..t_lower.len() - 1];
                    mime_lower.starts_with(prefix)
                } else {
                    mime_lower == t_lower
                }
            });

            if !is_allowed {
                return Err(StorageError::InvalidFileType(format!(
                    "MIME type '{}' not allowed",
                    mime_type
                )));
            }
        }

        Ok(())
    }
}

#[async_trait]
impl FileStorageClient for LocalStorageClient {
    async fn upload_files(
        &self,
        resource_id: &str,
        files: Vec<(Vec<u8>, String, String)>,
        options: Option<UploadOptions>,
    ) -> Result<FileUploadResult> {
        info!(
            "Uploading {} files for resource {} (local)",
            files.len(),
            resource_id
        );

        // Ensure resource directory exists
        let resource_dir = self.absolute_path(resource_id);
        if !resource_dir.exists() {
            std::fs::create_dir_all(&resource_dir)?;
        }

        let mut storage_urls = Vec::new();
        let mut display_urls = Vec::new();
        let mut errors = Vec::new();
        let mut total_bytes = 0u64;

        for (index, (data, filename, mime_type)) in files.into_iter().enumerate() {
            // Validate file
            if let Err(e) = self.validate_file(&data, &mime_type, &options) {
                errors.push(UploadError {
                    filename: filename.clone(),
                    error: e.to_string(),
                });
                continue;
            }

            let relative_path = self.generate_filename(resource_id, &filename, index);
            let absolute_path = self.absolute_path(&relative_path);

            match std::fs::write(&absolute_path, &data) {
                Ok(_) => {
                    info!("Successfully saved: {:?}", absolute_path);
                    total_bytes += data.len() as u64;
                    storage_urls.push(self.storage_url(&relative_path));
                    display_urls.push(self.display_url(&relative_path));
                }
                Err(e) => {
                    let error_msg = format!("Failed to save {}: {}", filename, e);
                    warn!("{}", error_msg);
                    errors.push(UploadError {
                        filename,
                        error: e.to_string(),
                    });
                }
            }
        }

        let uploaded_count = storage_urls.len();
        info!(
            "Upload complete: {} successful, {} failed",
            uploaded_count,
            errors.len()
        );

        Ok(FileUploadResult {
            storage_urls,
            display_urls,
            uploaded_count,
            total_bytes,
            errors,
            thumbnail_urls: None,
        })
    }

    async fn upload_file(
        &self,
        resource_id: &str,
        bytes: Vec<u8>,
        filename: &str,
        mime_type: &str,
    ) -> Result<String> {
        let result = self
            .upload_files(
                resource_id,
                vec![(bytes, filename.to_string(), mime_type.to_string())],
                None,
            )
            .await?;

        if let Some(url) = result.storage_urls.first() {
            Ok(url.clone())
        } else if let Some(err) = result.errors.first() {
            Err(StorageError::UploadError(err.error.clone()))
        } else {
            Err(StorageError::UploadError("Unknown upload error".into()))
        }
    }

    async fn get_files(&self, resource_id: &str) -> Result<FileRetrievalResult> {
        info!("Retrieving files for resource {} (local)", resource_id);

        let resource_dir = self.absolute_path(resource_id);

        if !resource_dir.exists() {
            return Ok(FileRetrievalResult::empty());
        }

        let mut storage_urls = Vec::new();
        let mut display_urls = Vec::new();

        let entries = std::fs::read_dir(&resource_dir)?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().and_then(|n| n.to_str());
                if let Some(name) = file_name {
                    let relative_path = format!("{}/{}", resource_id, name);
                    storage_urls.push(self.storage_url(&relative_path));
                    display_urls.push(self.display_url(&relative_path));
                }
            }
        }

        let count = storage_urls.len();
        info!("Found {} files for resource {}", count, resource_id);

        Ok(FileRetrievalResult {
            display_urls,
            storage_urls,
            count,
        })
    }

    async fn get_file(&self, storage_url: &str) -> Result<Vec<u8>> {
        let (_, relative_path) = self.parse_storage_url(storage_url).ok_or_else(|| {
            StorageError::InvalidUrl(format!("Invalid storage URL: {}", storage_url))
        })?;

        let absolute_path = self.absolute_path(&relative_path);

        if !absolute_path.exists() {
            return Err(StorageError::NotFound(format!(
                "File not found: {}",
                relative_path
            )));
        }

        std::fs::read(&absolute_path).map_err(|e| {
            StorageError::DownloadError(format!("Failed to read file: {}", e))
        })
    }

    async fn delete_files(&self, resource_id: &str) -> Result<()> {
        info!("Deleting files for resource {} (local)", resource_id);

        let resource_dir = self.absolute_path(resource_id);

        if !resource_dir.exists() {
            return Ok(());
        }

        std::fs::remove_dir_all(&resource_dir)?;
        debug!("Deleted directory: {:?}", resource_dir);

        Ok(())
    }

    async fn delete_file(&self, storage_url: &str) -> Result<()> {
        let (_, relative_path) = self.parse_storage_url(storage_url).ok_or_else(|| {
            StorageError::InvalidUrl(format!("Invalid storage URL: {}", storage_url))
        })?;

        let absolute_path = self.absolute_path(&relative_path);

        if absolute_path.exists() {
            std::fs::remove_file(&absolute_path)?;
            debug!("Deleted file: {:?}", absolute_path);
        }

        Ok(())
    }

    fn storage_to_display_urls(&self, storage_urls: &[String]) -> Vec<String> {
        storage_urls
            .iter()
            .map(|url| self.storage_to_display_url(url))
            .collect()
    }

    fn storage_to_display_url(&self, storage_url: &str) -> String {
        if let Some((_, relative_path)) = self.parse_storage_url(storage_url) {
            self.display_url(&relative_path)
        } else {
            storage_url.to_string()
        }
    }

    fn parse_storage_url(&self, storage_url: &str) -> Option<(String, String)> {
        if storage_url.starts_with("file://") {
            let path = &storage_url[7..]; // Remove "file://"

            // Check if path starts with our base path
            if path.starts_with(&self.config.base_path) {
                let relative = &path[self.config.base_path.len()..];
                let relative = relative.trim_start_matches('/');
                return Some((self.config.base_path.clone(), relative.to_string()));
            }
        }
        None
    }

    async fn is_configured(&self) -> bool {
        PathBuf::from(&self.config.base_path).exists()
    }

    fn backend_name(&self) -> &'static str {
        "local_filesystem"
    }
}
