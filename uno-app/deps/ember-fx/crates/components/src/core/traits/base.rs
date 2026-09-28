//! Base traits for all components.
//!
//! These traits define the fundamental interface that all components share.

use std::borrow::Cow;

/// Component type identifier.
///
/// Used for CSS class generation and debugging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentType {
    Button,
    Input,
    TextInput,
    PasswordInput,
    NumberInput,
    Textarea,
    Select,
    Checkbox,
    Radio,
    Switch,
    Slider,
    Card,
    Modal,
    Dropdown,
    Menu,
    Tabs,
    Table,
    Tree,
    Form,
    Custom(&'static str),
}

impl ComponentType {
    /// Get the kebab-case name for CSS class generation.
    ///
    /// # Example
    /// ```ignore
    /// use ember_fx::core::ComponentType;
    /// assert_eq!(ComponentType::Button.as_str(), "button");
    /// assert_eq!(ComponentType::TextInput.as_str(), "text-input");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Input => "input",
            Self::TextInput => "text-input",
            Self::PasswordInput => "password-input",
            Self::NumberInput => "number-input",
            Self::Textarea => "textarea",
            Self::Select => "select",
            Self::Checkbox => "checkbox",
            Self::Radio => "radio",
            Self::Switch => "switch",
            Self::Slider => "slider",
            Self::Card => "card",
            Self::Modal => "modal",
            Self::Dropdown => "dropdown",
            Self::Menu => "menu",
            Self::Tabs => "tabs",
            Self::Table => "table",
            Self::Tree => "tree",
            Self::Form => "form",
            Self::Custom(name) => name,
        }
    }
}

impl std::fmt::Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Core trait for all ember-fx components.
///
/// This trait provides the fundamental interface for component identification
/// and CSS class generation.
///
/// # Example
///
/// ```ignore
/// use ember_fx::core::{FxComponent, ComponentType, ComponentSize, Sizable};
///
/// struct MyButton {
///     size: ComponentSize,
/// }
///
/// impl FxComponent for MyButton {
///     fn component_type(&self) -> ComponentType {
///         ComponentType::Button
///     }
/// }
/// ```
pub trait FxComponent {
    /// Returns the component type for identification.
    fn component_type(&self) -> ComponentType;

    /// Generate the base CSS class for this component.
    ///
    /// Format: `fx-{component-type}-{design-system}`
    ///
    /// # Example
    /// ```ignore
    /// // Returns "fx-button-ant" for Ant design system
    /// let class = button.base_class("ant");
    /// ```
    fn base_class(&self, design_system: &str) -> String {
        format!("fx-{}-{}", self.component_type().as_str(), design_system)
    }

    /// Generate CSS classes for this component including variants.
    ///
    /// Default implementation returns just the base class.
    /// Override to add variant classes.
    fn css_classes(&self, design_system: &str) -> Cow<'static, str> {
        Cow::Owned(self.base_class(design_system))
    }
}
