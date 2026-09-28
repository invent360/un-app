//! Interaction traits for user input handling.
//!
//! These traits define how components respond to user interactions
//! like clicks, focus, and hover events.

/// Trait for clickable components.
///
/// Components implementing this trait respond to click/tap events.
pub trait Clickable {
    /// Check if the component is currently clickable.
    ///
    /// Returns `false` if disabled or otherwise non-interactive.
    fn is_clickable(&self) -> bool {
        true
    }

    /// Get the cursor style for this component.
    fn cursor_style(&self) -> &'static str {
        if self.is_clickable() {
            "pointer"
        } else {
            "default"
        }
    }
}

/// Trait for focusable components.
///
/// Components implementing this trait can receive keyboard focus.
pub trait Focusable {
    /// Check if the component can receive focus.
    fn is_focusable(&self) -> bool {
        true
    }

    /// Get the tabindex value.
    ///
    /// - `0`: Normal tab order
    /// - `-1`: Programmatically focusable only
    /// - `> 0`: Explicit tab order (discouraged)
    fn tab_index(&self) -> i32 {
        if self.is_focusable() { 0 } else { -1 }
    }

    /// Generate focus-visible CSS class.
    fn focus_class(&self, base_class: &str) -> String {
        format!("{}-focus", base_class)
    }

    /// Generate focus-within CSS class (for containers).
    fn focus_within_class(&self, base_class: &str) -> String {
        format!("{}-focus-within", base_class)
    }
}

/// Trait for hoverable components.
///
/// Components implementing this trait have hover states.
pub trait Hoverable {
    /// Check if hover effects are enabled.
    fn is_hoverable(&self) -> bool {
        true
    }

    /// Generate hover CSS class.
    fn hover_class(&self, base_class: &str) -> String {
        format!("{}-hover", base_class)
    }
}

/// Trait for pressable components (button-like).
///
/// Components implementing this trait have active/pressed states.
pub trait Pressable {
    /// Check if the component shows press feedback.
    fn is_pressable(&self) -> bool {
        true
    }

    /// Generate active/pressed CSS class.
    fn active_class(&self, base_class: &str) -> String {
        format!("{}-active", base_class)
    }
}

/// Trait for draggable components.
pub trait Draggable {
    /// Check if the component is draggable.
    fn is_draggable(&self) -> bool;

    /// Get the `draggable` attribute value.
    fn draggable_attr(&self) -> &'static str {
        if self.is_draggable() { "true" } else { "false" }
    }
}

/// Trait for components that can be selected.
pub trait Selectable {
    /// Check if the component is currently selected.
    fn is_selected(&self) -> bool;

    /// Generate the selected CSS class.
    fn selected_class(&self, base_class: &str) -> Option<String> {
        if self.is_selected() {
            Some(format!("{}-selected", base_class))
        } else {
            None
        }
    }

    /// Get the `aria-selected` attribute value.
    fn aria_selected(&self) -> Option<&'static str> {
        if self.is_selected() {
            Some("true")
        } else {
            Some("false")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestButton {
        clickable: bool,
    }

    impl Clickable for TestButton {
        fn is_clickable(&self) -> bool {
            self.clickable
        }
    }

    impl Focusable for TestButton {}
    impl Hoverable for TestButton {}

    #[test]
    fn test_clickable() {
        let btn = TestButton { clickable: true };
        assert!(btn.is_clickable());
        assert_eq!(btn.cursor_style(), "pointer");

        let disabled = TestButton { clickable: false };
        assert!(!disabled.is_clickable());
        assert_eq!(disabled.cursor_style(), "default");
    }

    #[test]
    fn test_focusable() {
        let btn = TestButton { clickable: true };
        assert!(btn.is_focusable());
        assert_eq!(btn.tab_index(), 0);
        assert_eq!(btn.focus_class("fx-btn"), "fx-btn-focus");
    }
}
