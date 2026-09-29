//! Wizard stage components
//!
//! Each stage of the claim wizard has its own dedicated component:
//! - EconomicsReviewStage: Show 50/40/10 split transparency
//! - ReviewStage: Review license details, validity period, and accept terms
//! - ReserveStage: Atomic license reservation in progress
//! - ClaimStage: Claim the license and show license key
//! - WhatNextStage: Show guides, tasks, and next steps
//! - SoldOutStage: All licenses have been claimed
//! - ComingSoonStage: No licenses exist in the system yet

mod economics_review;
mod review;
mod reserve;
mod claim;
mod what_next;
mod availability;

pub use economics_review::{EconomicsReviewStage, SplitCalculator};
pub use review::ReviewStage;
pub use reserve::ReserveStage;
pub use claim::ClaimStage;
pub use what_next::WhatNextStage;
pub use availability::{SoldOutStage, ComingSoonStage};
