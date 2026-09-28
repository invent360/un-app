//! Guide components for displaying step-by-step tutorials and walkthroughs.
//!
//! This module provides five variants for displaying guide content:
//!
//! - **GuideCarousel**: Full-screen/modal carousel with swipeable slides
//! - **GuideDrawer**: Side panel stepper with vertical navigation
//! - **GuideAccordion**: Inline expandable sections
//! - **GuideTour**: Spotlight tour that highlights UI elements
//! - **GuideStepper**: Wizard-like workflow with numbered steps (horizontal/vertical)
//!
//! # Example
//!
//! ```ignore
//! use ember_fx_components::{GuideCarousel, GuideStep, GuideSize};
//!
//! let steps = vec![
//!     GuideStep::new("1", "Welcome", "Get started with our app")
//!         .image("/images/step1.png"),
//!     GuideStep::new("2", "Setup", "Configure your preferences")
//!         .image("/images/step2.png"),
//! ];
//!
//! let active = RwSignal::new(0usize);
//! let visible = RwSignal::new(true);
//!
//! view! {
//!     <GuideCarousel
//!         steps=steps
//!         active_step=active
//!         visible=visible
//!         title="Getting Started"
//!     />
//! }
//! ```

mod types;
mod guide_carousel;
mod guide_drawer;
mod guide_accordion;
mod guide_tour;
mod guide_stepper;
mod guide_step;
mod guide_progress;
mod guide_controls;
mod markdown_text;

pub use types::*;
pub use guide_carousel::*;
pub use guide_drawer::*;
pub use guide_accordion::*;
pub use guide_tour::*;
pub use guide_stepper::*;
pub use guide_step::*;
pub use guide_progress::*;
pub use guide_controls::*;
pub use markdown_text::*;
