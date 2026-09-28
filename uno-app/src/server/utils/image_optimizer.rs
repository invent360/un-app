//! Image Optimization Utilities
//!
//! Provides server-side image optimization including:
//! - Format conversion (WebP, AVIF)
//! - Responsive image sizing
//! - Quality optimization
//! - Srcset generation for responsive images
//!
//! # Usage
//!
//! ```ignore
//! use crate::server::utils::image_optimizer::{ImageOptimizer, ImageConfig};
//!
//! let optimizer = ImageOptimizer::new(ImageConfig::default());
//! let optimized = optimizer.optimize("original.jpg", &[320, 640, 1024])?;
//! ```

use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// Supported output image formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    /// Original format (no conversion)
    Original,
    /// WebP format (good compression, wide support)
    WebP,
    /// AVIF format (best compression, growing support)
    Avif,
    /// JPEG format
    Jpeg,
    /// PNG format
    Png,
}

impl ImageFormat {
    /// Get file extension for format
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Original => "",
            Self::WebP => "webp",
            Self::Avif => "avif",
            Self::Jpeg => "jpg",
            Self::Png => "png",
        }
    }

    /// Get MIME type for format
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Original => "image/*",
            Self::WebP => "image/webp",
            Self::Avif => "image/avif",
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "webp" => Some(Self::WebP),
            "avif" => Some(Self::Avif),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            _ => None,
        }
    }
}

/// Image optimization configuration
#[derive(Debug, Clone)]
pub struct ImageConfig {
    /// Output directory for optimized images
    pub output_dir: PathBuf,
    /// Default quality (1-100)
    pub default_quality: u8,
    /// Quality settings per format
    pub format_quality: HashMap<ImageFormat, u8>,
    /// Enable WebP generation
    pub generate_webp: bool,
    /// Enable AVIF generation
    pub generate_avif: bool,
    /// Responsive breakpoints (widths in pixels)
    pub breakpoints: Vec<u32>,
    /// Maximum image dimension
    pub max_dimension: u32,
    /// Enable lazy loading attributes
    pub lazy_loading: bool,
    /// Cache duration in seconds
    pub cache_duration: u64,
}

impl Default for ImageConfig {
    fn default() -> Self {
        let mut format_quality = HashMap::new();
        format_quality.insert(ImageFormat::WebP, 85);
        format_quality.insert(ImageFormat::Avif, 80);
        format_quality.insert(ImageFormat::Jpeg, 85);
        format_quality.insert(ImageFormat::Png, 90);

        Self {
            output_dir: PathBuf::from("assets/optimized"),
            default_quality: 85,
            format_quality,
            generate_webp: true,
            generate_avif: true,
            breakpoints: vec![320, 640, 768, 1024, 1280, 1920],
            max_dimension: 2560,
            lazy_loading: true,
            cache_duration: 31536000, // 1 year
        }
    }
}

impl ImageConfig {
    /// Create config optimized for bandwidth-constrained regions
    pub fn low_bandwidth() -> Self {
        let mut config = Self::default();
        config.default_quality = 75;
        config.format_quality.insert(ImageFormat::WebP, 70);
        config.format_quality.insert(ImageFormat::Avif, 65);
        config.breakpoints = vec![320, 480, 768, 1024];
        config.max_dimension = 1920;
        config
    }

    /// Create config for high quality images
    pub fn high_quality() -> Self {
        let mut config = Self::default();
        config.default_quality = 95;
        config.format_quality.insert(ImageFormat::WebP, 90);
        config.format_quality.insert(ImageFormat::Avif, 85);
        config
    }

    /// Get quality for a specific format
    pub fn quality_for(&self, format: ImageFormat) -> u8 {
        self.format_quality
            .get(&format)
            .copied()
            .unwrap_or(self.default_quality)
    }
}

/// Represents an optimized image variant
#[derive(Debug, Clone)]
pub struct ImageVariant {
    /// Path to the optimized image
    pub path: PathBuf,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels (if known)
    pub height: Option<u32>,
    /// Image format
    pub format: ImageFormat,
    /// File size in bytes
    pub size_bytes: u64,
}

impl ImageVariant {
    /// Get URL path for the variant
    pub fn url_path(&self) -> String {
        format!("/{}", self.path.display())
    }
}

