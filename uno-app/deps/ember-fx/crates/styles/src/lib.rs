//! # ember-fx-styles
//!
//! CSS compilation and theme registry for ember-fx.
//!
//! This crate handles:
//! - CSS compilation via LightningCSS at build time
//! - Theme registry and lookup
//! - Embedded theme CSS via `include_str!()`
//!
//! ## Theme Structure
//!
//! Themes are compiled from JSON presets at build time and embedded into the binary.
//! The registry provides access to these compiled themes.
//!
//! ```ignore
//! use ember_fx_styles::ThemeRegistry;
//!
//! // Load theme CSS
//! if let Some(css) = ThemeRegistry::load_theme_css("ant", "dark") {
//!     // Use the CSS...
//! }
//! ```

mod registry;

pub use ember_fx_common::DesignSystem;
pub use registry::{ThemeRegistry, CompiledTheme, get_component_css};
