//! # ember-fx-core
//!
//! Foundation layer for ember-fx.
//!
//! This crate provides:
//! - Theme context and reactive state management
//! - Theme providers for wrapping applications
//! - Platform detection for Tauri 2.0 WebView environments
//!
//! ## Quick Start
//!
//! ```ignore
//! use ember_fx_core::{ThemeProvider, DesignSystem, use_theme};
//!
//! #[component]
//! pub fn App() -> impl IntoView {
//!     view! {
//!         <ThemeProvider
//!             initial_theme="dark"
//!             design_system=DesignSystem::Ant
//!         >
//!             <MyApp />
//!         </ThemeProvider>
//!     }
//! }
//! ```

mod context;
mod platform;
mod provider;

// Re-export from common
pub use ember_fx_common::{
    DesignSystem, Size, Direction, ThemeMode,
    THEME_STORAGE_KEY, DESIGN_SYSTEM_STORAGE_KEY,
};

// Re-export from utils
pub use ember_fx_utils::{
    inject_css, remove_css, has_css, inject_base_css,
    apply_theme_attribute, get_current_theme_attribute,
    THEME_STYLE_ID, COMPONENTS_STYLE_ID, BASE_STYLE_ID,
    ClassList,
};

// Re-export from styles
pub use ember_fx_styles::{ThemeRegistry, CompiledTheme, get_component_css};

// Core exports
pub use context::{ThemeContext, use_theme, try_use_theme, load_saved_theme, load_saved_design_system};
pub use platform::{Platform, get_user_agent, supports_touch, device_pixel_ratio};
pub use provider::{ThemeProvider, MinimalThemeProvider};

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::{
        ThemeContext, use_theme, try_use_theme,
        ThemeProvider, MinimalThemeProvider,
        Platform, DesignSystem, ThemeMode,
    };
}
