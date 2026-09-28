//! Wizard stage components
//!
//! Each stage of the claim wizard has its own dedicated component:
//! - ReviewStage: Review license details, validity period, and accept terms
//! - ClaimStage: Claim the license and show license key
//! - WhatNextStage: Show guides, tasks, and next steps
//! - SoldOutStage: All licenses have been claimed
//! - ComingSoonStage: No licenses exist in the system yet

mod review;
mod claim;
mod what_next;
mod availability;

pub use review::ReviewStage;
pub use claim::ClaimStage;
pub use what_next::WhatNextStage;
pub use availability::{SoldOutStage, ComingSoonStage};
