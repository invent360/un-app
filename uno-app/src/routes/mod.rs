//! Page route components

mod home;
mod claim;
mod tasks;
mod guides;
mod faq;
mod contact;
mod referrals;
mod preview;
mod debug;

pub use home::HomePage;
pub use claim::ClaimPage;
pub use tasks::TasksPage;
pub use guides::GuidesPage;
pub use faq::FaqPage;
pub use contact::ContactPage;
pub use referrals::ReferralsPage;
pub use preview::PreviewPage;
pub use debug::DebugPage;
