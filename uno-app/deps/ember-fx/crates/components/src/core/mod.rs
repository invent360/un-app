//! Core module for component infrastructure.
//!
//! This module provides the foundational traits and utilities for building
//! components in ember-fx. It is designed to be thin, modular, and performant.
//!
//! ## Trait Categories
//!
//! Traits are organized into semantic categories:
//! - **Base**: Core component interface (`FxComponent`, `Renderable`)
//! - **Size**: Sizing system (`Sizable`, `ComponentSize`)
//! - **Visual**: Appearance traits (`Themed`, `Colored`, `Rounded`, `Bordered`)
//! - **State**: State management (`DisabledState`, `LoadingState`)
//! - **Interaction**: User interaction (`Clickable`, `Focusable`, `Hoverable`)
//! - **Accessibility**: A11y support (`Accessible`, `KeyboardNavigable`)
//!
//! ## Usage
//!
//! Components implement only the traits they need:
//!
//! ```ignore
//! use ember_fx::core::traits::*;
//!
//! pub struct ButtonConfig {
//!     size: ComponentSize,
//!     disabled: bool,
//! }
//!
//! impl Sizable for ButtonConfig {
//!     fn size(&self) -> ComponentSize { self.size }
//! }
//!
//! impl DisabledState for ButtonConfig {
//!     fn is_disabled(&self) -> bool { self.disabled }
//! }
//! ```

pub mod traits;

// Re-export commonly used items
pub use traits::{
    // Base
    FxComponent, ComponentType,
    // Size
    Sizable, ComponentSize,
    // Visual
    Themed, Colored, Rounded, Bordered, ComponentColor, BorderRadius,
    // State
    DisabledState, LoadingState,
    // Interaction
    Clickable, Focusable, Hoverable,
    // Accessibility
    Accessible, AriaRole,
};
