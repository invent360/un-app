//! Wizard configuration types.
//!
//! Defines configuration structures for customizing wizard behavior.

use super::types::WizardStage;

/// Configuration for button labels.
///
/// Allows customization of navigation button labels based on context.
#[derive(Debug, Clone)]
pub struct ButtonConfig<T: WizardStage> {
    /// Function to generate next button label.
    /// Parameters: (current_stage, is_last_stage) -> label
    pub next_label: fn(&T, bool) -> String,
    /// Function to generate previous button label.
    /// Parameters: (current_stage, is_first_stage) -> label
    pub prev_label: fn(&T, bool) -> String,
    /// Static label for the submit button.
    pub submit_label: String,
    /// Static label for the cancel button.
    pub cancel_label: String,
}

impl<T: WizardStage> Default for ButtonConfig<T> {
    fn default() -> Self {
        Self {
            next_label: |_, is_last| {
                if is_last {
                    "Finish".to_string()
                } else {
                    "Next".to_string()
                }
            },
            prev_label: |_, is_first| {
                if is_first {
                    "Cancel".to_string()
                } else {
                    "Back".to_string()
                }
            },
            submit_label: "Submit".to_string(),
            cancel_label: "Cancel".to_string(),
        }
    }
}

impl<T: WizardStage> ButtonConfig<T> {
    /// Creates a new button config with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a custom next label function.
    pub fn with_next_label(mut self, f: fn(&T, bool) -> String) -> Self {
        self.next_label = f;
        self
    }

    /// Sets a custom previous label function.
    pub fn with_prev_label(mut self, f: fn(&T, bool) -> String) -> Self {
        self.prev_label = f;
        self
    }

    /// Sets the submit button label.
    pub fn with_submit_label(mut self, label: impl Into<String>) -> Self {
        self.submit_label = label.into();
        self
    }

    /// Sets the cancel button label.
    pub fn with_cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    /// Gets the next button label for the given stage.
    pub fn get_next_label(&self, stage: &T, is_last: bool) -> String {
        (self.next_label)(stage, is_last)
    }

    /// Gets the previous button label for the given stage.
    pub fn get_prev_label(&self, stage: &T, is_first: bool) -> String {
        (self.prev_label)(stage, is_first)
    }
}

/// Step configuration for the wizard.
///
/// Defines the sequence of steps and their display names.
#[derive(Debug, Clone)]
pub struct StepConfig<T: WizardStage> {
    /// Ordered list of stages.
    pub steps: Vec<T>,
    /// Display names for each step (optional, falls back to stage display_name).
    pub step_names: Vec<String>,
}

impl<T: WizardStage> Default for StepConfig<T> {
    fn default() -> Self {
        let stages = T::all_stages();
        let names = stages.iter().map(|s| s.display_name().to_string()).collect();
        Self {
            steps: stages,
            step_names: names,
        }
    }
}

impl<T: WizardStage> StepConfig<T> {
    /// Creates a new step config from all stages.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a step config with specific steps.
    pub fn with_steps(steps: Vec<T>) -> Self {
        let names = steps.iter().map(|s| s.display_name().to_string()).collect();
        Self { steps, step_names: names }
    }

    /// Creates a step config with custom names.
    pub fn with_custom_names(steps: Vec<T>, names: Vec<String>) -> Self {
        Self { steps, step_names: names }
    }

    /// Returns the total number of steps.
    pub fn total_steps(&self) -> usize {
        self.steps.len()
    }

    /// Returns whether the configuration is empty.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Gets the index of a stage in the step list.
    pub fn get_stage_index(&self, stage: &T) -> Option<usize> {
        self.steps.iter().position(|s| s == stage)
    }

    /// Gets the stage at a given index.
    pub fn get_stage_at(&self, index: usize) -> Option<&T> {
        self.steps.get(index)
    }

    /// Gets the next stage after the given stage.
    pub fn get_next_stage(&self, current: &T) -> Option<&T> {
        self.get_stage_index(current)
            .and_then(|idx| self.steps.get(idx + 1))
    }

