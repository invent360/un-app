//! CMS-driven section components for the home page
//!
//! These components render home page sections from CMS data
//! instead of hardcoded translations.

mod hero;
mod how_it_works;
mod earnings;
mod testimonials;
mod faq;

// Only export the main section components, not internal helpers
pub use hero::CmsHeroSection;
pub use how_it_works::CmsHowItWorksSection;
pub use earnings::CmsEarningsSection;
pub use testimonials::CmsTestimonialsSection;
pub use faq::CmsFaqSection;
