//! License-related components

mod android_download;
mod help_accordion;
mod license_reveal;
mod variant_card;

pub use android_download::AndroidDownload;
pub use help_accordion::HelpAccordion;
pub use license_reveal::LicenseReveal;
pub use variant_card::VariantCard;

// Re-export UrgencyBar from features for backward compatibility
pub use crate::features::license::UrgencyBar;
