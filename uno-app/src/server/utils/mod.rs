//! Server utilities
//!
//! Provides common utilities for server-side operations including:
//! - Image optimization and responsive image generation
//! - Format negotiation based on client capabilities
//! - Cloud storage URL conversion

pub mod image_optimizer;
pub mod url_converter;

pub use image_optimizer::{
    ImageOptimizer,
    ImageConfig,
    ImageFormat,
    ImageVariant,
    OptimizedImage,
    ResponsiveImageAttrs,
    dimensions,
    placeholder,
};

pub use url_converter::{
    convert_storage_urls,
    convert_storage_urls_in_json,
    storage_to_display_url,
    storage_to_display_urls,
};
