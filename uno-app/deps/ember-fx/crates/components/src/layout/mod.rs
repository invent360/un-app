//! Layout components for ember-fx.
//!
//! This module provides structural and container components:
//! - Card: Content container with header/body/footer
//! - Modal: Overlay dialogs
//! - Drawer: Slide-in panels
//! - Tabs: Tabbed content
//! - Collapse/Accordion: Expandable sections
//! - Divider: Visual separator

mod types;
mod card;
mod modal;
mod drawer;
mod tabs;
mod collapse;
mod divider;

pub use types::*;
pub use card::Card;
pub use modal::Modal;
pub use drawer::Drawer;
pub use tabs::{Tabs, TabPane};
pub use collapse::{Collapse, CollapsePanel};
pub use divider::Divider;