/// Optimized image with all variants
#[derive(Debug, Clone)]
pub struct OptimizedImage {
    /// Original image path
    pub original: PathBuf,
    /// Original dimensions
    pub original_width: u32,
    pub original_height: u32,
    /// All generated variants
    pub variants: Vec<ImageVariant>,
    /// Alt text (if provided)
    pub alt: Option<String>,
}

impl OptimizedImage {
    /// Generate srcset attribute for responsive images
    pub fn srcset(&self, format: ImageFormat) -> String {
        self.variants
            .iter()
            .filter(|v| v.format == format)
            .map(|v| format!("{} {}w", v.url_path(), v.width))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Generate sizes attribute based on common breakpoints
    pub fn sizes(&self) -> &'static str {
        "(max-width: 320px) 100vw, \
         (max-width: 768px) 100vw, \
         (max-width: 1024px) 75vw, \
         50vw"
    }

    /// Get the best variant for a given width
    pub fn variant_for_width(&self, width: u32, format: ImageFormat) -> Option<&ImageVariant> {
        self.variants
            .iter()
            .filter(|v| v.format == format && v.width >= width)
            .min_by_key(|v| v.width)
    }

    /// Get the smallest variant (for placeholder/preview)
    pub fn smallest_variant(&self, format: ImageFormat) -> Option<&ImageVariant> {
        self.variants
            .iter()
            .filter(|v| v.format == format)
            .min_by_key(|v| v.width)
    }

    /// Generate HTML picture element
    pub fn to_picture_html(&self, class: Option<&str>, lazy: bool) -> String {
        let class_attr = class.map(|c| format!(" class=\"{}\"", c)).unwrap_or_default();
        let loading_attr = if lazy { " loading=\"lazy\" decoding=\"async\"" } else { "" };
        let alt = self.alt.as_deref().unwrap_or("");

        // Get fallback image (original format, largest size)
        let fallback = self.variants
            .iter()
            .filter(|v| v.format == ImageFormat::Original || v.format == ImageFormat::Jpeg)
            .max_by_key(|v| v.width)
            .map(|v| v.url_path())
            .unwrap_or_else(|| self.original.display().to_string());

        let mut html = format!("<picture{}>\n", class_attr);

        // AVIF sources (best compression)
        let avif_srcset = self.srcset(ImageFormat::Avif);
        if !avif_srcset.is_empty() {
            html.push_str(&format!(
                "  <source type=\"image/avif\" srcset=\"{}\" sizes=\"{}\">\n",
                avif_srcset,
                self.sizes()
            ));
        }

        // WebP sources (good compression, wide support)
        let webp_srcset = self.srcset(ImageFormat::WebP);
        if !webp_srcset.is_empty() {
            html.push_str(&format!(
                "  <source type=\"image/webp\" srcset=\"{}\" sizes=\"{}\">\n",
                webp_srcset,
                self.sizes()
            ));
        }

        // Fallback img element
        html.push_str(&format!(
            "  <img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\"{}>",
            fallback,
            alt,
            self.original_width,
            self.original_height,
            loading_attr
        ));

        html.push_str("\n</picture>");
        html
    }
}

/// Image optimizer service
#[derive(Debug, Clone)]
pub struct ImageOptimizer {
    config: ImageConfig,
}

impl ImageOptimizer {
    /// Create new optimizer with configuration
    pub fn new(config: ImageConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn default_config() -> Self {
        Self::new(ImageConfig::default())
    }

    /// Get configuration
    pub fn config(&self) -> &ImageConfig {
        &self.config
    }

    /// Generate output path for a variant
    pub fn variant_path(&self, original: &Path, width: u32, format: ImageFormat) -> PathBuf {
        let stem = original.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image");

        let ext = if format == ImageFormat::Original {
            original.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("jpg")
        } else {
            format.extension()
        };

        self.config.output_dir.join(format!("{}-{}w.{}", stem, width, ext))
    }

    /// Generate srcset string for an image path
    pub fn generate_srcset(&self, base_path: &str, widths: &[u32], format: ImageFormat) -> String {
        let path = Path::new(base_path);
        let stem = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image");

        let ext = if format == ImageFormat::Original {
            path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("jpg")
        } else {
            format.extension()
        };

        widths
            .iter()
            .map(|&w| format!("/assets/optimized/{}-{}w.{} {}w", stem, w, ext, w))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Get cache headers for optimized images
    pub fn cache_headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("Cache-Control", format!("public, max-age={}, immutable", self.config.cache_duration)),
            ("Vary", "Accept".to_string()),
        ]
    }