    /// Gets the previous stage before the given stage.
    pub fn get_prev_stage(&self, current: &T) -> Option<&T> {
        self.get_stage_index(current)
            .filter(|&idx| idx > 0)
            .and_then(|idx| self.steps.get(idx - 1))
    }

    /// Returns whether the given stage is the first.
    pub fn is_first_stage(&self, stage: &T) -> bool {
        self.steps.first().map(|s| s == stage).unwrap_or(false)
    }

    /// Returns whether the given stage is the last.
    pub fn is_last_stage(&self, stage: &T) -> bool {
        self.steps.last().map(|s| s == stage).unwrap_or(false)
    }

    /// Gets the name for a stage.
    pub fn get_step_name(&self, stage: &T) -> String {
        self.get_stage_index(stage)
            .and_then(|idx| self.step_names.get(idx))
            .cloned()
            .unwrap_or_else(|| stage.display_name().to_string())
    }

    /// Filters steps based on a predicate.
    pub fn filtered<F>(&self, predicate: F) -> Self
    where
        F: Fn(&T) -> bool,
    {
        let filtered: Vec<_> = self.steps.iter()
            .enumerate()
            .filter(|(_, stage)| predicate(stage))
            .map(|(idx, stage)| (stage.clone(), self.step_names.get(idx).cloned().unwrap_or_default()))
            .collect();

        let (steps, names): (Vec<_>, Vec<_>) = filtered.into_iter().unzip();
        Self { steps, step_names: names }
    }
}

/// Complete wizard configuration.
///
/// Combines all configuration options for a wizard instance.
#[derive(Debug, Clone)]
pub struct WizardConfig<T: WizardStage> {
    /// Wizard title.
    pub title: String,
    /// Wizard description (optional).
    pub description: Option<String>,
    /// Step configuration.
    pub steps: StepConfig<T>,
    /// Button configuration.
    pub buttons: ButtonConfig<T>,
    /// Whether to show progress indicator.
    pub show_progress: bool,
    /// Whether to show step indicator.
    pub show_steps: bool,
    /// Whether steps are clickable for direct navigation.
    pub clickable_steps: bool,
    /// Whether to show cancel button.
    pub show_cancel: bool,
    /// Whether to validate before navigation.
    pub validate_on_navigate: bool,
}

impl<T: WizardStage> Default for WizardConfig<T> {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: None,
            steps: StepConfig::default(),
            buttons: ButtonConfig::default(),
            show_progress: true,
            show_steps: true,
            clickable_steps: false,
            show_cancel: false,
            validate_on_navigate: true,
        }
    }
}

impl<T: WizardStage> WizardConfig<T> {
    /// Creates a new wizard config with a title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }

    /// Sets the wizard description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the step configuration.
    pub fn with_steps(mut self, steps: StepConfig<T>) -> Self {
        self.steps = steps;
        self
    }

    /// Sets the button configuration.
    pub fn with_buttons(mut self, buttons: ButtonConfig<T>) -> Self {
        self.buttons = buttons;
        self
    }

    /// Enables or disables the progress indicator.
    pub fn with_progress(mut self, show: bool) -> Self {
        self.show_progress = show;
        self
    }

    /// Enables or disables the step indicator.
    pub fn with_step_indicator(mut self, show: bool) -> Self {
        self.show_steps = show;
        self
    }

    /// Enables or disables clickable steps.
    pub fn with_clickable_steps(mut self, clickable: bool) -> Self {
        self.clickable_steps = clickable;
        self
    }

    /// Enables or disables the cancel button.
    pub fn with_cancel_button(mut self, show: bool) -> Self {
        self.show_cancel = show;
        self
    }

    /// Enables or disables validation on navigation.
    pub fn with_validate_on_navigate(mut self, validate: bool) -> Self {
        self.validate_on_navigate = validate;
        self
    }

    /// Gets the next button label for the current stage.
    pub fn get_next_label(&self, stage: &T) -> String {
        let is_last = self.steps.is_last_stage(stage);
        self.buttons.get_next_label(stage, is_last)
    }

    /// Gets the previous button label for the current stage.
    pub fn get_prev_label(&self, stage: &T) -> String {
        let is_first = self.steps.is_first_stage(stage);
        self.buttons.get_prev_label(stage, is_first)
    }
}
