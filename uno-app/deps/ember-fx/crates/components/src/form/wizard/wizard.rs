//! Wizard orchestrator.
//!
//! The main wizard controller that manages state, navigation, and validation.

use super::actions::{WizardAction, WizardResult};
use super::config::WizardConfig;
use super::state::{ValidationError, WizardState};
use super::types::WizardStage;
use super::validator::StageValidator;

/// The main wizard orchestrator.
///
/// Manages wizard state, handles navigation, and coordinates validation.
///
/// # Type Parameters
///
/// - `T`: The stage enum implementing `WizardStage`
/// - `D`: The data type being collected/validated
///
/// # Example
///
/// ```rust
/// let config = WizardConfig::new("User Registration");
/// let wizard = Wizard::new(
///     config,
///     RegistrationStage::Welcome,
///     Box::new(RegistrationValidator),
///     UserData::default(),
/// );
/// ```
pub struct Wizard<T: WizardStage, D> {
    /// Wizard configuration.
    pub config: WizardConfig<T>,
    /// Current wizard state.
    pub state: WizardState<T>,
    /// Stage validator.
    validator: Box<dyn StageValidator<T, D>>,
    /// Wizard data.
    pub data: D,
    /// Initial stage (for reset).
    initial_stage: T,
}

impl<T: WizardStage + std::fmt::Debug, D: std::fmt::Debug> std::fmt::Debug for Wizard<T, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wizard")
            .field("config", &self.config)
            .field("state", &self.state)
            .field("validator", &"<dyn StageValidator>")
            .field("data", &self.data)
            .field("initial_stage", &self.initial_stage)
            .finish()
    }
}

impl<T: WizardStage, D: Clone> Wizard<T, D> {
    /// Creates a new wizard instance.
    pub fn new(
        config: WizardConfig<T>,
        initial_stage: T,
        validator: Box<dyn StageValidator<T, D>>,
        data: D,
    ) -> Self {
        Self {
            config,
            state: WizardState::new(initial_stage.clone()),
            validator,
            data,
            initial_stage,
        }
    }

    /// Returns the current stage.
    pub fn current_stage(&self) -> &T {
        self.state.current_stage()
    }

    /// Returns the wizard configuration.
    pub fn config(&self) -> &WizardConfig<T> {
        &self.config
    }

    /// Returns the wizard state.
    pub fn state(&self) -> &WizardState<T> {
        &self.state
    }

    /// Returns a mutable reference to the wizard state.
    pub fn state_mut(&mut self) -> &mut WizardState<T> {
        &mut self.state
    }

    /// Returns the wizard data.
    pub fn data(&self) -> &D {
        &self.data
    }

    /// Returns a mutable reference to the wizard data.
    pub fn data_mut(&mut self) -> &mut D {
        &mut self.data
    }

    /// Sets the wizard data.
    pub fn set_data(&mut self, data: D) {
        self.data = data;
    }

    /// Returns whether we can go back.
    pub fn can_go_back(&self) -> bool {
        self.state.can_go_back()
    }

    /// Returns whether we can go forward.
    pub fn can_go_forward(&self) -> bool {
        self.state.can_go_forward(&self.config.steps.steps)
    }

    /// Returns whether we're on the first stage.
    pub fn is_first_stage(&self) -> bool {
        self.config.steps.is_first_stage(self.current_stage())
    }

    /// Returns whether we're on the last stage.
    pub fn is_last_stage(&self) -> bool {
        self.config.steps.is_last_stage(self.current_stage())
    }

    /// Returns the progress as (current_step, total_steps).
    pub fn get_progress(&self) -> (usize, usize) {
        self.state.progress(&self.config.steps.steps)
    }

    /// Returns the progress percentage (0-100).
    pub fn get_progress_percentage(&self) -> u32 {
        self.state.progress_percentage(&self.config.steps.steps)
    }

    /// Validates the current stage.
    pub fn validate_current_stage(&self) -> Result<(), ValidationError> {
        self.validator.validate(self.current_stage(), &self.data)
    }

