//! Homepage-specific components
//!
//! Task-based earnings section, calculator, carousel, and Start Earning modal

mod earnings_calculator;
mod start_earning_modal;
mod task_carousel;
mod task_section;

pub use earnings_calculator::EarningsCalculator;
pub use start_earning_modal::{StartEarningModal, StartEarningButton, ClaimResult};
pub use task_carousel::TaskCarousel;
pub use task_section::{TaskBasedEarningsSection, TaskItem, TaskStatus, DeviceType};
