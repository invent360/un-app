//! Wizard state management.
//!
//! Defines the state structure for tracking wizard progress and errors.

use super::types::WizardStage;

/// Validation error container.
///
/// Collects validation errors for display and processing.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ValidationError {
    errors: Vec<String>,
}

impl ValidationError {
    /// Creates a new empty validation error container.
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Creates a validation error with a single message.
    pub fn with_message(message: impl Into<String>) -> Self {
        Self {
            errors: vec![message.into()],
        }
    }

    /// Adds an error message.
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
    }

    /// Returns whether there are any errors.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns all error messages.
    pub fn messages(&self) -> &[String] {
        &self.errors
    }

    /// Clears all errors.
    pub fn clear(&mut self) {
        self.errors.clear();
    }

    /// Returns the number of errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Returns whether there are no errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Merges another validation error into this one.
    pub fn merge(&mut self, other: ValidationError) {
        self.errors.extend(other.errors);
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.errors.join("; "))
    }
}

impl std::error::Error for ValidationError {}

impl From<String> for ValidationError {
    fn from(s: String) -> Self {
        Self::with_message(s)
    }
}

impl From<&str> for ValidationError {
    fn from(s: &str) -> Self {
        Self::with_message(s)
    }
}

/// Wizard state tracking structure.
///
/// Maintains the current state of the wizard including the current stage,
/// navigation capabilities, and error state.
#[derive(Debug, Clone)]
pub struct WizardState<T: WizardStage> {
    /// The current active stage.
    current_stage: T,
    /// Validation errors for the current stage.
    errors: ValidationError,
    /// Whether to show error notification.
    show_notification: bool,
    /// Whether the wizard has been submitted.
    submitted: bool,
    /// Whether the wizard is currently submitting.
    submitting: bool,
    /// History of visited stages for back navigation.
    history: Vec<T>,
}

impl<T: WizardStage> WizardState<T> {
    /// Creates a new wizard state starting at the given stage.
    pub fn new(initial_stage: T) -> Self {
        Self {
            current_stage: initial_stage,
            errors: ValidationError::new(),
            show_notification: false,
            submitted: false,
            submitting: false,
            history: Vec::new(),
        }
    }

    /// Returns the current stage.
    pub fn current_stage(&self) -> &T {
        &self.current_stage
    }

    /// Sets the current stage.
    pub fn set_current_stage(&mut self, stage: T) {
        // Add current stage to history before changing
        if self.current_stage != stage {
            self.history.push(self.current_stage.clone());
        }
        self.current_stage = stage;
    }

    /// Returns the current stage index (0-based).
    pub fn current_index(&self) -> usize {
        self.current_stage.index()
    }

    /// Returns whether this is the first stage.
    pub fn is_first_stage(&self) -> bool {
        self.current_index() == 0
    }

    /// Returns whether this is the last stage.
    pub fn is_last_stage(&self, steps: &[T]) -> bool {
        steps
            .iter()
            .position(|s| s == &self.current_stage)
            .map(|idx| idx == steps.len() - 1)
            .unwrap_or(false)
    }

    /// Returns whether back navigation is possible.
    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty()
    }

    /// Returns whether forward navigation is possible.
    pub fn can_go_forward(&self, steps: &[T]) -> bool {
        !self.is_last_stage(steps)
    }

    /// Goes back to the previous stage.
    pub fn go_back(&mut self) -> Option<T> {
        self.history.pop().map(|stage| {
            self.current_stage = stage.clone();
            self.clear_errors();
            stage
        })
    }

    /// Returns the validation errors.
    pub fn errors(&self) -> &ValidationError {
        &self.errors
    }

    /// Returns the error messages.
    pub fn error_messages(&self) -> &[String] {
        self.errors.messages()
    }

    /// Returns whether there are errors.
    pub fn has_errors(&self) -> bool {
        self.errors.has_errors()
    }

    /// Sets validation errors.
    pub fn set_errors(&mut self, errors: ValidationError) {
        self.errors = errors;
        self.show_notification = self.errors.has_errors();
    }

    /// Adds an error message.
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.add_error(error);
        self.show_notification = true;
    }

    /// Clears all errors.
    pub fn clear_errors(&mut self) {
        self.errors.clear();
        self.show_notification = false;
    }

    /// Returns whether to show error notification.
    pub fn show_notification(&self) -> bool {
        self.show_notification
    }

    /// Dismisses the error notification without clearing errors.
    pub fn dismiss_notification(&mut self) {
        self.show_notification = false;
    }

    /// Returns whether the wizard has been submitted.
    pub fn is_submitted(&self) -> bool {
        self.submitted
    }

    /// Marks the wizard as submitted.
    pub fn set_submitted(&mut self, submitted: bool) {
        self.submitted = submitted;
    }

    /// Returns whether the wizard is currently submitting.
    pub fn is_submitting(&self) -> bool {
        self.submitting
    }

    /// Sets the submitting state.
    pub fn set_submitting(&mut self, submitting: bool) {
        self.submitting = submitting;
    }

    /// Returns the navigation history.
    pub fn history(&self) -> &[T] {
        &self.history
    }

    /// Clears the navigation history.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Returns the progress as (current_step, total_steps).
    pub fn progress(&self, steps: &[T]) -> (usize, usize) {
        let current = steps
            .iter()
            .position(|s| s == &self.current_stage)
            .map(|idx| idx + 1)
            .unwrap_or(1);
        (current, steps.len())
    }

    /// Returns the progress percentage (0-100).
    pub fn progress_percentage(&self, steps: &[T]) -> u32 {
        let (current, total) = self.progress(steps);
        if total == 0 {
            0
        } else {
            ((current as f32 / total as f32) * 100.0) as u32
        }
    }
}