    /// Determine optimal format based on Accept header
    pub fn negotiate_format(&self, accept_header: Option<&str>) -> ImageFormat {
        let accept = match accept_header {
            Some(h) => h.to_lowercase(),
            None => return ImageFormat::Jpeg,
        };

        // Prefer AVIF > WebP > JPEG based on Accept header
        if self.config.generate_avif && accept.contains("image/avif") {
            ImageFormat::Avif
        } else if self.config.generate_webp && accept.contains("image/webp") {
            ImageFormat::WebP
        } else {
            ImageFormat::Jpeg
        }
    }

    /// Generate responsive image attributes
    pub fn responsive_attrs(&self, src: &str, alt: &str, width: u32, height: u32) -> ResponsiveImageAttrs {
        ResponsiveImageAttrs {
            src: src.to_string(),
            srcset: self.generate_srcset(src, &self.config.breakpoints, ImageFormat::Original),
            srcset_webp: if self.config.generate_webp {
                Some(self.generate_srcset(src, &self.config.breakpoints, ImageFormat::WebP))
            } else {
                None
            },
            srcset_avif: if self.config.generate_avif {
                Some(self.generate_srcset(src, &self.config.breakpoints, ImageFormat::Avif))
            } else {
                None
            },
            sizes: "(max-width: 320px) 100vw, (max-width: 768px) 100vw, (max-width: 1024px) 75vw, 50vw".to_string(),
            alt: alt.to_string(),
            width,
            height,
            loading: if self.config.lazy_loading { "lazy" } else { "eager" }.to_string(),
            decoding: "async".to_string(),
        }
    }
}

/// Responsive image HTML attributes
#[derive(Debug, Clone)]
pub struct ResponsiveImageAttrs {
    pub src: String,
    pub srcset: String,
    pub srcset_webp: Option<String>,
    pub srcset_avif: Option<String>,
    pub sizes: String,
    pub alt: String,
    pub width: u32,
    pub height: u32,
    pub loading: String,
    pub decoding: String,
}

impl ResponsiveImageAttrs {
    /// Generate HTML img element
    pub fn to_img_html(&self) -> String {
        format!(
            r#"<img src="{}" srcset="{}" sizes="{}" alt="{}" width="{}" height="{}" loading="{}" decoding="{}">"#,
            self.src, self.srcset, self.sizes, self.alt, self.width, self.height, self.loading, self.decoding
        )
    }

    /// Generate HTML picture element with format sources
    pub fn to_picture_html(&self) -> String {
        let mut html = String::from("<picture>\n");

        // AVIF source
        if let Some(ref avif) = self.srcset_avif {
            html.push_str(&format!(
                "  <source type=\"image/avif\" srcset=\"{}\" sizes=\"{}\">\n",
                avif, self.sizes
            ));
        }

        // WebP source
        if let Some(ref webp) = self.srcset_webp {
            html.push_str(&format!(
                "  <source type=\"image/webp\" srcset=\"{}\" sizes=\"{}\">\n",
                webp, self.sizes
            ));
        }

        // Fallback img
        html.push_str(&format!(
            "  <img src=\"{}\" srcset=\"{}\" sizes=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\" loading=\"{}\" decoding=\"{}\">\n",
            self.src, self.srcset, self.sizes, self.alt, self.width, self.height, self.loading, self.decoding
        ));

        html.push_str("</picture>");
        html
    }
}

/// Placeholder/blur-up image utilities
pub mod placeholder {
    use super::*;

    /// Low Quality Image Placeholder (LQIP) configuration
    #[derive(Debug, Clone)]
    pub struct LqipConfig {
        /// Width of placeholder image
        pub width: u32,
        /// Quality (very low for small file size)
        pub quality: u8,
        /// Apply blur effect
        pub blur: bool,
    }

    impl Default for LqipConfig {
        fn default() -> Self {
            Self {
                width: 20,
                quality: 20,
                blur: true,
            }
        }
    }

    /// Generate inline data URI for tiny placeholder
    pub fn inline_placeholder_css(width: u32, height: u32, color: &str) -> String {
        format!(
            "background: {} center/cover no-repeat; aspect-ratio: {}/{};",
            color, width, height
        )
    }

