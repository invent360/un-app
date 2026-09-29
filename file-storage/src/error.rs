//! Error types for file storage operations

use thiserror::Error;

/// Errors that can occur during file storage operations
#[derive(Error, Debug)]
pub enum StorageError {
    /// Storage backend is not configured
    #[error("Storage backend not configured: {0}")]
    NotConfigured(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    /// Authorization failed
    #[error("Authorization failed: {0}")]
    AuthorizationError(String),

    /// Upload operation failed
    #[error("Upload failed: {0}")]
    UploadError(String),

    /// Download operation failed
    #[error("Download failed: {0}")]
    DownloadError(String),

    /// Delete operation failed
    #[error("Delete failed: {0}")]
    DeleteError(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Invalid URL format
    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),

    /// File exceeds size limit
    #[error("File too large: {size} bytes exceeds limit of {limit} bytes")]
    FileTooLarge { size: u64, limit: u64 },

    /// Invalid file type
    #[error("Invalid file type: {0}")]
    InvalidFileType(String),

    /// Path traversal attack detected
    #[error("Path traversal rejected: {0}")]
    PathTraversalRejected(String),

    /// Invalid resource identifier
    #[error("Invalid resource ID: {0}")]
    InvalidResourceId(String),

    /// Symlink attack detected
    #[error("Symlink not allowed: {0}")]
    SymlinkRejected(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Operation timed out
    #[error("Timeout: {0}")]
    Timeout(String),

    /// Rate limited by backend
    #[error("Rate limited: retry after {retry_after_secs} seconds")]
    RateLimited { retry_after_secs: u64 },

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// File processing error
    #[error("File processing error: {0}")]
    ProcessingError(String),

    /// Internal error
    #[error("Internal error: {0}")]
    InternalError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(String),
}

impl StorageError {
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::NetworkError(_) | Self::Timeout(_) | Self::RateLimited { .. }
        )
    }

    /// Get HTTP status code equivalent
    pub fn status_code(&self) -> u16 {
        match self {
            Self::NotConfigured(_) | Self::ConfigError(_) => 500,
            Self::AuthenticationError(_) => 401,
            Self::AuthorizationError(_) => 403,
            Self::NotFound(_) => 404,
            Self::InvalidUrl(_) | Self::InvalidFileType(_) | Self::InvalidResourceId(_) => 400,
            Self::PathTraversalRejected(_) | Self::SymlinkRejected(_) => 403,
            Self::FileTooLarge { .. } => 413,
            Self::RateLimited { .. } => 429,
            Self::Timeout(_) => 504,
            _ => 500,
        }
    }
}

impl From<reqwest::Error> for StorageError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            StorageError::Timeout(err.to_string())
        } else if err.is_connect() {
            StorageError::NetworkError(format!("Connection failed: {}", err))
        } else {
            StorageError::NetworkError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerializationError(err.to_string())
    }
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::IoError(err.to_string())
    }
}

/// Result type alias for storage operations
pub type Result<T> = std::result::Result<T, StorageError>;
