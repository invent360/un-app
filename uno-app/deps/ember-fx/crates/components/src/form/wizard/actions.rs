//! Wizard action and result types.
//!
//! Defines the action-result pattern for wizard interactions.

use super::types::WizardStage;

/// Wizard actions that can be performed.
///
/// Encapsulates all possible user interactions with the wizard.
#[derive(Debug, Clone)]
pub enum WizardAction<T: WizardStage> {
    /// Move to the next stage without validation.
    Next,
    /// Move to the previous stage.
    Previous,
    /// Jump directly to a specific stage.
    GoTo(T),
    /// Submit the wizard (final stage only).
    Submit,
    /// Validate current stage and move to next if valid.
    ValidateAndNext,
    /// Show an error message.
    ShowError(String),
    /// Dismiss the error notification.
    DismissError,
    /// Reset the wizard to initial state.
    Reset,
    /// Cancel the wizard.
    Cancel,
}

impl<T: WizardStage> WizardAction<T> {
    /// Creates a Next action.
    pub fn next() -> Self {
        Self::Next
    }

    /// Creates a Previous action.
    pub fn previous() -> Self {
        Self::Previous
    }

    /// Creates a GoTo action.
    pub fn go_to(stage: T) -> Self {
        Self::GoTo(stage)
    }

    /// Creates a Submit action.
    pub fn submit() -> Self {
        Self::Submit
    }

    /// Creates a ValidateAndNext action.
    pub fn validate_and_next() -> Self {
        Self::ValidateAndNext
    }

    /// Creates a ShowError action.
    pub fn show_error(message: impl Into<String>) -> Self {
        Self::ShowError(message.into())
    }

    /// Creates a DismissError action.
    pub fn dismiss_error() -> Self {
        Self::DismissError
    }

    /// Creates a Reset action.
    pub fn reset() -> Self {
        Self::Reset
    }

    /// Creates a Cancel action.
    pub fn cancel() -> Self {
        Self::Cancel
    }
}

/// Result of processing a wizard action.
///
/// Provides feedback about the outcome of an action.
#[derive(Debug, Clone)]
pub enum WizardResult<T: WizardStage> {
    /// Successfully moved to a new stage.
    StageChanged {
        /// The previous stage.
        from: T,
        /// The new current stage.
        to: T,
    },
    /// Validation failed for the current stage.
    ValidationFailed {
        /// The stage that failed validation.
        stage: T,
        /// Error messages.
        errors: Vec<String>,
    },
    /// Wizard was successfully submitted.
    Submitted,
    /// Error notification was dismissed.
    ErrorDismissed,
    /// Wizard was reset to initial state.
    Reset {
        /// The initial stage.
        initial_stage: T,
    },
    /// Wizard was cancelled.
    Cancelled,
    /// No change occurred (e.g., trying to go next on last stage).
    NoChange,
    /// Navigation was blocked (e.g., trying to go back on first stage).
    Blocked {
        /// Reason for blocking.
        reason: String,
    },
}

impl<T: WizardStage> WizardResult<T> {
    /// Returns whether the result represents a successful stage change.
    pub fn is_stage_changed(&self) -> bool {
        matches!(self, Self::StageChanged { .. })
    }

    /// Returns whether the result represents a validation failure.
    pub fn is_validation_failed(&self) -> bool {
        matches!(self, Self::ValidationFailed { .. })
    }

    /// Returns whether the result represents a successful submission.
    pub fn is_submitted(&self) -> bool {
        matches!(self, Self::Submitted)
    }

    /// Returns whether no change occurred.
    pub fn is_no_change(&self) -> bool {
        matches!(self, Self::NoChange)
    }

    /// Returns the new stage if the result is a stage change.
    pub fn new_stage(&self) -> Option<&T> {
        match self {
            Self::StageChanged { to, .. } => Some(to),
            _ => None,
        }
    }

    /// Returns the error messages if the result is a validation failure.
    pub fn errors(&self) -> Option<&[String]> {
        match self {
            Self::ValidationFailed { errors, .. } => Some(errors),
            _ => None,
        }
    }
}
