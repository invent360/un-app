//! Storage backend implementations

#[cfg(feature = "gcs")]
pub mod gcs;

#[cfg(feature = "local")]
pub mod local;

#[cfg(feature = "gcs")]
pub use gcs::GoogleCloudStorageClient;

#[cfg(feature = "local")]
pub use local::LocalStorageClient;
