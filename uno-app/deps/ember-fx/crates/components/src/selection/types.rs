//! Selection component type definitions.
//!
//! Shared types for selection components (Checkbox, Radio, Switch, Select).

use std::fmt;

/// Selection component size variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum SelectionSize {
    /// Small size
    Sm,
    /// Medium size (default)
    #[default]
    Md,
    /// Large size
    Lg,
}

impl SelectionSize {
    /// Get the CSS suffix for this size.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }

    /// Get the full CSS class for this size.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sm" | "small" => Some(Self::Sm),
            "md" | "medium" | "default" => Some(Self::Md),
            "lg" | "large" => Some(Self::Lg),
            _ => None,
        }
    }
}

impl fmt::Display for SelectionSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Checkbox/Radio visual style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum CheckStyle {
    /// Standard checkbox/radio appearance
    #[default]
    Standard,
    /// Button-like appearance
    Button,
    /// Card-like appearance with border
    Card,
}

impl CheckStyle {
    /// Get the CSS suffix for this style.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Button => "button",
            Self::Card => "card",
        }
    }

    /// Get the full CSS class for this style.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl fmt::Display for CheckStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Layout direction for groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum GroupLayout {
    /// Horizontal layout
    #[default]
    Horizontal,
    /// Vertical layout
    Vertical,
}

impl GroupLayout {
    /// Get the CSS suffix for this layout.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    /// Get the full CSS class for this layout.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl fmt::Display for GroupLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Select dropdown placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum DropdownPlacement {
    /// Below the trigger (default)
    #[default]
    Bottom,
    /// Above the trigger
    Top,
    /// Auto-detect based on available space
    Auto,
}

impl DropdownPlacement {
    /// Get the CSS suffix for this placement.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Bottom => "bottom",
            Self::Top => "top",
            Self::Auto => "auto",
        }
    }
}

impl fmt::Display for DropdownPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Option item for Select/MultiSelect components.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectOption<T: Clone + PartialEq> {
    /// The value of the option
    pub value: T,
    /// Display label
    pub label: String,
    /// Whether the option is disabled
    pub disabled: bool,
}

impl<T: Clone + PartialEq> SelectOption<T> {
    /// Create a new option with value and label.
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            disabled: false,
        }
    }

    /// Create a disabled option.
    pub fn disabled(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            disabled: true,
        }
    }
}

/// Simple string-based option for convenience.
pub type StringOption = SelectOption<String>;

impl StringOption {
    /// Create from a simple string (value = label).
    pub fn from_str(s: impl Into<String>) -> Self {
        let s = s.into();
        Self::new(s.clone(), s)
    }
}

/// Tree node for TreeSelect component.
#[derive(Debug, Clone, PartialEq)]
pub struct TreeSelectNode<T: Clone + PartialEq> {
    /// The value of the node.
    pub value: T,
    /// Display label.
    pub label: String,
    /// Child nodes.
    pub children: Vec<TreeSelectNode<T>>,
    /// Whether this node is disabled.
    pub disabled: bool,
    /// Whether this node is selectable (vs just a group header).
    pub selectable: bool,
    /// Whether this node can be checked (for checkable trees).
    pub checkable: bool,
    /// Optional icon.
    pub icon: Option<String>,
}

impl<T: Clone + PartialEq> TreeSelectNode<T> {
    /// Create a leaf node (no children).
    pub fn leaf(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            children: Vec::new(),
            disabled: false,
            selectable: true,
            checkable: true,
            icon: None,
        }
    }

    /// Create a branch node with children.
    pub fn branch(value: T, label: impl Into<String>, children: Vec<TreeSelectNode<T>>) -> Self {
        Self {
            value,
            label: label.into(),
            children,
            disabled: false,
            selectable: true,
            checkable: true,
            icon: None,
        }
    }

    /// Set disabled state.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Set non-selectable (group header only).
    pub fn non_selectable(mut self) -> Self {
        self.selectable = false;
        self
    }

    /// Set non-checkable.
    pub fn non_checkable(mut self) -> Self {
        self.checkable = false;
        self
    }

    /// Add an icon.
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Check if this node has children.
    pub fn is_branch(&self) -> bool {
        !self.children.is_empty()
    }

    /// Check if this is a leaf node.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

/// Simple string-based tree node.
pub type StringTreeNode = TreeSelectNode<String>;

/// Cascader option for multi-level selection.
#[derive(Debug, Clone, PartialEq)]
pub struct CascaderOption<T: Clone + PartialEq> {
    /// The value of the option.
    pub value: T,
    /// Display label.
    pub label: String,
    /// Child options (next level).
    pub children: Vec<CascaderOption<T>>,
    /// Whether this option is disabled.
    pub disabled: bool,
    /// Whether this is explicitly a leaf (even without children).
    pub is_leaf: bool,
}

impl<T: Clone + PartialEq> CascaderOption<T> {
    /// Create a leaf option (no children).
    pub fn leaf(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            children: Vec::new(),
            disabled: false,
            is_leaf: true,
        }
    }

    /// Create a branch option with children.
    pub fn branch(value: T, label: impl Into<String>, children: Vec<CascaderOption<T>>) -> Self {
        Self {
            value,
            label: label.into(),
            children,
            disabled: false,
            is_leaf: false,
        }
    }

    /// Set disabled state.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Check if this option has children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Simple string-based cascader option.
