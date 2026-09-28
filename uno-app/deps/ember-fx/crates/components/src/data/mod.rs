//! Data display components for ember-fx.
//!
//! This module provides data presentation components:
//! - Avatar: User avatars with images or initials
//! - Tooltip: Hover hints
//! - Popover: Rich hover content
//! - List: Structured list displays
//! - Table: Data tables with sorting/filtering

mod types;
mod avatar;
mod tooltip;
mod popover;
mod list;
mod table;

pub use types::*;
pub use avatar::{Avatar, AvatarGroup};
pub use tooltip::Tooltip;
pub use popover::Popover;
pub use list::{List, ListItem, ListItemMeta};
pub use table::Table;
