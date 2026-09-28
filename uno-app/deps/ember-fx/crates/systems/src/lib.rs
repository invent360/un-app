//! # ember-fx-systems
//!
//! Design system implementations for ember-fx.
//!
//! This crate provides concrete implementations for various design systems:
//! - **Ant Design** - Enterprise-grade design system
//! - **Material Design 3** - Google's design system
//! - **Cupertino** - Apple Human Interface Guidelines
//! - **DaisyUI** - Tailwind-based component library
//! - **PrimeReact** - Rich component library
//! - **Flutter Material** - Cross-platform Flutter aesthetics
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx_systems::{DesignSystemImpl, AntDesignSystem};
//!
//! let system = AntDesignSystem::default();
//! println!("Prefix: {}", system.class_prefix());
//! ```

mod traits;

#[cfg(feature = "ant")]
pub mod ant;

#[cfg(feature = "material")]
pub mod material;

#[cfg(feature = "cupertino")]
pub mod cupertino;

#[cfg(feature = "daisyui")]
pub mod daisyui;

#[cfg(feature = "prime")]
pub mod prime;

#[cfg(feature = "flutter")]
pub mod flutter;

// Re-exports
pub use ember_fx_common::DesignSystem;
pub use traits::{DesignSystemImpl, DesignTokens, ColorPalette, Spacing, Typography};

#[cfg(feature = "ant")]
pub use ant::AntDesignSystem;

#[cfg(feature = "material")]
pub use material::MaterialDesignSystem;

#[cfg(feature = "cupertino")]
pub use cupertino::CupertinoDesignSystem;

#[cfg(feature = "daisyui")]
pub use daisyui::DaisyUIDesignSystem;

#[cfg(feature = "prime")]
pub use prime::PrimeDesignSystem;

#[cfg(feature = "flutter")]
pub use flutter::FlutterDesignSystem;
