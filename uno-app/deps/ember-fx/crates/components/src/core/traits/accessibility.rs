//! Accessibility traits for a11y support.
//!
//! These traits ensure components are accessible to all users,
//! including those using assistive technologies.

use std::borrow::Cow;

/// ARIA role values for components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaRole {
    Button,
    Checkbox,
    Dialog,
    Grid,
    GridCell,
    Link,
    Listbox,
    Menu,
    MenuItem,
    MenuItemCheckbox,
    MenuItemRadio,
    Option,
    Progressbar,
    Radio,
    RadioGroup,
    Slider,
    Spinbutton,
    Switch,
    Tab,
    TabList,
    TabPanel,
    Textbox,
    Tree,
    TreeItem,
    Alert,
    AlertDialog,
    Status,
    Tooltip,
    None,
}

impl AriaRole {
    /// Get the ARIA role string value.
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            Self::Button => Some("button"),
            Self::Checkbox => Some("checkbox"),
            Self::Dialog => Some("dialog"),
            Self::Grid => Some("grid"),
            Self::GridCell => Some("gridcell"),
            Self::Link => Some("link"),
            Self::Listbox => Some("listbox"),
            Self::Menu => Some("menu"),
            Self::MenuItem => Some("menuitem"),
            Self::MenuItemCheckbox => Some("menuitemcheckbox"),
            Self::MenuItemRadio => Some("menuitemradio"),
            Self::Option => Some("option"),
            Self::Progressbar => Some("progressbar"),
            Self::Radio => Some("radio"),
            Self::RadioGroup => Some("radiogroup"),
            Self::Slider => Some("slider"),
            Self::Spinbutton => Some("spinbutton"),
            Self::Switch => Some("switch"),
            Self::Tab => Some("tab"),
            Self::TabList => Some("tablist"),
            Self::TabPanel => Some("tabpanel"),
            Self::Textbox => Some("textbox"),
            Self::Tree => Some("tree"),
            Self::TreeItem => Some("treeitem"),
            Self::Alert => Some("alert"),
            Self::AlertDialog => Some("alertdialog"),
            Self::Status => Some("status"),
            Self::Tooltip => Some("tooltip"),
            Self::None => None,
        }
    }
}

/// Core accessibility trait for all interactive components.
///
/// This trait provides the foundation for accessible components
/// by ensuring proper ARIA attributes and semantic structure.
///
/// # Example
///
/// ```ignore
/// use ember_fx::core::{Accessible, AriaRole};
///
/// struct Button {
///     label: String,
/// }
///
/// impl Accessible for Button {
///     fn aria_role(&self) -> AriaRole {
///         AriaRole::Button
///     }
///
///     fn aria_label(&self) -> Option<Cow<'_, str>> {
///         Some(Cow::Borrowed(&self.label))
///     }
/// }
/// ```
pub trait Accessible {
    /// Get the ARIA role for this component.
    fn aria_role(&self) -> AriaRole {
        AriaRole::None
    }

    /// Get the accessible label for screen readers.
    fn aria_label(&self) -> Option<Cow<'_, str>> {
        None
    }

    /// Get the ID of the element that labels this component.
    fn aria_labelledby(&self) -> Option<&str> {
        None
    }

    /// Get the ID of the element that describes this component.
    fn aria_describedby(&self) -> Option<&str> {
        None
    }

    /// Check if the component is hidden from assistive technology.
    fn aria_hidden(&self) -> Option<bool> {
        None
    }

    /// Get the live region politeness setting.
    fn aria_live(&self) -> Option<&'static str> {
        None
    }
}

/// Trait for components that support keyboard navigation.
pub trait KeyboardNavigable {
    /// Check if keyboard navigation is enabled.
    fn supports_keyboard_nav(&self) -> bool {
        true
    }

    /// Get keyboard shortcuts for this component.
    fn keyboard_shortcuts(&self) -> &[(&'static str, &'static str)] {
        &[]
    }
}

/// Trait for components with expanded/collapsed state.
pub trait Expandable {
    /// Check if the component is currently expanded.
    fn is_expanded(&self) -> bool;

    /// Get the `aria-expanded` attribute value.
    fn aria_expanded(&self) -> &'static str {
        if self.is_expanded() { "true" } else { "false" }
    }

    /// Get the ID of the controlled element.
    fn aria_controls(&self) -> Option<&str> {
        None
    }
}

/// Trait for components with checked state (checkbox, radio, switch).
pub trait Checkable {
    /// Check if the component is currently checked.
    fn is_checked(&self) -> bool;

    /// Check if the component is in indeterminate state.
    fn is_indeterminate(&self) -> bool {
        false
    }

    /// Get the `aria-checked` attribute value.
    fn aria_checked(&self) -> &'static str {
        if self.is_indeterminate() {
            "mixed"
        } else if self.is_checked() {
            "true"
        } else {
            "false"
        }
    }
}

/// Trait for components with value range (slider, progress).
pub trait RangeValue {
    /// Get the current value.
    fn value(&self) -> f64;

    /// Get the minimum value.
    fn min_value(&self) -> f64 {
        0.0
    }

    /// Get the maximum value.
    fn max_value(&self) -> f64 {
        100.0
    }

    /// Get the value as a formatted string for display.
    fn value_text(&self) -> Option<String> {
        None
    }

    /// Get `aria-valuenow` as string.
    fn aria_valuenow(&self) -> String {
        self.value().to_string()
    }

    /// Get `aria-valuemin` as string.
    fn aria_valuemin(&self) -> String {
        self.min_value().to_string()
    }

    /// Get `aria-valuemax` as string.
    fn aria_valuemax(&self) -> String {
        self.max_value().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestButton {
        label: String,
    }

    impl Accessible for TestButton {
        fn aria_role(&self) -> AriaRole {
            AriaRole::Button
        }

        fn aria_label(&self) -> Option<Cow<'_, str>> {
            Some(Cow::Borrowed(&self.label))
        }
    }

    #[test]
    fn test_accessible() {
        let btn = TestButton { label: "Submit".to_string() };
        assert_eq!(btn.aria_role().as_str(), Some("button"));
        assert_eq!(btn.aria_label(), Some(Cow::Borrowed("Submit")));
    }

    #[test]
    fn test_aria_role() {
        assert_eq!(AriaRole::Button.as_str(), Some("button"));
        assert_eq!(AriaRole::Checkbox.as_str(), Some("checkbox"));
        assert_eq!(AriaRole::None.as_str(), None);
    }
}
