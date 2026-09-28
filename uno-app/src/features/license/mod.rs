//! License feature module
//!
//! This module contains shared components and utilities for license-related
//! functionality that spans multiple parts of the application (pages, wizard, etc.).
//!
//! # Components
//!
//! - [`VariantCard`] - Unified variant card component (works in full and compact modes)
//! - [`UrgencyBar`] - Progress bar showing license availability
//!
//! # Usage
//!
//! ```ignore
//! use crate::features::license::{VariantCard, VariantCardMode, UrgencyBar};
//!
//! // Full mode for license page
//! view! { <VariantCard variant=variant mode=VariantCardMode::Full on_claim=on_claim /> }
//!
//! // Compact mode for wizard selection
//! view! { <VariantCard variant=variant mode=VariantCardMode::Compact on_select=on_select /> }
//! ```

mod variant_card;
mod urgency_bar;

pub use variant_card::{VariantCard, VariantCardMode};
pub use urgency_bar::{UrgencyBar, UrgencyLevel};