pub type StringCascaderOption = CascaderOption<String>;

/// Cascader trigger mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum CascaderTrigger {
    /// Expand on click (default).
    #[default]
    Click,
    /// Expand on hover.
    Hover,
}

impl CascaderTrigger {
    /// Get the CSS suffix for this trigger.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Click => "click",
            Self::Hover => "hover",
        }
    }

    /// Get the full CSS class.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-trigger-{}", prefix, self.as_suffix())
    }
}

impl fmt::Display for CascaderTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Mention trigger configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct MentionTrigger {
    /// The trigger character (e.g., '@', '#', '/').
    pub character: char,
    /// Optional prefix to add to the mention.
    pub prefix: Option<String>,
}

impl MentionTrigger {
    /// Create a new trigger with a character.
    pub fn new(character: char) -> Self {
        Self {
            character,
            prefix: None,
        }
    }

    /// Add a prefix.
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }
}

impl Default for MentionTrigger {
    fn default() -> Self {
        Self::new('@')
    }
}

/// Mention option/suggestion.
#[derive(Debug, Clone, PartialEq)]
pub struct MentionOption {
    /// The value to insert.
    pub value: String,
    /// Display label.
    pub label: String,
    /// Optional avatar/image URL.
    pub avatar: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Whether this option is disabled.
    pub disabled: bool,
}

impl MentionOption {
    /// Create a new mention option.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            avatar: None,
            description: None,
            disabled: false,
        }
    }

    /// Create a simple option where value equals label.
    pub fn simple(value: impl Into<String>) -> Self {
        let v = value.into();
        Self::new(v.clone(), v)
    }

    /// Add an avatar.
    pub fn with_avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    /// Add a description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set disabled state.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_size() {
        assert_eq!(SelectionSize::default(), SelectionSize::Md);
        assert_eq!(SelectionSize::Lg.as_suffix(), "lg");
        assert_eq!(SelectionSize::Sm.class("fx-checkbox"), "fx-checkbox-sm");
    }

    #[test]
    fn test_check_style() {
        assert_eq!(CheckStyle::default(), CheckStyle::Standard);
        assert_eq!(CheckStyle::Button.as_suffix(), "button");
    }

    #[test]
    fn test_group_layout() {
        assert_eq!(GroupLayout::default(), GroupLayout::Horizontal);
        assert_eq!(GroupLayout::Vertical.as_suffix(), "vertical");
    }

    #[test]
    fn test_select_option() {
        let opt = SelectOption::new("value1", "Label 1");
        assert_eq!(opt.value, "value1");
        assert_eq!(opt.label, "Label 1");
        assert!(!opt.disabled);

        let disabled = SelectOption::disabled("value2", "Label 2");
        assert!(disabled.disabled);
    }

    #[test]
    fn test_string_option() {
        let opt = StringOption::from_str("Option 1");
        assert_eq!(opt.value, "Option 1");
        assert_eq!(opt.label, "Option 1");
    }

    #[test]
    fn test_tree_select_node() {
        let leaf = StringTreeNode::leaf("leaf1".to_string(), "Leaf 1");
        assert!(leaf.is_leaf());
        assert!(!leaf.is_branch());

        let branch = StringTreeNode::branch(
            "branch1".to_string(),
            "Branch 1",
            vec![leaf],
        );
        assert!(branch.is_branch());
        assert!(!branch.is_leaf());
        assert_eq!(branch.children.len(), 1);
    }

    #[test]
    fn test_cascader_option() {
        let leaf = StringCascaderOption::leaf("leaf1".to_string(), "Leaf 1");
        assert!(leaf.is_leaf);
        assert!(!leaf.has_children());

        let branch = StringCascaderOption::branch(
            "branch1".to_string(),
            "Branch 1",
            vec![leaf],
        );
        assert!(!branch.is_leaf);
        assert!(branch.has_children());
    }

    #[test]
    fn test_cascader_trigger() {
        assert_eq!(CascaderTrigger::default(), CascaderTrigger::Click);
        assert_eq!(CascaderTrigger::Hover.as_suffix(), "hover");
    }

    #[test]
    fn test_mention_trigger() {
        let trigger = MentionTrigger::default();
        assert_eq!(trigger.character, '@');
        assert!(trigger.prefix.is_none());

        let custom = MentionTrigger::new('#').with_prefix("tag:");
        assert_eq!(custom.character, '#');
        assert_eq!(custom.prefix, Some("tag:".to_string()));
    }

    #[test]
    fn test_mention_option() {
        let opt = MentionOption::new("jdoe", "John Doe")
            .with_avatar("/avatars/jdoe.png")
            .with_description("Software Engineer");

        assert_eq!(opt.value, "jdoe");
        assert_eq!(opt.label, "John Doe");
        assert_eq!(opt.avatar, Some("/avatars/jdoe.png".to_string()));
        assert_eq!(opt.description, Some("Software Engineer".to_string()));
        assert!(!opt.disabled);
    }
}
