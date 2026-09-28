//! Wizard component module.
//!
//! A comprehensive wizard/multi-step form component system with:
//! - Type-safe stage management via the `WizardStage` trait
//! - Flexible validation through the `StageValidator` trait
//! - Action-result pattern for state changes
//! - Responsive UI with Ant Design styling
//!
//! # Architecture
//!
//! The wizard system is split into business logic and UI layers:
//!
//! **Business Logic:**
//! - `WizardStage` - Trait for stage enums
//! - `Wizard` - Main orchestrator managing state and navigation
//! - `WizardState` - State tracking (current stage, errors, etc.)
//! - `WizardConfig` - Configuration (title, steps, buttons)
//! - `WizardAction/WizardResult` - Action-result pattern
//! - `StageValidator` - Validation interface
//!
//! **UI Components:**
//! - `WizardContainer` - Main container component
//! - `WizardStepper` - Step indicator
//! - `WizardProgress` - Progress bar/circle
//! - `WizardNavigation` - Previous/Next/Submit buttons
//! - `WizardErrorDisplay` - Error message display
//!
//! # Example
//!
//! ```rust
//! use ember_fx_components::form::wizard::*;
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//! enum RegistrationStage {
//!     Welcome,
//!     PersonalInfo,
//!     Preferences,
//!     Review,
//! }
//!
//! impl WizardStage for RegistrationStage {
//!     fn display_name(&self) -> &'static str {
//!         match self {
//!             Self::Welcome => "Welcome",
//!             Self::PersonalInfo => "Personal Info",
//!             Self::Preferences => "Preferences",
//!             Self::Review => "Review",
//!         }
//!     }
//!
//!     fn all_stages() -> Vec<Self> {
//!         vec![Self::Welcome, Self::PersonalInfo, Self::Preferences, Self::Review]
//!     }
//! }
//!
//! // In a component:
//! let (current, set_current) = signal(0usize);
//! let steps = Signal::derive(|| RegistrationStage::all_stages());
//!
//! view! {
//!     <WizardContainer
//!         title="User Registration"
//!         steps=steps
//!         current=current
//!         on_previous=move |_| set_current.update(|c| if *c > 0 { *c -= 1 })
//!         on_next=move |_| set_current.update(|c| *c += 1)
//!         on_submit=move |_| { /* handle submit */ }
//!     >
//!         // Step content here
//!     </WizardContainer>
//! }
//! ```

// Core types and traits
mod types;
mod state;
mod config;
mod actions;
mod validator;
mod wizard;

// UI components
pub mod ui;

// Re-export core types
pub use types::{
    WizardStage,
    WizardLayout,
    StepperOrientation,
    NavigationAlignment,
    ErrorDisplayStyle,
    ProgressStyle,
};

// Re-export state types
pub use state::{
    ValidationError,
    WizardState,
};

// Re-export config types
pub use config::{
    ButtonConfig,
    StepConfig,
    WizardConfig,
};

// Re-export action types
pub use actions::{
    WizardAction,
    WizardResult,
};

// Re-export validator types
pub use validator::{
    StageValidator,
    NoOpValidator,
    AlwaysFailValidator,
    CompositeValidator,
    FnValidator,
};

// Re-export wizard orchestrator
pub use wizard::Wizard;

// Re-export UI components
pub use ui::{
    WizardContainer,
    WizardStep,
    WizardContent,
    WizardStepper,
    StepItem,
    WizardProgress,
    WizardCircularProgress,
    WizardStepCounter,
    WizardNavigation,
    WizardCompactNavigation,
    NavigationLabels,
    WizardErrorDisplay,
    WizardSuccessDisplay,
};

/// Prelude for convenient imports.
pub mod prelude {
    pub use super::{
        // Core traits
        WizardStage,
        StageValidator,
        // Types
        WizardLayout,
        StepperOrientation,
        NavigationAlignment,
        ErrorDisplayStyle,
        ProgressStyle,
        ValidationError,
        // Config
        WizardConfig,
        StepConfig,
        ButtonConfig,
        // Actions
        WizardAction,
        WizardResult,
        // State
        WizardState,
        // Orchestrator
        Wizard,
        // Validators
        NoOpValidator,
        FnValidator,
        // UI Components
        WizardContainer,
        WizardStep,
        WizardStepper,
        WizardProgress,
        WizardNavigation,
        WizardErrorDisplay,
    };
}
