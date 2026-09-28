//! Ant Design layout components.
//!
//! This module provides Ant Design styled layout components:
//!
//! - [`AntMainLayout`] - Root layout wrapper with sidebar, header, and mobile nav
//! - [`AntHeader`] - Fixed top navigation bar
//! - [`AntSidebar`] - Collapsible sidebar navigation
//! - [`AntMobileNav`] - Bottom navigation for mobile devices
//! - [`AntTopMobileNav`] - Top navigation for mobile devices (mirrored bottom nav)
//! - [`AntSettingsDrawer`] - Settings drawer for layout configuration
//! - [`AntSettingsTrigger`] - Floating button to open settings drawer

mod main_layout;
mod header;
mod sidebar;
mod mobile_nav;
mod top_mobile_nav;
mod settings_drawer;

pub use main_layout::AntMainLayout;
pub use header::AntHeader;
pub use sidebar::AntSidebar;
pub use mobile_nav::AntMobileNav;
pub use top_mobile_nav::AntTopMobileNav;
pub use settings_drawer::{AntSettingsDrawer, AntSettingsTrigger};
