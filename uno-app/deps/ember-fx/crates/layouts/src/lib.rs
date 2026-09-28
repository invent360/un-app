//! # ember-fx-layouts
//!
//! Modular app shell layouts for ember-fx.
//!
//! This crate provides reusable layout components for building app shells:
//!
//! - **MainLayout** - Root app shell with sidebar, header, and mobile nav
//! - **Header** - Fixed top navigation bar
//! - **Sidebar** - Collapsible sidebar navigation
//! - **MobileNav** - Bottom navigation for mobile devices
//!
//! ## Quick Start
//!
//! ```ignore
//! use ember_fx_layouts::{MainLayout, NavItem, LogoConfig, UserConfig};
//! use ember_fx_core::ThemeProvider;
//!
//! // Define navigation items
//! let nav_items = vec![
//!     NavItem::new("dashboard", "Dashboard", "/dashboard")
//!         .icon("<path d='M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z'/>"),
//!     NavItem::new("wallet", "Wallet", "/wallet")
//!         .icon("<path d='M21 18v1c0 1.1-.9 2-2 2H5...'/>"),
//!     NavItem::group("settings", "Settings")
//!         .icon("<path d='M19.14 12.94c...'/>")
//!         .children(vec![
//!             NavItem::new("profile", "Profile", "/settings/profile"),
//!             NavItem::new("security", "Security", "/settings/security"),
//!         ]),
//! ];
//!
//! view! {
//!     <ThemeProvider initial_theme="dark">
//!         <MainLayout
//!             nav_items=nav_items
//!             logo=LogoConfig::text("S", "Stax Board").href("/")
//!             user=UserConfig::new("John Doe").avatar("/avatar.jpg")
//!             auth_paths=vec!["/login".to_string()]
//!             show_theme_toggle=true
//!         >
//!             <Router>
//!                 <Routes />
//!             </Router>
//!         </MainLayout>
//!     </ThemeProvider>
//! }
//! ```
//!
//! ## Design Systems
//!
//! Layouts are available in different design system flavors:
//!
//! - **ant** (default) - Ant Design styled layouts
//! - **mui** - Material Design (coming soon)
//! - **prime** - PrimeReact style (coming soon)
//!
//! Enable a design system via Cargo features:
//!
//! ```toml
//! [dependencies]
//! ember-fx-layouts = { version = "0.2", features = ["ant"] }
//! ```
//!
//! ## Layout Context
//!
//! The `MainLayout` provides a `LayoutContext` that can be accessed anywhere
//! in the component tree:
//!
//! ```ignore
//! use ember_fx_layouts::use_layout;
//!
//! let ctx = use_layout();
//!
//! // Toggle sidebar
//! ctx.toggle_collapsed();
//!
//! // Check mobile menu state
//! if ctx.mobile_menu_open.get() {
//!     // ...
//! }
//! ```

mod context;
mod types;
pub mod components;
pub mod designs;

// Re-export common types
pub use ember_fx_common::{DesignSystem, ThemeMode};
pub use ember_fx_core::{ThemeContext, use_theme, try_use_theme};

// Core exports
pub use context::{LayoutContext, use_layout, try_use_layout, provide_layout};
pub use types::{
    NavItem, MobileNavItem, LogoConfig, UserConfig,
    LayoutMode, SidebarPosition, NavPosition,
    nav_to_mobile,
};

// Component exports
pub use components::{
    Logo, ThemeToggle, UserMenu, NavItemComponent, CollapseToggle,
};

// Re-export design system modules for direct access
#[cfg(feature = "ant")]
pub use designs::ant;

// Design system aliases - the default export names point to the enabled design system
#[cfg(feature = "ant")]
pub use designs::ant::{
    AntMainLayout as MainLayout,
    AntHeader as Header,
    AntSidebar as Sidebar,
    AntMobileNav as MobileNav,
    AntTopMobileNav as TopMobileNav,
    AntSettingsDrawer as SettingsDrawer,
    AntSettingsTrigger as SettingsTrigger,
};

/// Prelude for convenient imports.
///
/// ```ignore
/// use ember_fx_layouts::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        // Context
        LayoutContext, use_layout, try_use_layout,
        // Types
        NavItem, MobileNavItem, LogoConfig, UserConfig,
        LayoutMode, SidebarPosition, NavPosition,
        nav_to_mobile,
    };

    #[cfg(feature = "ant")]
    pub use crate::designs::ant::{
        AntMainLayout, AntHeader, AntSidebar, AntMobileNav,
        AntTopMobileNav, AntSettingsDrawer, AntSettingsTrigger,
    };

    // Aliases
    #[cfg(feature = "ant")]
    pub use crate::{MainLayout, Header, Sidebar, MobileNav, TopMobileNav, SettingsDrawer, SettingsTrigger};
}
