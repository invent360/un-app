//! Visualization component types.

/// Result status variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResultStatus {
    /// Success status (default).
    #[default]
    Success,
    /// Error status.
    Error,
    /// Info status.
    Info,
    /// Warning status.
    Warning,
    /// 404 Not Found.
    NotFound,
    /// 403 Forbidden.
    Forbidden,
    /// 500 Server Error.
    ServerError,
}

impl ResultStatus {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Error => "error",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::NotFound => "404",
            Self::Forbidden => "403",
            Self::ServerError => "500",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }

    /// Returns the default icon for the status.
    pub fn default_icon(&self) -> &'static str {
        match self {
            Self::Success => "✓",
            Self::Error => "✕",
            Self::Info => "ℹ",
            Self::Warning => "⚠",
            Self::NotFound => "404",
            Self::Forbidden => "403",
            Self::ServerError => "500",
        }
    }
}

/// Timeline mode variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TimelineMode {
    /// Left aligned (default).
    #[default]
    Left,
    /// Right aligned.
    Right,
    /// Alternate sides.
    Alternate,
}

impl TimelineMode {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Alternate => "alternate",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Timeline item color.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TimelineItemColor {
    /// Blue (default).
    #[default]
    Blue,
    /// Green.
    Green,
    /// Red.
    Red,
    /// Gray.
    Gray,
    /// Custom color.
    Custom(String),
}

impl TimelineItemColor {
    /// Returns the CSS color value.
    pub fn as_css(&self) -> String {
        match self {
            Self::Blue => "var(--fx-color-primary, #1890ff)".to_string(),
            Self::Green => "var(--fx-color-success, #52c41a)".to_string(),
            Self::Red => "var(--fx-color-error, #ff4d4f)".to_string(),
            Self::Gray => "var(--fx-color-text-tertiary, rgba(255, 255, 255, 0.45))".to_string(),
            Self::Custom(color) => color.clone(),
        }
    }
}

/// Timeline item configuration.
#[derive(Debug, Clone)]
pub struct TimelineItem {
    /// Item content.
    pub content: String,
    /// Item label (for alternate mode).
    pub label: Option<String>,
    /// Dot color.
    pub color: TimelineItemColor,
    /// Custom dot content.
    pub dot: Option<String>,
}

impl TimelineItem {
    /// Create a new timeline item.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            label: None,
            color: TimelineItemColor::default(),
            dot: None,
        }
    }

    /// Set the label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the color.
    pub fn color(mut self, color: TimelineItemColor) -> Self {
        self.color = color;
        self
    }

    /// Set custom dot content.
    pub fn dot(mut self, dot: impl Into<String>) -> Self {
        self.dot = Some(dot.into());
        self
    }
}

/// Statistic value prefix/suffix position.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StatisticValueStyle {
    /// Default style.
    #[default]
    Default,
    /// Positive (green, up arrow).
    Positive,
    /// Negative (red, down arrow).
    Negative,
}

impl StatisticValueStyle {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Positive => "positive",
            Self::Negative => "negative",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Descriptions layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DescriptionsLayout {
    /// Horizontal layout (default).
    #[default]
    Horizontal,
    /// Vertical layout.
    Vertical,
}

impl DescriptionsLayout {
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

/// Descriptions size.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DescriptionsSize {
    /// Small size.
    Small,
    /// Default size.
    #[default]
    Default,
}

impl DescriptionsSize {
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

/// Description item configuration.
#[derive(Debug, Clone)]
pub struct DescriptionItem {
    /// Item label.
    pub label: String,
    /// Item content.
    pub content: String,
    /// Span columns.
    pub span: usize,
}

impl DescriptionItem {
    /// Create a new description item.
    pub fn new(label: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            content: content.into(),
            span: 1,
        }
    }

    /// Set the span.
    pub fn span(mut self, span: usize) -> Self {
        self.span = span;
        self
    }
}

/// Tree node configuration.
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// Node key.
    pub key: String,
    /// Node title.
    pub title: String,
    /// Whether the node is disabled.
    pub disabled: bool,
    /// Whether the node is a leaf (no children).
    pub is_leaf: bool,
    /// Child nodes.
    pub children: Vec<TreeNode>,
    /// Custom icon.
    pub icon: Option<String>,
}

impl TreeNode {
    /// Create a new tree node.
    pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            disabled: false,
            is_leaf: false,
            children: Vec::new(),
            icon: None,
        }
    }

    /// Set whether disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set whether leaf node.
    pub fn is_leaf(mut self, is_leaf: bool) -> Self {
        self.is_leaf = is_leaf;
        self
    }

    /// Add children.
    pub fn children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    /// Set icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}
