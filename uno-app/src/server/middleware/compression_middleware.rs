//! HTTP Response Compression Middleware
//!
//! Provides response compression using Brotli, Gzip, and Deflate encoding.
//! Brotli is preferred for better compression ratios when supported by clients.
//!
//! # Features
//!
//! - Automatic content negotiation based on Accept-Encoding header
//! - Configurable minimum response size for compression
//! - Excludes already compressed content (images, videos, etc.)
//! - Supports streaming responses
//!
//! # Usage
//!
//! ```ignore
//! use crate::server::middleware::CompressionConfig;
//!
//! App::new()
//!     .wrap(CompressionConfig::default().middleware())
//! ```

use actix_web::http::header::{self, ContentEncoding, HeaderValue};

/// Compression configuration
#[derive(Clone)]
pub struct CompressionConfig {
    /// Minimum response size in bytes to trigger compression (default: 1KB)
    pub min_size: usize,
    /// Enable Brotli compression (default: true)
    pub enable_brotli: bool,
    /// Enable Gzip compression (default: true)
    pub enable_gzip: bool,
    /// Enable Deflate compression (default: true)
    pub enable_deflate: bool,
    /// Brotli quality level (0-11, default: 4 for good balance)
    pub brotli_quality: u32,
    /// Gzip quality level (0-9, default: 6)
    pub gzip_quality: u32,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            min_size: 1024,           // 1KB minimum
            enable_brotli: true,
            enable_gzip: true,
            enable_deflate: true,
            brotli_quality: 4,        // Balance between speed and compression
            gzip_quality: 6,
        }
    }
}

impl CompressionConfig {
    /// Create with high compression (slower but smaller output)
    pub fn high_compression() -> Self {
        Self {
            min_size: 512,
            enable_brotli: true,
            enable_gzip: true,
            enable_deflate: true,
            brotli_quality: 8,
            gzip_quality: 9,
        }
    }

    /// Create with fast compression (less compression but faster)
    pub fn fast_compression() -> Self {
        Self {
            min_size: 2048,
            enable_brotli: true,
            enable_gzip: true,
            enable_deflate: false, // Skip deflate for speed
            brotli_quality: 2,
            gzip_quality: 4,
        }
    }

    /// Set minimum response size
    pub fn min_size(mut self, size: usize) -> Self {
        self.min_size = size;
        self
    }

    /// Disable Brotli compression
    pub fn no_brotli(mut self) -> Self {
        self.enable_brotli = false;
        self
    }

    /// Set Brotli quality (0-11)
    pub fn brotli_quality(mut self, quality: u32) -> Self {
        self.brotli_quality = quality.min(11);
        self
    }

