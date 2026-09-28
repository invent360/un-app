//! # ember-fx-tools
//!
//! Developer tools and debugging utilities for ember-fx.
//!
//! This crate provides components for debugging and inspecting ember-fx applications:
//!
//! - **ThemeDebugger** - Display current theme state and switch themes
//! - **CSSVariableViewer** - View all CSS custom properties
//! - **ComponentInspector** - Inspect component hierarchy and props
//! - **PerformanceOverlay** - Display render timing and metrics
//! - **A11yChecker** - Check accessibility compliance
//! - **Performance** - Performance measurement utilities (timing, benchmarks)
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx_tools::ThemeDebugger;
//!
//! view! {
//!     <ThemeDebugger />
//! }
//! ```
//!
//! ## Feature Flags
//!
//! - `dev-tools` - Enable all development tools (default: disabled in release)

mod theme_debugger;
mod css_viewer;
mod component_inspector;
mod performance_overlay;
mod a11y_checker;
pub mod performance;

pub use theme_debugger::ThemeDebugger;
pub use css_viewer::CSSVariableViewer;
pub use component_inspector::ComponentInspector;
pub use performance_overlay::PerformanceOverlay;
pub use a11y_checker::A11yChecker;
pub use performance::{
    PerfTimer, MetricStats, PerformanceReport,
    now, record_metric, get_metrics, clear_metrics,
    measure_css_injection, measure_theme_switch, benchmark,
};

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::{
        ThemeDebugger, CSSVariableViewer, ComponentInspector,
        PerformanceOverlay, A11yChecker, PerformanceReport,
        PerfTimer, benchmark,
    };
}
