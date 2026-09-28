//! # ember-fx-mobile
//!
//! Mobile-specific components for ember-fx.
//!
//! This crate provides components optimized for mobile and touch interfaces:
//!
//! - **MobileNavBar** - Top navigation bar with mobile-specific features
//! - **MobileTabBar** - Bottom tab navigation for mobile apps
//! - **SwipeContainer** - Container with swipe gesture detection
//! - **PullToRefresh** - Pull-to-refresh functionality
//! - **SafeAreaProvider** - Safe area inset management for notched devices
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx_mobile::{MobileNavBar, MobileTabBar};
//!
//! view! {
//!     <MobileNavBar title="My App" />
//!     <main>
//!         // Content
//!     </main>
//!     <MobileTabBar />
//! }
//! ```

mod nav_bar;
mod tab_bar;
mod swipe_container;
mod pull_to_refresh;
mod safe_area;

pub use nav_bar::{MobileNavBar, NavBarAction};
pub use tab_bar::{MobileTabBar, TabItem};
pub use swipe_container::{SwipeContainer, SwipeDirection};
pub use pull_to_refresh::PullToRefresh;
pub use safe_area::{SafeAreaProvider, SafeAreaInsets, use_safe_area};

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::{
        MobileNavBar, NavBarAction,
        MobileTabBar, TabItem,
        SwipeContainer, SwipeDirection,
        PullToRefresh,
        SafeAreaProvider, SafeAreaInsets, use_safe_area,
    };
}
