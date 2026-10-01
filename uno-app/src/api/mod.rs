//! Server functions (Leptos server fns)
//!
//! These functions are compiled for both client (as remote calls)
//! and server (as actual implementations).

mod chatbot;
mod licenses;
mod stats;
pub mod faq;
mod guides;
mod tasks;
mod referrals;
mod cms_review;
mod home;
pub mod dashboard;

pub use chatbot::*;
pub use licenses::*;
pub use stats::*;
pub use faq::*;
pub use guides::*;
pub use tasks::*;
pub use referrals::*;
pub use cms_review::*;
pub use home::*;
pub use dashboard::*;

// Re-export server function registration (SSR only)
#[cfg(feature = "ssr")]
pub use faq::register_faq_server_fns;
#[cfg(feature = "ssr")]
pub use guides::register_guides_server_fns;
#[cfg(feature = "ssr")]
pub use tasks::register_tasks_server_fns;
#[cfg(feature = "ssr")]
pub use home::register_home_server_fns;
