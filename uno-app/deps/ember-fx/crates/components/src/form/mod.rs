//! Form components module.
//!
//! Provides form-related components including wizards/multi-step forms.

pub mod wizard;

pub use wizard::{
    // Core types
    WizardStage,
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
    StageValidator,
    NoOpValidator,
    FnValidator,
    CompositeValidator,
    AlwaysFailValidator,
    // UI Components
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