    /// Determine best encoding based on Accept-Encoding header
    pub fn negotiate_encoding(&self, accept_encoding: Option<&str>) -> Option<ContentEncoding> {
        let accept = accept_encoding?;

        // Parse Accept-Encoding and find best match
        // Prefer Brotli > Gzip > Deflate based on quality values
        let mut best_encoding: Option<(ContentEncoding, f32)> = None;

        for part in accept.split(',') {
            let part = part.trim();
            let (encoding, quality) = if let Some((enc, q)) = part.split_once(";q=") {
                (enc.trim(), q.trim().parse().unwrap_or(1.0))
            } else {
                (part, 1.0)
            };

            let content_encoding = match encoding.to_lowercase().as_str() {
                "br" if self.enable_brotli => Some(ContentEncoding::Brotli),
                "gzip" if self.enable_gzip => Some(ContentEncoding::Gzip),
                "deflate" if self.enable_deflate => Some(ContentEncoding::Deflate),
                "*" => {
                    // Wildcard - use our preferred order
                    if self.enable_brotli {
                        Some(ContentEncoding::Brotli)
                    } else if self.enable_gzip {
                        Some(ContentEncoding::Gzip)
                    } else if self.enable_deflate {
                        Some(ContentEncoding::Deflate)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            if let Some(enc) = content_encoding {
                // Prefer Brotli when quality is equal (better compression)
                let effective_quality = match enc {
                    ContentEncoding::Brotli => quality + 0.01,
                    ContentEncoding::Gzip => quality,
                    ContentEncoding::Deflate => quality - 0.01,
                    _ => quality,
                };

                match best_encoding {
                    None => best_encoding = Some((enc, effective_quality)),
                    Some((_, current_quality)) if effective_quality > current_quality => {
                        best_encoding = Some((enc, effective_quality));
                    }
                    _ => {}
                }
            }
        }

        best_encoding.map(|(enc, _)| enc)
    }

    /// Check if content type should be compressed
    pub fn should_compress(content_type: Option<&str>) -> bool {
        let ct = match content_type {
            Some(ct) => ct.to_lowercase(),
            None => return true, // Compress unknown types
        };

        // Don't compress already-compressed formats
        let skip_types = [
            "image/png",
            "image/jpeg",
            "image/gif",
            "image/webp",
            "image/avif",
            "video/",
            "audio/",
            "application/zip",
            "application/gzip",
            "application/x-brotli",
            "application/octet-stream",
        ];

        !skip_types.iter().any(|&t| ct.starts_with(t))
    }
}

/// Compression statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct CompressionStats {
    /// Total requests processed
    pub requests: u64,
    /// Requests that were compressed
    pub compressed: u64,
    /// Total bytes before compression
    pub bytes_in: u64,
    /// Total bytes after compression
    pub bytes_out: u64,
}

impl CompressionStats {
    /// Calculate compression ratio
    pub fn ratio(&self) -> f64 {
        if self.bytes_in == 0 {
            0.0
        } else {
            1.0 - (self.bytes_out as f64 / self.bytes_in as f64)
        }
    }

    /// Calculate bytes saved
    pub fn bytes_saved(&self) -> u64 {
        self.bytes_in.saturating_sub(self.bytes_out)
    }
}

/// MIME types that should always be compressed
pub const COMPRESSIBLE_TYPES: &[&str] = &[
    "text/html",
    "text/css",
    "text/javascript",
    "text/plain",
    "text/xml",
    "application/json",
    "application/javascript",
    "application/xml",
    "application/xhtml+xml",
    "application/rss+xml",
    "application/atom+xml",
    "image/svg+xml",
    "font/ttf",
    "font/otf",
    "application/font-woff",
    "application/font-woff2",
];

/// Check if a MIME type is compressible
pub fn is_compressible(content_type: &str) -> bool {
    let ct = content_type.to_lowercase();
    COMPRESSIBLE_TYPES.iter().any(|&t| ct.starts_with(t))
}

/// Add Vary header for caching with compression
pub fn add_vary_header(headers: &mut actix_web::http::header::HeaderMap) {
    if let Some(existing) = headers.get(header::VARY) {
        if let Ok(existing_str) = existing.to_str() {
            if !existing_str.to_lowercase().contains("accept-encoding") {
                let new_value = format!("{}, Accept-Encoding", existing_str);
                if let Ok(value) = HeaderValue::from_str(&new_value) {
                    headers.insert(header::VARY, value);
                }
            }
        }
    } else {
        headers.insert(header::VARY, HeaderValue::from_static("Accept-Encoding"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negotiate_encoding_brotli_preferred() {
        let config = CompressionConfig::default();

        let encoding = config.negotiate_encoding(Some("gzip, br, deflate"));
        assert!(matches!(encoding, Some(ContentEncoding::Brotli)));
    }

    #[test]
    fn test_negotiate_encoding_with_quality() {
        let config = CompressionConfig::default();

        // Gzip with higher quality should win
        let encoding = config.negotiate_encoding(Some("gzip;q=1.0, br;q=0.5"));
        assert!(matches!(encoding, Some(ContentEncoding::Gzip)));
    }

    #[test]
    fn test_negotiate_encoding_wildcard() {
        let config = CompressionConfig::default();

        let encoding = config.negotiate_encoding(Some("*"));
        assert!(matches!(encoding, Some(ContentEncoding::Brotli)));
    }

    #[test]
    fn test_should_compress_text() {
        assert!(CompressionConfig::should_compress(Some("text/html")));
        assert!(CompressionConfig::should_compress(Some("application/json")));
    }

    #[test]
    fn test_should_not_compress_images() {
        assert!(!CompressionConfig::should_compress(Some("image/png")));
        assert!(!CompressionConfig::should_compress(Some("image/jpeg")));
    }

    #[test]
    fn test_is_compressible() {
        assert!(is_compressible("text/html"));
        assert!(is_compressible("application/json"));
        assert!(is_compressible("text/css"));
        assert!(!is_compressible("image/png"));
    }

    #[test]
    fn test_compression_stats() {
        let stats = CompressionStats {
            requests: 100,
            compressed: 80,
            bytes_in: 10000,
            bytes_out: 3000,
        };

        assert!((stats.ratio() - 0.7).abs() < 0.001);
        assert_eq!(stats.bytes_saved(), 7000);
    }
}
