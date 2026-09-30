//! Configuration builder for file storage backends

use crate::error::{Result, StorageError};
use std::time::Duration;

/// Storage backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StorageBackend {
    /// Google Cloud Storage
    Gcs,
    /// AWS S3
    S3,
    /// Azure Blob Storage
    Azure,
    /// Host-local persistent filesystem.
    #[default]
    Local,
}

impl StorageBackend {
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "gcs" | "google" | "google_cloud_storage" => Some(Self::Gcs),
            "s3" | "aws" | "aws_s3" => Some(Self::S3),
            "azure" | "azure_blob" => Some(Self::Azure),
            "local" | "filesystem" | "file" => Some(Self::Local),
            _ => None,
        }
    }
}

/// Configuration builder for storage clients
#[derive(Debug, Clone)]
pub struct StorageConfigBuilder {
    backend: StorageBackend,
    // GCS-specific
    gcs_bucket: Option<String>,
    gcs_service_account_path: Option<String>,
    gcs_service_account_json: Option<String>,
    // S3-specific (future)
    s3_bucket: Option<String>,
    s3_region: Option<String>,
    s3_access_key_id: Option<String>,
    s3_secret_access_key: Option<String>,
    // Local-specific
    local_base_path: Option<String>,
    local_base_url: Option<String>,
    // Common settings
    signed_url_expiration: Duration,
    max_file_size: u64,
    allowed_mime_types: Vec<String>,
    connection_timeout: Duration,
    request_timeout: Duration,
}

impl Default for StorageConfigBuilder {
    fn default() -> Self {
        Self {
            backend: StorageBackend::default(),
            gcs_bucket: None,
            gcs_service_account_path: None,
            gcs_service_account_json: None,
            s3_bucket: None,
            s3_region: None,
            s3_access_key_id: None,
            s3_secret_access_key: None,
            local_base_path: None,
            local_base_url: None,
            signed_url_expiration: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
            max_file_size: 10 * 1024 * 1024,                              // 10MB
            allowed_mime_types: default_allowed_mime_types(),
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(120),
        }
    }
}

fn default_allowed_mime_types() -> Vec<String> {
    vec![
        // Images
        "image/jpeg".into(),
        "image/png".into(),
        "image/gif".into(),
        "image/webp".into(),
        "image/avif".into(),
        "image/svg+xml".into(),
        // Documents
        "application/pdf".into(),
        "application/msword".into(),
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document".into(),
        "text/plain".into(),
        // Video
        "video/mp4".into(),
        "video/webm".into(),
        "video/quicktime".into(),
        // Audio
        "audio/mpeg".into(),
        "audio/wav".into(),
        "audio/ogg".into(),
    ]
}

impl StorageConfigBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self> {
        let mut builder = Self::new();

        // Detect backend from env
        if let Ok(backend_str) = std::env::var("FILE_STORAGE_BACKEND") {
            if let Some(backend) = StorageBackend::from_str(&backend_str) {
                builder = builder.backend(backend);
            } else {
                return Err(StorageError::ConfigError(format!(
                    "Unknown storage backend: {}",
                    backend_str
                )));
            }
        }

        // GCS config
        if let Ok(bucket) = std::env::var("GCS_BUCKET_NAME") {
            builder = builder.gcs_bucket(&bucket);
        }
        if let Ok(path) = std::env::var("GCS_SERVICE_ACCOUNT_KEY") {
            builder = builder.gcs_service_account_path(&path);
        }
        if let Ok(json) = std::env::var("GCS_SERVICE_ACCOUNT_JSON") {
            builder = builder.gcs_service_account_json(&json);
        }

        // Local config
        if let Ok(path) = std::env::var("FILE_STORAGE_LOCAL_PATH") {
            builder = builder.local_base_path(&path);
        }
        if let Ok(url) = std::env::var("FILE_STORAGE_LOCAL_URL") {
            builder = builder.local_base_url(&url);
        }

        // Common config
        if let Ok(size_str) = std::env::var("FILE_MAX_SIZE") {
            if let Ok(bytes) = size_str.parse::<u64>() {
                builder = builder.max_file_size(bytes);
            }
        }

        if let Ok(expiry_str) = std::env::var("FILE_SIGNED_URL_EXPIRY_SECS") {
            if let Ok(secs) = expiry_str.parse::<u64>() {
                builder = builder.signed_url_expiration(Duration::from_secs(secs));
            }
        }

