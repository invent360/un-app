//! Factory for creating storage clients

use crate::config::{StorageBackend, StorageConfig, StorageConfigBuilder};
use crate::error::{Result, StorageError};
use crate::traits::DynFileStorageClient;
use std::sync::Arc;

/// Create a storage client from configuration
pub fn create_client(config: StorageConfig) -> Result<DynFileStorageClient> {
    match config {
        #[cfg(feature = "gcs")]
        StorageConfig::Gcs(gcs_config) => {
            let client = crate::backends::gcs::GoogleCloudStorageClient::new(gcs_config)?;
            Ok(Arc::new(client))
        }
        #[cfg(not(feature = "gcs"))]
        StorageConfig::Gcs(_) => Err(StorageError::ConfigError(
            "GCS feature not enabled. Enable with `features = [\"gcs\"]`".into(),
        )),
        #[cfg(feature = "local")]
        StorageConfig::Local(local_config) => {
            let client = crate::backends::local::LocalStorageClient::new(local_config)?;
            Ok(Arc::new(client))
        }
        #[cfg(not(feature = "local"))]
        StorageConfig::Local(_) => Err(StorageError::ConfigError(
            "Local feature not enabled. Enable with `features = [\"local\"]`".into(),
        )),
    }
}

/// Create a storage client from environment variables
pub fn create_client_from_env() -> Result<DynFileStorageClient> {
    let config = StorageConfigBuilder::from_env()?.build()?;
    create_client(config)
}

/// Create a storage client using the builder
pub fn create_client_with_builder<F>(builder_fn: F) -> Result<DynFileStorageClient>
where
    F: FnOnce(StorageConfigBuilder) -> StorageConfigBuilder,
{
    let builder = builder_fn(StorageConfigBuilder::new());
    let config = builder.build()?;
    create_client(config)
}

/// Check if a storage backend is configured via environment variables
pub fn is_backend_configured(backend: StorageBackend) -> bool {
    match backend {
        StorageBackend::Gcs => {
            std::env::var("GCS_BUCKET_NAME").is_ok()
                && (std::env::var("GCS_SERVICE_ACCOUNT_KEY").is_ok()
                    || std::env::var("GCS_SERVICE_ACCOUNT_JSON").is_ok())
        }
        StorageBackend::Local => std::env::var("FILE_STORAGE_LOCAL_PATH").is_ok(),
        StorageBackend::S3 => {
            std::env::var("AWS_S3_BUCKET").is_ok()
                && std::env::var("AWS_ACCESS_KEY_ID").is_ok()
                && std::env::var("AWS_SECRET_ACCESS_KEY").is_ok()
        }
        StorageBackend::Azure => {
            std::env::var("AZURE_STORAGE_ACCOUNT").is_ok()
                && std::env::var("AZURE_STORAGE_KEY").is_ok()
        }
    }
}

/// Get the configured backend from environment, or None if not configured
pub fn detect_configured_backend() -> Option<StorageBackend> {
    // Check explicit backend setting first
    if let Ok(backend_str) = std::env::var("FILE_STORAGE_BACKEND") {
        return StorageBackend::from_str(&backend_str);
    }

    // Local is the sole implicit backend. Ambient cloud credentials never select it.
    if is_backend_configured(StorageBackend::Local) {
        return Some(StorageBackend::Local);
    }
    None
}
