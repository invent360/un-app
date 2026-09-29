//! Economics components for revenue split transparency
//!
//! These components display the 50/40/10 revenue split model
//! and help users understand how earnings are distributed.

mod earnings_dashboard;
mod referral_attribution;
mod split_display;

pub use earnings_dashboard::EarningsDashboard;
pub use referral_attribution::ReferralAttribution;
pub use split_display::{SplitDisplay, SplitBar, SplitCard};
