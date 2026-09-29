//! Local filesystem storage client implementation
//!
//! SECURITY: This module implements path traversal protection, symlink rejection,
//! and resource ID validation to prevent directory escape attacks.

use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

use crate::config::LocalConfig;
use crate::error::{Result, StorageError};
use crate::traits::FileStorageClient;
use crate::types::{FileRetrievalResult, FileUploadResult, UploadError, UploadOptions};

/// Allowed characters in resource IDs (alphanumeric, dash, underscore)
const RESOURCE_ID_PATTERN: &[char] = &['-', '_'];

/// Allowed file extensions (whitelist)
const ALLOWED_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "webp", "avif", "svg",
    "pdf", "doc", "docx", "txt", "csv", "xls", "xlsx",
    "mp4", "webm", "mov", "mp3", "wav", "ogg",
    "json", "xml", "bin",
];

/// Local filesystem storage client
pub struct LocalStorageClient {
    config: LocalConfig,
    /// Canonicalized base path for containment checks
    canonical_base: PathBuf,
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

        // Canonicalize the base path once at startup for security checks
        let canonical_base = base_path.canonicalize().map_err(|e| {
            StorageError::ConfigError(format!(
                "Failed to canonicalize base path '{}': {}",
                config.base_path, e
            ))
        })?;

