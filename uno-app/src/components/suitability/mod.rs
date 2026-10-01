//! Suitability/Eligibility stepper component for R5-13
//!
//! A modal wizard that guides users through eligibility checking
//! before they can claim a license.
//!
//! ## Stages
//! 1. Country/Device - Detect country, verify device compatibility
//! 2. Connectivity - Check existing data/power situation
//! 3. Net Benefit - Show realistic earnings vs costs
//! 4. Terms & Consent - Accept terms, optional marketing
//!
//! ## Features
//! - Persists answers to localStorage for session resumption
//! - Explains unknown/ineligible/waitlist clearly
//! - No deposit or phone purchase required

pub mod state;
mod stepper;
pub mod stages;

pub use state::{
    provide_suitability_context, use_suitability_state,
    SuitabilityState, SuitabilityStage, SuitabilityAnswers,
    DeviceType, ConnectivityQuality, PowerSituation,
};
pub use stepper::SuitabilityStepper;
