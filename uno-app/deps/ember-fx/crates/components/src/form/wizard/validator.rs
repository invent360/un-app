//! Wizard validation traits.
//!
//! Defines the validation interface for wizard stages.

use super::state::ValidationError;
use super::types::WizardStage;

/// Trait for implementing stage-specific validation.
///
/// Implementors can provide custom validation logic for each wizard stage.
///
/// # Example
///
/// ```rust
/// struct UserDataValidator;
///
/// impl StageValidator<UserStage, UserData> for UserDataValidator {
///     fn validate(&self, stage: &UserStage, data: &UserData) -> Result<(), ValidationError> {
///         match stage {
///             UserStage::PersonalInfo => {
///                 let mut errors = ValidationError::new();
///                 if data.name.is_empty() {
///                     errors.add_error("Name is required");
///                 }
///                 if data.email.is_empty() {
///                     errors.add_error("Email is required");
///                 }
///                 if errors.has_errors() {
///                     Err(errors)
///                 } else {
///                     Ok(())
///                 }
///             }
///             _ => Ok(())
///         }
///     }
/// }
/// ```
pub trait StageValidator<T: WizardStage, D>: Send + Sync {
    /// Validates the wizard data for the specified stage.
    ///
    /// Returns `Ok(())` if validation passes, or `Err(ValidationError)` with
    /// error messages if validation fails.
    fn validate(&self, stage: &T, data: &D) -> Result<(), ValidationError>;

    /// Checks if the wizard can proceed from the current stage.
    ///
    /// Default implementation calls `validate()` and checks if the result is `Ok`.
    fn can_proceed(&self, stage: &T, data: &D) -> bool {
        self.validate(stage, data).is_ok()
    }

    /// Validates all stages up to and including the specified stage.
    ///
    /// Useful for checking if the wizard can be submitted.
    fn validate_up_to(&self, stages: &[T], target: &T, data: &D) -> Result<(), ValidationError> {
        let mut all_errors = ValidationError::new();

        for stage in stages {
            if let Err(errors) = self.validate(stage, data) {
                all_errors.merge(errors);
            }
            if stage == target {
                break;
            }
        }

        if all_errors.has_errors() {
            Err(all_errors)
        } else {
            Ok(())
        }
    }
}

/// A no-op validator that allows all stages to pass.
///
/// Useful for wizards without validation or during development.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoOpValidator;

impl<T: WizardStage, D> StageValidator<T, D> for NoOpValidator {
    fn validate(&self, _stage: &T, _data: &D) -> Result<(), ValidationError> {
        Ok(())
    }
}

/// A validator that always fails with a specified message.
///
/// Useful for testing or placeholder validation.
#[derive(Debug, Clone)]
pub struct AlwaysFailValidator {
    message: String,
}

impl AlwaysFailValidator {
    /// Creates a new validator that always fails with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl<T: WizardStage, D> StageValidator<T, D> for AlwaysFailValidator {
    fn validate(&self, _stage: &T, _data: &D) -> Result<(), ValidationError> {
        Err(ValidationError::with_message(self.message.clone()))
    }
}

/// A validator composed of multiple validators.
///
/// Runs all validators and collects all errors.
pub struct CompositeValidator<T: WizardStage, D> {
    validators: Vec<Box<dyn StageValidator<T, D>>>,
}

impl<T: WizardStage, D> CompositeValidator<T, D> {
    /// Creates a new composite validator.
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    /// Adds a validator to the composite.
    pub fn add<V: StageValidator<T, D> + 'static>(mut self, validator: V) -> Self {
        self.validators.push(Box::new(validator));
        self
    }
}

impl<T: WizardStage, D> Default for CompositeValidator<T, D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: WizardStage, D> StageValidator<T, D> for CompositeValidator<T, D> {
    fn validate(&self, stage: &T, data: &D) -> Result<(), ValidationError> {
        let mut all_errors = ValidationError::new();

        for validator in &self.validators {
            if let Err(errors) = validator.validate(stage, data) {
                all_errors.merge(errors);
            }
        }

        if all_errors.has_errors() {
            Err(all_errors)
        } else {
            Ok(())
        }
    }
}

/// A validator that applies a closure.
///
/// Convenient for simple inline validation logic.
pub struct FnValidator<F> {
    validate_fn: F,
}

impl<F> FnValidator<F> {
    /// Creates a new function-based validator.
    pub fn new(f: F) -> Self {
        Self { validate_fn: f }
    }
}

impl<T, D, F> StageValidator<T, D> for FnValidator<F>
where
    T: WizardStage,
    F: Fn(&T, &D) -> Result<(), ValidationError> + Send + Sync,
{
    fn validate(&self, stage: &T, data: &D) -> Result<(), ValidationError> {
        (self.validate_fn)(stage, data)
    }
}
