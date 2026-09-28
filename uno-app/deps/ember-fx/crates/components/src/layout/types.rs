//! Layout component types.

/// Card size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CardSize {
    /// Small card.
    Small,
    /// Default card size.
    #[default]
    Default,
}

impl CardSize {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "default",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Modal/Dialog size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ModalSize {
    /// Small modal (400px).
    Small,
    /// Default modal (520px).
    #[default]
    Default,
    /// Large modal (800px).
    Large,
    /// Full screen modal.
    FullScreen,
}

impl ModalSize {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "default",
            Self::Large => "lg",
            Self::FullScreen => "fullscreen",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Drawer placement options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DrawerPlacement {
    /// Drawer from the top.
    Top,
    /// Drawer from the right (default).
    #[default]
    Right,
    /// Drawer from the bottom.
    Bottom,
    /// Drawer from the left.
    Left,
}

impl DrawerPlacement {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Drawer size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DrawerSize {
    /// Small drawer (256px).
    Small,
    /// Default drawer (378px).
    #[default]
    Default,
    /// Large drawer (736px).
    Large,
}

impl DrawerSize {
    /// Returns the size in pixels for horizontal drawers.
    pub fn width(&self) -> u32 {
        match self {
            Self::Small => 256,
            Self::Default => 378,
            Self::Large => 736,
        }
    }

    /// Returns the size in pixels for vertical drawers.
    pub fn height(&self) -> u32 {
        match self {
            Self::Small => 256,
            Self::Default => 378,
            Self::Large => 50,  // 50% of viewport
        }
    }
}

/// Tab position options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TabPosition {
    /// Tabs at the top (default).
    #[default]
    Top,
    /// Tabs on the right.
    Right,
    /// Tabs at the bottom.
    Bottom,
    /// Tabs on the left.
    Left,
}

impl TabPosition {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Tab type/style variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TabType {
    /// Line style tabs (default).
    #[default]
    Line,
    /// Card style tabs.
    Card,
    /// Editable card tabs.
    EditableCard,
}

impl TabType {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Card => "card",
            Self::EditableCard => "editable-card",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Tab item configuration.
#[derive(Debug, Clone)]
pub struct TabItem {
    /// Unique key for the tab.
    pub key: String,
    /// Tab label.
    pub label: String,
    /// Whether the tab is disabled.
    pub disabled: bool,
    /// Whether the tab can be closed (for editable-card type).
    pub closable: bool,
    /// Icon for the tab.
    pub icon: Option<String>,
}

impl TabItem {
    /// Create a new tab item.
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            disabled: false,
            closable: true,
            icon: None,
        }
    }

    /// Set whether the tab is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set whether the tab can be closed.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set the tab icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Collapse/Accordion expand icon position.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CollapseIconPosition {
    /// Icon on the left (default).
    #[default]
    Start,
    /// Icon on the right.
    End,
}

impl CollapseIconPosition {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-icon-{}", prefix, self.as_suffix())
    }
}

/// Divider type variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DividerType {
    /// Horizontal divider (default).
    #[default]
    Horizontal,
    /// Vertical divider.
    Vertical,
}

impl DividerType {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Divider text orientation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DividerOrientation {
    /// Text on the left.
    Left,
    /// Text in the center (default).
    #[default]
    Center,
    /// Text on the right.
    Right,
}

impl DividerOrientation {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-text-{}", prefix, self.as_suffix())
    }
}
