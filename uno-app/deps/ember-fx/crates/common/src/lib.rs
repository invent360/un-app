//! # ember-fx-common
//!
//! Shared types, traits, enums, and constants used across ember-fx crates.
//!
//! This crate provides the foundational types that all other ember-fx crates depend on,
//! ensuring consistency and reducing duplication.

mod design_system;
mod size;
mod direction;
mod theme_mode;
mod traits;
mod constants;
pub mod validation;

pub use design_system::DesignSystem;
pub use size::Size;
pub use direction::Direction;
pub use theme_mode::ThemeMode;
pub use traits::ComponentProps;
pub use constants::*;
pub use validation::{ValidationRule, ValidationResult, ValidateOn, validate_value, validate_all};
