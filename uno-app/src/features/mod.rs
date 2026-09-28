//! Features module
//!
//! This module contains cross-cutting feature implementations that span
//! multiple parts of the application. Features are distinct from components
//! in that they represent business capabilities rather than UI primitives.
//!
//! # Structure
//!
//! ```text
//! features/
//! ├── license/       # License claiming and display features
//! │   ├── variant_card.rs   # Unified variant card component
//! │   └── urgency_bar.rs    # License availability progress bar
//! └── mod.rs
//! ```
//!
//! # Design Philosophy
//!
//! - **Components** (`src/components/`): Reusable UI primitives (buttons, modals, cards)
//! - **Features** (`src/features/`): Business-specific functionality that may use multiple components
//! - **Routes** (`src/routes/`): Page-level components that compose features
//!
//! Features help avoid duplication by providing shared implementations that
//! can be used across different contexts (e.g., the same variant card works
//! on the license page and in the wizard).

pub mod license;

pub use license::{VariantCard, VariantCardMode, UrgencyBar, UrgencyLevel};
