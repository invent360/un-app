//! Import/export related models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::license::LicenseInput;

/// Request to publish multiple licenses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishLicensesRequest {
    /// Licenses to publish.
    pub licenses: Vec<LicenseInput>,
    /// Optional idempotency key to prevent duplicate processing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

impl PublishLicensesRequest {
    /// Create a new publish request.
    pub fn new(licenses: Vec<LicenseInput>) -> Self {
        Self {
            licenses,
            idempotency_key: None,
        }
    }

    /// Add an idempotency key.
    pub fn with_idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());
        self
    }
}

/// Result of a publish operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResult {
    /// Number of licenses successfully created.
    pub created: usize,
    /// Number of licenses that failed.
    pub failed: usize,
    /// Individual errors (if any).
    pub errors: Vec<PublishError>,
    /// Timestamp of the operation.
    pub timestamp: DateTime<Utc>,
}

impl PublishResult {
    /// Create a successful result.
    pub fn success(created: usize) -> Self {
        Self {
            created,
            failed: 0,
            errors: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    /// Create a partial success result.
    pub fn partial(created: usize, failed: usize, errors: Vec<PublishError>) -> Self {
        Self {
            created,
            failed,
            errors,
            timestamp: Utc::now(),
        }
    }

    /// Check if the operation was fully successful.
    pub fn is_success(&self) -> bool {
        self.failed == 0 && self.errors.is_empty()
    }
}

/// Individual error during publish.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishError {
    /// Index of the license in the original request (0-based).
    pub index: usize,
    /// Error message.
    pub message: String,
}

impl PublishError {
    /// Create a new publish error.
    pub fn new(index: usize, message: impl Into<String>) -> Self {
        Self {
            index,
            message: message.into(),
        }
    }
}

/// CSV import request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvImportRequest {
    /// Raw CSV data.
    pub csv_data: String,
    /// Whether the CSV has a header row.
    pub has_header: bool,
}

impl CsvImportRequest {
    /// Create a new CSV import request.
    pub fn new(csv_data: impl Into<String>) -> Self {
        Self {
            csv_data: csv_data.into(),
            has_header: true,
        }
    }
}

/// Result of a CSV import.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    /// Number of rows parsed.
    pub rows_parsed: usize,
    /// Number of licenses created.
    pub licenses_created: usize,
    /// Number of rows that failed.
    pub rows_failed: usize,
    /// Parse/validation errors.
    pub errors: Vec<ImportError>,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

/// Individual error during CSV import.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportError {
    /// Row number (1-based for user display).
    pub row: usize,
    /// Column name or index.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    /// Error message.
    pub message: String,
}

impl ImportError {
    /// Create a new import error.
    pub fn new(row: usize, message: impl Into<String>) -> Self {
        Self {
            row,
            column: None,
            message: message.into(),
        }
    }

    /// Add column information.
    pub fn with_column(mut self, column: impl Into<String>) -> Self {
        self.column = Some(column.into());
        self
    }
}

/// Revoke request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeRequest {
    /// License IDs to revoke.
    pub license_ids: Vec<String>,
    /// Optional reason for revocation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Revoke result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeResult {
    /// Number of licenses revoked.
    pub revoked: usize,
    /// Number that failed.
    pub failed: usize,
    /// Errors (if any).
    pub errors: Vec<RevokeError>,
}

/// Individual revoke error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeError {
    /// License ID.
    pub license_id: String,
    /// Error message.
    pub message: String,
}