    /// Validates a specific stage.
    pub fn validate_stage(&self, stage: &T) -> Result<(), ValidationError> {
        self.validator.validate(stage, &self.data)
    }

    /// Handles a wizard action and returns the result.
    pub fn handle_action(&mut self, action: WizardAction<T>) -> WizardResult<T> {
        match action {
            WizardAction::Next => self.next(),
            WizardAction::Previous => self.previous(),
            WizardAction::GoTo(stage) => self.navigate_to(stage),
            WizardAction::Submit => self.submit(),
            WizardAction::ValidateAndNext => self.validate_and_next(),
            WizardAction::ShowError(msg) => {
                self.state.add_error(msg);
                WizardResult::NoChange
            }
            WizardAction::DismissError => {
                self.state.dismiss_notification();
                WizardResult::ErrorDismissed
            }
            WizardAction::Reset => self.reset(),
            WizardAction::Cancel => WizardResult::Cancelled,
        }
    }

    /// Moves to the next stage without validation.
    pub fn next(&mut self) -> WizardResult<T> {
        let current = self.current_stage().clone();

        if let Some(next_stage) = self.config.steps.get_next_stage(&current) {
            let next = next_stage.clone();
            self.state.set_current_stage(next.clone());
            self.state.clear_errors();
            WizardResult::StageChanged { from: current, to: next }
        } else {
            WizardResult::NoChange
        }
    }

    /// Validates the current stage and moves to next if valid.
    pub fn validate_and_next(&mut self) -> WizardResult<T> {
        match self.validate_current_stage() {
            Ok(()) => self.next(),
            Err(errors) => {
                let stage = self.current_stage().clone();
                self.state.set_errors(errors.clone());
                WizardResult::ValidationFailed {
                    stage,
                    errors: errors.messages().to_vec(),
                }
            }
        }
    }

    /// Moves to the previous stage.
    pub fn previous(&mut self) -> WizardResult<T> {
        let current = self.current_stage().clone();

        if let Some(prev) = self.state.go_back() {
            WizardResult::StageChanged { from: current, to: prev }
        } else if let Some(prev_stage) = self.config.steps.get_prev_stage(&current) {
            let prev = prev_stage.clone();
            self.state.set_current_stage(prev.clone());
            self.state.clear_errors();
            WizardResult::StageChanged { from: current, to: prev }
        } else {
            WizardResult::Blocked {
                reason: "Already at first stage".to_string(),
            }
        }
    }

    /// Navigates directly to a specific stage.
    pub fn navigate_to(&mut self, stage: T) -> WizardResult<T> {
        // Check if stage is valid
        if !self.config.steps.steps.contains(&stage) {
            return WizardResult::Blocked {
                reason: format!("Invalid stage: {:?}", stage),
            };
        }

        let current = self.current_stage().clone();
        if current == stage {
            return WizardResult::NoChange;
        }

        // If navigating forward with validation enabled, validate current stage
        if self.config.validate_on_navigate {
            let current_idx = self.config.steps.get_stage_index(&current).unwrap_or(0);
            let target_idx = self.config.steps.get_stage_index(&stage).unwrap_or(0);

            if target_idx > current_idx {
                // Validate all stages up to current
                if let Err(errors) = self.validate_current_stage() {
                    self.state.set_errors(errors.clone());
                    return WizardResult::ValidationFailed {
                        stage: current,
                        errors: errors.messages().to_vec(),
                    };
                }
            }
        }

        self.state.set_current_stage(stage.clone());
        self.state.clear_errors();
        WizardResult::StageChanged { from: current, to: stage }
    }

    /// Submits the wizard.
    pub fn submit(&mut self) -> WizardResult<T> {
        // Validate current stage before submission
        if let Err(errors) = self.validate_current_stage() {
            let stage = self.current_stage().clone();
            self.state.set_errors(errors.clone());
            return WizardResult::ValidationFailed {
                stage,
                errors: errors.messages().to_vec(),
            };
        }

        self.state.set_submitted(true);
        WizardResult::Submitted
    }