    /// Generate blur-up CSS for image loading
    pub fn blur_up_css() -> &'static str {
        r#"
        .img-blur-up {
            filter: blur(10px);
            transition: filter 0.3s ease-out;
        }
        .img-blur-up.loaded {
            filter: blur(0);
        }
        "#
    }

    /// Generate JavaScript for blur-up effect
    pub fn blur_up_js() -> &'static str {
        r#"
        document.querySelectorAll('.img-blur-up').forEach(img => {
            if (img.complete) {
                img.classList.add('loaded');
            } else {
                img.addEventListener('load', () => img.classList.add('loaded'));
            }
        });
        "#
    }
}

/// Image dimension calculations
pub mod dimensions {
    /// Calculate dimensions maintaining aspect ratio
    pub fn fit_within(
        original_width: u32,
        original_height: u32,
        max_width: u32,
        max_height: u32,
    ) -> (u32, u32) {
        let width_ratio = max_width as f64 / original_width as f64;
        let height_ratio = max_height as f64 / original_height as f64;
        let ratio = width_ratio.min(height_ratio);

        if ratio >= 1.0 {
            // Image already fits
            (original_width, original_height)
        } else {
            (
                (original_width as f64 * ratio).round() as u32,
                (original_height as f64 * ratio).round() as u32,
            )
        }
    }

    /// Calculate height for a given width maintaining aspect ratio
    pub fn height_for_width(original_width: u32, original_height: u32, target_width: u32) -> u32 {
        let ratio = target_width as f64 / original_width as f64;
        (original_height as f64 * ratio).round() as u32
    }

    /// Generate responsive widths based on original size
    pub fn responsive_widths(original_width: u32, breakpoints: &[u32]) -> Vec<u32> {
        breakpoints
            .iter()
            .filter(|&&w| w <= original_width)
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_format_extension() {
        assert_eq!(ImageFormat::WebP.extension(), "webp");
        assert_eq!(ImageFormat::Avif.extension(), "avif");
        assert_eq!(ImageFormat::Jpeg.extension(), "jpg");
    }

    #[test]
    fn test_format_detection() {
        assert_eq!(ImageFormat::from_extension("webp"), Some(ImageFormat::WebP));
        assert_eq!(ImageFormat::from_extension("JPEG"), Some(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_extension("unknown"), None);
    }

    #[test]
    fn test_config_quality() {
        let config = ImageConfig::default();
        assert_eq!(config.quality_for(ImageFormat::WebP), 85);
        assert_eq!(config.quality_for(ImageFormat::Avif), 80);
    }

    #[test]
    fn test_variant_path_generation() {
        let optimizer = ImageOptimizer::default_config();
        let path = optimizer.variant_path(Path::new("hero.jpg"), 640, ImageFormat::WebP);
        assert!(path.to_string_lossy().contains("hero-640w.webp"));
    }

    #[test]
    fn test_format_negotiation() {
        let optimizer = ImageOptimizer::default_config();

        assert_eq!(
            optimizer.negotiate_format(Some("image/avif, image/webp, image/*")),
            ImageFormat::Avif
        );
        assert_eq!(
            optimizer.negotiate_format(Some("image/webp, image/*")),
            ImageFormat::WebP
        );
        assert_eq!(
            optimizer.negotiate_format(Some("image/*")),
            ImageFormat::Jpeg
        );
    }

    #[test]
    fn test_dimensions_fit_within() {
        // Image larger than container
        let (w, h) = dimensions::fit_within(2000, 1000, 800, 600);
        assert_eq!(w, 800);
        assert_eq!(h, 400);

        // Image smaller than container
        let (w, h) = dimensions::fit_within(400, 300, 800, 600);
        assert_eq!(w, 400);
        assert_eq!(h, 300);
    }

    #[test]
    fn test_responsive_widths() {
        let breakpoints = vec![320, 640, 1024, 1920];
        let widths = dimensions::responsive_widths(1000, &breakpoints);
        assert_eq!(widths, vec![320, 640]);
    }

    #[test]
    fn test_srcset_generation() {
        let optimizer = ImageOptimizer::default_config();
        let srcset = optimizer.generate_srcset("/images/hero.jpg", &[320, 640], ImageFormat::WebP);
        assert!(srcset.contains("hero-320w.webp 320w"));
        assert!(srcset.contains("hero-640w.webp 640w"));
    }
}
