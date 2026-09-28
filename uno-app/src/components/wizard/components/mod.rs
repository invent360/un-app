//! Wizard sub-components

mod variant_card;
mod task_accordion;
mod earnings_compare;
mod install_guide;
mod download_buttons;
mod progress_bar;
mod step_carousel;

pub use variant_card::WizardVariantCard;
pub use task_accordion::TaskAccordion;
pub use earnings_compare::EarningsCompare;
pub use install_guide::InstallGuide;
pub use download_buttons::DownloadButtons;
pub use progress_bar::WizardProgressBar;
pub use step_carousel::{StepCarousel, SlideConfig, MediaType, parse_slides_from_json};