    /// Resets the wizard to initial state.
    pub fn reset(&mut self) -> WizardResult<T> {
        let initial = self.initial_stage.clone();
        self.state = WizardState::new(initial.clone());
        WizardResult::Reset { initial_stage: initial }
    }

    /// Goes back to initial stage (alias for reset).
    pub fn go_back_to_start(&mut self) -> WizardResult<T> {
        self.reset()
    }

    /// Returns all steps.
    pub fn steps(&self) -> &[T] {
        &self.config.steps.steps
    }

    /// Returns step names.
    pub fn step_names(&self) -> &[String] {
        &self.config.steps.step_names
    }

    /// Returns the total number of steps.
    pub fn total_steps(&self) -> usize {
        self.config.steps.total_steps()
    }

    /// Returns the current step index (1-based).
    pub fn current_step(&self) -> usize {
        self.config
            .steps
            .get_stage_index(self.current_stage())
            .map(|idx| idx + 1)
            .unwrap_or(1)
    }

    /// Returns the name of the current stage.
    pub fn current_stage_name(&self) -> String {
        self.config.steps.get_step_name(self.current_stage())
    }

    /// Returns the next button label for the current stage.
    pub fn get_next_label(&self) -> String {
        self.config.get_next_label(self.current_stage())
    }

    /// Returns the previous button label for the current stage.
    pub fn get_prev_label(&self) -> String {
        self.config.get_prev_label(self.current_stage())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::validator::NoOpValidator;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestStage {
        First,
        Second,
        Third,
    }

    impl WizardStage for TestStage {
        fn display_name(&self) -> &'static str {
            match self {
                Self::First => "First",
                Self::Second => "Second",
                Self::Third => "Third",
            }
        }

        fn all_stages() -> Vec<Self> {
            vec![Self::First, Self::Second, Self::Third]
        }
    }

    #[test]
    fn test_wizard_creation() {
        let config = WizardConfig::new("Test Wizard");
        let wizard: Wizard<TestStage, ()> = Wizard::new(
            config,
            TestStage::First,
            Box::new(NoOpValidator),
            (),
        );

        assert_eq!(wizard.current_stage(), &TestStage::First);
        assert!(wizard.is_first_stage());
        assert!(!wizard.is_last_stage());
    }

    #[test]
    fn test_wizard_navigation() {
        let config = WizardConfig::new("Test Wizard");
        let mut wizard: Wizard<TestStage, ()> = Wizard::new(
            config,
            TestStage::First,
            Box::new(NoOpValidator),
            (),
        );

        // Go to next
        let result = wizard.next();
        assert!(result.is_stage_changed());
        assert_eq!(wizard.current_stage(), &TestStage::Second);

        // Go to next again
        wizard.next();
        assert_eq!(wizard.current_stage(), &TestStage::Third);
        assert!(wizard.is_last_stage());

        // Try to go next on last stage
        let result = wizard.next();
        assert!(result.is_no_change());

        // Go back
        wizard.previous();
        assert_eq!(wizard.current_stage(), &TestStage::Second);
    }

    #[test]
    fn test_wizard_progress() {
        let config = WizardConfig::new("Test Wizard");
        let mut wizard: Wizard<TestStage, ()> = Wizard::new(
            config,
            TestStage::First,
            Box::new(NoOpValidator),
            (),
        );

        assert_eq!(wizard.get_progress(), (1, 3));
        assert_eq!(wizard.get_progress_percentage(), 33);

        wizard.next();
        assert_eq!(wizard.get_progress(), (2, 3));
        assert_eq!(wizard.get_progress_percentage(), 66);

        wizard.next();
        assert_eq!(wizard.get_progress(), (3, 3));
        assert_eq!(wizard.get_progress_percentage(), 100);
    }
}
