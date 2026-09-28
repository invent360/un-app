//! # ember-fx-icons
//!
//! SVG icon system for ember-fx.
//!
//! This crate provides a flexible icon system with support for multiple icon sets:
//! - **Heroicons** - Beautiful hand-crafted SVG icons by the makers of Tailwind CSS
//! - **Lucide** - Beautiful & consistent icon toolkit
//! - **Phosphor** - Flexible icon family for interfaces
//! - **Tabler** - 5,000+ stroke-based icons organized by category
//! - **Crypto** - Cryptocurrency token icons (BTC, ETH, USDC, etc.)
//! - **Animals** - Animal icons (pets, farm animals, wildlife, sea creatures, insects)
//! - **Nature** - Nature icons (plants, farming, weather, landscape)
//! - **Finance** - Finance icons (money, banking, trading, business)
//! - **Commerce** - Commerce icons (shopping, products, shipping)
//! - **Social** - Social icons (communication, social actions, media, profiles)
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx_icons::{Icon, IconName};
//!
//! view! {
//!     <Icon name=IconName::Check class="w-4 h-4" />
//! }
//! ```
//!
//! ## Custom Icons
//!
//! You can also use custom SVG content:
//!
//! ```ignore
//! view! {
//!     <Icon svg="<svg>...</svg>" class="w-4 h-4" />
//! }
//! ```
//!
//! ## Tabler Icons
//!
//! Enable tabler features for comprehensive icon coverage:
//!
//! ```ignore
//! use ember_fx_icons::tabler::{TablerIcon, ArrowIcon};
//! use ember_fx_icons::types::{IconSize, IconVariant};
//!
//! view! {
//!     <TablerIcon icon=ArrowIcon::ArrowDown size=IconSize::Lg />
//! }
//! ```
//!
//! ## Cryptocurrency Token Icons
//!
//! Enable the `crypto` feature to use token icons:
//!
//! ```ignore
//! use ember_fx_icons::{Token, TokenIcon};
//!
//! view! {
//!     <Token icon=TokenIcon::BTC class="w-6 h-6" />
//! }
//! ```
//!
//! ## Animal Icons
//!
//! Enable the `animals` feature:
//!
//! ```ignore
//! use ember_fx_icons::{Animal, AnimalIcon};
//!
//! view! {
//!     <Animal icon=AnimalIcon::Dog class="w-6 h-6" />
//! }
//! ```
//!
//! ## Nature Icons
//!
//! Enable the `nature` feature:
//!
//! ```ignore
//! use ember_fx_icons::{Nature, NatureIcon};
//!
//! view! {
//!     <Nature icon=NatureIcon::Tree class="w-6 h-6" />
//! }
//! ```
//!
//! ## Finance Icons
//!
//! Enable the `finance` feature:
//!
//! ```ignore
//! use ember_fx_icons::{Finance, FinanceIcon};
//!
//! view! {
//!     <Finance icon=FinanceIcon::Wallet class="w-6 h-6" />
//! }
//! ```
//!
//! ## Commerce Icons
//!
//! Enable the `commerce` feature:
//!
//! ```ignore
//! use ember_fx_icons::{Commerce, CommerceIcon};
//!
//! view! {
//!     <Commerce icon=CommerceIcon::Cart class="w-6 h-6" />
//! }
//! ```
//!
//! ## Social Icons
//!
//! Enable the `social` feature:
//!
//! ```ignore
//! use ember_fx_icons::{Social, SocialIcon};
//!
//! view! {
//!     <Social icon=SocialIcon::Like class="w-6 h-6" />
//! }
//! ```

mod icon;
mod registry;
pub mod types;

#[cfg(feature = "tabler")]
pub mod tabler;

#[cfg(feature = "crypto")]
mod tokens;

#[cfg(feature = "animals")]
mod animals;

#[cfg(feature = "nature")]
mod nature;

#[cfg(feature = "finance")]
mod finance;

#[cfg(feature = "commerce")]
mod commerce;

#[cfg(feature = "social")]
mod social;

#[cfg(feature = "flags")]
mod flags;

pub use icon::Icon;
pub use registry::{IconName, IconSet, get_icon_svg};

#[cfg(feature = "crypto")]
pub use tokens::{Token, TokenIcon, get_token_svg};

#[cfg(feature = "animals")]
pub use animals::{Animal, AnimalIcon, get_animal_svg};

#[cfg(feature = "nature")]
pub use nature::{Nature, NatureIcon, get_nature_svg};

#[cfg(feature = "finance")]
pub use finance::{Finance, FinanceIcon, get_finance_svg};

#[cfg(feature = "commerce")]
pub use commerce::{Commerce, CommerceIcon, get_commerce_svg};

#[cfg(feature = "social")]
pub use social::{Social, SocialIcon, get_social_svg};

#[cfg(feature = "flags")]
pub use flags::{Flag, FlagIcon, FlagAspect, FlagVariant, get_flag_svg, get_flag_svg_1x1, get_flag_svg_4x3};

/// Common icon sizes as CSS classes.
pub mod sizes {
    pub const XS: &str = "w-3 h-3";
    pub const SM: &str = "w-4 h-4";
    pub const MD: &str = "w-5 h-5";
    pub const LG: &str = "w-6 h-6";
    pub const XL: &str = "w-8 h-8";
}
