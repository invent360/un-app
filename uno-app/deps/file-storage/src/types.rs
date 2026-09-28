//! Common types for file storage operations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// File category based on MIME type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FileCategory {
    /// Image files (jpeg, png, gif, webp, avif, svg)
    Image,
    /// Document files (pdf, doc, docx, txt)
    Document,
    /// Video files (mp4, webm, mov)
    Video,
    /// Audio files (mp3, wav, ogg)
    Audio,
    /// Other file types
    #[default]
    Other,
}

impl FileCategory {
    /// Determine category from MIME type
    pub fn from_mime_type(mime_type: &str) -> Self {
        let mime_lower = mime_type.to_lowercase();
        if mime_lower.starts_with("image/") {
            FileCategory::Image
        } else if mime_lower.starts_with("video/") {
            FileCategory::Video
        } else if mime_lower.starts_with("audio/") {
            FileCategory::Audio
        } else if mime_lower == "application/pdf"
            || mime_lower.starts_with("application/msword")
            || mime_lower.starts_with("application/vnd.openxmlformats-officedocument")
            || mime_lower == "text/plain"
        {
            FileCategory::Document
        } else {
            FileCategory::Other
        }
    }

    /// Get common file extensions for this category
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            FileCategory::Image => &["jpg", "jpeg", "png", "gif", "webp", "avif", "svg", "bmp", "ico"],
            FileCategory::Document => &["pdf", "doc", "docx", "txt", "rtf", "odt"],
            FileCategory::Video => &["mp4", "webm", "mov", "avi", "mkv"],
            FileCategory::Audio => &["mp3", "wav", "ogg", "flac", "aac", "m4a"],
            FileCategory::Other => &[],
        }
    }
}

/// Options for file upload operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadOptions {
    /// Maximum file size in bytes (None = use default)
    pub max_size: Option<u64>,
    /// Allowed MIME types (empty = allow all configured types)
    pub allowed_types: Vec<String>,
    /// Custom metadata to store with file
    pub metadata: Option<FileMetadata>,
    /// File visibility setting
    pub visibility: FileVisibility,
    /// Generate a thumbnail for images
    pub generate_thumbnail: bool,
    /// Thumbnail dimensions (width, height)
    pub thumbnail_size: Option<(u32, u32)>,
}

/// File visibility setting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FileVisibility {
    /// Private - requires signed URL for access
    #[default]
    Private,
    /// Public - publicly accessible
    Public,
}

/// File metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Alternative text (for accessibility)
    pub alt_text: Option<String>,
    /// Caption or description
    pub caption: Option<String>,
    /// File category tag
    pub category: Option<String>,
    /// Custom tags
    pub tags: Vec<String>,
    /// User who uploaded the file
    pub uploaded_by: Option<String>,
    /// Custom key-value metadata
    pub custom: HashMap<String, String>,
}

/// Result of uploading files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResult {
    /// Storage URLs (compact format: gcs://bucket/object)
    pub storage_urls: Vec<String>,
    /// Display URLs (browser-loadable, may be signed)
    pub display_urls: Vec<String>,
    /// Number of files successfully uploaded
    pub uploaded_count: usize,
    /// Total bytes uploaded
    pub total_bytes: u64,
    /// Errors for failed uploads
    pub errors: Vec<UploadError>,
    /// Thumbnail URLs if generated
    pub thumbnail_urls: Option<Vec<String>>,
}

impl FileUploadResult {
    /// Create an empty result
    pub fn empty() -> Self {
        Self {
            storage_urls: Vec::new(),
            display_urls: Vec::new(),
            uploaded_count: 0,
            total_bytes: 0,
            errors: Vec::new(),
            thumbnail_urls: None,
        }
    }

    /// Check if all uploads succeeded
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }

    /// Check if some uploads failed
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

/// Error information for a failed upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadError {
    /// Original filename
    pub filename: String,
    /// Error message
    pub error: String,
}

/// Result of retrieving files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRetrievalResult {
    /// Display URLs (browser-loadable)
    pub display_urls: Vec<String>,
    /// Storage URLs (for reference)
    pub storage_urls: Vec<String>,
    /// Total count
    pub count: usize,
}

impl FileRetrievalResult {
    /// Create an empty result
    pub fn empty() -> Self {
        Self {
            display_urls: Vec::new(),
            storage_urls: Vec::new(),
            count: 0,
        }
    }
}

/// Information about a stored file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// Storage URL (compact format)
    pub storage_url: String,
    /// Display URL (browser-loadable)
    pub display_url: String,
    /// Original filename
    pub filename: String,
    /// MIME type
    pub mime_type: String,
    /// File size in bytes
    pub size_bytes: u64,
    /// File category
    pub category: FileCategory,
    /// When the file was uploaded
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// File metadata
    pub metadata: Option<FileMetadata>,
}
