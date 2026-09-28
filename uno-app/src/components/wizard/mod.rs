//! License claiming wizard component
//!
//! A 5-stage wizard for claiming and onboarding:
//! 1. Review - Review license details before claiming
//! 2. Claim - License claimed, shows key and download info
//! 3. Download - Download & install the app
//! 4. Sign Up - Create account in the app
//! 5. Activate - Start earning

mod claim_wizard;
mod state;
pub mod stages;
pub mod components;

pub use claim_wizard::ClaimWizard;
pub use state::{ClaimWizardStage, ClaimWizardState, use_wizard_state, provide_wizard_context};
