//! # File Storage
//!
//! A modular cloud file storage library with pluggable backends.
//!
//! ## Features
//!
//! - **Pluggable backends**: Google Cloud Storage, AWS S3 (planned), Azure (planned), local filesystem
//! - **Trait-based abstraction**: Swap backends without changing application code
//! - **Secure by default**: Private buckets with signed URLs
//! - **Environment-based configuration**: Easy deployment configuration
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use file_storage::{create_client_from_env, FileStorageClient};
//!
//! #[tokio::main]
//! async fn main() -> file_storage::Result<()> {
//!     // Create client from environment variables
//!     let client = create_client_from_env()?;
//!
//!     // Upload a file
//!     let storage_url = client.upload_file(
//!         "resource-123",
//!         include_bytes!("../test.jpg").to_vec(),
//!         "test.jpg",
//!         "image/jpeg"
//!     ).await?;
//!
//!     // Get display URL for browser
//!     let display_url = client.storage_to_display_url(&storage_url);
//!     println!("File available at: {}", display_url);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! ### Google Cloud Storage
//!
//! ```bash
//! export FILE_STORAGE_BACKEND=gcs
//! export GCS_BUCKET_NAME=my-bucket
//! export GCS_SERVICE_ACCOUNT_KEY=/path/to/service-account.json
//! ```
//!
//! ### Local Filesystem
//!
//! ```bash
//! export FILE_STORAGE_BACKEND=local
//! export FILE_STORAGE_LOCAL_PATH=/tmp/file-storage
//! export FILE_STORAGE_LOCAL_URL=/files  # URL prefix for serving
//! ```
//!
//! ## Storage URLs
//!
//! The library uses two URL formats:
//!
//! - **Storage URLs**: Compact format for database storage (e.g., `gcs://bucket/path/file.jpg`)
//! - **Display URLs**: Browser-loadable URLs (e.g., signed URLs with expiration)
//!
//! Use `storage_to_display_url()` to convert for browser access.

pub mod backends;
pub mod config;
pub mod error;
pub mod factory;
pub mod traits;
pub mod types;

// Re-export main types at crate root
pub use config::{GcsConfig, LocalConfig, StorageBackend, StorageConfig, StorageConfigBuilder};
pub use error::{Result, StorageError};
pub use factory::{
    create_client, create_client_from_env, create_client_with_builder, detect_configured_backend,
    is_backend_configured,
};
pub use traits::{DynFileStorageClient, FileStorageClient};
pub use types::{
    FileCategory, FileInfo, FileMetadata, FileRetrievalResult, FileUploadResult, FileVisibility,
    UploadError, UploadOptions,
};

// Re-export backend clients when features are enabled
#[cfg(feature = "gcs")]
pub use backends::gcs::GoogleCloudStorageClient;

#[cfg(feature = "local")]
pub use backends::local::LocalStorageClient;