        Ok(builder)
    }

    /// Set the storage backend
    pub fn backend(mut self, backend: StorageBackend) -> Self {
        self.backend = backend;
        self
    }

    /// Set GCS bucket name
    pub fn gcs_bucket(mut self, bucket: &str) -> Self {
        self.gcs_bucket = Some(bucket.to_string());
        self
    }

    /// Set path to GCS service account key file
    pub fn gcs_service_account_path(mut self, path: &str) -> Self {
        self.gcs_service_account_path = Some(path.to_string());
        self
    }

    /// Set GCS service account key as JSON string
    pub fn gcs_service_account_json(mut self, json: &str) -> Self {
        self.gcs_service_account_json = Some(json.to_string());
        self
    }

    /// Set local storage base path
    pub fn local_base_path(mut self, path: &str) -> Self {
        self.local_base_path = Some(path.to_string());
        self
    }

    /// Set local storage base URL (for serving files)
    pub fn local_base_url(mut self, url: &str) -> Self {
        self.local_base_url = Some(url.to_string());
        self
    }

    /// Set signed URL expiration duration
    pub fn signed_url_expiration(mut self, duration: Duration) -> Self {
        self.signed_url_expiration = duration;
        self
    }

    /// Set maximum file size in bytes
    pub fn max_file_size(mut self, bytes: u64) -> Self {
        self.max_file_size = bytes;
        self
    }

    /// Set allowed MIME types
    pub fn allowed_mime_types(mut self, types: Vec<String>) -> Self {
        self.allowed_mime_types = types;
        self
    }

    /// Set connection timeout
    pub fn connection_timeout(mut self, duration: Duration) -> Self {
        self.connection_timeout = duration;
        self
    }

    /// Set request timeout
    pub fn request_timeout(mut self, duration: Duration) -> Self {
        self.request_timeout = duration;
        self
    }

    /// Get the configured backend type
    pub fn get_backend(&self) -> StorageBackend {
        self.backend
    }

    /// Build the GCS configuration
    pub fn build_gcs_config(&self) -> Result<GcsConfig> {
        let bucket = self.gcs_bucket.clone().ok_or_else(|| {
            StorageError::ConfigError("GCS bucket name required (GCS_BUCKET_NAME)".into())
        })?;

        if self.gcs_service_account_path.is_none() && self.gcs_service_account_json.is_none() {
            return Err(StorageError::ConfigError(
                "GCS service account key required (GCS_SERVICE_ACCOUNT_KEY or GCS_SERVICE_ACCOUNT_JSON)".into(),
            ));
        }

        Ok(GcsConfig {
            bucket,
            service_account_path: self.gcs_service_account_path.clone(),
            service_account_json: self.gcs_service_account_json.clone(),
            signed_url_expiration: self.signed_url_expiration,
            max_file_size: self.max_file_size,
            allowed_mime_types: self.allowed_mime_types.clone(),
            connection_timeout: self.connection_timeout,
            request_timeout: self.request_timeout,
        })
    }

    /// Build the local storage configuration
    pub fn build_local_config(&self) -> Result<LocalConfig> {
        let base_path = self.local_base_path.clone().ok_or_else(|| {
            StorageError::ConfigError(
                "Local storage base path required (FILE_STORAGE_LOCAL_PATH)".into(),
            )
        })?;

        let base_url = self
            .local_base_url
            .clone()
            .unwrap_or_else(|| "/files".to_string());

        Ok(LocalConfig {
            base_path,
            base_url,
            max_file_size: self.max_file_size,
            allowed_mime_types: self.allowed_mime_types.clone(),
        })
    }

    /// Build the configuration for the selected backend
    pub fn build(self) -> Result<StorageConfig> {
        match self.backend {
            StorageBackend::Gcs => Ok(StorageConfig::Gcs(self.build_gcs_config()?)),
            StorageBackend::Local => Ok(StorageConfig::Local(self.build_local_config()?)),
            StorageBackend::S3 => Err(StorageError::ConfigError(
                "S3 backend not yet implemented".into(),
            )),
            StorageBackend::Azure => Err(StorageError::ConfigError(
                "Azure backend not yet implemented".into(),
            )),
        }
    }
}

/// Resolved storage configuration
#[derive(Debug, Clone)]
pub enum StorageConfig {
    /// Google Cloud Storage configuration
    Gcs(GcsConfig),
    /// Local filesystem configuration
    Local(LocalConfig),
}

/// Google Cloud Storage configuration
#[derive(Debug, Clone)]
pub struct GcsConfig {
    /// Bucket name
    pub bucket: String,
    /// Path to service account key file
    pub service_account_path: Option<String>,
    /// Service account key as JSON string
    pub service_account_json: Option<String>,
    /// Signed URL expiration duration
    pub signed_url_expiration: Duration,
    /// Maximum file size
    pub max_file_size: u64,
    /// Allowed MIME types
    pub allowed_mime_types: Vec<String>,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
}

/// Local filesystem configuration
#[derive(Debug, Clone)]
pub struct LocalConfig {
    /// Base path for file storage
    pub base_path: String,
    /// Base URL for serving files
    pub base_url: String,
    /// Maximum file size
    pub max_file_size: u64,
    /// Allowed MIME types
    pub allowed_mime_types: Vec<String>,
}
