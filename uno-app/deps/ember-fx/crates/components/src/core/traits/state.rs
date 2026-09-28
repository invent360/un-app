//! State traits for component state management.
//!
//! These traits handle common component states like disabled and loading.

/// Trait for components that can be disabled.
///
/// Disabled components should:
/// - Not respond to user interaction
/// - Have reduced visual opacity
/// - Have appropriate ARIA attributes
///
/// # Example
///
/// ```ignore
/// use ember_fx::core::DisabledState;
///
/// struct Button {
///     disabled: bool,
/// }
///
/// impl DisabledState for Button {
///     fn is_disabled(&self) -> bool {
///         self.disabled
///     }
/// }
/// ```
pub trait DisabledState {
    /// Check if the component is currently disabled.
    fn is_disabled(&self) -> bool;

    /// Generate the disabled CSS class if disabled.
    fn disabled_class(&self, base_class: &str) -> Option<String> {
        if self.is_disabled() {
            Some(format!("{}-disabled", base_class))
        } else {
            None
        }
    }

    /// Get the `aria-disabled` attribute value.
    fn aria_disabled(&self) -> Option<&'static str> {
        if self.is_disabled() {
            Some("true")
        } else {
            None
        }
    }
}

/// Trait for components that can show a loading state.
///
/// Loading components should:
/// - Display a loading indicator (spinner, skeleton, etc.)
/// - Optionally disable interaction while loading
/// - Have appropriate ARIA attributes
///
/// # Example
///
/// ```ignore
/// use ember_fx::core::LoadingState;
///
/// struct Button {
///     loading: bool,
/// }
///
/// impl LoadingState for Button {
///     fn is_loading(&self) -> bool {
///         self.loading
///     }
/// }
/// ```
pub trait LoadingState {
    /// Check if the component is currently loading.
    fn is_loading(&self) -> bool;

    /// Generate the loading CSS class if loading.
    fn loading_class(&self, base_class: &str) -> Option<String> {
        if self.is_loading() {
            Some(format!("{}-loading", base_class))
        } else {
            None
        }
    }

    /// Check if the component should be disabled while loading.
    ///
    /// Default is `true` - override if interaction should remain enabled.
    fn disable_while_loading(&self) -> bool {
        true
    }

    /// Get the `aria-busy` attribute value.
    fn aria_busy(&self) -> Option<&'static str> {
        if self.is_loading() {
            Some("true")
        } else {
            None
        }
    }
}

/// Trait for components with read-only state.
///
/// Read-only components display data but don't allow editing.
/// Unlike disabled, they should still be focusable and readable.
pub trait ReadOnlyState {
    /// Check if the component is read-only.
    fn is_readonly(&self) -> bool;

    /// Generate the read-only CSS class if read-only.
    fn readonly_class(&self, base_class: &str) -> Option<String> {
        if self.is_readonly() {
            Some(format!("{}-readonly", base_class))
        } else {
            None
        }
    }

    /// Get the `aria-readonly` attribute value.
    fn aria_readonly(&self) -> Option<&'static str> {
        if self.is_readonly() {
            Some("true")
        } else {
            None
        }
    }
}

/// Trait for components with required state (form inputs).
pub trait RequiredState {
    /// Check if the component is required.
    fn is_required(&self) -> bool;

    /// Generate the required CSS class.
    fn required_class(&self, base_class: &str) -> Option<String> {
        if self.is_required() {
            Some(format!("{}-required", base_class))
        } else {
            None
        }
    }

    /// Get the `aria-required` attribute value.
    fn aria_required(&self) -> Option<&'static str> {
        if self.is_required() {
            Some("true")
        } else {
            None
        }
    }
}

/// Trait for components with error state (form validation).
pub trait ErrorState {
    /// Check if the component has an error.
    fn has_error(&self) -> bool;

    /// Get the error message, if any.
    fn error_message(&self) -> Option<&str> {
        None
    }

    /// Generate the error CSS class.
    fn error_class(&self, base_class: &str) -> Option<String> {
        if self.has_error() {
            Some(format!("{}-error", base_class))
        } else {
            None
        }
    }

    /// Get the `aria-invalid` attribute value.
    fn aria_invalid(&self) -> Option<&'static str> {
        if self.has_error() {
            Some("true")
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestButton {
        disabled: bool,
        loading: bool,
    }

    impl DisabledState for TestButton {
        fn is_disabled(&self) -> bool {
            self.disabled
        }
    }

    impl LoadingState for TestButton {
        fn is_loading(&self) -> bool {
            self.loading
        }
    }

    #[test]
    fn test_disabled_state() {
        let btn = TestButton { disabled: true, loading: false };
        assert!(btn.is_disabled());
        assert_eq!(btn.disabled_class("fx-btn"), Some("fx-btn-disabled".to_string()));
        assert_eq!(btn.aria_disabled(), Some("true"));
    }

    #[test]
    fn test_loading_state() {
        let btn = TestButton { disabled: false, loading: true };
        assert!(btn.is_loading());
        assert_eq!(btn.loading_class("fx-btn"), Some("fx-btn-loading".to_string()));
        assert_eq!(btn.aria_busy(), Some("true"));
    }
}
