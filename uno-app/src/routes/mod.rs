//! Page route components

mod home;
mod claim;
mod tasks;
mod guides;
mod faq;
mod contact;
mod referrals;
mod preview;
mod dashboard;
mod account;
mod setup;
mod support;
mod agent;
mod operator;

// Debug routes only available when debug-routes feature is enabled
// WARNING: Never enable in production - exposes sensitive data
#[cfg(feature = "debug-routes")]
mod debug;

pub use home::HomePage;
pub use claim::ClaimPage;
pub use tasks::TasksPage;
pub use guides::GuidesPage;
pub use faq::FaqPage;
pub use contact::ContactPage;
pub use referrals::ReferralsPage;
pub use preview::PreviewPage;
pub use dashboard::DashboardPage;
pub use account::AccountPage;
pub use setup::SetupPage;
pub use support::SupportPage;
pub use agent::AgentPage;
pub use operator::OperatorPage;

#[cfg(feature = "debug-routes")]
pub use debug::DebugPage;
