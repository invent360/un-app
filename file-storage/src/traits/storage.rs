//! Core trait for file storage backends

use async_trait::async_trait;
use std::sync::Arc;

use crate::error::Result;
use crate::types::{FileRetrievalResult, FileUploadResult, UploadOptions};

/// Type alias for dynamic file storage client
pub type DynFileStorageClient = Arc<dyn FileStorageClient + Send + Sync>;

/// Core trait for pluggable file storage backends.
///
/// Implementations can include:
/// - Google Cloud Storage (GCS)
/// - AWS S3
/// - Azure Blob Storage
/// - Local filesystem
///
/// Each backend handles its own authentication and configuration.
/// Storage URLs use compact format (e.g., "gcs://bucket/object") for efficient storage.
/// Display URLs are browser-loadable (e.g., signed URLs for private buckets).
#[async_trait]
pub trait FileStorageClient: Send + Sync {
    /// Upload multiple files for a resource.
    ///
    /// # Arguments
    /// * `resource_id` - Unique identifier for the resource (used as folder/prefix)
    /// * `files` - Vector of tuples: (file_bytes, filename, mime_type)
    /// * `options` - Optional upload configuration
    ///
    /// # Returns
    /// Result containing storage URLs and upload statistics
    async fn upload_files(
        &self,
        resource_id: &str,
        files: Vec<(Vec<u8>, String, String)>,
        options: Option<UploadOptions>,
    ) -> Result<FileUploadResult>;

    /// Upload a single file.
    ///
    /// # Arguments
    /// * `resource_id` - Unique identifier for the resource
    /// * `bytes` - File content as bytes
    /// * `filename` - Original filename
    /// * `mime_type` - MIME type of the file
    ///
    /// # Returns
    /// Storage URL (compact format)
    async fn upload_file(
        &self,
        resource_id: &str,
        bytes: Vec<u8>,
        filename: &str,
        mime_type: &str,
    ) -> Result<String>;

    /// Get all files for a resource.
    ///
    /// # Arguments
    /// * `resource_id` - Unique identifier for the resource
    ///
    /// # Returns
    /// Result containing display URLs (browser-loadable)
    async fn get_files(&self, resource_id: &str) -> Result<FileRetrievalResult>;

    /// Get a single file's content by its storage URL.
    ///
    /// # Arguments
    /// * `storage_url` - Storage URL (e.g., "gcs://bucket/object")
    ///
    /// # Returns
    /// File content as bytes
    async fn get_file(&self, storage_url: &str) -> Result<Vec<u8>>;

    /// Delete all files for a resource.
    ///
    /// # Arguments
    /// * `resource_id` - Unique identifier for the resource
    async fn delete_files(&self, resource_id: &str) -> Result<()>;

    /// Delete a specific file by storage URL.
    ///
    /// # Arguments
    /// * `storage_url` - Storage URL of the file to delete
    async fn delete_file(&self, storage_url: &str) -> Result<()>;

    /// Convert storage URLs to display URLs.
    ///
    /// Storage URLs are compact format for database storage (e.g., "gcs://bucket/object").
    /// Display URLs are browser-loadable (e.g., signed URLs with expiration).
    ///
    /// # Arguments
    /// * `storage_urls` - Vector of storage-format URLs
    ///
    /// # Returns
    /// Vector of browser-loadable URLs
    fn storage_to_display_urls(&self, storage_urls: &[String]) -> Vec<String>;

    /// Convert a single storage URL to display URL.
    fn storage_to_display_url(&self, storage_url: &str) -> String;

    /// Parse a storage URL to extract backend-specific components.
    ///
    /// # Returns
    /// Some((bucket_or_path, object_name)) if valid, None otherwise
    fn parse_storage_url(&self, storage_url: &str) -> Option<(String, String)>;

    /// Check if the storage backend is properly configured.
    async fn is_configured(&self) -> bool;

    /// Get the name of the storage backend.
    fn backend_name(&self) -> &'static str;
}
