//! Shared type definitions used across client and server

mod license;
mod error;
mod faq;
mod stats;
mod pagination;
mod content;
mod content_admin;
mod content_review;
mod referral;
pub mod audit;
pub mod rbac;
pub mod schema;  // Schema-driven CMS types
pub mod onboarding;  // R3-13: Onboarding journey state machine

pub use license::*;
pub use error::*;
pub use faq::*;
pub use stats::*;
pub use pagination::*;
pub use content::*;
pub use content_admin::*;
pub use content_review::*;
pub use referral::*;
pub use audit::*;
pub use schema::*;
pub use onboarding::*;
