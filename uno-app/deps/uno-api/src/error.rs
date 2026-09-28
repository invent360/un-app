//! Error types for uno-api.

use thiserror::Error;

/// Database operation errors.
#[derive(Debug, Error)]
pub enum DbError {
    /// Record not found.
    #[error("Record not found: {0}")]
    NotFound(String),

    /// Duplicate key/constraint violation.
    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),

    /// Connection error.
    #[error("Database connection error: {0}")]
    ConnectionError(String),

    /// Query execution error.
    #[error("Query error: {0}")]
    QueryError(String),

    /// Transaction error.
    #[error("Transaction error: {0}")]
    TransactionError(String),

    /// Other database errors.
    #[error("Database error: {0}")]
    Other(String),
}

/// Authentication errors.
#[derive(Debug, Error)]
pub enum AuthError {
    /// Invalid signature.
    #[error("Invalid signature")]
    InvalidSignature,

    /// Timestamp expired (replay protection).
    #[error("Request timestamp expired")]
    TimestampExpired,

    /// Unknown client ID.
    #[error("Unknown client: {0}")]
    UnknownClient(String),

    /// Missing authentication header.
    #[error("Missing authentication")]
    MissingAuth,

    /// Malformed authentication data.
    #[error("Malformed authentication: {0}")]
    MalformedAuth(String),
}

/// API operation errors.
#[derive(Debug, Error)]
pub enum ApiError {
    /// Database error.
    #[error("Database error: {0}")]
    Database(#[from] DbError),

    /// Authentication error.
    #[error("Authentication error: {0}")]
    Auth(#[from] AuthError),

    /// Validation error.
    #[error("Validation error: {0}")]
    Validation(String),

    /// CSV parsing error.
    #[error("CSV parsing error: {0}")]
    CsvParse(String),

    /// HTTP client error.
    #[error("HTTP error: {0}")]
    Http(String),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Resource not found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Conflict (e.g., duplicate).
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl ApiError {
    /// Create a validation error.
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create a not found error.
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    /// Create a conflict error.
    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict(msg.into())
    }

    /// Create an internal error.
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}