        Ok(Self {
            config,
            canonical_base,
        })
    }

    /// Validate a resource ID to prevent path traversal via malicious IDs
    ///
    /// Resource IDs must be:
    /// - Non-empty
    /// - Alphanumeric with optional dashes and underscores
    /// - No path separators, dots, or other special characters
    fn validate_resource_id(&self, resource_id: &str) -> Result<()> {
        if resource_id.is_empty() {
            return Err(StorageError::InvalidResourceId(
                "Resource ID cannot be empty".into(),
            ));
        }

        // Check for dangerous patterns
        if resource_id.contains("..") || resource_id.contains('/') || resource_id.contains('\\') {
            return Err(StorageError::PathTraversalRejected(format!(
                "Resource ID contains forbidden characters: {}",
                resource_id
            )));
        }

        // Validate each character is allowed
        for c in resource_id.chars() {
            if !c.is_alphanumeric() && !RESOURCE_ID_PATTERN.contains(&c) {
                return Err(StorageError::InvalidResourceId(format!(
                    "Invalid character '{}' in resource ID",
                    c
                )));
            }
        }

        Ok(())
    }

    /// Validate a filename to prevent path traversal and dangerous extensions
    fn validate_filename(&self, filename: &str) -> Result<String> {
        if filename.is_empty() {
            return Err(StorageError::InvalidResourceId(
                "Filename cannot be empty".into(),
            ));
        }

        // Check for traversal patterns
        if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
            return Err(StorageError::PathTraversalRejected(format!(
                "Filename contains forbidden characters: {}",
                filename
            )));
        }

        // Extract and validate extension
        let extension = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_else(|| "bin".to_string());

        if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
            return Err(StorageError::InvalidFileType(format!(
                "Extension '{}' not allowed",
                extension
            )));
        }

        Ok(extension)
    }

    /// Compute a safe absolute path and verify it's contained within the base directory.
    ///
    /// SECURITY: This is the core path traversal prevention. It:
    /// 1. Joins the relative path to the base path
    /// 2. Canonicalizes the result (resolves .., symlinks, etc.)
    /// 3. Verifies the result starts with the canonical base path
    fn safe_path(&self, relative_path: &str) -> Result<PathBuf> {
        // Build the target path
        let target = self.canonical_base.join(relative_path);

        // For paths that don't exist yet (uploads), we need to check parent
        let check_path = if target.exists() {
            target.canonicalize()
        } else {
            // Check if parent exists and is safe
            if let Some(parent) = target.parent() {
                if parent.exists() {
                    // Canonicalize parent and append the final component
                    let canonical_parent = parent.canonicalize().map_err(|e| {
                        StorageError::PathTraversalRejected(format!(
                            "Cannot resolve parent path: {}",
                            e
                        ))
                    })?;

                    // Verify parent is within base
                    if !canonical_parent.starts_with(&self.canonical_base) {
                        return Err(StorageError::PathTraversalRejected(
                            "Path escapes storage directory".into(),
                        ));
                    }

                    // Return the path with the final component
                    if let Some(filename) = target.file_name() {
                        return Ok(canonical_parent.join(filename));
                    }
                }
            }
            // Parent doesn't exist or path is invalid - return raw path for mkdir
            Ok(target.clone())
        };

        match check_path {
            Ok(canonical) => {
                // Verify the canonical path is within our base directory
                if !canonical.starts_with(&self.canonical_base) {
                    return Err(StorageError::PathTraversalRejected(format!(
                        "Path '{}' escapes storage directory",
                        relative_path
                    )));
                }

                // Check for symlinks in existing paths
                if canonical.exists() && canonical.is_symlink() {
                    return Err(StorageError::SymlinkRejected(
                        "Symlinks are not allowed".into(),
                    ));
                }

                Ok(canonical)
            }
            Err(e) => {
                // If canonicalization fails, the path may contain invalid components
                Err(StorageError::PathTraversalRejected(format!(
                    "Invalid path: {}",
                    e
                )))
            }
        }
    }

    /// Generate storage URL for local file
    fn storage_url(&self, relative_path: &str) -> String {
        format!("file://{}/{}", self.config.base_path, relative_path)
    }

    /// Generate display URL for local file
    fn display_url(&self, relative_path: &str) -> String {
        format!("{}/{}", self.config.base_url, relative_path)
    }

    /// Generate a unique filename with validated components
    fn generate_filename(&self, resource_id: &str, original_name: &str, index: usize) -> Result<String> {
        // Validate resource ID
        self.validate_resource_id(resource_id)?;

        // Validate and extract extension from original name
        let extension = self.validate_filename(original_name)?;

        let timestamp = chrono::Utc::now().timestamp_millis();
        Ok(format!("{}/{}_{}.{}", resource_id, index, timestamp, extension))
    }

    /// Detect MIME type from file magic bytes (content-based validation)
    ///
    /// SECURITY: This validates actual file content rather than trusting client-provided
    /// MIME types, preventing malicious file uploads disguised as safe types.
    fn detect_mime_from_magic(&self, data: &[u8]) -> Option<&'static str> {
        if data.len() < 8 {
            return None;
        }

        // JPEG: FF D8 FF
        if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return Some("image/jpeg");
        }

        // PNG: 89 50 4E 47 0D 0A 1A 0A
        if data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
            return Some("image/png");
        }

        // GIF: 47 49 46 38 (GIF8)
        if data.starts_with(&[0x47, 0x49, 0x46, 0x38]) {
            return Some("image/gif");
        }

        // WebP: RIFF....WEBP
        if data.len() >= 12 && data.starts_with(b"RIFF") && &data[8..12] == b"WEBP" {
            return Some("image/webp");
        }

        // PDF: %PDF
        if data.starts_with(b"%PDF") {
            return Some("application/pdf");
        }

        // MP4/MOV: ....ftyp (file type box)
        if data.len() >= 8 && &data[4..8] == b"ftyp" {
            return Some("video/mp4");
        }

        // MP3: ID3 tag or MPEG sync
        if data.starts_with(&[0x49, 0x44, 0x33]) || // ID3
           (data.len() >= 2 && data[0] == 0xFF && (data[1] & 0xE0) == 0xE0) {
            return Some("audio/mpeg");
        }

        // WAV: RIFF....WAVE
        if data.len() >= 12 && data.starts_with(b"RIFF") && &data[8..12] == b"WAVE" {
            return Some("audio/wav");
        }

        // OGG: OggS
        if data.starts_with(b"OggS") {
            return Some("audio/ogg");
        }

        // WebM: EBML header with webm doctype
        if data.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
            return Some("video/webm");
        }

        // AVIF: ....ftypavif
        if data.len() >= 12 && &data[4..8] == b"ftyp" && &data[8..12] == b"avif" {
            return Some("image/avif");
        }

        // Plain text files (JSON, CSV, TXT, XML) - check for printable ASCII
        if data.iter().take(512).all(|&b| b.is_ascii() || b == b'\n' || b == b'\r' || b == b'\t') {
            if data.starts_with(b"{") || data.starts_with(b"[") {
                return Some("application/json");
            }
            if data.starts_with(b"<?xml") {
                return Some("application/xml");
            }
            return Some("text/plain");
        }

        None
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

        // SECURITY: Content-based MIME type validation
        // Detect actual file type from magic bytes
        let detected_mime = self.detect_mime_from_magic(data);
        let mime_lower = mime_type.to_lowercase();

        // For binary files, verify the claimed type matches detected type
        if let Some(detected) = detected_mime {
            let detected_category = detected.split('/').next().unwrap_or("");
            let claimed_category = mime_lower.split('/').next().unwrap_or("");

            // Reject if claimed category doesn't match detected category
            // (e.g., claiming image/* but uploading executable)
            if !claimed_category.is_empty()
                && !detected_category.is_empty()
                && claimed_category != detected_category
                && detected_category != "text" // Allow text detection for various formats
            {
                warn!(
                    "MIME type mismatch: claimed '{}', detected '{}'",
                    mime_type, detected
                );
                return Err(StorageError::InvalidFileType(format!(
                    "Content type mismatch: claimed '{}' but detected '{}'",
                    mime_type, detected
                )));
            }
        }

        // SECURITY: Reject SVG files (potential XSS vector) unless explicitly allowed
        if mime_lower == "image/svg+xml" || mime_lower.contains("svg") {
            // Check if SVG is explicitly in allowed types
            let svg_allowed = options
                .as_ref()
                .and_then(|o| {
                    if o.allowed_types.iter().any(|t| t.to_lowercase().contains("svg")) {
                        Some(true)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| {
                    self.config.allowed_mime_types.iter().any(|t| t.to_lowercase().contains("svg"))
                });

            if !svg_allowed {
                return Err(StorageError::InvalidFileType(
                    "SVG files are not allowed (potential XSS risk)".into(),
                ));
            }

            // For SVG, also check content for dangerous patterns
            if let Ok(content) = std::str::from_utf8(data) {
                let content_lower = content.to_lowercase();
                if content_lower.contains("<script")
                    || content_lower.contains("javascript:")
                    || content_lower.contains("onerror")
                    || content_lower.contains("onload")
                    || content_lower.contains("onclick")
                {
                    return Err(StorageError::InvalidFileType(
                        "SVG contains potentially dangerous content".into(),
                    ));
                }
            }
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

        // SECURITY: Validate resource ID before any filesystem operations
        self.validate_resource_id(resource_id)?;

        // SECURITY: Use safe_path to ensure directory is within base
        let resource_dir = self.safe_path(resource_id)?;
        if !resource_dir.exists() {
            std::fs::create_dir_all(&resource_dir)?;
        }

        let mut storage_urls = Vec::new();
        let mut display_urls = Vec::new();
        let mut errors = Vec::new();
        let mut total_bytes = 0u64;

        for (index, (data, filename, mime_type)) in files.into_iter().enumerate() {
            // Validate file content and MIME type
            if let Err(e) = self.validate_file(&data, &mime_type, &options) {
                errors.push(UploadError {
                    filename: filename.clone(),
                    error: e.to_string(),
                });
                continue;
            }

            // SECURITY: Generate validated filename (checks extension, resource ID)
            let relative_path = match self.generate_filename(resource_id, &filename, index) {
                Ok(path) => path,
                Err(e) => {
                    errors.push(UploadError {
                        filename: filename.clone(),
                        error: e.to_string(),
                    });
                    continue;
                }
            };

            // SECURITY: Get safe absolute path with containment verification
            let absolute_path = match self.safe_path(&relative_path) {
                Ok(path) => path,
                Err(e) => {
                    warn!("Path security check failed for {}: {}", filename, e);
                    errors.push(UploadError {
                        filename: filename.clone(),
                        error: e.to_string(),
                    });
                    continue;
                }
            };

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

        // SECURITY: Validate resource ID
        self.validate_resource_id(resource_id)?;

        // SECURITY: Use safe_path to prevent directory traversal
        let resource_dir = self.safe_path(resource_id)?;

        if !resource_dir.exists() {
            return Ok(FileRetrievalResult::empty());
        }

        // SECURITY: Verify it's a directory, not a file or symlink
        if !resource_dir.is_dir() {
            return Err(StorageError::InvalidResourceId(
                "Resource path is not a directory".into(),
            ));
        }

        let mut storage_urls = Vec::new();
        let mut display_urls = Vec::new();

        let entries = std::fs::read_dir(&resource_dir)?;
        for entry in entries.flatten() {
            let path = entry.path();

            // SECURITY: Skip symlinks
            if path.is_symlink() {
                warn!("Skipping symlink: {:?}", path);
                continue;
            }

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

        // SECURITY: Use safe_path to prevent directory traversal
        let absolute_path = self.safe_path(&relative_path)?;

        // SECURITY: Verify it's a regular file, not a symlink
        if absolute_path.is_symlink() {
            return Err(StorageError::SymlinkRejected(
                "Cannot read symlinked files".into(),
            ));
        }

        if !absolute_path.exists() {
            return Err(StorageError::NotFound(format!(
                "File not found: {}",
                relative_path
            )));
        }

        if !absolute_path.is_file() {
            return Err(StorageError::InvalidUrl(
                "Path is not a regular file".into(),
            ));
        }

        std::fs::read(&absolute_path).map_err(|e| {
            StorageError::DownloadError(format!("Failed to read file: {}", e))
        })
    }

    async fn delete_files(&self, resource_id: &str) -> Result<()> {
        info!("Deleting files for resource {} (local)", resource_id);

        // SECURITY: Validate resource ID
        self.validate_resource_id(resource_id)?;

        // SECURITY: Use safe_path to prevent directory traversal
        let resource_dir = self.safe_path(resource_id)?;

        if !resource_dir.exists() {
            return Ok(());
        }

        // SECURITY: Verify it's a directory within our base
        if !resource_dir.is_dir() {
            return Err(StorageError::InvalidResourceId(
                "Resource path is not a directory".into(),
            ));
        }

        // SECURITY: Double-check containment before deletion
        if !resource_dir.starts_with(&self.canonical_base) {
            return Err(StorageError::PathTraversalRejected(
                "Cannot delete outside storage directory".into(),
            ));
        }

        std::fs::remove_dir_all(&resource_dir)?;
        debug!("Deleted directory: {:?}", resource_dir);

        Ok(())
    }

    async fn delete_file(&self, storage_url: &str) -> Result<()> {
        let (_, relative_path) = self.parse_storage_url(storage_url).ok_or_else(|| {
            StorageError::InvalidUrl(format!("Invalid storage URL: {}", storage_url))
        })?;

        // SECURITY: Use safe_path to prevent directory traversal
        let absolute_path = self.safe_path(&relative_path)?;

        // SECURITY: Double-check containment before deletion
        if !absolute_path.starts_with(&self.canonical_base) {
            return Err(StorageError::PathTraversalRejected(
                "Cannot delete outside storage directory".into(),
            ));
        }

        if absolute_path.exists() {
            // SECURITY: Verify it's a regular file
            if !absolute_path.is_file() {
                return Err(StorageError::InvalidUrl(
                    "Path is not a regular file".into(),
                ));
            }

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
