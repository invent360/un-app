//! Tabler Icons - A comprehensive icon library.
//!
//! This module provides access to Tabler Icons, a set of over 5,000 high-quality
//! SVG icons organized by category.
//!
//! ## Features
//!
//! Each category is feature-gated for optimal bundle size:
//!
//! - `tabler-arrows` - Navigation and directional arrows
//! - `tabler-system` - System and UI action icons
//! - `tabler-communication` - Messaging and communication icons
//! - `tabler-media` - Media playback and controls
//! - `tabler-document` - File and document icons
//! - `tabler-commerce` - E-commerce and shopping icons
//! - `tabler-map` - Location and mapping icons
//! - `tabler-charts` - Data visualization icons
//! - `tabler-brand` - Brand and social media logos
//! - `tabler-design` - Design and creative tools
//! - `tabler-devices` - Device and hardware icons
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx_icons::tabler::{TablerIcon, ArrowIcon, IconVariant};
//!
//! view! {
//!     <TablerIcon icon=ArrowIcon::ArrowDown />
//!     <TablerIcon icon=ArrowIcon::ArrowUp variant=IconVariant::Filled />
//! }
//! ```

mod icon;

#[cfg(feature = "tabler-arrows")]
pub mod arrows;

#[cfg(feature = "tabler-system")]
pub mod system;

#[cfg(feature = "tabler-communication")]
pub mod communication;

#[cfg(feature = "tabler-media")]
pub mod media;

#[cfg(feature = "tabler-document")]
pub mod document;

#[cfg(feature = "tabler-commerce")]
pub mod commerce;

#[cfg(feature = "tabler-map")]
pub mod map;

#[cfg(feature = "tabler-charts")]
pub mod charts;

#[cfg(feature = "tabler-brand")]
pub mod brand;

#[cfg(feature = "tabler-design")]
pub mod design;

#[cfg(feature = "tabler-devices")]
pub mod devices;

// Re-exports
pub use icon::TablerIcon;

#[cfg(feature = "tabler-arrows")]
pub use arrows::ArrowsIcon;

#[cfg(feature = "tabler-system")]
pub use system::SystemIcon;

#[cfg(feature = "tabler-communication")]
pub use communication::CommunicationIcon;

#[cfg(feature = "tabler-media")]
pub use media::MediaIcon;

#[cfg(feature = "tabler-document")]
pub use document::DocumentIcon;

#[cfg(feature = "tabler-commerce")]
pub use commerce::CommerceIcon;

#[cfg(feature = "tabler-map")]
pub use map::MapIcon;

#[cfg(feature = "tabler-charts")]
pub use charts::ChartsIcon;

#[cfg(feature = "tabler-brand")]
pub use brand::BrandIcon;

#[cfg(feature = "tabler-design")]
pub use design::DesignIcon;

#[cfg(feature = "tabler-devices")]
pub use devices::DevicesIcon;

use crate::types::IconVariant;

/// Trait for tabler icon enums.
pub trait TablerIconData: Copy {
    /// Returns the icon name in kebab-case.
    fn name(&self) -> &'static str;

    /// Returns the outline SVG content.
    fn outline_svg(&self) -> &'static str;

    /// Returns the filled SVG content, if available.
    fn filled_svg(&self) -> Option<&'static str>;

    /// Returns the SVG for the specified variant.
    fn svg(&self, variant: IconVariant) -> &'static str {
        match variant {
            IconVariant::Outline => self.outline_svg(),
            IconVariant::Filled => self.filled_svg().unwrap_or_else(|| self.outline_svg()),
        }
    }

    /// Returns true if this icon has a filled variant.
    fn has_filled(&self) -> bool {
        self.filled_svg().is_some()
    }
}
