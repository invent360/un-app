//! Task display components for CMS-driven content.
//!
//! This module provides components for displaying task content from a CMS,
//! including task cards, task grids, and task carousels.
//!
//! # Components
//!
//! - [`TaskCard`] - Individual task card with earnings and status
//! - [`TaskCardGrid`] - Grid layout for multiple task cards
//!
//! # Example
//!
//! ```ignore
//! use ember_fx_components::{TaskCard, TaskData, EarningsTier, TaskStatus};
//!
//! let task = TaskData::new("1", "telemetry", "Telemetry Collection", "Share anonymous data")
//!     .image("/images/telemetry.png")
//!     .status(TaskStatus::Active)
//!     .difficulty(TaskDifficulty::Easy)
//!     .duration("5-10 min/day")
//!     .tier(EarningsTier::new("1 Device", 5.0, 8.0)
//!         .feature("Basic connection tasks")
//!         .feature("Minimal battery usage"))
//!     .tier(EarningsTier::new("2-3 Devices", 15.0, 25.0)
//!         .popular()
//!         .feature("All task types enabled")
//!         .feature("Higher uptime bonus"));
//!
//! view! {
//!     <TaskCard task=task show_earnings=true show_requirements=true />
//! }
//! ```
//!
//! # Styling
//!
//! Task components use the ember-fx theming system with CSS classes:
//!
//! - `.fx-task-card-{ds}` - Root card class
//! - `.fx-task-card-{ds}-cover` - Image cover
//! - `.fx-task-card-{ds}-content` - Content area
//! - `.fx-task-card-{ds}-earnings` - Earnings section
//! - `.fx-task-card-{ds}-tier` - Individual tier
//! - `.fx-task-card-{ds}-tier-popular` - Popular tier highlight
//! - `.fx-task-card-{ds}-active` - Active task state
//! - `.fx-task-card-{ds}-coming-soon` - Coming soon state
//!
//! Where `{ds}` is the design system (ant, material, etc.)

mod types;
mod task_card;

pub use types::{
    TaskData,
    TaskStatus,
    TaskType,
    TaskDifficulty,
    TaskCardSize,
    TaskCardVariant,
    TaskCardLayout,
    EarningsTier,
    EarningsPeriod,
};

pub use task_card::{TaskCard, TaskCardGrid};
